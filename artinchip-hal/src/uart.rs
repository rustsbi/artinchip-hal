//! Universal Asynchronous Receiver-Transmitter (UART).

mod blocking;
mod config;
mod instance;
mod pad;
mod pin_mux;
mod register;
mod uart_ext;

pub use config::*;
pub use instance::Uart;
pub use pad::*;
pub use register::*;
pub use uart_ext::UartExt;
