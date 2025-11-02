//! Artinchip D13x chip series.

use core::marker::PhantomData;
use paste::paste;

macro_rules! gpio {
    ($Pads:ident, $G:expr, [$($N:tt),*]) => {
        paste! {
            #[doc = concat!("GPIO", $G, " pads.")]
            pub struct $Pads {
                $(
                    #[doc = concat!("GPIO pad P", $G, $N, ".")]
                    pub [<p $G:lower $N>]: GpioPad<$G, $N>,
                )*
            }

            impl $Pads {
                #[inline]
                const fn __new() -> Self {
                    Self {
                        $(
                            [<p $G:lower $N>]: GpioPad {
                                _private: PhantomData,
                            },
                        )*
                    }
                }
            }
        }

        impl<'a, const N: u8> artinchip_hal::gpio::PadExt<'a, $G, N> for &'a mut GpioPad<$G, N> {
            #[inline]
            fn into_pull_up_output(self) -> artinchip_hal::gpio::Output<'a, $G, N> {
                unsafe { artinchip_hal::gpio::Output::new_pull_up(&*GPIO::ptr()) }
            }
            #[inline]
            fn into_pull_down_output(self) -> artinchip_hal::gpio::Output<'a, $G, N> {
                unsafe { artinchip_hal::gpio::Output::new_pull_down(&*GPIO::ptr()) }
            }
            #[inline]
            fn into_floating_output(self) -> artinchip_hal::gpio::Output<'a, $G, N> {
                unsafe { artinchip_hal::gpio::Output::new_floating(&*GPIO::ptr()) }
            }
            #[inline]
            fn into_pull_up_input(self) -> artinchip_hal::gpio::Input<'a, $G, N> {
                unsafe { artinchip_hal::gpio::Input::new_pull_up(&*GPIO::ptr()) }
            }
            #[inline]
            fn into_pull_down_input(self) -> artinchip_hal::gpio::Input<'a, $G, N> {
                unsafe { artinchip_hal::gpio::Input::new_pull_down(&*GPIO::ptr()) }
            }
            #[inline]
            fn into_floating_input(self) -> artinchip_hal::gpio::Input<'a, $G, N> {
                unsafe { artinchip_hal::gpio::Input::new_floating(&*GPIO::ptr()) }
            }
            #[inline]
            fn into_function<const F: u8>(self) -> artinchip_hal::gpio::Function<'a, $G, N, F> {
                unsafe { artinchip_hal::gpio::Function::new_with_func(&*GPIO::ptr()) }
            }
        }

        impl<const N: u8> artinchip_hal::gpio::PadExt<'static, $G, N> for GpioPad<$G, N> {
            #[inline]
            fn into_pull_up_output(self) -> artinchip_hal::gpio::Output<'static, $G, N> {
                unsafe { artinchip_hal::gpio::Output::new_pull_up(&*GPIO::ptr()) }
            }
            #[inline]
            fn into_pull_down_output(self) -> artinchip_hal::gpio::Output<'static, $G, N> {
                unsafe { artinchip_hal::gpio::Output::new_pull_down(&*GPIO::ptr()) }
            }
            #[inline]
            fn into_floating_output(self) -> artinchip_hal::gpio::Output<'static, $G, N> {
                unsafe { artinchip_hal::gpio::Output::new_floating(&*GPIO::ptr()) }
            }
            #[inline]
            fn into_pull_up_input(self) -> artinchip_hal::gpio::Input<'static, $G, N> {
                unsafe { artinchip_hal::gpio::Input::new_pull_up(&*GPIO::ptr()) }
            }
            #[inline]
            fn into_pull_down_input(self) -> artinchip_hal::gpio::Input<'static, $G, N> {
                unsafe { artinchip_hal::gpio::Input::new_pull_down(&*GPIO::ptr()) }
            }
            #[inline]
            fn into_floating_input(self) -> artinchip_hal::gpio::Input<'static, $G, N> {
                unsafe { artinchip_hal::gpio::Input::new_floating(&*GPIO::ptr()) }
            }
            #[inline]
            fn into_function<const F: u8>(
                self,
            ) -> artinchip_hal::gpio::Function<'static, $G, N, F> {
                unsafe { artinchip_hal::gpio::Function::new_with_func(&*GPIO::ptr()) }
            }
        }
    };
}

#[rustfmt::skip]
gpio!(GpioAPads, 'A', [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]);
#[rustfmt::skip]
gpio!(GpioBPads, 'B', [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17]);
#[rustfmt::skip]
gpio!(GpioCPads, 'C', [0,1,2,3,4,5,6,7,8,9,10,11]);
#[rustfmt::skip]
gpio!(GpioDPads, 'D', [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27]);
#[rustfmt::skip]
gpio!(GpioEPads, 'E', [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17]);
#[rustfmt::skip]
gpio!(GpioUPads, 'U', [0,1]);

/// Artinchip D13x peripheral ownership structures available on BootROM.
pub struct Peripherals {
    /// General Purpose Input/Output peripheral group A.
    pub gpioa: GpioAPads,
    /// General Purpose Input/Output peripheral group B.
    pub gpiob: GpioBPads,
    /// General Purpose Input/Output peripheral group C.
    pub gpioc: GpioCPads,
    /// General Purpose Input/Output peripheral group D.
    pub gpiod: GpioDPads,
    /// General Purpose Input/Output peripheral group E.
    pub gpioe: GpioEPads,
    /// General Purpose Input/Output peripheral group U.
    pub gpiou: GpioUPads,
    // TODO all other peripherals.
}

impl Peripherals {
    #[doc(hidden)]
    #[inline]
    pub const fn __new() -> Self {
        Self {
            gpioa: GpioAPads::__new(),
            gpiob: GpioBPads::__new(),
            gpioc: GpioCPads::__new(),
            gpiod: GpioDPads::__new(),
            gpioe: GpioEPads::__new(),
            gpiou: GpioUPads::__new(),
        }
    }

    /// Take initialized peripherals.
    ///
    /// TODO ensure must called once.
    #[inline]
    pub const fn take() -> Self {
        Self::__new()
    }
}

/// General Purpose Input/Output peripheral.
pub struct GPIO {
    _private: (),
}

impl GPIO {
    #[inline]
    pub const fn ptr() -> *const artinchip_hal::gpio::RegisterBlock {
        0x18700000 as *const _
    }
}

/// GPIO pad with statically known GPIO group and number.
pub struct GpioPad<const G: char, const N: u8> {
    _private: PhantomData<()>,
}
