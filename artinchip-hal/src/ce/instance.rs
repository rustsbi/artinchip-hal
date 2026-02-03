//! CE instance.

use super::register::RegisterBlock;
use core::marker::PhantomData;

/// CE instance.
pub struct Ce {
    reg: *const RegisterBlock,
    _private: PhantomData<()>,
}

impl Ce {
    /// Create a new CE instance.
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
