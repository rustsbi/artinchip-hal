//! Async blinky application booted by `app-bootloader`.
//!
//! Linked at the `load_address` declared in `boards/*.toml` and packed into the
//! `app` partition as an `"AIC "` loader image.
#![no_std]
#![no_main]

use artinchip_hal::prelude::*;
use artinchip_hal::uart::{UartConfig, uart_logger_init};
use artinchip_rt::embassy_time::Timer;
use artinchip_rt::{Peripherals, app_entry, default_clk_tree, prelude::*};
use embassy_futures::join::join;
use log::info;
use panic_halt as _;

#[app_entry]
async fn app_main() {
    let mut p = Peripherals::take();
    let clocks = default_clk_tree();

    let _uart = {
        let tx = p.gpioa.pa0.into_uart0_tx();
        let rx = p.gpioa.pa1.into_uart0_rx();
        uart_logger_init(p.uart0, tx, rx, UartConfig::default(), &clocks, &mut p.cmu).ok()
    };

    let led = p.gpioa.pa5.into_pull_up_output();

    info!("app-blinky: heartbeat 5 s, LED 600 ms");

    join(blink(led), heartbeat()).await;
}

async fn blink(mut led: artinchip_rt::gpio::Output<'static, 'A', 5>) {
    loop {
        led.toggle().ok();
        Timer::after_millis(600).await;
    }
}

async fn heartbeat() {
    let mut beats = 0u32;
    loop {
        Timer::after_secs(5).await;
        beats += 1;
        info!("app-blinky: heartbeat #{beats}");
    }
}
