//! Input GPIO pad implementation.

use super::{
    register::{GpioGroup, PinConfig, PinDriveStrength, PinPull},
    set_mode::{FromRegisters, WithinGpioGroup, set_mode},
};

/// Input mode GPIO pad.
pub struct Input<'a> {
    number: u8,
    regs: &'a GpioGroup,
}

impl<'a> Input<'a> {
    const PIN_CONFIG: PinConfig = PinConfig::zeroed()
        .disable_general_output()
        .enable_general_input()
        .set_pin_func(1)
        .set_drive_strength(PinDriveStrength::Level3);

    // Macro internal function for ROM runtime; DO NOT USE.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn __new(number: u8, regs: &'a GpioGroup, pin_config: PinConfig) -> Self {
        set_mode(Self { number, regs }, pin_config)
    }

    /// Configures the pin to operate as a pull up input.
    #[inline]
    pub fn new_pull_up(number: u8, regs: &'a GpioGroup) -> Self {
        unsafe { Self::__new(number, regs, Self::PIN_CONFIG.set_pin_pull(PinPull::PullUp)) }
    }

    /// Configures the pin to operate as a pull down input.
    #[inline]
    pub fn new_pull_down(number: u8, regs: &'a GpioGroup) -> Self {
        unsafe {
            Self::__new(
                number,
                regs,
                Self::PIN_CONFIG.set_pin_pull(PinPull::PullDown),
            )
        }
    }

    /// Configures the pin to operate as a floating input.
    #[inline]
    pub fn new_floating(number: u8, regs: &'a GpioGroup) -> Self {
        unsafe {
            Self::__new(
                number,
                regs,
                Self::PIN_CONFIG.set_pin_pull(PinPull::Disabled),
            )
        }
    }
}

impl<'a> embedded_hal::digital::ErrorType for Input<'a> {
    type Error = core::convert::Infallible;
}

impl<'a> embedded_hal::digital::InputPin for Input<'a> {
    #[inline]
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.regs.input_state.read().is_high(self.number as usize))
    }
    #[inline]
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.regs.input_state.read().is_low(self.number as usize))
    }
}

impl<'a> WithinGpioGroup<'a> for Input<'a> {
    fn gpio_number(&self) -> u8 {
        self.number
    }
    fn gpio_group(&self) -> &'a GpioGroup {
        self.regs
    }
}

impl<'a> FromRegisters<'a> for Input<'a> {
    unsafe fn from_gpio(number: u8, regs: &'a GpioGroup) -> Self {
        Input { number, regs }
    }
}
