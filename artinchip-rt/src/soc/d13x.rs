//! ArtInChip D13x chip series.

use artinchip_hal::gtc::Gtc;
use artinchip_hal::uart::Uart;
use paste::paste;

/// ArtInChip D13x peripheral ownership structures available on BootROM.
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
    /// Clock Manage Unit peripheral.
    pub cmu: CMU,
    /// Universal Asynchronous Receiver-Transmitter 0.
    pub uart0: Uart<0>,
    /// Universal Asynchronous Receiver-Transmitter 1.
    pub uart1: Uart<1>,
    /// Universal Asynchronous Receiver-Transmitter 2.
    pub uart2: Uart<2>,
    /// Universal Asynchronous Receiver-Transmitter 3.
    pub uart3: Uart<3>,
    /// Universal Asynchronous Receiver-Transmitter 4.
    pub uart4: Uart<4>,
    /// Universal Asynchronous Receiver-Transmitter 5.
    pub uart5: Uart<5>,
    /// Universal Asynchronous Receiver-Transmitter 6.
    pub uart6: Uart<6>,
    /// Universal Asynchronous Receiver-Transmitter 7.
    pub uart7: Uart<7>,
    /// Generic Timer Controller.
    pub gtc: Gtc,
    // TODO all other peripherals.
}

soc! {
    /// Clock Manage Unit peripheral.
    pub struct CMU => 0x18020000, artinchip_hal::cmu::RegisterBlock;
    /// General Purpose Input/Output peripheral.
    pub struct GPIO => 0x18700000, artinchip_hal::gpio::RegisterBlock;
    /// Universal Asynchronous Receiver-Transmitter 0.
    pub struct UART0 => 0x18710000, artinchip_hal::uart::RegisterBlock;
    /// Universal Asynchronous Receiver-Transmitter 1.
    pub struct UART1 => 0x18711000, artinchip_hal::uart::RegisterBlock;
    /// Universal Asynchronous Receiver-Transmitter 2.
    pub struct UART2 => 0x18712000, artinchip_hal::uart::RegisterBlock;
    /// Universal Asynchronous Receiver-Transmitter 3.
    pub struct UART3 => 0x18713000, artinchip_hal::uart::RegisterBlock;
    /// Universal Asynchronous Receiver-Transmitter 4.
    pub struct UART4 => 0x18714000, artinchip_hal::uart::RegisterBlock;
    /// Universal Asynchronous Receiver-Transmitter 5.
    pub struct UART5 => 0x18715000, artinchip_hal::uart::RegisterBlock;
    /// Universal Asynchronous Receiver-Transmitter 6.
    pub struct UART6 => 0x18716000, artinchip_hal::uart::RegisterBlock;
    /// Universal Asynchronous Receiver-Transmitter 7.
    pub struct UART7 => 0x18717000, artinchip_hal::uart::RegisterBlock;
    /// Generic Timer Controller.
    pub struct GTC => 0x19050000, artinchip_hal::gtc::RegisterBlock;
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
            cmu: CMU { _private: () },
            uart0: Uart::__new(UART0::ptr()),
            uart1: Uart::__new(UART1::ptr()),
            uart2: Uart::__new(UART2::ptr()),
            uart3: Uart::__new(UART3::ptr()),
            uart4: Uart::__new(UART4::ptr()),
            uart5: Uart::__new(UART5::ptr()),
            uart6: Uart::__new(UART6::ptr()),
            uart7: Uart::__new(UART7::ptr()),
            gtc: Gtc::__new(GTC::ptr()),
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

// GPIO Pads
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
