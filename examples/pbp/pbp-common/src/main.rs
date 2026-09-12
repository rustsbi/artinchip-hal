#![no_std]
#![no_main]

//! First-stage PBP shared by every slot that needs one: brings PSRAM up when
//! needed, then either boots the bootloader or returns to its caller.
//!
//! The entry is return-capable, so the same image also serves the
//! `image.updater.psram` service call: bring PSRAM up and hand the machine back to
//! the host, which then writes `image.updater.bootloader` into PSRAM.
//!
//! Which happens is decided by the surviving XSPI state: PSRAM decoding XIP means
//! the psram run did it, so run the RAM-slot loader and do **not** bring PSRAM up
//! again (replaying it under the host's container loses it); PSRAM off is either
//! that call or a flash start, which `rom_upgrading` separates. The loader comes
//! from [`artinchip_hal::system`], RAM slot first, the `"AIC "` container at flash
//! offset 0 next.

use artinchip_hal::gtc::CntFreq;
use artinchip_hal::prelude::*;
use artinchip_hal::qspi::*;
use artinchip_hal::system::{self, Error as ImageError, ReadAt};
use artinchip_hal::uart::*;
use artinchip_hal::xspi::{Chip, PsramModel, Xspi, XspiExt};
use artinchip_rt::{Peripherals, pbp_entry, prelude::*};
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::SpiBus;
use log::{debug, error};
use panic_halt as _;
use w25qxxxjv::{Model, SpiSpeed, W25QXXXJV};

/// BootROM always reads the first-stage image from flash offset 0.
const BOOTLOADER_FLASH_BASE: u32 = 0;

/// PSRAM address the host downloads the `image.updater.bootloader` container into
/// (its `ram` field in `app-bootloader.toml`).
const UPG_BOOTLOADER_SLOT: u32 = 0x4010_0000;

/// Size of that window that has to hold up; the container is about 83 KiB.
const UPG_BOOTLOADER_WINDOW: usize = 128 * 1024;

/// Bring-up attempts allowed before giving up on the host's window.
const PSRAM_ATTEMPTS: u32 = 4;

/// App name `aicfwc` records for the bootloader container. The RAM slot is only
/// trusted when the container there is named, because fresh PSRAM is undefined.
/// Keep in sync with the component `name` in `app-bootloader.toml`.
const BOOTLOADER_APP_NAME: &str = "bootloader";

/// [`ReadAt`] adapter over the PSRAM window.
struct Ram;

impl ReadAt for Ram {
    fn read_at(&mut self, offset: u32, buf: &mut [u8]) -> Result<(), ImageError> {
        // Safety: callers only probe the PSRAM window this image just brought up.
        unsafe {
            core::ptr::copy_nonoverlapping(offset as *const u8, buf.as_mut_ptr(), buf.len());
        }
        Ok(())
    }
}

/// [`ReadAt`] adapter over the SPI-NOR driver.
struct Nor<SPI: SpiBus, CS: OutputPin>(W25QXXXJV<SPI, CS>);

impl<SPI: SpiBus, CS: OutputPin> ReadAt for Nor<SPI, CS> {
    fn read_at(&mut self, offset: u32, buf: &mut [u8]) -> Result<(), ImageError> {
        self.0.read_data(offset, buf).map_err(|_| ImageError::Read)
    }
}

/// Whether XSPI is already brought up and decoding XIP. Set at the end of the
/// PSRAM bring-up and cleared only by a chip reset, so it survives across slots.
fn psram_is_up(xspi: &Xspi) -> bool {
    let ctrl = xspi.register_block().ctrl.read();
    ctrl.is_xspi_enabled() && ctrl.is_xip_enabled()
}

/// D13x USB device controller register block.
const USB_DEV_BASE: u32 = 0x1020_0000;

/// Whether the BootROM's USB downloader is running: its module clock is gated on
/// every non-upgrade start, so a gated block reads zero (as the `pbp-usb` dump
/// measured). That is what separates the psram service call from a flash start.
fn rom_upgrading() -> bool {
    // Safety: the SoC's fixed USB device base. A gated block reads back zero
    // rather than faulting, which is the point of the probe.
    unsafe { core::ptr::read_volatile(USB_DEV_BASE as *const u32) != 0 }
}

/// Park the core; a failed first stage must not fall back into the BootROM.
fn halt() -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[pbp_entry]
fn pbp_main(boot_param: BootParam, _private_data: &[u8]) {
    // Unconditionally first: probe the BOOT key before the console claims PA0
    // (the probe repurposes the pin and restores it). `upg_requested` reports
    // `false` for a host-driven `image.updater.*` start.
    let upg = upg_requested(&boot_param);

    let mut p = Peripherals::take();

    // Which start this is: PSRAM already up, and is the ROM's downloader driving
    // us? Probed before `p.gtc` is moved out of `p`.
    let psram_up = psram_is_up(&p.xspi);
    let from_host = rom_upgrading();

    let tx = p.gpioa.pa0.into_uart0_tx();
    let rx = p.gpioa.pa1.into_uart0_rx();
    let _uart0 = uart_logger_init(p.uart0, tx, rx, UartConfig::default(), &mut p.cmu).unwrap();

    // Ask the hardware directly so a mis-decoded `boot_param` cannot break a burn:
    // `upg` and `from_host` must both agree no host is driving us. Acted on after
    // the console is up so the reason is visible; the ROM reinitializes anyway.
    if upg && !from_host {
        debug!("BOOT key held, entering ROM upgrade mode");
        // An unknown BootROM revision leaves nothing to hand over to; a normal
        // boot beats a hang.
        if !unsafe { enter_upg_mode() } {
            error!("unknown BootROM revision, cannot enter ROM upgrade mode");
        }
    }

    let mut delay = p.gtc.new_timer_delay(CntFreq::Freq4M, &mut p.cmu);

    if !psram_up {
        // Verify the window the host is about to write. The first bring-up after
        // power-up is the fragile one, and a cold phase scan can settle on a
        // marginal phase whose own pattern test passes while a large write further
        // up fails: the bootloader run then cannot find the container, falls back
        // to the flash loader and times out into the app ("the first burn jumps to
        // the app, later ones are fine"). The verify is destructive by design - the
        // host overwrites the window next and a flash start never uses it.
        debug!("Initializing PSRAM...");
        let register_block = p.xspi.register_block();
        let mut verified = false;
        for attempt in 0..PSRAM_ATTEMPTS {
            let xspi = Xspi::__new(register_block);
            match xspi.new_driver(
                Chip::D13x,
                PsramModel::Aps3208K,
                &mut delay,
                &p.syscfg,
                &p.cmu,
            ) {
                Ok(mut driver) => {
                    driver.cfg.xip_base = UPG_BOOTLOADER_SLOT as usize;
                    // Let the DLL settle before trusting the scan.
                    delay.delay_ms(2);
                    if driver.pattern_test(UPG_BOOTLOADER_WINDOW) {
                        verified = true;
                        break;
                    }
                    error!("host window verify failed (attempt {attempt})");
                }
                Err(e) => error!("PSRAM init failed: {e:?}"),
            }
            delay.delay_ms(2);
        }
        if verified {
            debug!("PSRAM init OK🦀!");
        } else {
            // Carry on: the host's write then fails visibly instead of the machine
            // being left half-configured with nobody driving it.
            error!("host window never verified");
        }

        if from_host {
            // The host's downloader asked us to make PSRAM usable and is waiting
            // for the machine back; it writes the bootloader into it next.
            return;
        }

        // Otherwise a flash start: fall through to the SPI-NOR path below.
    } else {
        // PSRAM was already up, so the psram run preceded us: this is the
        // bootloader role. The RAM slot is only consulted here - on a flash start
        // PSRAM was off and whatever is there is a stale earlier session, never a
        // reason to prefer it over the installed image.
        if let Ok(loader) = system::resolve(&mut Ram, UPG_BOOTLOADER_SLOT)
            && loader.name() == Some(BOOTLOADER_APP_NAME)
        {
            if system::load(&mut Ram, &loader).is_err() {
                error!("bootloader copy failed");
                halt();
            }
            unsafe { dcache_clean_invalidate_range(loader.dst as usize, loader.length as usize) };

            // Consume the container: PSRAM survives a warm reset, and a leftover
            // valid `"AIC "` image would make the next boot mistake it for a live
            // one. Cleaned by hand because the loader decides from PSRAM.
            unsafe {
                (UPG_BOOTLOADER_SLOT as *mut u32).write_volatile(0);
                dcache_clean_invalidate_range(UPG_BOOTLOADER_SLOT as usize, 4);
            }

            // Serve the host for as long as it takes.
            system::set_role(system::Role::Upgrade);
            debug!("Jumping to bootloader @ {:#x}", loader.entry);
            unsafe { system::jump_to(loader.entry) }
        }

        // No container: fall through to the installed image.
    }

    // SPI-NOR on QSPI0: SCK PB4 / MOSI PB5 / MISO PB1 / CS PB2.
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
    let flash = match W25QXXXJV::new(qspi0, cs, SpiSpeed::Single, Model::Q128) {
        Ok(flash) => flash,
        Err(e) => {
            error!("SPI-NOR init failed: {e:?}");
            halt();
        }
    };
    let mut nor = Nor(flash);

    let loader = match system::resolve(&mut nor, BOOTLOADER_FLASH_BASE) {
        Ok(loader) => loader,
        Err(_) => {
            error!("bootloader not found");
            halt();
        }
    };
    if system::load(&mut nor, &loader).is_err() {
        error!("bootloader copy failed");
        halt();
    }
    unsafe { dcache_clean_invalidate_range(loader.dst as usize, loader.length as usize) };

    // An idle board must not sit in upgrade mode, so the loader may time out.
    system::set_role(system::Role::Boot);
    debug!("Jumping to bootloader @ {:#x}", loader.entry);
    unsafe { system::jump_to(loader.entry) }
}
