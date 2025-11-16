//! UART pin multiplexer macros.

/// Implements the `Transmit` and `UartPad` traits for multiple UART pins.
#[allow(unused_macros)]
macro_rules! uart_tx {
    ($uart_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl crate::uart::pad::UartPad<$uart_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
            impl crate::uart::pad::Transmit<$uart_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
        )+
    };
}

/// Implements the `Receive` and `UartPad` traits for multiple UART pins.
#[allow(unused_macros)]
macro_rules! uart_rx {
    ($uart_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl crate::uart::pad::UartPad<$uart_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
            impl crate::uart::pad::Receive<$uart_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
        )+
    };
}
