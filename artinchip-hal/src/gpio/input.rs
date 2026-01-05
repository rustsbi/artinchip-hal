//! Input GPIO pad implementation.

use super::{
    GpioPad,
    mode::{FromRegisters, WithinGpioGroup, set_mode},
    register::{GpioGroup, PinConfig, PinDriveStrength, PinPull, RegisterBlock},
};

/// Input mode GPIO pad.
pub struct Input<'a, const G: char, const N: u8> {
    regs: &'a RegisterBlock,
}

impl<'a, const G: char, const N: u8> Input<'a, G, N> {
    const PIN_CONFIG: PinConfig = PinConfig::zeroed()
        .disable_general_output()
        .enable_general_input()
        .set_pin_func(1)
        .set_drive_strength(PinDriveStrength::Level3);

    // Macro internal function for ROM runtime; DO NOT USE.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn __new(regs: &'a RegisterBlock, pin_config: PinConfig) -> Self {
        set_mode(Self { regs }, pin_config)
    }

    /// Configures the pin to operate as a pull up input.
    #[inline]
    pub fn new_pull_up(regs: &'a RegisterBlock) -> Self {
        unsafe { Self::__new(regs, Self::PIN_CONFIG.set_pin_pull(PinPull::PullUp)) }
    }

    /// Configures the pin to operate as a pull down input.
    #[inline]
    pub fn new_pull_down(regs: &'a RegisterBlock) -> Self {
        unsafe { Self::__new(regs, Self::PIN_CONFIG.set_pin_pull(PinPull::PullDown)) }
    }

    /// Configures the pin to operate as a floating input.
    #[inline]
    pub fn new_floating(regs: &'a RegisterBlock) -> Self {
        unsafe { Self::__new(regs, Self::PIN_CONFIG.set_pin_pull(PinPull::Disabled)) }
    }

    /// Free current input mode GPIO pad and return the original pad.
    ///
    /// Once freed, the GPIO will be reset.
    #[inline]
    pub fn free(self) -> GpioPad<G, N> {
        unsafe {
            self.group().pin_config[N as usize].write(PinConfig::zeroed());
        }
        GpioPad::__new(self.regs)
    }
}

impl<'a, const G: char, const N: u8> embedded_hal::digital::ErrorType for Input<'a, G, N> {
    type Error = core::convert::Infallible;
}

impl<'a, const G: char, const N: u8> embedded_hal::digital::InputPin for Input<'a, G, N> {
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.group().input_state.read().is_high(N as usize))
    }
    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.group().input_state.read().is_low(N as usize))
    }
}

impl<'a, const G: char, const N: u8> WithinGpioGroup<'a, G> for Input<'a, G, N> {
    #[inline]
    fn group(&self) -> &'a GpioGroup {
        &self.regs.groups[self.group_index()]
    }
    #[inline]
    fn block(&self) -> &'a RegisterBlock {
        self.regs
    }
}

impl<'a, const N: u8, const G: char> FromRegisters<'a, N> for Input<'a, G, N> {
    #[inline]
    unsafe fn from_gpio(regs: &'a RegisterBlock) -> Self {
        Input { regs }
    }
}
