//! UART extension traits.

use super::blocking::BlockingSerial;
use super::config::UartConfig;
use super::pad::UartPad;
use super::pad::{Receive, Transmit};
use crate::cmu::Cmu;
#[cfg(feature = "clic_interrupts")]
use {super::instance::*, super::non_blocking::*, crate::interrupt::clic::typelevel};

pub trait UartExt<'a, const I: u8> {
    /// Greats a blocking UART interface with the specified pads.
    fn new_blocking<TX, RX>(
        self,
        tx: TX,
        rx: RX,
        config: UartConfig,
        cmu: &mut Cmu,
    ) -> BlockingSerial<'a, I, TX, RX>
    where
        TX: UartPad<I> + Transmit<I>,
        RX: UartPad<I> + Receive<I>;
    /// Creates a non-blocking UART interface with the specified pads.
    #[cfg(feature = "clic_interrupts")]
    fn new_async<TX, RX, IRQS>(
        self,
        tx: TX,
        rx: RX,
        config: UartConfig,
        cmu: &mut Cmu,
        _irqs: IRQS,
    ) -> AsyncSerial<'a, I, TX, RX>
    where
        Self: Sized + UartInterrupt<I>,
        TX: UartPad<I> + Transmit<I>,
        RX: UartPad<I> + Receive<I>,
        Uart<I>: UartInterrupt<I>,
        AsyncUartHandler<I>: typelevel::Handler<<Uart<I> as UartInterrupt<I>>::Interrupt>,
        IRQS: typelevel::Binding<<Uart<I> as UartInterrupt<I>>::Interrupt, AsyncUartHandler<I>>;
}
