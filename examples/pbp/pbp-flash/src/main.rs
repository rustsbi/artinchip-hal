//! Boot-flash configuration report and single/quad + FIFO/DMA verification example.
#![no_std]
#![no_main]

#[cfg(all(feature = "nor", feature = "nand"))]
compile_error!("select at most one medium feature: `nor` or `nand`");
#[cfg(not(any(feature = "nor", feature = "nand")))]
compile_error!("select a medium feature: `nor` or `nand`");

#[cfg(feature = "nand")]
mod nand;
#[cfg(feature = "nor")]
mod nor;

use artinchip_hal::gtc::CntFreq;
use artinchip_hal::prelude::*;
use artinchip_hal::uart::*;
use artinchip_rt::{Peripherals, default_clk_tree, pbp_entry, prelude::*};
use log::info;
use panic_halt as _;

/// 16-byte aligned buffer required by GPDMA transfers.
#[cfg(any(feature = "quad", feature = "dma"))]
#[repr(align(16))]
pub(crate) struct DmaBuffer<const N: usize>(pub(crate) [u8; N]);

#[pbp_entry]
fn pbp_main(boot_param: BootParam, _private_data: &[u8]) {
    check_startup(&boot_param);
    let mut p = Peripherals::take();
    let clocks = default_clk_tree();
    clocks.configure_plls(&mut p.cmu);

    let mut delay = p.gtc.new_timer_delay(CntFreq::Freq4M, &mut p.cmu);

    let _console = {
        let tx = p.gpioa.pa0.into_uart0_tx();
        let rx = p.gpioa.pa1.into_uart0_rx();
        uart_logger_init(p.uart0, tx, rx, UartConfig::default(), &clocks, &mut p.cmu).unwrap()
    };
    let mut led = p.gpioa.pa5.into_pull_up_output();
    info!("Welcome to pbp flash example by artinchip-hal🦀!");

    let medium = if cfg!(feature = "nor") {
        "NOR flash"
    } else {
        "NAND flash"
    };
    let data = if cfg!(feature = "quad") {
        "QUAD data"
    } else {
        "SINGLE data"
    };
    let transport = if cfg!(feature = "dma") {
        "GPDMA mode"
    } else {
        "FIFO mode"
    };
    info!("Config: {medium}, {data}, {transport}");

    #[cfg(feature = "nor")]
    let ok = nor::run(
        p.qspi0,
        p.gpiob,
        #[cfg(feature = "dma")]
        p.dma,
        delay,
        &clocks,
        &mut p.cmu,
    );

    #[cfg(feature = "nand")]
    let ok = nand::run(
        p.qspi0,
        p.gpiob,
        #[cfg(feature = "dma")]
        p.dma,
        delay,
        &clocks,
        &mut p.cmu,
    );

    blink(&mut led, &mut delay, ok);
}

#[cfg(any(feature = "quad", feature = "dma"))]
pub(crate) fn outcome(condition: bool) -> &'static str {
    if condition { "OK" } else { "FAIL" }
}

pub(crate) fn ascii(buffer: &[u8; 4]) -> &str {
    core::str::from_utf8(buffer).unwrap_or("????")
}

fn blink<L, D>(led: &mut L, delay: &mut D, ok: bool) -> !
where
    L: embedded_hal::digital::OutputPin,
    D: embedded_hal::delay::DelayNs,
{
    let period = if ok { 500u32 } else { 200u32 };
    loop {
        led.set_high().ok();
        delay.delay_ms(period);
        led.set_low().ok();
        delay.delay_ms(period);
    }
}
