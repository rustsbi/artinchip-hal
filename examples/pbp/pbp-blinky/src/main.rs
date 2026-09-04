#![no_std]
#![no_main]

use artinchip_hal::gtc::CntFreq;
use artinchip_hal::prelude::*;
use artinchip_hal::uart::*;
use artinchip_rt::{Peripherals, pbp_entry, prelude::*};
use log::info;
use panic_halt as _;

#[pbp_entry]
fn pbp_main(boot_param: BootParam, _private_data: &[u8]) {
    check_startup(&boot_param);

    let mut p = Peripherals::take();
    let mut delay = p.gtc.new_timer_delay(CntFreq::Freq4M, &mut p.cmu);

    #[cfg(not(feature = "d21x"))]
    let (mut led, _uart) = {
        let tx = p.gpioa.pa0.into_uart0_tx();
        let rx = p.gpioa.pa1.into_uart0_rx();
        let uart = uart_logger_init(p.uart0, tx, rx, UartConfig::default(), &mut p.cmu).unwrap();
        (p.gpioa.pa5.into_pull_up_output(), uart)
    };

    #[cfg(feature = "d21x")]
    let (mut led, _uart) = {
        let tx = p.gpiod.pd6.into_uart1_tx();
        let rx = p.gpiod.pd7.into_uart1_rx();
        let uart = uart_logger_init(p.uart1, tx, rx, UartConfig::default(), &mut p.cmu).unwrap();
        (p.gpioe.pe15.into_pull_up_output(), uart)
    };

    info!("Welcome to pbp blinky example by artinchip-hal🦀!");

    loop {
        led.toggle().ok();
        delay.delay_ms(500);
    }
}
