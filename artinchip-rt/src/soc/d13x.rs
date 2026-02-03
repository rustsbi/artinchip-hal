//! ArtInChip D13x chip series.

use artinchip_hal::instances::*;
use paste::paste;

/// ArtInChip D13x peripheral ownership structures available on BootROM.
pub struct Peripherals {
    /// Direct Memory Access.
    pub dma: Dma,
    /// Crypto Engine.
    pub ce: Ce,
    /// Expanded Serial Peripheral Interface.
    pub xspi: Xspi,
    /// Quad Serial Peripheral Interface 0.
    pub qspi0: Qspi<0>,
    /// Quad Serial Peripheral Interface 1.
    pub qspi1: Qspi<1>,
    /// Quad Serial Peripheral Interface 2.
    pub qspi2: Qspi<2>,
    /// Quad Serial Peripheral Interface 3.
    pub qspi3: Qspi<3>,
    /// Secure Digital Host Controller 0.
    pub sdmc0: Sdmc<0>,
    /// Secure Digital Host Controller 1.
    pub sdmc1: Sdmc<1>,
    /// System Configuration.
    pub syscfg: SysCfg,
    /// Clock Manage Unit.
    pub cmu: Cmu,
    /// Serial Peripheral Interface Encryption.
    pub spi_enc: SpiEnc,
    /// AXI Configuration.
    pub axi_cfg: AxiCfg,
    /// General Purpose Input/Output group A.
    pub gpioa: GpioAPads,
    /// General Purpose Input/Output group B.
    pub gpiob: GpioBPads,
    /// General Purpose Input/Output group C.
    pub gpioc: GpioCPads,
    /// General Purpose Input/Output group D.
    pub gpiod: GpioDPads,
    /// General Purpose Input/Output group E.
    pub gpioe: GpioEPads,
    /// General Purpose Input/Output group U.
    pub gpiou: GpioUPads,
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
    /// Warm Reset Information.
    pub wri: Wri,
    /// Secure ID.
    pub sid: Sid,
    /// Real Time Clock.
    pub rtc: Rtc,
    /// Generic Timer Controller.
    pub gtc: Gtc,
    /// Inter-Integrated Circuit 0.
    pub i2c0: I2c<0>,
    /// Inter-Integrated Circuit 1.
    pub i2c1: I2c<1>,
    /// Inter-Integrated Circuit 2.
    pub i2c2: I2c<2>,
    /// Core Local Interrupt.
    pub clint: Clint,
    /// Core Local Interrupt Controller.
    pub clic: Clic,
}

soc! {
    /// Direct Memory Access.
    pub struct DMA => 0x10000000, artinchip_hal::dma::RegisterBlock;
    /// Crypto Engine.
    pub struct CE => 0x10020000, artinchip_hal::ce::RegisterBlock;
    /// Expanded Serial Peripheral Interface.
    pub struct XSPI => 0x10300000, artinchip_hal::xspi::RegisterBlock;
    /// Quad Serial Peripheral Interface 0.
    pub struct QSPI0 => 0x10400000, artinchip_hal::qspi::RegisterBlock;
    /// Quad Serial Peripheral Interface 1.
    pub struct QSPI1 => 0x10410000, artinchip_hal::qspi::RegisterBlock;
    /// Quad Serial Peripheral Interface 2.
    pub struct QSPI2 => 0x10420000, artinchip_hal::qspi::RegisterBlock;
    /// Quad Serial Peripheral Interface 3.
    pub struct QSPI3 => 0x10430000, artinchip_hal::qspi::RegisterBlock;
    /// Secure Digital Host Controller 0.
    pub struct SDMC0 => 0x10440000, artinchip_hal::sdmc::RegisterBlock;
    /// Secure Digital Host Controller 1.
    pub struct SDMC1 => 0x10450000, artinchip_hal::sdmc::RegisterBlock;
    /// System Configuration.
    pub struct SYSCFG => 0x18000000, artinchip_hal::sys_cfg::RegisterBlock;
    /// Clock Manage Unit.
    pub struct CMU => 0x18020000, artinchip_hal::cmu::RegisterBlock;
    /// Serial Peripheral Interface Encryption.
    pub struct SPI_ENC => 0x18100000, artinchip_hal::spi_enc::RegisterBlock;
    /// AXI Configuration.
    pub struct AXICFG => 0x184FE000, artinchip_hal::axi_cfg::RegisterBlock;
    /// General Purpose Input/Output.
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
    /// Warm Reset Information.
    pub struct WRI => 0x1900F000, artinchip_hal::wri::RegisterBlock;
    /// Secure ID.
    pub struct SID => 0x19010000, artinchip_hal::sid::RegisterBlock;
    /// Real Time Clock.
    pub struct RTC => 0x19030000, artinchip_hal::rtc::RegisterBlock;
    /// Generic Timer Controller.
    pub struct GTC => 0x19050000, artinchip_hal::gtc::RegisterBlock;
    /// Inter-Integrated Circuit 0.
    pub struct I2C0 => 0x19220000, artinchip_hal::i2c::RegisterBlock;
    /// Inter-Integrated Circuit 1.
    pub struct I2C1 => 0x19221000, artinchip_hal::i2c::RegisterBlock;
    /// Inter-Integrated Circuit 2.
    pub struct I2C2 => 0x19222000, artinchip_hal::i2c::RegisterBlock;
    /// Core Local Interrupt.
    pub struct CLINT => 0x20000000, artinchip_hal::clint::RegisterBlock;
    /// Core Local Interrupt Controller.
    pub struct CLIC => 0x20800000, artinchip_hal::clic::RegisterBlock;
}

impl Peripherals {
    #[doc(hidden)]
    #[inline]
    pub const fn __new() -> Self {
        Self {
            dma: Dma::__new(DMA::ptr()),
            ce: Ce::__new(CE::ptr()),
            xspi: Xspi::__new(XSPI::ptr()),
            qspi0: Qspi::__new(QSPI0::ptr()),
            qspi1: Qspi::__new(QSPI1::ptr()),
            qspi2: Qspi::__new(QSPI2::ptr()),
            qspi3: Qspi::__new(QSPI3::ptr()),
            sdmc0: Sdmc::__new(SDMC0::ptr()),
            sdmc1: Sdmc::__new(SDMC1::ptr()),
            syscfg: SysCfg::__new(SYSCFG::ptr()),
            cmu: Cmu::__new(CMU::ptr()),
            spi_enc: SpiEnc::__new(SPI_ENC::ptr()),
            axi_cfg: AxiCfg::__new(AXICFG::ptr()),
            gpioa: GpioAPads::__new(),
            gpiob: GpioBPads::__new(),
            gpioc: GpioCPads::__new(),
            gpiod: GpioDPads::__new(),
            gpioe: GpioEPads::__new(),
            gpiou: GpioUPads::__new(),
            uart0: Uart::__new(UART0::ptr()),
            uart1: Uart::__new(UART1::ptr()),
            uart2: Uart::__new(UART2::ptr()),
            uart3: Uart::__new(UART3::ptr()),
            uart4: Uart::__new(UART4::ptr()),
            uart5: Uart::__new(UART5::ptr()),
            uart6: Uart::__new(UART6::ptr()),
            uart7: Uart::__new(UART7::ptr()),
            wri: Wri::__new(WRI::ptr()),
            sid: Sid::__new(SID::ptr()),
            rtc: Rtc::__new(RTC::ptr()),
            gtc: Gtc::__new(GTC::ptr()),
            i2c0: I2c::__new(I2C0::ptr()),
            i2c1: I2c::__new(I2C1::ptr()),
            i2c2: I2c::__new(I2C2::ptr()),
            clint: Clint::__new(CLINT::ptr()),
            clic: Clic::__new(CLIC::ptr()),
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
