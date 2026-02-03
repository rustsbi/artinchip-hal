//! DMA instance.

use super::register::RegisterBlock;
use core::marker::PhantomData;

/// DMA instance.
pub struct Dma {
    reg: *const RegisterBlock,
    _private: PhantomData<()>,
}

impl Dma {
    /// Create a new DMA instance.
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
