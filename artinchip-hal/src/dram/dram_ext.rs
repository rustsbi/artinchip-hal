//! DRAM extension traits.

use super::config::DramConfig;
use super::driver::DramDriver;
use super::error::DramError;
use crate::cmu::{ClockTree, Cmu};
use crate::sid::Sid;
use crate::sys_cfg::{SysCfg, V2};

/// Extension trait for bringing the DRAM up.
pub trait DramExt<'a> {
    /// Bring the controller and PHY up for `config` and check the PHY's verdict.
    fn new_driver(
        self,
        cmu: &mut Cmu,
        syscfg: &mut SysCfg<V2>,
        sid: &mut Sid,
        config: &DramConfig,
        clock_tree: &ClockTree,
    ) -> Result<DramDriver<'a>, DramError>;
}
