//! PWM extension traits.

use super::config::PwmConfig;
use super::driver::PwmChDriver;
use super::pad::PwmPads;
use crate::cmu::{ClockTree, Cmu};

pub trait PwmExt<'a, const I: u8> {
    /// Create a new PWM channel driver.
    fn new_driver<PAD>(
        self,
        pad: PAD,
        config: PwmConfig,
        clock_tree: &ClockTree,
        cmu: &mut Cmu,
    ) -> PwmChDriver<'a, I, PAD>
    where
        PAD: PwmPads<I>;
}
