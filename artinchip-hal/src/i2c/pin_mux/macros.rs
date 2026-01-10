//! I2C pin multiplexer macros.

/// Implements the `SerialClock` and `I2cPad` traits for multiple I2C pins.
#[allow(unused_macros)]
macro_rules! i2c_scl {
    ($i2c_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl crate::i2c::pad::I2cPad<$i2c_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
            impl crate::i2c::pad::SerialClock<$i2c_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
        )+
    };
}

/// Implements the `SerialData` and `I2cPad` traits for multiple I2C pins.
#[allow(unused_macros)]
macro_rules! i2c_sda {
    ($i2c_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl crate::i2c::pad::I2cPad<$i2c_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
            impl crate::i2c::pad::SerialData<$i2c_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
        )+
    };
}
