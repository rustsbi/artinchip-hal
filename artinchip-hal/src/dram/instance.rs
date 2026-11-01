//! DRAM instance.

use core::marker::PhantomData;

use super::config::DramConfig;
use super::dram_ext::DramExt;
use super::driver::DramDriver;
use super::error::DramError;
use super::register::{ControllerMode, ControllerRegisterBlock, PhyRegisterBlock};
use crate::cmu::{ClockTree, Cmu};
use crate::sid::Sid;
use crate::sys_cfg::{SysCfg, V2};

/// DRAM controller and PHY instance.
pub struct Dram {
    controller: *const ControllerRegisterBlock,
    phy: *const PhyRegisterBlock,
    dram_base: usize,
    _private: PhantomData<()>,
}

impl Dram {
    /// Create a new DRAM instance from the controller, PHY register blocks, and DRAM window base address.
    pub const fn __new(
        controller: *const ControllerRegisterBlock,
        phy: *const PhyRegisterBlock,
        dram_base: usize,
    ) -> Self {
        Self {
            controller,
            phy,
            dram_base,
            _private: PhantomData,
        }
    }

    /// Base address of the decoded DRAM memory window.
    #[inline]
    pub const fn dram_base(&self) -> usize {
        self.dram_base
    }

    /// Get a reference to the controller register block.
    pub const fn controller(&self) -> &'static ControllerRegisterBlock {
        unsafe { &*self.controller }
    }

    /// Get a reference to the PHY register block.
    pub const fn phy(&self) -> &'static PhyRegisterBlock {
        unsafe { &*self.phy }
    }

    /// Whether the DRAM rail is enabled and the controller is in normal operating mode.
    pub fn is_up(&self, syscfg: &SysCfg<V2>) -> bool {
        let rail = syscfg.register_block().ldo1x_cfg.read().is_enabled();
        let status = self.controller().status.read();
        rail && status.mode() == ControllerMode::Normal
    }
}

impl DramExt<'static> for Dram {
    fn new_driver(
        self,
        cmu: &mut Cmu,
        syscfg: &mut SysCfg<V2>,
        sid: &mut Sid,
        config: &DramConfig,
        clock_tree: &ClockTree,
    ) -> Result<DramDriver<'static>, DramError> {
        DramDriver::new(
            self.controller(),
            self.phy(),
            self.dram_base(),
            cmu,
            syscfg,
            sid,
            config,
            clock_tree,
        )
    }
}
