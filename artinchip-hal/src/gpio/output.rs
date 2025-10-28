use crate::gpio::OutputClear;

use super::{
    register::{GpioGroup, OutputSet, PinConfig, PinDriveStrength, PinPull},
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
    pub unsafe fn __new(number: u8, regs: &'a GpioGroup) -> Self {
        set_mode(Self { number, regs })
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
                .write(OutputClear::default().clear_output(self.number as usize))
        };
        Ok(())
    }
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe {
            self.regs
                .output_set
                .write(OutputSet::default().set_output(self.number as usize))
        };
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
    const CONFIG: PinConfig = PinConfig::zeroed()
        .set_pin_pull(PinPull::Disabled)
        .enable_general_output()
        .disable_general_input()
        .set_drive_strength(PinDriveStrength::Level3)
        .set_pin_func(1);
    #[inline]
    unsafe fn from_gpio(number: u8, regs: &'a GpioGroup) -> Self {
        Self { number, regs }
    }
}
