//! I2C extension traits.

use super::blocking::BlockingQspi;
use super::config::QspiConfig;
use super::pad::*;
use crate::cmu::Cmu;

pub trait QspiExt<'a, const I: u8> {
    /// Creates a blocking Q interface with the specified pads.
    #[allow(clippy::too_many_arguments)]
    fn new_blocking<SCK, MOSI, MISO, CS, WP, HOLD>(
        self,
        sck: SCK,
        mosi: Option<MOSI>,
        miso: Option<MISO>,
        cs: Option<CS>,
        wp: Option<WP>,
        hold: Option<HOLD>,
        config: QspiConfig,
        cmu: &Cmu,
    ) -> BlockingQspi<'a, I, SCK, MOSI, MISO, CS, WP, HOLD>
    where
        SCK: QspiPad<I> + SerialClock<I>,
        MISO: QspiPad<I> + MasterInSlaveOut<I>,
        MOSI: QspiPad<I> + MasterOutSlaveIn<I>,
        CS: QspiPad<I> + ChipSelect<I>,
        WP: QspiPad<I> + WriteProtect<I>,
        HOLD: QspiPad<I> + Hold<I>;
}
