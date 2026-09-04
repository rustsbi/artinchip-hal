//! ArtInChip D21x chip series (C906, RV64).

use crate::gpio::PadExt as _;
use artinchip_hal::instances::*;
use paste::paste;

/// ArtInChip D21x peripheral ownership structures.
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
    /// General Purpose Input/Output group F.
    pub gpiof: GpioFPads,
    /// General Purpose Input/Output group G.
    pub gpiog: GpioGPads,
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
    /// Watch Dog Timer.
    pub wdog: Wdog,
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
    /// Pulse Width Modulation.
    pub pwm: PwmChannels,
    /// Core Local Interrupt.
    pub clint: Clint,
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
    /// Watch Dog Timer.
    pub struct WDOG => 0x19000000, artinchip_hal::wdog::RegisterBlock;
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
    /// Pulse Width Modulation.
    pub struct PWM => 0x19240000, artinchip_hal::pwm::RegisterBlock;
    /// Core Local Interrupt.
    pub struct CLINT => 0x24000000, artinchip_hal::clint::RegisterBlock;
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
            gpiof: GpioFPads::__new(),
            gpiog: GpioGPads::__new(),
            gpiou: GpioUPads::__new(),
            uart0: Uart::__new(UART0::ptr()),
            uart1: Uart::__new(UART1::ptr()),
            uart2: Uart::__new(UART2::ptr()),
            uart3: Uart::__new(UART3::ptr()),
            uart4: Uart::__new(UART4::ptr()),
            uart5: Uart::__new(UART5::ptr()),
            uart6: Uart::__new(UART6::ptr()),
            uart7: Uart::__new(UART7::ptr()),
            wdog: Wdog::__new(WDOG::ptr()),
            wri: Wri::__new(WRI::ptr()),
            sid: Sid::__new(SID::ptr()),
            rtc: Rtc::__new(RTC::ptr()),
            gtc: Gtc::__new(GTC::ptr()),
            i2c0: I2c::__new(I2C0::ptr()),
            i2c1: I2c::__new(I2C1::ptr()),
            i2c2: I2c::__new(I2C2::ptr()),
            pwm: PwmChannels::__new(PWM::ptr()),
            clint: Clint::__new(CLINT::ptr()),
        }
    }

    /// Take initialized peripherals.
    ///
    /// TODO ensure must called once.
    #[inline]
    pub fn take() -> Self {
        Self::__new()
    }
}

// GPIO Pads
#[rustfmt::skip]
gpio!(GpioAPads, 'A', [0,1,2,3,4,5,6,7,8,9,10,11]);
#[rustfmt::skip]
gpio!(GpioBPads, 'B', [0,1,2,3,4,5,6,7,8,9,10,11]);
#[rustfmt::skip]
gpio!(GpioCPads, 'C', [0,1,2,3,4,5,6,7]);
#[rustfmt::skip]
gpio!(GpioDPads, 'D', [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27]);
#[rustfmt::skip]
gpio!(GpioEPads, 'E', [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19]);
#[rustfmt::skip]
gpio!(GpioFPads, 'F', [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]);
#[rustfmt::skip]
gpio!(GpioGPads, 'G', [0,1,2,3]);
#[rustfmt::skip]
gpio!(GpioUPads, 'U', [0,1,2,3]);

// QSPI pin mux for D21x series.
qspi_sck!(0, ('B', 5, 3), ('E', 16, 3)); // QSPI0
qspi_mosi!(0, ('B', 4, 3), ('E', 18, 3));
qspi_miso!(0, ('B', 3, 3), ('E', 19, 3));
qspi_cs!(0, ('B', 2, 3), ('E', 17, 3));
qspi_hold!(0, ('B', 0, 3));
qspi_wp!(0, ('B', 1, 3));
qspi_sck!(1, ('B', 11, 3), ('D', 13, 4), ('D', 24, 6)); // QSPI1
qspi_mosi!(1, ('B', 10, 3), ('D', 12, 4), ('D', 26, 6));
qspi_miso!(1, ('B', 9, 3), ('D', 11, 4), ('D', 27, 6));
qspi_cs!(1, ('B', 8, 3), ('D', 10, 4), ('D', 25, 6));
qspi_hold!(1, ('B', 6, 3), ('D', 8, 4));
qspi_wp!(1, ('B', 7, 3), ('D', 9, 4));
qspi_sck!(2, ('C', 0, 4), ('D', 0, 3), ('F', 0, 3)); // QSPI2
qspi_mosi!(2, ('C', 2, 4), ('D', 2, 3), ('F', 2, 3));
qspi_miso!(2, ('C', 3, 4), ('D', 7, 3), ('F', 3, 3));
qspi_cs!(2, ('C', 1, 4), ('D', 1, 3), ('F', 1, 3));
qspi_sck!(3, ('D', 3, 3), ('D', 14, 4), ('E', 12, 3)); // QSPI3
qspi_mosi!(3, ('D', 5, 3), ('D', 16, 4), ('E', 14, 3));
qspi_miso!(3, ('D', 6, 3), ('D', 17, 4), ('E', 15, 3));
qspi_cs!(3, ('D', 4, 3), ('D', 15, 4), ('E', 13, 3));

// UART pin mux for D21x series.
uart_tx!(0, ('A', 0, 5), ('C', 2, 6), ('U', 1, 4), ('U', 3, 4)); // UART0
uart_rx!(0, ('A', 1, 5), ('C', 4, 6), ('U', 0, 4), ('U', 2, 4));
uart_tx!(1, ('A', 4, 5), ('C', 0, 5), ('D', 6, 5), ('U', 1, 5)); // UART1
uart_rx!(1, ('A', 5, 5), ('C', 1, 5), ('D', 7, 5), ('U', 0, 5));
uart_tx!(2, ('A', 8, 5), ('C', 3, 5), ('D', 8, 5), ('U', 3, 5)); // UART2
uart_rx!(2, ('A', 9, 5), ('C', 4, 5), ('D', 9, 5), ('U', 2, 5));
uart_tx!(3, ('C', 5, 5), ('D', 10, 5), ('E', 14, 5), ('F', 11, 5)); // UART3
uart_rx!(3, ('C', 6, 5), ('D', 11, 5), ('E', 15, 5), ('F', 12, 5));
uart_tx!(4, ('B', 6, 5), ('D', 12, 5), ('E', 2, 5), ('F', 14, 5)); // UART4
uart_rx!(4, ('B', 7, 5), ('D', 13, 5), ('E', 3, 5), ('F', 15, 5));
uart_tx!(5, ('B', 8, 5), ('D', 24, 5), ('E', 4, 5), ('F', 0, 5)); // UART5
uart_rx!(5, ('B', 9, 5), ('D', 25, 5), ('E', 5, 5), ('F', 1, 5));
#[rustfmt::skip]
uart_tx!(6, ('B', 10, 5), ('D', 22, 6), ('E', 6, 5), ('E', 8, 5), ('F', 4, 5)); // UART6
#[rustfmt::skip]
uart_rx!(6, ('B', 11, 5), ('D', 23, 6), ('E', 7, 5), ('E', 9, 5), ('F', 5, 5));
uart_tx!(7, ('B', 0, 5), ('D', 20, 5), ('E', 8, 5), ('F', 6, 5)); // UART7
uart_rx!(7, ('B', 1, 5), ('D', 21, 5), ('E', 9, 5), ('F', 7, 5));

// I2C pin mux for D21x series.
i2c_scl!(0, ('A', 0, 4), ('D', 6, 4), ('E', 0, 4)); // I2C0
i2c_sda!(0, ('A', 1, 4), ('D', 7, 4), ('E', 1, 4));
i2c_scl!(1, ('A', 6, 4), ('B', 0, 4), ('D', 18, 5)); // I2C1
i2c_sda!(1, ('A', 7, 4), ('B', 1, 4), ('D', 19, 5));
i2c_scl!(2, ('A', 8, 4), ('B', 6, 4), ('E', 12, 5)); // I2C2
i2c_sda!(2, ('A', 9, 4), ('B', 7, 4), ('E', 13, 5));

// PWM pin mux for D21x series.
pwm_a!(0, ('C', 0, 7), ('D', 0, 7), ('E', 0, 8)); // Channel 0
pwm_b!(0, ('C', 1, 7), ('D', 1, 7), ('E', 1, 8));
pwm_a!(1, ('C', 2, 7), ('D', 2, 7), ('E', 2, 8)); // Channel 1
pwm_b!(1, ('C', 3, 7), ('D', 3, 7), ('E', 3, 8));
pwm_a!(2, ('C', 4, 7), ('D', 4, 7), ('E', 4, 8)); // Channel 2
pwm_b!(2, ('C', 5, 7), ('D', 5, 7), ('E', 5, 8));
pwm_a!(3, ('C', 6, 7), ('D', 26, 5), ('E', 18, 5)); // Channel 3
pwm_b!(3, ('C', 7, 7), ('D', 27, 5), ('E', 19, 5));
