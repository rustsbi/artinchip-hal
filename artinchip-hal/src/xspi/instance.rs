//! XSPI instance.

use super::register::RegisterBlock;
use core::marker::PhantomData;

/// XSPI instance.
pub struct Xspi {
    reg: *const RegisterBlock,
    _private: PhantomData<()>,
}

impl Xspi {
    /// Create a new XSPI instance.
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
