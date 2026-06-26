//! Universal Asynchronous Receiver-Transmitter (UART).

mod blocking;
mod config;
mod instance;
#[cfg(feature = "clic_interrupts")]
mod non_blocking;
mod pad;
mod register;
mod uart_ext;

pub use config::*;
pub use instance::Uart;
#[cfg(feature = "clic_interrupts")]
pub use non_blocking::*;
pub use pad::*;
pub use register::*;
pub use uart_ext::UartExt;
