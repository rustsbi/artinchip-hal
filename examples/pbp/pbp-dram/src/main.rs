#![no_std]
#![no_main]

use artinchip_hal::dram::{self, DramConfig};
use artinchip_hal::prelude::*;
use artinchip_hal::uart::*;
use artinchip_rt::{Peripherals, default_clk_tree, pbp_entry, prelude::*};
use log::{error, info};
use panic_halt as _;

#[pbp_entry]
fn pbp_main(boot_param: BootParam, _private_data: &[u8]) {
    check_startup(&boot_param);
    let mut p = Peripherals::take();
    let clocks = default_clk_tree();
    clocks.configure_plls(&mut p.cmu);

    let tx = p.gpioa.pa0.into_uart0_tx();
    let rx = p.gpioa.pa1.into_uart0_rx();
    let _console =
        uart_logger_init(p.uart0, tx, rx, UartConfig::default(), &clocks, &mut p.cmu).unwrap();

    info!("Welcome to pbp dram example by artinchip-hal🦀!");

    let detected = dram::detect(&mut p.cmu, &mut p.sid);
    let config = DramConfig::for_detected(&detected);
    if detected.dram_type.is_some() {
        info!(
            "Using {} {} MiB info in efuse",
            config.dram_type.name(),
            config.memsize_mib()
        );
    } else {
        info!(
            "No DRAM info in efuse, using {} {} MiB",
            config.dram_type.name(),
            config.memsize_mib()
        );
    }

    match p
        .dram
        .new_driver(&mut p.cmu, &mut p.syscfg, &mut p.sid, &config, &clocks)
    {
        Ok(mut driver) => {
            // DDR PHY training must run at the BootROM clock; switch the CPU
            // only after the training/probe completed.
            clocks.switch_cpu_to_pll_int0(&mut p.cmu);
            if driver.probe().is_ok() {
                // The 1 MiB window first (fast verification), then the full memory window
                for &len in &[1024 * 1024, config.memsize as usize] {
                    let ok = pattern_test(driver.dram_base(), len);
                    info!("Pattern test {} bytes -> {}", len, ok);
                }
            } else {
                error!("DRAM window probe failed");
            }
        }
        Err(e) => {
            error!("DRAM init failed: {e}");
        }
    }

    info!("Done");
    loop {
        core::hint::spin_loop();
    }
}

/// Sweep `bytes` of DRAM with an address-dependent pattern.
fn pattern_test(base: usize, bytes: usize) -> bool {
    let words = bytes / 4;

    for index in 0..words {
        let address = base + index * 4;
        unsafe { core::ptr::write_volatile(address as *mut u32, index as u32) };
    }

    for index in 0..words {
        let address = base + index * 4;
        let got = unsafe { core::ptr::read_volatile(address as *const u32) };
        if got != index as u32 {
            error!(
                "DRAM: Memory test mismatch at 0x{address:08X}: wrote 0x{index:08X}, read 0x{got:08X}"
            );
            return false;
        }
    }
    true
}
