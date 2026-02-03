//! SPI ENC instance.

use super::register::RegisterBlock;
use core::marker::PhantomData;

/// SPI ENC instance.
pub struct SpiEnc {
    reg: *const RegisterBlock,
    _private: PhantomData<()>,
}

impl SpiEnc {
    /// Create a new SPI ENC instance.
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
