//! GTC instance.

use super::delay::TimerDelay;
use super::gtc_ext::GtcExt;
use super::register::{CntFreq, RegisterBlock};
use crate::cmu;
use core::marker::PhantomData;

/// Generic Timer Counter.
pub struct Gtc {
    reg: *const RegisterBlock,
    _private: PhantomData<()>,
}

impl Gtc {
    /// Create a new GTC instance.
    pub const fn __new(reg: *const RegisterBlock) -> Self {
        Self {
            reg,
            _private: PhantomData,
        }
    }

    /// Get a reference to the register block.
    pub const fn register_block(&self) -> &RegisterBlock {
        unsafe { &*self.reg }
    }
}

impl GtcExt<'static> for Gtc {
    fn new_timer_delay(self, freq: CntFreq, cmu: &cmu::RegisterBlock) -> TimerDelay<'static> {
        TimerDelay::new(unsafe { &*self.reg }, freq, cmu)
    }
}
