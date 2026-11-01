//! Second-stage bootloader and `USBUPG` firmware upgrade agent.
#![no_std]
#![no_main]

#[cfg(all(feature = "d13x", feature = "d21x"))]
compile_error!("select exactly one chip feature: `d13x` or `d21x`");
#[cfg(not(any(feature = "d13x", feature = "d21x")))]
compile_error!("select a chip feature: `d13x` or `d21x`");

#[cfg(feature = "d13x")]
mod d13x;
#[cfg(feature = "d21x")]
mod d21x;
mod layout;
mod storage;

use core::cell::Cell;
use core::convert::Infallible;

use artinchip_hal::aic_upg::{AicUpgBuffers, AicUpgUsb, Engine, StorageBackend};
use artinchip_hal::bind_interrupts;
use artinchip_hal::cmu::{ClockTree, Cmu};
#[cfg(feature = "dma")]
use artinchip_hal::dma::Dma;
use artinchip_hal::gtc::{CntFreq, GtcExt as _, TimerDelay};
use artinchip_hal::interrupt::InterruptSource as _;
use artinchip_hal::interrupt::typelevel::USB_DEV;
use artinchip_hal::qspi::{Qspi, QspiConfig, QuadSpiBus};
use artinchip_hal::sys_cfg::SocSysCfg;
use artinchip_hal::system::{self, Error as ImageError, ReadAt, Role};
use artinchip_hal::uart::{UartConfig, uart_logger_init};
use artinchip_hal::usbdrd::device::{UsbDev, UsbDevInterrupt};
use artinchip_hal::usbdrd::{UsbConfig, UsbPads};
use artinchip_hal::wdog::{Wdog, WdogExt as _};
use artinchip_rt::sync::StaticSlot;
use artinchip_rt::{Peripherals, boot_entry, default_clk_tree, prelude::*};
use embassy_futures::block_on;
use embedded_hal::digital::OutputPin;
use log::{debug, error};

#[cfg(feature = "d13x")]
use artinchip_rt::soc::d13x::GpioBPads;
#[cfg(feature = "d21x")]
use artinchip_rt::soc::d21x::GpioBPads;
#[cfg(feature = "d13x")]
use d13x::Board;
#[cfg(feature = "d21x")]
use d21x::Board;

use layout::{BOOT_APP, FLASH_LAYOUT, PARTITION_TABLE};

bind_interrupts!(struct Irqs {
    USB_DEV => UsbDevInterrupt;
});

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    error!("app-bootloader: PANIC: {panic_info}");
    system::halt()
}

/// Host-visible staging buffer size (`GET_MEM_BUF` / `WRITE`).
const SCRATCH_SIZE: usize = 4 * 1024;

/// Partition containing the bootloader itself (`[[partition]]` in `boards/*.toml`).
const BOOTLOADER_PARTITION: &str = "bootloader";

/// GTC ticks per millisecond at `CntFreq::Freq4M`.
const TICKS_PER_MS: u64 = 4_000;

/// Cache-line-aligned DMA/staging buffer (64 bytes covers both D13x and D21x).
#[repr(align(64))]
pub(crate) struct DmaBuf<const N: usize>(pub(crate) [u8; N]);

static SCRATCH: StaticSlot<DmaBuf<SCRATCH_SIZE>> = StaticSlot::new(DmaBuf([0; SCRATCH_SIZE]));
static UPG_BUFFERS: StaticSlot<AicUpgBuffers> = StaticSlot::new(AicUpgBuffers::new());

/// Session deadline policy for the USB upgrade loop.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostPolicy {
    /// Wait up to `ms` milliseconds for a host upgrade command, then try booting flash.
    Window { ms: u32 },
    /// Serve the host indefinitely with no timeout.
    Unbounded,
}

/// BootROM arguments forwarded to the booted application in `a0`/`a1`/`a2`.
#[derive(Clone, Copy)]
pub struct AppArguments<'a> {
    pub boot_param: BootParam,
    pub private: &'a [u8],
}

/// Hardware and storage handles passed to [`upgrade_role`].
pub struct UpgradeContext<'a, St: StorageBackend, PAD: UsbPads<0> + 'static> {
    pub storage: &'a mut St,
    pub usb: UsbDev,
    pub pads: PAD,
    pub tick: TimerDelay<'static>,
    pub cmu: &'a mut Cmu,
    pub syscfg: &'a mut SocSysCfg,
    pub wdog: Wdog,
}

/// Chip-specific QSPI bus/pin mapping and USB hand-over hooks.
pub trait BootChip {
    /// QSPI bus built by the chip module, including any DMA wrapper.
    type Bus: QuadSpiBus + 'static;
    /// Chip-select pin driven by the flash driver.
    type Cs: OutputPin<Error = Infallible> + 'static;

    const HOST_POLICY: HostPolicy;

    fn init_bus(
        qspi0: Qspi<0>,
        gpiob: GpioBPads,
        #[cfg(feature = "dma")] dma: Dma,
        clocks: &ClockTree,
        cmu: &mut Cmu,
        config: QspiConfig,
    ) -> (Self::Bus, Self::Cs);

    fn before_usb(usb: &mut UsbDev, cmu: &mut Cmu) {
        let _ = (usb, cmu);
    }

    fn app_args(boot_param: BootParam, private: &[u8]) -> AppArguments<'_>;
}

/// Media storage type for chip `C`; `nor` / `nand` decides the flash driver.
pub type BoardStorage<C> = storage::Storage<<C as BootChip>::Bus, <C as BootChip>::Cs>;

/// [`ReadAt`] adapter over [`StorageBackend`].
struct StorageRead<'a>(&'a mut dyn StorageBackend);

impl ReadAt for StorageRead<'_> {
    fn read_at(&mut self, offset: u32, buf: &mut [u8]) -> Result<(), ImageError> {
        if self.0.read(offset as u64, buf) == buf.len() {
            Ok(())
        } else {
            Err(ImageError::Read)
        }
    }
}

/// Boot an application, if one is present after the bootloader partition.
///
/// With `app` named in the board preset, only that AIC image is accepted;
/// otherwise the first valid image after the bootloader wins.
fn find_boot_app(src: &mut impl ReadAt) -> Option<system::Loader> {
    let candidates = FLASH_LAYOUT
        .iter()
        .skip_while(|partition| partition.name != BOOTLOADER_PARTITION)
        .skip(1);

    for partition in candidates {
        let Ok(loader) = system::resolve(src, partition.offset as u32) else {
            // An empty or non-image partition (e.g. `data`) is normal, not an
            // error: keep looking.
            continue;
        };

        if !BOOT_APP.is_empty() && loader.name() != Some(BOOT_APP) {
            debug!(
                "app-bootloader: {} holds {:?}, want {BOOT_APP:?}",
                partition.name,
                loader.name().unwrap_or("<unnamed>")
            );
            continue;
        }
        return Some(loader);
    }

    if BOOT_APP.is_empty() {
        debug!("app-bootloader: No bootable app image, staying in upgrade mode");
    } else {
        debug!("app-bootloader: No app named {BOOT_APP:?}, staying in upgrade mode");
    }
    None
}

/// Copy `loader` into RAM, flush caches, and jump to its entry point.
///
/// # Safety
/// `loader` must describe a valid AIC payload targeting writable, executable RAM.
unsafe fn boot_app(src: &mut impl ReadAt, loader: &system::Loader, args: AppArguments<'_>) -> ! {
    if let Err(err) = system::load(src, loader) {
        error!("app-bootloader: App copy failed: {err:?}, halting");
        system::halt();
    }
    unsafe { dcache_clean_invalidate_range(loader.dst as usize, loader.length as usize) };
    debug!(
        "app-bootloader: Booting app {:?} @ {:#x}",
        loader.name().unwrap_or("<unnamed>"),
        loader.entry
    );
    let (private_addr, private_len) = if args.private.is_empty() {
        (core::ptr::null(), 0)
    } else {
        (args.private.as_ptr(), args.private.len() as u32)
    };
    unsafe {
        system::jump_to(
            loader.entry,
            args.boot_param.as_raw(),
            private_addr,
            private_len,
        )
    }
}

pub fn boot_role<St: StorageBackend>(
    storage: &mut St,
    mut usb: UsbDev,
    cmu: &mut Cmu,
    args: AppArguments<'_>,
) -> ! {
    usb.release(cmu);

    let loader = {
        let mut src = StorageRead(storage);
        find_boot_app(&mut src)
    };
    let Some(loader) = loader else {
        error!("app-bootloader: No bootable app");
        system::halt();
    };
    let mut src = StorageRead(storage);
    unsafe { boot_app(&mut src, &loader, args) }
}

/// Reset the chip through the watchdog so the BootROM restarts the flash boot
/// chain this session just wrote, rather than jumping into the old RAM copy.
fn reset_board(wdog: Wdog, cmu: &mut Cmu) -> ! {
    let mut wdog = wdog.new_driver(cmu);
    // CLR < IRQ < RST: RST is the threshold that resets the chip.
    wdog.configure_scene_and_apply(0, 0, 1, 2);
    loop {
        core::hint::spin_loop();
    }
}

pub fn upgrade_role<C, PAD>(mut ctx: UpgradeContext<'_, BoardStorage<C>, PAD>) -> !
where
    C: BootChip,
    PAD: UsbPads<0> + 'static,
{
    USB_DEV::disable();
    C::before_usb(&mut ctx.usb, ctx.cmu);

    let buffers = UPG_BUFFERS.take();
    let usb_config = UsbConfig::default();
    let mut upg = match AicUpgUsb::new(
        ctx.usb, ctx.pads, usb_config, Irqs, buffers, ctx.cmu, ctx.syscfg,
    ) {
        Ok(upg) => upg,
        Err(err) => {
            error!("app-bootloader: USB controller init failed: {err:?}");
            system::halt();
        }
    };

    let mut engine = Engine::new(ctx.storage, PARTITION_TABLE);
    match C::HOST_POLICY {
        HostPolicy::Window { ms } => {
            let deadline = ctx.tick.get_tick() + u64::from(ms) * TICKS_PER_MS;
            let seen_host = Cell::new(false);
            let tick = ctx.tick;
            block_on(upg.run(&mut engine, |eng| {
                if eng.host_upgrading() {
                    seen_host.set(true);
                }
                !seen_host.get() && tick.get_tick() >= deadline
            }));
        }
        HostPolicy::Unbounded => {
            block_on(upg.run(&mut engine, |_| false));
        }
    }

    let empty_args = AppArguments {
        boot_param: BootParam::from_raw(0),
        private: &[],
    };

    if engine.reset_requested() {
        debug!("app-bootloader: Host asked to reset, resetting the board");
        drop(engine);
        upg.deinit(ctx.cmu);
        reset_board(ctx.wdog, ctx.cmu);
    }

    if matches!(C::HOST_POLICY, HostPolicy::Window { .. }) {
        drop(engine);
        block_on(upg.disconnect());

        let loader = {
            let mut src = StorageRead(ctx.storage);
            find_boot_app(&mut src)
        };
        if let Some(loader) = loader {
            upg.deinit(ctx.cmu);
            let mut src = StorageRead(ctx.storage);
            unsafe { boot_app(&mut src, &loader, empty_args) }
        }

        debug!("app-bootloader: Nothing to boot, staying in upgrade mode");
        let mut engine = Engine::new(ctx.storage, PARTITION_TABLE);
        block_on(upg.run(&mut engine, |_| false));
    }

    debug!("app-bootloader: Upgrade session ended without reset, halting");
    system::halt()
}

fn run<C: BootChip>(boot_param: BootParam, private_data: &[u8]) -> ! {
    USB_DEV::disable();

    let mut p = Peripherals::take();
    let clocks = default_clk_tree();

    let uart0_tx = p.gpioa.pa0.into_uart0_tx();
    let uart0_rx = p.gpioa.pa1.into_uart0_rx();
    let _uart0 = uart_logger_init(
        p.uart0,
        uart0_tx,
        uart0_rx,
        UartConfig::default(),
        &clocks,
        &mut p.cmu,
    )
    .ok();

    let tick = p.gtc.new_timer_delay(CntFreq::Freq4M, &mut p.cmu);
    let scratch = &mut SCRATCH.take().0;
    let (bus, cs) = C::init_bus(
        p.qspi0,
        p.gpiob,
        #[cfg(feature = "dma")]
        p.dma,
        &clocks,
        &mut p.cmu,
        storage::qspi_config(),
    );
    let mut storage = storage::init(bus, cs, tick, boot_param, scratch);

    let role = system::role();
    debug!("app-bootloader: Role {role:?}");

    if role == Role::Boot {
        boot_role(
            &mut storage,
            p.usb_dev,
            &mut p.cmu,
            C::app_args(boot_param, private_data),
        );
    }

    let dm = p.gpiou.pu0.into_usb0_dm();
    let dp = p.gpiou.pu1.into_usb0_dp();
    upgrade_role::<C, _>(UpgradeContext {
        storage: &mut storage,
        usb: p.usb_dev,
        pads: (dm, dp),
        tick,
        cmu: &mut p.cmu,
        syscfg: &mut p.syscfg,
        wdog: p.wdog,
    })
}

#[boot_entry]
fn app_main(boot_param: BootParam, private_data: &[u8]) -> ! {
    // Deliberately skip `check_startup()`: PA0 is handled one stage earlier by
    // `pbp-common`, whereas probing a held BOOT key here would bounce a host-driven
    // RAM upgrade session back into the BootROM mid-burn.
    run::<Board>(boot_param, private_data)
}
