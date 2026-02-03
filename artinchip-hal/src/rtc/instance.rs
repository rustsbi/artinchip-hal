//! RTC instance.

use super::register::RegisterBlock;
use core::marker::PhantomData;

/// RTC instance.
pub struct Rtc {
    reg: *const RegisterBlock,
    _private: PhantomData<()>,
}

impl Rtc {
    /// Create a new RTC instance.
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
