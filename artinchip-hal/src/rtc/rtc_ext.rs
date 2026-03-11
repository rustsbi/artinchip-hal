//! RTC extension traits.

use super::time::RealTime;
use crate::cmu::Cmu;

pub trait RtcExt<'a> {
    /// Creates a new RealTime instance.
    fn new_real_time(self, cmu: &Cmu) -> RealTime<'a>;
}
