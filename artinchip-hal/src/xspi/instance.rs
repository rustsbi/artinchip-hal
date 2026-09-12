//! XSPI instance.

use super::config::{Chip, PsramModel};
use super::driver::XspiDriver;
use super::error::PsramError;
use super::register::RegisterBlock;
use super::xspi_ext::XspiExt;
use crate::cmu::Cmu;
use crate::sys_cfg::SysCfg;
use core::marker::PhantomData;
use embedded_hal::delay::DelayNs;

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

impl<'a> XspiExt<'a> for Xspi {
    fn new_driver(
        self,
        chip: Chip,
        model: PsramModel,
        delay: &mut impl DelayNs,
        syscfg: &'a SysCfg,
        cmu: &'a Cmu,
    ) -> Result<XspiDriver<'a>, PsramError> {
        XspiDriver::__new(self.register_block(), chip, model, delay, syscfg, cmu)
    }
}
