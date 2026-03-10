#![no_std]
#![no_main]

use artinchip_hal::gpio::*;
use artinchip_hal::gtc::*;
use artinchip_hal::prelude::*;
use artinchip_hal::qspi::*;
use artinchip_hal::uart::*;
use artinchip_rt::{Peripherals, pbp_entry};
use embedded_io::Write;
use panic_halt as _;

use w25qxxxjv::{Model, SpiSpeed, W25QXXXJV};

#[pbp_entry]
fn pbp_main(_boot_param: u32, _private_data: &[u8]) {
    let p = Peripherals::take();

    let tx = p.gpioa.pa0.into_function::<5>();
    let rx = p.gpioa.pa1.into_function::<5>();
    let mut uart0 = p.uart0.new_blocking(tx, rx, UartConfig::default(), &p.cmu);

    let mut led = p.gpioa.pa5.into_pull_up_output();
    let mut delay = p.gtc.new_timer_delay(CntFreq::Freq4M, &p.cmu);

    let sck = p.gpiob.pb4.into_function::<2>();
    let miso = p.gpiob.pb1.into_function::<2>();
    let mosi = p.gpiob.pb5.into_function::<2>();

    let cs = p.gpiob.pb2.into_pull_up_output();

    writeln!(uart0, "Welcome to pbp flash example by artinchip-hal🦀!").ok();

    let qspi0 = p.qspi0.new_blocking(
        sck,
        Some(mosi),
        Some(miso),
        None::<NoPad>,
        None::<NoPad>,
        None::<NoPad>,
        QspiConfig::nor_flash(),
        &p.cmu,
    );

    writeln!(uart0, "QSPI initialized").ok();

    let mut flash = match W25QXXXJV::new(qspi0, cs, SpiSpeed::Single, Model::Q128) {
        Ok(flash) => flash,
        Err(e) => {
            writeln!(uart0, "ERROR: Failed to initialize W25Q driver: {:?}", e).ok();
            loop {
                led.toggle().ok();
                delay.delay_ms(200);
            }
        }
    };

    writeln!(uart0, "W25Q driver created").ok();
    delay.delay_ms(100);

    match flash.manufacture_device_id() {
        Ok(id) => {
            writeln!(uart0, "Manufacturer ID: 0x{}", id[0]).ok();
            writeln!(uart0, "Device ID:       0x{}", id[1]).ok();
        }
        Err(e) => {
            writeln!(uart0, "ERROR: Failed to read ID: {:?}", e).ok();
        }
    }

    match flash.read_unique_id() {
        Ok(uid) => {
            writeln!(uart0, "Unique ID: 0x{:016X}", uid).ok();
        }
        Err(e) => {
            writeln!(uart0, "ERROR: Failed to read Unique ID: {:?}", e).ok();
        }
    }

    let mut read_buf = [0u8; 64];

    // Read first 64 bytes from flash
    match flash.read_data(0x0000, &mut read_buf) {
        Ok(_) => {
            writeln!(uart0, "First 64 bytes from flash:").ok();

            // Print 64 bytes, 16 bytes per line
            for i in 0..4 {
                write!(uart0, "{:04X}: ", i * 16).ok();
                for j in 0..16 {
                    write!(uart0, "{:02X} ", read_buf[i * 16 + j]).ok();
                }
                writeln!(uart0, "").ok();
            }

            // Check magic number
            let mut magic_num = [0u8; 4];
            match flash.read_data(0x0, &mut magic_num) {
                Ok(_) => {
                    writeln!(uart0, "Magic number1(expected: \"AIC \"):").ok();
                    for i in 0..4 {
                        write!(uart0, "{}", magic_num[i] as char).ok();
                    }
                    writeln!(uart0, "").ok();
                }
                Err(e) => {
                    writeln!(uart0, "ERROR: Failed to read magic number1: {:?}", e).ok();
                }
            }
            match flash.read_data(0xFF, &mut magic_num) {
                Ok(_) => {
                    writeln!(uart0, "Magic number2(expected: \"PBP \"):").ok();
                    for i in 0..4 {
                        write!(uart0, "{}", magic_num[i] as char).ok();
                    }
                    writeln!(uart0, "").ok();
                }
                Err(e) => {
                    writeln!(uart0, "ERROR: Failed to read magic number2: {:?}", e).ok();
                }
            }
        }
        Err(e) => {
            writeln!(uart0, "ERROR: Failed to read flash: {:?}", e).ok();
        }
    }

    writeln!(uart0, "Tests complete").ok();

    loop {
        led.toggle().ok();
        delay.delay_ms(500);
    }
}
