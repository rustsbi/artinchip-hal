//! Output GPIO pad implementation.

use super::{
    GpioPad,
    mode::{FromRegisters, WithinGpioGroup, set_mode},
    register::{
        GpioGroup, OutputClear, OutputSet, OutputToggle, PinConfig, PinDriveStrength, PinPull,
        RegisterBlock,
    },
};

/// Output mode GPIO pad.
pub struct Output<'a, const G: char, const N: u8> {
    regs: &'a RegisterBlock,
}

impl<'a, const G: char, const N: u8> Output<'a, G, N> {
    const PIN_CONFIG: PinConfig = PinConfig::zeroed()
        .enable_general_output()
        .disable_general_input()
        .set_pin_func(1)
        .set_drive_strength(PinDriveStrength::Level3);

    // Macro internal function for ROM runtime; DO NOT USE.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn __new(regs: &'a RegisterBlock, pin_config: PinConfig) -> Self {
        set_mode(Self { regs }, pin_config)
    }

    /// Configures the pin to operate as a pull up output.
    #[inline]
    pub fn new_pull_up(regs: &'a RegisterBlock) -> Self {
        unsafe { Self::__new(regs, Self::PIN_CONFIG.set_pin_pull(PinPull::PullUp)) }
    }

    /// Configures the pin to operate as a pull down output.
    #[inline]
    pub fn new_pull_down(regs: &'a RegisterBlock) -> Self {
        unsafe { Self::__new(regs, Self::PIN_CONFIG.set_pin_pull(PinPull::PullDown)) }
    }

    /// Configures the pin to operate as a floating output.
    #[inline]
    pub fn new_floating(regs: &'a RegisterBlock) -> Self {
        unsafe { Self::__new(regs, Self::PIN_CONFIG.set_pin_pull(PinPull::Disabled)) }
    }

    /// Configures the pin drive strength.
    #[inline]
    pub fn set_drive_strength(&mut self, strength: PinDriveStrength) {
        unsafe {
            self.group().pin_config[N as usize].modify(|r| r.set_drive_strength(strength));
        }
    }

    /// Free current output mode GPIO pad and return the original pad.
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

impl<'a, const G: char, const N: u8> embedded_hal::digital::ErrorType for Output<'a, G, N> {
    type Error = core::convert::Infallible;
}

impl<'a, const G: char, const N: u8> embedded_hal::digital::OutputPin for Output<'a, G, N> {
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe {
            self.group()
                .output_clear
                .write(OutputClear::default().clear_output(N as usize));
        }
        Ok(())
    }
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe {
            self.group()
                .output_set
                .write(OutputSet::default().set_output(N as usize));
        }
        Ok(())
    }
}

impl<'a, const G: char, const N: u8> embedded_hal::digital::StatefulOutputPin for Output<'a, G, N> {
    #[inline]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.group().output_config.read().is_high(N as usize))
    }
    #[inline]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.group().output_config.read().is_low(N as usize))
    }
    #[inline]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        unsafe {
            self.group()
                .output_toggle
                .write(OutputToggle::default().toggle_output(N as usize));
        }
        Ok(())
    }
}

impl<'a, const G: char, const N: u8> WithinGpioGroup<'a, G> for Output<'a, G, N> {
    #[inline]
    fn group(&self) -> &'a GpioGroup {
        &self.regs.groups[self.group_index()]
    }
    #[inline]
    fn block(&self) -> &'a RegisterBlock {
        self.regs
    }
}

impl<'a, const N: u8, const G: char> FromRegisters<'a, N> for Output<'a, G, N> {
    #[inline]
    unsafe fn from_gpio(regs: &'a RegisterBlock) -> Self {
        Self { regs }
    }
}
