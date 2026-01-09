//! I2C extension traits.

use super::blocking::BlockingI2c;
use super::config::I2cConfig;
use super::pad::{I2cPad, SerialClock, SerialData};
use crate::cmu;

pub trait I2cExt<'a, const I: u8> {
    /// Creates a blocking I2C interface with the specified pads.
    fn new_blocking<SCL, SDA>(
        self,
        scl: SCL,
        sda: SDA,
        config: I2cConfig,
        cmu: &cmu::RegisterBlock,
    ) -> BlockingI2c<'a, I, SCL, SDA>
    where
        SCL: I2cPad<I> + SerialClock<I>,
        SDA: I2cPad<I> + SerialData<I>;
}
