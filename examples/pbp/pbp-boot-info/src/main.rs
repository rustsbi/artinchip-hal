#![no_std]
#![no_main]

use artinchip_hal::gtc::CntFreq;
use artinchip_hal::prelude::*;
use artinchip_hal::uart::*;
use artinchip_hal::wdog::RegWrMode;
use artinchip_rt::{Peripherals, default_clk_tree, pbp_entry, prelude::*};
use log::info;
use panic_halt as _;

#[pbp_entry]
fn pbp_main(boot_param: BootParam, _private_data: &[u8]) {
    check_startup(&boot_param);
    let mut p = Peripherals::take();
    let clocks = default_clk_tree();
    clocks.configure_plls(&mut p.cmu);
    let tx = p.gpioa.pa0.into_uart0_tx();
    let rx = p.gpioa.pa1.into_uart0_rx();

    let _uart0 =
        uart_logger_init(p.uart0, tx, rx, UartConfig::default(), &clocks, &mut p.cmu).unwrap();
    let mut delay = p.gtc.new_timer_delay(CntFreq::Freq4M, &mut p.cmu);

    let reset_info = p.wri.new_reset_info();
    let time = p.rtc.new_real_time(&mut p.cmu);
    let mut wdog = p.wdog.new_driver(&mut p.cmu);

    wdog.op_wr_en();
    wdog.set_thd(0, 12, 14, 16);
    wdog.set_wr_mode(RegWrMode::WriteProtect);
    wdog.op_cfg_sw(0);

    info!("Welcome to pbp boot info example by artinchip-hal🦀!");

    info!("Reset reason: {:?}", reset_info.reason());
    info!("Watchdog active channel: {}", wdog.channel_id());
    info!("Watchdog write mode: {:?}", wdog.wr_mode());
    info!("Watchdog thresholds:");
    info!("  Clear threshold: {}", wdog.thd(0).0);
    info!("  IRQ threshold: {}", wdog.thd(0).1);
    info!("  Reset threshold: {}", wdog.thd(0).2);

    loop {
        info!("Current time: {} seconds", time.time());
        delay.delay_ms(2000);
    }
}
