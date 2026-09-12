//! USB host instance.

use super::host_register::RegisterBlock;
use core::marker::PhantomData;

/// USB host controller with a statically known instance number.
pub struct UsbHost<const I: u8> {
    reg: *const RegisterBlock,
    _private: PhantomData<()>,
}

impl<const I: u8> UsbHost<I> {
    /// Create a new USB host controller instance.
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
