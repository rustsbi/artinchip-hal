//! SoC peripheral support for ArtInChip chips.
#![no_std]

pub mod axi_cfg;
pub mod ce;
pub mod clic;
pub mod clint;
pub mod cmu;
pub mod dma;
pub mod gpio;
pub mod gtc;
pub mod i2c;
pub mod qspi;
pub mod rtc;
pub mod sdmc;
pub mod sid;
pub mod spi_enc;
pub mod sys_cfg;
pub mod uart;
pub mod wri;
pub mod xspi;

mod macros;

/// ArtInChip HAL prelude.
pub mod prelude {
    pub use crate::gpio::PadExt as _;
    pub use crate::gtc::GtcExt as _;
    pub use crate::uart::UartExt as _;
    pub use embedded_hal::digital::{InputPin as _, OutputPin as _, StatefulOutputPin as _};
    pub use embedded_hal::i2c::I2c as _;
}

/// ArtInChip HAL instances.
pub mod instances {
    pub use crate::axi_cfg::AxiCfg;
    pub use crate::ce::Ce;
    pub use crate::clic::Clic;
    pub use crate::clint::Clint;
    pub use crate::cmu::Cmu;
    pub use crate::dma::Dma;
    pub use crate::gtc::Gtc;
    pub use crate::i2c::I2c;
    pub use crate::qspi::Qspi;
    pub use crate::rtc::Rtc;
    pub use crate::sdmc::Sdmc;
    pub use crate::sid::Sid;
    pub use crate::spi_enc::SpiEnc;
    pub use crate::sys_cfg::SysCfg;
    pub use crate::uart::Uart;
    pub use crate::wri::Wri;
    pub use crate::xspi::Xspi;
}
