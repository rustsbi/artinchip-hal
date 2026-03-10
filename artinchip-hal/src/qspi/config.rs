//! QSPI configuration.

use super::register::{CsLevel, CsPin, CtrlMode};
use embedded_hal::spi::{MODE_0, Mode, Polarity};
use embedded_time::rate::Hertz;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QspiConfig {
    pub freq: Hertz,
    /// QSPI only supports modes 0 and 2.
    pub mode: Mode,
    pub ctrl_mode: CtrlMode,
    pub work_mode: WorkMode,
    pub bit_mode_en: bool,
    pub lsb_first: bool,
    pub cs_config: Option<CsConfig>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkMode {
    /// 1-bit data transfer.
    Standard,
    /// 1-bit bidirectional (half-duplex).
    ThreeWire,
    /// 2-bit data only.
    Dual,
    /// 2-bit address and data.
    DualIO,
    /// 4-bit data only.
    Quad,
    /// 4-bit address and data.
    QuadIO,
    /// 4-bit command, address and data.
    Qpi,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CsConfig {
    pub polarity: Polarity,
    pub level: CsLevel,
    pub pin: CsPin,
}

impl Default for QspiConfig {
    fn default() -> Self {
        Self {
            freq: Hertz(1_000_000),
            mode: MODE_0,
            ctrl_mode: CtrlMode::Master,
            work_mode: WorkMode::Standard,
            bit_mode_en: false,
            lsb_first: false,
            cs_config: Some(CsConfig {
                polarity: Polarity::IdleHigh,
                level: CsLevel::Low,
                pin: CsPin::Cs0,
            }),
        }
    }
}

impl QspiConfig {
    pub fn nor_flash() -> Self {
        Self {
            // TODO: higher frequency when pll clock configuration is supported.
            freq: Hertz(10_000_000),
            mode: MODE_0,
            ctrl_mode: CtrlMode::Master,
            work_mode: WorkMode::Standard,
            lsb_first: false,
            bit_mode_en: false,
            cs_config: None,
        }
    }
}
