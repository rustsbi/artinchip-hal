//! D21x first-stage PBP: DRAM bring-up and bootloader hand-over.

mod ddr;
mod nand;

use artinchip_hal::system::{self, Ram, Role};
use artinchip_hal::uart::{UartConfig, uart_logger_init};
use artinchip_rt::{DRAM_BASE, Peripherals, default_clk_tree, prelude::*};
use log::{debug, error};

use crate::layout::{BOOTLOADER_APP_NAME, UPG_BOOTLOADER_SLOT};

/// Maximum 1 GiB DRAM decode window used when DRAM was already initialized by an earlier call.
const DRAM_WINDOW: u32 = 0x4000_0000;

pub fn run(boot_param: &BootParam, private_data: &[u8]) {
    let upg = upg_requested(boot_param);

    let mut p = Peripherals::take();

    let from_host = boot_param.boot_device() == BootDevice::Usb;
    let clocks = default_clk_tree();
    if !from_host {
        clocks.configure_plls(&mut p.cmu);
    }

    let dram_up = p.dram.is_up(&p.syscfg);

    let tx = p.gpioa.pa0.into_uart0_tx();
    let rx = p.gpioa.pa1.into_uart0_rx();
    let _console =
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
    debug!("NAND flash, {data}, {transport}");

    if upg && !from_host {
        debug!("BOOT key held, entering ROM upgrade mode");
        if !unsafe { enter_upg_mode() } {
            error!("Unknown BootROM revision, cannot enter ROM upgrade mode");
        }
    }

    if from_host && dram_up {
        let mut ram = unsafe { Ram::new() };
        match system::resolve(&mut ram, UPG_BOOTLOADER_SLOT) {
            Ok(loader) if loader.name() == Some(BOOTLOADER_APP_NAME) => {
                if !loader.fits_window(DRAM_BASE as u32, DRAM_WINDOW) {
                    error!(
                        "RAM container names dst 0x{:08X} + 0x{:X} / entry 0x{:08X} outside DRAM",
                        loader.dst, loader.length, loader.entry
                    );
                    system::halt();
                }

                system::set_role(Role::Upgrade);
                match unsafe {
                    system::boot_loader(
                        &mut ram,
                        &loader,
                        boot_param.as_raw(),
                        private_data.as_ptr(),
                        private_data.len() as u32,
                    )
                } {
                    Err(e) => {
                        error!("Failed to boot RAM container: {e}");
                        system::halt();
                    }
                    // `boot_loader` only returns `Err`: success jumps away.
                    Ok(never) => match never {},
                }
            }
            Ok(_) => {
                error!("RAM container was parsed, but payload was not the bootloader");
            }
            Err(e) => {
                error!("Failed to parse RAM container loaded by host: {e}");
            }
        }
    }

    let dram_bytes = if dram_up {
        debug!("DRAM already initialized (e.g. by BootROM or host); keeping current config");
        DRAM_WINDOW
    } else {
        ddr::bring_up(p.dram, &mut p.cmu, &mut p.syscfg, &mut p.sid, &clocks)
    };

    // DRAM training needs the BootROM clock; switch the CPU to PLL_INT0 only
    // afterwards. A host-driven start keeps the BootROM clock for USB hand-over.
    if !from_host {
        clocks.switch_cpu_to_pll_int0(&mut p.cmu);
    }

    nand::boot_flash(
        p.qspi0,
        p.gpiob,
        p.gtc,
        #[cfg(feature = "dma")]
        p.dma,
        &clocks,
        &mut p.cmu,
        boot_param,
        private_data,
        from_host,
        dram_bytes,
    )
}
