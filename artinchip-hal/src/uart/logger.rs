//! UARTx logger driven by BlockingSerial — polling TX works even with interrupts disabled.

use core::fmt::Write as _;
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::{
    cmu::{ClockTree, Cmu},
    uart::*,
};
use heapless::String;
use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};

static REG: AtomicPtr<RegisterBlock> = AtomicPtr::new(core::ptr::null_mut());

struct UartLogger;

#[inline]
fn logger_regs() -> Option<&'static RegisterBlock> {
    let ptr = REG.load(Ordering::Acquire);
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { &*ptr })
    }
}

/// Write bytes directly to the logger UART, without a log-level prefix.
///
/// This is for dumping file contents or other diagnostic payloads where the
/// caller wants the bytes on the wire exactly as they are. It is a no-op before
/// [`uart_logger_init`] has run.
pub fn uart_logger_write_raw(bytes: &[u8]) {
    let Some(reg) = logger_regs() else {
        return;
    };
    let uart16550 = &reg.uart16550;
    for &byte in bytes {
        while !uart16550.lsr().read().is_transmitter_fifo_empty() {
            core::hint::spin_loop();
        }
        uart16550.rbr_thr().tx_data(byte);
    }
    while !uart16550.lsr().read().is_transmitter_empty() {
        core::hint::spin_loop();
    }
}

impl Log for UartLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        !REG.load(Ordering::Acquire).is_null()
    }

    fn log(&self, record: &Record) {
        let Some(reg) = logger_regs() else {
            return;
        };
        let uart16550 = &reg.uart16550;

        let mut s: String<256> = String::new();
        write!(s, "[{}] {}", record.level(), record.args()).ok();

        for &byte in s.as_bytes() {
            while !uart16550.lsr().read().is_transmitter_fifo_empty() {
                core::hint::spin_loop();
            }
            uart16550.rbr_thr().tx_data(byte);
        }
        // Newline after each record
        while !uart16550.lsr().read().is_transmitter_fifo_empty() {
            core::hint::spin_loop();
        }
        uart16550.rbr_thr().tx_data(b'\n');

        self.flush();
    }

    fn flush(&self) {
        let Some(reg) = logger_regs() else {
            return;
        };
        while !reg.uart16550.lsr().read().is_transmitter_empty() {
            core::hint::spin_loop();
        }
    }
}

/// Initialize the global logger on UARTx and return a `BlockingSerial`.
///
/// The returned `BlockingSerial` can be used for subsequent blocking I/O
/// independently of the logger (theoretically it won't conflict with logging,
/// but using UART0 for general data transfer is not recommended).
pub fn uart_logger_init<TX, RX, const I: u8>(
    uart: Uart<I>,
    tx: TX,
    rx: RX,
    config: UartConfig,
    clock_tree: &ClockTree,
    cmu: &mut Cmu,
) -> Result<BlockingSerial<'static, I, TX, RX>, SetLoggerError>
where
    TX: UartPad<I> + Transmit<I>,
    RX: UartPad<I> + Receive<I>,
    Uart<I>: UartExt<'static, I>,
{
    let reg_ptr = core::ptr::from_ref(uart.register_block()).cast_mut();

    // `new_blocking` handles all hardware initialization.
    let serial = uart.new_blocking(tx, rx, config, clock_tree, cmu);

    REG.store(reg_ptr, Ordering::Release);

    log::set_logger(&UartLogger)?;
    log::set_max_level(LevelFilter::Trace);

    Ok(serial)
}
