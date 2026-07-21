//! Async serial communication interface.

use core::cell::RefCell;
use core::future::poll_fn;
use core::task::Poll;

use critical_section::Mutex;
use embassy_sync::waitqueue::AtomicWaker;
use uart16550::{PendingInterrupt, TriggerLevel};

use super::config::{StopBits, UartConfig};
use super::instance::{Uart, UartInterrupt};
use super::pad::{Receive, Transmit, UartPad};
use super::register::RegisterBlock;
use crate::cmu::Cmu;
use crate::interrupt::clic::typelevel::{self, Interrupt as _};
use crate::types::RingBuffer;

const RX_BUF_SIZE: usize = 256;
const TX_BUF_SIZE: usize = 512;
const TX_FIFO_DEPTH: u16 = 256;
const RX_BATCH_SPIN_LIMIT: usize = 2000;

/// Per-UART async state: wakers for rx/tx, and ring buffers protected by critical sections.
pub struct AsyncState {
    pub rx_waker: AtomicWaker,
    pub tx_waker: AtomicWaker,
    pub rx_buffer: Mutex<RefCell<RingBuffer<u8, RX_BUF_SIZE>>>,
    pub tx_buffer: Mutex<RefCell<RingBuffer<u8, TX_BUF_SIZE>>>,
}

impl AsyncState {
    pub const fn new() -> Self {
        Self {
            rx_waker: AtomicWaker::new(),
            tx_waker: AtomicWaker::new(),
            rx_buffer: Mutex::new(RefCell::new(RingBuffer::new())),
            tx_buffer: Mutex::new(RefCell::new(RingBuffer::new())),
        }
    }
}

impl Default for AsyncState {
    fn default() -> Self {
        Self::new()
    }
}

/// Global array holding the state for up to 8 UART instances.
pub static UART_STATES: [AsyncState; 8] = [
    AsyncState::new(),
    AsyncState::new(),
    AsyncState::new(),
    AsyncState::new(),
    AsyncState::new(),
    AsyncState::new(),
    AsyncState::new(),
    AsyncState::new(),
];

/// If the tx buffer is non-empty and THRE is not already enabled,
/// enable THRE so the interrupt handler will drain the buffer.
#[inline]
fn kick_tx_if_idle(reg: &RegisterBlock, state: &AsyncState) {
    // Just peek into the buffer to see if there's any data
    let has_data = critical_section::with(|cs| !state.tx_buffer.borrow_ref(cs).is_empty());

    if !has_data {
        return;
    }

    let uart16550 = &reg.uart16550;
    critical_section::with(|_| {
        let ier = uart16550.ier().read();
        if !ier.thre_enabled() {
            // CPU will immediately receive an interrupt,
            // and jump to `on_interrupt` to handle the data in the buffer.
            uart16550.ier().write(ier.enable_thre());
        }
    });
}

pub struct AsyncUartHandler<const I: u8>;

impl<const I: u8> typelevel::Handler<<Uart<I> as UartInterrupt<I>>::Interrupt>
    for AsyncUartHandler<I>
where
    Uart<I>: UartInterrupt<I>,
{
    unsafe fn on_interrupt() {
        let reg = unsafe { Uart::<I>::regs_at_index() };
        let uart16550 = &reg.uart16550;
        let state = &UART_STATES[I as usize];

        let iir = uart16550.iir_fcr().read();
        let pending = match iir.pending_interrupts() {
            Some(p) => p,
            None => {
                <Uart<I> as UartInterrupt<I>>::Interrupt::clear_pending();
                return;
            }
        };

        match pending {
            PendingInterrupt::TransmitterHoldingRegisterEmpty => {
                let mut wrote_any = false;

                loop {
                    if reg.tfl.read().tx_level() >= TX_FIFO_DEPTH {
                        break;
                    }

                    let byte =
                        critical_section::with(|cs| state.tx_buffer.borrow_ref_mut(cs).pop());
                    let Some(byte) = byte else {
                        critical_section::with(|_| {
                            // FIFO is empty, disable THRE to avoid infinite IRQ storm.
                            let ier = uart16550.ier().read();
                            uart16550.ier().write(ier.disable_thre());
                            // Read IIR to clear the latched THRE status;
                            // otherwise the CLIC will keep re-triggering.
                            let _ = uart16550.iir_fcr().read();
                        });

                        if wrote_any {
                            state.tx_waker.wake();
                        }

                        break;
                    };

                    reg.uart16550.rbr_thr().tx_data(byte);
                    wrote_any = true;
                }
            }
            PendingInterrupt::ReceivedDataAvailable | PendingInterrupt::ReceivedDataTimeout => {
                critical_section::with(|cs| {
                    let mut rx_buf = state.rx_buffer.borrow_ref_mut(cs);
                    loop {
                        let lsr = uart16550.lsr().read();
                        // Must read RBR on any of: data ready, overrun, parity, or
                        // framing error.  Otherwise overrun errors leave stale data
                        // in the FIFO, causing subsequent reads to be misaligned.
                        if !lsr.is_data_ready()
                            && !lsr.is_overrun_error()
                            && !lsr.is_parity_error()
                            && !lsr.is_framing_error()
                        {
                            break;
                        }
                        rx_buf.push(uart16550.rbr_thr().rx_data()).ok();
                    }
                });
                state.rx_waker.wake();
            }
            _ => {}
        }

        <Uart<I> as UartInterrupt<I>>::Interrupt::clear_pending();
    }
}

pub struct AsyncSerial<'a, const I: u8, TX, RX>
where
    TX: UartPad<I> + Transmit<I>,
    RX: UartPad<I> + Receive<I>,
    Uart<I>: UartInterrupt<I>,
{
    pub reg: &'a RegisterBlock,
    _tx: TX,
    _rx: RX,
}

impl<'a, const I: u8, TX, RX> AsyncSerial<'a, I, TX, RX>
where
    TX: UartPad<I> + Transmit<I>,
    RX: UartPad<I> + Receive<I>,
    Uart<I>: UartInterrupt<I>,
{
    pub fn new(reg: &'a RegisterBlock, tx: TX, rx: RX, config: UartConfig, cmu: &mut Cmu) -> Self {
        // Enable clocks for the UART instance
        let fix_mod_clk_rate = 48_000_000;
        let fix_mod_div = 24;
        let clk = cmu.register_block();
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
            // Initialize module clock
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

        // Enable FIFO and set trigger levels.
        uart16550.iir_fcr().write(TriggerLevel::_14.and_reset());

        // Enable the hardware receiver via RXCTL register,
        // without this, no data reaches the RBR regardless of IER settings.
        unsafe {
            reg.rx_ctl.modify(|v| v.enable_rx());
        }

        // Enable RX interrupts so received bytes are pushed into the async RX buffer.
        // THRE is enabled on demand by `kick_tx_if_idle` when there is pending TX data.
        uart16550
            .ier()
            .write(uart16550.ier().read().enable_rda().enable_rls());

        Self {
            reg,
            _tx: tx,
            _rx: rx,
        }
    }
}

impl<'a, const I: u8, TX, RX> embedded_io_async::ErrorType for AsyncSerial<'a, I, TX, RX>
where
    TX: UartPad<I> + Transmit<I>,
    RX: UartPad<I> + Receive<I>,
    Uart<I>: UartInterrupt<I>,
{
    type Error = core::convert::Infallible;
}

impl<'a, const I: u8, TX, RX> embedded_io_async::Write for AsyncSerial<'a, I, TX, RX>
where
    TX: UartPad<I> + Transmit<I>,
    RX: UartPad<I> + Receive<I>,
    Uart<I>: UartInterrupt<I>,
{
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let state = &UART_STATES[I as usize];

        let mut written = 0usize;

        while written < buf.len() {
            let chunk_written = critical_section::with(|cs| {
                let mut tx_buf = state.tx_buffer.borrow_ref_mut(cs);
                let mut chunk_written = 0;

                if tx_buf.is_full() {
                    return 0;
                }

                for &byte in &buf[written..] {
                    if tx_buf.push(byte).is_ok() {
                        chunk_written += 1;
                    } else {
                        break;
                    }
                }

                chunk_written
            });

            written += chunk_written;

            if written == buf.len() {
                break;
            }

            if chunk_written == 0 {
                poll_fn(|cx| {
                    state.tx_waker.register(cx.waker());

                    let has_space =
                        critical_section::with(|cs| !state.tx_buffer.borrow_ref(cs).is_full());

                    if has_space {
                        Poll::Ready(())
                    } else {
                        Poll::Pending
                    }
                })
                .await;
            } else {
                kick_tx_if_idle(self.reg, state);
            }
        }

        // Kick once to avoid relying solely on THRE edge, which may cause await to hang occasionally.
        kick_tx_if_idle(self.reg, state);

        Ok(written)
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        let state = &UART_STATES[I as usize];
        let reg = self.reg;

        // Kick once to ensure data transfer starts even if interrupt chain is not established.
        kick_tx_if_idle(self.reg, state);

        // Async wait: let the interrupt handler empty the RAM buffer (tx_buffer)
        poll_fn(|cx| {
            state.tx_waker.register(cx.waker());

            // If the interrupt does not continue to trigger, but the hardware FIFO is empty, actively kick again.
            kick_tx_if_idle(self.reg, state);

            let tx_empty = critical_section::with(|cs| state.tx_buffer.borrow_ref(cs).is_empty());
            if tx_empty {
                Poll::Ready(Ok::<(), Self::Error>(()))
            } else {
                // Wake the task to ensure it gets polled again, in case the interrupt was missed.
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        })
        .await?;

        // Sync wait: RAM is empty, wait for the underlying hardware to physically send out the last bit of data
        while reg.usr.read().is_busy() {
            core::hint::spin_loop();
        }

        Ok(())
    }
}

impl<'a, const I: u8, TX, RX> embedded_io_async::Read for AsyncSerial<'a, I, TX, RX>
where
    TX: UartPad<I> + Transmit<I>,
    RX: UartPad<I> + Receive<I>,
    Uart<I>: UartInterrupt<I>,
{
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let state = &UART_STATES[I as usize];

        let mut read_len = 0usize;

        loop {
            let chunk_read = critical_section::with(|cs| {
                let mut rx_buf = state.rx_buffer.borrow_ref_mut(cs);
                let mut chunk_read = 0;

                while read_len + chunk_read < buf.len() {
                    if let Some(byte) = rx_buf.pop() {
                        buf[read_len + chunk_read] = byte;
                        chunk_read += 1;
                    } else {
                        break;
                    }
                }

                chunk_read
            });

            read_len += chunk_read;

            if read_len > 0 {
                let mut quiet_spins = 0usize;

                while read_len < buf.len() {
                    let got_more = critical_section::with(|cs| {
                        let mut rx_buf = state.rx_buffer.borrow_ref_mut(cs);
                        let mut chunk_read = 0;

                        while read_len + chunk_read < buf.len() {
                            if let Some(byte) = rx_buf.pop() {
                                buf[read_len + chunk_read] = byte;
                                chunk_read += 1;
                            } else {
                                break;
                            }
                        }

                        chunk_read
                    });

                    if got_more > 0 {
                        read_len += got_more;
                        quiet_spins = 0;
                        continue;
                    }

                    if quiet_spins >= RX_BATCH_SPIN_LIMIT {
                        return Ok(read_len);
                    }

                    quiet_spins += 1;
                    core::hint::spin_loop();
                }

                return Ok(read_len);
            }

            poll_fn(|cx| {
                state.rx_waker.register(cx.waker());

                let has_data =
                    critical_section::with(|cs| !state.rx_buffer.borrow_ref(cs).is_empty());

                if has_data {
                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            })
            .await;
        }
    }
}
