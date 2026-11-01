//! D13x first-stage PBP: PSRAM bring-up and SPI-NOR / PSRAM hand-over.

mod nor;
mod psram;

use artinchip_hal::gtc::CntFreq;
use artinchip_hal::prelude::*;
use artinchip_hal::system::{self, Ram, Role};
use artinchip_hal::uart::{UartConfig, uart_logger_init};
use artinchip_hal::usbdrd::device::UsbDev;
use artinchip_rt::{Peripherals, default_clk_tree, prelude::*};
use log::{debug, error};

use crate::layout::{BOOTLOADER_APP_NAME, UPG_BOOTLOADER_SLOT};

/// Detect whether the BootROM's USB downloader is clocked (host-driven service call).
fn rom_upgrading(usb_dev: &UsbDev) -> bool {
    let base = usb_dev.register_block() as *const _ as *const u32;
    unsafe { core::ptr::read_volatile(base) != 0 }
}

pub fn run(boot_param: &BootParam, private_data: &[u8]) {
    let upg = upg_requested(boot_param);

    let mut p = Peripherals::take();
    let clocks = default_clk_tree();
    clocks.configure_plls(&mut p.cmu);

    let psram_up = p.xspi.is_up();
    let from_host = rom_upgrading(&p.usb_dev);

    let tx = p.gpioa.pa0.into_uart0_tx();
    let rx = p.gpioa.pa1.into_uart0_rx();
    let _uart0 =
        uart_logger_init(p.uart0, tx, rx, UartConfig::default(), &clocks, &mut p.cmu).unwrap();

    let data = if cfg!(feature = "quad") {
        "QUAD data"
    } else {
        "SINGLE data"
    };
    let transport = if cfg!(feature = "dma") {
        "GPDMA mode"
    } else {
        "FIFO mode"
    };
    // A host-driven D13x start runs this PBP twice (psram updater, then
    // bootloader updater). Only the first round owns the flash configuration.
    if !from_host || !psram_up {
        debug!("NOR flash, {data}, {transport}");
    }

    if upg && !from_host {
        debug!("BOOT key held, entering ROM upgrade mode");
        if !unsafe { enter_upg_mode() } {
            error!("Unknown BootROM revision, cannot enter ROM upgrade mode");
        }
    }

    let mut delay = p.gtc.new_timer_delay(CntFreq::Freq4M, &mut p.cmu);

    if !psram_up {
        psram::bring_up(&p.xspi, &mut delay, &p.syscfg, &p.cmu, &clocks);
        if from_host {
            return;
        }
    } else {
        let mut ram = unsafe { Ram::new() };
        if let Ok(loader) = system::resolve(&mut ram, UPG_BOOTLOADER_SLOT)
            && loader.name() == Some(BOOTLOADER_APP_NAME)
        {
            if system::load(&mut ram, &loader).is_err() {
                error!("Bootloader copy failed");
                system::halt();
            }
            unsafe { dcache_clean_invalidate_range(loader.dst as usize, loader.length as usize) };

            // Clear the container magic so warm resets do not re-enter the RAM slot.
            unsafe {
                (UPG_BOOTLOADER_SLOT as *mut u32).write_volatile(0);
                dcache_clean_invalidate_range(UPG_BOOTLOADER_SLOT as usize, 4);
            }

            clocks.switch_cpu_to_pll_int0(&mut p.cmu);
            system::set_role(Role::Upgrade);
            debug!("Jumping to bootloader @ {:#x}", loader.entry);
            unsafe {
                system::jump_to(
                    loader.entry,
                    boot_param.as_raw(),
                    private_data.as_ptr(),
                    private_data.len() as u32,
                )
            }
        }
    }

    clocks.switch_cpu_to_pll_int0(&mut p.cmu);
    nor::boot(
        p.qspi0,
        p.gpiob,
        delay,
        #[cfg(feature = "dma")]
        p.dma,
        &clocks,
        &mut p.cmu,
        boot_param,
        private_data,
    )
}
