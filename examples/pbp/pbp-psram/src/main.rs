#![no_std]
#![no_main]

use artinchip_hal::gtc::CntFreq;
use artinchip_hal::prelude::*;
use artinchip_hal::uart::*;
use artinchip_hal::xspi::{Chip, PsramModel, XspiExt};
use artinchip_rt::{Peripherals, pbp_entry, prelude::*};
use log::{error, info};
use panic_halt as _;

/// Full PSRAM XIP window exercised by the coverage test (8 MiB).
const PSRAM_WINDOW: usize = 8 * 1024 * 1024;

#[pbp_entry]
fn pbp_main(boot_param: BootParam, _private_data: &[u8]) {
    check_startup(&boot_param);

    let mut p = Peripherals::take();

    let tx = p.gpioa.pa0.into_uart0_tx();
    let rx = p.gpioa.pa1.into_uart0_rx();
    let _uart0 = uart_logger_init(p.uart0, tx, rx, UartConfig::default(), &mut p.cmu).unwrap();

    info!("Welcome to pbp psram example by artinchip-hal🦀!");

    let mut delay = p.gtc.new_timer_delay(CntFreq::Freq4M, &mut p.cmu);
    let driver = match p.xspi.new_driver(
        Chip::D13x,
        PsramModel::Aps3208K,
        &mut delay,
        &p.syscfg,
        &p.cmu,
    ) {
        Ok(driver) => driver,
        Err(e) => {
            error!("Init failed: {:?}", e);
            loop {
                delay.delay_ms(1000);
            }
        }
    };

    // The training window first (fast), then larger windows so a partial
    // decode/addressing fault shows up as a size-dependent failure.
    let training_len = driver.cfg.training_len;
    for &len in &[training_len, 1024 * 1024, PSRAM_WINDOW] {
        let ok = driver.pattern_test(len);
        info!("Pattern test {} bytes -> {}", len, ok);
    }

    info!("Done");
    loop {
        delay.delay_ms(1000);
    }
}
