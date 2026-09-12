#![no_std]
#![no_main]
#![feature(abi_riscv_interrupt)]

mod storage;

use artinchip_hal::clic_bind_interrupts;
use artinchip_hal::cmu::Cmu;
use artinchip_hal::gtc::*;
use artinchip_hal::prelude::NoPad;
use artinchip_hal::qspi::*;
use artinchip_hal::system::{self, Error as ImageError, ReadAt, Role};
use artinchip_hal::uart::{UartConfig, uart_logger_init};
use artinchip_hal::usbdrd::UsbConfig;
use artinchip_hal::usbdrd::{
    AsyncUpgHandler, Engine, NorFlash, NorStorage, SECTOR_SIZE, Storage as _, UsbDevExt as _,
};
use artinchip_hal::wdog::{Wdog, WdogExt as _};
use artinchip_rt::{Peripherals, app_entry, prelude::*};
use embassy_futures::block_on;
use log::debug;
use w25qxxxjv::{Model, SpiSpeed, W25QXXXJV};

use storage::{BOOT_APP, FLASH_LAYOUT, NorFlashDelay, PARTITION_TABLE};

// Bind the USB device interrupt; the vector table is filled in by the reset code.
clic_bind_interrupts!(struct Irqs {
    USB_DEV => AsyncUpgHandler;
});

/// Report panics over UART: a silent halt would make a failed `.unwrap()`
/// indistinguishable from a hang, which this example needs to see.
#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    debug!("app-bootloader: PANIC: {panic_info}");
    loop {
        core::hint::spin_loop();
    }
}

const SCRATCH_SIZE: usize = 4 * 1024;

/// Partition the bootloader itself lives in (`[[partition]]` in the manifest).
/// Auto-boot starts *after* this one, so layout order decides what runs next.
const BOOTLOADER_PARTITION: &str = "bootloader";

/// GTC ticks per millisecond. The counter base is exactly 4 MHz (`GTC_CNTFID0` is
/// hardwired to `0x003D0900` for a 24 MHz `PCLK`), so 4000 ticks = 1 ms.
const TICKS_PER_MS: u64 = 4_000;

/// How long either role listens for a host before booting the application.
///
/// The `Upgrade` role drops this deadline for good once a real upgrade command
/// arrives, so a started burn runs to its `reset`; the window only covers a host
/// that re-enumerated us and then went quiet. Must stay below AiBurn's ~10-12 s.
const HOST_DELAY_MS: u32 = 300;

/// Cache-line-aligned DMA buffer: the controller DMAs whole lines, so a buffer
/// sharing one with unrelated data would discard it on invalidate. 64 bytes
/// covers both the D13x (32) and D21x (64) lines.
#[repr(align(64))]
struct DmaBuf<const N: usize>([u8; N]);

static mut SECTOR_BUF: DmaBuf<SECTOR_SIZE> = DmaBuf([0; SECTOR_SIZE]);
static mut SCRATCH: DmaBuf<SCRATCH_SIZE> = DmaBuf([0; SCRATCH_SIZE]);

/// [`ReadAt`] adapter over the upgrade storage, used to resolve the app image.
struct StorageRead<'a, F: NorFlash>(&'a mut NorStorage<F>);

impl<F: NorFlash> ReadAt for StorageRead<'_, F> {
    fn read_at(&mut self, offset: u32, buf: &mut [u8]) -> Result<(), ImageError> {
        if self.0.read(offset as u64, buf) == buf.len() {
            Ok(())
        } else {
            Err(ImageError::Read)
        }
    }
}

/// Boot an application, if there is one.
///
/// With `[image] app` named, the image whose AIC header carries that name over the
/// partitions after the bootloader's; empty (the default) boots the **first**
/// such partition holding a valid `"AIC "` image, which the manifest's layout
/// order makes "whatever comes next".
///
/// Returns only when there is nothing bootable, so the caller stays in the USB
/// upgrade loop and a missing or corrupt app stays recoverable.
fn try_boot_app(src: &mut impl ReadAt) {
    // The bootloader's own partition holds the running image plus the PBP, not an
    // application, so it is skipped.
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
        if let Err(err) = system::load(src, &loader) {
            debug!("app-bootloader: app copy failed: {err:?}, staying in upgrade mode");
            return;
        }
        unsafe { dcache_clean_invalidate_range(loader.dst as usize, loader.length as usize) };
        debug!(
            "app-bootloader: booting app {:?} @ {:#x}",
            loader.name().unwrap_or("<unnamed>"),
            loader.entry
        );
        unsafe { system::jump_to(loader.entry) }
    }

    if BOOT_APP.is_empty() {
        debug!("app-bootloader: no bootable app image, staying in upgrade mode");
    } else {
        debug!("app-bootloader: no app named {BOOT_APP:?}, staying in upgrade mode");
    }
}

/// Reset the chip for real.
///
/// AiBurn sends `reset` last, once the SPL (and app) are in flash. Jumping into
/// the PSRAM copy would skip the chain flash now describes, so reset through the
/// watchdog and come up through the BootROM - what the BOOT/RESET button does.
fn reset_board(wdog: Wdog, cmu: &mut Cmu) -> ! {
    let mut wdog = wdog.new_driver(cmu);
    // CLR < IRQ < RST: `RST` is the threshold that resets the chip. Applying a
    // scene also clears the counter, so the reset lands `rst_seconds` from now.
    wdog.configure_scene_and_apply(0, 0, 1, 2);
    loop {
        core::hint::spin_loop();
    }
}

#[app_entry("app-bootloader")]
fn app_main(_boot_param: BootParam, _private_data: &[u8]) {
    // Deliberately NOT calling `check_startup()`.
    //
    // This image is both the RAM payload the host `EXEC`s while the BOOT key is
    // *held* and the flash-resident bootloader. `check_startup()` probes PA0 as the
    // low-active key and, when low, jumps back into the BootROM upgrade entry -
    // which while the key is held bounces the freshly downloaded loader back into
    // the ROM, dropping it off the bus mid-burn. The key is handled one stage
    // earlier by `pbp-common`'s `upg_requested`, which can tell a host-driven start
    // apart from a flash one (this image cannot: `_start` clobbers `a0`).
    let mut p = Peripherals::take();

    // A second stage still needs its own console: the first stage's logger lives
    // in a different image. UART0 (PA0/PA1), 115200 8N1.
    let uart0_tx = p.gpioa.pa0.into_uart0_tx();
    let uart0_rx = p.gpioa.pa1.into_uart0_rx();
    // A failed `set_logger` must not kill the loader and cannot be reported (the
    // panic handler logs through the same logger), so `.ok()`.
    let _uart0 = uart_logger_init(
        p.uart0,
        uart0_tx,
        uart0_rx,
        UartConfig::default(),
        &mut p.cmu,
    )
    .ok();

    let sector_buf = unsafe { &mut *core::ptr::addr_of_mut!(SECTOR_BUF) };
    let scratch = unsafe { &mut *core::ptr::addr_of_mut!(SCRATCH) };

    // QSPI0 on D13x: SCK PB4, MOSI PB5, MISO PB1, CS PB2.
    //
    // The stage before differs by role: as the flash-resident SPL `pbp-common`
    // configured QSPI0, but as the RAM updater the vendor PBP reports the USB boot
    // device and never touches the NOR. So initialize here rather than attach: a
    // controller left by someone else cannot be validated, and a wrong config
    // fails silently in the NOR ID read.
    let sck = p.gpiob.pb4.into_qspi0_sck();
    let miso = p.gpiob.pb1.into_qspi0_miso();
    let mosi = p.gpiob.pb5.into_qspi0_mosi();
    let cs = p.gpiob.pb2.into_pull_up_output();
    let pad = (
        sck,
        Some(mosi),
        Some(miso),
        None::<NoPad>,
        None::<NoPad>,
        None::<NoPad>,
    );
    let qspi0 = p
        .qspi0
        .new_blocking(pad, QspiConfig::nor_flash(), &mut p.cmu);
    let delay = p.gtc.new_timer_delay(CntFreq::Freq4M, &mut p.cmu);
    // Second handle to the same counter, kept for the `Boot` role's host-wait
    // window while `delay` itself goes to the NOR driver.
    let tick = delay;
    let mut nor = match W25QXXXJV::new(qspi0, cs, SpiSpeed::Single, Model::Q128) {
        Ok(flash) => flash,
        Err(_) => {
            debug!("app-bootloader: SPI-NOR init failed, halting");
            loop {
                core::hint::spin_loop();
            }
        }
    };
    // Prove the QSPI link works before touching the protocol. The IDs are folded
    // into the one storage line below; a failure gets its own message there,
    // because that is the only case where they are worth reading.
    let id = nor.manufacture_device_id();
    let uid = nor.read_unique_id();

    let flash = NorFlashDelay::new(nor, delay);
    let mut storage = NorStorage::new(flash, &mut sector_buf.0, &mut scratch.0, FLASH_LAYOUT);

    let (scratch_addr, scratch_len, boot_stage) = (
        storage.scratch_addr(),
        storage.scratch_len(),
        storage.boot_stage(),
    );
    match (id, uid) {
        (Ok(id), Ok(uid)) => debug!(
            "app-bootloader: SPI-NOR mfr=0x{:02x} dev=0x{:02x} uid=0x{:016x}, scratch={scratch_addr:#x} ({scratch_len} B), boot_stage={boot_stage}",
            id[0], id[1], uid
        ),
        (id, uid) => {
            if let Err(err) = id {
                debug!("app-bootloader: SPI-NOR ID read failed: {err:?}");
            }
            if let Err(err) = uid {
                debug!("app-bootloader: SPI-NOR UID read failed: {err:?}");
            }
            debug!(
                "app-bootloader: scratch={scratch_addr:#x} ({scratch_len} B), boot_stage={boot_stage}"
            );
        }
    }
    let mut engine = Engine::new(&mut storage, PARTITION_TABLE);

    let dm = p.gpiou.pu0.into_usb0_dm();
    let dp = p.gpiou.pu1.into_usb0_dp();
    let mut upg = match p.usb_dev.new_async_aic_upg(
        (dm, dp),
        UsbConfig::default(),
        &mut p.cmu,
        &mut p.syscfg,
        Irqs,
    ) {
        Ok(upg) => upg,
        Err(err) => {
            debug!("app-bootloader: USB controller init FAILED: {:?}", err);
            loop {
                core::hint::spin_loop();
            }
        }
    };

    // The role comes from the stage that started us (`system::Role`) and decides
    // what happens when the window runs out. Both roles use the same window.
    //
    // * `Upgrade`: a host `EXEC`ed us; the clock stops for good once it sends a
    //   state-changing command, while a mere probe keeps it running so an open tool
    //   cannot pin the board here forever.
    // * `Boot`: a flash start, so the deadline is **final** - a host that happens
    //   to be attached must not keep the board in upgrade mode.
    let role = system::role();
    debug!("app-bootloader: role {role:?}");
    let deadline = tick.get_tick() + HOST_DELAY_MS as u64 * TICKS_PER_MS;
    if role == Role::Upgrade {
        block_on(upg.run(&mut engine, |engine| {
            !engine.host_upgrading() && tick.get_tick() >= deadline
        }));

        if engine.reset_requested() {
            debug!("app-bootloader: host asked to reset, resetting the board");
            reset_board(p.wdog, &mut p.cmu);
        }
    } else {
        block_on(upg.run(&mut engine, |_| tick.get_tick() >= deadline));
    }
    debug!("app-bootloader: no host within {HOST_DELAY_MS} ms, normal boot");

    // Nobody talked for the whole window: drop the USB pull-up so nothing keeps
    // seeing an upgrade device that does not answer, then boot the application.
    drop(engine);
    upg.disconnect();
    try_boot_app(&mut StorageRead(&mut storage));

    // Nothing bootable: stay in the upgrade loop (no timeout) so the board can
    // still be programmed over USB.
    debug!("app-bootloader: nothing to boot, staying in upgrade mode");
    let mut engine = Engine::new(&mut storage, PARTITION_TABLE);
    block_on(upg.run(&mut engine, |_| false));
}
