//! QSPI pin multiplexer macros.

/// Implements the `SerialClock` and `QspiPad` traits for multiple QSPI pins.
#[allow(unused_macros)]
macro_rules! qspi_sck {
    ($qspi_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl crate::qspi::pad::QspiPad<$qspi_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
            impl crate::qspi::pad::SerialClock<$qspi_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
        )+
    };
}

/// Implements the `MasterOutSlaveIn` and `QspiPad` traits for multiple QSPI pins.
#[allow(unused_macros)]
macro_rules! qspi_mosi {
    ($qspi_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl crate::qspi::pad::QspiPad<$qspi_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
            impl crate::qspi::pad::MasterOutSlaveIn<$qspi_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
        )+
    };
}

/// Implements the `MasterInSlaveOut` and `QspiPad` traits for multiple QSPI pins.
#[allow(unused_macros)]
macro_rules! qspi_miso {
    ($qspi_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl crate::qspi::pad::QspiPad<$qspi_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
            impl crate::qspi::pad::MasterInSlaveOut<$qspi_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
        )+
    };
}

/// Implements the `ChipSelect` and `QspiPad` traits for multiple QSPI pins.
#[allow(unused_macros)]
macro_rules! qspi_cs {
    ($qspi_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl crate::qspi::pad::QspiPad<$qspi_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
            impl crate::qspi::pad::ChipSelect<$qspi_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
        )+
    };
}

/// Implements the `WriteProtect` and `QspiPad` traits for multiple QSPI pins.
#[allow(unused_macros)]
macro_rules! qspi_wp {
    ($qspi_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl crate::qspi::pad::QspiPad<$qspi_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
            impl crate::qspi::pad::WriteProtect<$qspi_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
        )+
    };
}

/// Implements the `Hold` and `QspiPad` traits for multiple QSPI pins.
#[allow(unused_macros)]
macro_rules! qspi_hold {
    ($qspi_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl crate::qspi::pad::QspiPad<$qspi_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
            impl crate::qspi::pad::Hold<$qspi_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
        )+
    };
}
