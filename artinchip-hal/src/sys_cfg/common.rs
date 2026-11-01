//! Register bitfields shared across SYSCFG hardware revisions.

/// Interrupt request control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IrqControl(u32);

impl IrqControl {
    const CMP_RST_EN: u32 = 0x1 << 31;
    const CMP_IRQ_EN: u32 = 0x1;

    /// Enable comparator reset (`CMP_RST_EN`).
    #[doc(alias = "CMP_RST_EN")]
    #[inline]
    pub const fn enable_cmp_rst(self) -> Self {
        Self(self.0 | Self::CMP_RST_EN)
    }
    /// Disable comparator reset.
    #[inline]
    pub const fn disable_cmp_rst(self) -> Self {
        Self(self.0 & !Self::CMP_RST_EN)
    }
    /// Check if comparator reset is enabled.
    #[inline]
    pub const fn is_cmp_rst_enabled(self) -> bool {
        self.0 & Self::CMP_RST_EN != 0
    }
    /// Enable comparator interrupt (`CMP_IRQ_EN`).
    #[doc(alias = "CMP_IRQ_EN")]
    #[inline]
    pub const fn enable_cmp_irq(self) -> Self {
        Self(self.0 | Self::CMP_IRQ_EN)
    }
    /// Disable comparator interrupt.
    #[inline]
    pub const fn disable_cmp_irq(self) -> Self {
        Self(self.0 & !Self::CMP_IRQ_EN)
    }
    /// Check if comparator interrupt is enabled.
    #[inline]
    pub const fn is_cmp_irq_enabled(self) -> bool {
        self.0 & Self::CMP_IRQ_EN != 0
    }
}

/// Interrupt request status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IrqStatus(u32);

impl IrqStatus {
    const CMP_IRQ_STA: u32 = 0x1;

    /// Check if comparator interrupt is pending (`CMP_IRQ_STA`).
    ///
    /// When compare mode is low voltage detection (CMP_MODE=0):
    /// - 0: Pin input voltage has always been higher than comparison voltage.
    /// - 1: Pin input voltage has dropped below comparison voltage.
    ///
    /// When compare mode is high voltage detection (CMP_MODE=1):
    /// - 0: Pin input voltage has always been lower than comparison voltage.
    /// - 1: Pin input voltage has risen above comparison voltage.
    #[doc(alias = "CMP_IRQ_STA")]
    #[inline]
    pub const fn is_cmp_irq_pending(self) -> bool {
        self.0 & Self::CMP_IRQ_STA != 0
    }
    /// Clear comparator interrupt.
    #[inline]
    pub const fn clear_cmp_irq(self) -> Self {
        Self(self.0 | Self::CMP_IRQ_STA)
    }
}

/// Compare mode selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CmpMode {
    /// Low voltage detection.
    LowVoltage,
    /// High voltage detection.
    HighVoltage,
}

/// Compare voltage selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CmpVoltage {
    /// 0.90 v.
    V0_90,
    /// 0.85 v.
    V0_85,
    /// 0.80 v.
    V0_80,
    /// 0.75 v.
    V0_75,
    /// 0.70 v.
    V0_70,
    /// 0.65 v.
    V0_65,
    /// 0.60 v.
    V0_60,
    /// 0.55 v.
    V0_55,
}

/// Compare configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CompareConfig(u32);

impl CompareConfig {
    const CMP_DB: u32 = 0xFF << 24;
    const CMP_MODE: u32 = 0x1 << 5;
    const CMP_EN: u32 = 0x1 << 4;
    const CMP_SEL: u32 = 0x7;

    /// Set compare debounce value (`CMP_DB`).
    ///
    /// The debounce circuit uses APB bus clock for debouncing.
    /// Debounce period = (CMP_DB + 1) * Tpclk, default Tpclk is 10 ns.
    #[doc(alias = "CMP_DB")]
    #[inline]
    pub const fn set_cmp_db(self, val: u8) -> Self {
        Self((self.0 & !Self::CMP_DB) | (Self::CMP_DB & ((val as u32) << 24)))
    }
    /// Get compare debounce value.
    #[inline]
    pub const fn cmp_db(self) -> u8 {
        ((self.0 & Self::CMP_DB) >> 24) as u8
    }
    /// Set compare mode (`CMP_MODE`).
    #[doc(alias = "CMP_MODE")]
    #[inline]
    pub const fn set_cmp_mode(self, mode: CmpMode) -> Self {
        Self((self.0 & !Self::CMP_MODE) | (Self::CMP_MODE & ((mode as u32) << 5)))
    }
    /// Get compare mode.
    #[inline]
    pub const fn cmp_mode(self) -> CmpMode {
        match (self.0 & Self::CMP_MODE) >> 5 {
            0 => CmpMode::LowVoltage,
            1 => CmpMode::HighVoltage,
            _ => unreachable!(),
        }
    }
    /// Enable compare (`CMP_EN`).
    #[doc(alias = "CMP_EN")]
    #[inline]
    pub const fn enable_cmp(self) -> Self {
        Self(self.0 | Self::CMP_EN)
    }
    /// Disable compare.
    #[inline]
    pub const fn disable_cmp(self) -> Self {
        Self(self.0 & !Self::CMP_EN)
    }
    /// Check if compare is enabled.
    #[inline]
    pub const fn is_cmp_enabled(self) -> bool {
        self.0 & Self::CMP_EN != 0
    }
    /// Set compare voltage selection (`CMP_SEL`).
    #[doc(alias = "CMP_SEL")]
    #[inline]
    pub const fn set_cmp_voltage_sel(self, vol: CmpVoltage) -> Self {
        Self((self.0 & !Self::CMP_SEL) | (Self::CMP_SEL & (vol as u32)))
    }
    /// Get compare voltage selection.
    #[inline]
    pub const fn cmp_voltage_sel(self) -> CmpVoltage {
        match self.0 & Self::CMP_SEL {
            0 => CmpVoltage::V0_90,
            1 => CmpVoltage::V0_85,
            2 => CmpVoltage::V0_80,
            3 => CmpVoltage::V0_75,
            4 => CmpVoltage::V0_70,
            5 => CmpVoltage::V0_65,
            6 => CmpVoltage::V0_60,
            7 => CmpVoltage::V0_55,
            _ => unreachable!(),
        }
    }
}

/// External calibration resistor register (DDR_REXT, USB0_REXT, USB1_REXT).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ExternalResistor(u32);

/// Legacy alias for [ExternalResistor].
pub type Usb0RExt = ExternalResistor;

impl ExternalResistor {
    const RES_CAL_EN: u32 = 0x1 << 8;
    const RES_CAL_VAL: u32 = 0xFF;

    /// Enable resistor calibration (`RES_CAL_EN`).
    #[doc(alias = "RES_CAL_EN")]
    #[inline]
    pub const fn enable_res_cal(self) -> Self {
        Self(self.0 | Self::RES_CAL_EN)
    }
    /// Disable resistor calibration.
    #[inline]
    pub const fn disable_res_cal(self) -> Self {
        Self(self.0 & !Self::RES_CAL_EN)
    }
    /// Check if resistor calibration is enabled.
    #[inline]
    pub const fn is_res_cal_enabled(self) -> bool {
        self.0 & Self::RES_CAL_EN != 0
    }
    /// Set resistor calibration value (`RES_CAL_VAL`).
    ///
    /// Adjust this value so that the calibrated resistance reaches the target value of 200 ohms for USB.
    #[doc(alias = "RES_CAL_VAL")]
    #[inline]
    pub const fn set_res_cal_val(self, val: u8) -> Self {
        Self((self.0 & !Self::RES_CAL_VAL) | (Self::RES_CAL_VAL & (val as u32)))
    }
    /// Get resistor calibration value.
    #[inline]
    pub const fn res_cal_val(self) -> u8 {
        (self.0 & Self::RES_CAL_VAL) as u8
    }
}

/// PSEN ring oscillator selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RoSel {
    /// RVT_40 cell.
    Rvt40Cell = 1,
    /// LVT_40 cell.
    Lvt40Cell = 2,
    /// ULVT_40 cell.
    Ulvt40Cell = 3,
    /// RVT_50 cell.
    Rvt50Cell = 5,
    /// LVT_50 cell.
    Lvt50Cell = 6,
    /// ULVT_50 cell.
    Ulvt50Cell = 7,
}

/// Process sensor configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PsenConfig(u32);

impl PsenConfig {
    const CNT_TIME: u32 = 0xFFFF << 16;
    const RO_SEL: u32 = 0x7 << 1;
    const PSEN_START: u32 = 0x1;

    /// Set count time (`CNT_TIME`).
    ///
    /// Unit: APB bus clock cycles.
    #[doc(alias = "CNT_TIME")]
    #[inline]
    pub const fn set_cnt_time(self, time: u16) -> Self {
        Self((self.0 & !Self::CNT_TIME) | (Self::CNT_TIME & ((time as u32) << 16)))
    }
    /// Get count time.
    #[inline]
    pub const fn cnt_time(self) -> u16 {
        ((self.0 & Self::CNT_TIME) >> 16) as u16
    }
    /// Set ring oscillator selection (`RO_SEL`).
    #[doc(alias = "RO_SEL")]
    #[inline]
    pub const fn set_ro_sel(self, sel: RoSel) -> Self {
        Self((self.0 & !Self::RO_SEL) | (Self::RO_SEL & ((sel as u32) << 1)))
    }
    /// Get ring oscillator selection.
    #[inline]
    pub const fn ro_sel(self) -> RoSel {
        match (self.0 & Self::RO_SEL) >> 1 {
            1 => RoSel::Rvt40Cell,
            2 => RoSel::Lvt40Cell,
            3 => RoSel::Ulvt40Cell,
            5 => RoSel::Rvt50Cell,
            6 => RoSel::Lvt50Cell,
            7 => RoSel::Ulvt50Cell,
            _ => unreachable!(),
        }
    }
    /// Start psen test bit (`PSEN_START`).
    ///
    /// After PSEN test ends, this bit is automatically cleared.
    #[doc(alias = "PSEN_START")]
    #[inline]
    pub const fn set_psen_start(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::PSEN_START)
        } else {
            Self(self.0 & !Self::PSEN_START)
        }
    }
    /// Get psen test bit.
    #[inline]
    pub const fn psen_start(self) -> bool {
        self.0 & Self::PSEN_START != 0
    }
}

/// Process sensor counter value register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PsenCntVal(u32);

impl PsenCntVal {
    const CNT_VAL: u32 = 0xFFFF;

    /// Get counter value (`CNT_VAL`).
    #[doc(alias = "CNT_VAL")]
    #[inline]
    pub const fn cnt_val(self) -> u16 {
        (self.0 & Self::CNT_VAL) as u16
    }
}

/// GE sram parameter register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct GeSramParam(u32);

impl GeSramParam {
    const SRAM_PAR: u32 = 0xFFFF;

    /// Set sram parameter (`SRAM_PAR`).
    #[doc(alias = "SRAM_PAR")]
    #[inline]
    pub const fn set_sram_par(self, val: u16) -> Self {
        Self((self.0 & !Self::SRAM_PAR) | (Self::SRAM_PAR & (val as u32)))
    }
    /// Get SRAM parameter.
    #[inline]
    pub const fn sram_par(self) -> u16 {
        (self.0 & Self::SRAM_PAR) as u16
    }
}

/// SRAM clock configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SramClkConfig(u32);

impl SramClkConfig {
    const XSPI: u32 = 0x1 << 17;
    const SDFM: u32 = 0x1 << 16;
    const AUDIO: u32 = 0x1 << 15;
    const MIPI: u32 = 0x1 << 13;
    const SPI: u32 = 0x1 << 12;
    const USB: u32 = 0x1 << 11;
    const SD: u32 = 0x1 << 9;
    const UART: u32 = 0x1 << 10;
    const CE: u32 = 0x1 << 8;
    const DE: u32 = 0x1 << 7;
    const GE: u32 = 0x1 << 6;
    const VE: u32 = 0x1 << 5;
    const DVP: u32 = 0x1 << 4;
    const GMAC: u32 = 0x1 << 3;
    const DMA: u32 = 0x1 << 2;
    const DDR: u32 = 0x1 << 1;
    const SYS: u32 = 0x1;

    /// Enable system sram clock ungating (`SYS`).
    #[doc(alias = "SYS")]
    #[inline]
    pub const fn enable_sys(self) -> Self {
        Self(self.0 | Self::SYS)
    }
    /// Disable system sram clock ungating.
    #[inline]
    pub const fn disable_sys(self) -> Self {
        Self(self.0 & !Self::SYS)
    }
    /// Check if system sram clock ungating is enabled.
    #[inline]
    pub const fn is_sys_enabled(self) -> bool {
        self.0 & Self::SYS != 0
    }
    /// Enable ddr sram clock ungating (`DDR`).
    #[doc(alias = "DDR")]
    #[inline]
    pub const fn enable_ddr(self) -> Self {
        Self(self.0 | Self::DDR)
    }
    /// Disable ddr sram clock ungating.
    #[inline]
    pub const fn disable_ddr(self) -> Self {
        Self(self.0 & !Self::DDR)
    }
    /// Check if ddr sram clock ungating is enabled.
    #[inline]
    pub const fn is_ddr_enabled(self) -> bool {
        self.0 & Self::DDR != 0
    }
    /// Enable dma sram clock ungating (`DMA`).
    #[doc(alias = "DMA")]
    #[inline]
    pub const fn enable_dma(self) -> Self {
        Self(self.0 | Self::DMA)
    }
    /// Disable dma sram clock ungating.
    #[inline]
    pub const fn disable_dma(self) -> Self {
        Self(self.0 & !Self::DMA)
    }
    /// Check if dma sram clock ungating is enabled.
    #[inline]
    pub const fn is_dma_enabled(self) -> bool {
        self.0 & Self::DMA != 0
    }
    /// Enable gmac sram clock ungating (`GMAC`).
    #[doc(alias = "GMAC")]
    #[inline]
    pub const fn enable_gmac(self) -> Self {
        Self(self.0 | Self::GMAC)
    }
    /// Disable gmac sram clock ungating.
    #[inline]
    pub const fn disable_gmac(self) -> Self {
        Self(self.0 & !Self::GMAC)
    }
    /// Check if gmac sram clock ungating is enabled.
    #[inline]
    pub const fn is_gmac_enabled(self) -> bool {
        self.0 & Self::GMAC != 0
    }
    /// Enable dvp sram clock ungating (`DVP`).
    #[doc(alias = "DVP")]
    #[inline]
    pub const fn enable_dvp(self) -> Self {
        Self(self.0 | Self::DVP)
    }
    /// Disable dvp sram clock ungating.
    #[inline]
    pub const fn disable_dvp(self) -> Self {
        Self(self.0 & !Self::DVP)
    }
    /// Check if dvp sram clock ungating is enabled.
    #[inline]
    pub const fn is_dvp_enabled(self) -> bool {
        self.0 & Self::DVP != 0
    }
    /// Enable ve sram clock ungating (`VE`).
    #[doc(alias = "VE")]
    #[inline]
    pub const fn enable_ve(self) -> Self {
        Self(self.0 | Self::VE)
    }
    /// Disable ve sram clock ungating.
    #[inline]
    pub const fn disable_ve(self) -> Self {
        Self(self.0 & !Self::VE)
    }
    /// Check if ve sram clock ungating is enabled.
    #[inline]
    pub const fn is_ve_enabled(self) -> bool {
        self.0 & Self::VE != 0
    }
    /// Enable ge sram clock ungating (`GE`).
    #[doc(alias = "GE")]
    #[inline]
    pub const fn enable_ge(self) -> Self {
        Self(self.0 | Self::GE)
    }
    /// Disable ge sram clock ungating.
    #[inline]
    pub const fn disable_ge(self) -> Self {
        Self(self.0 & !Self::GE)
    }
    /// Check if ge sram clock ungating is enabled.
    #[inline]
    pub const fn is_ge_enabled(self) -> bool {
        self.0 & Self::GE != 0
    }
    /// Enable de sram clock ungating (`DE`).
    #[doc(alias = "DE")]
    #[inline]
    pub const fn enable_de(self) -> Self {
        Self(self.0 | Self::DE)
    }
    /// Disable de sram clock ungating.
    #[inline]
    pub const fn disable_de(self) -> Self {
        Self(self.0 & !Self::DE)
    }
    /// Check if de sram clock ungating is enabled.
    #[inline]
    pub const fn is_de_enabled(self) -> bool {
        self.0 & Self::DE != 0
    }
    /// Enable ce sram clock ungating (`CE`).
    #[doc(alias = "CE")]
    #[inline]
    pub const fn enable_ce(self) -> Self {
        Self(self.0 | Self::CE)
    }
    /// Disable ce sram clock ungating.
    #[inline]
    pub const fn disable_ce(self) -> Self {
        Self(self.0 & !Self::CE)
    }
    /// Check if ce sram clock ungating is enabled.
    #[inline]
    pub const fn is_ce_enabled(self) -> bool {
        self.0 & Self::CE != 0
    }
    /// Enable uart sram clock ungating (`UART`).
    #[doc(alias = "UART")]
    #[inline]
    pub const fn enable_uart(self) -> Self {
        Self(self.0 | Self::UART)
    }
    /// Disable uart sram clock ungating.
    #[inline]
    pub const fn disable_uart(self) -> Self {
        Self(self.0 & !Self::UART)
    }
    /// Check if uart sram clock ungating is enabled.
    #[inline]
    pub const fn is_uart_enabled(self) -> bool {
        self.0 & Self::UART != 0
    }
    /// Enable sd sram clock ungating (`SD`).
    #[doc(alias = "SD")]
    #[inline]
    pub const fn enable_sd(self) -> Self {
        Self(self.0 | Self::SD)
    }
    /// Disable sd sram clock ungating.
    #[inline]
    pub const fn disable_sd(self) -> Self {
        Self(self.0 & !Self::SD)
    }
    /// Check if sd sram clock ungating is enabled.
    #[inline]
    pub const fn is_sd_enabled(self) -> bool {
        self.0 & Self::SD != 0
    }
    /// Enable usb sram clock ungating (`USB`).
    #[doc(alias = "USB")]
    #[inline]
    pub const fn enable_usb(self) -> Self {
        Self(self.0 | Self::USB)
    }
    /// Disable usb sram clock ungating.
    #[inline]
    pub const fn disable_usb(self) -> Self {
        Self(self.0 & !Self::USB)
    }
    /// Check if usb sram clock ungating is enabled.
    #[inline]
    pub const fn is_usb_enabled(self) -> bool {
        self.0 & Self::USB != 0
    }
    /// Enable spi sram clock ungating (`SPI`).
    #[doc(alias = "SPI")]
    #[inline]
    pub const fn enable_spi(self) -> Self {
        Self(self.0 | Self::SPI)
    }
    /// Disable spi sram clock ungating.
    #[inline]
    pub const fn disable_spi(self) -> Self {
        Self(self.0 & !Self::SPI)
    }
    /// Check if spi sram clock ungating is enabled.
    #[inline]
    pub const fn is_spi_enabled(self) -> bool {
        self.0 & Self::SPI != 0
    }
    /// Enable mipi sram clock ungating (`MIPI`).
    #[doc(alias = "MIPI")]
    #[inline]
    pub const fn enable_mipi(self) -> Self {
        Self(self.0 | Self::MIPI)
    }
    /// Disable mipi sram clock ungating.
    #[inline]
    pub const fn disable_mipi(self) -> Self {
        Self(self.0 & !Self::MIPI)
    }
    /// Check if mipi sram clock ungating is enabled.
    #[inline]
    pub const fn is_mipi_enabled(self) -> bool {
        self.0 & Self::MIPI != 0
    }
    /// Enable audio sram clock ungating (`AUDIO`).
    #[doc(alias = "AUDIO")]
    #[inline]
    pub const fn enable_audio(self) -> Self {
        Self(self.0 | Self::AUDIO)
    }
    /// Disable audio sram clock ungating.
    #[inline]
    pub const fn disable_audio(self) -> Self {
        Self(self.0 & !Self::AUDIO)
    }
    /// Check if audio sram clock ungating is enabled.
    #[inline]
    pub const fn is_audio_enabled(self) -> bool {
        self.0 & Self::AUDIO != 0
    }
    /// Enable sdfm sram clock ungating (`SDFM`).
    #[doc(alias = "SDFM")]
    #[inline]
    pub const fn enable_sdfm(self) -> Self {
        Self(self.0 | Self::SDFM)
    }
    /// Disable sdfm sram clock ungating.
    #[inline]
    pub const fn disable_sdfm(self) -> Self {
        Self(self.0 & !Self::SDFM)
    }
    /// Check if sdfm sram clock ungating is enabled.
    #[inline]
    pub const fn is_sdfm_enabled(self) -> bool {
        self.0 & Self::SDFM != 0
    }
    /// Enable xspi sram clock ungating (`XSPI`).
    #[doc(alias = "XSPI")]
    #[inline]
    pub const fn enable_xspi(self) -> Self {
        Self(self.0 | Self::XSPI)
    }
    /// Disable xspi sram clock ungating.
    #[inline]
    pub const fn disable_xspi(self) -> Self {
        Self(self.0 & !Self::XSPI)
    }
    /// Check if xspi sram clock ungating is enabled.
    #[inline]
    pub const fn is_xspi_enabled(self) -> bool {
        self.0 & Self::XSPI != 0
    }
}

/// AXI matrix slave0 address area.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AxiMatS0Area {
    /// 0x30040000~0x3005FFFF.
    Area0 = 0x1,
    /// 0x30060000~0x3007FFFF.
    Area1 = 0x2,
    /// 0x30080000~0x3009FFFF.
    Area2 = 0x4,
    /// 0x300A0000~0x300BFFFF.
    Area3 = 0x8,
    /// 0x300C0000~0x300DFFFF.
    Area4 = 0x10,
    /// 0x300E0000~0x300FFFFF.
    Area5 = 0x20,
    /// 0x30100000~0x3011FFFF.
    Area6 = 0x40,
    /// 0x30120000~0x3013FFFF.
    Area7 = 0x80,
}

/// AXI matrix slave1 size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AxiMatS1Size {
    /// S1 0 KB.
    Size0Kb,
    /// S1 128 KB.
    Size128Kb,
    /// S1 256 KB.
    Size256Kb,
    /// S1 384 KB.
    Size384Kb,
    /// S1 512 KB.
    Size512Kb,
    /// S1 640 KB.
    Size640Kb,
    /// S1 768 KB.
    Size768Kb,
}

/// SRAM mapping configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct SramMapConfig(u32);

impl SramMapConfig {
    const AXI_MAT_S0_CFG: u32 = 0xFF << 8;
    const AXI_MAT_S1_SIZE: u32 = 0x7 << 4;
    const CPU_TCM_SRAM_ACLK_GATE: u32 = 0x1 << 1;
    const CPU_TCM_SRAM_CFG: u32 = 0x1;

    /// Enable axi matrix slave0 address area write (`AXI_MAT_S0_CFG`).
    #[doc(alias = "AXI_MAT_S0_CFG")]
    #[inline]
    pub const fn enable_axi_mat_s0_area(self, area: AxiMatS0Area) -> Self {
        Self(self.0 | (Self::AXI_MAT_S0_CFG & ((area as u32) << 8)))
    }
    /// Disable axi matrix slave0 address area write.
    #[inline]
    pub const fn disable_axi_mat_s0_area(self, area: AxiMatS0Area) -> Self {
        Self(self.0 & !(Self::AXI_MAT_S0_CFG & ((area as u32) << 8)))
    }
    /// Check if axi matrix slave0 address area write is enabled.
    #[inline]
    pub const fn is_axi_mat_s0_area_enabled(self, area: AxiMatS0Area) -> bool {
        self.0 & (Self::AXI_MAT_S0_CFG & ((area as u32) << 8)) != 0
    }
    /// Set axi matrix slave1 size (`AXI_MAT_S1_SIZE`).
    ///
    /// The capacity of Slave0 is 1024 MB minus the capacities of TCM and Slave1.
    /// - `CPU_TCM_SRAM_CFG`=0: The capacity of Slave0 is 1024 KB minus that of Slave1.
    /// - `CPU_TCM_SRAM_CFG`=1: The capacity of Slave0 is 768 KB minus that of Slave1.
    #[doc(alias = "AXI_MAT_S1_SIZE")]
    #[inline]
    pub const fn set_axi_mat_s1_size(self, size: AxiMatS1Size) -> Self {
        Self((self.0 & !Self::AXI_MAT_S1_SIZE) | (Self::AXI_MAT_S1_SIZE & ((size as u32) << 4)))
    }
    /// Get axi matrix slave1 size.
    #[inline]
    pub const fn axi_mat_s1_size(self) -> AxiMatS1Size {
        match (self.0 & Self::AXI_MAT_S1_SIZE) >> 4 {
            0 => AxiMatS1Size::Size0Kb,
            1 => AxiMatS1Size::Size128Kb,
            2 => AxiMatS1Size::Size256Kb,
            3 => AxiMatS1Size::Size384Kb,
            4 => AxiMatS1Size::Size512Kb,
            5 => AxiMatS1Size::Size640Kb,
            6 => AxiMatS1Size::Size768Kb,
            _ => unreachable!(),
        }
    }
    /// Set cpu tcm sram aclk gate (`CPU_TCM_SRAM_ACLK_GATE`).
    ///
    /// Note: Before enabling `CPU_TCM_SRAM_CFG`, this bit field must first be set to 0 to avoid clock glitches.
    #[doc(alias = "CPU_TCM_SRAM_ACLK_GATE")]
    #[inline]
    pub const fn set_cpu_tcm_sram_aclk_gate(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::CPU_TCM_SRAM_ACLK_GATE)
        } else {
            Self(self.0 & !Self::CPU_TCM_SRAM_ACLK_GATE)
        }
    }
    /// Get cpu tcm sram aclk gate.
    #[inline]
    pub const fn cpu_tcm_sram_aclk_gate(self) -> bool {
        self.0 & Self::CPU_TCM_SRAM_ACLK_GATE != 0
    }
    /// Enable cpu tcm (`CPU_TCM_SRAM_CFG`).
    ///
    /// Note: ITCM is fixed at 128 KB, and DTCM is fixed at 64 KB.
    #[doc(alias = "CPU_TCM_SRAM_CFG")]
    #[inline]
    pub const fn enable_cpu_tcm(self) -> Self {
        Self(self.0 | Self::CPU_TCM_SRAM_CFG)
    }
    /// Disable cpu tcm.
    #[inline]
    pub const fn disable_cpu_tcm(self) -> Self {
        Self(self.0 & !Self::CPU_TCM_SRAM_CFG)
    }
    /// Check if cpu tcm is enabled.
    #[inline]
    pub const fn is_cpu_tcm_enabled(self) -> bool {
        self.0 & Self::CPU_TCM_SRAM_CFG != 0
    }
}

/// Flash io[0:2] mapping.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FlashIomap012 {
    /// WP - SO - CS.
    WpSoCs,
    /// WP - CS - SO.
    WpCsSo,
    /// SO - WP - CS.
    SoWpCs,
    /// SO - CS - WP.
    SoCsWp,
    /// CS - WP - SO.
    CsWpSo,
    /// CS - SO - WP.
    CsSoWp,
}

/// Flash io[3:5] mapping.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FlashIomap345 {
    /// HOLD - SCLK - SI.
    HoldSclkSi,
    /// HOLD - SI - SCLK.
    HoldSiSclk,
    /// SCLK - HOLD - SI.
    SclkHoldSi,
    /// SCLK - SI - HOLD.
    SclkSiHold,
    /// SI - HOLD - SCLK.
    SiHoldSclk,
    /// SI - SCLK - HOLD.
    SiSclkHold,
}

/// Flash access interface selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FlashSrcSel {
    /// SiP Flash interface disabled, inaccessible.
    None,
    /// SiP Flash interface accessed via PIN mapping, accessible by external devices.
    Pin,
    /// SiP Flash accessed via SPI0 controller.
    Spi0,
    /// SiP Flash accessed via SPI1 controller.
    Spi1,
}

/// FLASH configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FlashConfig(u32);

impl FlashConfig {
    const FLASH_IOMAP_012: u32 = 0x7 << 12;
    const FLASH_IOMAP_345: u32 = 0x7 << 8;
    const FLASH_SRC_SEL: u32 = 0x3;

    /// Set flash io[0:2] mapping (`FLASH_IOMAP_012`).
    ///
    /// After BROM starts, it needs to obtain the FLASH_IOMAP information from EFUSE and configure it into this bit field.
    #[doc(alias = "FLASH_IOMAP_012")]
    #[inline]
    pub const fn set_flash_iomap_012(self, mapping: FlashIomap012) -> Self {
        Self((self.0 & !Self::FLASH_IOMAP_012) | (Self::FLASH_IOMAP_012 & ((mapping as u32) << 12)))
    }
    /// Get flash io[0:2] mapping.
    #[inline]
    pub const fn flash_iomap_012(self) -> FlashIomap012 {
        match (self.0 & Self::FLASH_IOMAP_012) >> 12 {
            0 => FlashIomap012::WpSoCs,
            1 => FlashIomap012::WpCsSo,
            2 => FlashIomap012::SoWpCs,
            3 => FlashIomap012::SoCsWp,
            4 => FlashIomap012::CsWpSo,
            5 => FlashIomap012::CsSoWp,
            _ => unreachable!(),
        }
    }
    /// Set flash io[3:5] mapping (`FLASH_IOMAP_345`).
    ///
    /// After BROM starts, it needs to obtain the FLASH_IOMAP information from EFUSE and configure it into this bit field.
    #[doc(alias = "FLASH_IOMAP_345")]
    #[inline]
    pub const fn set_flash_iomap_345(self, mapping: FlashIomap345) -> Self {
        Self((self.0 & !Self::FLASH_IOMAP_345) | (Self::FLASH_IOMAP_345 & ((mapping as u32) << 8)))
    }
    /// Get flash io[3:5] mapping.
    #[inline]
    pub const fn flash_iomap_345(self) -> FlashIomap345 {
        match (self.0 & Self::FLASH_IOMAP_345) >> 8 {
            0 => FlashIomap345::HoldSclkSi,
            1 => FlashIomap345::HoldSiSclk,
            2 => FlashIomap345::SclkHoldSi,
            3 => FlashIomap345::SclkSiHold,
            4 => FlashIomap345::SiHoldSclk,
            5 => FlashIomap345::SiSclkHold,
            _ => unreachable!(),
        }
    }
    /// Set flash access interface selection (`FLASH_SRC_SEL`).
    #[doc(alias = "FLASH_SRC_SEL")]
    #[inline]
    pub const fn set_flash_src_sel(self, sel: FlashSrcSel) -> Self {
        Self((self.0 & !Self::FLASH_SRC_SEL) | (Self::FLASH_SRC_SEL & (sel as u32)))
    }
    /// Get flash access interface selection.
    #[inline]
    pub const fn flash_src_sel(self) -> FlashSrcSel {
        match self.0 & Self::FLASH_SRC_SEL {
            0 => FlashSrcSel::None,
            1 => FlashSrcSel::Pin,
            2 => FlashSrcSel::Spi0,
            3 => FlashSrcSel::Spi1,
            _ => unreachable!(),
        }
    }
}

/// Encoder 0 selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Enc0Sel {
    /// IO0=QEP_A, IO1=QEP_B, IO2=QEP_I.
    Qep0,
    /// IO0=EDAT_DE, IO1=EDAT_DIO, IO2=EDAT_CLK.
    Edat0,
    /// IO0=TA_DE, IO1=TA_DIO.
    Ta0,
    /// IO0=BIS_MA, IO1=BIS_SLO.
    Bis0,
}

/// Encoder 1 selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Enc1Sel {
    /// IO0=QEP_A, IO1=QEP_B, IO2=QEP_I.
    Qep1,
    /// IO0=EDAT_DE, IO1=EDAT_DIO, IO2=EDAT_CLK.
    Edat1,
    /// IO0=TA_DE, IO1=TA_DIO.
    Ta1,
    /// IO0=BIS_MA, IO1=BIS_SLO.
    Bis1,
}

/// ENCODER configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct EncoderConfig(u32);

impl EncoderConfig {
    const ENC1_SEL: u32 = 0x3 << 16;
    const ENC0_SEL: u32 = 0x3;

    /// Set encoder 1 selection (`ENC1_SEL`).
    #[doc(alias = "ENC1_SEL")]
    #[inline]
    pub const fn set_enc1_sel(self, sel: Enc1Sel) -> Self {
        Self((self.0 & !Self::ENC1_SEL) | (Self::ENC1_SEL & ((sel as u32) << 16)))
    }
    /// Get encoder 1 selection.
    #[inline]
    pub const fn enc1_sel(self) -> Enc1Sel {
        match (self.0 & Self::ENC1_SEL) >> 16 {
            0 => Enc1Sel::Qep1,
            1 => Enc1Sel::Edat1,
            2 => Enc1Sel::Ta1,
            3 => Enc1Sel::Bis1,
            _ => unreachable!(),
        }
    }
    /// Set encoder 0 selection (`ENC0_SEL`).
    #[doc(alias = "ENC0_SEL")]
    #[inline]
    pub const fn set_enc0_sel(self, sel: Enc0Sel) -> Self {
        Self((self.0 & !Self::ENC0_SEL) | (Self::ENC0_SEL & (sel as u32)))
    }
    /// Get encoder 0 selection.
    #[inline]
    pub const fn enc0_sel(self) -> Enc0Sel {
        match self.0 & Self::ENC0_SEL {
            0 => Enc0Sel::Qep0,
            1 => Enc0Sel::Edat0,
            2 => Enc0Sel::Ta0,
            3 => Enc0Sel::Bis0,
            _ => unreachable!(),
        }
    }
}

/// DRD mode selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DrdMode {
    Host,
    Device,
}

/// USB0 configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Usb0Config(u32);

impl Usb0Config {
    const DRD_MODE: u32 = 0x1;

    /// Set drd mode selection (`DRD_MODE`).
    #[doc(alias = "DRD_MODE")]
    #[inline]
    pub const fn set_drd_mode(self, mode: DrdMode) -> Self {
        Self((self.0 & !Self::DRD_MODE) | (Self::DRD_MODE & (mode as u32)))
    }
    /// Get drd mode selection.
    #[inline]
    pub const fn drd_mode(self) -> DrdMode {
        match self.0 & Self::DRD_MODE {
            0 => DrdMode::Host,
            1 => DrdMode::Device,
            _ => unreachable!(),
        }
    }
}

/// RMII external clock selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RmiiExtclkSel {
    Intclk,
    Extclk,
}

/// EMAC configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct EmacConfig(u32);

impl EmacConfig {
    const REFCLK_INV: u32 = 0x1 << 29;
    const REFCLK_DLY_CHAIN_SEL: u32 = 0x1F << 24;
    const RXCLK_INV: u32 = 0x1 << 23;
    const RXCLK_DLY_SEL: u32 = 0x1F << 18;
    const TXCLK_INV: u32 = 0x1 << 17;
    const TXCLK_DLY_CHAIN_SEL: u32 = 0x1F << 12;
    const SW_TXCLK_DIV2: u32 = 0xF << 8;
    const SW_TXCLK_DIV1: u32 = 0xF << 4;
    const SW_TXCLK_DIV_EN: u32 = 0x1 << 2;
    const RMII_EXTCLK_SEL: u32 = 0x1 << 1;

    /// Enable reference clock inversion (`REFCLK_INV`).
    #[doc(alias = "REFCLK_INV")]
    #[inline]
    pub const fn enable_refclk_inv(self) -> Self {
        Self(self.0 | Self::REFCLK_INV)
    }
    /// Disable reference clock inversion.
    #[inline]
    pub const fn disable_refclk_inv(self) -> Self {
        Self(self.0 & !Self::REFCLK_INV)
    }
    /// Check if reference clock inversion is enabled.
    #[inline]
    pub const fn is_refclk_inv_enabled(self) -> bool {
        (self.0 & Self::REFCLK_INV) != 0
    }
    /// Set reference clock delay chain selection (`REFCLK_DLY_CHAIN_SEL`).
    #[doc(alias = "REFCLK_DLY_CHAIN_SEL")]
    #[inline]
    pub const fn set_refclk_dly_chain_sel(self, sel: u8) -> Self {
        assert!(
            sel < 0x20,
            "Reference clock delay chain selection out of range (expected 0..=31)"
        );
        Self(
            (self.0 & !Self::REFCLK_DLY_CHAIN_SEL)
                | (Self::REFCLK_DLY_CHAIN_SEL & ((sel as u32) << 24)),
        )
    }
    /// Get reference clock delay chain selection.
    #[inline]
    pub const fn refclk_dly_chain_sel(self) -> u8 {
        ((self.0 & Self::REFCLK_DLY_CHAIN_SEL) >> 24) as u8
    }
    /// Enable receive clock inversion (`RXCLK_INV`).
    #[doc(alias = "RXCLK_INV")]
    #[inline]
    pub const fn enable_rxclk_inv(self) -> Self {
        Self(self.0 | Self::RXCLK_INV)
    }
    /// Disable receive clock inversion.
    #[inline]
    pub const fn disable_rxclk_inv(self) -> Self {
        Self(self.0 & !Self::RXCLK_INV)
    }
    /// Check if receive clock inversion is enabled.
    #[inline]
    pub const fn is_rxclk_inv_enabled(self) -> bool {
        (self.0 & Self::RXCLK_INV) != 0
    }
    /// Set receive clock delay selection (`RXCLK_DLY_SEL`).
    #[doc(alias = "RXCLK_DLY_SEL")]
    #[inline]
    pub const fn set_rxclk_dly_sel(self, sel: u8) -> Self {
        assert!(
            sel < 0x20,
            "Receive clock delay selection out of range (expected 0..=31)"
        );
        Self((self.0 & !Self::RXCLK_DLY_SEL) | (Self::RXCLK_DLY_SEL & ((sel as u32) << 18)))
    }
    /// Get receive clock delay selection.
    #[inline]
    pub const fn rxclk_dly_sel(self) -> u8 {
        ((self.0 & Self::RXCLK_DLY_SEL) >> 18) as u8
    }
    /// Enable transmit clock inversion (`TXCLK_INV`).
    #[doc(alias = "TXCLK_INV")]
    #[inline]
    pub const fn enable_txclk_inv(self) -> Self {
        Self(self.0 | Self::TXCLK_INV)
    }
    /// Disable transmit clock inversion.
    #[inline]
    pub const fn disable_txclk_inv(self) -> Self {
        Self(self.0 & !Self::TXCLK_INV)
    }
    /// Check if transmit clock inversion is enabled.
    #[inline]
    pub const fn is_txclk_inv_enabled(self) -> bool {
        (self.0 & Self::TXCLK_INV) != 0
    }
    /// Set transmit clock delay chain selection (`TXCLK_DLY_CHAIN_SEL`).
    #[doc(alias = "TXCLK_DLY_CHAIN_SEL")]
    #[inline]
    pub fn set_txclk_dly_chain_sel(self, sel: u8) -> Self {
        assert!(
            sel < 0x20,
            "Transmit clock delay chain selection out of range (expected 0..=31)"
        );
        Self(
            (self.0 & !Self::TXCLK_DLY_CHAIN_SEL)
                | (Self::TXCLK_DLY_CHAIN_SEL & ((sel as u32) << 12)),
        )
    }
    /// Get transmit clock delay chain selection.
    #[inline]
    pub const fn txclk_dly_chain_sel(self) -> u8 {
        ((self.0 & Self::TXCLK_DLY_CHAIN_SEL) >> 12) as u8
    }
    /// Set software transmit clock divider 2 (`SW_TXCLK_DIV2`).
    #[doc(alias = "SW_TXCLK_DIV2")]
    #[inline]
    pub const fn set_sw_txclk_div2(self, div: u8) -> Self {
        assert!(
            div < 0x10,
            "Software transmit clock divider 2 out of range (expected 0..=15)"
        );
        Self((self.0 & !Self::SW_TXCLK_DIV2) | (Self::SW_TXCLK_DIV2 & ((div as u32) << 8)))
    }
    /// Get software transmit clock divider 2.
    #[inline]
    pub const fn sw_txclk_div2(self) -> u8 {
        ((self.0 & Self::SW_TXCLK_DIV2) >> 8) as u8
    }
    /// Set software transmit clock divider 1 (`SW_TXCLK_DIV1`).
    #[doc(alias = "SW_TXCLK_DIV1")]
    #[inline]
    pub const fn set_sw_txclk_div1(self, div: u8) -> Self {
        assert!(
            div < 0x10,
            "Software transmit clock divider 1 out of range (expected 0..=15)"
        );
        Self((self.0 & !Self::SW_TXCLK_DIV1) | (Self::SW_TXCLK_DIV1 & ((div as u32) << 4)))
    }
    /// Get software transmit clock divider 1.
    #[inline]
    pub const fn sw_txclk_div1(self) -> u8 {
        ((self.0 & Self::SW_TXCLK_DIV1) >> 4) as u8
    }
    /// Enable software transmit clock divider (`SW_TXCLK_DIV_EN`).
    #[doc(alias = "SW_TXCLK_DIV_EN")]
    #[inline]
    pub const fn enable_sw_txclk_div(self) -> Self {
        Self(self.0 | Self::SW_TXCLK_DIV_EN)
    }
    /// Disable software transmit clock divider.
    #[inline]
    pub const fn disable_sw_txclk_div(self) -> Self {
        Self(self.0 & !Self::SW_TXCLK_DIV_EN)
    }
    /// Check if software transmit clock divider is enabled.
    #[inline]
    pub const fn is_sw_txclk_div_enabled(self) -> bool {
        (self.0 & Self::SW_TXCLK_DIV_EN) != 0
    }
    /// Set RMII external clock selection (`RMII_EXTCLK_SEL`).
    #[doc(alias = "RMII_EXTCLK_SEL")]
    #[inline]
    pub const fn set_rmii_extclk_sel(self, sel: RmiiExtclkSel) -> Self {
        Self((self.0 & !Self::RMII_EXTCLK_SEL) | (Self::RMII_EXTCLK_SEL & ((sel as u32) << 1)))
    }
    /// Get RMII external clock selection.
    #[inline]
    pub const fn rmii_extclk_sel(self) -> RmiiExtclkSel {
        match (self.0 & Self::RMII_EXTCLK_SEL) >> 1 {
            0 => RmiiExtclkSel::Intclk,
            1 => RmiiExtclkSel::Extclk,
            _ => unreachable!(),
        }
    }
}

/// CMU ATB selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CmuAtbSel {
    Disable,
    VbiasVbp,
    VbiasVbpc,
    VbiasVbn,
    VbiasVbnc,
    VbiasVfbb,
    LdoDvDv,
    LdoDvVbpc,
    LdoDvVbp,
    LdoDvVbnc,
    LdoDvVbn,
    LdoAvInt0Av,
    LdoAvInt0Vbpc,
    LdoAvInt0Vbp,
    LdoAvInt0Vbnc,
    LdoAvInt0Vbn,
    LdoAvInt1Av,
    LdoAvFra0Av,
    LdoAvFra0Vbpc,
    LdoAvFra0Vbp,
    LdoAvFra0Vbnc,
    LdoAvFra0Vbn,
    LdoAvFra1Av,
    LdoAvFra2Av,
    Int0IbCp,
    Int1IbCp,
    Fra0IbCp,
    Fra1IbCp,
    Fra2IbCp,
}

/// ATB_CMU_ANA_TOP register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AtbCmuAnaTop(u32);

impl AtbCmuAnaTop {
    const CMU_ATBSEL: u32 = 0x1F;

    /// Set CMU ATB selection (`CMU_ATBSEL`).
    #[doc(alias = "CMU_ATBSEL")]
    #[inline]
    pub const fn set_cmu_atbsel(self, sel: CmuAtbSel) -> Self {
        Self((self.0 & !Self::CMU_ATBSEL) | (Self::CMU_ATBSEL & (sel as u32)))
    }
    /// Get CMU ATB selection.
    #[inline]
    pub const fn cmu_atbsel(self) -> CmuAtbSel {
        match self.0 & Self::CMU_ATBSEL {
            0 => CmuAtbSel::Disable,
            1 => CmuAtbSel::VbiasVbp,
            2 => CmuAtbSel::VbiasVbpc,
            3 => CmuAtbSel::VbiasVbn,
            4 => CmuAtbSel::VbiasVbnc,
            5 => CmuAtbSel::VbiasVfbb,
            6 => CmuAtbSel::LdoDvDv,
            7 => CmuAtbSel::LdoDvVbpc,
            8 => CmuAtbSel::LdoDvVbp,
            9 => CmuAtbSel::LdoDvVbnc,
            10 => CmuAtbSel::LdoDvVbn,
            11 => CmuAtbSel::LdoAvInt0Av,
            12 => CmuAtbSel::LdoAvInt0Vbpc,
            13 => CmuAtbSel::LdoAvInt0Vbp,
            14 => CmuAtbSel::LdoAvInt0Vbnc,
            15 => CmuAtbSel::LdoAvInt0Vbn,
            16 => CmuAtbSel::LdoAvInt1Av,
            17 => CmuAtbSel::LdoAvFra0Av,
            18 => CmuAtbSel::LdoAvFra0Vbpc,
            19 => CmuAtbSel::LdoAvFra0Vbp,
            20 => CmuAtbSel::LdoAvFra0Vbnc,
            21 => CmuAtbSel::LdoAvFra0Vbn,
            22 => CmuAtbSel::LdoAvFra1Av,
            23 => CmuAtbSel::LdoAvFra2Av,
            24 => CmuAtbSel::Int0IbCp,
            25 => CmuAtbSel::Int1IbCp,
            26 => CmuAtbSel::Fra0IbCp,
            27 => CmuAtbSel::Fra1IbCp,
            28 => CmuAtbSel::Fra2IbCp,
            _ => unreachable!(),
        }
    }
}

/// DLL analog test selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DllAtbSel {
    LockTest,
    Vbn,
    Vctrl,
    Vbp,
    Dvdd,
    LdoVfb,
    Icp,
}

/// ATB_DLL_TOP_C register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AtbDllTopC(u32);

impl AtbDllTopC {
    const DLL1_ATB_SEL: u32 = 0x7 << 12;
    const DLL1_EN_ATB: u32 = 0x1 << 8;
    const DLL0_ATB_SEL: u32 = 0x7 << 4;
    const DLL0_EN_ATB: u32 = 0x1;

    /// Set dll1 analog test selection (`DLL1_ATB_SEL`).
    #[doc(alias = "DLL1_ATB_SEL")]
    #[inline]
    pub const fn set_dll1_atb_sel(self, sel: DllAtbSel) -> Self {
        Self((self.0 & !Self::DLL1_ATB_SEL) | (Self::DLL1_ATB_SEL & ((sel as u32) << 12)))
    }
    /// Get dll1 analog test selection.
    #[inline]
    pub const fn dll1_atb_sel(self) -> DllAtbSel {
        match (self.0 & Self::DLL1_ATB_SEL) >> 12 {
            0 => DllAtbSel::LockTest,
            1 => DllAtbSel::Vbn,
            2 => DllAtbSel::Vctrl,
            3 => DllAtbSel::Vbp,
            4 => DllAtbSel::Dvdd,
            5 => DllAtbSel::LdoVfb,
            6 => DllAtbSel::Icp,
            _ => unreachable!(),
        }
    }
    /// Enable dll1 analog test (`DLL1_EN_ATB`).
    #[doc(alias = "DLL1_EN_ATB")]
    #[inline]
    pub const fn enable_dll1_atb(self) -> Self {
        Self(self.0 | Self::DLL1_EN_ATB)
    }
    /// Disable dll1 analog test.
    #[inline]
    pub const fn disable_dll1_atb(self) -> Self {
        Self(self.0 & !Self::DLL1_EN_ATB)
    }
    /// Check if dll1 analog test is enabled.
    #[inline]
    pub const fn is_dll1_atb_enabled(self) -> bool {
        (self.0 & Self::DLL1_EN_ATB) != 0
    }
    /// Set dll0 analog test selection (`DLL0_ATB_SEL`).
    #[doc(alias = "DLL0_ATB_SEL")]
    #[inline]
    pub const fn set_dll0_atb_sel(self, sel: DllAtbSel) -> Self {
        Self((self.0 & !Self::DLL0_ATB_SEL) | (Self::DLL0_ATB_SEL & ((sel as u32) << 4)))
    }
    /// Get dll0 analog test selection.
    #[inline]
    pub const fn dll0_atb_sel(self) -> DllAtbSel {
        match (self.0 & Self::DLL0_ATB_SEL) >> 4 {
            0 => DllAtbSel::LockTest,
            1 => DllAtbSel::Vbn,
            2 => DllAtbSel::Vctrl,
            3 => DllAtbSel::Vbp,
            4 => DllAtbSel::Dvdd,
            5 => DllAtbSel::LdoVfb,
            6 => DllAtbSel::Icp,
            _ => unreachable!(),
        }
    }
    /// Enable dll0 analog test (`DLL0_EN_ATB`).
    #[doc(alias = "DLL0_EN_ATB")]
    #[inline]
    pub const fn enable_dll0_atb(self) -> Self {
        Self(self.0 | Self::DLL0_EN_ATB)
    }
    /// Disable dll0 analog test.
    #[inline]
    pub const fn disable_dll0_atb(self) -> Self {
        Self(self.0 & !Self::DLL0_EN_ATB)
    }
    /// Check if dll0 analog test is enabled.
    #[inline]
    pub const fn is_dll0_atb_enabled(self) -> bool {
        (self.0 & Self::DLL0_EN_ATB) != 0
    }
}

/// Analog test selection 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AtbSel0 {
    RtpVin,
    Vrefp,
    Vrefn,
    RtpTouch,
}

/// ATB_GPADC register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AtbGpadc(u32);

impl AtbGpadc {
    const ATB_SEL: u32 = 0x3 << 4;
    const ATB_EN: u32 = 0x1;

    /// Set analog test selection (`ATB_SEL`).
    #[doc(alias = "ATB_SEL")]
    #[inline]
    pub const fn set_atb_sel(self, sel: AtbSel0) -> Self {
        Self((self.0 & !Self::ATB_SEL) | (Self::ATB_SEL & ((sel as u32) << 4)))
    }
    /// Get analog test selection.
    #[inline]
    pub const fn atb_sel(self) -> AtbSel0 {
        match (self.0 & Self::ATB_SEL) >> 4 {
            0 => AtbSel0::RtpVin,
            1 => AtbSel0::Vrefp,
            2 => AtbSel0::Vrefn,
            3 => AtbSel0::RtpTouch,
            _ => unreachable!(),
        }
    }
    /// Enable analog test (`ATB_EN`).
    #[doc(alias = "ATB_EN")]
    #[inline]
    pub const fn enable_atb(self) -> Self {
        Self(self.0 | Self::ATB_EN)
    }
    /// Disable analog test.
    #[inline]
    pub const fn disable_atb(self) -> Self {
        Self(self.0 & !Self::ATB_EN)
    }
    /// Check if analog test is enabled.
    #[inline]
    pub const fn is_atb_enabled(self) -> bool {
        (self.0 & Self::ATB_EN) != 0
    }
}

/// Analog test selection 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AtbSel1 {
    VresAtb,
    Ipn5uAtb,
    V0p4Atb,
    Vpck,
}

/// ATB_MIPI_DPHY register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AtbMipiDphy(u32);

impl AtbMipiDphy {
    const ATB_SEL: u32 = 0x3 << 4;
    const ATB_EN: u32 = 0x1;

    /// Set analog test selection (`ATB_SEL`).
    #[doc(alias = "ATB_SEL")]
    #[inline]
    pub const fn set_atb_sel(self, sel: AtbSel1) -> Self {
        Self((self.0 & !Self::ATB_SEL) | (Self::ATB_SEL & ((sel as u32) << 4)))
    }
    /// Get analog test selection.
    #[inline]
    pub const fn atb_sel(self) -> AtbSel1 {
        match (self.0 & Self::ATB_SEL) >> 4 {
            0 => AtbSel1::VresAtb,
            1 => AtbSel1::Ipn5uAtb,
            2 => AtbSel1::V0p4Atb,
            3 => AtbSel1::Vpck,
            _ => unreachable!(),
        }
    }
    /// Enable analog test (`ATB_EN`).
    #[doc(alias = "ATB_EN")]
    #[inline]
    pub const fn enable_atb(self) -> Self {
        Self(self.0 | Self::ATB_EN)
    }
    /// Disable analog test.
    #[inline]
    pub const fn disable_atb(self) -> Self {
        Self(self.0 & !Self::ATB_EN)
    }
    /// Check if analog test is enabled.
    #[inline]
    pub const fn is_atb_enabled(self) -> bool {
        (self.0 & Self::ATB_EN) != 0
    }
}

/// Analog test selection 2.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AtbSel2 {
    VrefOut,
    Vosc,
    IbpIbas,
    IbpVdet,
}

/// ATB_RTC_ANA_TOP register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AtbRtcAnaTop(u32);

impl AtbRtcAnaTop {
    const ATB_SEL: u32 = 0x3 << 4;
    const ATB_EN: u32 = 0x1;

    /// Set analog test selection (`ATB_SEL`).
    #[doc(alias = "ATB_SEL")]
    #[inline]
    pub const fn set_atb_sel(self, sel: AtbSel2) -> Self {
        Self((self.0 & !Self::ATB_SEL) | (Self::ATB_SEL & ((sel as u32) << 4)))
    }
    /// Get analog test selection.
    #[inline]
    pub const fn atb_sel(self) -> AtbSel2 {
        match (self.0 & Self::ATB_SEL) >> 4 {
            0 => AtbSel2::VrefOut,
            1 => AtbSel2::Vosc,
            2 => AtbSel2::IbpIbas,
            3 => AtbSel2::IbpVdet,
            _ => unreachable!(),
        }
    }
    /// Enable analog test (`ATB_EN`).
    #[doc(alias = "ATB_EN")]
    #[inline]
    pub const fn enable_atb(self) -> Self {
        Self(self.0 | Self::ATB_EN)
    }
    /// Disable analog test.
    #[inline]
    pub const fn disable_atb(self) -> Self {
        Self(self.0 & !Self::ATB_EN)
    }
    /// Check if analog test is enabled.
    #[inline]
    pub const fn is_atb_enabled(self) -> bool {
        (self.0 & Self::ATB_EN) != 0
    }
}

/// Subpll analog test selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SubpllAtb {
    Disable,
    VbiasVbp,
    VbiasVbpc,
    VbiasVbn,
    VbiasVbnc,
    VbiasVfbb,
    LdoDvDv,
    LdoDvVbpc,
    LdoDvVbp,
    LdoDvVbnc,
    LdoDvVbn,
    LdoAvInt0Av,
    LdoAvInt0Vbpc,
    LdoAvInt0Vbp,
    LdoAvInt0Vbnc,
    LdoAvInt0Vbn,
    Int0IbCp,
}

/// ATB_USB_PLL_AFE register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AtbUsbPllAfe(u32);

impl AtbUsbPllAfe {
    const SUBPLL_ATB: u32 = 0x1F;

    /// Set subpll analog test selection (`SUBPLL_ATB`).
    #[doc(alias = "SUBPLL_ATB")]
    #[inline]
    pub const fn set_subpll_atb(self, sel: SubpllAtb) -> Self {
        Self((self.0 & !Self::SUBPLL_ATB) | (Self::SUBPLL_ATB & (sel as u32)))
    }
    /// Get subpll analog test selection.
    #[inline]
    pub const fn subpll_atb(self) -> SubpllAtb {
        match self.0 & Self::SUBPLL_ATB {
            0 => SubpllAtb::Disable,
            1 => SubpllAtb::VbiasVbp,
            2 => SubpllAtb::VbiasVbpc,
            3 => SubpllAtb::VbiasVbn,
            4 => SubpllAtb::VbiasVbnc,
            5 => SubpllAtb::VbiasVfbb,
            6 => SubpllAtb::LdoDvDv,
            7 => SubpllAtb::LdoDvVbpc,
            8 => SubpllAtb::LdoDvVbp,
            9 => SubpllAtb::LdoDvVbnc,
            10 => SubpllAtb::LdoDvVbn,
            11 => SubpllAtb::LdoAvInt0Av,
            12 => SubpllAtb::LdoAvInt0Vbpc,
            13 => SubpllAtb::LdoAvInt0Vbp,
            14 => SubpllAtb::LdoAvInt0Vbnc,
            15 => SubpllAtb::LdoAvInt0Vbn,
            16 => SubpllAtb::Int0IbCp,
            _ => unreachable!(),
        }
    }
}

/// SQRX analog test selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SqrxTestSel {
    SqOffsetP = 0x1,
    SqOffsetM = 0x2,
    Dp = 0x8,
    Dm = 0x10,
}

/// TX analog test selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TxTestSel {
    /// Atb_op and avdd_atb output.
    AtbOpAvddAtb,
    /// Vref.
    Vref,
    /// Vreg.
    Vreg,
    /// Rtune.
    Rtune,
}

/// ATB_USB_PHY_AFE register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AtbUsbPhyAfe(u32);

impl AtbUsbPhyAfe {
    const SQRX_TEST: u32 = 0x1F << 8;
    const TX_TEST: u32 = 0x3 << 4;
    const ATB_OP: u32 = 0x1 << 1;
    const AVDD_ATB: u32 = 0x1;

    /// Enable sqrx (`SQRX_TEST`).
    #[doc(alias = "SQRX_TEST")]
    #[inline]
    pub const fn enable_sqrx(self, sel: SqrxTestSel) -> Self {
        Self(self.0 | (Self::SQRX_TEST & ((sel as u32) << 8)))
    }
    /// Disable sqrx.
    #[inline]
    pub const fn disable_sqrx(self, sel: SqrxTestSel) -> Self {
        Self(self.0 & !(Self::SQRX_TEST & ((sel as u32) << 8)))
    }
    /// Check if sqrx is enabled.
    #[inline]
    pub const fn is_sqrx_enabled(self, sel: SqrxTestSel) -> bool {
        (self.0 & (Self::SQRX_TEST & ((sel as u32) << 8))) != 0
    }
    #[doc(alias = "TX_TEST")]
    #[inline]
    pub const fn set_tx_test_sel(self, sel: TxTestSel) -> Self {
        Self((self.0 & !Self::TX_TEST) | (Self::TX_TEST & ((sel as u32) << 4)))
    }
    /// Get tx analog test selection.
    #[inline]
    pub const fn tx_test_sel(self) -> TxTestSel {
        match (self.0 & Self::TX_TEST) >> 4 {
            0 => TxTestSel::AtbOpAvddAtb,
            1 => TxTestSel::Vref,
            2 => TxTestSel::Vreg,
            3 => TxTestSel::Rtune,
            _ => unreachable!(),
        }
    }
    /// Enable ldo power transistor gate voltage test (`ATB_OP`).
    #[doc(alias = "ATB_OP")]
    #[inline]
    pub const fn enable_atb_op(self) -> Self {
        Self(self.0 | Self::ATB_OP)
    }
    /// Disable ldo power transistor gate voltage test.
    #[inline]
    pub const fn disable_atb_op(self) -> Self {
        Self(self.0 & !Self::ATB_OP)
    }
    /// Check if ldo power transistor gate voltage test is enabled.
    #[inline]
    pub const fn is_atb_op_enabled(self) -> bool {
        (self.0 & Self::ATB_OP) != 0
    }
    /// Enable ldo output test (`AVDD_ATB`).
    #[doc(alias = "AVDD_ATB")]
    #[inline]
    pub const fn enable_avdd_atb(self) -> Self {
        Self(self.0 | Self::AVDD_ATB)
    }
    /// Disable ldo output test.
    #[inline]
    pub const fn disable_avdd_atb(self) -> Self {
        Self(self.0 & !Self::AVDD_ATB)
    }
    /// Check if ldo output test is enabled.
    #[inline]
    pub const fn is_avdd_atb_enabled(self) -> bool {
        (self.0 & Self::AVDD_ATB) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_should_panic;

    #[test]
    fn struct_irq_control_functions() {
        let mut val = IrqControl(0x0);

        val = val.enable_cmp_rst();
        assert!(val.is_cmp_rst_enabled());
        assert_eq!(val.0, 0x8000_0000);
        val = val.disable_cmp_rst();
        assert!(!val.is_cmp_rst_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_cmp_irq();
        assert!(val.is_cmp_irq_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_cmp_irq();
        assert!(!val.is_cmp_irq_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_irq_status_functions() {
        let mut val = IrqStatus(0x0);

        val = val.clear_cmp_irq();
        assert!(val.is_cmp_irq_pending());
        assert_eq!(val.0, 0x0000_0001);
    }

    #[test]
    fn struct_compare_config_functions() {
        let mut val = CompareConfig(0x0);

        val = val.set_cmp_db(0xFF);
        assert_eq!(val.cmp_db(), 0xFF);
        assert_eq!(val.0, 0xFF00_0000);

        val = CompareConfig(0x0).set_cmp_mode(CmpMode::HighVoltage);
        assert_eq!(val.cmp_mode(), CmpMode::HighVoltage);
        assert_eq!(val.0, 0x0000_0020);
        val = val.set_cmp_mode(CmpMode::LowVoltage);
        assert_eq!(val.cmp_mode(), CmpMode::LowVoltage);
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_cmp();
        assert!(val.is_cmp_enabled());
        assert_eq!(val.0, 0x0000_0010);
        val = val.disable_cmp();
        assert!(!val.is_cmp_enabled());
        assert_eq!(val.0, 0x0000_0000);

        for i in 0..8 {
            let (sel, reg_val) = match i {
                0 => (CmpVoltage::V0_90, 0x0000_0000),
                1 => (CmpVoltage::V0_85, 0x0000_0001),
                2 => (CmpVoltage::V0_80, 0x0000_0002),
                3 => (CmpVoltage::V0_75, 0x0000_0003),
                4 => (CmpVoltage::V0_70, 0x0000_0004),
                5 => (CmpVoltage::V0_65, 0x0000_0005),
                6 => (CmpVoltage::V0_60, 0x0000_0006),
                7 => (CmpVoltage::V0_55, 0x0000_0007),
                _ => unreachable!(),
            };

            val = val.set_cmp_voltage_sel(sel);
            assert_eq!(val.cmp_voltage_sel(), sel);
            assert_eq!(val.0, reg_val);
        }
    }

    #[test]
    fn struct_usb0r_ext_functions() {
        let mut val = ExternalResistor(0x0);

        val = val.enable_res_cal();
        assert!(val.is_res_cal_enabled());
        assert_eq!(val.0, 0x0000_0100);
        val = val.disable_res_cal();
        assert!(!val.is_res_cal_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_res_cal_val(0xFF);
        assert_eq!(val.res_cal_val(), 0xFF);
        assert_eq!(val.0, 0x0000_00FF);
    }

    #[test]
    fn struct_psen_config_functions() {
        let mut val = PsenConfig(0x0);

        val = val.set_cnt_time(0xFFFF);
        assert_eq!(val.cnt_time(), 0xFFFF);
        assert_eq!(val.0, 0xFFFF0000);

        val = PsenConfig(0x0);
        let rol_test_cases = [
            (RoSel::Rvt40Cell, 0x0000_0002),
            (RoSel::Lvt40Cell, 0x0000_0004),
            (RoSel::Ulvt40Cell, 0x0000_0006),
            (RoSel::Rvt50Cell, 0x0000_000A),
            (RoSel::Lvt50Cell, 0x0000_000C),
            (RoSel::Ulvt50Cell, 0x0000_000E),
        ];

        for (sel, reg_val) in rol_test_cases {
            val = val.set_ro_sel(sel);
            assert_eq!(val.ro_sel(), sel);
            assert_eq!(val.0, reg_val);
        }

        val = PsenConfig(0x0).set_psen_start(true);
        assert!(val.psen_start());
        assert_eq!(val.0, 0x0000_0001);
        val = val.set_psen_start(false);
        assert!(!val.psen_start());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_psen_cnt_val_functions() {
        let val = PsenCntVal(0x0000_FFFF);
        assert_eq!(val.cnt_val(), 0xFFFF);
    }

    #[test]
    fn struct_ge_sram_param_functions() {
        let val = GeSramParam(0x0).set_sram_par(0xFFFF);
        assert_eq!(val.sram_par(), 0xFFFF);
        assert_eq!(val.0, 0x0000_FFFF);
    }

    #[test]
    fn struct_sram_clk_config_functions() {
        let mut val = SramClkConfig(0x0);

        val = val.enable_sys();
        assert!(val.is_sys_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_sys();
        assert!(!val.is_sys_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_ddr();
        assert!(val.is_ddr_enabled());
        assert_eq!(val.0, 0x0000_0002);
        val = val.disable_ddr();
        assert!(!val.is_ddr_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_dma();
        assert!(val.is_dma_enabled());
        assert_eq!(val.0, 0x0000_0004);
        val = val.disable_dma();
        assert!(!val.is_dma_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_gmac();
        assert!(val.is_gmac_enabled());
        assert_eq!(val.0, 0x0000_0008);
        val = val.disable_gmac();
        assert!(!val.is_gmac_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_dvp();
        assert!(val.is_dvp_enabled());
        assert_eq!(val.0, 0x0000_0010);
        val = val.disable_dvp();
        assert!(!val.is_dvp_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_ve();
        assert!(val.is_ve_enabled());
        assert_eq!(val.0, 0x0000_0020);
        val = val.disable_ve();
        assert!(!val.is_ve_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_ge();
        assert!(val.is_ge_enabled());
        assert_eq!(val.0, 0x0000_0040);
        val = val.disable_ge();
        assert!(!val.is_ge_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_de();
        assert!(val.is_de_enabled());
        assert_eq!(val.0, 0x0000_0080);
        val = val.disable_de();
        assert!(!val.is_de_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_ce();
        assert!(val.is_ce_enabled());
        assert_eq!(val.0, 0x0000_0100);
        val = val.disable_ce();
        assert!(!val.is_ce_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_uart();
        assert!(val.is_uart_enabled());
        assert_eq!(val.0, 0x0000_0400);
        val = val.disable_uart();
        assert!(!val.is_uart_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_sd();
        assert!(val.is_sd_enabled());
        assert_eq!(val.0, 0x0000_0200);
        val = val.disable_sd();
        assert!(!val.is_sd_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_usb();
        assert!(val.is_usb_enabled());
        assert_eq!(val.0, 0x0000_0800);
        val = val.disable_usb();
        assert!(!val.is_usb_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_spi();
        assert!(val.is_spi_enabled());
        assert_eq!(val.0, 0x0000_1000);
        val = val.disable_spi();
        assert!(!val.is_spi_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_mipi();
        assert!(val.is_mipi_enabled());
        assert_eq!(val.0, 0x0000_2000);
        val = val.disable_mipi();
        assert!(!val.is_mipi_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_audio();
        assert!(val.is_audio_enabled());
        assert_eq!(val.0, 0x0000_8000);
        val = val.disable_audio();
        assert!(!val.is_audio_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_sdfm();
        assert!(val.is_sdfm_enabled());
        assert_eq!(val.0, 0x0001_0000);
        val = val.disable_sdfm();
        assert!(!val.is_sdfm_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_xspi();
        assert!(val.is_xspi_enabled());
        assert_eq!(val.0, 0x0002_0000);
        val = val.disable_xspi();
        assert!(!val.is_xspi_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_sram_map_config_functions() {
        let mut val = SramMapConfig(0x0);

        let area_test_cases = [
            (AxiMatS0Area::Area0, 0x0000_0100),
            (AxiMatS0Area::Area1, 0x0000_0200),
            (AxiMatS0Area::Area2, 0x0000_0400),
            (AxiMatS0Area::Area3, 0x0000_0800),
            (AxiMatS0Area::Area4, 0x0000_1000),
            (AxiMatS0Area::Area5, 0x0000_2000),
            (AxiMatS0Area::Area6, 0x0000_4000),
            (AxiMatS0Area::Area7, 0x0000_8000),
        ];

        for (area, reg_val) in area_test_cases {
            val = val.enable_axi_mat_s0_area(area);
            assert!(val.is_axi_mat_s0_area_enabled(area));
            assert_eq!(val.0, reg_val);
            val = val.disable_axi_mat_s0_area(area);
            assert!(!val.is_axi_mat_s0_area_enabled(area));
            assert_eq!(val.0, 0x0000_0000);
        }

        val = SramMapConfig(0x0);
        for i in 0..7 {
            let (size, reg_val) = match i {
                0 => (AxiMatS1Size::Size0Kb, 0x0000_0000),
                1 => (AxiMatS1Size::Size128Kb, 0x0000_0010),
                2 => (AxiMatS1Size::Size256Kb, 0x0000_0020),
                3 => (AxiMatS1Size::Size384Kb, 0x0000_0030),
                4 => (AxiMatS1Size::Size512Kb, 0x0000_0040),
                5 => (AxiMatS1Size::Size640Kb, 0x0000_0050),
                6 => (AxiMatS1Size::Size768Kb, 0x0000_0060),
                _ => continue,
            };

            val = val.set_axi_mat_s1_size(size);
            assert_eq!(val.axi_mat_s1_size(), size);
            assert_eq!(val.0, reg_val);
        }

        val = SramMapConfig(0x0).set_cpu_tcm_sram_aclk_gate(true);
        assert!(val.cpu_tcm_sram_aclk_gate());
        assert_eq!(val.0, 0x0000_0002);
        val = val.set_cpu_tcm_sram_aclk_gate(false);
        assert!(!val.cpu_tcm_sram_aclk_gate());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_cpu_tcm();
        assert!(val.is_cpu_tcm_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_cpu_tcm();
        assert!(!val.is_cpu_tcm_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_flash_config_functions() {
        let mut val = FlashConfig(0x0);

        for i in 0..6 {
            let (map, reg_val) = match i {
                0 => (FlashIomap012::WpSoCs, 0x0000_0000),
                1 => (FlashIomap012::WpCsSo, 0x0000_1000),
                2 => (FlashIomap012::SoWpCs, 0x0000_2000),
                3 => (FlashIomap012::SoCsWp, 0x0000_3000),
                4 => (FlashIomap012::CsWpSo, 0x0000_4000),
                5 => (FlashIomap012::CsSoWp, 0x0000_5000),
                _ => continue,
            };

            val = val.set_flash_iomap_012(map);
            assert_eq!(val.flash_iomap_012(), map);
            assert_eq!(val.0, reg_val);
        }

        val = FlashConfig(0x0);
        for i in 0..6 {
            let (map, reg_val) = match i {
                0 => (FlashIomap345::HoldSclkSi, 0x0000_0000),
                1 => (FlashIomap345::HoldSiSclk, 0x0000_0100),
                2 => (FlashIomap345::SclkHoldSi, 0x0000_0200),
                3 => (FlashIomap345::SclkSiHold, 0x0000_0300),
                4 => (FlashIomap345::SiHoldSclk, 0x0000_0400),
                5 => (FlashIomap345::SiSclkHold, 0x0000_0500),
                _ => continue,
            };

            val = val.set_flash_iomap_345(map);
            assert_eq!(val.flash_iomap_345(), map);
            assert_eq!(val.0, reg_val);
        }

        val = FlashConfig(0x0);
        for i in 0..4 {
            let (sel, reg_val) = match i {
                0 => (FlashSrcSel::None, 0x0000_0000),
                1 => (FlashSrcSel::Pin, 0x0000_0001),
                2 => (FlashSrcSel::Spi0, 0x0000_0002),
                3 => (FlashSrcSel::Spi1, 0x0000_0003),
                _ => continue,
            };

            val = val.set_flash_src_sel(sel);
            assert_eq!(val.flash_src_sel(), sel);
            assert_eq!(val.0, reg_val);
        }
    }

    #[test]
    fn struct_encoder_config_functions() {
        let mut val = EncoderConfig(0x0);

        for i in 0..4 {
            let (sel, reg_val) = match i {
                0 => (Enc0Sel::Qep0, 0x0000_0000),
                1 => (Enc0Sel::Edat0, 0x0000_0001),
                2 => (Enc0Sel::Ta0, 0x0000_0002),
                3 => (Enc0Sel::Bis0, 0x0000_0003),
                _ => continue,
            };

            val = val.set_enc0_sel(sel);
            assert_eq!(val.enc0_sel(), sel);
            assert_eq!(val.0, reg_val);
        }

        val = EncoderConfig(0x0);
        for i in 0..4 {
            let (sel, reg_val) = match i {
                0 => (Enc1Sel::Qep1, 0x0000_0000),
                1 => (Enc1Sel::Edat1, 0x0001_0000),
                2 => (Enc1Sel::Ta1, 0x0002_0000),
                3 => (Enc1Sel::Bis1, 0x0003_0000),
                _ => continue,
            };

            val = val.set_enc1_sel(sel);
            assert_eq!(val.enc1_sel(), sel);
            assert_eq!(val.0, reg_val);
        }
    }

    #[test]
    fn struct_usb0_config_functions() {
        let mut val = Usb0Config(0x0);

        val = val.set_drd_mode(DrdMode::Device);
        assert_eq!(val.drd_mode(), DrdMode::Device);
        assert_eq!(val.0, 0x0000_0001);

        val = val.set_drd_mode(DrdMode::Host);
        assert_eq!(val.drd_mode(), DrdMode::Host);
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_emac_config_functions() {
        let mut val = EmacConfig(0x0);

        val = val.enable_refclk_inv();
        assert!(val.is_refclk_inv_enabled());
        assert_eq!(val.0, 0x2000_0000);
        val = val.disable_refclk_inv();
        assert!(!val.is_refclk_inv_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_refclk_dly_chain_sel(0x1F);
        assert_eq!(val.refclk_dly_chain_sel(), 0x1F);
        assert_eq!(val.0, 0x1F00_0000);

        val = EmacConfig(0x0).enable_rxclk_inv();
        assert!(val.is_rxclk_inv_enabled());
        assert_eq!(val.0, 0x0080_0000);
        val = val.disable_rxclk_inv();
        assert!(!val.is_rxclk_inv_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_rxclk_dly_sel(0x1F);
        assert_eq!(val.rxclk_dly_sel(), 0x1F);
        assert_eq!(val.0, 0x007C_0000);

        val = EmacConfig(0x0).enable_txclk_inv();
        assert!(val.is_txclk_inv_enabled());
        assert_eq!(val.0, 0x0002_0000);
        val = val.disable_txclk_inv();
        assert!(!val.is_txclk_inv_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_txclk_dly_chain_sel(0x1F);
        assert_eq!(val.txclk_dly_chain_sel(), 0x1F);
        assert_eq!(val.0, 0x0001_F000);

        val = EmacConfig(0x0).set_sw_txclk_div2(0xF);
        assert_eq!(val.sw_txclk_div2(), 0xF);
        assert_eq!(val.0, 0x0000_0F00);

        val = EmacConfig(0x0).set_sw_txclk_div1(0xF);
        assert_eq!(val.sw_txclk_div1(), 0xF);
        assert_eq!(val.0, 0x0000_00F0);

        val = EmacConfig(0x0).enable_sw_txclk_div();
        assert!(val.is_sw_txclk_div_enabled());
        assert_eq!(val.0, 0x0000_0004);
        val = val.disable_sw_txclk_div();
        assert!(!val.is_sw_txclk_div_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_rmii_extclk_sel(RmiiExtclkSel::Extclk);
        assert_eq!(val.rmii_extclk_sel(), RmiiExtclkSel::Extclk);
        assert_eq!(val.0, 0x0000_0002);
        val = val.set_rmii_extclk_sel(RmiiExtclkSel::Intclk);
        assert_eq!(val.rmii_extclk_sel(), RmiiExtclkSel::Intclk);
        assert_eq!(val.0, 0x0000_0000);
    }

    test_should_panic!(
        (
            test_emac_config_set_refclk_dly_chain_sel_panic,
            EmacConfig(0x0).set_refclk_dly_chain_sel(0x20),
            "Reference clock delay chain selection out of range (expected 0..=31)"
        ),
        (
            test_emac_config_set_rxclk_dly_sel_panic,
            EmacConfig(0x0).set_rxclk_dly_sel(0x20),
            "Receive clock delay selection out of range (expected 0..=31)"
        ),
        (
            test_emac_config_set_txclk_dly_chain_sel_panic,
            EmacConfig(0x0).set_txclk_dly_chain_sel(0x20),
            "Transmit clock delay chain selection out of range (expected 0..=31)"
        ),
        (
            test_emac_config_set_sw_txclk_div2_panic,
            EmacConfig(0x0).set_sw_txclk_div2(0x10),
            "Software transmit clock divider 2 out of range (expected 0..=15)"
        ),
        (
            test_emac_config_set_sw_txclk_div1_panic,
            EmacConfig(0x0).set_sw_txclk_div1(0x10),
            "Software transmit clock divider 1 out of range (expected 0..=15)"
        ),
    );

    #[test]
    fn struct_atb_cmu_ana_top_functions() {
        let mut val = AtbCmuAnaTop(0x0);

        for i in 0..29 {
            let (sel, reg_val) = match i {
                0 => (CmuAtbSel::Disable, 0x0000_0000),
                1 => (CmuAtbSel::VbiasVbp, 0x0000_0001),
                2 => (CmuAtbSel::VbiasVbpc, 0x0000_0002),
                3 => (CmuAtbSel::VbiasVbn, 0x0000_0003),
                4 => (CmuAtbSel::VbiasVbnc, 0x0000_0004),
                5 => (CmuAtbSel::VbiasVfbb, 0x0000_0005),
                6 => (CmuAtbSel::LdoDvDv, 0x0000_0006),
                7 => (CmuAtbSel::LdoDvVbpc, 0x0000_0007),
                8 => (CmuAtbSel::LdoDvVbp, 0x0000_0008),
                9 => (CmuAtbSel::LdoDvVbnc, 0x0000_0009),
                10 => (CmuAtbSel::LdoDvVbn, 0x0000_000A),
                11 => (CmuAtbSel::LdoAvInt0Av, 0x0000_000B),
                12 => (CmuAtbSel::LdoAvInt0Vbpc, 0x0000_000C),
                13 => (CmuAtbSel::LdoAvInt0Vbp, 0x0000_000D),
                14 => (CmuAtbSel::LdoAvInt0Vbnc, 0x0000_000E),
                15 => (CmuAtbSel::LdoAvInt0Vbn, 0x0000_000F),
                16 => (CmuAtbSel::LdoAvInt1Av, 0x0000_0010),
                17 => (CmuAtbSel::LdoAvFra0Av, 0x0000_0011),
                18 => (CmuAtbSel::LdoAvFra0Vbpc, 0x0000_0012),
                19 => (CmuAtbSel::LdoAvFra0Vbp, 0x0000_0013),
                20 => (CmuAtbSel::LdoAvFra0Vbnc, 0x0000_0014),
                21 => (CmuAtbSel::LdoAvFra0Vbn, 0x0000_0015),
                22 => (CmuAtbSel::LdoAvFra1Av, 0x0000_0016),
                23 => (CmuAtbSel::LdoAvFra2Av, 0x0000_0017),
                24 => (CmuAtbSel::Int0IbCp, 0x0000_0018),
                25 => (CmuAtbSel::Int1IbCp, 0x0000_0019),
                26 => (CmuAtbSel::Fra0IbCp, 0x0000_001A),
                27 => (CmuAtbSel::Fra1IbCp, 0x0000_001B),
                28 => (CmuAtbSel::Fra2IbCp, 0x0000_001C),
                _ => unreachable!(),
            };

            val = val.set_cmu_atbsel(sel);
            assert_eq!(val.cmu_atbsel(), sel);
            assert_eq!(val.0, reg_val);
        }
    }

    #[test]
    fn struct_atb_dll_top_c_functions() {
        let mut val = AtbDllTopC(0x0);

        val = val.enable_dll0_atb();
        assert!(val.is_dll0_atb_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_dll0_atb();
        assert!(!val.is_dll0_atb_enabled());
        assert_eq!(val.0, 0x0000_0000);

        for i in 0..7 {
            let (sel, reg_val) = match i {
                0 => (DllAtbSel::LockTest, 0x0000_0000),
                1 => (DllAtbSel::Vbn, 0x0000_0010),
                2 => (DllAtbSel::Vctrl, 0x0000_0020),
                3 => (DllAtbSel::Vbp, 0x0000_0030),
                4 => (DllAtbSel::Dvdd, 0x0000_0040),
                5 => (DllAtbSel::LdoVfb, 0x0000_0050),
                6 => (DllAtbSel::Icp, 0x0000_0060),
                _ => continue,
            };

            val = val.set_dll0_atb_sel(sel);
            assert_eq!(val.dll0_atb_sel(), sel);
            assert_eq!(val.0, reg_val);
        }

        val = AtbDllTopC(0x0).enable_dll1_atb();
        assert!(val.is_dll1_atb_enabled());
        assert_eq!(val.0, 0x0000_0100);
        val = val.disable_dll1_atb();
        assert!(!val.is_dll1_atb_enabled());
        assert_eq!(val.0, 0x0000_0000);

        for i in 0..7 {
            let (sel, reg_val) = match i {
                0 => (DllAtbSel::LockTest, 0x0000_0000),
                1 => (DllAtbSel::Vbn, 0x0000_1000),
                2 => (DllAtbSel::Vctrl, 0x0000_2000),
                3 => (DllAtbSel::Vbp, 0x0000_3000),
                4 => (DllAtbSel::Dvdd, 0x0000_4000),
                5 => (DllAtbSel::LdoVfb, 0x0000_5000),
                6 => (DllAtbSel::Icp, 0x0000_6000),
                _ => continue,
            };

            val = val.set_dll1_atb_sel(sel);
            assert_eq!(val.dll1_atb_sel(), sel);
            assert_eq!(val.0, reg_val);
        }
    }

    #[test]
    fn struct_atb_gpadc_functions() {
        let mut val = AtbGpadc(0x0);

        for i in 0..4 {
            let (sel, reg_val) = match i {
                0 => (AtbSel0::RtpVin, 0x0000_0000),
                1 => (AtbSel0::Vrefp, 0x0000_0010),
                2 => (AtbSel0::Vrefn, 0x0000_0020),
                3 => (AtbSel0::RtpTouch, 0x0000_0030),
                _ => continue,
            };

            val = val.set_atb_sel(sel);
            assert_eq!(val.atb_sel(), sel);
            assert_eq!(val.0, reg_val);
        }

        val = AtbGpadc(0x0).enable_atb();
        assert!(val.is_atb_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_atb();
        assert!(!val.is_atb_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_atb_mipi_dphy_functions() {
        let mut val = AtbMipiDphy(0x0);

        for i in 0..4 {
            let (sel, reg_val) = match i {
                0 => (AtbSel1::VresAtb, 0x0000_0000),
                1 => (AtbSel1::Ipn5uAtb, 0x0000_0010),
                2 => (AtbSel1::V0p4Atb, 0x0000_0020),
                3 => (AtbSel1::Vpck, 0x0000_0030),
                _ => continue,
            };

            val = val.set_atb_sel(sel);
            assert_eq!(val.atb_sel(), sel);
            assert_eq!(val.0, reg_val);
        }

        val = AtbMipiDphy(0x0).enable_atb();
        assert!(val.is_atb_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_atb();
        assert!(!val.is_atb_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_atb_rtc_ana_top_functions() {
        let mut val = AtbRtcAnaTop(0x0);

        for i in 0..4 {
            let (sel, reg_val) = match i {
                0 => (AtbSel2::VrefOut, 0x0000_0000),
                1 => (AtbSel2::Vosc, 0x0000_0010),
                2 => (AtbSel2::IbpIbas, 0x0000_0020),
                3 => (AtbSel2::IbpVdet, 0x0000_0030),
                _ => continue,
            };

            val = val.set_atb_sel(sel);
            assert_eq!(val.atb_sel(), sel);
            assert_eq!(val.0, reg_val);
        }

        val = AtbRtcAnaTop(0x0).enable_atb();
        assert!(val.is_atb_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_atb();
        assert!(!val.is_atb_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_atb_usb_pll_afe_functions() {
        let mut val = AtbUsbPllAfe(0x0);

        for i in 0..17 {
            let (sel, reg_val) = match i {
                0 => (SubpllAtb::Disable, 0x0000_0000),
                1 => (SubpllAtb::VbiasVbp, 0x0000_0001),
                2 => (SubpllAtb::VbiasVbpc, 0x0000_0002),
                3 => (SubpllAtb::VbiasVbn, 0x0000_0003),
                4 => (SubpllAtb::VbiasVbnc, 0x0000_0004),
                5 => (SubpllAtb::VbiasVfbb, 0x0000_0005),
                6 => (SubpllAtb::LdoDvDv, 0x0000_0006),
                7 => (SubpllAtb::LdoDvVbpc, 0x0000_0007),
                8 => (SubpllAtb::LdoDvVbp, 0x0000_0008),
                9 => (SubpllAtb::LdoDvVbnc, 0x0000_0009),
                10 => (SubpllAtb::LdoDvVbn, 0x0000_000A),
                11 => (SubpllAtb::LdoAvInt0Av, 0x0000_000B),
                12 => (SubpllAtb::LdoAvInt0Vbpc, 0x0000_000C),
                13 => (SubpllAtb::LdoAvInt0Vbp, 0x0000_000D),
                14 => (SubpllAtb::LdoAvInt0Vbnc, 0x0000_000E),
                15 => (SubpllAtb::LdoAvInt0Vbn, 0x0000_000F),
                16 => (SubpllAtb::Int0IbCp, 0x0000_0010),
                _ => continue,
            };

            val = val.set_subpll_atb(sel);
            assert_eq!(val.subpll_atb(), sel);
            assert_eq!(val.0, reg_val);
        }
    }

    #[test]
    fn struct_atb_usb_phy_afe_functions() {
        let mut val = AtbUsbPhyAfe(0x0);

        val = val.enable_atb_op();
        assert!(val.is_atb_op_enabled());
        assert_eq!(val.0, 0x0000_0002);
        val = val.disable_atb_op();
        assert!(!val.is_atb_op_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_avdd_atb();
        assert!(val.is_avdd_atb_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_avdd_atb();
        assert!(!val.is_avdd_atb_enabled());
        assert_eq!(val.0, 0x0000_0000);

        for i in 0..4 {
            let (sel, reg_val) = match i {
                0 => (TxTestSel::AtbOpAvddAtb, 0x0000_0000),
                1 => (TxTestSel::Vref, 0x0000_0010),
                2 => (TxTestSel::Vreg, 0x0000_0020),
                3 => (TxTestSel::Rtune, 0x0000_0030),
                _ => continue,
            };

            val = val.set_tx_test_sel(sel);
            assert_eq!(val.tx_test_sel(), sel);
            assert_eq!(val.0, reg_val);
        }

        val = AtbUsbPhyAfe(0x0);
        let sqrx_test_cases = [
            (SqrxTestSel::SqOffsetP, 0x0000_0100),
            (SqrxTestSel::SqOffsetM, 0x0000_0200),
            (SqrxTestSel::Dp, 0x0000_0800),
            (SqrxTestSel::Dm, 0x0000_1000),
        ];

        for (sel, reg_val) in sqrx_test_cases {
            val = val.enable_sqrx(sel);
            assert!(val.is_sqrx_enabled(sel));
            assert_eq!(val.0, reg_val);
            val = val.disable_sqrx(sel);
            assert!(!val.is_sqrx_enabled(sel));
            assert_eq!(val.0, 0x0000_0000);
        }
    }
}
