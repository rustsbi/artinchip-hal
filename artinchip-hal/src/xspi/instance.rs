//! XSPI instance.

use super::config::{Chip, PsramModel};
use super::driver::XspiDriver;
use super::error::PsramError;
use super::register::RegisterBlock;
use super::xspi_ext::XspiExt;
use crate::cmu::{ClockTree, Cmu};
use crate::sys_cfg::{SysCfg, V1};
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

    /// Whether the controller is enabled and decoding XIP.
    pub fn is_up(&self) -> bool {
        let ctrl = self.register_block().ctrl.read();
        ctrl.is_xspi_enabled() && ctrl.is_xip_enabled()
    }
}

impl<'a> XspiExt<'a> for Xspi {
    fn new_driver(
        self,
        chip: Chip,
        model: PsramModel,
        delay: &mut impl DelayNs,
        syscfg: &'a SysCfg<V1>,
        cmu: &'a Cmu,
        clock_tree: &ClockTree,
    ) -> Result<XspiDriver<'a>, PsramError> {
        XspiDriver::__new(
            self.register_block(),
            chip,
            model,
            delay,
            syscfg,
            cmu,
            clock_tree,
        )
    }
}
