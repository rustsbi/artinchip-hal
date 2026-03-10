//! I2C extension traits.

use super::blocking::BlockingQspi;
use super::config::QspiConfig;
use super::pad::*;
use crate::cmu::Cmu;

pub trait QspiExt<'a, const I: u8> {
    /// Creates a blocking QSPI interface with the specified pads.
    fn new_blocking<PAD>(self, pad: PAD, config: QspiConfig, cmu: &Cmu) -> BlockingQspi<'a, I, PAD>
    where
        PAD: QspiPads<I>;
}
