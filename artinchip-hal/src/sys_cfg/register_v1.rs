//! SYSCFG v1 register block and LDO registers (D12x / D13x).

use volatile_register::{RO, RW};

pub use super::common::*;

/// SYSCFG v1 configuration register block.
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x8],
    /// Interrupt request control register (`IRQ_CTL`).
    #[doc(alias = "IRQ_CTL")]
    pub irq_ctrl: RW<IrqControl>,
    /// Interrupt request status register (`IRQ_STA`).
    #[doc(alias = "IRQ_STA")]
    pub irq_status: RW<IrqStatus>,
    _reserved1: [u8; 0x10],
    /// LDO25 configuration register (`LDO25_CFG`).
    #[doc(alias = "LDO25_CFG")]
    pub ldo25_cfg: RW<Ldo25Config>,
    /// LDO18 configuration register (`LDO18_CFG`).
    #[doc(alias = "LDO18_CFG")]
    pub ldo18_cfg: RW<Ldo18Config>,
    /// LDO1x configuration register (`LDO1x_CFG`).
    #[doc(alias = "LDO1x_CFG")]
    pub ldo1x_cfg: RW<Ldo1xConfig>,
    _reserved2: [u8; 0x10],
    /// Compare configuration register (`CMP_CFG`).
    #[doc(alias = "CMP_CFG")]
    pub cmp_cfg: RW<CompareConfig>,
    _reserved3: [u8; 0x8],
    /// USB0 external resistor register (`USB0_REXT`).
    #[doc(alias = "USB0_REXT")]
    pub usb0_rext: RW<ExternalResistor>,
    _reserved4: [u8; 0x74],
    /// Process sensor configuration register (`PSEN_CFG`).
    #[doc(alias = "PSEN_CFG")]
    pub psen_cfg: RW<PsenConfig>,
    /// Process sensor counter value register (`PSEN_CNT_VAL`).
    #[doc(alias = "PSEN_CNT_VAL")]
    pub psen_cnt_val: RO<PsenCntVal>,
    _reserved5: [u8; 0x38],
    /// System sram parameter register (`SYS_SRAM_PAR`).
    #[doc(alias = "SYS_SRAM_PAR")]
    pub sys_sram_par: RW<u32>,
    /// CPU sram parameter register (`CPU_SRAM_PAR`).
    #[doc(alias = "CPU_SRAM_PAR")]
    pub cpu_sram_par: RW<u32>,
    /// USB sram parameter register (`USB_SRAM_PAR`).
    #[doc(alias = "USB_SRAM_PAR")]
    pub usb_sram_par: RW<u32>,
    /// VE sram parameter register (`VE_SRAM_PAR`).
    #[doc(alias = "VE_SRAM_PAR")]
    pub ve_sram_par: RW<u32>,
    /// GE sram parameter register (`GE_SRAM_PAR`).
    #[doc(alias = "GE_SRAM_PAR")]
    pub ge_sram_par: RW<GeSramParam>,
    /// DE sram parameter register (`DE_SRAM_PAR`).
    #[doc(alias = "DE_SRAM_PAR")]
    pub de_sram_par: RW<u32>,
    _reserved6: [u8; 0x28],
    /// SRAM clock configuration register (`SRAM_CLK_CFG`).
    #[doc(alias = "SRAM_CLK_CFG")]
    pub sram_clk_cfg: RW<SramClkConfig>,
    _reserved7: [u8; 0x1C],
    /// SRAM mapping configuration register (`SRAM_MAP_CFG`).
    #[doc(alias = "SRAM_MAP_CFG")]
    pub sram_map_cfg: RW<SramMapConfig>,
    _reserved8: [u8; 0x8C],
    /// FLASH configuration register (`FLASH_CFG`).
    #[doc(alias = "FLASH_CFG")]
    pub flash_cfg: RW<FlashConfig>,
    /// ENCODER configuration register (`ENCODER_CFG`).
    #[doc(alias = "ENCODER_CFG")]
    pub encoder_cfg: RW<EncoderConfig>,
    _reserved9: [u8; 0x214],
    /// USB0 configuration register (`USB0_CFG`).
    #[doc(alias = "USB0_CFG")]
    pub usb0_cfg: RW<Usb0Config>,
    /// EMAC configuration register (`EMAC_CFG`).
    #[doc(alias = "EMAC_CFG")]
    pub emac_cfg: RW<EmacConfig>,
    _reserved10: [u8; 0xB34],
    /// ATB_CMU_ANA_TOP register (`ATB_CMU_ANA_TOP`).
    #[doc(alias = "ATB_CMU_ANA_TOP")]
    pub atb_cmu_ana_top: RW<AtbCmuAnaTop>,
    /// ATB_DLL_TOP_C register (`ATB_DLL_TOP_C`).
    #[doc(alias = "ATB_DLL_TOP_C")]
    pub atb_dll_top_c: RW<AtbDllTopC>,
    /// ATB_GPADC register (`ATB_GPADC`).
    #[doc(alias = "ATB_GPADC")]
    pub atb_gpadc: RW<AtbGpadc>,
    /// ATB_MIPI_DPHY register (`ATB_MIPI_DPHY`).
    #[doc(alias = "ATB_MIPI_DPHY")]
    pub atb_mipi_dphy: RW<AtbMipiDphy>,
    /// ATB_RTC_ANA_TOP register (`ATB_RTC_ANA_TOP`).
    #[doc(alias = "ATB_RTC_ANA_TOP")]
    pub atb_rtc_ana_top: RW<AtbRtcAnaTop>,
    /// ATB_USB_PLL_AFE register (`ATB_USB_PLL_AFE`).
    #[doc(alias = "ATB_USB_PLL_AFE")]
    pub atb_usb_pll_afe: RW<AtbUsbPllAfe>,
    /// ATB_USB_PHY_AFE register (`ATB_USB_PHY_AFE`).
    #[doc(alias = "ATB_USB_PHY_AFE")]
    pub atb_usb_phy_afe: RW<AtbUsbPhyAfe>,
    _reserved11: [u8; 0x98],
    /// SYSCFG version register (`SYSCFG_VER`).
    #[doc(alias = "SYSCFG_VER")]
    pub version: RO<u32>,
}

/// LDO25 voltage setting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ldo25Voltage {
    /// 2.40 v.
    V2_40,
    /// 2.50 v.
    V2_50,
    /// 2.60 v.
    V2_60,
    /// 2.70 v.
    V2_70,
    /// 2.80 v.
    V2_80,
    /// 2.90 v.
    V2_90,
    /// 3.00 v.
    V3_00,
    /// 3.10 v.
    V3_10,
}

/// LDO25 configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Ldo25Config(u32);

impl Ldo25Config {
    const LVDS0_IBIAS_EN: u32 = 0x1 << 18;
    const XSPI_DLLC1_IBIAS_EN: u32 = 0x1 << 17;
    const XSPI_DLLC0_IBIAS_EN: u32 = 0x1 << 16;
    const BG_CTRL: u32 = 0xFF << 8;
    const LDO25_EN: u32 = 0x1 << 4;
    const LDO25_VAL: u32 = 0x7;

    /// Enable lvds0 reference current (`LVDS0_IBIAS_EN`).
    #[doc(alias = "LVDS0_IBIAS_EN")]
    #[inline]
    pub const fn enable_lvds0_ibias(self) -> Self {
        Self(self.0 | Self::LVDS0_IBIAS_EN)
    }
    /// Disable lvds0 reference current.
    #[inline]
    pub const fn disable_lvds0_ibias(self) -> Self {
        Self(self.0 & !Self::LVDS0_IBIAS_EN)
    }
    /// Check if lvds0 reference current is enabled.
    #[inline]
    pub const fn is_lvds0_ibias_enabled(self) -> bool {
        self.0 & Self::LVDS0_IBIAS_EN != 0
    }
    /// Enable xspi dllc1 reference current (`XSPI_DLLC1_IBIAS_EN`).
    #[doc(alias = "XSPI_DLLC1_IBIAS_EN")]
    #[inline]
    pub const fn enable_xspi_dllc1_ibias(self) -> Self {
        Self(self.0 | Self::XSPI_DLLC1_IBIAS_EN)
    }
    /// Disable xspi dllc1 reference current.
    #[inline]
    pub const fn disable_xspi_dllc1_ibias(self) -> Self {
        Self(self.0 & !Self::XSPI_DLLC1_IBIAS_EN)
    }
    /// Check if xspi dllc1 reference current is enabled.
    #[inline]
    pub const fn is_xspi_dllc1_ibias_enabled(self) -> bool {
        self.0 & Self::XSPI_DLLC1_IBIAS_EN != 0
    }
    /// Enable xspi dllc0 reference current (`XSPI_DLLC0_IBIAS_EN`).
    #[doc(alias = "XSPI_DLLC0_IBIAS_EN")]
    #[inline]
    pub const fn enable_xspi_dllc0_ibias(self) -> Self {
        Self(self.0 | Self::XSPI_DLLC0_IBIAS_EN)
    }
    /// Disable xspi dllc0 reference current.
    #[inline]
    pub const fn disable_xspi_dllc0_ibias(self) -> Self {
        Self(self.0 & !Self::XSPI_DLLC0_IBIAS_EN)
    }
    /// Check if xspi dllc0 reference current is enabled.
    #[inline]
    pub const fn is_xspi_dllc0_ibias_enabled(self) -> bool {
        self.0 & Self::XSPI_DLLC0_IBIAS_EN != 0
    }
    /// Set bg voltage control (`BG_CTRL`).
    #[doc(alias = "BG_CTRL")]
    #[inline]
    pub const fn set_bg_ctrl(self, val: u8) -> Self {
        Self((self.0 & !Self::BG_CTRL) | (Self::BG_CTRL & ((val as u32) << 8)))
    }
    /// Get bg voltage control.
    #[inline]
    pub const fn bg_ctrl(self) -> u8 {
        ((self.0 & Self::BG_CTRL) >> 8) as u8
    }
    /// Enable ldo25 (`LDO25_EN`).
    #[doc(alias = "LDO25_EN")]
    #[inline]
    pub const fn enable_ldo25(self) -> Self {
        Self(self.0 | Self::LDO25_EN)
    }
    /// Disable ldo25.
    #[inline]
    pub const fn disable_ldo25(self) -> Self {
        Self(self.0 & !Self::LDO25_EN)
    }
    /// Check if ldo25 is enabled.
    #[inline]
    pub const fn is_ldo25_enabled(self) -> bool {
        self.0 & Self::LDO25_EN != 0
    }
    /// Set ldo25 voltage (`LDO25_VAL`).
    ///
    /// To avoid affecting EFUSE programming, this should be configured to 2.50v.
    #[doc(alias = "LDO25_VAL")]
    #[inline]
    pub const fn set_ldo25_voltage(self, vol: Ldo25Voltage) -> Self {
        Self((self.0 & !Self::LDO25_VAL) | (Self::LDO25_VAL & (vol as u32)))
    }
    /// Get ldo25 voltage.
    #[inline]
    pub const fn ldo25_voltage(self) -> Ldo25Voltage {
        match self.0 & Self::LDO25_VAL {
            0 => Ldo25Voltage::V2_40,
            1 => Ldo25Voltage::V2_50,
            2 => Ldo25Voltage::V2_60,
            3 => Ldo25Voltage::V2_70,
            4 => Ldo25Voltage::V2_80,
            5 => Ldo25Voltage::V2_90,
            6 => Ldo25Voltage::V3_00,
            7 => Ldo25Voltage::V3_10,
            _ => unreachable!(),
        }
    }
}

/// ATB2 analog test module selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Atb2AnaSel {
    /// ATB2_IN0 output.
    Atb2In0,
    /// ATB2_IN1 output.
    Atb2In1,
    /// ATB2_IN2 output.
    Atb2In2,
    /// ATB2_IN3 output.
    Atb2In3,
}

/// LDO18 voltage setting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ldo18Voltage {
    /// 1.71 v.
    V1_71,
    /// 1.74 v.
    V1_74,
    /// 1.77 v.
    V1_77,
    /// 1.80 v.
    V1_80,
    /// 1.83 v.
    V1_83,
    /// 1.86 v.
    V1_86,
    /// 1.89 v.
    V1_89,
    /// 1.92 v.
    V1_92,
}

/// LDO18 configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Ldo18Config(u32);

impl Ldo18Config {
    const ATB2_ANA_EN: u32 = 0x1 << 27;
    const ATB2_ANA_SEL: u32 = 0x3 << 24;
    const LDO18_PD_FAST: u32 = 0x1 << 5;
    const LDO18_EN: u32 = 0x1 << 4;
    const LDO18_VAL: u32 = 0x7;

    /// Enable atb2 analog test (`ATB2_ANA_EN`).
    #[doc(alias = "ATB2_ANA_EN")]
    #[inline]
    pub const fn enable_atb2_ana(self) -> Self {
        Self(self.0 | Self::ATB2_ANA_EN)
    }
    /// Disable atb2 analog test.
    #[inline]
    pub const fn disable_atb2_ana(self) -> Self {
        Self(self.0 & !Self::ATB2_ANA_EN)
    }
    /// Check if atb2 analog test is enabled.
    #[inline]
    pub const fn is_atb2_ana_enabled(self) -> bool {
        self.0 & Self::ATB2_ANA_EN != 0
    }
    /// Set atb2 analog test module selection (`ATB2_ANA_SEL`).
    #[doc(alias = "ATB2_ANA_SEL")]
    #[inline]
    pub const fn set_atb2_ana_sel(self, sel: Atb2AnaSel) -> Self {
        Self((self.0 & !Self::ATB2_ANA_SEL) | (Self::ATB2_ANA_SEL & ((sel as u32) << 24)))
    }
    /// Get atb2 analog test module selection.
    #[inline]
    pub const fn atb2_ana_sel(self) -> Atb2AnaSel {
        match (self.0 & Self::ATB2_ANA_SEL) >> 24 {
            0 => Atb2AnaSel::Atb2In0,
            1 => Atb2AnaSel::Atb2In1,
            2 => Atb2AnaSel::Atb2In2,
            3 => Atb2AnaSel::Atb2In3,
            _ => unreachable!(),
        }
    }
    /// Enable ldo18 fast power down (`LDO18_PD_FAST`).
    #[doc(alias = "LDO18_PD_FAST")]
    #[inline]
    pub const fn enable_ldo18_pd_fast(self) -> Self {
        Self(self.0 | Self::LDO18_PD_FAST)
    }
    /// Disable ldo18 fast power down (normal power down).
    #[inline]
    pub const fn disable_ldo18_pd_fast(self) -> Self {
        Self(self.0 & !Self::LDO18_PD_FAST)
    }
    /// Check if ldo18 fast power down is enabled.
    #[inline]
    pub const fn is_ldo18_pd_fast_enabled(self) -> bool {
        self.0 & Self::LDO18_PD_FAST != 0
    }
    /// Enable ldo18 (`LDO18_EN`).
    #[doc(alias = "LDO18_EN")]
    #[inline]
    pub const fn enable_ldo18(self) -> Self {
        Self(self.0 | Self::LDO18_EN)
    }
    /// Disable ldo18.
    #[inline]
    pub const fn disable_ldo18(self) -> Self {
        Self(self.0 & !Self::LDO18_EN)
    }
    /// Check if ldo18 is enabled.
    #[inline]
    pub const fn is_ldo18_enabled(self) -> bool {
        self.0 & Self::LDO18_EN != 0
    }
    /// Set ldo18 voltage (`LDO18_VAL`).
    #[doc(alias = "LDO18_VAL")]
    #[inline]
    pub const fn set_ldo18_voltage(self, vol: Ldo18Voltage) -> Self {
        Self((self.0 & !Self::LDO18_VAL) | (Self::LDO18_VAL & (vol as u32)))
    }
    /// Get ldo18 voltage.
    #[inline]
    pub const fn ldo18_voltage(self) -> Ldo18Voltage {
        match self.0 & Self::LDO18_VAL {
            0 => Ldo18Voltage::V1_71,
            1 => Ldo18Voltage::V1_74,
            2 => Ldo18Voltage::V1_77,
            3 => Ldo18Voltage::V1_80,
            4 => Ldo18Voltage::V1_83,
            5 => Ldo18Voltage::V1_86,
            6 => Ldo18Voltage::V1_89,
            7 => Ldo18Voltage::V1_92,
            _ => unreachable!(),
        }
    }
}

/// LDO1x voltage setting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ldo1xVoltage {
    /// 0.90 v.
    V0_90,
    /// 0.95 v.
    V0_95,
    /// 1.00 v.
    V1_00,
    /// 1.05 v.
    V1_05,
    /// 1.10 v.
    V1_10,
    /// 1.15 v.
    V1_15,
    /// 1.20 v.
    V1_20,
    /// 1.25 v.
    V1_25,
    /// 1.30 v.
    V1_30,
    /// 1.35 v.
    V1_35,
    /// 1.40 v.
    V1_40,
    /// 1.50 v.
    V1_50,
    /// 1.60 v.
    V1_60,
    /// 1.70 v.
    V1_70,
    /// 1.80 v.
    V1_80,
    /// 1.90 v.
    V1_90,
}

/// LDO1x configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Ldo1xConfig(u32);

impl Ldo1xConfig {
    const LDO1X_SOFT_EN: u32 = 0x1 << 6;
    const LDO1_PD_FAST: u32 = 0x1 << 5;
    const LDO1X_EN: u32 = 0x1 << 4;
    const LDO1X_VAL: u32 = 0xF;

    /// Enable ldo1x software mode (`LDO1X_SOFT_EN`).
    ///
    /// - 0: hardware mode, LDO on/off determined by package bonding (default LDO on).
    /// - 1: software mode, LDO on/off determined by register BIT4.
    #[doc(alias = "LDO1X_SOFT_EN")]
    #[inline]
    pub const fn enable_ldo1x_soft_mode(self) -> Self {
        Self(self.0 | Self::LDO1X_SOFT_EN)
    }
    /// Disable ldo1x software mode (hardware mode).
    #[inline]
    pub const fn disable_ldo1x_soft_mode(self) -> Self {
        Self(self.0 & !Self::LDO1X_SOFT_EN)
    }
    /// Check if ldo1x software mode is enabled.
    #[inline]
    pub const fn is_ldo1x_soft_mode_enabled(self) -> bool {
        self.0 & Self::LDO1X_SOFT_EN != 0
    }
    /// Enable ldo1 fast power down (`LDO1_PD_FAST`).
    #[doc(alias = "LDO1_PD_FAST")]
    #[inline]
    pub const fn enable_ldo1_pd_fast(self) -> Self {
        Self(self.0 | Self::LDO1_PD_FAST)
    }
    /// Disable ldo1 fast power down (normal power down).
    #[inline]
    pub const fn disable_ldo1_pd_fast(self) -> Self {
        Self(self.0 & !Self::LDO1_PD_FAST)
    }
    /// Check if ldo1 fast power down is enabled.
    #[inline]
    pub const fn is_ldo1_pd_fast_enabled(self) -> bool {
        self.0 & Self::LDO1_PD_FAST != 0
    }
    /// Enable ldo1x (`LDO1X_EN`).
    ///
    /// Note: This bit is only valid when `LDO1X_SOFT_EN` is set to 1.
    #[doc(alias = "LDO1X_EN")]
    #[inline]
    pub const fn enable_ldo1x(self) -> Self {
        Self(self.0 | Self::LDO1X_EN)
    }
    /// Disable ldo1x.
    #[inline]
    pub const fn disable_ldo1x(self) -> Self {
        Self(self.0 & !Self::LDO1X_EN)
    }
    /// Check if ldo1x is enabled.
    ///
    /// Note: This bit is only valid when `LDO1X_SOFT_EN` is set to 1.
    #[inline]
    pub const fn is_ldo1x_enabled(self) -> bool {
        self.0 & Self::LDO1X_EN != 0
    }
    /// Set ldo1x voltage (`LDO1X_VAL`).
    #[doc(alias = "LDO1X_VAL")]
    #[inline]
    pub const fn set_ldo1x_voltage(self, vol: Ldo1xVoltage) -> Self {
        Self((self.0 & !Self::LDO1X_VAL) | (Self::LDO1X_VAL & (vol as u32)))
    }
    /// Get ldo1x voltage.
    #[inline]
    pub const fn ldo1x_voltage(self) -> Ldo1xVoltage {
        match self.0 & Self::LDO1X_VAL {
            0 => Ldo1xVoltage::V0_90,
            1 => Ldo1xVoltage::V0_95,
            2 => Ldo1xVoltage::V1_00,
            3 => Ldo1xVoltage::V1_05,
            4 => Ldo1xVoltage::V1_10,
            5 => Ldo1xVoltage::V1_15,
            6 => Ldo1xVoltage::V1_20,
            7 => Ldo1xVoltage::V1_25,
            8 => Ldo1xVoltage::V1_30,
            9 => Ldo1xVoltage::V1_35,
            10 => Ldo1xVoltage::V1_40,
            11 => Ldo1xVoltage::V1_50,
            12 => Ldo1xVoltage::V1_60,
            13 => Ldo1xVoltage::V1_70,
            14 => Ldo1xVoltage::V1_80,
            15 => Ldo1xVoltage::V1_90,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, irq_ctrl), 0x8);
        assert_eq!(offset_of!(RegisterBlock, irq_status), 0xC);
        assert_eq!(offset_of!(RegisterBlock, ldo25_cfg), 0x20);
        assert_eq!(offset_of!(RegisterBlock, ldo18_cfg), 0x24);
        assert_eq!(offset_of!(RegisterBlock, ldo1x_cfg), 0x28);
        assert_eq!(offset_of!(RegisterBlock, cmp_cfg), 0x3C);
        assert_eq!(offset_of!(RegisterBlock, usb0_rext), 0x48);
        assert_eq!(offset_of!(RegisterBlock, psen_cfg), 0xC0);
        assert_eq!(offset_of!(RegisterBlock, psen_cnt_val), 0xC4);
        assert_eq!(offset_of!(RegisterBlock, sys_sram_par), 0x100);
        assert_eq!(offset_of!(RegisterBlock, cpu_sram_par), 0x104);
        assert_eq!(offset_of!(RegisterBlock, usb_sram_par), 0x108);
        assert_eq!(offset_of!(RegisterBlock, ve_sram_par), 0x10C);
        assert_eq!(offset_of!(RegisterBlock, ge_sram_par), 0x110);
        assert_eq!(offset_of!(RegisterBlock, de_sram_par), 0x114);
        assert_eq!(offset_of!(RegisterBlock, sram_clk_cfg), 0x140);
        assert_eq!(offset_of!(RegisterBlock, sram_map_cfg), 0x160);
        assert_eq!(offset_of!(RegisterBlock, flash_cfg), 0x1F0);
        assert_eq!(offset_of!(RegisterBlock, encoder_cfg), 0x1F4);
        assert_eq!(offset_of!(RegisterBlock, usb0_cfg), 0x40C);
        assert_eq!(offset_of!(RegisterBlock, emac_cfg), 0x410);
        assert_eq!(offset_of!(RegisterBlock, atb_cmu_ana_top), 0xF48);
        assert_eq!(offset_of!(RegisterBlock, atb_dll_top_c), 0xF4C);
        assert_eq!(offset_of!(RegisterBlock, atb_gpadc), 0xF50);
        assert_eq!(offset_of!(RegisterBlock, atb_mipi_dphy), 0xF54);
        assert_eq!(offset_of!(RegisterBlock, atb_rtc_ana_top), 0xF58);
        assert_eq!(offset_of!(RegisterBlock, atb_usb_pll_afe), 0xF5C);
        assert_eq!(offset_of!(RegisterBlock, atb_usb_phy_afe), 0xF60);
        assert_eq!(offset_of!(RegisterBlock, version), 0xFFC);
    }

    #[test]
    fn struct_ldo25_config_functions() {
        let mut val = Ldo25Config(0x0);

        val = val.enable_lvds0_ibias();
        assert!(val.is_lvds0_ibias_enabled());
        assert_eq!(val.0, 0x0004_0000);
        val = val.disable_lvds0_ibias();
        assert!(!val.is_lvds0_ibias_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_xspi_dllc1_ibias();
        assert!(val.is_xspi_dllc1_ibias_enabled());
        assert_eq!(val.0, 0x0002_0000);
        val = val.disable_xspi_dllc1_ibias();
        assert!(!val.is_xspi_dllc1_ibias_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_xspi_dllc0_ibias();
        assert!(val.is_xspi_dllc0_ibias_enabled());
        assert_eq!(val.0, 0x0001_0000);
        val = val.disable_xspi_dllc0_ibias();
        assert!(!val.is_xspi_dllc0_ibias_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_bg_ctrl(0xFF);
        assert_eq!(val.bg_ctrl(), 0xFF);
        assert_eq!(val.0, 0x0000_FF00);

        val = Ldo25Config(0x0).enable_ldo25();
        assert!(val.is_ldo25_enabled());
        assert_eq!(val.0, 0x0000_0010);
        val = val.disable_ldo25();
        assert!(!val.is_ldo25_enabled());
        assert_eq!(val.0, 0x0000_0000);

        for i in 0..8 {
            let (vol, reg_val) = match i {
                0 => (Ldo25Voltage::V2_40, 0x0000_0000),
                1 => (Ldo25Voltage::V2_50, 0x0000_0001),
                2 => (Ldo25Voltage::V2_60, 0x0000_0002),
                3 => (Ldo25Voltage::V2_70, 0x0000_0003),
                4 => (Ldo25Voltage::V2_80, 0x0000_0004),
                5 => (Ldo25Voltage::V2_90, 0x0000_0005),
                6 => (Ldo25Voltage::V3_00, 0x0000_0006),
                7 => (Ldo25Voltage::V3_10, 0x0000_0007),
                _ => unreachable!(),
            };

            val = val.set_ldo25_voltage(vol);
            assert_eq!(val.ldo25_voltage(), vol);
            assert_eq!(val.0, reg_val);
        }
    }

    #[test]
    fn struct_ldo18_config_functions() {
        let mut val = Ldo18Config(0x0);

        val = val.enable_atb2_ana();
        assert!(val.is_atb2_ana_enabled());
        assert_eq!(val.0, 0x0800_0000);
        val = val.disable_atb2_ana();
        assert!(!val.is_atb2_ana_enabled());
        assert_eq!(val.0, 0x0000_0000);

        for i in 0..4 {
            let (sel, reg_val) = match i {
                0 => (Atb2AnaSel::Atb2In0, 0x0000_0000),
                1 => (Atb2AnaSel::Atb2In1, 0x0100_0000),
                2 => (Atb2AnaSel::Atb2In2, 0x0200_0000),
                3 => (Atb2AnaSel::Atb2In3, 0x0300_0000),
                _ => continue,
            };

            val = val.set_atb2_ana_sel(sel);
            assert_eq!(val.atb2_ana_sel(), sel);
            assert_eq!(val.0, reg_val);
        }

        val = Ldo18Config(0x0).enable_ldo18_pd_fast();
        assert!(val.is_ldo18_pd_fast_enabled());
        assert_eq!(val.0, 0x0000_0020);
        val = val.disable_ldo18_pd_fast();
        assert!(!val.is_ldo18_pd_fast_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_ldo18();
        assert!(val.is_ldo18_enabled());
        assert_eq!(val.0, 0x0000_0010);
        val = val.disable_ldo18();
        assert!(!val.is_ldo18_enabled());
        assert_eq!(val.0, 0x0000_0000);

        for i in 0..8 {
            let (vol, reg_val) = match i {
                0 => (Ldo18Voltage::V1_71, 0x0000_0000),
                1 => (Ldo18Voltage::V1_74, 0x0000_0001),
                2 => (Ldo18Voltage::V1_77, 0x0000_0002),
                3 => (Ldo18Voltage::V1_80, 0x0000_0003),
                4 => (Ldo18Voltage::V1_83, 0x0000_0004),
                5 => (Ldo18Voltage::V1_86, 0x0000_0005),
                6 => (Ldo18Voltage::V1_89, 0x0000_0006),
                7 => (Ldo18Voltage::V1_92, 0x0000_0007),
                _ => continue,
            };

            val = val.set_ldo18_voltage(vol);
            assert_eq!(val.ldo18_voltage(), vol);
            assert_eq!(val.0, reg_val);
        }
    }

    #[test]
    fn struct_ldo1x_config_functions() {
        let mut val = Ldo1xConfig(0x0);

        val = val.enable_ldo1x_soft_mode();
        assert!(val.is_ldo1x_soft_mode_enabled());
        assert_eq!(val.0, 0x0000_0040);
        val = val.disable_ldo1x_soft_mode();
        assert!(!val.is_ldo1x_soft_mode_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_ldo1_pd_fast();
        assert!(val.is_ldo1_pd_fast_enabled());
        assert_eq!(val.0, 0x0000_0020);
        val = val.disable_ldo1_pd_fast();
        assert!(!val.is_ldo1_pd_fast_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_ldo1x();
        assert!(val.is_ldo1x_enabled());
        assert_eq!(val.0, 0x0000_0010);
        val = val.disable_ldo1x();
        assert!(!val.is_ldo1x_enabled());
        assert_eq!(val.0, 0x0000_0000);

        for i in 0..16 {
            let (vol, reg_val) = match i {
                0 => (Ldo1xVoltage::V0_90, 0x0000_0000),
                1 => (Ldo1xVoltage::V0_95, 0x0000_0001),
                2 => (Ldo1xVoltage::V1_00, 0x0000_0002),
                3 => (Ldo1xVoltage::V1_05, 0x0000_0003),
                4 => (Ldo1xVoltage::V1_10, 0x0000_0004),
                5 => (Ldo1xVoltage::V1_15, 0x0000_0005),
                6 => (Ldo1xVoltage::V1_20, 0x0000_0006),
                7 => (Ldo1xVoltage::V1_25, 0x0000_0007),
                8 => (Ldo1xVoltage::V1_30, 0x0000_0008),
                9 => (Ldo1xVoltage::V1_35, 0x0000_0009),
                10 => (Ldo1xVoltage::V1_40, 0x0000_000A),
                11 => (Ldo1xVoltage::V1_50, 0x0000_000B),
                12 => (Ldo1xVoltage::V1_60, 0x0000_000C),
                13 => (Ldo1xVoltage::V1_70, 0x0000_000D),
                14 => (Ldo1xVoltage::V1_80, 0x0000_000E),
                15 => (Ldo1xVoltage::V1_90, 0x0000_000F),
                _ => unreachable!(),
            };

            val = val.set_ldo1x_voltage(vol);
            assert_eq!(val.ldo1x_voltage(), vol);
            assert_eq!(val.0, reg_val);
        }
    }
}
