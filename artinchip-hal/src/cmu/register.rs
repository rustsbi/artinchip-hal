//! CMU register blocks and registers.

use volatile_register::{RO, RW};

/// Clock Manage Unit register block.
#[repr(C)]
pub struct RegisterBlock {
    /// PLL_INT0 general register (`PLL_INT0_GEN`).
    #[doc(alias = "PLL_INT0_GEN")]
    pub pll_int0_general: RW<PllGeneral>,
    /// PLL_INT1 general register (`PLL_INT1_GEN`).
    #[doc(alias = "PLL_INT1_GEN")]
    pub pll_int1_general: RW<PllGeneral>,
    _reserved0: [u8; 0x18],
    /// PLL_FRA0 general register (`PLL_FRA0_GEN`).
    #[doc(alias = "PLL_FRA0_GEN")]
    pub pll_fra0_general: RW<PllGeneral>,
    /// PLL_FRA1 general register (`PLL_FRA1_GEN`).
    #[doc(alias = "PLL_FRA1_GEN")]
    pub pll_fra1_general: RW<PllGeneral>,
    /// PLL_FRA2 general register (`PLL_FRA2_GEN`).
    #[doc(alias = "PLL_FRA2_GEN")]
    pub pll_fra2_general: RW<PllGeneral>,
    _reserved1: [u8; 0x14],
    /// PLL_INT0 configuration register (`PLL_INT0_CFG`).
    #[doc(alias = "PLL_INT0_CFG")]
    pub pll_int0_config: RW<PllIntConfig>,
    /// PLL_INT1 configuration register (`PLL_INT1_CFG`).
    #[doc(alias = "PLL_INT1_CFG")]
    pub pll_int1_config: RW<PllIntConfig>,
    _reserved2: [u8; 0x18],
    /// PLL_FRA0 configuration register (`PLL_FRA0_CFG`).
    #[doc(alias = "PLL_FRA0_CFG")]
    pub pll_fra0_config: RW<PllFraConfig>,
    /// PLL_FRA1 configuration register (`PLL_FRA1_CFG`).
    #[doc(alias = "PLL_FRA1_CFG")]
    pub pll_fra1_config: RW<PllFraConfig>,
    /// PLL_FRA2 configuration register (`PLL_FRA2_CFG`).
    #[doc(alias = "PLL_FRA2_CFG")]
    pub pll_fra2_config: RW<PllFraConfig>,
    _reserved3: [u8; 0x14],
    /// PLL_FRA0 spread spectrum register (`PLL_FRA0_SDM`).
    #[doc(alias = "PLL_FRA0_SDM")]
    pub pll_fra0_spread_spectrum: RW<PllFraSdm>,
    /// PLL_FRA1 spread spectrum register (`PLL_FRA1_SDM`).
    #[doc(alias = "PLL_FRA1_SDM")]
    pub pll_fra1_spread_spectrum: RW<PllFraSdm>,
    /// PLL_FRA2 spread spectrum register (`PLL_FRA2_SDM`).
    #[doc(alias = "PLL_FRA2_SDM")]
    pub pll_fra2_spread_spectrum: RW<PllFraSdm>,
    _reserved4: [u8; 0x14],
    /// PLL common register (`PLL_COMMON`).
    #[doc(alias = "PLL_COMMON")]
    pub pll_common: RW<PllCommon>,
    /// PLL input register (`PLL_IN`).
    #[doc(alias = "PLL_IN")]
    pub pll_input: RW<PllInput>,
    _reserved5: [u8; 0x38],
    /// Clock 0 output register (`CLK_OUT0`).
    #[doc(alias = "CLK_OUT0")]
    pub clock_out0: RW<ClockOutput>,
    /// Clock 1 output register (`CLK_OUT1`).
    #[doc(alias = "CLK_OUT1")]
    pub clock_out1: RW<ClockOutput>,
    /// Clock 2 output register (`CLK_OUT2`).
    #[doc(alias = "CLK_OUT2")]
    pub clock_out2: RW<ClockOutput>,
    /// Clock 3 output register (`CLK_OUT3`).
    #[doc(alias = "CLK_OUT3")]
    pub clock_out3: RW<ClockOutput>,
    _reserved6: [u8; 0x10],
    /// AXI and AHB clock register (`CLK_AXI_AHB`).
    #[doc(alias = "CLK_AXI_AHB")]
    pub clock_axi_ahb: RW<ClkOnPllIntx>,
    _reserved7: [u8; 0xC],
    /// AHB clock register (`CLK_AHB`).
    #[doc(alias = "CLK_AHB")]
    pub clock_ahb: RW<ClkOnPllIntx>,
    _reserved8: [u8; 0xC],
    /// APB0 clock register (`CLK_APB0`).
    #[doc(alias = "CLK_APB0")]
    pub clock_apb0: RW<ClkOnPllIntx>,
    _reserved9: [u8; 0xDC],
    /// CPU clock register (`CLK_CPU`).
    #[doc(alias = "CLK_CPU")]
    pub clock_cpu: RW<CpuClock>,
    /// DM clock register (`CLK_DM`).
    #[doc(alias = "CLK_DM")]
    pub clock_dm: RW<ClkOnPllIntx>,
    _reserved10: [u8; 0x4],
    /// WDOG clock register (`CLK_WDOG`).
    #[doc(alias = "CLK_WDOG")]
    pub clock_wdog: RW<WdogClock>,
    /// DDR clock register (`CLK_DDR`).
    #[doc(alias = "CLK_DDR")]
    pub clock_ddr: RW<DdrClock>,
    _reserved11: [u8; 0xC],
    /// DISP clock register (`CLK_DISP`).
    #[doc(alias = "CLK_DISP")]
    pub clock_disp: RW<DispClock>,
    _reserved12: [u8; 0xC],
    /// Audio serial clock register (`CLK_AUD_SCLK`).
    #[doc(alias = "CLK_AUD_SCLK")]
    pub clock_audio_serial: RW<AudioSerialClock>,
    _reserved13: [u8; 0xC],
    /// PWMCS and SDFM clock register (`CLK_PWMCS_SDFM`).
    #[doc(alias = "CLK_PWMCS_SDFM")]
    pub clock_pwmcs_sdfm: RW<SimpleModule0Clock>,
    _reserved14: [u8; 0x1CC],
    /// DMA clock register (`CLK_DMA`).
    #[doc(alias = "CLK_DMA")]
    pub clock_dma: RW<SimpleModule2Clock>,
    _reserved15: [u8; 0x4],
    /// CE clock register (`CLK_CE`).
    #[doc(alias = "CLK_CE")]
    pub clock_ce: RW<NormalModuleClock>,
    /// USB DEV clock register (`CLK_USB_DEV`).
    #[doc(alias = "CLK_USB_DEV")]
    pub clock_usb_dev: RW<SimpleModule2Clock>,
    /// USB HOST0 clock register (`CLK_USB_HOST0`|`CLK_USB_HOST`).
    #[doc(alias = "CLK_USB_HOST0")]
    #[doc(alias = "CLK_USB_HOST")]
    pub clock_usb_host0: RW<SimpleModule2Clock>,
    /// USB HOST1 clock register (`CLK_USB_HOST1`).
    #[doc(alias = "CLK_USB_HOST1")]
    pub clock_usb_host1: RW<SimpleModule2Clock>,
    _reserved16: [u8; 0x8],
    /// USB PHY0 clock register (`CLK_USB_PHY0`|`CLK_USB_PHY`).
    #[doc(alias = "CLK_USB_PHY0")]
    #[doc(alias = "CLK_USB_PHY")]
    pub clock_usb_phy0: RW<SimpleModule4Clock>,
    /// USB PHY1 clock register (`CLK_USB_PHY1`).
    #[doc(alias = "CLK_USB_PHY1")]
    pub clock_usb_phy1: RW<SimpleModule4Clock>,
    _reserved17: [u8; 0x8],
    /// EMAC or GMAC0 clock register (`CLK_EMAC`|`CLK_GMAC0`).
    #[doc(alias = "CLK_EMAC")]
    #[doc(alias = "CLK_GMAC0")]
    pub clock_emac_gmac0: RW<NormalModuleClock>,
    /// GMAC1 clock register (`CLK_GMAC1`).
    #[doc(alias = "CLK_GMAC1")]
    pub clock_gmac1: RW<NormalModuleClock>,
    _reserved18: [u8; 0x14],
    /// XSPI clock register (`CLK_XSPI`).
    #[doc(alias = "CLK_XSPI")]
    pub clock_xspi: RW<NormalModuleClock>,
    /// QSPI0 clock register (`CLK_QSPI0`).
    #[doc(alias = "CLK_QSPI0")]
    pub clock_qspi0: RW<NormalModuleClock>,
    /// QSPI1 clock register (`CLK_QSPI1`).
    #[doc(alias = "CLK_QSPI1")]
    pub clock_qspi1: RW<NormalModuleClock>,
    /// QSPI2 clock register (`CLK_QSPI2`).
    #[doc(alias = "CLK_QSPI2")]
    pub clock_qspi2: RW<NormalModuleClock>,
    /// QSPI3 clock register (`CLK_QSPI3`).
    #[doc(alias = "CLK_QSPI3")]
    pub clock_qspi3: RW<NormalModuleClock>,
    /// SDMC0 clock register (`CLK_SDMC0`).
    #[doc(alias = "CLK_SDMC0")]
    pub clock_sdmc0: RW<NormalModuleClock>,
    /// SDMC1 clock register (`CLK_SDMC1`).
    #[doc(alias = "CLK_SDMC1")]
    pub clock_sdmc1: RW<NormalModuleClock>,
    /// SDMC2 clock register (`CLK_SDMC2`).
    #[doc(alias = "CLK_SDMC2")]
    pub clock_sdmc2: RW<NormalModuleClock>,
    _reserved19: [u8; 0x14],
    /// CORDIC clock register (`CLK_CORDIC`).
    #[doc(alias = "CLK_CORDIC")]
    pub clock_cordic: RW<SimpleModule2Clock>,
    /// HCL clock register (`CLK_HCL`).
    #[doc(alias = "CLK_HCL")]
    pub clock_hcl: RW<SimpleModule2Clock>,
    _reserved20: [u8; 0x8],
    /// PBUS clock register (`CLK_PBUS`).
    #[doc(alias = "CLK_PBUS")]
    pub clock_pbus: RW<SimpleModule2Clock>,
    _reserved21: [u8; 0x35C],
    /// SYSCFG clock register (`CLK_SYSCFG`).
    #[doc(alias = "CLK_SYSCFG")]
    pub clock_syscfg: RW<SimpleModule1Clock>,
    _reserved22: [u8; 0xC],
    /// SPI_ENC clock register (`CLK_SPI_ENC`).
    #[doc(alias = "CLK_SPI_ENC")]
    pub clock_spi_enc: RW<SimpleModule5Clock>,
    /// PWMCS clock register (`CLK_PWMCS`).
    #[doc(alias = "CLK_PWMCS")]
    pub clock_pwmcs: RW<NormalModuleClock>,
    /// PSADC clock register (`CLK_PSADC`).
    #[doc(alias = "CLK_PSADC")]
    pub clock_psadc: RW<NormalModuleClock>,
    /// MTOP clock register (`CLK_MTOP`).
    #[doc(alias = "CLK_MTOP")]
    pub clock_mtop: RW<SimpleModule2Clock>,
    /// I2S0 clock register (`CLK_I2S0`|`CLK_I2S`).
    #[doc(alias = "CLK_I2S0")]
    #[doc(alias = "CLK_I2S")]
    pub clock_i2s0: RW<SimpleModule5Clock>,
    /// I2S1 clock register (`CLK_I2S1`).
    #[doc(alias = "CLK_I2S1")]
    pub clock_i2s1: RW<SimpleModule5Clock>,
    _reserved23: [u8; 0x14],
    /// GPIO clock register (`CLK_GPIO`).
    #[doc(alias = "CLK_GPIO")]
    pub clock_gpio: RW<SimpleModule2Clock>,
    /// UART0 clock register (`CLK_UART0`).
    #[doc(alias = "CLK_UART0")]
    pub clock_uart0: RW<NormalModuleClock>,
    /// UART1 clock register (`CLK_UART1`).
    #[doc(alias = "CLK_UART1")]
    pub clock_uart1: RW<NormalModuleClock>,
    /// UART2 clock register (`CLK_UART2`).
    #[doc(alias = "CLK_UART2")]
    pub clock_uart2: RW<NormalModuleClock>,
    /// UART3 clock register (`CLK_UART3`).
    #[doc(alias = "CLK_UART3")]
    pub clock_uart3: RW<NormalModuleClock>,
    /// UART4 clock register (`CLK_UART4`).
    #[doc(alias = "CLK_UART4")]
    pub clock_uart4: RW<NormalModuleClock>,
    /// UART5 clock register (`CLK_UART5`).
    #[doc(alias = "CLK_UART5")]
    pub clock_uart5: RW<NormalModuleClock>,
    /// UART6 clock register (`CLK_UART6`).
    #[doc(alias = "CLK_UART6")]
    pub clock_uart6: RW<NormalModuleClock>,
    /// UART7 clock register (`CLK_UART7`).
    #[doc(alias = "CLK_UART7")]
    pub clock_uart7: RW<NormalModuleClock>,
    _reserved24: [u8; 0x10],
    /// TA interface clock register (`CLK_TA_IF`).
    #[doc(alias = "CLK_TA_IF")]
    pub clock_ta_if: RW<NormalModuleClock>,
    /// EDAT interface clock register (`CLK_EDAT_IF`).
    #[doc(alias = "CLK_EDAT_IF")]
    pub clock_edat_if: RW<NormalModuleClock>,
    /// BIS interface clock register (`CLK_BIS_IF`).
    #[doc(alias = "CLK_BIS_IF")]
    pub clock_bis_if: RW<NormalModuleClock>,
    /// SDFM clock register (`CLK_SDFM`).
    #[doc(alias = "CLK_SDFM")]
    pub clock_sdfm: RW<SimpleModule5Clock>,
    /// LCD clock register (`CLK_LCD`).
    #[doc(alias = "CLK_LCD")]
    pub clock_lcd: RW<SimpleModule5Clock>,
    /// LVDS clock register (`CLK_LVDS`).
    #[doc(alias = "CLK_LVDS")]
    pub clock_lvds: RW<SimpleModule5Clock>,
    /// MIPI_DSI clock register (`CLK_MIPI_DSI`).
    #[doc(alias = "CLK_MIPI_DSI")]
    pub clock_mipi_dsi: RW<SimpleModule5Clock>,
    _reserved25: [u8; 0x4],
    /// DVP clock register (`CLK_DVP`).
    #[doc(alias = "CLK_DVP")]
    pub clock_dvp: RW<NormalModuleClock>,
    _reserved26: [u8; 0x4],
    /// MIPI_CSI clock register (`CLK_MIPI_CSI`).
    #[doc(alias = "CLK_MIPI_CSI")]
    pub clock_mipi_csi: RW<SimpleModule6Clock>,
    _reserved27: [u8; 0x24],
    /// DE clock register (`CLK_DE`).
    #[doc(alias = "CLK_DE")]
    pub clock_de: RW<NormalModuleClock>,
    /// GE clock register (`CLK_GE`).
    #[doc(alias = "CLK_GE")]
    pub clock_ge: RW<NormalModuleClock>,
    /// VE clock register (`CLK_VE`).
    #[doc(alias = "CLK_VE")]
    pub clock_ve: RW<NormalModuleClock>,
    _reserved28: [u8; 0x38],
    /// SID clock register (`CLK_SID`).
    #[doc(alias = "CLK_SID")]
    pub clock_sid: RW<SimpleModule3Clock>,
    /// RTC clock register (`CLK_RTC`).
    #[doc(alias = "CLK_RTC")]
    pub clock_rtc: RW<SimpleModule1Clock>,
    /// GTC clock register (`CLK_GTC`).
    #[doc(alias = "CLK_GTC")]
    pub clock_gtc: RW<SimpleModule2Clock>,
    _reserved29: [u8; 0x50],
    /// I2C0 clock register (`CLK_I2C0`).
    #[doc(alias = "CLK_I2C0")]
    pub clock_i2c0: RW<SimpleModule2Clock>,
    /// I2C1 clock register (`CLK_I2C1`).
    #[doc(alias = "CLK_I2C1")]
    pub clock_i2c1: RW<SimpleModule2Clock>,
    /// I2C2 clock register (`CLK_I2C2`).
    #[doc(alias = "CLK_I2C2")]
    pub clock_i2c2: RW<SimpleModule2Clock>,
    /// I2C3 clock register (`CLK_I2C3`).
    #[doc(alias = "CLK_I2C3")]
    pub clock_i2c3: RW<SimpleModule2Clock>,
    _reserved30: [u8; 0x10],
    /// CAN0 clock register (`CLK_CAN0`).
    #[doc(alias = "CLK_CAN0")]
    pub clock_can0: RW<SimpleModule2Clock>,
    /// CAN1 clock register (`CLK_CAN1`).
    #[doc(alias = "CLK_CAN1")]
    pub clock_can1: RW<SimpleModule2Clock>,
    _reserved31: [u8; 0x8],
    /// PWM/XPWM0 clock register (`CLK_PWM`).
    #[doc(alias = "CLK_PWM")]
    pub clock_pwm: RW<NormalModuleClock>,
    _reserved32: [u8; 0xC],
    /// ADCIM clock register (`CLK_ADCIM`).
    #[doc(alias = "CLK_ADCIM")]
    pub clock_adcim: RW<NormalModuleClock>,
    /// GPAI clock register (`CLK_GPAI`).
    #[doc(alias = "CLK_GPAI")]
    pub clock_gpai: RW<SimpleModule2Clock>,
    /// RTP clock register (`CLK_RTP`).
    #[doc(alias = "CLK_RTP")]
    pub clock_rtp: RW<SimpleModule2Clock>,
    /// THS clock register (`CLK_THS`).
    #[doc(alias = "CLK_THS")]
    pub clock_ths: RW<SimpleModule2Clock>,
    /// CIR clock register (`CLK_CIR`).
    #[doc(alias = "CLK_CIR")]
    pub clock_cir: RW<SimpleModule2Clock>,
    _reserved33: [u8; 0x648],
    /// CMU version register (`VERSION`).
    #[doc(alias = "VERSION")]
    pub version: RO<u32>,
}

/// Pll output selection
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PllOutput {
    /// OSC_24M clock output.
    Osc24M,
    /// Pll clock output.
    PllClk,
}

/// Pll general register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PllGeneral(u32);

impl PllGeneral {
    const PLL_REG_GATE: u32 = 0x1 << 31;
    const PLL_ICP: u32 = 0x1F << 24;
    const PLL_OUT_MUX: u32 = 0x1 << 20;
    const FACTOR_M_EN: u32 = 0x1 << 19;
    const PLL_OUT_SYS: u32 = 0x1 << 18;
    const PLL_LOCK: u32 = 0x1 << 17;
    const PLL_EN: u32 = 0x1 << 16;
    const FACTOR_N: u32 = 0xFF << 8;
    const FACTOR_M: u32 = 0x3 << 4;
    const FACTOR_P: u32 = 0x1;

    /// Enable pll register (`PLL_REG_GATE`).
    ///
    /// To ensure proper PLL output, this register must not be configured as 0.
    #[doc(alias = "PLL_REG_GATE")]
    #[inline]
    pub const fn enable_pll_reg(self) -> Self {
        Self(self.0 | Self::PLL_REG_GATE)
    }
    /// Disable pll register.
    #[inline]
    pub const fn disable_pll_reg(self) -> Self {
        Self(self.0 & !Self::PLL_REG_GATE)
    }
    /// Check if pll register is enabled.
    #[inline]
    pub const fn is_pll_reg_enabled(self) -> bool {
        (self.0 & Self::PLL_REG_GATE) != 0
    }
    /// Set adjustment of pll loop bandwidth (`PLL_ICP`).
    ///
    /// The smaller the value, the lower the bandwidth.
    #[doc(alias = "PLL_ICP")]
    #[inline]
    pub const fn set_pll_icp(self, val: u8) -> Self {
        assert!(
            val < 32,
            "Value out of bounds for PLL_ICP (expected 0..=31)"
        );
        Self((self.0 & !Self::PLL_ICP) | (Self::PLL_ICP & ((val as u32) << 24)))
    }
    /// Get adjustment of pll loop bandwidth.
    #[inline]
    pub const fn pll_icp(self) -> u8 {
        ((self.0 & Self::PLL_ICP) >> 24) as u8
    }
    /// Set pll output selection (`PLL_OUT_MUX`).
    ///
    /// This selection is unaffected by other registers.
    /// When the PLL is damaged or inactive, the clock can be bypassed to OSC_24M for operation.
    /// OSC_24M clock may also be used during startup and sleep scenarios.
    #[doc(alias = "PLL_OUT_MUX")]
    #[inline]
    pub const fn set_pll_output_sel(self, selection: PllOutput) -> Self {
        Self((self.0 & !Self::PLL_OUT_MUX) | (Self::PLL_OUT_MUX & ((selection as u32) << 20)))
    }
    /// Get pll output selection.
    #[inline]
    pub const fn pll_output_sel(self) -> PllOutput {
        match (self.0 & Self::PLL_OUT_MUX) >> 20 {
            0 => PllOutput::Osc24M,
            1 => PllOutput::PllClk,
            _ => unreachable!(),
        }
    }
    /// Enable pll output divider coefficient (`FACTOR_M_EN`).
    #[doc(alias = "FACTOR_M_EN")]
    #[inline]
    pub const fn enable_facter_m(self) -> Self {
        Self(self.0 | Self::FACTOR_M_EN)
    }
    /// Disable pll output divider coefficient.
    #[inline]
    pub const fn disable_facter_m(self) -> Self {
        Self(self.0 & !Self::FACTOR_M_EN)
    }
    /// Check if pll output divider coefficient is enabled.
    #[inline]
    pub const fn is_factor_m_enabled(self) -> bool {
        (self.0 & Self::FACTOR_M_EN) != 0
    }
    /// Enable pll output to system (`PLL_OUT_SYS`).
    #[inline]
    #[doc(alias = "PLL_OUT_SYS")]
    pub const fn enabled_pll_out2sys(self) -> Self {
        Self(self.0 | Self::PLL_OUT_SYS)
    }
    /// Disable pll output to system.
    #[inline]
    pub const fn disable_pll_out2sys(self) -> Self {
        Self(self.0 & !Self::PLL_OUT_SYS)
    }
    /// Check if pll output to system is enabled.
    #[inline]
    pub const fn is_pll_out2sys_enabled(self) -> bool {
        (self.0 & Self::PLL_OUT_SYS) != 0
    }
    /// Check if pll is locked (`PLL_LOCK`).
    ///
    /// After enabling the PLL, poll this register until stable before
    /// proceeding with other operations to ensure proper PLL operation.
    #[doc(alias = "PLL_LOCK")]
    #[inline]
    pub const fn is_pll_locked(self) -> bool {
        (self.0 & Self::PLL_LOCK) != 0
    }
    /// Enable pll circuit (`PLL_EN`).
    ///
    /// PLL output frequency calculation:
    /// ```text
    /// PLL_O = [24 / (P+1)] * [(N+1) / (M+1)] MHz.
    /// ```
    /// P=`FACTOR_P`, N=`FACTOR_N`, M=`FACTOR_M`.
    #[doc(alias = "PLL_EN")]
    #[inline]
    pub const fn enable_pll(self) -> Self {
        Self(self.0 | Self::PLL_EN)
    }
    /// Disable pll circuit.
    #[inline]
    pub const fn disable_pll(self) -> Self {
        Self(self.0 & !Self::PLL_EN)
    }
    /// Check if pll circuit is enabled.
    #[inline]
    pub const fn is_pll_enabled(self) -> bool {
        (self.0 & Self::PLL_EN) != 0
    }
    /// Set pll multiplier coefficient (`FACTOR_N`).
    ///
    /// Value range: 14..=199.
    #[doc(alias = "FACTOR_N")]
    #[inline]
    pub const fn set_factor_n(self, val: u8) -> Self {
        assert!(
            val > 13 && val < 200,
            "Value out of bounds for FACTOR_N (expected 14..=199)"
        );
        Self((self.0 & !Self::FACTOR_N) | (Self::FACTOR_N & ((val as u32) << 8)))
    }
    /// Get pll multiplier coefficient.
    #[inline]
    pub const fn factor_n(self) -> u8 {
        ((self.0 & Self::FACTOR_N) >> 8) as u8
    }
    /// Set pll output divider coefficient (`FACTOR_M`).
    ///
    /// Must set `FACTOR_M_EN` = 1 when `FACTOR_M` ≠ 0,
    /// otherwise PLL will have no clock output.
    #[doc(alias = "FACTOR_M")]
    #[inline]
    pub const fn set_factor_m(self, val: u8) -> Self {
        assert!(val < 4, "Value out of bounds for FACTOR_M (expected 0..=3)");
        Self((self.0 & !Self::FACTOR_M) | (Self::FACTOR_M & ((val as u32) << 4)))
    }
    /// Get pll output divider coefficient.
    #[inline]
    pub const fn factor_m(self) -> u8 {
        ((self.0 & Self::FACTOR_M) >> 4) as u8
    }
    /// Set pll input divider coefficient (`FACTOR_P`).
    #[doc(alias = "FACTOR_P")]
    #[inline]
    pub const fn set_factor_p(self, val: u8) -> Self {
        assert!(val < 2, "Value out of bounds for FACTOR_P (expected 0..=1)");
        Self((self.0 & !Self::FACTOR_P) | (Self::FACTOR_P & (val as u32)))
    }
    /// Get pll input divider coefficient.
    #[inline]
    pub const fn factor_p(self) -> u8 {
        (self.0 & Self::FACTOR_P) as u8
    }
}

/// Ldo bias current.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LdoBiasCurrent {
    /// 4uA.
    FouruA,
    /// 2uA.
    TwouA,
}

/// Bias current multiplier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BiasCurrentMultiplier {
    /// Bias current x 4.
    FoutTimes,
    /// Bias current x 1.
    OneTimes,
}

/// PLL_INT0/PLL_INT1 configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PllIntConfig(u32);

impl PllIntConfig {
    const PLL_LOCK_TIME: u32 = 0x7 << 28;
    const PLL_IVCO: u32 = 0x7 << 24;
    const PLL_VCO_SEL: u32 = 0x3 << 20;
    const PLL_VCO_RST: u32 = 0x1 << 19;
    const PLL_VCO_GAIN: u32 = 0x7 << 16;
    const PLL_BINT: u32 = 0x7F << 8;
    const PLL_CINT: u32 = 0x7F;

    const LDO_BIAS_CURRENT_MASK: u32 = 0x1 << 9;
    const BIAS_CURRENT_MULTI_MASK: u32 = 0x1 << 8;

    /// Set pll lock time (`PLL_LOCK_TIME`).
    #[doc(alias = "PLL_LOCK_TIME")]
    #[inline]
    pub const fn set_pll_lock_time(self, val: u8) -> Self {
        assert!(
            val < 8,
            "Value out of bounds for PLL_LOCK_TIME (expected 0..=7)"
        );
        Self((self.0 & !Self::PLL_LOCK_TIME) | (Self::PLL_LOCK_TIME & ((val as u32) << 28)))
    }
    /// Get pll lock time.
    #[inline]
    pub const fn pll_lock_time(self) -> u8 {
        ((self.0 & Self::PLL_LOCK_TIME) >> 28) as u8
    }
    /// Set PLL_IVCO bit (`PLL_IVCO`).
    #[doc(alias = "PLL_IVCO")]
    #[inline]
    pub const fn set_pll_ivco(self, val: u8) -> Self {
        assert!(val < 8, "Value out of bounds for PLL_IVCO (expected 0..=7)");
        Self((self.0 & !Self::PLL_IVCO) | (Self::PLL_IVCO & ((val as u32) << 24)))
    }
    /// Get PLL_IVCO bit.
    #[inline]
    pub const fn pll_ivco(self) -> u8 {
        ((self.0 & Self::PLL_IVCO) >> 24) as u8
    }
    /// Set PLL_VCO_SEL bit (`PLL_VCO_SEL`).
    #[doc(alias = "PLL_VCO_SEL")]
    #[inline]
    pub const fn set_pll_vco_sel(self, val: u8) -> Self {
        assert!(
            val < 4,
            "Value out of bounds for PLL_VCO_SEL (expected 0..=3)"
        );
        Self((self.0 & !Self::PLL_VCO_SEL) | (Self::PLL_VCO_SEL & ((val as u32) << 20)))
    }
    /// Get PLL_VCO_RST bit.
    #[inline]
    pub const fn pll_vco_sel(self) -> u8 {
        ((self.0 & Self::PLL_VCO_SEL) >> 20) as u8
    }
    /// Set PLL_VCO_RST bit (`PLL_VCO_RST`).
    #[doc(alias = "PLL_VCO_RST")]
    #[inline]
    pub const fn set_pll_vco_rst(self, val: u8) -> Self {
        assert!(
            val < 2,
            "Value out of bounds for PLL_VCO_RST (expected 0..=1)"
        );
        Self((self.0 & !Self::PLL_VCO_RST) | (Self::PLL_VCO_RST & ((val as u32) << 19)))
    }
    /// Get PLL_VCO_RST bit.
    #[inline]
    pub const fn pll_vco_rst(self) -> u8 {
        ((self.0 & Self::PLL_VCO_RST) >> 19) as u8
    }
    /// Set PLL_VCO_GAIN bit (`PLL_VCO_GAIN`).
    #[doc(alias = "PLL_VCO_GAIN")]
    #[inline]
    pub const fn set_pll_vco_gain(self, val: u8) -> Self {
        assert!(
            val < 8,
            "Value out of bounds for PLL_VCO_GAIN (expected 0..=7)"
        );
        Self((self.0 & !Self::PLL_VCO_GAIN) | (Self::PLL_VCO_GAIN & ((val as u32) << 16)))
    }
    /// Get PLL_VCO_GAIN bit.
    #[inline]
    pub const fn pll_vco_gain(self) -> u8 {
        ((self.0 & Self::PLL_VCO_GAIN) >> 16) as u8
    }
    /// Set PLL_BINT bit (`PLL_BINT`).
    #[doc(alias = "PLL_BINT")]
    #[inline]
    pub const fn set_pll_bint(self, val: u8) -> Self {
        assert!(
            val < 128,
            "Value out of bounds for PLL_BINT (expected 0..=127)"
        );
        Self((self.0 & !Self::PLL_BINT) | (Self::PLL_BINT & ((val as u32) << 8)))
    }
    /// Get PLL_BINT bit.
    #[inline]
    pub const fn pll_bint(self) -> u8 {
        ((self.0 & Self::PLL_BINT) >> 8) as u8
    }
    /// Set ldo bias current.
    #[inline]
    pub const fn set_ldo_bias_current(self, current: LdoBiasCurrent) -> Self {
        Self(
            (self.0 & !Self::LDO_BIAS_CURRENT_MASK)
                | (Self::LDO_BIAS_CURRENT_MASK & ((current as u32) << 9)),
        )
    }
    /// Get ldo bias current.
    #[inline]
    pub const fn ldo_bias_current(self) -> LdoBiasCurrent {
        match (self.0 & Self::LDO_BIAS_CURRENT_MASK) >> 9 {
            0 => LdoBiasCurrent::FouruA,
            1 => LdoBiasCurrent::TwouA,
            _ => unreachable!(),
        }
    }
    /// Set bias current multiplier.
    #[inline]
    pub const fn set_bias_current_multiplier(self, mutiplier: BiasCurrentMultiplier) -> Self {
        Self(
            (self.0 & !Self::BIAS_CURRENT_MULTI_MASK)
                | (Self::BIAS_CURRENT_MULTI_MASK & ((mutiplier as u32) << 8)),
        )
    }
    /// Get bias current multiplier.
    #[inline]
    pub const fn bias_current_multiplier(self) -> BiasCurrentMultiplier {
        match (self.0 & Self::BIAS_CURRENT_MULTI_MASK) >> 8 {
            0 => BiasCurrentMultiplier::FoutTimes,
            1 => BiasCurrentMultiplier::OneTimes,
            _ => unreachable!(),
        }
    }
    /// Set pll initial oscillation frequency (`PLL_CINT`).
    #[doc(alias = "PLL_CINT")]
    #[inline]
    pub const fn set_pll_cint(self, val: u8) -> Self {
        assert!(
            val < 128,
            "Value out of bounds for PLL_CINT (expected 0..=127)"
        );
        Self((self.0 & !Self::PLL_CINT) | (Self::PLL_CINT & (val as u32)))
    }
    /// Get pll initial oscillation frequency.
    #[inline]
    pub const fn pll_cint(self) -> u8 {
        (self.0 & Self::PLL_CINT) as u8
    }
}

/// PLL_FRA0/PLL_FRA1/PLL_FRA2 configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PllFraConfig(u32);

impl PllFraConfig {
    const DITHER_EN: u32 = 0x1 << 24;
    const FRA_EN: u32 = 0x1 << 20;
    const FRA_IN: u32 = 0x1FFFF;

    /// Enable dither (`DITHER_EN`).
    ///
    /// This register field is for internal debugging.
    #[doc(alias = "DITHER_EN")]
    #[inline]
    pub const fn enable_dither(self) -> Self {
        Self(self.0 | Self::DITHER_EN)
    }
    /// Disable dither.
    #[inline]
    pub const fn disable_dither(self) -> Self {
        Self(self.0 & !Self::DITHER_EN)
    }
    /// Check if dither is enabled.
    #[inline]
    pub const fn is_dither_enabled(self) -> bool {
        (self.0 & Self::DITHER_EN) != 0
    }
    /// Enable fractional divider (`FRA_EN`).
    #[doc(alias = "FRA_EN")]
    #[inline]
    pub const fn enable_fra_div(self) -> Self {
        Self(self.0 | Self::FRA_EN)
    }
    /// Disable fractional divider.
    #[inline]
    pub const fn disable_fra_div(self) -> Self {
        Self(self.0 & !Self::FRA_EN)
    }
    /// Check if fractional divider is enabled.
    #[inline]
    pub const fn is_fra_div_enabled(self) -> bool {
        (self.0 & Self::FRA_EN) != 0
    }
    /// Set fractional divider coefficient (`FRA_IN`).
    #[doc(alias = "FRA_IN")]
    #[inline]
    pub const fn set_fra_div(self, val: u32) -> Self {
        assert!(
            val < 0x20000,
            "Value out of bounds for FRA_IN (expected 0..=0x1FFFF)"
        );
        Self((self.0 & !Self::FRA_IN) | (Self::FRA_IN & val))
    }
    /// Get fractional divider coefficient.
    #[inline]
    pub const fn fra_div(self) -> u32 {
        self.0 & Self::FRA_IN
    }
}

/// Spread spectrum mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SdmMode {
    /// Dc = 0.
    Dc0,
    /// Dc = 1.
    Dc1,
    /// Triangular wave.
    Triangular,
}

/// Spread spectrum freqency.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SdmFreq {
    /// 31.5 KHz.
    F31P5KHz,
    /// 32.0 KHz.
    F32P0KHz,
    /// 32.5 KHz.
    F32P5KHz,
    /// 33.0 KHz.
    F33P0KHz,
}

/// PLL_FRA0/PLL_FRA1/PLL_FRA2 spread spectrum register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PllFraSdm(u32);

impl PllFraSdm {
    const SDM_EN: u32 = 0x1 << 31;
    const SDM_MODE: u32 = 0x3 << 29;
    const SDM_STEP: u32 = 0x1FF << 20;
    const SDM_FREQ: u32 = 0x3 << 17;
    const SDM_BOT: u32 = 0x1FFFF;

    /// Enable spread spectrum (`SDM_EN`).
    #[doc(alias = "SDM_EN")]
    #[inline]
    pub const fn enable_sdm(self) -> Self {
        Self(self.0 | Self::SDM_EN)
    }
    /// Disable spread spectrum.
    #[inline]
    pub const fn disable_sdm(self) -> Self {
        Self(self.0 & !Self::SDM_EN)
    }
    /// Check if spread spectrum is enabled.
    #[inline]
    pub const fn is_sdm_enabled(self) -> bool {
        (self.0 & Self::SDM_EN) != 0
    }
    /// Set spread spectrum mode (`SDM_MODE`).
    #[doc(alias = "SDM_MODE")]
    #[inline]
    pub const fn set_sdm_mode(self, mode: SdmMode) -> Self {
        Self((self.0 & !Self::SDM_MODE) | (Self::SDM_MODE & ((mode as u32) << 29)))
    }
    /// Get spread spectrum mode.
    #[inline]
    pub const fn sdm_mode(self) -> SdmMode {
        match (self.0 & Self::SDM_MODE) >> 29 {
            0 => SdmMode::Dc0,
            1 => SdmMode::Dc1,
            2 => SdmMode::Triangular,
            _ => unreachable!(),
        }
    }
    /// Set spread spectrum step (`SDM_STEP`).
    #[doc(alias = "SDM_STEP")]
    #[inline]
    pub const fn set_sdm_step(self, val: u16) -> Self {
        assert!(
            val < 512,
            "Value out of bounds for SDM_STEP (expected 0..=511)"
        );
        Self((self.0 & !Self::SDM_STEP) | (Self::SDM_STEP & ((val as u32) << 20)))
    }
    /// Get spread spectrum step.
    #[inline]
    pub const fn sdm_step(self) -> u16 {
        ((self.0 & Self::SDM_STEP) >> 20) as u16
    }
    /// Set spread spectrum freqency (`SDM_FREQ`).
    #[doc(alias = "SDM_FREQ")]
    #[inline]
    pub const fn set_sdm_freq(self, freq: SdmFreq) -> Self {
        Self((self.0 & !Self::SDM_FREQ) | (Self::SDM_FREQ & ((freq as u32) << 17)))
    }
    /// Get spread spectrum freqency.
    #[inline]
    pub const fn sdm_freq(self) -> SdmFreq {
        match (self.0 & Self::SDM_FREQ) >> 17 {
            0 => SdmFreq::F31P5KHz,
            1 => SdmFreq::F32P0KHz,
            2 => SdmFreq::F32P5KHz,
            3 => SdmFreq::F33P0KHz,
            _ => unreachable!(),
        }
    }
    /// Set spread spectrum bottom level (`SDM_BOT`).
    ///
    /// The smaller the value, the higher the amplitude.
    #[doc(alias = "SDM_BOT")]
    pub const fn set_sdm_bottom(self, val: u32) -> Self {
        assert!(
            val < 0x20000,
            "Value out of bounds for SDM_BOT (expected 0..=0x1FFFF)"
        );
        Self((self.0 & !Self::SDM_BOT) | (Self::SDM_BOT & val))
    }
    /// Get spread spectrum bottom level.
    #[inline]
    pub const fn sdm_bottom(self) -> u32 {
        self.0 & Self::SDM_BOT
    }
}

/// Ldo voltage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LdoVoltage {
    /// 0.90V.
    U0P9V,
    /// 0.95V.
    U0P95V,
    /// 1.00V.
    U1P0V,
    /// 1.05V.
    U1P05V,
    /// 1.10V.
    U1P1V,
    /// 1.15V.
    U1P15V,
    /// 1.20V.
    U1P2V,
    /// 1.25V.
    U1P25V,
}

/// Pll common register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PllCommon(u32);

impl PllCommon {
    const COMMON_GATE: u32 = 0x1 << 31;
    const LDO_VSET: u32 = 0x7 << 1;
    const LDO_EN: u32 = 0x1;

    /// Enable the entire circuit (`COMMON_GATE`).
    ///
    /// Including PLL/LDO/BIAS.
    #[doc(alias = "COMMON_GATE")]
    #[inline]
    pub const fn enable_circuit(self) -> Self {
        Self(self.0 | Self::COMMON_GATE)
    }
    /// Disable the entire circuit.
    #[inline]
    pub const fn disable_circuit(self) -> Self {
        Self(self.0 & !Self::COMMON_GATE)
    }
    /// Check if the entire circuit is enabled.
    #[inline]
    pub const fn is_circuit_enabled(self) -> bool {
        (self.0 & Self::COMMON_GATE) != 0
    }
    /// Set ldo voltage (`LDO_VSET`).
    #[doc(alias = "LDO_VSET")]
    #[inline]
    pub const fn set_ldo_voltage(self, v: LdoVoltage) -> Self {
        Self((self.0 & !Self::LDO_VSET) | (Self::LDO_VSET & ((v as u32) << 1)))
    }
    /// Get ldo voltage.
    #[inline]
    pub const fn ldo_voltage(self) -> LdoVoltage {
        match (self.0 & Self::LDO_VSET) >> 1 {
            0 => LdoVoltage::U0P9V,
            1 => LdoVoltage::U0P95V,
            2 => LdoVoltage::U1P0V,
            3 => LdoVoltage::U1P05V,
            4 => LdoVoltage::U1P1V,
            5 => LdoVoltage::U1P15V,
            6 => LdoVoltage::U1P2V,
            7 => LdoVoltage::U1P25V,
            _ => unreachable!(),
        }
    }
    /// Enable ldo(`LDO_EN`).
    #[doc(alias = "LDO_EN")]
    #[inline]
    pub const fn enable_ldo(self) -> Self {
        Self(self.0 | Self::LDO_EN)
    }
    /// Disable ldo.
    #[inline]
    pub const fn disable_ldo(self) -> Self {
        Self(self.0 & !Self::LDO_EN)
    }
    /// Check if ldo is enabled.
    #[inline]
    pub const fn is_ldo_enabled(self) -> bool {
        (self.0 & Self::LDO_EN) != 0
    }
}

/// Oscillator output frequency selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OscOutFreq {
    /// 18 MHz.
    F18MHz,
    /// 20 MHz.
    F20MHz,
    /// 22 MHz.
    F22MHz,
    /// 24 MHz.
    F24MHz,
    /// 26 MHz.
    F26MHz,
    /// 28 MHz.
    F28MHz,
    /// 30 MHz.
    F30MHz,
    /// 32 MHz.
    F32MHz,
}

/// Clock selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ClockSelection {
    /// 24MHz crystal oscillator.
    Osc24M,
    /// External crystal oscillator.
    Xtal,
}

/// Pll input register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PllInput(u32);

impl PllInput {
    const XTAL_GM24M: u32 = 0x3 << 30;
    const XTAL_START24M: u32 = 0x1 << 29;
    const XTAL_EN: u32 = 0x1 << 28;
    const OSC_OUT_TR24M: u32 = 0x7F << 8;
    const OSC_OUT_SEL: u32 = 0x7 << 4;
    const OSC24M_EN: u32 = 0x1 << 1;
    const PLL_IN_SEL: u32 = 0x1;

    /// Set external crystal drive adjustment value (`XTAL_GM24M`).
    ///
    /// The larger the value, the stronger the drive.
    #[doc(alias = "XTAL_GM24M")]
    #[inline]
    pub const fn set_xtal_drive(self, val: u8) -> Self {
        assert!(
            val < 4,
            "Value out of bounds for XTAL_GM24M (expected 0..=3)"
        );
        Self((self.0 & !Self::XTAL_GM24M) | (Self::XTAL_GM24M & ((val as u32) << 30)))
    }
    /// Get external crystal drive adjustment value.
    #[inline]
    pub const fn xtal_drive(self) -> u8 {
        ((self.0 & Self::XTAL_GM24M) >> 30) as u8
    }
    /// Enable external crystal (`XTAL_START24M`).
    ///
    /// This register can be disabled after the external crystal has begun oscillation,
    /// thereby conserving power.
    #[doc(alias = "XTAL_START24M")]
    #[inline]
    pub const fn enable_xtal(self) -> Self {
        Self(self.0 | Self::XTAL_START24M)
    }
    /// Disable external crystal.
    #[inline]
    pub const fn disable_xtal(self) -> Self {
        Self(self.0 & !Self::XTAL_START24M)
    }
    /// Check if external crystal is enabled.
    #[inline]
    pub const fn is_xtal_enabled(self) -> bool {
        (self.0 & Self::XTAL_START24M) != 0
    }
    /// Enable external crystal oscillation (`XTAL_EN`).
    #[doc(alias = "XTAL_EN")]
    #[inline]
    pub const fn enable_xtal_osc(self) -> Self {
        Self(self.0 | Self::XTAL_EN)
    }
    /// Disable external crystal oscillation.
    #[inline]
    pub const fn disable_xtal_osc(self) -> Self {
        Self(self.0 & !Self::XTAL_EN)
    }
    /// Check if external crystal oscillation is enabled.
    #[inline]
    pub const fn is_xtal_osc_enabled(self) -> bool {
        (self.0 & Self::XTAL_EN) != 0
    }
    /// Set oscillator frequency (`OSC_OUT_TR24M`).
    ///
    /// - 0: 12 MHz.
    /// - ...
    /// - 64: 24 MHz.
    /// - ...
    /// - 127: 36 MHz.
    /// - Each step = 187.5 kHz.
    #[doc(alias = "OSC_OUT_TR24M")]
    #[inline]
    pub const fn set_osc_freq(self, val: u8) -> Self {
        assert!(
            val < 128,
            "Value out of bounds for OSC_OUT_TR24M (expected 0..=127)"
        );
        Self((self.0 & !Self::OSC_OUT_TR24M) | (Self::OSC_OUT_TR24M & ((val as u32) << 8)))
    }
    /// Get oscillator frequency.
    #[inline]
    pub const fn osc_freq(self) -> u8 {
        ((self.0 & Self::OSC_OUT_TR24M) >> 8) as u8
    }
    /// Set oscillator output frequency (`OSC_OUT_SEL`).
    #[doc(alias = "OSC_OUT_SEL")]
    #[inline]
    pub const fn set_osc_out_freq(self, freq: OscOutFreq) -> Self {
        Self((self.0 & !Self::OSC_OUT_SEL) | (Self::OSC_OUT_SEL & ((freq as u32) << 4)))
    }
    /// Get oscillator output frequency.
    #[inline]
    pub const fn osc_out_freq(self) -> OscOutFreq {
        match (self.0 & Self::OSC_OUT_SEL) >> 4 {
            0 => OscOutFreq::F18MHz,
            1 => OscOutFreq::F20MHz,
            2 => OscOutFreq::F22MHz,
            3 => OscOutFreq::F24MHz,
            4 => OscOutFreq::F26MHz,
            5 => OscOutFreq::F28MHz,
            6 => OscOutFreq::F30MHz,
            7 => OscOutFreq::F32MHz,
            _ => unreachable!(),
        }
    }
    /// Enable 24MHz crystal oscillation (`OSC24M_EN`).
    #[doc(alias = "OSC24M_EN")]
    #[inline]
    pub const fn enable_osc24m(self) -> Self {
        Self(self.0 | Self::OSC24M_EN)
    }
    /// Disable 24MHz crystal oscillation.
    #[inline]
    pub const fn disable_osc24m(self) -> Self {
        Self(self.0 & !Self::OSC24M_EN)
    }
    /// Check if 24MHz crystal oscillation is enabled.
    #[inline]
    pub const fn is_osc24m_enabled(self) -> bool {
        (self.0 & Self::OSC24M_EN) != 0
    }
    /// Set clock selection (`PLL_IN_SEL`).
    ///
    /// When using an external crystal oscillator,
    /// it is necessary to first configure `XTAL_EN`=1, delay for 1 ms, and then configure `PLL_IN_SEL`=1.
    #[doc(alias = "PLL_IN_SEL")]
    #[inline]
    pub const fn set_clk_sel(self, clock: ClockSelection) -> Self {
        Self((self.0 & !Self::PLL_IN_SEL) | (Self::PLL_IN_SEL & (clock as u32)))
    }
    /// Get clock selection.
    #[inline]
    pub const fn clk_sel(self) -> ClockSelection {
        match self.0 & Self::PLL_IN_SEL {
            0 => ClockSelection::Osc24M,
            1 => ClockSelection::Xtal,
            _ => unreachable!(),
        }
    }
}

/// Clock source selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ClockSourceSelection {
    /// PLL_INT1.
    PllInt1,
    /// PLL_FRA2.
    PllFra2 = 2,
}

/// Clock output register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ClockOutput(u32);

impl ClockOutput {
    const CLK_OUT_EN: u32 = 0x1 << 16;
    const CLK_SRC_SEL: u32 = 0x7 << 12;
    const CLK_DIV_N: u32 = 0xFF;

    /// Enable clock output (`CLK_OUT_EN`).
    ///
    /// ```text
    /// The clock output frequency CLK_OUT = CLK_SRC / (DIV_N + 1).
    /// The divider coefficient (DIV_N + 1) ranges from 1 to 256, and the clock output has a duty cycle of 50%.
    /// ```
    #[doc(alias = "CLK_OUT_EN")]
    #[inline]
    pub const fn enable_clk_out(self) -> Self {
        Self(self.0 | Self::CLK_OUT_EN)
    }
    /// Disable clock output.
    #[inline]
    pub const fn disable_clk_out(self) -> Self {
        Self(self.0 & !Self::CLK_OUT_EN)
    }
    /// Check if clock output is enabled.
    #[inline]
    pub const fn is_clk_out(self) -> bool {
        (self.0 & Self::CLK_OUT_EN) != 0
    }
    /// Set clock source selection (`CLK_SRC_SEL`).
    #[inline]
    pub const fn set_clk_src_sel(self, source: ClockSourceSelection) -> Self {
        Self((self.0 & !Self::CLK_SRC_SEL) | (Self::CLK_SRC_SEL & ((source as u32) << 12)))
    }
    /// Get clock source selection.
    #[inline]
    pub const fn clk_src_sel(self) -> ClockSourceSelection {
        match (self.0 & Self::CLK_SRC_SEL) >> 12 {
            0 => ClockSourceSelection::PllInt1,
            2 => ClockSourceSelection::PllFra2,
            _ => unreachable!(),
        }
    }
    /// Set clock divider coefficient (`CLK_DIV_N`).
    #[doc(alias = "CLK_DIV_N")]
    #[inline]
    pub const fn set_clk_div_n(self, val: u8) -> Self {
        Self((self.0 & !Self::CLK_DIV_N) | (Self::CLK_DIV_N & (val as u32)))
    }
    /// Get clock divider coefficient.
    #[inline]
    pub const fn clk_div_n(self) -> u8 {
        (self.0 & Self::CLK_DIV_N) as u8
    }
}

/// Clock selection on PLL_INTx.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ClockSelectionIntx {
    /// 24MHz clock.
    Clk24M,
    /// PLL_INTx/(DIV+1).
    ///
    /// The frequency coefficient is DIV + 1, with a 50% duty cycle,
    /// supporting dynamic frequency modulation.
    PllIntx,
}

/// Bus clock on PLL_INTx register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ClkOnPllIntx(u32);

impl ClkOnPllIntx {
    const BUS_CLK_SEL: u32 = 0x1 << 8;
    const BUS_CLK_DIV: u32 = 0x1F;

    /// Set bus clock (`BUS_CLK_SEL`).
    #[doc(alias = "BUS_CLK_SEL")]
    #[inline]
    pub const fn set_bus_clk(self, clk: ClockSelectionIntx) -> Self {
        Self((self.0 & !Self::BUS_CLK_SEL) | (Self::BUS_CLK_SEL & ((clk as u32) << 8)))
    }
    /// Get bus clock.
    #[inline]
    pub const fn bus_clk(self) -> ClockSelectionIntx {
        match (self.0 & Self::BUS_CLK_SEL) >> 8 {
            0 => ClockSelectionIntx::Clk24M,
            1 => ClockSelectionIntx::PllIntx,
            _ => unreachable!(),
        }
    }
    /// Set bus clock divider coefficient (`BUS_CLK_DIV`).
    #[doc(alias = "BUS_CLK_DIV")]
    #[inline]
    pub const fn set_bus_clk_div(self, val: u8) -> Self {
        assert!(
            val < 32,
            "Value out of bounds for BUS_CLK_DIV (expected 0..=31)"
        );
        Self((self.0 & !Self::BUS_CLK_DIV) | (Self::BUS_CLK_DIV & (val as u32)))
    }
    /// Get bus clock divider coefficient.
    #[inline]
    pub const fn bus_clk_div(self) -> u8 {
        (self.0 & Self::BUS_CLK_DIV) as u8
    }
}

/// Cpu clock register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CpuClock(u32);

impl CpuClock {
    const CPU_AXI_CLK_DIV: u32 = 0x1F << 16;
    const CPU_CLK_SEL: u32 = 0x1 << 8;
    const CLK_DIV: u32 = 0x1F;

    /// Set cpu axi clock divider coefficient (`CPU_AXI_CLK_DIV`|`CPU_CLK_USELESS`).
    ///
    /// ```text
    /// The frequency coefficient is DIV + 1 (1~32).
    /// The clock source comes from the CPU clock.
    /// This register is only meaningful for D21x chips.
    /// ```
    #[doc(alias = "CPU_AXI_CLK_DIV")]
    #[doc(alias = "CPU_CLK_USELESS")]
    #[inline]
    pub const fn set_cpu_axi_clk_div(self, val: u8) -> Self {
        assert!(
            val < 32,
            "Value out of bounds for CPU_AXI_CLK_DIV (expected 0..=31)"
        );
        Self((self.0 & !Self::CPU_AXI_CLK_DIV) | (Self::CPU_AXI_CLK_DIV & ((val as u32) << 16)))
    }
    /// Get cpu axi clock divider coefficient.
    #[inline]
    pub const fn cpu_axi_clk_div(self) -> u8 {
        ((self.0 & Self::CPU_AXI_CLK_DIV) >> 16) as u8
    }
    /// Set cpu clock (`CPU_CLK_SEL`).
    #[doc(alias = "CPU_CLK_SEL")]
    #[inline]
    pub const fn set_bus_clk(self, clk: ClockSelectionIntx) -> Self {
        Self((self.0 & !Self::CPU_CLK_SEL) | (Self::CPU_CLK_SEL & ((clk as u32) << 8)))
    }
    /// Get cpu clock.
    #[inline]
    pub const fn bus_clk(self) -> ClockSelectionIntx {
        match (self.0 & Self::CPU_CLK_SEL) >> 8 {
            0 => ClockSelectionIntx::Clk24M,
            1 => ClockSelectionIntx::PllIntx,
            _ => unreachable!(),
        }
    }
    /// Set input clock divider coefficient (`CLK_DIV`).
    #[doc(alias = "CLK_DIV")]
    #[inline]
    pub const fn set_clk_div(self, val: u8) -> Self {
        assert!(
            val < 32,
            "Value out of bounds for CLK_DIV (expected 0..=31)"
        );
        Self((self.0 & !Self::CLK_DIV) | (Self::CLK_DIV & (val as u32)))
    }
    /// Get input clock divider coefficient.
    #[inline]
    pub const fn clk_div(self) -> u8 {
        (self.0 & Self::CLK_DIV) as u8
    }
}

/// Wdog clock register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct WdogClock(u32);

impl WdogClock {
    const CLK_WDOG_WR_DIS: u32 = 0x1 << 28;
    const WDOG_RSTN: u32 = 0x1 << 13;
    const MOD_BUS_EN: u32 = 0x1 << 12;
    const MOD_CLK_EN: u32 = 0x1 << 8;

    /// Enable wdog clock read/write (`CLK_WDOG_WR_DIS`).
    ///
    /// ## WARNING ##
    /// Once this bit is set to 1, this register cannot be written to, meaning it cannot be cleared via software;
    /// it can only be reset during the chip's reset process.
    #[doc(alias = "CLK_WDOG_WR_DIS")]
    #[inline]
    pub const fn enable_wdog_clk_rw(self) -> Self {
        Self(self.0 & !Self::CLK_WDOG_WR_DIS)
    }
    /// Disable wdog clock read/write.
    #[inline]
    pub const fn disable_wdog_clk_rw(self) -> Self {
        Self(self.0 | Self::CLK_WDOG_WR_DIS)
    }
    /// Check if wdog clock read/write is enabled.
    #[inline]
    pub const fn is_wdog_clk_rw_enabled(self) -> bool {
        (self.0 & Self::CLK_WDOG_WR_DIS) == 0
    }
    /// Enable wdog reset (WDOG_RSTN).
    #[doc(alias = "WDOG_RSTN")]
    #[inline]
    pub const fn enable_wdog_rst(self) -> Self {
        Self(self.0 & !Self::WDOG_RSTN)
    }
    /// Disable wdog clock reset.
    #[inline]
    pub const fn disable_wdog_rst(self) -> Self {
        Self(self.0 | Self::WDOG_RSTN)
    }
    /// Check if wdog clock reset is enabled.
    #[inline]
    pub const fn is_wdog_rst_enabled(self) -> bool {
        (self.0 & Self::WDOG_RSTN) == 0
    }
    /// Enable bus clock (`MOD_BUS_EN`).
    #[doc(alias = "MOD_BUS_EN")]
    #[inline]
    pub const fn enable_bus_clk(self) -> Self {
        Self(self.0 | Self::MOD_BUS_EN)
    }
    /// Disable bus clock.
    #[inline]
    pub const fn disable_bus_clk(self) -> Self {
        Self(self.0 & !Self::MOD_BUS_EN)
    }
    /// Check if bus clock is enabled.
    #[inline]
    pub const fn is_bus_clk_enabled(self) -> bool {
        (self.0 & Self::MOD_BUS_EN) != 0
    }
    /// Enable module clock (`MOD_CLK_EN`).
    #[doc(alias = "MOD_CLK_EN")]
    #[inline]
    pub const fn enable_module_clk(self) -> Self {
        Self(self.0 | Self::MOD_CLK_EN)
    }
    /// Disable module clock.
    #[inline]
    pub const fn disable_module_clk(self) -> Self {
        Self(self.0 & !Self::MOD_CLK_EN)
    }
    /// Check if module clock is enabled.
    #[inline]
    pub const fn is_module_clk_enabled(self) -> bool {
        (self.0 & Self::MOD_CLK_EN) != 0
    }
}

/// Ddr clock register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DdrClock(u32);

impl DdrClock {
    const FIFO_RSTN: u32 = 0x1 << 20;
    const AXI_RSTN: u32 = 0x1 << 19;
    const AXI_CLK_EN: u32 = 0x1 << 18;
    const AHB_RSTN: u32 = 0x1 << 17;
    const AHB_CLK_EN: u32 = 0x1 << 16;
    const PHY_APB_RSTN: u32 = 0x1 << 15;
    const PHY_APB_CLK_EN: u32 = 0x1 << 14;
    const PHY_RSTN: u32 = 0x1 << 13;
    const PHY_CLK_EN: u32 = 0x1 << 12;
    const CTRL_APB_RSTN: u32 = 0x1 << 11;
    const CTRL_APB_CLK_EN: u32 = 0x1 << 10;
    const CTRL_RSTN: u32 = 0x1 << 9;
    const CTRL_CLK_EN: u32 = 0x1 << 8;
    const MOD_CLK_DIV: u32 = 0x1F;

    /// Enable fifo reset (`FIFO_RSTN`).
    #[doc(alias = "FIFO_RSTN")]
    #[inline]
    pub const fn enable_fifo_reset(self) -> Self {
        Self(self.0 & !Self::FIFO_RSTN)
    }
    /// Disable fifo reset.
    #[inline]
    pub const fn disable_fifo_reset(self) -> Self {
        Self(self.0 | Self::FIFO_RSTN)
    }
    /// Check if fifo reset is enabled.
    #[inline]
    pub const fn is_fifo_reset_enabled(self) -> bool {
        (self.0 & Self::FIFO_RSTN) == 0
    }
    /// Enable axi bus reset (`AXI_RSTN`).
    #[doc(alias = "AXI_RSTN")]
    #[inline]
    pub const fn enable_axi_bus_reset(self) -> Self {
        Self(self.0 & !Self::AXI_RSTN)
    }
    /// Disable axi bus reset.
    #[inline]
    pub const fn disable_axi_bus_reset(self) -> Self {
        Self(self.0 | Self::AXI_RSTN)
    }
    /// Check if axi bus reset is enabled.
    #[inline]
    pub const fn is_axi_bus_reset_enabled(self) -> bool {
        (self.0 & Self::AXI_RSTN) == 0
    }
    /// Enable axi clock (`AXI_CLK_EN`).
    #[doc(alias = "AXI_CLK_EN")]
    #[inline]
    pub const fn enable_axi_clk(self) -> Self {
        Self(self.0 | Self::AXI_CLK_EN)
    }
    /// Disable axi clock.
    #[inline]
    pub const fn disable_axi_clk(self) -> Self {
        Self(self.0 & !Self::AXI_CLK_EN)
    }
    /// Check if axi clock is enabled.
    #[inline]
    pub const fn is_axi_clk_enabled(self) -> bool {
        (self.0 & Self::AXI_CLK_EN) != 0
    }
    /// Enable ahb bus reset (`AHB_RSTN`).
    #[doc(alias = "AHB_RSTN")]
    #[inline]
    pub const fn enable_ahb_bus_reset(self) -> Self {
        Self(self.0 & !Self::AHB_RSTN)
    }
    /// Disable ahb bus reset.
    #[inline]
    pub const fn disable_ahb_bus_reset(self) -> Self {
        Self(self.0 | Self::AHB_RSTN)
    }
    /// Check if ahb bus reset is enabled.
    #[inline]
    pub const fn is_ahb_bus_reset_enabled(self) -> bool {
        (self.0 & Self::AHB_RSTN) == 0
    }
    /// Enable ahb clock (`AHB_CLK_EN`).
    #[doc(alias = "AHB_CLK_EN")]
    #[inline]
    pub const fn enable_ahb_clk(self) -> Self {
        Self(self.0 | Self::AHB_CLK_EN)
    }
    /// Disable ahb clock.
    #[inline]
    pub const fn disable_ahb_clk(self) -> Self {
        Self(self.0 & !Self::AHB_CLK_EN)
    }
    /// Check if ahb clock is enabled.
    #[inline]
    pub const fn is_ahb_clk_enabled(self) -> bool {
        (self.0 & Self::AHB_CLK_EN) != 0
    }
    /// Enable phy apb bus reset (`PHY_APB_RSTN`).
    #[doc(alias = "PHY_APB_RSTN")]
    #[inline]
    pub const fn enable_phy_apb_bus_reset(self) -> Self {
        Self(self.0 & !Self::PHY_APB_RSTN)
    }
    /// Disable phy apb bus reset.
    #[inline]
    pub const fn disable_phy_apb_bus_reset(self) -> Self {
        Self(self.0 | Self::PHY_APB_RSTN)
    }
    /// Check if phy apb bus reset is enabled.
    #[inline]
    pub const fn is_phy_apb_bus_reset_enabled(self) -> bool {
        (self.0 & Self::PHY_APB_RSTN) == 0
    }
    /// Enable phy apb clock (`PHY_APB_CLK_EN`).
    #[doc(alias = "PHY_APB_CLK_EN")]
    #[inline]
    pub const fn enable_phy_apb_clk(self) -> Self {
        Self(self.0 | Self::PHY_APB_CLK_EN)
    }
    /// Disable phy apb clock.
    #[inline]
    pub const fn disable_phy_apb_clk(self) -> Self {
        Self(self.0 & !Self::PHY_APB_CLK_EN)
    }
    /// Check if phy apb clock is enabled.
    #[inline]
    pub const fn is_phy_apb_clk_enabled(self) -> bool {
        (self.0 & Self::PHY_APB_CLK_EN) != 0
    }
    /// Enable phy reset (`PHY_RSTN`).
    #[doc(alias = "PHY_RSTN")]
    #[inline]
    pub const fn enable_phy_reset(self) -> Self {
        Self(self.0 & !Self::PHY_RSTN)
    }
    /// Disable phy reset.
    #[inline]
    pub const fn disable_phy_reset(self) -> Self {
        Self(self.0 | Self::PHY_RSTN)
    }
    /// Check if phy reset is enabled.
    #[inline]
    pub const fn is_phy_reset_enabled(self) -> bool {
        (self.0 & Self::PHY_RSTN) == 0
    }
    /// Enable phy module clock (`PHY_CLK_EN`).
    #[doc(alias = "PHY_CLK_EN")]
    #[inline]
    pub const fn enable_phy_module_clk(self) -> Self {
        Self(self.0 | Self::PHY_CLK_EN)
    }
    /// Disable phy module clock.
    #[inline]
    pub const fn disable_phy_module_clk(self) -> Self {
        Self(self.0 & !Self::PHY_CLK_EN)
    }
    /// Check if phy module clock is enabled.
    #[inline]
    pub const fn is_phy_module_clk_enabled(self) -> bool {
        (self.0 & Self::PHY_CLK_EN) != 0
    }
    /// Enable controller apb bus clock reset (`CTRL_APB_RSTN`).
    #[doc(alias = "CTRL_APB_RSTN")]
    #[inline]
    pub const fn enable_ctrl_apb_reset(self) -> Self {
        Self(self.0 & !Self::CTRL_APB_RSTN)
    }
    /// Disable controller apb bus clock reset.
    #[inline]
    pub const fn disable_ctrl_apb_reset(self) -> Self {
        Self(self.0 | Self::CTRL_APB_RSTN)
    }
    /// Check if controller apb bus clock reset is enabled.
    #[inline]
    pub const fn is_ctrl_apb_reset_enabled(self) -> bool {
        (self.0 & Self::CTRL_APB_RSTN) == 0
    }
    /// Enable controller apb bus clock (`CTRL_APB_CLK_EN`).
    #[doc(alias = "CTRL_APB_CLK_EN")]
    #[inline]
    pub const fn enable_ctrl_apb_clk(self) -> Self {
        Self(self.0 | Self::CTRL_APB_CLK_EN)
    }
    /// Disable controller apb bus clock.
    #[inline]
    pub const fn disable_ctrl_apb_clk(self) -> Self {
        Self(self.0 & !Self::CTRL_APB_CLK_EN)
    }
    /// Check if controller apb bus clock is enabled.
    #[inline]
    pub const fn is_ctrl_apb_clk_enabled(self) -> bool {
        (self.0 & Self::CTRL_APB_CLK_EN) != 0
    }
    /// Enable controller reset (`CTRL_RSTN`).
    #[doc(alias = "CTRL_RSTN")]
    #[inline]
    pub const fn enable_ctrl_reset(self) -> Self {
        Self(self.0 & !Self::CTRL_RSTN)
    }
    /// Disable controller reset.
    #[inline]
    pub const fn disable_ctrl_reset(self) -> Self {
        Self(self.0 | Self::CTRL_RSTN)
    }
    /// Check if controller reset is enabled.
    #[inline]
    pub const fn is_ctrl_reset_enabled(self) -> bool {
        (self.0 & Self::CTRL_RSTN) == 0
    }
    /// Enable controller clock (`CTRL_CLK_EN`).
    #[doc(alias = "CTRL_CLK_EN")]
    #[inline]
    pub const fn enable_ctrl_clk(self) -> Self {
        Self(self.0 | Self::CTRL_CLK_EN)
    }
    /// Disable controller clock.
    #[inline]
    pub const fn disable_ctrl_clk(self) -> Self {
        Self(self.0 & !Self::CTRL_CLK_EN)
    }
    /// Check if controller clock is enabled.
    #[inline]
    pub const fn is_ctrl_clk_enabled(self) -> bool {
        (self.0 & Self::CTRL_CLK_EN) != 0
    }
    /// Set module clock divider coefficient (`MOD_CLK_DIV`).
    ///
    /// The module clock divider coefficient is DIV + 1, with a 50% duty cycle.
    #[doc(alias = "MOD_CLK_DIV")]
    #[inline]
    pub const fn set_module_clk_div(self, val: u8) -> Self {
        assert!(
            val < 32,
            "Value out of bounds for MOD_CLK_DIV (expected 0..=31)"
        );
        Self((self.0 & !Self::MOD_CLK_DIV) | (Self::MOD_CLK_DIV & (val as u32)))
    }
    /// Get module clock divider coefficient.
    #[inline]
    pub const fn module_clk_div(self) -> u8 {
        (self.0 & Self::MOD_CLK_DIV) as u8
    }
}

/// Pixel clock divider coefficient selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PixelClockSelection {
    /// The integer divider coefficient,
    /// with a 50% duty cycle,
    /// is expressed as 2L × (M + 1).
    CalculateByDivs,
    /// Fixed divider coefficient of 3.5, not 50% duty cycle.
    FixDiv,
}

/// Display clock register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DispClock(u32);

impl DispClock {
    const PIXCLK_DIV_SEL: u32 = 0x1 << 12;
    const PIXCLK_DIV_L: u32 = 0x3 << 10;
    const PIXCLK_DIV_M: u32 = 0x1F << 4;
    const SCLK_DIV_N: u32 = 0x7;

    /// Set pixel clock divider coefficient selection (`PIXCLK_DIV_SEL`).
    ///
    /// The pixel clock is derived from the serial clock.
    #[doc(alias = "PIXCLK_DIV_SEL")]
    #[inline]
    pub const fn set_pix_clk_div(self, selection: PixelClockSelection) -> Self {
        Self((self.0 & !Self::PIXCLK_DIV_SEL) | (Self::PIXCLK_DIV_SEL & ((selection as u32) << 12)))
    }
    /// Get pixel clock divider coefficient selection.
    #[inline]
    pub const fn pix_clk_div(self) -> PixelClockSelection {
        match (self.0 & Self::PIXCLK_DIV_SEL) >> 12 {
            0 => PixelClockSelection::CalculateByDivs,
            1 => PixelClockSelection::FixDiv,
            _ => unreachable!(),
        }
    }
    /// Set pixel clock divider coefficient l (`PIXCLK_DIV_L`).
    ///
    /// The divider coefficient is 2^L (1/2/4/8), with a 50% duty cycle.
    #[doc(alias = "PIXCLK_DIV_L")]
    #[inline]
    pub const fn set_pix_clk_div_l(self, val: u8) -> Self {
        assert!(
            val < 4,
            "Value out of bounds for PIXCLK_DIV_L (expected 0..=3)"
        );
        Self((self.0 & !Self::PIXCLK_DIV_L) | (Self::PIXCLK_DIV_L & ((val as u32) << 10)))
    }
    /// Get pixel clock divider coefficient l.
    #[inline]
    pub const fn pix_clk_div_l(self) -> u8 {
        ((self.0 & Self::PIXCLK_DIV_L) >> 10) as u8
    }
    /// Set pixel clock divider coefficient m (`PIXCLK_DIV_M`).
    ///
    /// The divider coefficient is M+1 (1~32), with a 50% duty cycle.
    #[doc(alias = "PIXCLK_DIV_M")]
    #[inline]
    pub const fn set_pix_clk_div_m(self, val: u8) -> Self {
        assert!(
            val < 32,
            "Value out of bounds for PIXCLK_DIV_M (expected 0..=31)"
        );
        Self((self.0 & !Self::PIXCLK_DIV_M) | (Self::PIXCLK_DIV_M & ((val as u32) << 4)))
    }
    /// Get pixel clock divider coefficient m.
    #[inline]
    pub const fn pix_clk_div_m(self) -> u8 {
        ((self.0 & Self::PIXCLK_DIV_M) >> 4) as u8
    }
    /// Set serial clock divider coefficient n (`SCLK_DIV_N`).
    ///
    /// The divider coefficient is 2^N (1/2/4/8/16/32/64/128), with a 50% duty cycle.
    #[doc(alias = "SCLK_DIV_N")]
    #[inline]
    pub const fn set_serial_clk_div_n(self, val: u8) -> Self {
        assert!(
            val < 8,
            "Value out of bounds for SCLK_DIV_N (expected 0..=7)"
        );
        Self((self.0 & !Self::SCLK_DIV_N) | (Self::SCLK_DIV_N & (val as u32)))
    }
    /// Get serial clock divider coefficient n.
    #[inline]
    pub const fn serial_clk_div_n(self) -> u8 {
        (self.0 & Self::SCLK_DIV_N) as u8
    }
}

/// Serial clock divider coefficient.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SerialClockDivider {
    /// The clock divider coefficient is 49.
    ///
    /// AUD_SCLK is approximately 24.490 MHz, which is about 24.576 MHz for 48 KHz applications.
    Div49,
    /// The clock divider coefficient is 53.
    ///
    /// AUD_SCLK is approximately 22.6415 MHz, about 22.5792 MHz for 44.1 KHz applications, with a precision deviation of within 5‰.
    Div53,
}

/// Audio serial clock register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AudioSerialClock(u32);

impl AudioSerialClock {
    const SCLK_DIV: u32 = 0x1;

    /// Set serial clock divider coefficient (`SCLK_DIV`).
    ///
    /// SCLK is sourced from PLL_INT1, with a fixed output of 1.2 GHz.
    #[doc(alias = "SCLK_DIV")]
    #[inline]
    pub const fn set_serial_clk_div(self, div: SerialClockDivider) -> Self {
        Self((self.0 & !Self::SCLK_DIV) | (Self::SCLK_DIV & (div as u32)))
    }
    /// Get serial clock divider coefficient.
    #[inline]
    pub const fn serial_clk_div(self) -> SerialClockDivider {
        match self.0 & Self::SCLK_DIV {
            0 => SerialClockDivider::Div49,
            1 => SerialClockDivider::Div53,
            _ => unreachable!(),
        }
    }
}

/// Simple module type 0 clock register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SimpleModule0Clock(u32);

impl SimpleModule0Clock {
    const MOD_CLK_DIV: u32 = 0x1F;

    /// Set module clock divider coefficient (`MOD_CLK_DIV`).
    ///
    /// The module clock divider coefficient is DIV + 1, with a 50% duty cycle.
    #[doc(alias = "MOD_CLK_DIV")]
    #[inline]
    pub const fn set_module_clk_div(self, val: u8) -> Self {
        assert!(
            val < 32,
            "Value out of bounds for MOD_CLK_DIV (expected 0..=31)"
        );
        Self((self.0 & !Self::MOD_CLK_DIV) | (Self::MOD_CLK_DIV & (val as u32)))
    }
    /// Get module clock divider coefficient.
    #[inline]
    pub const fn module_clk_div(self) -> u8 {
        (self.0 & Self::MOD_CLK_DIV) as u8
    }
}

/// Simple module type 1 clock register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SimpleModule1Clock(u32);

impl SimpleModule1Clock {
    const MOD_BUS_EN: u32 = 0x1 << 12;

    /// Enable bus clock (`MOD_BUS_EN`).
    #[doc(alias = "MOD_BUS_EN")]
    #[inline]
    pub const fn enable_bus_clk(self) -> Self {
        Self(self.0 | Self::MOD_BUS_EN)
    }
    /// Disable bus clock.
    #[inline]
    pub const fn disable_bus_clk(self) -> Self {
        Self(self.0 & !Self::MOD_BUS_EN)
    }
    /// Check if bus clock is enabled.
    #[inline]
    pub const fn is_bus_clk_enabled(self) -> bool {
        (self.0 & Self::MOD_BUS_EN) != 0
    }
}

/// Simple module type 2 clock register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SimpleModule2Clock(u32);

impl SimpleModule2Clock {
    const MOD_RSTN: u32 = 0x1 << 13;
    const MOD_BUS_EN: u32 = 0x1 << 12;

    /// Enable module reset (`MOD_RSTN`).
    ///
    /// This signal simultaneously resets the module and the register bus.
    #[doc(alias = "MOD_RSTN")]
    #[inline]
    pub const fn enable_module_reset(self) -> Self {
        Self(self.0 & !Self::MOD_RSTN)
    }
    /// Disable module reset.
    #[inline]
    pub const fn disable_module_reset(self) -> Self {
        Self(self.0 | Self::MOD_RSTN)
    }
    /// Check if module reset is enabled.
    #[inline]
    pub const fn is_module_reset_enabled(self) -> bool {
        (self.0 & Self::MOD_RSTN) == 0
    }
    /// Enable bus clock (`MOD_BUS_EN`).
    #[doc(alias = "MOD_BUS_EN")]
    #[inline]
    pub const fn enable_bus_clk(self) -> Self {
        Self(self.0 | Self::MOD_BUS_EN)
    }
    /// Disable bus clock.
    #[inline]
    pub const fn disable_bus_clk(self) -> Self {
        Self(self.0 & !Self::MOD_BUS_EN)
    }
    /// Check if bus clock is enabled.
    #[inline]
    pub const fn is_bus_clk_enabled(self) -> bool {
        (self.0 & Self::MOD_BUS_EN) != 0
    }
}

/// Simple module type 3 clock register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SimpleModule3Clock(u32);

impl SimpleModule3Clock {
    const MOD_BUS_EN: u32 = 0x1 << 12;
    const MOD_CLK_EN: u32 = 0x1 << 8;

    /// Enable bus clock (`MOD_BUS_EN`).
    #[doc(alias = "MOD_BUS_EN")]
    #[inline]
    pub const fn enable_bus_clk(self) -> Self {
        Self(self.0 | Self::MOD_BUS_EN)
    }
    /// Disable bus clock.
    #[inline]
    pub const fn disable_bus_clk(self) -> Self {
        Self(self.0 & !Self::MOD_BUS_EN)
    }
    /// Check if bus clock is enabled.
    #[inline]
    pub const fn is_bus_clk_enabled(self) -> bool {
        (self.0 & Self::MOD_BUS_EN) != 0
    }
    /// Enable module clock (`MOD_CLK_EN`).
    #[doc(alias = "MOD_CLK_EN")]
    #[inline]
    pub const fn enable_module_clk(self) -> Self {
        Self(self.0 | Self::MOD_CLK_EN)
    }
    /// Disable module clock.
    #[inline]
    pub const fn disable_module_clk(self) -> Self {
        Self(self.0 & !Self::MOD_CLK_EN)
    }
    /// Check if module clock is enabled.
    #[inline]
    pub const fn is_module_clk_enabled(self) -> bool {
        (self.0 & Self::MOD_CLK_EN) != 0
    }
}

/// Simple module type 4 clock register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SimpleModule4Clock(u32);

impl SimpleModule4Clock {
    const MOD_RSTN: u32 = 0x1 << 13;
    const MOD_CLK_EN: u32 = 0x1 << 8;

    /// Enable module reset (`MOD_RSTN`).
    ///
    /// This signal simultaneously resets the module and the register bus.
    #[doc(alias = "MOD_RSTN")]
    #[inline]
    pub const fn enable_module_reset(self) -> Self {
        Self(self.0 & !Self::MOD_RSTN)
    }
    /// Disable module reset.
    #[inline]
    pub const fn disable_module_reset(self) -> Self {
        Self(self.0 | Self::MOD_RSTN)
    }
    /// Check if module reset is enabled.
    #[inline]
    pub const fn is_module_reset_enabled(self) -> bool {
        (self.0 & Self::MOD_RSTN) == 0
    }
    /// Enable module clock (`MOD_CLK_EN`).
    #[doc(alias = "MOD_CLK_EN")]
    #[inline]
    pub const fn enable_module_clk(self) -> Self {
        Self(self.0 | Self::MOD_CLK_EN)
    }
    /// Disable module clock.
    #[inline]
    pub const fn disable_module_clk(self) -> Self {
        Self(self.0 & !Self::MOD_CLK_EN)
    }
    /// Check if module clock is enabled.
    #[inline]
    pub const fn is_module_clk_enabled(self) -> bool {
        (self.0 & Self::MOD_CLK_EN) != 0
    }
}

/// Simple module type 5 clock register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SimpleModule5Clock(u32);

impl SimpleModule5Clock {
    const MOD_RSTN: u32 = 0x1 << 13;
    const MOD_BUS_EN: u32 = 0x1 << 12;
    const MOD_CLK_EN: u32 = 0x1 << 8;

    /// Enable module reset (`MOD_RSTN`).
    ///
    /// This signal simultaneously resets the module and the register bus.
    #[doc(alias = "MOD_RSTN")]
    #[inline]
    pub const fn enable_module_reset(self) -> Self {
        Self(self.0 & !Self::MOD_RSTN)
    }
    /// Disable module reset.
    #[inline]
    pub const fn disable_module_reset(self) -> Self {
        Self(self.0 | Self::MOD_RSTN)
    }
    /// Check if module reset is enabled.
    #[inline]
    pub const fn is_module_reset_enabled(self) -> bool {
        (self.0 & Self::MOD_RSTN) == 0
    }
    /// Enable bus clock (`MOD_BUS_EN`).
    #[doc(alias = "MOD_BUS_EN")]
    #[inline]
    pub const fn enable_bus_clk(self) -> Self {
        Self(self.0 | Self::MOD_BUS_EN)
    }
    /// Disable bus clock.
    #[inline]
    pub const fn disable_bus_clk(self) -> Self {
        Self(self.0 & !Self::MOD_BUS_EN)
    }
    /// Check if bus clock is enabled.
    #[inline]
    pub const fn is_bus_clk_enabled(self) -> bool {
        (self.0 & Self::MOD_BUS_EN) != 0
    }
    /// Enable module clock (`MOD_CLK_EN`).
    #[doc(alias = "MOD_CLK_EN")]
    #[inline]
    pub const fn enable_module_clk(self) -> Self {
        Self(self.0 | Self::MOD_CLK_EN)
    }
    /// Disable module clock.
    #[inline]
    pub const fn disable_module_clk(self) -> Self {
        Self(self.0 & !Self::MOD_CLK_EN)
    }
    /// Check if module clock is enabled.
    #[inline]
    pub const fn is_module_clk_enabled(self) -> bool {
        (self.0 & Self::MOD_CLK_EN) != 0
    }
}

/// Simple module type 6 clock register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SimpleModule6Clock(u32);

impl SimpleModule6Clock {
    const MOD_RSTN: u32 = 0x1 << 13;
    const MOD_CLK_EN: u32 = 0x1 << 8;
    const MOD_CLK_DIV: u32 = 0x1F;

    /// Enable module reset (`MOD_RSTN`).
    ///
    /// This signal simultaneously resets the module and the register bus.
    #[doc(alias = "MOD_RSTN")]
    #[inline]
    pub const fn enable_module_reset(self) -> Self {
        Self(self.0 & !Self::MOD_RSTN)
    }
    /// Disable module reset.
    #[inline]
    pub const fn disable_module_reset(self) -> Self {
        Self(self.0 | Self::MOD_RSTN)
    }
    /// Check if module reset is enabled.
    #[inline]
    pub const fn is_module_reset_enabled(self) -> bool {
        (self.0 & Self::MOD_RSTN) == 0
    }
    /// Enable module clock (`MOD_CLK_EN`).
    #[doc(alias = "MOD_CLK_EN")]
    #[inline]
    pub const fn enable_module_clk(self) -> Self {
        Self(self.0 | Self::MOD_CLK_EN)
    }
    /// Disable module clock.
    #[inline]
    pub const fn disable_module_clk(self) -> Self {
        Self(self.0 & !Self::MOD_CLK_EN)
    }
    /// Check if module clock is enabled.
    #[inline]
    pub const fn is_module_clk_enabled(self) -> bool {
        (self.0 & Self::MOD_CLK_EN) != 0
    }
    /// Set module clock divider coefficient (`MOD_CLK_DIV`).
    ///
    /// The module clock divider coefficient is DIV + 1, with a 50% duty cycle.
    #[doc(alias = "MOD_CLK_DIV")]
    #[inline]
    pub const fn set_module_clk_div(self, val: u8) -> Self {
        assert!(
            val < 32,
            "Value out of bounds for MOD_CLK_DIV (expected 0..=31)"
        );
        Self((self.0 & !Self::MOD_CLK_DIV) | (Self::MOD_CLK_DIV & (val as u32)))
    }
    /// Get module clock divider coefficient.
    #[inline]
    pub const fn module_clk_div(self) -> u8 {
        (self.0 & Self::MOD_CLK_DIV) as u8
    }
}

/// Normal module clock register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NormalModuleClock(u32);

impl NormalModuleClock {
    const MOD_RSTN: u32 = 0x1 << 13;
    const MOD_BUS_EN: u32 = 0x1 << 12;
    const MOD_CLK_EN: u32 = 0x1 << 8;
    const MOD_CLK_DIV: u32 = 0x1F;

    /// Enable module reset (`MOD_RSTN`).
    ///
    /// This signal simultaneously resets the module and the register bus.
    #[doc(alias = "MOD_RSTN")]
    #[inline]
    pub const fn enable_module_reset(self) -> Self {
        Self(self.0 & !Self::MOD_RSTN)
    }
    /// Disable module reset.
    #[inline]
    pub const fn disable_module_reset(self) -> Self {
        Self(self.0 | Self::MOD_RSTN)
    }
    /// Check if module reset is enabled.
    #[inline]
    pub const fn is_module_reset_enabled(self) -> bool {
        (self.0 & Self::MOD_RSTN) == 0
    }
    /// Enable bus clock (`MOD_BUS_EN`).
    #[doc(alias = "MOD_BUS_EN")]
    #[inline]
    pub const fn enable_bus_clk(self) -> Self {
        Self(self.0 | Self::MOD_BUS_EN)
    }
    /// Disable bus clock.
    #[inline]
    pub const fn disable_bus_clk(self) -> Self {
        Self(self.0 & !Self::MOD_BUS_EN)
    }
    /// Check if bus clock is enabled.
    #[inline]
    pub const fn is_bus_clk_enabled(self) -> bool {
        (self.0 & Self::MOD_BUS_EN) != 0
    }
    /// Enable module clock (`MOD_CLK_EN`).
    #[doc(alias = "MOD_CLK_EN")]
    #[inline]
    pub const fn enable_module_clk(self) -> Self {
        Self(self.0 | Self::MOD_CLK_EN)
    }
    /// Disable module clock.
    #[inline]
    pub const fn disable_module_clk(self) -> Self {
        Self(self.0 & !Self::MOD_CLK_EN)
    }
    /// Check if module clock is enabled.
    #[inline]
    pub const fn is_module_clk_enabled(self) -> bool {
        (self.0 & Self::MOD_CLK_EN) != 0
    }
    /// Set module clock divider coefficient (`MOD_CLK_DIV`).
    ///
    /// The module clock divider coefficient is DIV + 1, with a 50% duty cycle.
    #[doc(alias = "MOD_CLK_DIV")]
    #[inline]
    pub const fn set_module_clk_div(self, val: u8) -> Self {
        assert!(
            val < 32,
            "Value out of bounds for MOD_CLK_DIV (expected 0..=31)"
        );
        Self((self.0 & !Self::MOD_CLK_DIV) | (Self::MOD_CLK_DIV & (val as u32)))
    }
    /// Get module clock divider coefficient.
    #[inline]
    pub const fn module_clk_div(self) -> u8 {
        (self.0 & Self::MOD_CLK_DIV) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AudioSerialClock, BiasCurrentMultiplier, ClkOnPllIntx, ClockOutput, ClockSelection,
        ClockSelectionIntx, ClockSourceSelection, CpuClock, DdrClock, DispClock, LdoBiasCurrent,
        LdoVoltage, NormalModuleClock, OscOutFreq, PixelClockSelection, PllCommon, PllFraConfig,
        PllFraSdm, PllGeneral, PllInput, PllIntConfig, PllOutput, RegisterBlock, SdmFreq, SdmMode,
        SerialClockDivider, SimpleModule0Clock, SimpleModule1Clock, SimpleModule2Clock,
        SimpleModule3Clock, SimpleModule4Clock, SimpleModule5Clock, SimpleModule6Clock, WdogClock,
    };
    use crate::test_should_panic;
    use core::mem::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, pll_int0_general), 0x0);
        assert_eq!(offset_of!(RegisterBlock, pll_int1_general), 0x4);
        assert_eq!(offset_of!(RegisterBlock, pll_fra0_general), 0x20);
        assert_eq!(offset_of!(RegisterBlock, pll_fra1_general), 0x24);
        assert_eq!(offset_of!(RegisterBlock, pll_fra2_general), 0x28);
        assert_eq!(offset_of!(RegisterBlock, pll_int0_config), 0x40);
        assert_eq!(offset_of!(RegisterBlock, pll_int1_config), 0x44);
        assert_eq!(offset_of!(RegisterBlock, pll_fra0_config), 0x60);
        assert_eq!(offset_of!(RegisterBlock, pll_fra1_config), 0x64);
        assert_eq!(offset_of!(RegisterBlock, pll_fra2_config), 0x68);
        assert_eq!(offset_of!(RegisterBlock, pll_fra0_spread_spectrum), 0x80);
        assert_eq!(offset_of!(RegisterBlock, pll_fra1_spread_spectrum), 0x84);
        assert_eq!(offset_of!(RegisterBlock, pll_fra2_spread_spectrum), 0x88);
        assert_eq!(offset_of!(RegisterBlock, pll_common), 0xA0);
        assert_eq!(offset_of!(RegisterBlock, pll_input), 0xA4);
        assert_eq!(offset_of!(RegisterBlock, clock_out0), 0xE0);
        assert_eq!(offset_of!(RegisterBlock, clock_out1), 0xE4);
        assert_eq!(offset_of!(RegisterBlock, clock_out2), 0xE8);
        assert_eq!(offset_of!(RegisterBlock, clock_out3), 0xEC);
        assert_eq!(offset_of!(RegisterBlock, clock_axi_ahb), 0x100);
        assert_eq!(offset_of!(RegisterBlock, clock_ahb), 0x110);
        assert_eq!(offset_of!(RegisterBlock, clock_apb0), 0x120);
        assert_eq!(offset_of!(RegisterBlock, clock_cpu), 0x200);
        assert_eq!(offset_of!(RegisterBlock, clock_dm), 0x204);
        assert_eq!(offset_of!(RegisterBlock, clock_wdog), 0x20C);
        assert_eq!(offset_of!(RegisterBlock, clock_ddr), 0x210);
        assert_eq!(offset_of!(RegisterBlock, clock_disp), 0x220);
        assert_eq!(offset_of!(RegisterBlock, clock_audio_serial), 0x230);
        assert_eq!(offset_of!(RegisterBlock, clock_pwmcs_sdfm), 0x240);
        assert_eq!(offset_of!(RegisterBlock, clock_dma), 0x410);
        assert_eq!(offset_of!(RegisterBlock, clock_ce), 0x418);
        assert_eq!(offset_of!(RegisterBlock, clock_usb_dev), 0x41C);
        assert_eq!(offset_of!(RegisterBlock, clock_usb_host0), 0x420);
        assert_eq!(offset_of!(RegisterBlock, clock_usb_host1), 0x424);
        assert_eq!(offset_of!(RegisterBlock, clock_usb_phy0), 0x430);
        assert_eq!(offset_of!(RegisterBlock, clock_usb_phy1), 0x434);
        assert_eq!(offset_of!(RegisterBlock, clock_emac_gmac0), 0x440);
        assert_eq!(offset_of!(RegisterBlock, clock_gmac1), 0x444);
        assert_eq!(offset_of!(RegisterBlock, clock_xspi), 0x45C);
        assert_eq!(offset_of!(RegisterBlock, clock_qspi0), 0x460);
        assert_eq!(offset_of!(RegisterBlock, clock_qspi1), 0x464);
        assert_eq!(offset_of!(RegisterBlock, clock_qspi2), 0x468);
        assert_eq!(offset_of!(RegisterBlock, clock_qspi3), 0x46C);
        assert_eq!(offset_of!(RegisterBlock, clock_sdmc0), 0x470);
        assert_eq!(offset_of!(RegisterBlock, clock_sdmc1), 0x474);
        assert_eq!(offset_of!(RegisterBlock, clock_sdmc2), 0x478);
        assert_eq!(offset_of!(RegisterBlock, clock_cordic), 0x490);
        assert_eq!(offset_of!(RegisterBlock, clock_hcl), 0x494);
        assert_eq!(offset_of!(RegisterBlock, clock_pbus), 0x4A0);
        assert_eq!(offset_of!(RegisterBlock, clock_syscfg), 0x800);
        assert_eq!(offset_of!(RegisterBlock, clock_spi_enc), 0x810);
        assert_eq!(offset_of!(RegisterBlock, clock_pwmcs), 0x814);
        assert_eq!(offset_of!(RegisterBlock, clock_psadc), 0x818);
        assert_eq!(offset_of!(RegisterBlock, clock_mtop), 0x81C);
        assert_eq!(offset_of!(RegisterBlock, clock_i2s0), 0x820);
        assert_eq!(offset_of!(RegisterBlock, clock_i2s1), 0x824);
        assert_eq!(offset_of!(RegisterBlock, clock_gpio), 0x83C);
        assert_eq!(offset_of!(RegisterBlock, clock_uart0), 0x840);
        assert_eq!(offset_of!(RegisterBlock, clock_uart1), 0x844);
        assert_eq!(offset_of!(RegisterBlock, clock_uart2), 0x848);
        assert_eq!(offset_of!(RegisterBlock, clock_uart3), 0x84C);
        assert_eq!(offset_of!(RegisterBlock, clock_uart4), 0x850);
        assert_eq!(offset_of!(RegisterBlock, clock_uart5), 0x854);
        assert_eq!(offset_of!(RegisterBlock, clock_uart6), 0x858);
        assert_eq!(offset_of!(RegisterBlock, clock_uart7), 0x85C);
        assert_eq!(offset_of!(RegisterBlock, clock_ta_if), 0x870);
        assert_eq!(offset_of!(RegisterBlock, clock_edat_if), 0x874);
        assert_eq!(offset_of!(RegisterBlock, clock_bis_if), 0x878);
        assert_eq!(offset_of!(RegisterBlock, clock_sdfm), 0x87C);
        assert_eq!(offset_of!(RegisterBlock, clock_lcd), 0x880);
        assert_eq!(offset_of!(RegisterBlock, clock_lvds), 0x884);
        assert_eq!(offset_of!(RegisterBlock, clock_mipi_dsi), 0x888);
        assert_eq!(offset_of!(RegisterBlock, clock_dvp), 0x890);
        assert_eq!(offset_of!(RegisterBlock, clock_mipi_csi), 0x898);
        assert_eq!(offset_of!(RegisterBlock, clock_de), 0x8C0);
        assert_eq!(offset_of!(RegisterBlock, clock_ge), 0x8C4);
        assert_eq!(offset_of!(RegisterBlock, clock_ve), 0x8C8);
        assert_eq!(offset_of!(RegisterBlock, clock_sid), 0x904);
        assert_eq!(offset_of!(RegisterBlock, clock_rtc), 0x908);
        assert_eq!(offset_of!(RegisterBlock, clock_gtc), 0x90C);
        assert_eq!(offset_of!(RegisterBlock, clock_i2c0), 0x960);
        assert_eq!(offset_of!(RegisterBlock, clock_i2c1), 0x964);
        assert_eq!(offset_of!(RegisterBlock, clock_i2c2), 0x968);
        assert_eq!(offset_of!(RegisterBlock, clock_i2c3), 0x96C);
        assert_eq!(offset_of!(RegisterBlock, clock_can0), 0x980);
        assert_eq!(offset_of!(RegisterBlock, clock_can1), 0x984);
        assert_eq!(offset_of!(RegisterBlock, clock_pwm), 0x990);
        assert_eq!(offset_of!(RegisterBlock, clock_adcim), 0x9A0);
        assert_eq!(offset_of!(RegisterBlock, clock_gpai), 0x9A4);
        assert_eq!(offset_of!(RegisterBlock, clock_rtp), 0x9A8);
        assert_eq!(offset_of!(RegisterBlock, clock_ths), 0x9AC);
        assert_eq!(offset_of!(RegisterBlock, clock_cir), 0x9B0);
        assert_eq!(offset_of!(RegisterBlock, version), 0xFFC);
    }

    #[test]
    fn struct_pll_general_functions() {
        let mut val = PllGeneral(0x0);

        val = val.enable_pll_reg();
        assert!(val.is_pll_reg_enabled());
        assert_eq!(val.0, 0x8000_0000);

        val = val.disable_pll_reg();
        assert!(!val.is_pll_reg_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_pll_icp(0x1F);
        assert_eq!(val.pll_icp(), 0x1F);
        assert_eq!(val.0, 0x1F00_0000);

        val = PllGeneral(0x0).set_pll_output_sel(PllOutput::PllClk);
        assert_eq!(val.pll_output_sel(), PllOutput::PllClk);
        assert_eq!(val.0, 0x0010_0000);

        val = val.set_pll_output_sel(PllOutput::Osc24M);
        assert_eq!(val.pll_output_sel(), PllOutput::Osc24M);
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_facter_m();
        assert!(val.is_factor_m_enabled());
        assert_eq!(val.0, 0x0008_0000);

        val = val.disable_facter_m();
        assert!(!val.is_factor_m_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enabled_pll_out2sys();
        assert!(val.is_pll_out2sys_enabled());
        assert_eq!(val.0, 0x0004_0000);

        val = val.disable_pll_out2sys();
        assert!(!val.is_pll_out2sys_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = PllGeneral(0x0002_0000);
        assert!(val.is_pll_locked());

        val = PllGeneral(0x0).enable_pll();
        assert!(val.is_pll_enabled());
        assert_eq!(val.0, 0x0001_0000);

        val = val.disable_pll();
        assert!(!val.is_pll_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_factor_n(0xC7);
        assert_eq!(val.factor_n(), 0xC7);
        assert_eq!(val.0, 0x0000_C700);

        val = PllGeneral(0x0).set_factor_m(0x3);
        assert_eq!(val.factor_m(), 0x3);
        assert_eq!(val.0, 0x0000_0030);

        val = PllGeneral(0x0).set_factor_p(0x1);
        assert_eq!(val.factor_p(), 0x1);
        assert_eq!(val.0, 0x0000_0001);
    }

    test_should_panic!(
        (
            test_pll_general_set_pll_icp_panic,
            PllGeneral(0x0).set_pll_icp(0x20),
            "Value out of bounds for PLL_ICP (expected 0..=31)"
        ),
        (
            test_pll_general_set_factor_n_panic,
            PllGeneral(0x0).set_factor_n(200),
            "Value out of bounds for FACTOR_N (expected 14..=199)"
        ),
        (
            test_pll_general_set_factor_m_panic,
            PllGeneral(0x0).set_factor_m(0x4),
            "Value out of bounds for FACTOR_M (expected 0..=3)"
        ),
        (
            test_pll_general_set_factor_p_panic,
            PllGeneral(0x0).set_factor_p(0x2),
            "Value out of bounds for FACTOR_P (expected 0..=1)"
        )
    );

    #[test]
    fn struct_pll_int_config_functions() {
        let mut val = PllIntConfig(0x0);

        val = val.set_pll_lock_time(0x7);
        assert_eq!(val.pll_lock_time(), 0x7);
        assert_eq!(val.0, 0x7000_0000);

        val = PllIntConfig(0x0).set_pll_ivco(0x5);
        assert_eq!(val.pll_ivco(), 0x5);
        assert_eq!(val.0, 0x0500_0000);

        val = PllIntConfig(0x0).set_pll_vco_sel(0x3);
        assert_eq!(val.pll_vco_sel(), 0x3);
        assert_eq!(val.0, 0x0030_0000);

        val = PllIntConfig(0x0).set_pll_vco_rst(0x1);
        assert_eq!(val.pll_vco_rst(), 0x1);
        assert_eq!(val.0, 0x0008_0000);

        val = PllIntConfig(0x0).set_pll_vco_gain(0x4);
        assert_eq!(val.pll_vco_gain(), 0x4);
        assert_eq!(val.0, 0x0004_0000);

        val = PllIntConfig(0x0).set_pll_bint(0x7F);
        assert_eq!(val.pll_bint(), 0x7F);
        assert_eq!(val.0, 0x0000_7F00);

        val = PllIntConfig(0x0).set_ldo_bias_current(LdoBiasCurrent::TwouA);
        assert_eq!(val.ldo_bias_current(), LdoBiasCurrent::TwouA);
        assert_eq!(val.0, 0x0000_0200);

        val = val.set_ldo_bias_current(LdoBiasCurrent::FouruA);
        assert_eq!(val.ldo_bias_current(), LdoBiasCurrent::FouruA);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_bias_current_multiplier(BiasCurrentMultiplier::OneTimes);
        assert_eq!(
            val.bias_current_multiplier(),
            BiasCurrentMultiplier::OneTimes
        );
        assert_eq!(val.0, 0x0000_0100);

        val = val.set_bias_current_multiplier(BiasCurrentMultiplier::FoutTimes);
        assert_eq!(
            val.bias_current_multiplier(),
            BiasCurrentMultiplier::FoutTimes
        );
        assert_eq!(val.0, 0x0000_0000);

        val = PllIntConfig(0x0).set_pll_cint(0x7F);
        assert_eq!(val.pll_cint(), 0x7F);
        assert_eq!(val.0, 0x0000_007F);
    }

    test_should_panic!(
        (
            test_pll_int_config_set_pll_lock_time_panic,
            PllIntConfig(0x0).set_pll_lock_time(8),
            "Value out of bounds for PLL_LOCK_TIME (expected 0..=7)"
        ),
        (
            test_pll_int_config_set_pll_ivco_panic,
            PllIntConfig(0x0).set_pll_ivco(8),
            "Value out of bounds for PLL_IVCO (expected 0..=7)"
        ),
        (
            test_pll_int_config_set_pll_vco_sel_panic,
            PllIntConfig(0x0).set_pll_vco_sel(4),
            "Value out of bounds for PLL_VCO_SEL (expected 0..=3)"
        ),
        (
            test_pll_int_config_set_pll_vco_rst_panic,
            PllIntConfig(0x0).set_pll_vco_rst(2),
            "Value out of bounds for PLL_VCO_RST (expected 0..=1)"
        ),
        (
            test_pll_int_config_set_pll_vco_gain_panic,
            PllIntConfig(0x0).set_pll_vco_gain(8),
            "Value out of bounds for PLL_VCO_GAIN (expected 0..=7)"
        ),
        (
            test_pll_int_config_set_pll_bint_panic,
            PllIntConfig(0x0).set_pll_bint(128),
            "Value out of bounds for PLL_BINT (expected 0..=127)"
        ),
        (
            test_pll_int_config_set_pll_cint_panic,
            PllIntConfig(0x0).set_pll_cint(128),
            "Value out of bounds for PLL_CINT (expected 0..=127)"
        )
    );

    #[test]
    fn struct_pll_fra_config_functions() {
        let mut val = PllFraConfig(0x0);

        val = val.enable_dither();
        assert!(val.is_dither_enabled());
        assert_eq!(val.0, 0x0100_0000);

        val = val.disable_dither();
        assert!(!val.is_dither_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_fra_div();
        assert!(val.is_fra_div_enabled());
        assert_eq!(val.0, 0x0010_0000);

        val = val.disable_fra_div();
        assert!(!val.is_fra_div_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_fra_div(0x1FFFF);
        assert_eq!(val.fra_div(), 0x1FFFF);
        assert_eq!(val.0, 0x0001_FFFF);
    }

    test_should_panic!((
        test_pll_fra_config_set_fra_div_panic,
        PllFraConfig(0x0).set_fra_div(0x20000),
        "Value out of bounds for FRA_IN (expected 0..=0x1FFFF)"
    ));

    #[test]
    fn struct_pll_fra_sdm_functions() {
        let mut val = PllFraSdm(0x0);

        val = val.enable_sdm();
        assert!(val.is_sdm_enabled());
        assert_eq!(val.0, 0x8000_0000);

        val = val.disable_sdm();
        assert!(!val.is_sdm_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = PllFraSdm(0x0).set_sdm_mode(SdmMode::Dc1);
        assert_eq!(val.sdm_mode(), SdmMode::Dc1);
        assert_eq!(val.0, 0x2000_0000);

        val = val.set_sdm_mode(SdmMode::Triangular);
        assert_eq!(val.sdm_mode(), SdmMode::Triangular);
        assert_eq!(val.0, 0x4000_0000);

        val = val.set_sdm_mode(SdmMode::Dc0);
        assert_eq!(val.sdm_mode(), SdmMode::Dc0);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_sdm_step(0x1FF);
        assert_eq!(val.sdm_step(), 0x1FF);
        assert_eq!(val.0, 0x1FF0_0000);

        val = PllFraSdm(0x0).set_sdm_freq(SdmFreq::F32P5KHz);
        assert_eq!(val.sdm_freq(), SdmFreq::F32P5KHz);
        assert_eq!(val.0, 0x0004_0000);

        val = val.set_sdm_freq(SdmFreq::F33P0KHz);
        assert_eq!(val.sdm_freq(), SdmFreq::F33P0KHz);
        assert_eq!(val.0, 0x0006_0000);

        val = val.set_sdm_freq(SdmFreq::F32P0KHz);
        assert_eq!(val.sdm_freq(), SdmFreq::F32P0KHz);
        assert_eq!(val.0, 0x0002_0000);

        val = val.set_sdm_freq(SdmFreq::F31P5KHz);
        assert_eq!(val.sdm_freq(), SdmFreq::F31P5KHz);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_sdm_bottom(0x1FFFF);
        assert_eq!(val.sdm_bottom(), 0x1FFFF);
        assert_eq!(val.0, 0x0001_FFFF);
    }

    test_should_panic!(
        (
            test_pll_fra_sdm_set_sdm_step_panic,
            PllFraSdm(0x0).set_sdm_step(512),
            "Value out of bounds for SDM_STEP (expected 0..=511)"
        ),
        (
            test_pll_fra_sdm_set_sdm_bottom_panic,
            PllFraSdm(0x0).set_sdm_bottom(0x20000),
            "Value out of bounds for SDM_BOT (expected 0..=0x1FFFF)"
        )
    );

    #[test]
    fn struct_pll_common_functions() {
        let mut val = PllCommon(0x0);

        val = val.enable_circuit();
        assert!(val.is_circuit_enabled());
        assert_eq!(val.0, 0x8000_0000);

        val = val.disable_circuit();
        assert!(!val.is_circuit_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_ldo_voltage(LdoVoltage::U0P9V);
        assert_eq!(val.ldo_voltage(), LdoVoltage::U0P9V);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_ldo_voltage(LdoVoltage::U0P95V);
        assert_eq!(val.ldo_voltage(), LdoVoltage::U0P95V);
        assert_eq!(val.0, 0x0000_0002);

        val = val.set_ldo_voltage(LdoVoltage::U1P0V);
        assert_eq!(val.ldo_voltage(), LdoVoltage::U1P0V);
        assert_eq!(val.0, 0x0000_0004);

        val = val.set_ldo_voltage(LdoVoltage::U1P05V);
        assert_eq!(val.ldo_voltage(), LdoVoltage::U1P05V);
        assert_eq!(val.0, 0x0000_0006);

        val = val.set_ldo_voltage(LdoVoltage::U1P1V);
        assert_eq!(val.ldo_voltage(), LdoVoltage::U1P1V);
        assert_eq!(val.0, 0x0000_0008);

        val = val.set_ldo_voltage(LdoVoltage::U1P15V);
        assert_eq!(val.ldo_voltage(), LdoVoltage::U1P15V);
        assert_eq!(val.0, 0x0000_000A);

        val = val.set_ldo_voltage(LdoVoltage::U1P2V);
        assert_eq!(val.ldo_voltage(), LdoVoltage::U1P2V);
        assert_eq!(val.0, 0x0000_000C);

        val = val.set_ldo_voltage(LdoVoltage::U1P25V);
        assert_eq!(val.ldo_voltage(), LdoVoltage::U1P25V);
        assert_eq!(val.0, 0x0000_000E);

        val = PllCommon(0x0).enable_ldo();
        assert!(val.is_ldo_enabled());
        assert_eq!(val.0, 0x0000_0001);

        val = val.disable_ldo();
        assert!(!val.is_ldo_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_pll_input_functions() {
        let mut val = PllInput(0x0);

        val = val.set_xtal_drive(0x3);
        assert_eq!(val.xtal_drive(), 0x3);
        assert_eq!(val.0, 0xC000_0000);

        val = PllInput(0x0).enable_xtal();
        assert!(val.is_xtal_enabled());
        assert_eq!(val.0, 0x2000_0000);

        val = val.disable_xtal();
        assert!(!val.is_xtal_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = PllInput(0x0).enable_xtal_osc();
        assert!(val.is_xtal_osc_enabled());
        assert_eq!(val.0, 0x1000_0000);

        val = val.disable_xtal_osc();
        assert!(!val.is_xtal_osc_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = PllInput(0x0).set_osc_freq(0x7F);
        assert_eq!(val.osc_freq(), 0x7F);
        assert_eq!(val.0, 0x0000_7F00);

        val = PllInput(0x0).set_osc_out_freq(OscOutFreq::F18MHz);
        assert_eq!(val.osc_out_freq(), OscOutFreq::F18MHz);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_osc_out_freq(OscOutFreq::F20MHz);
        assert_eq!(val.osc_out_freq(), OscOutFreq::F20MHz);
        assert_eq!(val.0, 0x0000_0010);

        val = val.set_osc_out_freq(OscOutFreq::F22MHz);
        assert_eq!(val.osc_out_freq(), OscOutFreq::F22MHz);
        assert_eq!(val.0, 0x0000_0020);

        val = val.set_osc_out_freq(OscOutFreq::F24MHz);
        assert_eq!(val.osc_out_freq(), OscOutFreq::F24MHz);
        assert_eq!(val.0, 0x0000_0030);

        val = val.set_osc_out_freq(OscOutFreq::F26MHz);
        assert_eq!(val.osc_out_freq(), OscOutFreq::F26MHz);
        assert_eq!(val.0, 0x0000_0040);

        val = val.set_osc_out_freq(OscOutFreq::F28MHz);
        assert_eq!(val.osc_out_freq(), OscOutFreq::F28MHz);
        assert_eq!(val.0, 0x0000_0050);

        val = val.set_osc_out_freq(OscOutFreq::F30MHz);
        assert_eq!(val.osc_out_freq(), OscOutFreq::F30MHz);
        assert_eq!(val.0, 0x0000_0060);

        val = val.set_osc_out_freq(OscOutFreq::F32MHz);
        assert_eq!(val.osc_out_freq(), OscOutFreq::F32MHz);
        assert_eq!(val.0, 0x0000_0070);

        val = PllInput(0x0).enable_osc24m();
        assert!(val.is_osc24m_enabled());
        assert_eq!(val.0, 0x0000_0002);

        val = val.disable_osc24m();
        assert!(!val.is_osc24m_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = PllInput(0x0).set_clk_sel(ClockSelection::Xtal);
        assert_eq!(val.clk_sel(), ClockSelection::Xtal);
        assert_eq!(val.0, 0x0000_0001);

        val = val.set_clk_sel(ClockSelection::Osc24M);
        assert_eq!(val.clk_sel(), ClockSelection::Osc24M);
        assert_eq!(val.0, 0x0000_0000);
    }

    test_should_panic!(
        (
            test_pll_input_set_xtal_drive_panic,
            PllInput(0x0).set_xtal_drive(4),
            "Value out of bounds for XTAL_GM24M (expected 0..=3)"
        ),
        (
            test_pll_input_set_osc_freq_panic,
            PllInput(0x0).set_osc_freq(128),
            "Value out of bounds for OSC_OUT_TR24M (expected 0..=127)"
        )
    );

    #[test]
    fn struct_clock_output_functions() {
        let mut val = ClockOutput(0x0);

        val = val.enable_clk_out();
        assert!(val.is_clk_out());
        assert_eq!(val.0, 0x0001_0000);

        val = val.disable_clk_out();
        assert!(!val.is_clk_out());
        assert_eq!(val.0, 0x0000_0000);

        val = ClockOutput(0x0).set_clk_src_sel(ClockSourceSelection::PllInt1);
        assert_eq!(val.clk_src_sel(), ClockSourceSelection::PllInt1);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_clk_src_sel(ClockSourceSelection::PllFra2);
        assert_eq!(val.clk_src_sel(), ClockSourceSelection::PllFra2);
        assert_eq!(val.0, 0x0000_2000);

        val = ClockOutput(0x0).set_clk_div_n(0xFF);
        assert_eq!(val.clk_div_n(), 0xFF);
        assert_eq!(val.0, 0x0000_00FF);
    }

    #[test]
    fn struct_clk_on_pll_intx_functions() {
        let mut val = ClkOnPllIntx(0x0);

        val = val.set_bus_clk(ClockSelectionIntx::Clk24M);
        assert_eq!(val.bus_clk(), ClockSelectionIntx::Clk24M);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_bus_clk(ClockSelectionIntx::PllIntx);
        assert_eq!(val.bus_clk(), ClockSelectionIntx::PllIntx);
        assert_eq!(val.0, 0x0000_0100);

        val = ClkOnPllIntx(0x0).set_bus_clk_div(0x1F);
        assert_eq!(val.bus_clk_div(), 0x1F);
        assert_eq!(val.0, 0x0000_001F);
    }

    test_should_panic!((
        test_clk_on_pll_intx_set_bus_clk_div_panic,
        ClkOnPllIntx(0x0).set_bus_clk_div(32),
        "Value out of bounds for BUS_CLK_DIV (expected 0..=31)"
    ));

    #[test]
    fn struct_cpu_clock_functions() {
        let mut val = CpuClock(0x0);

        val = val.set_cpu_axi_clk_div(0x1F);
        assert_eq!(val.cpu_axi_clk_div(), 0x1F);
        assert_eq!(val.0, 0x001F_0000);

        val = CpuClock(0x0).set_bus_clk(ClockSelectionIntx::Clk24M);
        assert_eq!(val.bus_clk(), ClockSelectionIntx::Clk24M);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_bus_clk(ClockSelectionIntx::PllIntx);
        assert_eq!(val.bus_clk(), ClockSelectionIntx::PllIntx);
        assert_eq!(val.0, 0x0000_0100);

        val = CpuClock(0x0).set_clk_div(0x1F);
        assert_eq!(val.clk_div(), 0x1F);
        assert_eq!(val.0, 0x0000_001F);
    }

    test_should_panic!(
        (
            test_cpu_clock_set_cpu_axi_clk_div_panic,
            CpuClock(0x0).set_cpu_axi_clk_div(32),
            "Value out of bounds for CPU_AXI_CLK_DIV (expected 0..=31)"
        ),
        (
            test_cpu_clock_set_clk_div_panic,
            CpuClock(0x0).set_clk_div(32),
            "Value out of bounds for CLK_DIV (expected 0..=31)"
        )
    );

    #[test]
    fn struct_wdog_clock_functions() {
        let mut val = WdogClock(0x0);

        val = val.enable_wdog_clk_rw();
        assert!(val.is_wdog_clk_rw_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.disable_wdog_clk_rw();
        assert!(!val.is_wdog_clk_rw_enabled());
        assert_eq!(val.0, 0x1000_0000);

        val = WdogClock(0x0).enable_wdog_rst();
        assert!(val.is_wdog_rst_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.disable_wdog_rst();
        assert!(!val.is_wdog_rst_enabled());
        assert_eq!(val.0, 0x0000_2000);

        val = WdogClock(0x0).enable_bus_clk();
        assert!(val.is_bus_clk_enabled());
        assert_eq!(val.0, 0x0000_1000);

        val = val.disable_bus_clk();
        assert!(!val.is_bus_clk_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_module_clk();
        assert!(val.is_module_clk_enabled());
        assert_eq!(val.0, 0x0000_0100);

        val = val.disable_module_clk();
        assert!(!val.is_module_clk_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_ddr_clock_functions() {
        let mut val = DdrClock(0x0);

        val = val.enable_fifo_reset();
        assert!(val.is_fifo_reset_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.disable_fifo_reset();
        assert!(!val.is_fifo_reset_enabled());
        assert_eq!(val.0, 0x0010_0000);

        val = DdrClock(0x0).enable_axi_bus_reset();
        assert!(val.is_axi_bus_reset_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.disable_axi_bus_reset();
        assert!(!val.is_axi_bus_reset_enabled());
        assert_eq!(val.0, 0x0008_0000);

        val = DdrClock(0x0).enable_axi_clk();
        assert!(val.is_axi_clk_enabled());
        assert_eq!(val.0, 0x0004_0000);

        val = val.disable_axi_clk();
        assert!(!val.is_axi_clk_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_ahb_bus_reset();
        assert!(val.is_ahb_bus_reset_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.disable_ahb_bus_reset();
        assert!(!val.is_ahb_bus_reset_enabled());
        assert_eq!(val.0, 0x0002_0000);

        val = DdrClock(0x0).enable_ahb_clk();
        assert!(val.is_ahb_clk_enabled());
        assert_eq!(val.0, 0x0001_0000);

        val = val.disable_ahb_clk();
        assert!(!val.is_ahb_clk_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_phy_apb_bus_reset();
        assert!(val.is_phy_apb_bus_reset_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.disable_phy_apb_bus_reset();
        assert!(!val.is_phy_apb_bus_reset_enabled());
        assert_eq!(val.0, 0x0000_8000);

        val = DdrClock(0x0).enable_phy_apb_clk();
        assert!(val.is_phy_apb_clk_enabled());
        assert_eq!(val.0, 0x0000_4000);

        val = val.disable_phy_apb_clk();
        assert!(!val.is_phy_apb_clk_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_phy_reset();
        assert!(val.is_phy_reset_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.disable_phy_reset();
        assert!(!val.is_phy_reset_enabled());
        assert_eq!(val.0, 0x0000_2000);

        val = DdrClock(0x0).enable_phy_module_clk();
        assert!(val.is_phy_module_clk_enabled());
        assert_eq!(val.0, 0x0000_1000);

        val = val.disable_phy_module_clk();
        assert!(!val.is_phy_module_clk_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_ctrl_apb_reset();
        assert!(val.is_ctrl_apb_reset_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.disable_ctrl_apb_reset();
        assert!(!val.is_ctrl_apb_reset_enabled());
        assert_eq!(val.0, 0x0000_0800);

        val = DdrClock(0x0).enable_ctrl_apb_clk();
        assert!(val.is_ctrl_apb_clk_enabled());
        assert_eq!(val.0, 0x0000_0400);

        val = val.disable_ctrl_apb_clk();
        assert!(!val.is_ctrl_apb_clk_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_ctrl_reset();
        assert!(val.is_ctrl_reset_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.disable_ctrl_reset();
        assert!(!val.is_ctrl_reset_enabled());
        assert_eq!(val.0, 0x0000_0200);

        val = DdrClock(0x0).enable_ctrl_clk();
        assert!(val.is_ctrl_clk_enabled());
        assert_eq!(val.0, 0x0000_0100);

        val = val.disable_ctrl_clk();
        assert!(!val.is_ctrl_clk_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_module_clk_div(0x1F);
        assert_eq!(val.module_clk_div(), 0x1F);
        assert_eq!(val.0, 0x0000_001F);
    }

    test_should_panic!((
        test_ddr_clock_set_module_clk_div_panic,
        DdrClock(0x0).set_module_clk_div(32),
        "Value out of bounds for MOD_CLK_DIV (expected 0..=31)"
    ));

    #[test]
    fn struct_disp_clock_functions() {
        let mut val = DispClock(0x0);

        val = val.set_pix_clk_div(PixelClockSelection::CalculateByDivs);
        assert_eq!(val.pix_clk_div(), PixelClockSelection::CalculateByDivs);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_pix_clk_div(PixelClockSelection::FixDiv);
        assert_eq!(val.pix_clk_div(), PixelClockSelection::FixDiv);
        assert_eq!(val.0, 0x0000_1000);

        val = DispClock(0x0).set_pix_clk_div_l(0x3);
        assert_eq!(val.pix_clk_div_l(), 0x3);
        assert_eq!(val.0, 0x0000_0C00);

        val = DispClock(0x0).set_pix_clk_div_m(0x1F);
        assert_eq!(val.pix_clk_div_m(), 0x1F);
        assert_eq!(val.0, 0x0000_01F0);

        val = DispClock(0x0).set_serial_clk_div_n(0x7);
        assert_eq!(val.serial_clk_div_n(), 0x7);
        assert_eq!(val.0, 0x0000_0007);
    }

    test_should_panic!(
        (
            test_disp_clock_set_pix_clk_div_l_panic,
            DispClock(0x0).set_pix_clk_div_l(4),
            "Value out of bounds for PIXCLK_DIV_L (expected 0..=3)"
        ),
        (
            test_disp_clock_set_pix_clk_div_m_panic,
            DispClock(0x0).set_pix_clk_div_m(32),
            "Value out of bounds for PIXCLK_DIV_M (expected 0..=31)"
        ),
        (
            test_disp_clock_set_serial_clk_div_n_panic,
            DispClock(0x0).set_serial_clk_div_n(8),
            "Value out of bounds for SCLK_DIV_N (expected 0..=7)"
        )
    );

    #[test]
    fn struct_audio_serial_clock_functions() {
        let mut val = AudioSerialClock(0x0).set_serial_clk_div(SerialClockDivider::Div49);
        assert_eq!(val.serial_clk_div(), SerialClockDivider::Div49);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_serial_clk_div(SerialClockDivider::Div53);
        assert_eq!(val.serial_clk_div(), SerialClockDivider::Div53);
        assert_eq!(val.0, 0x0000_0001);
    }

    #[test]
    fn struct_simple_module0_clock_funcions() {
        let mut val = SimpleModule0Clock(0x0);

        val = val.set_module_clk_div(0x1F);
        assert_eq!(val.module_clk_div(), 0x1F);
        assert_eq!(val.0, 0x0000_001F);
    }

    test_should_panic!((
        test_simple_module0_clock_set_module_clk_div_panic,
        SimpleModule0Clock(0x0).set_module_clk_div(32),
        "Value out of bounds for MOD_CLK_DIV (expected 0..=31)"
    ));

    #[test]
    fn struct_simple_module1_clock_funcions() {
        let mut val = SimpleModule1Clock(0x0);

        val = val.enable_bus_clk();
        assert!(val.is_bus_clk_enabled());
        assert_eq!(val.0, 0x0000_1000);

        val = val.disable_bus_clk();
        assert!(!val.is_bus_clk_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_simple_module2_clock_funcions() {
        let mut val = SimpleModule2Clock(0x0);

        val = val.enable_module_reset();
        assert!(val.is_module_reset_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.disable_module_reset();
        assert!(!val.is_module_reset_enabled());
        assert_eq!(val.0, 0x0000_2000);

        val = SimpleModule2Clock(0x0).enable_bus_clk();
        assert!(val.is_bus_clk_enabled());
        assert_eq!(val.0, 0x0000_1000);

        val = val.disable_bus_clk();
        assert!(!val.is_bus_clk_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_simple_module3_clock_funcions() {
        let mut val = SimpleModule3Clock(0x0);

        val = val.enable_bus_clk();
        assert!(val.is_bus_clk_enabled());
        assert_eq!(val.0, 0x0000_1000);

        val = val.disable_bus_clk();
        assert!(!val.is_bus_clk_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_module_clk();
        assert!(val.is_module_clk_enabled());
        assert_eq!(val.0, 0x0000_0100);

        val = val.disable_module_clk();
        assert!(!val.is_module_clk_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_simple_module4_clock_funcions() {
        let mut val = SimpleModule4Clock(0x0);

        val = val.enable_module_reset();
        assert!(val.is_module_reset_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.disable_module_reset();
        assert!(!val.is_module_reset_enabled());
        assert_eq!(val.0, 0x0000_2000);

        val = SimpleModule4Clock(0x0).enable_module_clk();
        assert!(val.is_module_clk_enabled());
        assert_eq!(val.0, 0x0000_0100);

        val = val.disable_module_clk();
        assert!(!val.is_module_clk_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_simple_module5_clock_funcions() {
        let mut val = SimpleModule5Clock(0x0);

        val = val.enable_module_reset();
        assert!(val.is_module_reset_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.disable_module_reset();
        assert!(!val.is_module_reset_enabled());
        assert_eq!(val.0, 0x0000_2000);

        val = SimpleModule5Clock(0x0).enable_bus_clk();
        assert!(val.is_bus_clk_enabled());
        assert_eq!(val.0, 0x0000_1000);

        val = val.disable_bus_clk();
        assert!(!val.is_bus_clk_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_module_clk();
        assert!(val.is_module_clk_enabled());
        assert_eq!(val.0, 0x0000_0100);

        val = val.disable_module_clk();
        assert!(!val.is_module_clk_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_simple_module6_clock_funcions() {
        let mut val = SimpleModule6Clock(0x0);

        val = val.enable_module_reset();
        assert!(val.is_module_reset_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.disable_module_reset();
        assert!(!val.is_module_reset_enabled());
        assert_eq!(val.0, 0x0000_2000);

        val = SimpleModule6Clock(0x0).enable_module_clk();
        assert!(val.is_module_clk_enabled());
        assert_eq!(val.0, 0x0000_0100);

        val = val.disable_module_clk();
        assert!(!val.is_module_clk_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_module_clk_div(0x1F);
        assert_eq!(val.module_clk_div(), 0x1F);
        assert_eq!(val.0, 0x0000_001F);
    }

    test_should_panic!((
        test_simple_module6_clock_set_module_clk_div_panic,
        SimpleModule6Clock(0x0).set_module_clk_div(32),
        "Value out of bounds for MOD_CLK_DIV (expected 0..=31)"
    ));

    #[test]
    fn struct_normal_module_clock_functions() {
        let mut val = NormalModuleClock(0x0);

        val = val.enable_module_reset();
        assert!(val.is_module_reset_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.disable_module_reset();
        assert!(!val.is_module_reset_enabled());
        assert_eq!(val.0, 0x0000_2000);

        val = NormalModuleClock(0x0).enable_bus_clk();
        assert!(val.is_bus_clk_enabled());
        assert_eq!(val.0, 0x0000_1000);

        val = val.disable_bus_clk();
        assert!(!val.is_bus_clk_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_module_clk();
        assert!(val.is_module_clk_enabled());
        assert_eq!(val.0, 0x0000_0100);

        val = val.disable_module_clk();
        assert!(!val.is_module_clk_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_module_clk_div(0x1F);
        assert_eq!(val.module_clk_div(), 0x1F);
        assert_eq!(val.0, 0x0000_001F);
    }

    test_should_panic!((
        test_normal_module_clock_set_module_clk_div_panic,
        NormalModuleClock(0x0).set_module_clk_div(32),
        "Value out of bounds for MOD_CLK_DIV (expected 0..=31)"
    ));
}
