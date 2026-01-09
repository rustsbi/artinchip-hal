#![no_std]
#![no_main]

use artinchip_hal::gtc::CntFreq;
use artinchip_hal::i2c::*;
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
    let scl = p.gpioa.pa8.into_function::<4>();
    let sda = p.gpioa.pa9.into_function::<4>();

    let mut button = p.gpioa.pa5.into_pull_up_input();

    let mut touch_rst = p.gpioa.pa10.into_pull_up_output();
    let mut touch_int = p.gpioa.pa11.into_pull_up_output();

    let delay = p.gtc.new_timer_delay(CntFreq::Freq4M, &p.cmu);

    let mut uart0 = p.uart0.new_blocking(tx, rx, UartConfig::default(), &p.cmu);

    let mut i2c2 = p.i2c2.new_blocking(scl, sda, I2cConfig::default(), &p.cmu);

    writeln!(
        uart0,
        "Welcome to pbp i2c master example by artinchip-hal🦀!"
    )
    .ok();

    // Ensure rst and int pins are low
    touch_rst.set_low().ok();
    touch_int.set_low().ok();
    delay.delay_ms(10);

    // Initialize GT911 address
    writeln!(uart0, "Initializing GT911 address... ").ok();
    touch_int.set_high().ok();
    delay.delay_us(110);
    touch_rst.set_high().ok();
    delay.delay_ms(50);

    writeln!(uart0, "GT911 initialized, address is 0x14").ok();

    loop {
        if button.is_low().unwrap_or(false) {
            // Wait for button release
            while button.is_low().unwrap_or(false) {}

            writeln!(uart0, "Button pressed!  Reading ID_REG from GT911...").ok();

            let mut id_val = [0u8; 4];
            match i2c2.write_read(0x14u8, &[0x81, 0x40], &mut id_val) {
                Ok(_) => {
                    if let Ok(s) = core::str::from_utf8(&id_val) {
                        writeln!(uart0, "ID:  {}", s).ok();
                    }
                    writeln!(
                        uart0,
                        "Bytes:  {:02X} {:02X} {:02X} {:02X}",
                        id_val[0], id_val[1], id_val[2], id_val[3]
                    )
                    .ok();
                }
                Err(_) => {
                    writeln!(uart0, "Failed to read ID! ").ok();
                }
            }

            // Set pa5 to output mode and blink LED 3 times
            let pa5 = button.free();
            let mut led = pa5.into_pull_up_output();
            for _ in 0..3 {
                led.toggle().ok();
                delay.delay_ms(50);
                led.toggle().ok();
                delay.delay_ms(50);
            }

            // Set pa5 back to input mode
            let pa5 = led.free();
            button = pa5.into_pull_up_input();
        }
    }
}
