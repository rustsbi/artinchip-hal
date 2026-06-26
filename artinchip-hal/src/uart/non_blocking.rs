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
const TX_BUF_SIZE: usize = 256;
const TX_BUSY_SPIN_LIMIT: usize = 200_000;

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
                let byte = critical_section::with(|cs| state.tx_buffer.borrow_ref_mut(cs).pop());
                if let Some(byte) = byte {
                    let mut spins = 0;
                    while reg.usr.read().is_busy() {
                        core::hint::spin_loop();
                        spins += 1;
                        if spins >= TX_BUSY_SPIN_LIMIT {
                            break;
                        }
                    }
                    reg.uart16550.rbr_thr().tx_data(byte);
                } else {
                    critical_section::with(|_| {
                        // FIFO is empty, must disable THRE interrupt to avoid infinite interrupt storm
                        let ier = uart16550.ier().read();
                        uart16550.ier().write(ier.disable_thre());
                    });
                    state.tx_waker.wake();
                }
            }
            PendingInterrupt::ReceivedDataAvailable | PendingInterrupt::ReceivedDataTimeout => {
                critical_section::with(|cs| {
                    let mut rx_buf = state.rx_buffer.borrow_ref_mut(cs);
                    while uart16550.lsr().read().is_data_ready() {
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
            // Initialize module clock.
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

        let written = critical_section::with(|cs| {
            let mut tx_buf = state.tx_buffer.borrow_ref_mut(cs);
            let mut w = 0;
            for &b in buf {
                if tx_buf.push(b).is_ok() {
                    w += 1;
                } else {
                    break;
                }
            }
            w
        });

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
