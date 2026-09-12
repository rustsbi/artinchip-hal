//! XSPI extension traits.

use super::config::{Chip, PsramModel};
use super::driver::XspiDriver;
use super::error::PsramError;
use crate::cmu::Cmu;
use crate::sys_cfg::SysCfg;
use embedded_hal::delay::DelayNs;

pub trait XspiExt<'a> {
    /// Create and bring up an XSPI PSRAM driver for the given SoC and part.
    fn new_driver(
        self,
        chip: Chip,
        model: PsramModel,
        delay: &mut impl DelayNs,
        syscfg: &'a SysCfg,
        cmu: &'a Cmu,
    ) -> Result<XspiDriver<'a>, PsramError>;
}
