//! DMA instance.

use super::config::PwmConfig;
use super::driver::PwmChDriver;
use super::pad::PwmPads;
use super::pwm_ext::PwmExt;
use super::register::RegisterBlock;
use crate::cmu::Cmu;
use core::marker::PhantomData;

/// PWM channel instance.
pub struct PwmChannel<const I: u8> {
    reg: *const RegisterBlock,
    _private: PhantomData<()>,
}

impl<const I: u8> PwmChannel<I> {
    /// Create a new PWM channel instance.
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

impl<const I: u8> PwmExt<'static, I> for PwmChannel<I> {
    fn new_driver<PAD>(
        self,
        pad: PAD,
        config: PwmConfig,
        cmu: &mut Cmu,
    ) -> PwmChDriver<'static, I, PAD>
    where
        PAD: PwmPads<I>,
    {
        PwmChDriver::__new(self.register_block(), pad, config, cmu)
    }
}

// Pwm channels.
pub struct PwmChannels {
    pub ch0: PwmChannel<0>,
    pub ch1: PwmChannel<1>,
    pub ch2: PwmChannel<2>,
    pub ch3: PwmChannel<3>,
}

impl PwmChannels {
    pub const fn __new(reg: *const RegisterBlock) -> Self {
        Self {
            ch0: PwmChannel::__new(reg),
            ch1: PwmChannel::__new(reg),
            ch2: PwmChannel::__new(reg),
            ch3: PwmChannel::__new(reg),
        }
    }
}
