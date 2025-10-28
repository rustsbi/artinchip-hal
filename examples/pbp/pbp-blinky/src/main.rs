#![no_std]
#![no_main]

use artinchip_hal::prelude::*;
use artinchip_rt::{Peripherals, pbp_entry};
use panic_halt as _;
use riscv::asm::delay;

#[pbp_entry]
fn pbp_main(_boot_param: u32, _private_data: &[u8]) {
    let p = Peripherals::take();
    let mut pa5 = p.gpioa.pa5.into_output();

    loop {
        pa5.set_high().ok();
        delay(300_000);
        pa5.set_low().ok();
        delay(300_000);
    }
}
