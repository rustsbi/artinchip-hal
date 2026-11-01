#![no_std]
#![no_main]

use artinchip_hal::gtc::CntFreq;
use artinchip_hal::prelude::*;
use artinchip_hal::uart::*;
use artinchip_rt::{Peripherals, default_clk_tree, pbp_entry, prelude::*};
use log::info;
use panic_halt as _;

#[pbp_entry]
fn pbp_main(boot_param: BootParam, _private_data: &[u8]) {
    check_startup(&boot_param);
    let mut p = Peripherals::take();
    let clocks = default_clk_tree();
    clocks.configure_plls(&mut p.cmu);

    let mut delay = p.gtc.new_timer_delay(CntFreq::Freq4M, &mut p.cmu);

    let _uart = {
        let tx = p.gpioa.pa0.into_uart0_tx();
        let rx = p.gpioa.pa1.into_uart0_rx();
        uart_logger_init(p.uart0, tx, rx, UartConfig::default(), &clocks, &mut p.cmu).unwrap()
    };
    let mut led = p.gpioa.pa5.into_pull_up_output();

    info!("Welcome to pbp blinky example by artinchip-hal🦀!");

    loop {
        led.toggle().ok();
        delay.delay_ms(500);
    }
}
