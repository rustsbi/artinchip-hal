#![no_std]
#![no_main]

//! Application booted by `app-bootloader`.
//!
//! It is a plain AIC `loader` image in the `app` partition of
//! `app-bootloader.toml`; the bootloader resolves it by name and jumps here. It
//! toggles PE17 every 500 ms so it is obvious whether the boot path worked.

use artinchip_hal::gtc::CntFreq;
use artinchip_hal::prelude::*;
use artinchip_hal::uart::{UartConfig, uart_logger_init};
use artinchip_rt::{Peripherals, app_entry, prelude::*};
use log::info;
use panic_halt as _;

#[app_entry("blinky")]
fn app_main(_boot_param: BootParam, _private_data: &[u8]) {
    let mut p = Peripherals::take();

    // UART0 (PA0/PA1) so the app can say it was reached; the bootloader's logger
    // does not carry over between images.
    let tx = p.gpioa.pa0.into_uart0_tx();
    let rx = p.gpioa.pa1.into_uart0_rx();
    let _uart0 = uart_logger_init(p.uart0, tx, rx, UartConfig::default(), &mut p.cmu).ok();

    let mut delay = p.gtc.new_timer_delay(CntFreq::Freq4M, &mut p.cmu);
    let mut led = p.gpioe.pe17.into_pull_up_output();

    info!("app-blinky: toggling PE17 every 500 ms");
    loop {
        led.toggle().ok();
        delay.delay_ms(500);
    }
}
