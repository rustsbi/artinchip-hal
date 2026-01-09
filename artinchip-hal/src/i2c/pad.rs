//! I2C pad.

pub trait I2cPad<const I: u8> {}
pub trait SerialClock<const I: u8>: I2cPad<I> {}
pub trait SerialData<const I: u8>: I2cPad<I> {}
