//! GTC extension traits.

use super::delay::TimerDelay;
use super::register::CntFreq;
use crate::cmu::Cmu;

pub trait GtcExt<'a> {
    /// Creates a new TimerDelay instance.
    fn new_timer_delay(self, freq: CntFreq, cmu: &Cmu) -> TimerDelay<'a>;
}
