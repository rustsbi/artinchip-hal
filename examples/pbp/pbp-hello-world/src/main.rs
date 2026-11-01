#![no_std]
#![no_main]

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

    // Both boards log on UART0 (PA0/PA1).
    let _uart0 = {
        let tx = p.gpioa.pa0.into_uart0_tx();
        let rx = p.gpioa.pa1.into_uart0_rx();
        uart_logger_init(p.uart0, tx, rx, UartConfig::default(), &clocks, &mut p.cmu).unwrap()
    };

    info!("Welcome to pbp hello world example by artinchip-hal🦀!");

    loop {
        core::hint::spin_loop();
    }
}
