//! Artinchip D13x chip series.

use core::marker::PhantomData;

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

// TODO macro defined GPIOA, etc.

/// General Purpose Input/Output peripheral group A.
pub struct GPIOA {
    _private: (),
}

impl GPIOA {
    #[inline]
    pub const fn ptr() -> *const artinchip_hal::gpio::GpioGroup {
        &unsafe { &*(0x18700000 as *const artinchip_hal::gpio::RegisterBlock) }.groups[0]
            as *const _
    }
}

impl core::ops::Deref for GPIOA {
    type Target = artinchip_hal::gpio::GpioGroup;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &unsafe { &*(0x18700000 as *const artinchip_hal::gpio::RegisterBlock) }.groups[0]
    }
}

/// General Purpose Input/Output peripheral group B.
pub struct GPIOB {
    _private: (),
}

impl GPIOB {
    #[inline]
    pub const fn ptr() -> *const artinchip_hal::gpio::GpioGroup {
        &unsafe { &*(0x18700000 as *const artinchip_hal::gpio::RegisterBlock) }.groups[1]
            as *const _
    }
}

impl core::ops::Deref for GPIOB {
    type Target = artinchip_hal::gpio::GpioGroup;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &unsafe { &*(0x18700000 as *const artinchip_hal::gpio::RegisterBlock) }.groups[1]
    }
}

/// General Purpose Input/Output peripheral group C.
pub struct GPIOC {
    _private: (),
}

impl GPIOC {
    #[inline]
    pub const fn ptr() -> *const artinchip_hal::gpio::GpioGroup {
        &unsafe { &*(0x18700000 as *const artinchip_hal::gpio::RegisterBlock) }.groups[2]
            as *const _
    }
}

impl core::ops::Deref for GPIOC {
    type Target = artinchip_hal::gpio::GpioGroup;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &unsafe { &*(0x18700000 as *const artinchip_hal::gpio::RegisterBlock) }.groups[2]
    }
}

/// General Purpose Input/Output peripheral group D.
pub struct GPIOD {
    _private: (),
}

impl GPIOD {
    #[inline]
    pub const fn ptr() -> *const artinchip_hal::gpio::GpioGroup {
        &unsafe { &*(0x18700000 as *const artinchip_hal::gpio::RegisterBlock) }.groups[3]
            as *const _
    }
}

impl core::ops::Deref for GPIOD {
    type Target = artinchip_hal::gpio::GpioGroup;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &unsafe { &*(0x18700000 as *const artinchip_hal::gpio::RegisterBlock) }.groups[3]
    }
}

/// General Purpose Input/Output peripheral group E.
pub struct GPIOE {
    _private: (),
}

impl GPIOE {
    #[inline]
    pub const fn ptr() -> *const artinchip_hal::gpio::GpioGroup {
        &unsafe { &*(0x18700000 as *const artinchip_hal::gpio::RegisterBlock) }.groups[4]
            as *const _
    }
}

impl core::ops::Deref for GPIOE {
    type Target = artinchip_hal::gpio::GpioGroup;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &unsafe { &*(0x18700000 as *const artinchip_hal::gpio::RegisterBlock) }.groups[4]
    }
}

/// General Purpose Input/Output peripheral group U.
pub struct GPIOU {
    _private: (),
}

impl GPIOU {
    #[inline]
    pub const fn ptr() -> *const artinchip_hal::gpio::GpioGroup {
        &unsafe { &*(0x18700000 as *const artinchip_hal::gpio::RegisterBlock) }.groups[14]
            as *const _
    }
}

impl core::ops::Deref for GPIOU {
    type Target = artinchip_hal::gpio::GpioGroup;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &unsafe { &*(0x18700000 as *const artinchip_hal::gpio::RegisterBlock) }.groups[14]
    }
}

/// GPIOA pads.
pub struct GpioAPads {
    /// GPIO pad PA0.
    pub pa0: GpioPad<GPIOA, 0>,
    /// GPIO pad PA1.
    pub pa1: GpioPad<GPIOA, 1>,
    /// GPIO pad PA2.
    pub pa2: GpioPad<GPIOA, 2>,
    /// GPIO pad PA3.
    pub pa3: GpioPad<GPIOA, 3>,
    /// GPIO pad PA4.
    pub pa4: GpioPad<GPIOA, 4>,
    /// GPIO pad PA5.
    pub pa5: GpioPad<GPIOA, 5>,
    /// GPIO pad PA6.
    pub pa6: GpioPad<GPIOA, 6>,
    /// GPIO pad PA7.
    pub pa7: GpioPad<GPIOA, 7>,
    /// GPIO pad PA8.
    pub pa8: GpioPad<GPIOA, 8>,
    /// GPIO pad PA9.
    pub pa9: GpioPad<GPIOA, 9>,
    /// GPIO pad PA10.
    pub pa10: GpioPad<GPIOA, 10>,
    /// GPIO pad PA11.
    pub pa11: GpioPad<GPIOA, 11>,
    /// GPIO pad PA12.
    pub pa12: GpioPad<GPIOA, 12>,
    /// GPIO pad PA13.
    pub pa13: GpioPad<GPIOA, 13>,
    /// GPIO pad PA14.
    pub pa14: GpioPad<GPIOA, 14>,
    /// GPIO pad PA15.
    pub pa15: GpioPad<GPIOA, 15>,
}

impl GpioAPads {
    #[inline]
    const fn __new() -> Self {
        Self {
            pa0: GpioPad {
                _private: PhantomData,
            },
            pa1: GpioPad {
                _private: PhantomData,
            },
            pa2: GpioPad {
                _private: PhantomData,
            },
            pa3: GpioPad {
                _private: PhantomData,
            },
            pa4: GpioPad {
                _private: PhantomData,
            },
            pa5: GpioPad {
                _private: PhantomData,
            },
            pa6: GpioPad {
                _private: PhantomData,
            },
            pa7: GpioPad {
                _private: PhantomData,
            },
            pa8: GpioPad {
                _private: PhantomData,
            },
            pa9: GpioPad {
                _private: PhantomData,
            },
            pa10: GpioPad {
                _private: PhantomData,
            },
            pa11: GpioPad {
                _private: PhantomData,
            },
            pa12: GpioPad {
                _private: PhantomData,
            },
            pa13: GpioPad {
                _private: PhantomData,
            },
            pa14: GpioPad {
                _private: PhantomData,
            },
            pa15: GpioPad {
                _private: PhantomData,
            },
        }
    }
}

/// GPIOB pads.
pub struct GpioBPads {
    /// GPIO pad PB0.
    pub pb0: GpioPad<GPIOB, 0>,
    /// GPIO pad PB1.
    pub pb1: GpioPad<GPIOB, 1>,
    /// GPIO pad PB2.
    pub pb2: GpioPad<GPIOB, 2>,
    /// GPIO pad PB3.
    pub pb3: GpioPad<GPIOB, 3>,
    /// GPIO pad PB4.
    pub pb4: GpioPad<GPIOB, 4>,
    /// GPIO pad PB5.
    pub pb5: GpioPad<GPIOB, 5>,
    /// GPIO pad PB6.
    pub pb6: GpioPad<GPIOB, 6>,
    /// GPIO pad PB7.
    pub pb7: GpioPad<GPIOB, 7>,
    /// GPIO pad PB8.
    pub pb8: GpioPad<GPIOB, 8>,
    /// GPIO pad PB9.
    pub pb9: GpioPad<GPIOB, 9>,
    /// GPIO pad PB10.
    pub pb10: GpioPad<GPIOB, 10>,
    /// GPIO pad PB11.
    pub pb11: GpioPad<GPIOB, 11>,
    /// GPIO pad PB12.
    pub pb12: GpioPad<GPIOB, 12>,
    /// GPIO pad PB13.
    pub pb13: GpioPad<GPIOB, 13>,
    /// GPIO pad PB14.
    pub pb14: GpioPad<GPIOB, 14>,
    /// GPIO pad PB15.
    pub pb15: GpioPad<GPIOB, 15>,
    /// GPIO pad PB16.
    pub pb16: GpioPad<GPIOB, 16>,
    /// GPIO pad PB17.
    pub pb17: GpioPad<GPIOB, 17>,
}

impl GpioBPads {
    #[inline]
    const fn __new() -> Self {
        Self {
            pb0: GpioPad {
                _private: PhantomData,
            },
            pb1: GpioPad {
                _private: PhantomData,
            },
            pb2: GpioPad {
                _private: PhantomData,
            },
            pb3: GpioPad {
                _private: PhantomData,
            },
            pb4: GpioPad {
                _private: PhantomData,
            },
            pb5: GpioPad {
                _private: PhantomData,
            },
            pb6: GpioPad {
                _private: PhantomData,
            },
            pb7: GpioPad {
                _private: PhantomData,
            },
            pb8: GpioPad {
                _private: PhantomData,
            },
            pb9: GpioPad {
                _private: PhantomData,
            },
            pb10: GpioPad {
                _private: PhantomData,
            },
            pb11: GpioPad {
                _private: PhantomData,
            },
            pb12: GpioPad {
                _private: PhantomData,
            },
            pb13: GpioPad {
                _private: PhantomData,
            },
            pb14: GpioPad {
                _private: PhantomData,
            },
            pb15: GpioPad {
                _private: PhantomData,
            },
            pb16: GpioPad {
                _private: PhantomData,
            },
            pb17: GpioPad {
                _private: PhantomData,
            },
        }
    }
}

/// GPIOC pads.
pub struct GpioCPads {
    /// GPIO pad PC0.
    pub pc0: GpioPad<GPIOC, 0>,
    /// GPIO pad PC1.
    pub pc1: GpioPad<GPIOC, 1>,
    /// GPIO pad PC2.
    pub pc2: GpioPad<GPIOC, 2>,
    /// GPIO pad PC3.
    pub pc3: GpioPad<GPIOC, 3>,
    /// GPIO pad PC4.
    pub pc4: GpioPad<GPIOC, 4>,
    /// GPIO pad PC5.
    pub pc5: GpioPad<GPIOC, 5>,
    /// GPIO pad PC6.
    pub pc6: GpioPad<GPIOC, 6>,
    /// GPIO pad PC7.
    pub pc7: GpioPad<GPIOC, 7>,
    /// GPIO pad PC8.
    pub pc8: GpioPad<GPIOC, 8>,
    /// GPIO pad PC9.
    pub pc9: GpioPad<GPIOC, 9>,
    /// GPIO pad PC10.
    pub pc10: GpioPad<GPIOC, 10>,
    /// GPIO pad PC11.
    pub pc11: GpioPad<GPIOC, 11>,
}

impl GpioCPads {
    #[inline]
    const fn __new() -> Self {
        Self {
            pc0: GpioPad {
                _private: PhantomData,
            },
            pc1: GpioPad {
                _private: PhantomData,
            },
            pc2: GpioPad {
                _private: PhantomData,
            },
            pc3: GpioPad {
                _private: PhantomData,
            },
            pc4: GpioPad {
                _private: PhantomData,
            },
            pc5: GpioPad {
                _private: PhantomData,
            },
            pc6: GpioPad {
                _private: PhantomData,
            },
            pc7: GpioPad {
                _private: PhantomData,
            },
            pc8: GpioPad {
                _private: PhantomData,
            },
            pc9: GpioPad {
                _private: PhantomData,
            },
            pc10: GpioPad {
                _private: PhantomData,
            },
            pc11: GpioPad {
                _private: PhantomData,
            },
        }
    }
}

/// GPIOD pads.
pub struct GpioDPads {
    /// GPIO pad PD0.
    pub pd0: GpioPad<GPIOD, 0>,
    /// GPIO pad PD1.
    pub pd1: GpioPad<GPIOD, 1>,
    /// GPIO pad PD2.
    pub pd2: GpioPad<GPIOD, 2>,
    /// GPIO pad PD3.
    pub pd3: GpioPad<GPIOD, 3>,
    /// GPIO pad PD4.
    pub pd4: GpioPad<GPIOD, 4>,
    /// GPIO pad PD5.
    pub pd5: GpioPad<GPIOD, 5>,
    /// GPIO pad PD6.
    pub pd6: GpioPad<GPIOD, 6>,
    /// GPIO pad PD7.
    pub pd7: GpioPad<GPIOD, 7>,
    /// GPIO pad PD8.
    pub pd8: GpioPad<GPIOD, 8>,
    /// GPIO pad PD9.
    pub pd9: GpioPad<GPIOD, 9>,
    /// GPIO pad PD10.
    pub pd10: GpioPad<GPIOD, 10>,
    /// GPIO pad PD11.
    pub pd11: GpioPad<GPIOD, 11>,
    /// GPIO pad PD12.
    pub pd12: GpioPad<GPIOD, 12>,
    /// GPIO pad PD13.
    pub pd13: GpioPad<GPIOD, 13>,
    /// GPIO pad PD14.
    pub pd14: GpioPad<GPIOD, 14>,
    /// GPIO pad PD15.
    pub pd15: GpioPad<GPIOD, 15>,
    /// GPIO pad PD16.
    pub pd16: GpioPad<GPIOD, 16>,
    /// GPIO pad PD17.
    pub pd17: GpioPad<GPIOD, 17>,
    /// GPIO pad PD18.
    pub pd18: GpioPad<GPIOD, 18>,
    /// GPIO pad PD19.
    pub pd19: GpioPad<GPIOD, 19>,
    /// GPIO pad PD20.
    pub pd20: GpioPad<GPIOD, 20>,
    /// GPIO pad PD21.
    pub pd21: GpioPad<GPIOD, 21>,
    /// GPIO pad PD22.
    pub pd22: GpioPad<GPIOD, 22>,
    /// GPIO pad PD23.
    pub pd23: GpioPad<GPIOD, 23>,
    /// GPIO pad PD24.
    pub pd24: GpioPad<GPIOD, 24>,
    /// GPIO pad PD25.
    pub pd25: GpioPad<GPIOD, 25>,
    /// GPIO pad PD26.
    pub pd26: GpioPad<GPIOD, 26>,
    /// GPIO pad PD27.
    pub pd27: GpioPad<GPIOD, 27>,
}

impl GpioDPads {
    #[inline]
    const fn __new() -> Self {
        Self {
            pd0: GpioPad {
                _private: PhantomData,
            },
            pd1: GpioPad {
                _private: PhantomData,
            },
            pd2: GpioPad {
                _private: PhantomData,
            },
            pd3: GpioPad {
                _private: PhantomData,
            },
            pd4: GpioPad {
                _private: PhantomData,
            },
            pd5: GpioPad {
                _private: PhantomData,
            },
            pd6: GpioPad {
                _private: PhantomData,
            },
            pd7: GpioPad {
                _private: PhantomData,
            },
            pd8: GpioPad {
                _private: PhantomData,
            },
            pd9: GpioPad {
                _private: PhantomData,
            },
            pd10: GpioPad {
                _private: PhantomData,
            },
            pd11: GpioPad {
                _private: PhantomData,
            },
            pd12: GpioPad {
                _private: PhantomData,
            },
            pd13: GpioPad {
                _private: PhantomData,
            },
            pd14: GpioPad {
                _private: PhantomData,
            },
            pd15: GpioPad {
                _private: PhantomData,
            },
            pd16: GpioPad {
                _private: PhantomData,
            },
            pd17: GpioPad {
                _private: PhantomData,
            },
            pd18: GpioPad {
                _private: PhantomData,
            },
            pd19: GpioPad {
                _private: PhantomData,
            },
            pd20: GpioPad {
                _private: PhantomData,
            },
            pd21: GpioPad {
                _private: PhantomData,
            },
            pd22: GpioPad {
                _private: PhantomData,
            },
            pd23: GpioPad {
                _private: PhantomData,
            },
            pd24: GpioPad {
                _private: PhantomData,
            },
            pd25: GpioPad {
                _private: PhantomData,
            },
            pd26: GpioPad {
                _private: PhantomData,
            },
            pd27: GpioPad {
                _private: PhantomData,
            },
        }
    }
}

/// GPIOE pads.
pub struct GpioEPads {
    /// GPIO pad PE0.
    pub pe0: GpioPad<GPIOE, 0>,
    /// GPIO pad PE1.
    pub pe1: GpioPad<GPIOE, 1>,
    /// GPIO pad PE2.
    pub pe2: GpioPad<GPIOE, 2>,
    /// GPIO pad PE3.
    pub pe3: GpioPad<GPIOE, 3>,
    /// GPIO pad PE4.
    pub pe4: GpioPad<GPIOE, 4>,
    /// GPIO pad PE5.
    pub pe5: GpioPad<GPIOE, 5>,
    /// GPIO pad PE6.
    pub pe6: GpioPad<GPIOE, 6>,
    /// GPIO pad PE7.
    pub pe7: GpioPad<GPIOE, 7>,
    /// GPIO pad PE8.
    pub pe8: GpioPad<GPIOE, 8>,
    /// GPIO pad PE9.
    pub pe9: GpioPad<GPIOE, 9>,
    /// GPIO pad PE10.
    pub pe10: GpioPad<GPIOE, 10>,
    /// GPIO pad PE11.
    pub pe11: GpioPad<GPIOE, 11>,
    /// GPIO pad PE12.
    pub pe12: GpioPad<GPIOE, 12>,
    /// GPIO pad PE13.
    pub pe13: GpioPad<GPIOE, 13>,
    /// GPIO pad PE14.
    pub pe14: GpioPad<GPIOE, 14>,
    /// GPIO pad PE15.
    pub pe15: GpioPad<GPIOE, 15>,
    /// GPIO pad PE16.
    pub pe16: GpioPad<GPIOE, 16>,
    /// GPIO pad PE17.
    pub pe17: GpioPad<GPIOE, 17>,
}

impl GpioEPads {
    #[inline]
    const fn __new() -> Self {
        Self {
            pe0: GpioPad {
                _private: PhantomData,
            },
            pe1: GpioPad {
                _private: PhantomData,
            },
            pe2: GpioPad {
                _private: PhantomData,
            },
            pe3: GpioPad {
                _private: PhantomData,
            },
            pe4: GpioPad {
                _private: PhantomData,
            },
            pe5: GpioPad {
                _private: PhantomData,
            },
            pe6: GpioPad {
                _private: PhantomData,
            },
            pe7: GpioPad {
                _private: PhantomData,
            },
            pe8: GpioPad {
                _private: PhantomData,
            },
            pe9: GpioPad {
                _private: PhantomData,
            },
            pe10: GpioPad {
                _private: PhantomData,
            },
            pe11: GpioPad {
                _private: PhantomData,
            },
            pe12: GpioPad {
                _private: PhantomData,
            },
            pe13: GpioPad {
                _private: PhantomData,
            },
            pe14: GpioPad {
                _private: PhantomData,
            },
            pe15: GpioPad {
                _private: PhantomData,
            },
            pe16: GpioPad {
                _private: PhantomData,
            },
            pe17: GpioPad {
                _private: PhantomData,
            },
        }
    }
}

/// GPIOU pads.
pub struct GpioUPads {
    /// GPIO pad PU0.
    pub pu0: GpioPad<GPIOU, 0>,
    /// GPIO pad PU1.
    pub pu1: GpioPad<GPIOU, 1>,
}

impl GpioUPads {
    #[inline]
    const fn __new() -> Self {
        Self {
            pu0: GpioPad {
                _private: PhantomData,
            },
            pu1: GpioPad {
                _private: PhantomData,
            },
        }
    }
}

/// GPIO pad with statically known GPIO number.
pub struct GpioPad<T, const N: u8> {
    _private: PhantomData<T>,
}

// TODO macro defined GpioPad<GPIOx>.
// TODO other ownership modules.

macro_rules! pad_ext {
    ($gpio:ident) => {
        impl<'a, const N: u8> artinchip_hal::gpio::PadExt<'a> for &'a mut GpioPad<$gpio, N> {
            #[inline]
            fn into_output(self) -> artinchip_hal::gpio::Output<'a> {
                unsafe { artinchip_hal::gpio::Output::__new(N, &*$gpio::ptr()) }
            }
        }

        impl<const N: u8> artinchip_hal::gpio::PadExt<'static> for GpioPad<$gpio, N> {
            #[inline]
            fn into_output(self) -> artinchip_hal::gpio::Output<'static> {
                unsafe { artinchip_hal::gpio::Output::__new(N, &*$gpio::ptr()) }
            }
        }
    };
}

pad_ext!(GPIOA);
pad_ext!(GPIOB);
pad_ext!(GPIOC);
pad_ext!(GPIOD);
pad_ext!(GPIOE);
pad_ext!(GPIOU);
