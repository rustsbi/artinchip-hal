//! UART pad.

pub trait UartPad<const I: u8> {}
pub trait Transmit<const I: u8>: UartPad<I> {}
pub trait Receive<const I: u8>: UartPad<I> {}
