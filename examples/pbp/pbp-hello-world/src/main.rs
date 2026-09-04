#![no_std]
#![no_main]

use artinchip_hal::uart::*;
use artinchip_rt::{Peripherals, pbp_entry, prelude::*};
use log::info;
use panic_halt as _;

#[pbp_entry]
fn pbp_main(boot_param: BootParam, _private_data: &[u8]) {
    check_startup(&boot_param);

    let mut p = Peripherals::take();

    #[cfg(not(feature = "d21x"))]
    let _uart0 = {
        let tx = p.gpioa.pa0.into_uart0_tx();
        let rx = p.gpioa.pa1.into_uart0_rx();
        uart_logger_init(p.uart0, tx, rx, UartConfig::default(), &mut p.cmu).unwrap()
    };

    #[cfg(feature = "d21x")]
    let _uart1 = {
        let tx = p.gpiod.pd6.into_uart1_tx();
        let rx = p.gpiod.pd7.into_uart1_rx();
        uart_logger_init(p.uart1, tx, rx, UartConfig::default(), &mut p.cmu).unwrap()
    };

    info!("Welcome to pbp hello world example by artinchip-hal🦀!");

    loop {}
}
