//! Internal module of setting GPIO modes.

use super::register::{GpioGroup, PinConfig};

/// Internal function to set GPIO pad mode.
#[inline]
pub fn set_mode<'a, T, U>(mut value: T) -> U
where
    T: WithinGpioGroup<'a>,
    U: FromRegisters<'a>,
{
    // take ownership of pad
    let number = value.gpio_number();
    let regs = value.gpio_group();
    unsafe { write_mode::<T, U>(&mut value) };
    // return ownership of pad
    unsafe { U::from_gpio(number, regs) }
}

#[inline]
unsafe fn write_mode<'a, T: WithinGpioGroup<'a>, U: FromRegisters<'a>>(value: &mut T) {
    let regs = value.gpio_group();
    let number = value.gpio_number();
    // apply configuration
    unsafe { regs.pin_config[number as usize].write(U::CONFIG) };
}

pub trait WithinGpioGroup<'a> {
    fn gpio_number(&self) -> u8;
    fn gpio_group(&self) -> &'a GpioGroup;
}

pub trait FromRegisters<'a> {
    const CONFIG: PinConfig;
    unsafe fn from_gpio(number: u8, regs: &'a GpioGroup) -> Self;
}
