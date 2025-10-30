//! Output GPIO pad implementation.

use super::{
    register::{
        GpioGroup, OutputClear, OutputSet, OutputToggle, PinConfig, PinDriveStrength, PinPull,
    },
    set_mode::{FromRegisters, WithinGpioGroup, set_mode},
};

/// Output mode GPIO pad.
pub struct Output<'a> {
    number: u8,
    regs: &'a GpioGroup,
}

impl<'a> Output<'a> {
    // Macro internal function for ROM runtime; DO NOT USE.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn __new(number: u8, regs: &'a GpioGroup, pin_config: PinConfig) -> Self {
        set_mode(Self { number, regs }, pin_config)
    }
    /// Configures the pin to operate as a pull up output.
    #[inline]
    pub fn into_pull_up_output(number: u8, regs: &'a GpioGroup) -> Self {
        unsafe {
            Self::__new(
                number,
                regs,
                PinConfig::zeroed()
                    .enable_general_output()
                    .disable_general_input()
                    .set_pin_func(1)
                    .set_pin_pull(PinPull::PullUp)
                    .set_drive_strength(PinDriveStrength::Level3),
            )
        }
    }
    /// Configures the pin to operate as a pull down output.
    #[inline]
    pub fn into_pull_down_output(number: u8, regs: &'a GpioGroup) -> Self {
        unsafe {
            Self::__new(
                number,
                regs,
                PinConfig::zeroed()
                    .enable_general_output()
                    .disable_general_input()
                    .set_pin_func(1)
                    .set_pin_pull(PinPull::PullDown)
                    .set_drive_strength(PinDriveStrength::Level3),
            )
        }
    }
    /// Configures the pin to operate as a floating output.
    #[inline]
    pub fn into_floating_output(number: u8, regs: &'a GpioGroup) -> Self {
        unsafe {
            Self::__new(
                number,
                regs,
                PinConfig::zeroed()
                    .enable_general_output()
                    .disable_general_input()
                    .set_pin_func(1)
                    .set_pin_pull(PinPull::Disabled)
                    .set_drive_strength(PinDriveStrength::Level3),
            )
        }
    }
    /// Configures the pin drive strength.
    #[inline]
    pub fn set_drive_strength(&mut self, strength: PinDriveStrength) {
        unsafe {
            self.regs.pin_config[self.number as usize].modify(|r| r.set_drive_strength(strength));
        }
    }
}

impl<'a> embedded_hal::digital::ErrorType for Output<'a> {
    type Error = core::convert::Infallible;
}

impl<'a> embedded_hal::digital::OutputPin for Output<'a> {
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe {
            self.regs
                .output_clear
                .write(OutputClear::default().clear_output(self.number as usize));
        }
        Ok(())
    }
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe {
            self.regs
                .output_set
                .write(OutputSet::default().set_output(self.number as usize));
        }
        Ok(())
    }
}

impl<'a> embedded_hal::digital::StatefulOutputPin for Output<'a> {
    #[inline]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.regs.output_config.read().is_high(self.number as usize))
    }
    #[inline]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.regs.output_config.read().is_low(self.number as usize))
    }
    #[inline]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        unsafe {
            self.regs
                .output_toggle
                .write(OutputToggle::default().toggle_output(self.number as usize));
        }
        Ok(())
    }
}

impl<'a> WithinGpioGroup<'a> for Output<'a> {
    #[inline]
    fn gpio_number(&self) -> u8 {
        self.number
    }
    #[inline]
    fn gpio_group(&self) -> &'a GpioGroup {
        self.regs
    }
}

impl<'a> FromRegisters<'a> for Output<'a> {
    #[inline]
    unsafe fn from_gpio(number: u8, regs: &'a GpioGroup) -> Self {
        Self { number, regs }
    }
}
