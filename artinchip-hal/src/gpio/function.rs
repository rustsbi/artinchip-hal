//! Function mode GPIO pad implementation.

use super::{
    GpioPad,
    mode::{FromRegisters, WithinGpioGroup, set_mode},
    register::{GpioGroup, PinConfig, PinDriveStrength, PinPull, RegisterBlock},
};

/// Function mode GPIO pad.
pub struct Function<'a, const G: char, const N: u8, const F: u8> {
    regs: &'a RegisterBlock,
}

impl<'a, const G: char, const N: u8, const F: u8> Function<'a, G, N, F> {
    const PIN_CONFIG: PinConfig = PinConfig::zeroed()
        .disable_general_output()
        .disable_general_input()
        .set_pin_pull(PinPull::PullUp)
        .set_pin_func(F)
        .set_drive_strength(PinDriveStrength::Level3);

    // Macro internal function for ROM runtime; DO NOT USE.
    #[doc(hidden)]
    #[inline]
    pub unsafe fn __new(regs: &'a RegisterBlock, pin_config: PinConfig) -> Self {
        set_mode(Self { regs }, pin_config)
    }

    /// Configures the pin to operate as a function mode pin.
    #[inline]
    pub fn new_with_func(regs: &'a RegisterBlock) -> Self {
        unsafe { Self::__new(regs, Self::PIN_CONFIG) }
    }

    /// Configures the pin drive strength.
    #[inline]
    pub fn set_drive_strength(&mut self, strength: PinDriveStrength) {
        unsafe {
            self.group().pin_config[N as usize].modify(|r| r.set_drive_strength(strength));
        }
    }

    /// Configures the pin pull.
    #[inline]
    pub fn set_pull(&mut self, pull: PinPull) {
        unsafe {
            self.group().pin_config[N as usize].modify(|r| r.set_pin_pull(pull));
        }
    }

    /// Free current function mode GPIO pad and return the original pad.
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

impl<'a, const G: char, const N: u8, const F: u8> embedded_hal::digital::ErrorType
    for Function<'a, G, N, F>
{
    type Error = core::convert::Infallible;
}

impl<'a, const G: char, const N: u8, const F: u8> WithinGpioGroup<'a, G> for Function<'a, G, N, F> {
    #[inline]
    fn group(&self) -> &'a GpioGroup {
        &self.regs.groups[self.group_index()]
    }
    #[inline]
    fn block(&self) -> &'a RegisterBlock {
        self.regs
    }
}

impl<'a, const N: u8, const G: char, const F: u8> FromRegisters<'a, N> for Function<'a, G, N, F> {
    #[inline]
    unsafe fn from_gpio(regs: &'a RegisterBlock) -> Self {
        Self { regs }
    }
}
