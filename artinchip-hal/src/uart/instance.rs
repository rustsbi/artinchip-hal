//! UART instance.

use super::blocking::BlockingSerial;
use super::config::UartConfig;
use super::pad::{Receive, Transmit, UartPad};
use super::register::RegisterBlock;
use super::uart_ext::UartExt;
use crate::cmu::{ClockTree, Cmu};
use core::marker::PhantomData;
#[cfg(feature = "interrupts")]
use {
    super::non_blocking::*,
    crate::interrupt::{InterruptSource as _, typelevel},
};

/// Trait mapping const generic `I` to its type-level interrupt source.
#[cfg(feature = "interrupts")]
pub trait UartInterrupt<const I: u8> {
    type Interrupt: typelevel::Interrupt;
}

#[cfg(feature = "interrupts")]
macro_rules! impl_uart_interrupts {
    ( $( ($inst:literal, $irq_type:ident) ),* $(,)? ) => {
        $(
            impl UartInterrupt<$inst> for Uart<$inst> {
                type Interrupt = crate::interrupt::typelevel::$irq_type;
            }
        )*
    };
}

#[cfg(feature = "interrupts")]
impl_uart_interrupts! {
    (0, UART0),
    (1, UART1),
    (2, UART2),
    (3, UART3),
}
#[cfg(all(feature = "interrupts", not(feature = "d12x")))]
impl_uart_interrupts! {
    (4, UART4),
    (5, UART5),
    (6, UART6),
    (7, UART7),
}

/// UART with statically known instance number.
pub struct Uart<const I: u8> {
    reg: *const RegisterBlock,
    _private: PhantomData<()>,
}

impl<const I: u8> Uart<I> {
    /// Create a new UART instance.
    pub const fn __new(reg: *const RegisterBlock) -> Self {
        Self {
            reg,
            _private: PhantomData,
        }
    }

    /// Get a reference to the register block.
    pub const fn register_block(&self) -> &'static RegisterBlock {
        unsafe { &*self.reg }
    }

    /// Get register block for a specific index (used by the async interrupt handler).
    #[cfg(feature = "interrupts")]
    #[inline(always)]
    pub(crate) unsafe fn regs_at_index() -> &'static RegisterBlock {
        let base_addr = 0x18710000 + (I as usize) * 0x1000;

        unsafe { &*(base_addr as *const RegisterBlock) }
    }
}

impl<const I: u8> UartExt<'static, I> for Uart<I> {
    #[inline]
    fn new_blocking<TX, RX>(
        self,
        tx: TX,
        rx: RX,
        config: UartConfig,
        clock_tree: &ClockTree,
        cmu: &mut Cmu,
    ) -> BlockingSerial<'static, I, TX, RX>
    where
        TX: UartPad<I> + Transmit<I>,
        RX: UartPad<I> + Receive<I>,
    {
        BlockingSerial::new(self.register_block(), tx, rx, config, clock_tree, cmu)
    }

    #[cfg(feature = "interrupts")]
    #[inline]
    fn new_async<TX, RX, IRQS>(
        self,
        tx: TX,
        rx: RX,
        config: UartConfig,
        clock_tree: &ClockTree,
        cmu: &mut Cmu,
        _irqs: IRQS,
    ) -> AsyncSerial<'static, I, TX, RX>
    where
        Self: Sized + UartInterrupt<I>,
        TX: UartPad<I> + Transmit<I>,
        RX: UartPad<I> + Receive<I>,
        Uart<I>: UartInterrupt<I>,
        AsyncUartHandler<I>: typelevel::Handler<<Uart<I> as UartInterrupt<I>>::Interrupt>,
        IRQS: typelevel::Binding<<Uart<I> as UartInterrupt<I>>::Interrupt, AsyncUartHandler<I>>,
    {
        <Uart<I> as UartInterrupt<I>>::Interrupt::disable();
        IRQS::bind();
        let serial = AsyncSerial::new(self.register_block(), tx, rx, config, clock_tree, cmu);
        <Uart<I> as UartInterrupt<I>>::Interrupt::enable();
        serial
    }
}
