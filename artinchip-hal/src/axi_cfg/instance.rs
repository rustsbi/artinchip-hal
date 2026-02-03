//! AXI CFG instance.

use super::register::RegisterBlock;
use core::marker::PhantomData;

/// AXI CFG instance.
pub struct AxiCfg {
    reg: *const RegisterBlock,
    _private: PhantomData<()>,
}

impl AxiCfg {
    /// Create a new AXI CFG instance.
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
