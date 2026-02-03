//! WRI instance.

use super::register::RegisterBlock;
use core::marker::PhantomData;

/// WRI instance.
pub struct Wri {
    reg: *const RegisterBlock,
    _private: PhantomData<()>,
}

impl Wri {
    /// Create a new WRI instance.
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
