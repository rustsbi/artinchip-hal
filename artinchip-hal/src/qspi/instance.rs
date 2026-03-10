//! QSPI instance.

use super::blocking::BlockingQspi;
use super::config::QspiConfig;
use super::pad::*;
use super::qspi_ext::QspiExt;
use super::register::RegisterBlock;
use crate::cmu::Cmu;
use core::marker::PhantomData;

/// QSPI with statically known instance number.
pub struct Qspi<const I: u8> {
    reg: *const RegisterBlock,
    _private: PhantomData<()>,
}

impl<const I: u8> Qspi<I> {
    /// Create a new QSPI instance.
    pub const fn __new(reg: *const RegisterBlock) -> Self {
        Self {
            reg,
            _private: PhantomData,
        }
    }

    /// Get a reference to the register block.
    pub const fn register_block(&self) -> &'static RegisterBlock {
        unsafe { &*self.reg }
    }
}

impl<const I: u8> QspiExt<'static, I> for Qspi<I> {
    /// Creates a blocking Q interface with the specified pads.
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
    ) -> BlockingQspi<'static, I, SCK, MOSI, MISO, CS, WP, HOLD>
    where
        SCK: QspiPad<I> + SerialClock<I>,
        MOSI: QspiPad<I> + MasterOutSlaveIn<I>,
        MISO: QspiPad<I> + MasterInSlaveOut<I>,
        CS: QspiPad<I> + ChipSelect<I>,
        WP: QspiPad<I> + WriteProtect<I>,
        HOLD: QspiPad<I> + Hold<I>,
    {
        BlockingQspi::new(
            self.register_block(),
            sck,
            mosi,
            miso,
            cs,
            wp,
            hold,
            config,
            cmu,
        )
    }
}
