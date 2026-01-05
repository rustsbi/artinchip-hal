//! GTC extension traits.

use super::delay::TimerDelay;
use super::register::CntFreq;
use crate::cmu;

pub trait GtcExt<'a> {
    /// Creates a new TimerDelay instance.
    fn new_timer_delay(self, freq: CntFreq, cmu: &cmu::RegisterBlock) -> TimerDelay<'a>;
}
