#![no_std]
#![no_main]
#![feature(abi_riscv_interrupt)]

use artinchip_hal::clic_bind_interrupts;
use artinchip_hal::prelude::*;
use artinchip_hal::uart::*;
use artinchip_rt::{Peripherals, default_clk_tree, pbp_entry, prelude::*};
use core::fmt::Write as _;
use embassy_futures::block_on;
use embedded_io_async::Read as _;
use embedded_io_async::Write as _;
use heapless::String;
use log::info;
use panic_halt as _;

clic_bind_interrupts!(struct Irqs {
    UART3 => AsyncUartHandler<3>;
});

const TEST_MSG: &str = r#"die Ruinenstadt ist immer noch schön
ich warte lange Zeit auf deine Rückkehr
in der Hand ein Vergissmeinnicht

Sand wirbelt in die Höhe
schwarzer Wind und roter Stern
verblasste Blütenblätter
legen sich auf die Erde

Asche wirbelt in die Höhe
verwelkte Blütenblätter
werden wieder zur Erde
βίος

Regentropfen sind meine Tränen
Wind ist mein Atem und mein Erzählung
Zweige und Blätter sind meine Hände
denn mein Körper ist in Wurzeln gehüllt

wenn die Jahreszeit des Tauens kommt
werde ich wach und singe ein Lied
das Vergissmeinnicht, das du mir gegeben hast, ist hier"#;

#[pbp_entry]
fn pbp_main(boot_param: BootParam, _private_data: &[u8]) {
    check_startup(&boot_param);
    let mut p = Peripherals::take();
    let clocks = default_clk_tree();
    clocks.configure_plls(&mut p.cmu);

    let uart0_tx = p.gpioa.pa0.into_uart0_tx();
    let uart0_rx = p.gpioa.pa1.into_uart0_rx();
    let uart3_tx = p.gpioa.pa6.into_uart3_tx();
    let uart3_rx = p.gpioa.pa7.into_uart3_rx();
    let mut uart3_async = p.uart3.new_async(
        uart3_tx,
        uart3_rx,
        UartConfig::default(),
        &clocks,
        &mut p.cmu,
        Irqs,
    );
    let _uart0 = uart_logger_init(
        p.uart0,
        uart0_tx,
        uart0_rx,
        UartConfig::default(),
        &clocks,
        &mut p.cmu,
    )
    .unwrap();

    info!("Async UART example started. Please check the output on UART3.");
    info!("PA6 -> UART3_TX, PA7 -> UART3_RX");

    block_on(async {
        let mut buf: String<128> = String::new();
        let mut rx_buf = [0u8; 256];

        writeln!(buf, "Welcome to pbp async uart example by artinchip-hal🦀!").ok();
        uart3_async.write_all(buf.as_bytes()).await.ok();

        buf.clear();
        writeln!(buf, "The following is a test message sent via async UART3.").ok();
        uart3_async.write_all(buf.as_bytes()).await.ok();

        uart3_async.write_all(TEST_MSG.as_bytes()).await.ok();
        uart3_async.flush().await.ok();

        buf.clear();
        writeln!(buf, "\r\n\r\nPlease send a message and press Enter:").ok();
        uart3_async.write_all(buf.as_bytes()).await.ok();

        let read_len = uart3_async.read(&mut rx_buf).await.ok().unwrap_or(0);
        buf.clear();
        writeln!(buf, "You sent:").ok();
        uart3_async.write_all(buf.as_bytes()).await.ok();
        uart3_async.write_all(&rx_buf[..read_len]).await.ok();

        uart3_async.flush().await.ok();
    });

    info!("Async UART example completed. Please check the output on UART3.");

    loop {
        core::hint::spin_loop();
    }
}
