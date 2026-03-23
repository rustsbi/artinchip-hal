#![no_std]
#![no_main]

use artinchip_hal::prelude::*;
use artinchip_hal::pwm::*;
use artinchip_hal::uart::*;
use artinchip_rt::{Peripherals, pbp_entry};
use embedded_io::Write;
use embedded_time::rate::Hertz;
use panic_halt as _;

#[pbp_entry]
fn pbp_main(_boot_param: u32, _private_data: &[u8]) {
    let mut p = Peripherals::take();
    let tx = p.gpioa.pa0.into_uart0_tx();
    let rx = p.gpioa.pa1.into_uart0_rx();

    let mut uart0 = p
        .uart0
        .new_blocking(tx, rx, UartConfig::default(), &mut p.cmu);

    writeln!(uart0, "Welcome to pbp pwm example by artinchip-hal🦀!").ok();

    let pwm_pina = p.gpioe.pe16.into_pwm_ch3_a();
    let pwm_pinb = p.gpioe.pe17.into_pwm_ch3_b();
    let mut pwm_ch3 = p.pwm.ch3.new_driver(
        (Some(pwm_pina), Some(pwm_pinb)),
        PwmConfig::default(),
        &mut p.cmu,
    );
    pwm_ch3.set_freq_and_ratio(Hertz::new(1_000), 40.0, 20.0);
    writeln!(
        uart0,
        "PWM frequency: {} Hz, duty cycle a: {}%, duty cycle b: {}%",
        pwm_ch3.freq_and_ratio().0,
        pwm_ch3.freq_and_ratio().1,
        pwm_ch3.freq_and_ratio().2
    )
    .ok();
    writeln!(
        uart0,
        "PWM period: {} ns, duty time a: {} ns, duty time b: {} ns",
        pwm_ch3.period_and_duty().0,
        pwm_ch3.period_and_duty().1,
        pwm_ch3.period_and_duty().2
    )
    .ok();
    pwm_ch3.enable();

    loop {}
}
