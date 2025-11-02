//! Internal module of setting GPIO modes.

use super::register::{GpioGroup, PinConfig, RegisterBlock};

/// Internal function to set GPIO pad mode.
#[inline]
pub fn set_mode<'a, const G: char, const N: u8, T, U>(value: T, pin_config: PinConfig) -> U
where
    T: WithinGpioGroup<'a, G>,
    U: FromRegisters<'a, N>,
{
    // take ownership of pad
    let regs = value.group();
    // set pin configuration
    unsafe {
        regs.pin_config[N as usize].write(pin_config);
    }
    // return ownership of pad
    unsafe { U::from_gpio(value.block()) }
}

pub trait WithinGpioGroup<'a, const G: char> {
    fn group(&self) -> &'a GpioGroup;
    fn group_index(&self) -> usize {
        if G == 'U' {
            14
        } else {
            G as usize - 'A' as usize
        }
    }
    fn block(&self) -> &'a RegisterBlock;
}

pub trait FromRegisters<'a, const N: u8> {
    unsafe fn from_gpio(regs: &'a RegisterBlock) -> Self;
}
