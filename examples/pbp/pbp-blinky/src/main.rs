#![no_std]
#![no_main]

use artinchip_hal::gtc::CntFreq;
use artinchip_hal::prelude::*;
use artinchip_rt::{Peripherals, pbp_entry};
use panic_halt as _;

#[pbp_entry]
fn pbp_main(_boot_param: u32, _private_data: &[u8]) {
    let p = Peripherals::take();
    let delay = p.gtc.new_timer_delay(CntFreq::Freq4M, &p.cmu);
    let mut pa5 = p.gpioa.pa5.into_pull_up_output();

    loop {
        pa5.toggle().ok();
        delay.delay_ms(500);
    }
}
