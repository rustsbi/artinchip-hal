//! QSPI instance.

use super::register::RegisterBlock;
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
