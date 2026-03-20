#![no_std]
#![no_main]

use artinchip_hal::gtc::CntFreq;
use artinchip_hal::prelude::*;
use artinchip_hal::uart::*;
use artinchip_hal::wdog::RegWrMode;
use artinchip_rt::{Peripherals, pbp_entry};
use embedded_io::Write;
use panic_halt as _;

#[pbp_entry]
fn pbp_main(_boot_param: u32, _private_data: &[u8]) {
    let mut p = Peripherals::take();
    let tx = p.gpioa.pa0.into_uart0_tx();
    let rx = p.gpioa.pa1.into_uart0_rx();

    let mut uart0 = p
        .uart0
        .new_blocking(tx, rx, UartConfig::default(), &mut p.cmu);
    let mut delay = p.gtc.new_timer_delay(CntFreq::Freq4M, &mut p.cmu);

    let reset_info = p.wri.new_reset_info();
    let time = p.rtc.new_real_time(&mut p.cmu);
    let mut wdog = p.wdog.new_driver(&mut p.cmu);

    wdog.op_wr_en();
    wdog.set_thd(0, 12, 14, 16);
    wdog.set_wr_mode(RegWrMode::WriteProtect);
    wdog.op_cfg_sw(0);

    writeln!(
        uart0,
        "Welcome to pbp boot info example by artinchip-hal🦀!"
    )
    .ok();

    writeln!(uart0, "Reset reason: {:?}", reset_info.reason(),).ok();
    writeln!(uart0, "Watchdog active channel: {}", wdog.channel_id()).ok();
    writeln!(uart0, "Watchdog write mode: {:?}", wdog.wr_mode()).ok();
    writeln!(uart0, "Watchdog thresholds:").ok();
    writeln!(uart0, "  Clear threshold: {}", wdog.thd(0).0).ok();
    writeln!(uart0, "  IRQ threshold: {}", wdog.thd(0).1).ok();
    writeln!(uart0, "  Reset threshold: {}", wdog.thd(0).2).ok();

    loop {
        writeln!(uart0, "Current time: {} seconds", time.time()).ok();
        delay.delay_ms(2000);
    }
}
