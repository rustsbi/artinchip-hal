#![no_std]
#![no_main]

use artinchip_hal::gtc::*;
use artinchip_hal::prelude::*;
use artinchip_hal::qspi::*;
use artinchip_hal::uart::*;
use artinchip_rt::{Peripherals, default_clk_tree, pbp_entry, prelude::*};
use embedded_io::Write;
use log::{error, info};
use panic_halt as _;

use w25qxxxjv::{Model, SpiSpeed, W25QXXXJV};

#[pbp_entry]
fn pbp_main(boot_param: BootParam, _private_data: &[u8]) {
    check_startup(&boot_param);
    let mut p = Peripherals::take();
    let clocks = default_clk_tree();
    clocks.configure_plls(&mut p.cmu);

    let tx = p.gpioa.pa0.into_uart0_tx();
    let rx = p.gpioa.pa1.into_uart0_rx();
    let mut uart0 =
        uart_logger_init(p.uart0, tx, rx, UartConfig::default(), &clocks, &mut p.cmu).unwrap();

    let mut led = p.gpioa.pa5.into_pull_up_output();
    let mut delay = p.gtc.new_timer_delay(CntFreq::Freq4M, &mut p.cmu);

    let sck = p.gpiob.pb4.into_qspi0_sck();
    let miso = p.gpiob.pb1.into_qspi0_miso();
    let mosi = p.gpiob.pb5.into_qspi0_mosi();

    let cs = p.gpiob.pb2.into_pull_up_output();
    let qspi0_pad = (
        sck,
        Some(mosi),
        Some(miso),
        None::<NoPad>,
        None::<NoPad>,
        None::<NoPad>,
    );

    info!("Welcome to pbp flash example by artinchip-hal🦀!");

    let qspi0 = p
        .qspi0
        .new_blocking(qspi0_pad, QspiConfig::nor_flash(), &clocks, &mut p.cmu);

    info!("QSPI initialized");

    let mut flash = match W25QXXXJV::new(qspi0, cs, SpiSpeed::Single, Model::Q128) {
        Ok(flash) => flash,
        Err(e) => {
            error!("Failed to initialize W25Q driver: {:?}", e);
            loop {
                led.toggle().ok();
                delay.delay_ms(200);
            }
        }
    };

    info!("W25Q driver created");
    delay.delay_ms(100);

    match flash.manufacture_device_id() {
        Ok(id) => {
            info!("Manufacturer ID: 0x{:02X}", id[0]);
            info!("Device ID:       0x{:02X}", id[1]);
        }
        Err(e) => {
            error!("Failed to read ID: {:?}", e);
        }
    }

    match flash.read_unique_id() {
        Ok(uid) => {
            info!("Unique ID: 0x{:016X}", uid);
        }
        Err(e) => {
            error!("Failed to read Unique ID: {:?}", e);
        }
    }

    let mut read_buf = [0u8; 64];

    // Read first 64 bytes from flash
    match flash.read_data(0x0000, &mut read_buf) {
        Ok(_) => {
            info!("First 64 bytes from flash:");

            // Print 64 bytes, 16 bytes per line
            for i in 0..4 {
                write!(uart0, "{:04X}: ", i * 16).ok();
                for j in 0..16 {
                    write!(uart0, "{:02X} ", read_buf[i * 16 + j]).ok();
                }
                writeln!(uart0).ok();
            }

            // Check magic number
            let mut magic_num = [0u8; 4];
            match flash.read_data(0x0, &mut magic_num) {
                Ok(_) => {
                    info!("Magic number1(expected: \"AIC \"):");
                    write!(uart0, "\"").ok();
                    for byte in magic_num {
                        write!(uart0, "{}", byte as char).ok();
                    }
                    writeln!(uart0, "\"").ok();
                }
                Err(e) => {
                    error!("Failed to read magic number1: {:?}", e);
                }
            }
            match flash.read_data(0x100, &mut magic_num) {
                Ok(_) => {
                    info!("Magic number2(expected: \"PBP \"):");
                    write!(uart0, "\"").ok();
                    for byte in magic_num {
                        write!(uart0, "{}", byte as char).ok();
                    }
                    writeln!(uart0, "\"").ok();
                }
                Err(e) => {
                    error!("Failed to read magic number2: {:?}", e);
                }
            }
        }
        Err(e) => {
            error!("Failed to read flash: {:?}", e);
        }
    }

    info!("Tests complete");

    loop {
        led.toggle().ok();
        delay.delay_ms(500);
    }
}
