#![no_std]
#![no_main]

use artinchip_hal::gtc::CntFreq;
use artinchip_hal::prelude::*;
use artinchip_hal::uart::*;
use artinchip_rt::{Peripherals, pbp_entry};
use embedded_io::Write;
use panic_halt as _;

#[pbp_entry]
fn pbp_main(_boot_param: u32, _private_data: &[u8]) {
    let p = Peripherals::take();
    let tx = p.gpioa.pa0.into_function::<5>();
    let rx = p.gpioa.pa1.into_function::<5>();

    let mut uart0 = p.uart0.new_blocking(tx, rx, UartConfig::default(), &p.cmu);
    let delay = p.gtc.new_timer_delay(CntFreq::Freq4M, &p.cmu);

    let reset_info = p.wri.new_reset_info();
    let time = p.rtc.new_real_time(&p.cmu);

    if time.time() == 0 {
        time.set_time(3600);
    }

    writeln!(
        uart0,
        "Welcome to pbp boot info example by artinchip-hal🦀!"
    )
    .ok();

    writeln!(uart0, "Reset reason: {:?}", reset_info.reason(),).ok();

    loop {
        writeln!(uart0, "Current time: {} seconds", time.time()).ok();
        delay.delay_ms(2000);
    }
}
