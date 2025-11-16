//! GPIO pad.

use super::function::Function;
use super::input::Input;
use super::output::Output;
use super::pad_ext::PadExt;
use super::register::RegisterBlock;
use core::marker::PhantomData;

/// GPIO pad with statically known GPIO group and number.
pub struct GpioPad<const G: char, const N: u8> {
    reg: *const RegisterBlock,
    _private: PhantomData<()>,
}

impl<const G: char, const N: u8> GpioPad<G, N> {
    pub const fn __new(reg: *const RegisterBlock) -> Self {
        Self {
            reg,
            _private: PhantomData,
        }
    }
}

impl<const G: char, const N: u8> PadExt<'static, G, N> for GpioPad<G, N> {
    #[inline]
    fn into_pull_up_output(self) -> Output<'static, G, N> {
        unsafe { Output::new_pull_up(&*self.reg) }
    }
    #[inline]
    fn into_pull_down_output(self) -> Output<'static, G, N> {
        unsafe { Output::new_pull_down(&*self.reg) }
    }
    #[inline]
    fn into_floating_output(self) -> Output<'static, G, N> {
        unsafe { Output::new_floating(&*self.reg) }
    }
    #[inline]
    fn into_pull_up_input(self) -> Input<'static, G, N> {
        unsafe { Input::new_pull_up(&*self.reg) }
    }
    #[inline]
    fn into_pull_down_input(self) -> Input<'static, G, N> {
        unsafe { Input::new_pull_down(&*self.reg) }
    }
    #[inline]
    fn into_floating_input(self) -> Input<'static, G, N> {
        unsafe { Input::new_floating(&*self.reg) }
    }
    #[inline]
    fn into_function<const F: u8>(self) -> Function<'static, G, N, F> {
        unsafe { Function::new_with_func(&*self.reg) }
    }
}

impl<'a, const G: char, const N: u8> PadExt<'a, G, N> for &'a mut GpioPad<G, N> {
    #[inline]
    fn into_pull_up_output(self) -> Output<'a, G, N> {
        unsafe { Output::new_pull_up(&*self.reg) }
    }
    #[inline]
    fn into_pull_down_output(self) -> Output<'a, G, N> {
        unsafe { Output::new_pull_down(&*self.reg) }
    }
    #[inline]
    fn into_floating_output(self) -> Output<'a, G, N> {
        unsafe { Output::new_floating(&*self.reg) }
    }
    #[inline]
    fn into_pull_up_input(self) -> Input<'a, G, N> {
        unsafe { Input::new_pull_up(&*self.reg) }
    }
    #[inline]
    fn into_pull_down_input(self) -> Input<'a, G, N> {
        unsafe { Input::new_pull_down(&*self.reg) }
    }
    #[inline]
    fn into_floating_input(self) -> Input<'a, G, N> {
        unsafe { Input::new_floating(&*self.reg) }
    }
    #[inline]
    fn into_function<const F: u8>(self) -> Function<'a, G, N, F> {
        unsafe { Function::new_with_func(&*self.reg) }
    }
}
