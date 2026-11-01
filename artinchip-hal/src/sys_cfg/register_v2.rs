//! SYSCFG v2 register block (D21x).
//!
//! Reference: D21x System Configuration register list/description,
//! https://aicdoc.artinchip.com/topics/ic/syscfg/system_configuration_syscfg-d21x.html

use volatile_register::{RO, RW};

pub use super::common::*;

/// SYSCFG v2 register block.
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x20],
    /// LDO30 configuration (`LDO30_CFG`).
    #[doc(alias = "LDO30_CFG")]
    pub ldo30_cfg: RW<Ldo30Config>,
    /// LDO25 configuration (`LDO25_CFG`).
    #[doc(alias = "LDO25_CFG")]
    pub ldo25_cfg: RW<Ldo25Config>,
    /// LDO1x configuration (`LDO1x_CFG`, `VCC_DRAM` rail).
    #[doc(alias = "LDO1x_CFG")]
    pub ldo1x_cfg: RW<Ldo1xConfig>,
    _reserved1: [u8; 0x14],
    /// DDR reference voltage (`DDR_VREF`).
    #[doc(alias = "DDR_VREF")]
    pub ddr_vref: RW<DdrVref>,
    /// DDR external resistor (`DDR_REXT`).
    #[doc(alias = "DDR_REXT")]
    pub ddr_rext: RW<ExternalResistor>,
    /// USB0 external resistor (`USB0_REXT`).
    ///
    /// Trimmed by the DRAM analog calibration; the D21x register table only
    /// lists `USB1_REXT`, so this offset follows the programming model used by
    /// the current DRAM code.
    #[doc(alias = "USB0_REXT")]
    pub usb0_rext: RW<ExternalResistor>,
    /// USB1 external resistor (`USB1_REXT`).
    #[doc(alias = "USB1_REXT")]
    pub usb1_rext: RW<ExternalResistor>,
    _reserved2: [u8; 0x70],
    /// Process sensor configuration (`PSEN_CFG`).
    #[doc(alias = "PSEN_CFG")]
    pub psen_cfg: RW<PsenConfig>,
    /// Process sensor counter value (`PSEN_CNT_VAL`).
    #[doc(alias = "PSEN_CNT_VAL")]
    pub psen_cnt_val: RO<PsenCntVal>,
    _reserved3: [u8; 0x78],
    /// SRAM clock configuration (`SRAM_CLK_CFG`).
    #[doc(alias = "SRAM_CLK_CFG")]
    pub sram_clk_cfg: RW<SramClkConfig>,
    _reserved4: [u8; 0x2C8],
    /// USB0 configuration (`USB0_CFG`).
    #[doc(alias = "USB0_CFG")]
    pub usb0_cfg: RW<Usb0Config>,
    /// GMAC0 configuration (`GMAC0_CFG0`).
    #[doc(alias = "GMAC0_CFG0")]
    pub gmac0_cfg: RW<EmacConfig>,
    /// GMAC1 configuration (`GMAC1_CFG`).
    #[doc(alias = "GMAC1_CFG")]
    pub gmac1_cfg: RW<EmacConfig>,
    _reserved5: [u8; 0xBE4],
    /// SYSCFG version (`SYSCFG_VER`).
    #[doc(alias = "SYSCFG_VER")]
    pub version: RO<u32>,
}

/// `LDO30_CFG` voltage select (`LDO_VAL`, `[2:0]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ldo30Voltage {
    /// 2.80 V.
    Mv2800,
    /// 2.85 V.
    Mv2850,
    /// 2.90 V.
    Mv2900,
    /// 2.95 V.
    Mv2950,
    /// 3.00 V.
    Mv3000,
    /// 3.05 V.
    Mv3050,
    /// 3.10 V.
    Mv3100,
    /// 3.15 V.
    Mv3150,
}

impl Ldo30Voltage {
    /// Voltage in millivolts.
    pub const fn millivolts(self) -> u16 {
        2800 + (self as u16) * 50
    }

    const fn from_code(code: u32) -> Self {
        match code & 0x7 {
            0 => Self::Mv2800,
            1 => Self::Mv2850,
            2 => Self::Mv2900,
            3 => Self::Mv2950,
            4 => Self::Mv3000,
            5 => Self::Mv3050,
            6 => Self::Mv3100,
            _ => Self::Mv3150,
        }
    }
}

/// `LDO30_CFG` register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Ldo30Config(u32);

impl Ldo30Config {
    /// Zeroed register value.
    pub const ZERO: Self = Self(0);

    const ATB_ANA_EN: u32 = 0x1 << 27;
    const ATB_ANA_SEL: u32 = 0x7 << 24;
    const ATB_BIAS_EN: u32 = 0x1 << 22;
    const ATB_BIAS_SEL: u32 = 0x3 << 20;
    const LVDS0_IBIAS_EN: u32 = 0x1 << 18;
    const LVDS1_IBIAS_EN: u32 = 0x1 << 17;
    const BAK_IBIAS_EN: u32 = 0x1 << 16;
    const BG_CTRL: u32 = 0xFF << 8;
    const RTC_VAL_SEL: u32 = 0x7 << 4;
    const ENABLE: u32 = 0x1 << 3;
    const VOLTAGE: u32 = 0x7;

    /// Build a configuration from raw bits.
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }
    /// Raw register value.
    pub const fn raw(self) -> u32 {
        self.0
    }
    /// Get `BG_CTRL` trim (`[15:8]`).
    pub const fn bg_ctrl(self) -> u8 {
        ((self.0 & Self::BG_CTRL) >> 8) as u8
    }
    /// Set `BG_CTRL` trim (`[15:8]`).
    pub const fn set_bg_ctrl(self, value: u8) -> Self {
        Self((self.0 & !Self::BG_CTRL) | Self::BG_CTRL & ((value as u32) << 8))
    }
    /// Enable the LDO (`LDO_AVCC_EN`).
    pub const fn enable(self) -> Self {
        Self(self.0 | Self::ENABLE)
    }
    /// Disable the LDO.
    pub const fn disable(self) -> Self {
        Self(self.0 & !Self::ENABLE)
    }
    /// Whether the LDO is enabled.
    pub const fn is_enabled(self) -> bool {
        self.0 & Self::ENABLE != 0
    }
    /// Set the output voltage (`LDO_VAL`).
    pub const fn set_voltage(self, voltage: Ldo30Voltage) -> Self {
        Self((self.0 & !Self::VOLTAGE) | (voltage as u32))
    }
    /// Output voltage (`LDO_VAL`).
    pub const fn voltage(self) -> Ldo30Voltage {
        Ldo30Voltage::from_code(self.0)
    }
    /// Whether the analog-test path is enabled (`ATB_ANA_EN`).
    pub const fn is_atb_ana_enabled(self) -> bool {
        self.0 & Self::ATB_ANA_EN != 0
    }
    /// Whether the LVDS0 reference current is enabled (`LVDS0_IBIAS_EN`).
    pub const fn is_lvds0_ibias_enabled(self) -> bool {
        self.0 & Self::LVDS0_IBIAS_EN != 0
    }
    /// Whether the LVDS1 reference current is enabled (`LVDS1_IBIAS_EN`).
    pub const fn is_lvds1_ibias_enabled(self) -> bool {
        self.0 & Self::LVDS1_IBIAS_EN != 0
    }
    /// Whether the backup reference current is enabled (`BAK_IBIAS_EN`).
    pub const fn is_bak_ibias_enabled(self) -> bool {
        self.0 & Self::BAK_IBIAS_EN != 0
    }
    /// Whether the bias analog-test path is enabled (`ATB_BIAS_EN`).
    pub const fn is_atb_bias_enabled(self) -> bool {
        self.0 & Self::ATB_BIAS_EN != 0
    }
    /// Get `RTC_VAL_SEL` (`[6:4]`).
    pub const fn rtc_val_sel(self) -> u8 {
        ((self.0 & Self::RTC_VAL_SEL) >> 4) as u8
    }
    /// Get `ATB_ANA_SEL` (`[26:24]`).
    pub const fn atb_ana_sel(self) -> u8 {
        ((self.0 & Self::ATB_ANA_SEL) >> 24) as u8
    }
    /// Get `ATB_BIAS_SEL` (`[21:20]`).
    pub const fn atb_bias_sel(self) -> u8 {
        ((self.0 & Self::ATB_BIAS_SEL) >> 20) as u8
    }
}

/// `LDO25_CFG` voltage select (`LDO25_VAL`, `[2:0]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ldo25Voltage {
    /// 2.45 V.
    Mv2450,
    /// 2.50 V.
    Mv2500,
    /// 2.55 V.
    Mv2550,
    /// 2.60 V.
    Mv2600,
}

impl Ldo25Voltage {
    /// Voltage in millivolts.
    pub const fn millivolts(self) -> u16 {
        2450 + (self as u16) * 50
    }

    const fn from_code(code: u32) -> Self {
        match code & 0x3 {
            0 => Self::Mv2450,
            1 => Self::Mv2500,
            2 => Self::Mv2550,
            _ => Self::Mv2600,
        }
    }
}

/// `LDO25_CFG` register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Ldo25Config(u32);

impl Ldo25Config {
    /// Zeroed register value.
    pub const ZERO: Self = Self(0);

    const ATB_ANA_EN: u32 = 0x1 << 27;
    const ATB_ANA_SEL: u32 = 0x7 << 24;
    const ATB_BIAS_EN: u32 = 0x1 << 22;
    const ATB_BIAS_SEL: u32 = 0x3 << 20;
    const BG_CTRL: u32 = 0xFF << 8;
    const ENABLE: u32 = 0x1 << 3;
    const VOLTAGE: u32 = 0x7;

    /// Build a configuration from raw bits.
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }
    /// Raw register value.
    pub const fn raw(self) -> u32 {
        self.0
    }
    /// Get `BG_CTRL` trim (`[15:8]`).
    pub const fn bg_ctrl(self) -> u8 {
        ((self.0 & Self::BG_CTRL) >> 8) as u8
    }
    /// Set `BG_CTRL` trim (`[15:8]`).
    pub const fn set_bg_ctrl(self, value: u8) -> Self {
        Self((self.0 & !Self::BG_CTRL) | Self::BG_CTRL & ((value as u32) << 8))
    }
    /// Enable the LDO (`LDO25_EN`).
    pub const fn enable(self) -> Self {
        Self(self.0 | Self::ENABLE)
    }
    /// Disable the LDO.
    pub const fn disable(self) -> Self {
        Self(self.0 & !Self::ENABLE)
    }
    /// Whether the LDO is enabled.
    pub const fn is_enabled(self) -> bool {
        self.0 & Self::ENABLE != 0
    }
    /// Set the output voltage (`LDO25_VAL`).
    pub const fn set_voltage(self, voltage: Ldo25Voltage) -> Self {
        Self((self.0 & !Self::VOLTAGE) | (voltage as u32))
    }
    /// Output voltage (`LDO25_VAL`).
    pub const fn voltage(self) -> Ldo25Voltage {
        Ldo25Voltage::from_code(self.0)
    }
    /// Whether the analog-test path is enabled (`ATB_ANA_EN`).
    pub const fn is_atb_ana_enabled(self) -> bool {
        self.0 & Self::ATB_ANA_EN != 0
    }
    /// Whether the bias analog-test path is enabled (`ATB_BIAS_EN`).
    pub const fn is_atb_bias_enabled(self) -> bool {
        self.0 & Self::ATB_BIAS_EN != 0
    }
    /// Get `ATB_ANA_SEL` (`[26:24]`).
    pub const fn atb_ana_sel(self) -> u8 {
        ((self.0 & Self::ATB_ANA_SEL) >> 24) as u8
    }
    /// Get `ATB_BIAS_SEL` (`[21:20]`).
    pub const fn atb_bias_sel(self) -> u8 {
        ((self.0 & Self::ATB_BIAS_SEL) >> 20) as u8
    }
}

/// `VCC_DRAM` rail voltage select (`LDO1x_VAL`, `[2:0]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Ldo1xVoltage {
    /// 1.35 V.
    Mv1350,
    /// 1.40 V.
    Mv1400,
    /// 1.45 V.
    Mv1450,
    /// 1.50 V (DDR3).
    Mv1500,
    /// 1.55 V.
    Mv1550,
    /// 1.75 V.
    Mv1750,
    /// 1.80 V (DDR2).
    Mv1800,
    /// 1.85 V.
    Mv1850,
}

impl Ldo1xVoltage {
    /// Voltage in millivolts.
    pub const fn millivolts(self) -> u16 {
        match self {
            Self::Mv1350 => 1350,
            Self::Mv1400 => 1400,
            Self::Mv1450 => 1450,
            Self::Mv1500 => 1500,
            Self::Mv1550 => 1550,
            Self::Mv1750 => 1750,
            Self::Mv1800 => 1800,
            Self::Mv1850 => 1850,
        }
    }

    const fn from_code(code: u32) -> Self {
        match code & 0x7 {
            0 => Self::Mv1350,
            1 => Self::Mv1400,
            2 => Self::Mv1450,
            3 => Self::Mv1500,
            4 => Self::Mv1550,
            5 => Self::Mv1750,
            6 => Self::Mv1800,
            _ => Self::Mv1850,
        }
    }
}

/// `LDO1x_CFG` register (`VCC_DRAM` rail).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Ldo1xConfig(u32);

/// Legacy alias for D21x `Ldo1xConfig`.
pub type DdrLdo1xConfig = Ldo1xConfig;

/// Legacy alias for D21x `Ldo1xVoltage`.
pub type DdrLdo1xVoltage = Ldo1xVoltage;

impl Ldo1xConfig {
    /// Zeroed register value.
    pub const ZERO: Self = Self(0);

    const VAL_FB: u32 = 0x7 << 4;
    const ENABLE: u32 = 0x1 << 3;
    const VOLTAGE: u32 = 0x7;

    /// Build a rail configuration from raw bits.
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }
    /// Raw register value.
    pub const fn raw(self) -> u32 {
        self.0
    }
    /// Enable the `VCC_DRAM` rail (`LDO1x_EN`).
    pub const fn enable(self) -> Self {
        Self(self.0 | Self::ENABLE)
    }
    /// Disable the `VCC_DRAM` rail.
    pub const fn disable(self) -> Self {
        Self(self.0 & !Self::ENABLE)
    }
    /// Whether the rail is enabled (`LDO1x_CFG[3]`).
    pub const fn is_enabled(self) -> bool {
        self.0 & Self::ENABLE != 0
    }
    /// Get the feedback trim (`LDO1x_VAL_FB`, `[6:4]`).
    pub const fn val_fb(self) -> u8 {
        ((self.0 & Self::VAL_FB) >> 4) as u8
    }
    /// Set the feedback trim (`LDO1x_VAL_FB`, `[6:4]`).
    pub const fn set_val_fb(self, value: u8) -> Self {
        Self((self.0 & !Self::VAL_FB) | Self::VAL_FB & ((value as u32) << 4))
    }
    /// Set rail output voltage (`LDO1x_VAL`, `[2:0]`).
    pub const fn set_voltage(self, voltage: Ldo1xVoltage) -> Self {
        Self((self.0 & !Self::VOLTAGE) | (voltage as u32))
    }
    /// Selected rail voltage (`LDO1x_CFG[2:0]`).
    pub const fn voltage(self) -> Ldo1xVoltage {
        Ldo1xVoltage::from_code(self.0)
    }
    /// Selected rail voltage in millivolts.
    pub const fn millivolts(self) -> u16 {
        self.voltage().millivolts()
    }
}

/// `DDR_VREF` register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DdrVref(u32);

impl DdrVref {
    const RES1_SEL: u32 = 0xF << 8;
    const RES0_SEL: u32 = 0xF;

    /// Raw register value.
    pub const fn raw(self) -> u32 {
        self.0
    }
    /// Get `RES0_SEL` (`[3:0]`).
    pub const fn res0_sel(self) -> u8 {
        (self.0 & Self::RES0_SEL) as u8
    }
    /// Set `RES0_SEL` (`[3:0]`).
    pub const fn set_res0_sel(self, value: u8) -> Self {
        Self((self.0 & !Self::RES0_SEL) | (value as u32 & Self::RES0_SEL))
    }
    /// Get `RES1_SEL` (`[11:8]`).
    pub const fn res1_sel(self) -> u8 {
        ((self.0 & Self::RES1_SEL) >> 8) as u8
    }
    /// Set `RES1_SEL` (`[11:8]`).
    pub const fn set_res1_sel(self, value: u8) -> Self {
        Self((self.0 & !Self::RES1_SEL) | ((value as u32) << 8) & Self::RES1_SEL)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, ldo30_cfg), 0x020);
        assert_eq!(offset_of!(RegisterBlock, ldo25_cfg), 0x024);
        assert_eq!(offset_of!(RegisterBlock, ldo1x_cfg), 0x028);
        assert_eq!(offset_of!(RegisterBlock, ddr_vref), 0x040);
        assert_eq!(offset_of!(RegisterBlock, ddr_rext), 0x044);
        assert_eq!(offset_of!(RegisterBlock, usb0_rext), 0x048);
        assert_eq!(offset_of!(RegisterBlock, usb1_rext), 0x04C);
        assert_eq!(offset_of!(RegisterBlock, psen_cfg), 0x0C0);
        assert_eq!(offset_of!(RegisterBlock, psen_cnt_val), 0x0C4);
        assert_eq!(offset_of!(RegisterBlock, sram_clk_cfg), 0x140);
        assert_eq!(offset_of!(RegisterBlock, usb0_cfg), 0x40C);
        assert_eq!(offset_of!(RegisterBlock, gmac0_cfg), 0x410);
        assert_eq!(offset_of!(RegisterBlock, gmac1_cfg), 0x414);
        assert_eq!(offset_of!(RegisterBlock, version), 0xFFC);
    }
}
