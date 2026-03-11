//! QSPI pin multiplexer macros.

/// Implements the `SerialClock` traits for multiple QSPI pins.
#[allow(unused_macros)]
macro_rules! qspi_sck {
    ($qspi_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl crate::qspi::pad::SerialClock<$qspi_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
        )+
    };
}

/// Implements the `MasterOutSlaveIn` traits for multiple QSPI pins.
#[allow(unused_macros)]
macro_rules! qspi_mosi {
    ($qspi_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl crate::qspi::pad::MasterOutSlaveIn<$qspi_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
        )+
    };
}

/// Implements the `MasterInSlaveOut` traits for multiple QSPI pins.
#[allow(unused_macros)]
macro_rules! qspi_miso {
    ($qspi_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl crate::qspi::pad::MasterInSlaveOut<$qspi_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
        )+
    };
}

/// Implements the `ChipSelect` traits for multiple QSPI pins.
#[allow(unused_macros)]
macro_rules! qspi_cs {
    ($qspi_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl crate::qspi::pad::ChipSelect<$qspi_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
        )+
    };
}

/// Implements the `WriteProtect` traits for multiple QSPI pins.
#[allow(unused_macros)]
macro_rules! qspi_wp {
    ($qspi_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl crate::qspi::pad::WriteProtect<$qspi_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
        )+
    };
}

/// Implements the `Hold` traits for multiple QSPI pins.
#[allow(unused_macros)]
macro_rules! qspi_hold {
    ($qspi_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl crate::qspi::pad::Hold<$qspi_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
        )+
    };
}
