#![no_std]
#![no_main]

use artinchip_hal::prelude::*;
use artinchip_hal::uart::*;
use artinchip_rt::{Peripherals, pbp_entry};
use embedded_io::Write;
use embedded_time::rate::Baud;
use panic_halt as _;

#[pbp_entry]
fn pbp_main(_boot_param: u32, _private_data: &[u8]) {
    let p = Peripherals::take();
    let tx = p.gpioa.pa0.into_function::<5>();
    let rx = p.gpioa.pa1.into_function::<5>();
    let mut pa5 = p.gpioa.pa5.into_pull_up_input();

    let mut uart0 = p.uart0.new_blocking(
        tx,
        rx,
        UartConfig {
            baud_rate: Baud(115200_u32),
            data_bits: DataBits::Eight,
            stop_bits: StopBits::One,
            parity: Parity::None,
        },
        &p.cmu,
    );

    writeln!(
        uart0,
        "Welcome to pbp hello world example by artinchip-hal🦀!"
    )
    .ok();
    loop {
        if pa5.is_low().unwrap() {
            writeln!(uart0, "Button pressed!").ok();
            while pa5.is_low().unwrap() {
                // wait for button to release
                core::hint::spin_loop();
            }
        }
    }
}
