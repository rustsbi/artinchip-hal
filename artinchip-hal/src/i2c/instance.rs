//! I2C instance.

use super::blocking::BlockingI2c;
use super::config::I2cConfig;
use super::i2c_ext::I2cExt;
use super::pad::I2cPads;
use super::register::RegisterBlock;
use crate::cmu::Cmu;
use core::marker::PhantomData;

/// I2C with statically known instance number.
pub struct I2c<const I: u8> {
    reg: *const RegisterBlock,
    _private: PhantomData<()>,
}

impl<const I: u8> I2c<I> {
    /// Create a new I2C instance.
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

impl<const I: u8> I2cExt<'static, I> for I2c<I> {
    fn new_blocking<PAD>(
        self,
        pad: PAD,
        config: I2cConfig,
        cmu: &Cmu,
    ) -> BlockingI2c<'static, I, PAD>
    where
        PAD: I2cPads<I>,
    {
        BlockingI2c::new(self.register_block(), pad, config, cmu)
    }
}
