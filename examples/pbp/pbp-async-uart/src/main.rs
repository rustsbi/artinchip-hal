#![no_std]
#![no_main]

use artinchip_hal::clic_bind_interrupts;
use artinchip_hal::prelude::*;
use artinchip_hal::uart::*;
use artinchip_rt::{Peripherals, pbp_entry};
use core::fmt::Write as _;
use embassy_futures::block_on;
use embedded_io_async::Write as _;
use heapless::String;
use panic_halt as _;

clic_bind_interrupts!(struct Irqs {
    UART0 => AsyncUartHandler<0>;
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
fn pbp_main(_boot_param: u32, _private_data: &[u8]) {
    let mut p = Peripherals::take();

    unsafe {
        Irqs::init_clic_and_interrupts();
    }

    let uart0_tx = p.gpioa.pa0.into_uart0_tx();
    let uart0_rx = p.gpioa.pa1.into_uart0_rx();
    let mut uart0_async =
        p.uart0
            .new_async(uart0_tx, uart0_rx, UartConfig::default(), &mut p.cmu, Irqs);

    block_on(async {
        let mut buf: String<128> = String::new();

        writeln!(buf, "Welcome to pbp async uart example by artinchip-hal🦀!").ok();
        uart0_async.write_all(buf.as_bytes()).await.ok();

        buf.clear();
        writeln!(buf, "The following is a test message sent via async UART.").ok();
        uart0_async.write_all(buf.as_bytes()).await.ok();

        uart0_async.write_all(TEST_MSG.as_bytes()).await.ok();

        uart0_async.flush().await.ok();
    });

    loop {
        core::hint::spin_loop();
    }
}
