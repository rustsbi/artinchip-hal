#![no_std]
#![no_main]

use artinchip_hal::gtc::CntFreq;
use artinchip_hal::i2c::*;
use artinchip_hal::prelude::*;
use artinchip_hal::uart::*;
use artinchip_rt::{Peripherals, default_clk_tree, pbp_entry, prelude::*};
use log::{error, info};
use panic_halt as _;

#[pbp_entry]
fn pbp_main(boot_param: BootParam, _private_data: &[u8]) {
    check_startup(&boot_param);
    let mut p = Peripherals::take();
    let clocks = default_clk_tree();
    clocks.configure_plls(&mut p.cmu);

    let tx = p.gpioa.pa0.into_uart0_tx();
    let rx = p.gpioa.pa1.into_uart0_rx();
    let scl = p.gpioa.pa8.into_i2c2_scl();
    let sda = p.gpioa.pa9.into_i2c2_sda();

    let mut touch_rst = p.gpioa.pa10.into_pull_up_output();
    let mut touch_int = p.gpioa.pa11.into_pull_up_output();

    let mut delay = p.gtc.new_timer_delay(CntFreq::Freq4M, &mut p.cmu);

    let _uart0 =
        uart_logger_init(p.uart0, tx, rx, UartConfig::default(), &clocks, &mut p.cmu).unwrap();

    let mut i2c2 = p
        .i2c2
        .new_blocking((scl, sda), I2cConfig::default(), &mut p.cmu);

    info!("Welcome to pbp i2c master example by artinchip-hal🦀!");

    // Ensure rst and int pins are low
    touch_rst.set_low().ok();
    touch_int.set_low().ok();
    delay.delay_ms(10);

    // Initialize GT911 address
    info!("Initializing GT911 address... ");
    touch_int.set_high().ok();
    delay.delay_us(110);
    touch_rst.set_high().ok();
    delay.delay_ms(50);

    info!("Trying to read GT911 ID... ");

    let mut max_tries = 5;
    let mut id_val = [0u8; 4];
    while max_tries > 0 {
        match i2c2.write_read(0x14u8, &[0x81, 0x40], &mut id_val) {
            Ok(_) => {
                if let Ok(s) = core::str::from_utf8(&id_val) {
                    info!("ID:  {}", s);
                }
                info!(
                    "Bytes:  {:02X} {:02X} {:02X} {:02X}",
                    id_val[0], id_val[1], id_val[2], id_val[3]
                );
                delay.delay_ms(100);
                break;
            }
            Err(e) => {
                error!("Failed to read GT911 ID! ({:?})", e);
                max_tries -= 1;
            }
        }
    }

    loop {}
}
