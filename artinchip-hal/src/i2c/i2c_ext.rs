//! I2C extension traits.

use super::blocking::BlockingI2c;
use super::config::I2cConfig;
use super::pad::I2cPads;
use crate::cmu::Cmu;

pub trait I2cExt<'a, const I: u8> {
    /// Creates a blocking I2C interface with the specified pads.
    fn new_blocking<PAD>(self, pad: PAD, config: I2cConfig, cmu: &Cmu) -> BlockingI2c<'a, I, PAD>
    where
        PAD: I2cPads<I>;
}
