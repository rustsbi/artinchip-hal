//! WDOG instance.

use super::driver::WdogDriver;
use super::register::RegisterBlock;
use super::wdog_ext::WdogExt;
use crate::cmu::Cmu;
use core::marker::PhantomData;

/// WDOG instance.
pub struct Wdog {
    reg: *const RegisterBlock,
    _private: PhantomData<()>,
}

impl Wdog {
    /// Create a new WDOG instance.
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

impl WdogExt<'static> for Wdog {
    fn new_driver(self, cmu: &Cmu) -> WdogDriver<'static> {
        WdogDriver::new(self.register_block(), cmu)
    }
}
