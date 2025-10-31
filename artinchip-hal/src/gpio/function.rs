//! Function mode GPIO pad implementation.

use super::{
    register::{GpioGroup, PinConfig, PinDriveStrength, PinPull},
    set_mode::{FromRegisters, WithinGpioGroup, set_mode},
};

/// Function mode GPIO pad.
pub struct Function<'a, const F: u8> {
    number: u8,
    regs: &'a GpioGroup,
}

impl<'a, const F: u8> Function<'a, F> {
    const PIN_CONFIG: PinConfig = PinConfig::zeroed()
        .disable_general_output()
        .disable_general_input()
        .set_pin_pull(PinPull::PullUp)
        .set_pin_func(F)
        .set_drive_strength(PinDriveStrength::Level3);

    // Macro internal function for ROM runtime; DO NOT USE.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn __new(number: u8, regs: &'a GpioGroup, pin_config: PinConfig) -> Self {
        set_mode(Self { number, regs }, pin_config)
    }

    /// Configures the pin to operate as a function mode pin.
    #[inline]
    pub fn new_with_func(number: u8, regs: &'a GpioGroup) -> Self {
        unsafe { Self::__new(number, regs, Self::PIN_CONFIG) }
    }

    /// Configures the pin drive strength.
    #[inline]
    pub fn set_drive_strength(&mut self, strength: PinDriveStrength) {
        unsafe {
            self.regs.pin_config[self.number as usize].modify(|r| r.set_drive_strength(strength));
        }
    }

    /// Configures the pin pull.
    #[inline]
    pub fn set_pull(&mut self, pull: PinPull) {
        unsafe {
            self.regs.pin_config[self.number as usize].modify(|r| r.set_pin_pull(pull));
        }
    }
}

impl<'a, const F: u8> embedded_hal::digital::ErrorType for Function<'a, F> {
    type Error = core::convert::Infallible;
}

impl<'a, const F: u8> WithinGpioGroup<'a> for Function<'a, F> {
    fn gpio_number(&self) -> u8 {
        self.number
    }
    fn gpio_group(&self) -> &'a GpioGroup {
        self.regs
    }
}

impl<'a, const F: u8> FromRegisters<'a> for Function<'a, F> {
    unsafe fn from_gpio(number: u8, regs: &'a GpioGroup) -> Self {
        Function { number, regs }
    }
}
