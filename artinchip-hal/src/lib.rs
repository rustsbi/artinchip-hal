//! SoC peripheral support for ArtInChip chips.
#![no_std]
#![cfg_attr(
    all(
        any(target_arch = "riscv32", target_arch = "riscv64"),
        feature = "interrupts"
    ),
    feature(abi_riscv_interrupt)
)]

#[cfg(any(
    all(feature = "d12x", feature = "d13x"),
    all(feature = "d12x", feature = "d21x"),
    all(feature = "d13x", feature = "d21x"),
))]
compile_error!("Enable only one SoC feature: d12x, d13x, or d21x.");

#[cfg(all(feature = "d12x", feature = "async-usb"))]
compile_error!("D12x does not support the `async-usb` feature.");

// Ensure `riscv` (`critical-section-single-hart`) is linked for downstream crates.
use riscv as _;

pub mod aic_upg;
pub mod axi_cfg;
pub mod cache;
pub mod ce;
pub mod clic;
pub mod clint;
pub mod cmu;
pub mod dma;
pub mod dram;
pub mod gpio;
pub mod gtc;
pub mod i2c;
#[cfg(feature = "interrupts")]
pub mod interrupt;
pub mod pad;
pub mod pwm;
pub mod qspi;
pub mod rtc;
pub mod sdmc;
pub mod sid;
pub mod spi_enc;
pub mod spi_flash;
pub mod sys_cfg;
pub mod system;
pub mod types;
pub mod uart;
#[cfg(not(feature = "d12x"))]
pub mod usbdrd;
pub mod wdog;
pub mod wri;
pub mod xspi;

mod macros;

/// ArtInChip HAL prelude.
pub mod prelude {
    pub use crate::pad::NoPad;
    pub use crate::traits::*;
}

/// ArtInChip HAL traits.
pub mod traits {
    pub use crate::dma::DmaExt as _;
    pub use crate::dram::DramExt as _;
    pub use crate::gtc::GtcExt as _;
    pub use crate::pwm::PwmExt as _;
    pub use crate::qspi::QspiExt as _;
    pub use crate::rtc::RtcExt as _;
    pub use crate::uart::UartExt as _;
    #[cfg(not(feature = "d12x"))]
    pub use crate::usbdrd::device::UsbDevExt as _;
    pub use crate::wdog::WdogExt as _;
    pub use crate::wri::WriExt as _;
    pub use crate::xspi::XspiExt as _;
    pub use embedded_hal::delay::DelayNs as _;
    pub use embedded_hal::digital::{InputPin as _, OutputPin as _, StatefulOutputPin as _};
    pub use embedded_hal::i2c::I2c as _;
}

/// ArtInChip HAL instances.
pub mod instances {
    pub use crate::axi_cfg::AxiCfg;
    pub use crate::ce::Ce;
    pub use crate::clic::Clic;
    pub use crate::clint::{ClintV1, ClintV2};
    pub use crate::cmu::Cmu;
    pub use crate::dma::Dma;
    pub use crate::dram::{Dram, DramDriver};
    pub use crate::gtc::Gtc;
    pub use crate::i2c::I2c;
    pub use crate::pwm::{PwmChannel, PwmChannels};
    pub use crate::qspi::Qspi;
    pub use crate::rtc::Rtc;
    pub use crate::sdmc::Sdmc;
    pub use crate::sid::Sid;
    pub use crate::spi_enc::SpiEnc;
    pub use crate::sys_cfg::{SocSysCfg, SysCfg, V1, V2};
    pub use crate::uart::Uart;
    #[cfg(not(feature = "d12x"))]
    pub use crate::usbdrd::device::UsbDev;
    #[cfg(not(feature = "d12x"))]
    pub use crate::usbdrd::host::UsbHost;
    pub use crate::wdog::Wdog;
    pub use crate::wri::Wri;
    pub use crate::xspi::{Xspi, XspiDriver};
}
