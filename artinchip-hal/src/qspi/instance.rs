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
    fn new_blocking<PAD>(
        self,
        pad: PAD,
        config: QspiConfig,
        cmu: &Cmu,
    ) -> BlockingQspi<'static, I, PAD>
    where
        PAD: QspiPads<I>,
    {
        BlockingQspi::new(self.register_block(), pad, config, cmu)
    }
}
