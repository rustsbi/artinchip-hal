//! Blocking serial communication interface.

use super::config::{StopBits, UartConfig};
use super::instance::Uart;
use super::pad::{Receive, Transmit, UartPad};
use super::register::RegisterBlock;
use crate::cmu;
use uart16550::TriggerLevel;

/// Blocking serial communication interface.
pub struct BlockingSerial<'a, const I: u8, TX, RX>
where
    TX: UartPad<I> + Transmit<I>,
    RX: UartPad<I> + Receive<I>,
{
    reg: &'a RegisterBlock,
    tx: TX,
    rx: RX,
}

impl<'a, const I: u8, TX, RX> BlockingSerial<'a, I, TX, RX>
where
    TX: UartPad<I> + Transmit<I>,
    RX: UartPad<I> + Receive<I>,
{
    /// Create a new blocking serial.
    pub fn new(
        reg: &'a RegisterBlock,
        tx: TX,
        rx: RX,
        config: UartConfig,
        clk: &cmu::RegisterBlock,
    ) -> Self {
        // Enable clocks for the UART instance
        // TODO: flexible clock frequency handling
        let fix_mod_clk_rate = 48_000_000;
        let fix_mod_div = 24;
        let uart_clk = match I {
            0 => &clk.clock_uart0,
            1 => &clk.clock_uart1,
            2 => &clk.clock_uart2,
            3 => &clk.clock_uart3,
            4 => &clk.clock_uart4,
            5 => &clk.clock_uart5,
            6 => &clk.clock_uart6,
            7 => &clk.clock_uart7,
            _ => panic!("Invalid UART index"),
        };
        unsafe {
            // Initialize module clock.
            // Reference: https://aicdoc.artinchip.com/topics/ic/cmu/cmu-function2-d13x.html#topic_yvp_f24_4bc__table_qb3_bn5_ydc
            uart_clk.modify(|v| v.set_module_clk_div(fix_mod_div).enable_module_clk());
            uart_clk.modify(|v| v.enable_bus_clk());
            uart_clk.modify(|v| v.enable_module_reset());
            riscv::asm::delay(500);
            uart_clk.modify(|v| v.disable_module_reset());
        }

        // Parse configuration
        let baud_rate = config.baud_rate.0;
        let data_bits = config.data_bits;
        let stop_bits = config.stop_bits;
        let parity = config.parity;

        // Halt uart for configuration
        unsafe {
            reg.halt.modify(|v| v.set_halt_change_config_at_busy(true));
        }

        // Disable all interrupts
        let uart16550 = &reg.uart16550;
        let interrupt_types = uart16550.ier().read();
        uart16550.ier().write(
            interrupt_types
                .disable_ms()
                .disable_rda()
                .disable_rls()
                .disable_thre(),
        );

        // Write baud rate divisor
        let uart_divisor = fix_mod_clk_rate / (16 * baud_rate);
        uart16550.write_divisor(uart_divisor as u16);

        // Update HALT register to apply configuration
        unsafe {
            reg.halt.modify(|v| v.set_halt_change_update(true));
        }

        // Configure line control register
        let lcr = uart16550.lcr().read();
        uart16550.lcr().write(
            lcr.set_char_len(data_bits.to_char_len())
                .set_one_stop_bit(stop_bits == StopBits::One)
                .set_parity(parity.to_parity()),
        );

        // Enable FIFO and set trigger levels
        uart16550.iir_fcr().write(TriggerLevel::_14.and_reset());

        Self { reg, tx, rx }
    }

    /// Blocking write buffer.
    pub fn blocking_write(&mut self, buf: &[u8]) -> Result<usize, ()> {
        let uart16550 = &self.reg.uart16550;

        for &b in buf {
            // Wait until the transmitter FIFO is not full
            while !uart16550.lsr().read().is_transmitter_fifo_empty() {
                core::hint::spin_loop();
            }
            uart16550.rbr_thr().tx_data(b);
        }
        Ok(buf.len())
    }

    /// Blocking read buffer.
    pub fn blocking_read(&mut self, buf: &mut [u8]) -> Result<usize, ()> {
        // TODO: implement reading into buffer
        buf.fill(0);
        Ok(0)
    }

    /// Statically split into transmit and receive halves.
    pub fn split(self) -> (TransmitHalf<'a, I, TX>, ReceiveHalf<'a, I, RX>) {
        (
            TransmitHalf {
                reg: self.reg,
                _pad: self.tx,
            },
            ReceiveHalf {
                _reg: self.reg,
                _pad: self.rx,
            },
        )
    }

    /// Free the blocking serial and return UART instance, TX and RX pads.
    pub fn free(self, clk: &cmu::RegisterBlock) -> (Uart<I>, TX, RX) {
        unsafe {
            let uart_clk = match I {
                0 => &clk.clock_uart0,
                1 => &clk.clock_uart1,
                2 => &clk.clock_uart2,
                3 => &clk.clock_uart3,
                4 => &clk.clock_uart4,
                5 => &clk.clock_uart5,
                6 => &clk.clock_uart6,
                _ => &clk.clock_uart7,
            };
            uart_clk.modify(|v| {
                v.disable_bus_clk()
                    .disable_module_clk()
                    .enable_module_reset()
            });
        }
        (Uart::__new(self.reg), self.tx, self.rx)
    }
}

/// Transmit half of the serial interface.
pub struct TransmitHalf<'a, const I: u8, TX>
where
    TX: UartPad<I> + Transmit<I>,
{
    reg: &'a RegisterBlock,
    _pad: TX,
}

impl<'a, const I: u8, TX> TransmitHalf<'a, I, TX>
where
    TX: UartPad<I> + Transmit<I>,
{
    /// Blocking write buffer.
    pub fn blocking_write(&mut self, buf: &[u8]) -> Result<usize, ()> {
        let uart16550 = &self.reg.uart16550;

        for &b in buf {
            // Wait until the transmitter FIFO is not full
            while !uart16550.lsr().read().is_transmitter_fifo_empty() {
                core::hint::spin_loop();
            }
            uart16550.rbr_thr().tx_data(b);
        }
        Ok(buf.len())
    }
}

/// Receive half of the serial interface.
pub struct ReceiveHalf<'a, const I: u8, RX>
where
    RX: UartPad<I> + Receive<I>,
{
    _reg: &'a RegisterBlock,
    _pad: RX,
}

impl<'a, const I: u8, RX> ReceiveHalf<'a, I, RX>
where
    RX: UartPad<I> + Receive<I>,
{
    /// Blocking read buffer.
    pub fn blocking_read(&mut self, buf: &mut [u8]) -> Result<usize, ()> {
        // TODO: implement reading into buffer
        buf.fill(0);
        Ok(0)
    }
}

impl<'a, const I: u8, TX, RX> embedded_io::ErrorType for BlockingSerial<'a, I, TX, RX>
where
    TX: UartPad<I> + Transmit<I>,
    RX: UartPad<I> + Receive<I>,
{
    type Error = core::convert::Infallible;
}

impl<'a, const I: u8, TX> embedded_io::ErrorType for TransmitHalf<'a, I, TX>
where
    TX: UartPad<I> + Transmit<I>,
{
    type Error = core::convert::Infallible;
}

impl<'a, const I: u8, RX> embedded_io::ErrorType for ReceiveHalf<'a, I, RX>
where
    RX: UartPad<I> + Receive<I>,
{
    type Error = core::convert::Infallible;
}

impl<'a, const I: u8, TX, RX> embedded_io::Write for BlockingSerial<'a, I, TX, RX>
where
    TX: UartPad<I> + Transmit<I>,
    RX: UartPad<I> + Receive<I>,
{
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.blocking_write(buf).ok();
        Ok(buf.len())
    }
    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        while !self.reg.usr.read().is_transmit_fifo_empty() {
            core::hint::spin_loop();
        }
        Ok(())
    }
}

impl<'a, const I: u8, TX> embedded_io::Write for TransmitHalf<'a, I, TX>
where
    TX: UartPad<I> + Transmit<I>,
{
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.blocking_write(buf).ok();
        Ok(buf.len())
    }
    #[inline]
    fn flush(&mut self) -> Result<(), Self::Error> {
        while !self.reg.usr.read().is_transmit_fifo_empty() {
            core::hint::spin_loop();
        }
        Ok(())
    }
}

impl<'a, const I: u8, TX, RX> embedded_io::Read for BlockingSerial<'a, I, TX, RX>
where
    TX: UartPad<I> + Transmit<I>,
    RX: UartPad<I> + Receive<I>,
{
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.blocking_read(buf).ok();
        Ok(buf.len())
    }
}

impl<'a, const I: u8, RX> embedded_io::Read for ReceiveHalf<'a, I, RX>
where
    RX: UartPad<I> + Receive<I>,
{
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.blocking_read(buf).ok();
        Ok(buf.len())
    }
}
