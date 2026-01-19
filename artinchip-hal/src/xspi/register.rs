//! XSPI register blocks and registers.

use embedded_hal::spi::{Phase, Polarity};
use volatile_register::{RO, RW};

/// eXpanded Serial Peripheral Interface (XSPI) registers.
#[repr(C)]
pub struct RegisterBlock {
    /// XSPI control register (`XSPI_CTL`).
    #[doc(alias = "XSPI_CTL")]
    pub ctrl: RW<Control>,
    /// XSPI clock register (`XSPI_CLK`).
    #[doc(alias = "XSPI_CLK")]
    pub clk: RW<Clock>,
    /// XSPI transfer control register (`XSPI_TCR`).
    #[doc(alias = "XSPI_TCR")]
    pub trans_ctrl: RW<TransControl>,
    /// XSPI status register (`XSPI_STAS`).
    #[doc(alias = "XSPI_STAS")]
    pub status: RO<Status>,
    /// XSPI chip select 0 control register (`XSPI_CS0_CTL`).
    #[doc(alias = "XSPI_CS0_CTL")]
    pub cs0_ctrl: RW<CsControl>,
    /// XSPI chip select 0 dll control register (`XSPI_CS0_DCTL`).
    #[doc(alias = "XSPI_CS0_DCTL")]
    pub cs0_dll_ctrl: RW<CsDllControl>,
    /// XSPI chip select 1 control register (`XSPI_CS1_CTL`).
    #[doc(alias = "XSPI_CS1_CTL")]
    pub cs1_ctrl: RW<CsControl>,
    /// XSPI chip select 1 dll control register (`XSPI_CS1_DCTL`).
    #[doc(alias = "XSPI_CS1_DCTL")]
    pub cs1_dll_ctrl: RW<CsDllControl>,
    /// XSPI interrupt enable register (`XSPI_IER`).
    #[doc(alias = "XSPI_IER")]
    pub int_en: RW<IntEnable>,
    /// XSPI interrupt status register (`XSPI_ISR`).
    #[doc(alias = "XSPI_ISR")]
    pub int_status: RW<IntStatus>,
    /// XSPI fifo control register (`XSPI_FCR`).
    #[doc(alias = "XSPI_FCR")]
    pub fifo_ctrl: RW<FifoControl>,
    /// XSPI fifo status register (`XSPI_FSR`).
    #[doc(alias = "XSPI_FSR")]
    pub fifo_status: RO<FifoStatus>,
    /// XSPI start register (`XSPI_START`).
    #[doc(alias = "XSPI_START")]
    pub start: RW<Start>,
    /// XSPI address configuration register (`XSPI_ADDR`).
    #[doc(alias = "XSPI_ADDR")]
    pub addr: RW<u32>,
    /// XSPI format configuration register (`XSPI_FMR`).
    #[doc(alias = "XSPI_FMR")]
    pub fmt_config: RW<Format>,
    _reserved0: [u8; 0x4],
    /// XSPI burst type register (`XSPI_BTR`).
    #[doc(alias = "XSPI_BTR")]
    pub burst_type: RW<BurstType>,
    /// XSPI read command control register (`XSPI_RCC`).
    #[doc(alias = "XSPI_RCC")]
    pub rd_cmd_ctrl: RW<RdCmdControl>,
    /// XSPI dma mode control register (`XSPI_NDMA_MODE_CTL`).
    #[doc(alias = "XSPI_NDMA_MODE_CTL")]
    pub dma_mode_ctrl: RW<DmaModeControl>,
    _reserved1: [u8; 0x4],
    /// XSPI timeout configuration register (`XSPI_TO`).
    #[doc(alias = "XSPI_TO")]
    pub timeout_config: RW<u32>,
    /// XSPI lock configuration register (`XSPI_LCKCR`).
    #[doc(alias = "XSPI_LCKCR")]
    pub lock_config: RW<LockConfig>,
    /// XSPI lookup table update register (`XSPI_LUT_UP`).
    #[doc(alias = "XSPI_LUT_UP")]
    pub lut_up: RW<LutUp>,
    _reserved2: [u8; 0x4],
    /// XSPI channel 0 sequence register (`XSPI_CS0_SEQUENCE`).
    #[doc(alias = "XSPI_CS0_SEQUENCE")]
    pub cs0_sequence: RW<CsSequence>,
    /// XSPI channel 1 sequence register (`XSPI_CS1_SEQUENCE`).
    #[doc(alias = "XSPI_CS1_SEQUENCE")]
    pub cs1_sequence: RW<CsSequence>,
    /// XSPI io control register (`XSPI_IO_CTL`).
    #[doc(alias = "XSPI_IO_CTL")]
    pub io_ctrl: RW<IoControl>,
    _reserved3: [u8; 0x4],
    /// XSPI chip select 0 io configuration 1 register (`XSPI_CS0_IOCFG1`).
    #[doc(alias = "XSPI_CS0_IOCFG1")]
    pub cs0_io_cfg1: RW<CsIoConfig1>,
    /// XSPI chip select 0 io configuration 2 register (`XSPI_CS0_IOCFG2`).
    #[doc(alias = "XSPI_CS0_IOCFG2")]
    pub cs0_io_cfg2: RW<CsIoConfig2>,
    /// XSPI chip select 0 io configuration 3 register (`XSPI_CS0_IOCFG3`).
    #[doc(alias = "XSPI_CS0_IOCFG3")]
    pub cs0_io_cfg3: RW<CsIoConfig3>,
    /// XSPI chip select 0 io configuration 4 register (`XSPI_CS0_IOCFG4`).
    #[doc(alias = "XSPI_CS0_IOCFG4")]
    pub cs0_io_cfg4: RW<CsIoConfig4>,
    /// XSPI chip select 1 io configuration 1 register (`XSPI_CS1_IOCFG1`).
    #[doc(alias = "XSPI_CS1_IOCFG1")]
    pub cs1_io_cfg1: RW<CsIoConfig1>,
    /// XSPI chip select 1 io configuration 2 register (`XSPI_CS1_IOCFG2`).
    #[doc(alias = "XSPI_CS1_IOCFG2")]
    pub cs1_io_cfg2: RW<CsIoConfig2>,
    /// XSPI chip select 1 io configuration 3 register (`XSPI_CS1_IOCFG3`).
    #[doc(alias = "XSPI_CS1_IOCFG3")]
    pub cs1_io_cfg3: RW<CsIoConfig3>,
    /// XSPI chip select 1 io configuration 4 register (`XSPI_CS1_IOCFG4`).
    #[doc(alias = "XSPI_CS1_IOCFG4")]
    pub cs1_io_cfg4: RW<CsIoConfig4>,
    /// XSPI training configuration register (`XSPI_TRAINING_CFG`).
    #[doc(alias = "XSPI_TRAINING_CFG")]
    pub training_cfg: RW<TrainingConfig>,
    /// XSPI training pattern register (`XSPI_TRAINING_PATTERN`).
    #[doc(alias = "XSPI_TRAINING_PATTERN")]
    pub training_pattern: RW<u32>,
    _reserved4: [u8; 0x68],
    /// XSPI lookup tables (`XSPI_LUTn`).
    #[doc(alias = "XSPI_LUTn")]
    pub luts: [RW<Lut>; 32],
    _reserved5: [u8; 0x80],
    /// XSPI transmit data register (`XSPI_TDR`).
    #[doc(alias = "XSPI_TDR")]
    pub tx_data: RW<u32>,
    _reserved6: [u8; 0xFC],
    /// XSPI receive data register (`XSPI_RDR`).
    #[doc(alias = "XSPI_RDR")]
    pub rx_data: RO<u32>,
    _reserved7: [u8; 0xFC],
    /// XSPI debug register (`XSPI_DEBUG`).
    #[doc(alias = "XSPI_DEBUG")]
    pub debug: RW<u32>,
    /// XSPI debug select register (`XSPI_DEBUG_SEL`).
    #[doc(alias = "XSPI_DEBUG_SEL")]
    pub debug_sel: RW<DebugSel>,
    _reserved8: [u8; 0xBF4],
    /// XSPI version register (`VERSION`).
    #[doc(alias = "VERSION")]
    pub version: RO<u32>,
}

/// Column address control.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ColAddrCtrl {
    ByteAddr,
    WordAddr,
}

/// Pin width control.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PinWidth {
    X8,
    X16,
}

/// Boundary size control.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BoundarySize {
    Size2K,
    Size1K,
}

/// XSPI mode selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum XspiMode {
    Xccela,
    Hyperbus,
    OPI,
    SPI,
}

/// XSPI control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Control(u32);

impl Control {
    const AXI_ABITER_EN: u32 = 0x1 << 18;
    const COLUMN_ADDRESS_CONTROL: u32 = 0x1 << 17;
    const PIN_CTL: u32 = 0x1 << 16;
    const BOUNDARY_CONTROL: u32 = 0x3 << 13;
    const BOUNDARY_EN: u32 = 0x1 << 12;
    const RESET_EN: u32 = 0x1 << 9;
    const RESET_LEVEL: u32 = 0x1 << 8;
    const TIMEOUT_EN: u32 = 0x1 << 7;
    const PARALLEL_MODE: u32 = 0x1 << 6;
    const XSPI_MODE_SEL: u32 = 0x3 << 4;
    const AXI_WRAP_BURST_CTL: u32 = 0x1 << 3;
    const XIP_EN: u32 = 0x1 << 2;
    const IDL_LOW_POWER_EN: u32 = 0x1 << 1;
    const XSPI_EN: u32 = 0x1;

    /// Enable axi arbiter (`AXI_ABITER_EN`).
    ///
    /// Respond to AXI read requests while delaying write requests.
    #[doc(alias = "AXI_ABITER_EN")]
    #[inline]
    pub const fn enable_axi_arbiter(self) -> Self {
        Self(self.0 | Control::AXI_ABITER_EN)
    }
    /// Disable axi arbiter.
    #[inline]
    pub const fn disable_axi_arbiter(self) -> Self {
        Self(self.0 & !Control::AXI_ABITER_EN)
    }
    /// Check if axi arbiter is enabled.
    #[inline]
    pub const fn is_axi_arbiter_enabled(self) -> bool {
        self.0 & Control::AXI_ABITER_EN != 0
    }
    /// Set column address control (`COLUMN_ADDRESS_CONTROL`).
    #[doc(alias = "COLUMN_ADDRESS_CONTROL")]
    #[inline]
    pub const fn set_col_addr_ctrl(self, ctrl: ColAddrCtrl) -> Self {
        Self(
            (self.0 & !Control::COLUMN_ADDRESS_CONTROL)
                | (Control::COLUMN_ADDRESS_CONTROL & ((ctrl as u32) << 17)),
        )
    }
    /// Get column address control.
    #[inline]
    pub const fn col_addr_ctrl(self) -> ColAddrCtrl {
        match (self.0 & Control::COLUMN_ADDRESS_CONTROL) >> 17 {
            0 => ColAddrCtrl::ByteAddr,
            1 => ColAddrCtrl::WordAddr,
            _ => unreachable!(),
        }
    }
    /// Set pin width control (`PIN_CTL`).
    ///
    /// When pin width is set to x16, parallel mode must be enabled.
    /// At this time, DQ[7:0] of CS0 corresponds to x16 DQ[7:0], and DQ[7:0] of CS1 corresponds to x16 DQ[15:8].
    #[doc(alias = "PIN_CTL")]
    #[inline]
    pub const fn set_pin_width(self, width: PinWidth) -> Self {
        Self((self.0 & !Control::PIN_CTL) | (Control::PIN_CTL & ((width as u32) << 16)))
    }
    /// Get pin width control.
    #[inline]
    pub const fn pin_width(self) -> PinWidth {
        match (self.0 & Control::PIN_CTL) >> 16 {
            0 => PinWidth::X8,
            1 => PinWidth::X16,
            _ => unreachable!(),
        }
    }
    /// Set boundary size control (`BOUNDARY_CONTROL`).
    ///
    /// Boundary control must be enabled for this setting to take effect.
    #[doc(alias = "BOUNDARY_CONTROL")]
    #[inline]
    pub const fn set_boundary_size(self, size: BoundarySize) -> Self {
        Self(
            (self.0 & !Control::BOUNDARY_CONTROL)
                | (Control::BOUNDARY_CONTROL & ((size as u32) << 13)),
        )
    }
    /// Get boundary size control.
    #[inline]
    pub const fn boundary_size(self) -> BoundarySize {
        match (self.0 & Control::BOUNDARY_CONTROL) >> 13 {
            0 => BoundarySize::Size2K,
            1 => BoundarySize::Size1K,
            _ => unreachable!(),
        }
    }
    /// Enable boundary control (`BOUNDARY_EN`).
    #[inline]
    pub const fn enable_boundary(self) -> Self {
        Self(self.0 | Control::BOUNDARY_EN)
    }
    /// Disable boundary control.
    #[inline]
    pub const fn disable_boundary(self) -> Self {
        Self(self.0 & !Control::BOUNDARY_EN)
    }
    /// Check if boundary control is enabled.
    #[inline]
    pub const fn is_boundary_enabled(self) -> bool {
        self.0 & Control::BOUNDARY_EN != 0
    }
    /// Enable reset function (`RESET_EN`).
    ///
    /// The DM pin functions as the RESET pin, with the `RESET_LEVEL` controlling the voltage level.
    #[inline]
    pub const fn enable_reset(self) -> Self {
        Self(self.0 | Control::RESET_EN)
    }
    /// Disable reset function.
    #[inline]
    pub const fn disable_reset(self) -> Self {
        Self(self.0 & !Control::RESET_EN)
    }
    /// Check if reset function is enabled.
    #[inline]
    pub const fn is_reset_enabled(self) -> bool {
        self.0 & Control::RESET_EN != 0
    }
    /// Set reset level (`RESET_LEVEL`).
    #[inline]
    pub const fn set_reset_level(self, high: bool) -> Self {
        if high {
            Self(self.0 | Control::RESET_LEVEL)
        } else {
            Self(self.0 & !Control::RESET_LEVEL)
        }
    }
    /// Get reset level.
    #[inline]
    pub const fn reset_level(self) -> bool {
        self.0 & Control::RESET_LEVEL != 0
    }
    /// Enable timeout (`TIMEOUT_EN`).
    #[inline]
    pub const fn enable_timeout(self) -> Self {
        Self(self.0 | Control::TIMEOUT_EN)
    }
    /// Disable timeout.
    #[inline]
    pub const fn disable_timeout(self) -> Self {
        Self(self.0 & !Control::TIMEOUT_EN)
    }
    /// Check if timeout is enabled.
    #[inline]
    pub const fn is_timeout_enabled(self) -> bool {
        self.0 & Control::TIMEOUT_EN != 0
    }
    /// Enable parallel mode (`PARALLEL_MODE`).
    #[inline]
    pub const fn enable_parallel_mode(self) -> Self {
        Self(self.0 | Control::PARALLEL_MODE)
    }
    /// Disable parallel mode.
    #[inline]
    pub const fn disable_parallel_mode(self) -> Self {
        Self(self.0 & !Control::PARALLEL_MODE)
    }
    /// Check if parallel mode is enabled.
    #[inline]
    pub const fn is_parallel_mode_enabled(self) -> bool {
        self.0 & Control::PARALLEL_MODE != 0
    }
    /// Set xspi mode selection (`XSPI_MODE_SEL`).
    #[doc(alias = "XSPI_MODE_SEL")]
    #[inline]
    pub const fn set_xspi_mode(self, mode: XspiMode) -> Self {
        Self((self.0 & !Control::XSPI_MODE_SEL) | (Control::XSPI_MODE_SEL & ((mode as u32) << 4)))
    }
    /// Get xspi mode selection.
    #[inline]
    pub const fn xspi_mode(self) -> XspiMode {
        match (self.0 & Control::XSPI_MODE_SEL) >> 4 {
            0 => XspiMode::Xccela,
            1 => XspiMode::Hyperbus,
            2 => XspiMode::OPI,
            3 => XspiMode::SPI,
            _ => unreachable!(),
        }
    }
    /// Set axi wrap burst control (`AXI_WRAP_BURST_CTL`).
    ///
    /// - 0: wrap burst.
    /// - 1: wrap burst is split into linear burst output.
    ///
    /// Wrap burst only supports wrap4.
    #[inline]
    pub const fn set_axi_wrap_burst_ctrl(self, set: bool) -> Self {
        if set {
            Self(self.0 | Control::AXI_WRAP_BURST_CTL)
        } else {
            Self(self.0 & !Control::AXI_WRAP_BURST_CTL)
        }
    }
    /// Get axi wrap burst control.
    #[inline]
    pub const fn axi_wrap_burst_ctrl(self) -> bool {
        self.0 & Control::AXI_WRAP_BURST_CTL != 0
    }
    /// Enable xip mode (`XIP_EN`).
    ///
    /// Used to control the AXI access channel.
    /// If there is AXI access, the AXI bus must not stall at this time.
    #[inline]
    pub const fn enable_xip(self) -> Self {
        Self(self.0 | Control::XIP_EN)
    }
    /// Disable xip mode.
    #[inline]
    pub const fn disable_xip(self) -> Self {
        Self(self.0 & !Control::XIP_EN)
    }
    /// Check if xip mode is enabled.
    #[inline]
    pub const fn is_xip_enabled(self) -> bool {
        self.0 & Control::XIP_EN != 0
    }
    /// Enable idle low power mode (`IDL_LOW_POWER_EN`).
    #[inline]
    pub const fn enable_idle_lp(self) -> Self {
        Self(self.0 | Control::IDL_LOW_POWER_EN)
    }
    /// Disable idle low power mode.
    #[inline]
    pub const fn disable_idle_lp(self) -> Self {
        Self(self.0 & !Control::IDL_LOW_POWER_EN)
    }
    /// Check if idle low power mode is enabled.
    #[inline]
    pub const fn is_idle_lp_enabled(self) -> bool {
        self.0 & Control::IDL_LOW_POWER_EN != 0
    }
    /// Enable xspi module (`XSPI_EN`).
    #[inline]
    pub const fn enable_xspi(self) -> Self {
        Self(self.0 | Control::XSPI_EN)
    }
    /// Disable xspi module.
    #[inline]
    pub const fn disable_xspi(self) -> Self {
        Self(self.0 & !Control::XSPI_EN)
    }
    /// Check if xspi module is enabled.
    #[inline]
    pub const fn is_xspi_enabled(self) -> bool {
        self.0 & Control::XSPI_EN != 0
    }
}

/// Clock divider selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClockDivider {
    Divider1,
    Divider2,
}

/// XSPI clock register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Clock(u32);

impl Clock {
    const CLK_SEL: u32 = 0x1 << 12;
    const CDR1_M: u32 = 0xF << 8;
    const CDR2_N: u32 = 0xFF;

    /// Set clock divider selection (`CLK_SEL`).
    #[doc(alias = "CLK_SEL")]
    #[inline]
    pub const fn set_clock_divider(self, divider: ClockDivider) -> Self {
        Self((self.0 & !Clock::CLK_SEL) | (Clock::CLK_SEL & ((divider as u32) << 12)))
    }
    /// Get clock divider selection.
    #[inline]
    pub const fn clock_divider(self) -> ClockDivider {
        match (self.0 & Clock::CLK_SEL) >> 12 {
            0 => ClockDivider::Divider1,
            1 => ClockDivider::Divider2,
            _ => unreachable!(),
        }
    }
    /// Set clock divider 1 (`CDR1_M`).
    ///
    /// SPI_CLK = source clock / (2^`CDR1_M`).
    #[doc(alias = "CDR1_M")]
    #[inline]
    pub const fn set_clk_div1(self, val: u8) -> Self {
        assert!(val < 0x10, "Clock divider 1 out of range (expected 0..=15)");
        Self((self.0 & !Clock::CDR1_M) | (Clock::CDR1_M & ((val as u32) << 8)))
    }
    /// Get clock divider 1.
    #[inline]
    pub const fn clk_div1(self) -> u8 {
        ((self.0 & Clock::CDR1_M) >> 8) as u8
    }
    /// Set clock divider 2 (`CDR2_N`).
    ///
    /// SPI_CLK = source clock / (2 * (`CDR2_N` + 1)).
    #[doc(alias = "CDR2_N")]
    #[inline]
    pub const fn set_clk_div2(self, val: u8) -> Self {
        Self((self.0 & !Clock::CDR2_N) | (Clock::CDR2_N & (val as u32)))
    }
    /// Get clock divider 2.
    #[inline]
    pub const fn clk_div2(self) -> u8 {
        (self.0 & Clock::CDR2_N) as u8
    }
}

/// DQS clock gating control.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DqsClkGating {
    Normal,
    Before,
    Delay,
    Bypass,
}

/// Dummy cycles data fill type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DummyType {
    Fill0,
    Fill1,
}

/// CS level.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CsLevel {
    Low,
    High,
}

/// CS control mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CsCtrlMode {
    SpiController,
    Software,
}

/// CS selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CsSel {
    Cs0,
    Cs1,
}

/// CS polarity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CsPolarity {
    /// CS is high when idle.
    IdleHigh,
    /// CS is low when idle.
    IdleLow,
}

/// Dummy cycles data fill type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DummyFillType {
    Fill0,
    Fill1,
}

/// XSPI transfer control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TransControl(u32);

impl TransControl {
    const OPI_HOLD_EX: u32 = 0xF << 28;
    const DQS_CLK_GATING_CTL: u32 = 0xF << 24;
    const CS_RD_HOLD_CTL: u32 = 0xF << 20;
    const CS_WR_HOLD_CTL: u32 = 0xF << 16;
    const CS_SETUP_CTL: u32 = 0xF << 12;
    const JUMP_INS_EN: u32 = 0x1 << 11;
    const DUMMY_TYPE: u32 = 0x1 << 8;
    const CS_LEVEL: u32 = 0x1 << 7;
    const CS_OWNER: u32 = 0x1 << 6;
    const CS_SEL: u32 = 0x1 << 4;
    const CS_POL: u32 = 0x1 << 2;
    const CPOL: u32 = 0x1 << 1;
    const CPHA: u32 = 0x1;

    /// Set opi hold extension (`OPI_HOLD_EX`).
    #[doc(alias = "OPI_HOLD_EX")]
    #[inline]
    pub const fn set_opi_hold_ex(self, val: u8) -> Self {
        assert!(
            val < 0x10,
            "OPI hold extension out of range (expected 0..=15)"
        );
        Self((self.0 & !Self::OPI_HOLD_EX) | (Self::OPI_HOLD_EX & ((val as u32) << 28)))
    }
    /// Get opi hold extension.
    #[inline]
    pub const fn opi_hold_ex(self) -> u8 {
        ((self.0 & Self::OPI_HOLD_EX) >> 28) as u8
    }
    /// Set dqs clock gating control (`DQS_CLK_GATING_CTL`).
    #[doc(alias = "DQS_CLK_GATING_CTL")]
    #[inline]
    pub const fn set_dqs_clk_gating(self, gating: DqsClkGating) -> Self {
        Self(
            (self.0 & !Self::DQS_CLK_GATING_CTL)
                | (Self::DQS_CLK_GATING_CTL & ((gating as u32) << 24)),
        )
    }
    /// Get dqs clock gating control.
    #[inline]
    pub const fn dqs_clk_gating(self) -> DqsClkGating {
        match (self.0 & Self::DQS_CLK_GATING_CTL) >> 24 {
            0 => DqsClkGating::Normal,
            1 => DqsClkGating::Before,
            2 => DqsClkGating::Delay,
            3 => DqsClkGating::Bypass,
            _ => unreachable!(),
        }
    }
    /// Set cs read hold control (`CS_RD_HOLD_CTL`).
    ///
    /// CS read hold time indicates the minimum time CS must remain invalid between two command operations.
    /// N represents n cycles.
    /// Cannot be configured when `XSPI_BUSY` is 1.
    #[doc(alias = "CS_RD_HOLD_CTL")]
    #[inline]
    pub const fn set_cs_rd_hold(self, val: u8) -> Self {
        assert!(
            val < 0x10,
            "CS read hold time out of range (expected 0..=15)"
        );
        Self((self.0 & !Self::CS_RD_HOLD_CTL) | (Self::CS_RD_HOLD_CTL & ((val as u32) << 20)))
    }
    /// Get cs read hold control.
    #[inline]
    pub const fn cs_rd_hold(self) -> u8 {
        ((self.0 & Self::CS_RD_HOLD_CTL) >> 20) as u8
    }
    /// Set cs write hold control (`CS_WR_HOLD_CTL`).
    ///
    /// CS write hold time indicates the minimum time CS must remain invalid between two command operations.
    /// N represents n cycles.
    /// Cannot be configured when `XSPI_BUSY` is 1.
    #[doc(alias = "CS_WR_HOLD_CTL")]
    #[inline]
    pub const fn set_cs_wr_hold(self, val: u8) -> Self {
        assert!(
            val < 0x10,
            "CS write hold time out of range (expected 0..=15)"
        );
        Self((self.0 & !Self::CS_WR_HOLD_CTL) | (Self::CS_WR_HOLD_CTL & ((val as u32) << 16)))
    }
    /// Get cs write hold control.
    #[inline]
    pub const fn cs_wr_hold(self) -> u8 {
        ((self.0 & Self::CS_WR_HOLD_CTL) >> 16) as u8
    }
    /// Set cs setup control (`CS_SETUP_CTL`).
    ///
    /// After chip select is active, the setup time between data and CLK indicates when to output valid data.
    /// 0 and 1 represent 1 cycle, n (n>1) represents n XSPI_CLK cycles, and so on.
    /// Cannot be configured when `XSPI_BUSY` is 1.
    #[doc(alias = "CS_SETUP_CTL")]
    #[inline]
    pub const fn set_cs_setup(self, val: u8) -> Self {
        assert!(
            val < 0x10,
            "CS setup control out of range (expected 0..=15)"
        );
        Self((self.0 & !Self::CS_SETUP_CTL) | (Self::CS_SETUP_CTL & ((val as u32) << 12)))
    }
    /// Get cs setup control.
    #[inline]
    pub const fn cs_setup(self) -> u8 {
        ((self.0 & Self::CS_SETUP_CTL) >> 12) as u8
    }
    /// Enable jump instruction (`JUMP_INS_EN`).
    ///
    /// After XIP mode is enabled, for example, in NORFLASH continuous read data application scenarios,
    /// the LUT-defined JUMP_INS instruction can be used.
    #[doc(alias = "JUMP_INS_EN")]
    #[inline]
    pub const fn enable_jump_ins(self) -> Self {
        Self(self.0 | Self::JUMP_INS_EN)
    }
    /// Disable jump instruction.
    #[inline]
    pub const fn disable_jump_ins(self) -> Self {
        Self(self.0 & !Self::JUMP_INS_EN)
    }
    /// Check if jump instruction is enabled.
    #[inline]
    pub const fn is_jump_ins_enabled(self) -> bool {
        self.0 & Self::JUMP_INS_EN != 0
    }
    /// Set dummy cycles data fill type (`DUMMY_TYPE`).
    #[doc(alias = "DUMMY_TYPE")]
    #[inline]
    pub const fn set_dummy_fill_type(self, fill_type: DummyFillType) -> Self {
        Self((self.0 & !Self::DUMMY_TYPE) | (Self::DUMMY_TYPE & ((fill_type as u32) << 8)))
    }
    /// Get dummy cycles data fill type.
    #[inline]
    pub const fn dummy_fill_type(self) -> DummyFillType {
        match (self.0 & Self::DUMMY_TYPE) >> 8 {
            0 => DummyFillType::Fill0,
            1 => DummyFillType::Fill1,
            _ => unreachable!(),
        }
    }
    /// Set cs level (`CS_LEVEL`).
    ///
    /// In single-channel mode, this configuration is for CS0 or CS1 as determined by CS_SEL.
    /// In dual-channel mode, both CS0 and CS1 take effect simultaneously.
    /// Cannot be configured when `XSPI_BUSY` is 1.
    #[doc(alias = "CS_LEVEL")]
    #[inline]
    pub const fn set_cs_level(self, level: CsLevel) -> Self {
        Self((self.0 & !Self::CS_LEVEL) | (Self::CS_LEVEL & ((level as u32) << 7)))
    }
    /// Get cs level.
    #[inline]
    pub const fn cs_level(self) -> CsLevel {
        match (self.0 & Self::CS_LEVEL) >> 7 {
            0 => CsLevel::Low,
            1 => CsLevel::High,
            _ => unreachable!(),
        }
    }
    /// Set cs control mode (`CS_OWNER`).
    ///
    /// Usually, the CS signal is automatically sent by the controller.
    /// When this bit is set to 1, the CS_LEVEL voltage state must be manually configured.
    /// Cannot be configured when `XSPI_BUSY` is 1.
    #[doc(alias = "CS_OWNER")]
    #[inline]
    pub const fn set_cs_ctrl_mode(self, mode: CsCtrlMode) -> Self {
        Self((self.0 & !Self::CS_OWNER) | (Self::CS_OWNER & ((mode as u32) << 6)))
    }
    /// Get cs control.
    #[inline]
    pub const fn cs_ctrl_mode(self) -> CsCtrlMode {
        match (self.0 & Self::CS_OWNER) >> 6 {
            0 => CsCtrlMode::SpiController,
            1 => CsCtrlMode::Software,
            _ => unreachable!(),
        }
    }
    /// Set cs selection (`CS_SEL`).
    ///
    /// Determines whether channel 0 or channel 1 is selected.
    /// Cannot be configured when `XSPI_BUSY` is 1.
    #[doc(alias = "CS_SEL")]
    #[inline]
    pub const fn set_cs_sel(self, sel: CsSel) -> Self {
        Self((self.0 & !Self::CS_SEL) | (Self::CS_SEL & ((sel as u32) << 4)))
    }
    /// Get cs selection.
    #[inline]
    pub const fn cs_sel(self) -> CsSel {
        match (self.0 & Self::CS_SEL) >> 4 {
            0 => CsSel::Cs0,
            1 => CsSel::Cs1,
            _ => unreachable!(),
        }
    }
    /// Set cs polarity (`CS_POL`).
    ///
    /// Cannot be configured when `XSPI_BUSY` is 1.
    #[doc(alias = "CS_POL")]
    #[inline]
    pub const fn set_cs_pol(self, pol: CsPolarity) -> Self {
        Self((self.0 & !Self::CS_POL) | (Self::CS_POL & ((pol as u32) << 2)))
    }
    /// Get cs polarity.
    #[inline]
    pub const fn cs_pol(self) -> CsPolarity {
        match (self.0 & Self::CS_POL) >> 2 {
            0 => CsPolarity::IdleHigh,
            1 => CsPolarity::IdleLow,
            _ => unreachable!(),
        }
    }
    /// Set clock polarity control (`CPOL`).
    ///
    /// Cannot be configured when `XSPI_BUSY` is 1.
    #[doc(alias = "CPOL")]
    #[inline]
    pub const fn set_clk_pol(self, pol: Polarity) -> Self {
        Self((self.0 & !Self::CPOL) | (Self::CPOL & ((pol as u32) << 1)))
    }
    /// Get clock polarity control.
    #[inline]
    pub const fn clk_pol(self) -> Polarity {
        match (self.0 & Self::CPOL) >> 1 {
            0 => Polarity::IdleLow,
            1 => Polarity::IdleHigh,
            _ => unreachable!(),
        }
    }
    /// Set SPI clock/data phase control (`CPHA`).
    ///
    /// Cannot be configured when `XSPI_BUSY` is 1.
    #[doc(alias = "CPHA")]
    #[inline]
    pub const fn set_clk_pha(self, pha: Phase) -> Self {
        Self((self.0 & !Self::CPHA) | (Self::CPHA & (pha as u32)))
    }
    /// Get SPI clock/data phase control.
    #[inline]
    pub const fn clk_pha(self) -> Phase {
        match self.0 & Self::CPHA {
            0 => Phase::CaptureOnFirstTransition,
            1 => Phase::CaptureOnSecondTransition,
            _ => unreachable!(),
        }
    }
}

/// XSPI status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Status(u32);

impl Status {
    const AHB_TRANS: u32 = 0x1 << 2;
    const AXI_TRANS: u32 = 0x1 << 1;
    const XSPI_BUSY: u32 = 0x1;

    /// Check if ahb transaction is in progress (`AHB_TRANS`).
    #[doc(alias = "AHB_TRANS")]
    #[inline]
    pub const fn is_ahb_transfer(self) -> bool {
        self.0 & Self::AHB_TRANS != 0
    }
    /// Check if axi transaction is in progress (`AXI_TRANS`).
    #[doc(alias = "AXI_TRANS")]
    #[inline]
    pub const fn is_axi_transfer(self) -> bool {
        self.0 & Self::AXI_TRANS != 0
    }
    /// Check if xspi is busy (`XSPI_BUSY`).
    #[doc(alias = "XSPI_BUSY")]
    #[inline]
    pub const fn is_busy(self) -> bool {
        self.0 & Self::XSPI_BUSY != 0
    }
}

/// Read phase selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RdPha {
    /// 0°.
    Deg0,
    /// 90°.
    Deg90,
    /// 180°.
    Deg180,
    /// 360°.
    Deg360,
}

/// Read data sampling control.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RdSampleCtrl {
    /// DQS input clock sampling DLL control.
    DqsDll,
    /// Internal clock sampling delay chain control.
    InternalDelayChain,
    /// DQS input clock sampling delay chain control.
    DqsDelayChain,
}

/// XSPI chip select control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CsControl(u32);

impl CsControl {
    const WR_DELAY_CHAIN_SEL: u32 = 0x1F << 24;
    const WR_PHASE_SEL: u32 = 0x1 << 21;
    const WR_DELAY_CHAIN_EN: u32 = 0x1 << 20;
    const RD_PHASE: u32 = 0x3 << 16;
    const RD_DELAY_CYCLE: u32 = 0x7 << 12;
    const RD_DELAY_CHAIN_SEL: u32 = 0x1F << 4;
    const RD_VALID_CONTROL: u32 = 0x1 << 3;
    const RD_DELAY_CHAIN_EN: u32 = 0x1 << 2;
    const RD_SAMPLE_CTL: u32 = 0x3;

    /// Set write delay chain selection (`WR_DELAY_CHAIN_SEL`).
    #[doc(alias = "WR_DELAY_CHAIN_SEL")]
    #[inline]
    pub const fn set_wr_delay_chain_sel(self, val: u8) -> Self {
        assert!(
            val < 0x20,
            "Write delay chain selection out of range (expected 0..=31)"
        );
        Self(
            (self.0 & !Self::WR_DELAY_CHAIN_SEL)
                | (Self::WR_DELAY_CHAIN_SEL & ((val as u32) << 24)),
        )
    }
    /// Get write delay chain selection.
    #[inline]
    pub const fn wr_delay_chain_sel(self) -> u8 {
        ((self.0 & Self::WR_DELAY_CHAIN_SEL) >> 24) as u8
    }
    /// Set write phase selection (`WR_PHASE_SEL`).
    ///
    /// When SPI CPHA=1:
    /// - 0: 0°.
    /// - 1: 90°.
    ///
    /// When SPI CPHA=0:
    /// - 0: 180°.
    /// - 1: 270°.
    ///
    /// Note: When using SPI quad mode, this bit must be set to 0.
    #[doc(alias = "WR_PHASE_SEL")]
    #[inline]
    pub const fn set_wr_phase_sel(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::WR_PHASE_SEL)
        } else {
            Self(self.0 & !Self::WR_PHASE_SEL)
        }
    }
    /// Get write phase selection.
    #[inline]
    pub const fn wr_phase_sel(self) -> bool {
        self.0 & Self::WR_PHASE_SEL != 0
    }
    /// Enable write delay chain (`WR_DELAY_CHAIN_EN`).
    #[inline]
    pub const fn enable_wr_delay_chain(self) -> Self {
        Self(self.0 | Self::WR_DELAY_CHAIN_EN)
    }
    /// Disable write delay chain.
    #[inline]
    pub const fn disable_wr_delay_chain(self) -> Self {
        Self(self.0 & !Self::WR_DELAY_CHAIN_EN)
    }
    /// Check if write delay chain is enabled.
    #[inline]
    pub const fn is_wr_delay_chain_enabled(self) -> bool {
        self.0 & Self::WR_DELAY_CHAIN_EN != 0
    }
    /// Set read phase (`RD_PHASE`).
    #[doc(alias = "RD_PHASE")]
    #[inline]
    pub const fn set_rd_phase(self, phase: RdPha) -> Self {
        Self((self.0 & !Self::RD_PHASE) | (Self::RD_PHASE & ((phase as u32) << 16)))
    }
    /// Get read phase.
    #[inline]
    pub const fn rd_phase(self) -> RdPha {
        match (self.0 & Self::RD_PHASE) >> 16 {
            0 => RdPha::Deg0,
            1 => RdPha::Deg90,
            2 => RdPha::Deg180,
            3 => RdPha::Deg360,
            _ => unreachable!(),
        }
    }
    /// Set read delay cycle (`RD_DELAY_CYCLE`).
    #[doc(alias = "RD_DELAY_CYCLE")]
    #[inline]
    pub const fn set_rd_delay_cycle(self, val: u8) -> Self {
        assert!(val < 0x8, "Read delay cycle out of range (expected 0..=7)");
        Self((self.0 & !Self::RD_DELAY_CYCLE) | (Self::RD_DELAY_CYCLE & ((val as u32) << 12)))
    }
    /// Get read delay cycle.
    #[inline]
    pub const fn rd_delay_cycle(self) -> u8 {
        ((self.0 & Self::RD_DELAY_CYCLE) >> 12) as u8
    }
    /// Set read delay chain selection (`RD_DELAY_CHAIN_SEL`).
    #[doc(alias = "RD_DELAY_CHAIN_SEL")]
    #[inline]
    pub const fn set_rd_delay_chain_sel(self, val: u8) -> Self {
        assert!(
            val < 0x20,
            "Read delay chain selection out of range (expected 0..=31)"
        );
        Self(
            (self.0 & !Self::RD_DELAY_CHAIN_SEL) | (Self::RD_DELAY_CHAIN_SEL & ((val as u32) << 4)),
        )
    }
    /// Get read delay chain selection.
    #[inline]
    pub const fn rd_delay_chain_sel(self) -> u8 {
        ((self.0 & Self::RD_DELAY_CHAIN_SEL) >> 4) as u8
    }
    /// Set read valid control (`RD_VALID_CONTROL`).
    ///
    /// Control timing for internal control logic accessing asynchronous FIFO during read operations.
    #[doc(alias = "RD_VALID_CONTROL")]
    #[inline]
    pub const fn set_rd_valid_control(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::RD_VALID_CONTROL)
        } else {
            Self(self.0 & !Self::RD_VALID_CONTROL)
        }
    }
    /// Get read valid control.
    #[inline]
    pub const fn rd_valid_control(self) -> bool {
        self.0 & Self::RD_VALID_CONTROL != 0
    }
    /// Enable read delay chain (`RD_DELAY_CHAIN_EN`).
    #[inline]
    pub const fn enable_rd_delay_chain(self) -> Self {
        Self(self.0 | Self::RD_DELAY_CHAIN_EN)
    }
    /// Disable read delay chain.
    #[inline]
    pub const fn disable_rd_delay_chain(self) -> Self {
        Self(self.0 & !Self::RD_DELAY_CHAIN_EN)
    }
    /// Check if read delay chain is enabled.
    #[inline]
    pub const fn is_rd_delay_chain_enabled(self) -> bool {
        self.0 & Self::RD_DELAY_CHAIN_EN != 0
    }
    /// Set read sample control (`RD_SAMPLE_CTL`).
    #[doc(alias = "RD_SAMPLE_CTL")]
    #[inline]
    pub const fn set_rd_sample_ctrl(self, ctrl: RdSampleCtrl) -> Self {
        Self((self.0 & !Self::RD_SAMPLE_CTL) | (Self::RD_SAMPLE_CTL & (ctrl as u32)))
    }
    /// Get read sample control.
    #[inline]
    pub const fn rd_sample_ctrl(self) -> RdSampleCtrl {
        match self.0 & Self::RD_SAMPLE_CTL {
            0 => RdSampleCtrl::DqsDll,
            1 => RdSampleCtrl::InternalDelayChain,
            2 | 3 => RdSampleCtrl::DqsDelayChain,
            _ => unreachable!(),
        }
    }
}

/// Bypass selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bypass {
    /// 772p-935p-1.191n.
    Range0,
    /// 2.2n-2.92n-4.1n.
    Range1,
    /// 3.63n-4.92n-6.87n.
    Range2,
    /// 5.04n-6.91n-9.66n.
    Range3,
}

/// Reference clock range.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Icp {
    /// 50~100MHz.
    Range50To100M,
    /// 100~150MHz.
    Range100To150M,
    /// 150~200MHz.
    Range150To200M,
    /// 200~266MHz.
    Range200To266M,
}

/// Input clock phase selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PhaseSel {
    /// 22.5°.
    Deg22_5,
    /// 45°.
    Deg45,
    /// 67.5°.
    Deg67_5,
    /// 90°.
    Deg90,
    /// 112.5°.
    Deg112_5,
    /// 135°.
    Deg135,
    /// 157.5°.
    Deg157_5,
    /// 180°.
    Deg180,
    /// 202.5°.
    Deg202_5,
    /// 225°.
    Deg225,
    /// 247.5°.
    Deg247_5,
    /// 270°.
    Deg270,
    /// 292.5°.
    Deg292_5,
    /// 315°.
    Deg315,
    /// 337.5°.
    Deg337_5,
    /// BYPASS.
    Bypass,
}

/// XSPI chip select dll control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CsDllControl(u32);

impl CsDllControl {
    const FORCE_LOCK: u32 = 0x1 << 29;
    const EN_ATB: u32 = 0x1 << 28;
    const REG_ATBSEL: u32 = 0x7 << 24;
    const REG_BYPASS: u32 = 0x3 << 20;
    const REG_DLY: u32 = 0x3 << 16;
    const REG_ICP: u32 = 0x3 << 12;
    const PHASE_SEL: u32 = 0xF << 8;
    const EN_LVS: u32 = 0x1 << 5;
    const EN_LDO: u32 = 0x1 << 4;
    const EN_BYPASS: u32 = 0x1 << 3;
    const EN_CP: u32 = 0x1 << 2;
    const EN_VCDL: u32 = 0x1 << 1;
    const EN_DLL: u32 = 0x1;

    /// Set force lock (`FORCE_LOCK`).
    #[doc(alias = "FORCE_LOCK")]
    #[inline]
    pub const fn set_force_lock(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::FORCE_LOCK)
        } else {
            Self(self.0 & !Self::FORCE_LOCK)
        }
    }
    /// Get force lock.
    #[inline]
    pub const fn force_lock(self) -> bool {
        self.0 & Self::FORCE_LOCK != 0
    }
    /// Enable atb (`EN_ATB`).
    #[doc(alias = "EN_ATB")]
    #[inline]
    pub const fn enable_atb(self) -> Self {
        Self(self.0 | Self::EN_ATB)
    }
    /// Disable atb.
    #[inline]
    pub const fn disable_atb(self) -> Self {
        Self(self.0 & !Self::EN_ATB)
    }
    /// Check if atb is enabled.
    #[inline]
    pub const fn is_atb_enabled(self) -> bool {
        self.0 & Self::EN_ATB != 0
    }
    /// Set atb selection (`REG_ATBSEL`).
    #[doc(alias = "REG_ATBSEL")]
    #[inline]
    pub const fn set_atbsel(self, val: u8) -> Self {
        assert!(val < 0x8, "REG_ATBSEL out of range (expected 0..=7)");
        Self((self.0 & !Self::REG_ATBSEL) | (Self::REG_ATBSEL & ((val as u32) << 24)))
    }
    /// Get atb selection.
    #[inline]
    pub const fn atbsel(self) -> u8 {
        ((self.0 & Self::REG_ATBSEL) >> 24) as u8
    }
    /// Set bypass selection (`REG_BYPASS`).
    #[doc(alias = "REG_BYPASS")]
    #[inline]
    pub const fn set_bypass(self, bypass: Bypass) -> Self {
        Self((self.0 & !Self::REG_BYPASS) | (Self::REG_BYPASS & ((bypass as u32) << 20)))
    }
    /// Get bypass selection.
    #[inline]
    pub const fn bypass(self) -> Bypass {
        match (self.0 & Self::REG_BYPASS) >> 20 {
            0 => Bypass::Range0,
            1 => Bypass::Range1,
            2 => Bypass::Range2,
            3 => Bypass::Range3,
            _ => unreachable!(),
        }
    }
    /// Set delay (`REG_DLY`).
    #[doc(alias = "REG_DLY")]
    #[inline]
    pub const fn set_delay(self, val: u8) -> Self {
        assert!(val < 0x4, "Delay out of range (expected 0..=3)");
        Self((self.0 & !Self::REG_DLY) | (Self::REG_DLY & ((val as u32) << 16)))
    }
    /// Get delay.
    #[inline]
    pub const fn delay(self) -> u8 {
        ((self.0 & Self::REG_DLY) >> 16) as u8
    }
    /// Set icp (`REG_ICP`).
    #[doc(alias = "REG_ICP")]
    #[inline]
    pub const fn set_icp(self, icp: Icp) -> Self {
        Self((self.0 & !Self::REG_ICP) | (Self::REG_ICP & ((icp as u32) << 12)))
    }
    /// Get icp.
    #[inline]
    pub const fn icp(self) -> Icp {
        match (self.0 & Self::REG_ICP) >> 12 {
            0 => Icp::Range50To100M,
            1 => Icp::Range100To150M,
            2 => Icp::Range150To200M,
            3 => Icp::Range200To266M,
            _ => unreachable!(),
        }
    }
    /// Set phase selection (`PHASE_SEL`).
    ///
    /// When configuring 0~14, the following values must be configured:
    /// - `EN_DLL` = 1
    /// - `EN_VCDL` = 1
    /// - `EN_CP` = 1
    /// - `EN_BYPASS` = 0
    ///
    /// When configuring 15 (BYPASS), the following values must be configured:
    /// - `EN_DLL` = 0
    /// - `EN_VCDL` = 1
    /// - `EN_CP` = 0
    /// - `EN_BYPASS` = 1
    ///
    /// If `EN_DLL` = 0, `EN_VCDL` = 0, `EN_CP` = 0, `EN_BYPASS` = 0, then input clock is 0 phase.
    #[doc(alias = "PHASE_SEL")]
    #[inline]
    pub const fn set_phase_sel(self, phase: PhaseSel) -> Self {
        Self((self.0 & !Self::PHASE_SEL) | (Self::PHASE_SEL & ((phase as u32) << 8)))
    }
    /// Get phase selection.
    #[inline]
    pub const fn phase_sel(self) -> PhaseSel {
        match (self.0 & Self::PHASE_SEL) >> 8 {
            0 => PhaseSel::Deg22_5,
            1 => PhaseSel::Deg45,
            2 => PhaseSel::Deg67_5,
            3 => PhaseSel::Deg90,
            4 => PhaseSel::Deg112_5,
            5 => PhaseSel::Deg135,
            6 => PhaseSel::Deg157_5,
            7 => PhaseSel::Deg180,
            8 => PhaseSel::Deg202_5,
            9 => PhaseSel::Deg225,
            10 => PhaseSel::Deg247_5,
            11 => PhaseSel::Deg270,
            12 => PhaseSel::Deg292_5,
            13 => PhaseSel::Deg315,
            14 => PhaseSel::Deg337_5,
            15 => PhaseSel::Bypass,
            _ => unreachable!(),
        }
    }
    /// Enable lvs (`EN_LVS`).
    #[doc(alias = "EN_LVS")]
    #[inline]
    pub const fn enable_lvs(self) -> Self {
        Self(self.0 | Self::EN_LVS)
    }
    /// Disable lvs.
    #[inline]
    pub const fn disable_lvs(self) -> Self {
        Self(self.0 & !Self::EN_LVS)
    }
    /// Check if lvs is enabled.
    #[inline]
    pub const fn is_lvs_enabled(self) -> bool {
        self.0 & Self::EN_LVS != 0
    }
    /// Enable ldo (`EN_LDO`).
    #[doc(alias = "EN_LDO")]
    #[inline]
    pub const fn enable_ldo(self) -> Self {
        Self(self.0 | Self::EN_LDO)
    }
    /// Disable ldo.
    #[inline]
    pub const fn disable_ldo(self) -> Self {
        Self(self.0 & !Self::EN_LDO)
    }
    /// Check if ldo is enabled.
    #[inline]
    pub const fn is_ldo_enabled(self) -> bool {
        self.0 & Self::EN_LDO != 0
    }
    /// Enable bypass (`EN_BYPASS`).
    #[doc(alias = "EN_BYPASS")]
    #[inline]
    pub const fn enable_bypass(self) -> Self {
        Self(self.0 | Self::EN_BYPASS)
    }
    /// Disable bypass.
    #[inline]
    pub const fn disable_bypass(self) -> Self {
        Self(self.0 & !Self::EN_BYPASS)
    }
    /// Check if bypass is enabled.
    #[inline]
    pub const fn is_bypass_enabled(self) -> bool {
        self.0 & Self::EN_BYPASS != 0
    }
    /// Enable cp (`EN_CP`).
    #[doc(alias = "EN_CP")]
    #[inline]
    pub const fn enable_cp(self) -> Self {
        Self(self.0 | Self::EN_CP)
    }
    /// Disable cp.
    #[inline]
    pub const fn disable_cp(self) -> Self {
        Self(self.0 & !Self::EN_CP)
    }
    /// Check if cp is enabled.
    #[inline]
    pub const fn is_cp_enabled(self) -> bool {
        self.0 & Self::EN_CP != 0
    }
    /// Enable vcdl (`EN_VCDL`).
    #[doc(alias = "EN_VCDL")]
    #[inline]
    pub const fn enable_vcdl(self) -> Self {
        Self(self.0 | Self::EN_VCDL)
    }
    /// Disable vcdl.
    #[inline]
    pub const fn disable_vcdl(self) -> Self {
        Self(self.0 & !Self::EN_VCDL)
    }
    /// Check if vcdl is enabled.
    #[inline]
    pub const fn is_vcdl_enabled(self) -> bool {
        self.0 & Self::EN_VCDL != 0
    }
    /// Enable dll (`EN_DLL`).
    ///
    /// After reconfiguring the clock frequency, reconfigure this bit to enable DLL function.
    #[doc(alias = "EN_DLL")]
    #[inline]
    pub const fn enable_dll(self) -> Self {
        Self(self.0 | Self::EN_DLL)
    }
    /// Disable dll.
    #[inline]
    pub const fn disable_dll(self) -> Self {
        Self(self.0 & !Self::EN_DLL)
    }
    /// Check if dll is enabled.
    #[inline]
    pub const fn is_dll_enabled(self) -> bool {
        self.0 & Self::EN_DLL != 0
    }
}

/// XSPI interrupt enable register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IntEnable(u32);

impl IntEnable {
    const XIP_ERROR: u32 = 0x1 << 24;
    const AXI_ERROR: u32 = 0x1 << 23;
    const OPI_ERROR: u32 = 0x1 << 22;
    const HYPERBUS_ERROR: u32 = 0x1 << 21;
    const XCCELA_ERROR: u32 = 0x1 << 20;
    const AXI_TRAN_ERROR_EN: u32 = 0x1 << 19;
    const AHB_TRAN_ERROR_EN: u32 = 0x1 << 18;
    const LUT_INSTRUCTION_ERROR_EN: u32 = 0x1 << 17;
    const LUT_ADDR_OPRAND_ERROR_EN: u32 = 0x1 << 16;
    const CS1_TO_EN: u32 = 0x1 << 15;
    const CS0_TO_EN: u32 = 0x1 << 14;
    const CS1_DONE_EN: u32 = 0x1 << 13;
    const CS0_DONE_EN: u32 = 0x1 << 12;
    const TF_UDR_INT_EN: u32 = 0x1 << 11;
    const TF_OVF_INT_EN: u32 = 0x1 << 10;
    const RF_UDR_INT_EN: u32 = 0x1 << 9;
    const RF_OVF_INT_EN: u32 = 0x1 << 8;
    const TX_FULL_INT_EN: u32 = 0x1 << 6;
    const TX_EMP_INT_EN: u32 = 0x1 << 5;
    const TX_ERQ_INT_EN: u32 = 0x1 << 4;
    const RX_FULL_INT_EN: u32 = 0x1 << 2;
    const RX_EMP_INT_EN: u32 = 0x1 << 1;
    const RX_ERQ_INT_EN: u32 = 0x1;

    /// Enable xip error interrupt (`XIP_ERROR`).
    #[doc(alias = "XIP_ERROR")]
    #[inline]
    pub const fn enable_xip_error(self) -> Self {
        Self(self.0 | Self::XIP_ERROR)
    }
    /// Disable xip error interrupt.
    #[inline]
    pub const fn disable_xip_error(self) -> Self {
        Self(self.0 & !Self::XIP_ERROR)
    }
    /// Check if xip error interrupt is enabled.
    #[inline]
    pub const fn is_xip_error_enabled(self) -> bool {
        self.0 & Self::XIP_ERROR != 0
    }
    /// Enable axi error interrupt (`AXI_ERROR`).
    #[doc(alias = "AXI_ERROR")]
    #[inline]
    pub const fn enable_axi_error(self) -> Self {
        Self(self.0 | Self::AXI_ERROR)
    }
    /// Disable axi error interrupt.
    #[inline]
    pub const fn disable_axi_error(self) -> Self {
        Self(self.0 & !Self::AXI_ERROR)
    }
    /// Check if axi error interrupt is enabled.
    #[inline]
    pub const fn is_axi_error_enabled(self) -> bool {
        self.0 & Self::AXI_ERROR != 0
    }
    /// Enable opi error interrupt (`OPI_ERROR`).
    ///
    /// OPI protocol behavior configuration error.
    /// When XSPI is configured as OPI mode and LUT configuration does not comply with OPI protocol.
    #[doc(alias = "OPI_ERROR")]
    #[inline]
    pub const fn enable_opi_error(self) -> Self {
        Self(self.0 | Self::OPI_ERROR)
    }
    /// Disable opi error interrupt.
    #[inline]
    pub const fn disable_opi_error(self) -> Self {
        Self(self.0 & !Self::OPI_ERROR)
    }
    /// Check if opi error interrupt is enabled.
    #[inline]
    pub const fn is_opi_error_enabled(self) -> bool {
        self.0 & Self::OPI_ERROR != 0
    }
    /// Enable hyperbus error interrupt (`HYPERBUS_ERROR`).
    ///
    /// HYPERBUS protocol behavior configuration error.
    /// When XSPI is configured as HYPERBUS mode and LUT configuration does not comply with HYPERBUS protocol.
    #[doc(alias = "HYPERBUS_ERROR")]
    #[inline]
    pub const fn enable_hyperbus_error(self) -> Self {
        Self(self.0 | Self::HYPERBUS_ERROR)
    }
    /// Disable hyperbus error interrupt.
    #[inline]
    pub const fn disable_hyperbus_error(self) -> Self {
        Self(self.0 & !Self::HYPERBUS_ERROR)
    }
    /// Check if hyperbus error interrupt is enabled.
    #[inline]
    pub const fn is_hyperbus_error_enabled(self) -> bool {
        self.0 & Self::HYPERBUS_ERROR != 0
    }
    /// Enable xccela error interrupt (`XCCELA_ERROR`).
    ///
    /// XCCELA protocol behavior configuration error.
    /// When XSPI is configured as XCCELA mode and LUT configuration does not comply with XCCELA protocol.
    #[doc(alias = "XCCELA_ERROR")]
    #[inline]
    pub const fn enable_xccela_error(self) -> Self {
        Self(self.0 | Self::XCCELA_ERROR)
    }
    /// Disable xccela error interrupt.
    #[inline]
    pub const fn disable_xccela_error(self) -> Self {
        Self(self.0 & !Self::XCCELA_ERROR)
    }
    /// Check if xccela error interrupt is enabled.
    #[inline]
    pub const fn is_xccela_error_enabled(self) -> bool {
        self.0 & Self::XCCELA_ERROR != 0
    }
    /// Enable axi transmission error interrupt (`AXI_TRAN_ERROR_EN`).
    #[doc(alias = "AXI_TRAN_ERROR_EN")]
    #[inline]
    pub const fn enable_axi_trans_error(self) -> Self {
        Self(self.0 | Self::AXI_TRAN_ERROR_EN)
    }
    /// Disable axi transmission error interrupt.
    #[inline]
    pub const fn disable_axi_trans_error(self) -> Self {
        Self(self.0 & !Self::AXI_TRAN_ERROR_EN)
    }
    /// Check if axi transmission error interrupt is enabled.
    #[inline]
    pub const fn is_axi_trans_error_enabled(self) -> bool {
        self.0 & Self::AXI_TRAN_ERROR_EN != 0
    }
    /// Enable ahb transmission error interrupt (`AHB_TRAN_ERROR_EN`).
    #[doc(alias = "AHB_TRAN_ERROR_EN")]
    #[inline]
    pub const fn enable_ahb_trans_error(self) -> Self {
        Self(self.0 | Self::AHB_TRAN_ERROR_EN)
    }
    /// Disable ahb transmission error interrupt.
    #[inline]
    pub const fn disable_ahb_trans_error(self) -> Self {
        Self(self.0 & !Self::AHB_TRAN_ERROR_EN)
    }
    /// Check if ahb transmission error interrupt is enabled.
    #[inline]
    pub const fn is_ahb_trans_error_enabled(self) -> bool {
        self.0 & Self::AHB_TRAN_ERROR_EN != 0
    }
    /// Enable lut instruction error interrupt (`LUT_INSTRUCTION_ERROR_EN`).
    #[doc(alias = "LUT_INSTRUCTION_ERROR_EN")]
    #[inline]
    pub const fn enable_lut_instr_error(self) -> Self {
        Self(self.0 | Self::LUT_INSTRUCTION_ERROR_EN)
    }
    /// Disable lut instruction error interrupt.
    #[inline]
    pub const fn disable_lut_instr_error(self) -> Self {
        Self(self.0 & !Self::LUT_INSTRUCTION_ERROR_EN)
    }
    /// Check if lut instruction error interrupt is enabled.
    #[inline]
    pub const fn is_lut_instr_error_enabled(self) -> bool {
        self.0 & Self::LUT_INSTRUCTION_ERROR_EN != 0
    }
    /// Enable lut address operand error interrupt (`LUT_ADDR_OPRAND_ERROR_EN`).
    #[doc(alias = "LUT_ADDR_OPRAND_ERROR_EN")]
    #[inline]
    pub const fn enable_lut_addr_operand_error(self) -> Self {
        Self(self.0 | Self::LUT_ADDR_OPRAND_ERROR_EN)
    }
    /// Disable lut address operand error interrupt.
    #[inline]
    pub const fn disable_lut_addr_operand_error(self) -> Self {
        Self(self.0 & !Self::LUT_ADDR_OPRAND_ERROR_EN)
    }
    /// Check if lut address operand error interrupt is enabled.
    #[inline]
    pub const fn is_lut_addr_operand_error_enabled(self) -> bool {
        self.0 & Self::LUT_ADDR_OPRAND_ERROR_EN != 0
    }
    /// Enable cs1 timeout interrupt (`CS1_TO_EN`).
    #[doc(alias = "CS1_TO_EN")]
    #[inline]
    pub const fn enable_cs1_timeout(self) -> Self {
        Self(self.0 | Self::CS1_TO_EN)
    }
    /// Disable cs1 timeout interrupt.
    #[inline]
    pub const fn disable_cs1_timeout(self) -> Self {
        Self(self.0 & !Self::CS1_TO_EN)
    }
    /// Check if cs1 timeout interrupt is enabled.
    #[inline]
    pub const fn is_cs1_timeout_enabled(self) -> bool {
        self.0 & Self::CS1_TO_EN != 0
    }
    /// Enable cs0 timeout interrupt (`CS0_TO_EN`).
    #[doc(alias = "CS0_TO_EN")]
    #[inline]
    pub const fn enable_cs0_timeout(self) -> Self {
        Self(self.0 | Self::CS0_TO_EN)
    }
    /// Disable cs0 timeout interrupt.
    #[inline]
    pub const fn disable_cs0_timeout(self) -> Self {
        Self(self.0 & !Self::CS0_TO_EN)
    }
    /// Check if cs0 timeout interrupt is enabled.
    #[inline]
    pub const fn is_cs0_timeout_enabled(self) -> bool {
        self.0 & Self::CS0_TO_EN != 0
    }
    /// Enable cs1 done interrupt (`CS1_DONE_EN`).
    #[doc(alias = "CS1_DONE_EN")]
    #[inline]
    pub const fn enable_cs1_done(self) -> Self {
        Self(self.0 | Self::CS1_DONE_EN)
    }
    /// Disable cs1 done interrupt.
    #[inline]
    pub const fn disable_cs1_done(self) -> Self {
        Self(self.0 & !Self::CS1_DONE_EN)
    }
    /// Check if cs1 done interrupt is enabled.
    #[inline]
    pub const fn is_cs1_done_enabled(self) -> bool {
        self.0 & Self::CS1_DONE_EN != 0
    }
    /// Enable cs0 done interrupt (`CS0_DONE_EN`).
    #[doc(alias = "CS0_DONE_EN")]
    #[inline]
    pub const fn enable_cs0_done(self) -> Self {
        Self(self.0 | Self::CS0_DONE_EN)
    }
    /// Disable cs0 done interrupt.
    #[inline]
    pub const fn disable_cs0_done(self) -> Self {
        Self(self.0 & !Self::CS0_DONE_EN)
    }
    /// Check if cs0 done interrupt is enabled.
    #[inline]
    pub const fn is_cs0_done_enabled(self) -> bool {
        self.0 & Self::CS0_DONE_EN != 0
    }
    /// Enable tx fifo underflow interrupt (`TF_UDR_INT_EN`).
    ///
    /// Generated when hardware reads from empty TX FIFO.
    #[doc(alias = "TF_UDR_INT_EN")]
    #[inline]
    pub const fn enable_tx_fifo_underflow(self) -> Self {
        Self(self.0 | Self::TF_UDR_INT_EN)
    }
    /// Disable tx fifo underflow interrupt.
    #[inline]
    pub const fn disable_tx_fifo_underflow(self) -> Self {
        Self(self.0 & !Self::TF_UDR_INT_EN)
    }
    /// Check if tx fifo underflow interrupt is enabled.
    #[inline]
    pub const fn is_tx_fifo_underflow_enabled(self) -> bool {
        self.0 & Self::TF_UDR_INT_EN != 0
    }
    /// Enable tx fifo overflow interrupt (`TF_OVF_INT_EN`).
    ///
    /// Generated when CPU/DMA writes to full TX FIFO.
    #[doc(alias = "TF_OVF_INT_EN")]
    #[inline]
    pub const fn enable_tx_fifo_overflow(self) -> Self {
        Self(self.0 | Self::TF_OVF_INT_EN)
    }
    /// Disable tx fifo overflow interrupt.
    #[inline]
    pub const fn disable_tx_fifo_overflow(self) -> Self {
        Self(self.0 & !Self::TF_OVF_INT_EN)
    }
    /// Check if tx fifo overflow interrupt is enabled.
    #[inline]
    pub const fn is_tx_fifo_overflow_enabled(self) -> bool {
        self.0 & Self::TF_OVF_INT_EN != 0
    }
    /// Enable rx fifo underflow interrupt (`RF_UDR_INT_EN`).
    ///
    /// Generated when CPU/DMA reads from empty RX FIFO.
    #[doc(alias = "RF_UDR_INT_EN")]
    #[inline]
    pub const fn enable_rx_fifo_underflow(self) -> Self {
        Self(self.0 | Self::RF_UDR_INT_EN)
    }
    /// Disable rx fifo underflow interrupt.
    #[inline]
    pub const fn disable_rx_fifo_underflow(self) -> Self {
        Self(self.0 & !Self::RF_UDR_INT_EN)
    }
    /// Check if rx fifo underflow interrupt is enabled.
    #[inline]
    pub const fn is_rx_fifo_underflow_enabled(self) -> bool {
        self.0 & Self::RF_UDR_INT_EN != 0
    }
    /// Enable rx fifo overflow interrupt (`RF_OVF_INT_EN`).
    ///
    /// Generated when hardware writes to full RX FIFO.
    #[doc(alias = "RF_OVF_INT_EN")]
    #[inline]
    pub const fn enable_rx_fifo_overflow(self) -> Self {
        Self(self.0 | Self::RF_OVF_INT_EN)
    }
    /// Disable rx fifo overflow interrupt.
    #[inline]
    pub const fn disable_rx_fifo_overflow(self) -> Self {
        Self(self.0 & !Self::RF_OVF_INT_EN)
    }
    /// Check if rx fifo overflow interrupt is enabled.
    #[inline]
    pub const fn is_rx_fifo_overflow_enabled(self) -> bool {
        self.0 & Self::RF_OVF_INT_EN != 0
    }
    /// Enable tx fifo full interrupt (`TX_FULL_INT_EN`).
    #[doc(alias = "TX_FULL_INT_EN")]
    #[inline]
    pub const fn enable_tx_fifo_full(self) -> Self {
        Self(self.0 | Self::TX_FULL_INT_EN)
    }
    /// Disable tx fifo full interrupt.
    #[inline]
    pub const fn disable_tx_fifo_full(self) -> Self {
        Self(self.0 & !Self::TX_FULL_INT_EN)
    }
    /// Check if tx fifo full interrupt is enabled.
    #[inline]
    pub const fn is_tx_fifo_full_enabled(self) -> bool {
        self.0 & Self::TX_FULL_INT_EN != 0
    }
    /// Enable tx fifo empty interrupt (`TX_EMP_INT_EN`).
    #[doc(alias = "TX_EMP_INT_EN")]
    #[inline]
    pub const fn enable_tx_fifo_empty(self) -> Self {
        Self(self.0 | Self::TX_EMP_INT_EN)
    }
    /// Disable tx fifo empty interrupt.
    #[inline]
    pub const fn disable_tx_fifo_empty(self) -> Self {
        Self(self.0 & !Self::TX_EMP_INT_EN)
    }
    /// Check if tx fifo empty interrupt is enabled.
    #[inline]
    pub const fn is_tx_fifo_empty_enabled(self) -> bool {
        self.0 & Self::TX_EMP_INT_EN != 0
    }
    /// Enable tx fifo empty request interrupt (`TX_ERQ_INT_EN`).
    #[doc(alias = "TX_ERQ_INT_EN")]
    #[inline]
    pub const fn enable_tx_fifo_empty_req(self) -> Self {
        Self(self.0 | Self::TX_ERQ_INT_EN)
    }
    /// Disable tx fifo empty request interrupt.
    #[inline]
    pub const fn disable_tx_fifo_empty_req(self) -> Self {
        Self(self.0 & !Self::TX_ERQ_INT_EN)
    }
    /// Check if tx fifo empty request interrupt is enabled.
    #[inline]
    pub const fn is_tx_fifo_empty_req_enabled(self) -> bool {
        self.0 & Self::TX_ERQ_INT_EN != 0
    }
    /// Enable rx fifo full interrupt (`RX_FULL_INT_EN`).
    #[doc(alias = "RX_FULL_INT_EN")]
    #[inline]
    pub const fn enable_rx_fifo_full(self) -> Self {
        Self(self.0 | Self::RX_FULL_INT_EN)
    }
    /// Disable rx fifo full interrupt.
    #[inline]
    pub const fn disable_rx_fifo_full(self) -> Self {
        Self(self.0 & !Self::RX_FULL_INT_EN)
    }
    /// Check if rx fifo full interrupt is enabled.
    #[inline]
    pub const fn is_rx_fifo_full_enabled(self) -> bool {
        self.0 & Self::RX_FULL_INT_EN != 0
    }
    /// Enable rx fifo empty interrupt (`RX_EMP_INT_EN`).
    #[doc(alias = "RX_EMP_INT_EN")]
    #[inline]
    pub const fn enable_rx_fifo_empty(self) -> Self {
        Self(self.0 | Self::RX_EMP_INT_EN)
    }
    /// Disable rx fifo empty interrupt.
    #[inline]
    pub const fn disable_rx_fifo_empty(self) -> Self {
        Self(self.0 & !Self::RX_EMP_INT_EN)
    }
    /// Check if rx fifo empty interrupt is enabled.
    #[inline]
    pub const fn is_rx_fifo_empty_enabled(self) -> bool {
        self.0 & Self::RX_EMP_INT_EN != 0
    }
    /// Enable rx fifo empty request interrupt (`RX_ERQ_INT_EN`).
    #[doc(alias = "RX_ERQ_INT_EN")]
    #[inline]
    pub const fn enable_rx_fifo_empty_req(self) -> Self {
        Self(self.0 | Self::RX_ERQ_INT_EN)
    }
    /// Disable rx fifo empty request interrupt.
    #[inline]
    pub const fn disable_rx_fifo_empty_req(self) -> Self {
        Self(self.0 & !Self::RX_ERQ_INT_EN)
    }
    /// Check if rx fifo empty request interrupt is enabled.
    #[inline]
    pub const fn is_rx_fifo_empty_req_enabled(self) -> bool {
        self.0 & Self::RX_ERQ_INT_EN != 0
    }
}

/// XSPI interrupt status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IntStatus(u32);

impl IntStatus {
    const XIP_ERROR: u32 = 0x1 << 24;
    const AXI_ERROR: u32 = 0x1 << 23;
    const OPI_ERROR: u32 = 0x1 << 22;
    const HYPERBUS_ERROR: u32 = 0x1 << 21;
    const XCCELA_ERROR: u32 = 0x1 << 20;
    const AXI_TRAN_ERROR: u32 = 0x1 << 19;
    const AHB_TRAN_ERROR: u32 = 0x1 << 18;
    const LUT_INSTRUCTION_ERROR: u32 = 0x1 << 17;
    const LUT_ADDR_OPRAND_ERROR: u32 = 0x1 << 16;
    const CS1_TO: u32 = 0x1 << 15;
    const CS0_TO: u32 = 0x1 << 14;
    const CS1_DONE: u32 = 0x1 << 13;
    const CS0_DONE: u32 = 0x1 << 12;
    const TF_UDF: u32 = 0x1 << 11;
    const TF_OVF: u32 = 0x1 << 10;
    const RF_UDF: u32 = 0x1 << 9;
    const RF_OVF: u32 = 0x1 << 8;
    const TX_FULL: u32 = 0x1 << 6;
    const TX_EMP: u32 = 0x1 << 5;
    const TX_READY: u32 = 0x1 << 4;
    const RX_FULL: u32 = 0x1 << 2;
    const RX_EMP: u32 = 0x1 << 1;
    const RX_READY: u32 = 0x1;

    /// Check if xip error interrupt is pending (`XIP_ERROR`).
    #[doc(alias = "XIP_ERROR")]
    #[inline]
    pub const fn is_xip_err_pending(self) -> bool {
        self.0 & Self::XIP_ERROR != 0
    }
    /// Clear xip error interrupt.
    #[inline]
    pub const fn clear_xip_err(self) -> Self {
        Self(self.0 | Self::XIP_ERROR)
    }
    /// Check if axi error interrupt is pending (`AXI_ERROR`).
    #[doc(alias = "AXI_ERROR")]
    #[inline]
    pub const fn is_axi_err_pending(self) -> bool {
        self.0 & Self::AXI_ERROR != 0
    }
    /// Clear axi error interrupt.
    #[inline]
    pub const fn clear_axi_err(self) -> Self {
        Self(self.0 | Self::AXI_ERROR)
    }
    /// Check if opi error interrupt is pending (`OPI_ERROR`).
    ///
    /// Set to 1 indicates XSPI is configured as OPI mode and LUT configuration does not comply with OPI protocol.
    #[doc(alias = "OPI_ERROR")]
    #[inline]
    pub const fn is_opi_err_pending(self) -> bool {
        self.0 & Self::OPI_ERROR != 0
    }
    /// Clear opi error interrupt.
    #[inline]
    pub const fn clear_opi_err(self) -> Self {
        Self(self.0 | Self::OPI_ERROR)
    }
    /// Check if hyperbus error interrupt is pending (`HYPERBUS_ERROR`).
    ///
    /// Set to 1 indicates XSPI is configured as HYPERBUS mode and LUT configuration does not comply with HYPERBUS protocol.
    #[doc(alias = "HYPERBUS_ERROR")]
    #[inline]
    pub const fn is_hyperbus_err_pending(self) -> bool {
        self.0 & Self::HYPERBUS_ERROR != 0
    }
    /// Clear hyperbus error interrupt.
    #[inline]
    pub const fn clear_hyperbus_err(self) -> Self {
        Self(self.0 | Self::HYPERBUS_ERROR)
    }
    /// Check if xccela error interrupt is pending (`XCCELA_ERROR`).
    ///
    /// Set to 1 indicates XSPI is configured as XCCELA mode and LUT configuration does not comply with XCCELA protocol.
    #[doc(alias = "XCCELA_ERROR")]
    #[inline]
    pub const fn is_xccela_err_pending(self) -> bool {
        self.0 & Self::XCCELA_ERROR != 0
    }
    /// Clear xccela error interrupt.
    #[inline]
    pub const fn clear_xccela_err(self) -> Self {
        Self(self.0 | Self::XCCELA_ERROR)
    }
    /// Check if axi transmission error interrupt is pending (`AXI_TRAN_ERROR`).
    ///
    /// Set to 1 indicates AXI transfer request while AHB is transmitting.
    /// Need to end AHB transfer immediately and start AXI transfer.
    #[doc(alias = "AXI_TRAN_ERROR")]
    #[inline]
    pub const fn is_axi_trans_err_pending(self) -> bool {
        self.0 & Self::AXI_TRAN_ERROR != 0
    }
    /// Clear axi transmission error interrupt.
    #[inline]
    pub const fn clear_axi_trans_err(self) -> Self {
        Self(self.0 | Self::AXI_TRAN_ERROR)
    }
    /// Check if ahb transmission error interrupt is pending (`AHB_TRAN_ERROR`).
    ///
    /// Set to 1 indicates AHB transfer request while AXI is transmitting.
    #[doc(alias = "AHB_TRAN_ERROR")]
    #[inline]
    pub const fn is_ahb_trans_err_pending(self) -> bool {
        self.0 & Self::AHB_TRAN_ERROR != 0
    }
    /// Clear ahb transmission error interrupt.
    #[inline]
    pub const fn clear_ahb_trans_err(self) -> Self {
        Self(self.0 | Self::AHB_TRAN_ERROR)
    }
    /// Check if lut instruction error interrupt is pending (`LUT_INSTRUCTION_ERROR`).
    ///
    /// Set to 1 indicates LUT configuration has undefined instruction.
    #[doc(alias = "LUT_INSTRUCTION_ERROR")]
    #[inline]
    pub const fn is_lut_instr_err_pending(self) -> bool {
        self.0 & Self::LUT_INSTRUCTION_ERROR != 0
    }
    /// Clear lut instruction error interrupt.
    #[inline]
    pub const fn clear_lut_instr_err(self) -> Self {
        Self(self.0 | Self::LUT_INSTRUCTION_ERROR)
    }
    /// Check if lut address operand error interrupt is pending (`LUT_ADDR_OPRAND_ERROR`).
    ///
    /// Set to 1 indicates LUT address width configuration error.
    #[doc(alias = "LUT_ADDR_OPRAND_ERROR")]
    #[inline]
    pub const fn is_lut_addr_operand_err_pending(self) -> bool {
        self.0 & Self::LUT_ADDR_OPRAND_ERROR != 0
    }
    /// Clear lut address operand error interrupt.
    #[inline]
    pub const fn clear_lut_addr_operand_err(self) -> Self {
        Self(self.0 | Self::LUT_ADDR_OPRAND_ERROR)
    }
    /// Check if cs1 timeout interrupt is pending (`CS1_TO`).
    #[doc(alias = "CS1_TO")]
    #[inline]
    pub const fn is_cs1_timeout_pending(self) -> bool {
        self.0 & Self::CS1_TO != 0
    }
    /// Clear cs1 timeout interrupt.
    #[inline]
    pub const fn clear_cs1_timeout(self) -> Self {
        Self(self.0 | Self::CS1_TO)
    }
    /// Check if cs0 timeout interrupt is pending (`CS0_TO`).
    #[doc(alias = "CS0_TO")]
    #[inline]
    pub const fn is_cs0_timeout_pending(self) -> bool {
        self.0 & Self::CS0_TO != 0
    }
    /// Clear cs0 timeout interrupt.
    #[inline]
    pub const fn clear_cs0_timeout(self) -> Self {
        Self(self.0 | Self::CS0_TO)
    }
    /// Check if cs1 done interrupt is pending (`CS1_DONE`).
    #[doc(alias = "CS1_DONE")]
    #[inline]
    pub const fn is_cs1_done_pending(self) -> bool {
        self.0 & Self::CS1_DONE != 0
    }
    /// Clear cs1 done interrupt.
    #[inline]
    pub const fn clear_cs1_done(self) -> Self {
        Self(self.0 | Self::CS1_DONE)
    }
    /// Check if cs0 done interrupt is pending (`CS0_DONE`).
    #[doc(alias = "CS0_DONE")]
    #[inline]
    pub const fn is_cs0_done_pending(self) -> bool {
        self.0 & Self::CS0_DONE != 0
    }
    /// Clear cs0 done interrupt.
    #[inline]
    pub const fn clear_cs0_done(self) -> Self {
        Self(self.0 | Self::CS0_DONE)
    }
    /// Check if tx fifo underflow interrupt is pending (`TF_UDF`).
    #[doc(alias = "TF_UDF")]
    #[inline]
    pub const fn is_tx_fifo_underflow_pending(self) -> bool {
        self.0 & Self::TF_UDF != 0
    }
    /// Clear tx fifo underflow interrupt.
    #[inline]
    pub const fn clear_tx_fifo_underflow(self) -> Self {
        Self(self.0 | Self::TF_UDF)
    }
    /// Check if tx fifo overflow interrupt is pending (`TF_OVF`).
    #[doc(alias = "TF_OVF")]
    #[inline]
    pub const fn is_tx_fifo_overflow_pending(self) -> bool {
        self.0 & Self::TF_OVF != 0
    }
    /// Clear tx fifo overflow interrupt.
    #[inline]
    pub const fn clear_tx_fifo_overflow(self) -> Self {
        Self(self.0 | Self::TF_OVF)
    }
    /// Check if rx fifo underflow interrupt is pending (`RF_UDF`).
    #[doc(alias = "RF_UDF")]
    #[inline]
    pub const fn is_rx_fifo_underflow_pending(self) -> bool {
        self.0 & Self::RF_UDF != 0
    }
    /// Clear rx fifo underflow interrupt.
    #[inline]
    pub const fn clear_rx_fifo_underflow(self) -> Self {
        Self(self.0 | Self::RF_UDF)
    }
    /// Check if rx fifo overflow interrupt is pending (`RF_OVF`).
    #[doc(alias = "RF_OVF")]
    #[inline]
    pub const fn is_rx_fifo_overflow_pending(self) -> bool {
        self.0 & Self::RF_OVF != 0
    }
    /// Clear rx fifo overflow interrupt.
    #[inline]
    pub const fn clear_rx_fifo_overflow(self) -> Self {
        Self(self.0 | Self::RF_OVF)
    }
    /// Check if tx fifo full interrupt is pending (`TX_FULL`).
    #[doc(alias = "TX_FULL")]
    #[inline]
    pub const fn is_tx_fifo_full_pending(self) -> bool {
        self.0 & Self::TX_FULL != 0
    }
    /// Clear tx fifo full interrupt.
    #[inline]
    pub const fn clear_tx_fifo_full(self) -> Self {
        Self(self.0 | Self::TX_FULL)
    }
    /// Check if tx fifo empty interrupt is pending (`TX_EMP`).
    #[doc(alias = "TX_EMP")]
    #[inline]
    pub const fn is_tx_fifo_empty_pending(self) -> bool {
        self.0 & Self::TX_EMP != 0
    }
    /// Clear tx fifo empty interrupt.
    #[inline]
    pub const fn clear_tx_fifo_empty(self) -> Self {
        Self(self.0 | Self::TX_EMP)
    }
    /// Check if tx fifo ready interrupt is pending (`TX_READY`).
    #[doc(alias = "TX_READY")]
    #[inline]
    pub const fn is_tx_fifo_ready_pending(self) -> bool {
        self.0 & Self::TX_READY != 0
    }
    /// Clear tx fifo ready interrupt.
    #[inline]
    pub const fn clear_tx_fifo_ready(self) -> Self {
        Self(self.0 | Self::TX_READY)
    }
    /// Check if rx fifo full interrupt is pending (`RX_FULL`).
    #[doc(alias = "RX_FULL")]
    #[inline]
    pub const fn is_rx_fifo_full_pending(self) -> bool {
        self.0 & Self::RX_FULL != 0
    }
    /// Clear rx fifo full interrupt.
    #[inline]
    pub const fn clear_rx_fifo_full(self) -> Self {
        Self(self.0 | Self::RX_FULL)
    }
    /// Check if rx fifo empty interrupt is pending (`RX_EMP`).
    #[doc(alias = "RX_EMP")]
    #[inline]
    pub const fn is_rx_fifo_empty_pending(self) -> bool {
        self.0 & Self::RX_EMP != 0
    }
    /// Clear rx fifo empty interrupt.
    #[inline]
    pub const fn clear_rx_fifo_empty(self) -> Self {
        Self(self.0 | Self::RX_EMP)
    }
    /// Check if rx fifo ready interrupt is pending (`RX_READY`).
    #[doc(alias = "RX_READY")]
    #[inline]
    pub const fn is_rx_fifo_ready_pending(self) -> bool {
        self.0 & Self::RX_READY != 0
    }
    /// Clear rx fifo ready interrupt.
    #[inline]
    pub const fn clear_rx_fifo_ready(self) -> Self {
        Self(self.0 | Self::RX_READY)
    }
}

/// XSPI fifo control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FifoControl(u32);

impl FifoControl {
    const TX_FIFO_RST: u32 = 0x1 << 31;
    const TF_DRQ_EN: u32 = 0x1 << 24;
    const TX_TRIG_LEVEL: u32 = 0x7F << 16;
    const RF_RST: u32 = 0x1 << 15;
    const RF_DRQ_EN: u32 = 0x1 << 8;
    const RX_TRIG_LEVEL: u32 = 0x7F;

    /// Set tx fifo reset bit (`TX_FIFO_RST`).
    #[doc(alias = "TX_FIFO_RST")]
    #[inline]
    pub const fn set_tx_fifo_reset(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::TX_FIFO_RST)
        } else {
            Self(self.0 & !Self::TX_FIFO_RST)
        }
    }
    /// Get tx fifo reset bit.
    #[inline]
    pub const fn tx_fifo_reset(self) -> bool {
        self.0 & Self::TX_FIFO_RST != 0
    }
    /// Enable tx fifo dma request (`TF_DRQ_EN`).
    ///
    /// DMA transfer is requested when TX FIFO data depth is less than the trigger level.
    #[doc(alias = "TF_DRQ_EN")]
    #[inline]
    pub const fn enable_tx_dma_req(self) -> Self {
        Self(self.0 | Self::TF_DRQ_EN)
    }
    /// Disable tx fifo dma request.
    #[inline]
    pub const fn disable_tx_dma_req(self) -> Self {
        Self(self.0 & !Self::TF_DRQ_EN)
    }
    /// Check if tx fifo dma request is enabled.
    #[inline]
    pub const fn is_tx_dma_req_enabled(self) -> bool {
        self.0 & Self::TF_DRQ_EN != 0
    }
    /// Set tx fifo trigger level (`TX_TRIG_LEVEL`).
    #[doc(alias = "TX_TRIG_LEVEL")]
    #[inline]
    pub const fn set_tx_trigger_level(self, level: u8) -> Self {
        assert!(
            level < 0x80,
            "TX trigger level out of range (expected 0..=127)"
        );
        Self((self.0 & !Self::TX_TRIG_LEVEL) | (Self::TX_TRIG_LEVEL & ((level as u32) << 16)))
    }
    /// Get tx fifo trigger level.
    #[inline]
    pub const fn tx_trigger_level(self) -> u8 {
        ((self.0 & Self::TX_TRIG_LEVEL) >> 16) as u8
    }
    /// Set rx fifo reset bit (`RF_RST`).
    #[doc(alias = "RF_RST")]
    #[inline]
    pub const fn set_rx_fifo_reset(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::RF_RST)
        } else {
            Self(self.0 & !Self::RF_RST)
        }
    }
    /// Get rx fifo reset bit.
    #[inline]
    pub const fn rx_fifo_reset(self) -> bool {
        self.0 & Self::RF_RST != 0
    }
    /// Enable rx fifo dma request (`RF_DRQ_EN`).
    ///
    /// DMA transfer is requested when RX FIFO data depth is greater than the trigger level.
    #[doc(alias = "RF_DRQ_EN")]
    #[inline]
    pub const fn enable_rx_dma_req(self) -> Self {
        Self(self.0 | Self::RF_DRQ_EN)
    }
    /// Disable rx fifo dma request.
    #[inline]
    pub const fn disable_rx_dma_req(self) -> Self {
        Self(self.0 & !Self::RF_DRQ_EN)
    }
    /// Check if rx fifo dma request is enabled.
    #[inline]
    pub const fn is_rx_dma_req_enabled(self) -> bool {
        self.0 & Self::RF_DRQ_EN != 0
    }
    /// Set rx fifo trigger level (`RX_TRIG_LEVEL`).
    #[doc(alias = "RX_TRIG_LEVEL")]
    #[inline]
    pub const fn set_rx_trigger_level(self, level: u8) -> Self {
        assert!(
            level < 0x80,
            "RX trigger level out of range (expected 0..=127)"
        );
        Self((self.0 & !Self::RX_TRIG_LEVEL) | (Self::RX_TRIG_LEVEL & (level as u32)))
    }
    /// Get rx fifo trigger level.
    #[inline]
    pub const fn rx_trigger_level(self) -> u8 {
        (self.0 & Self::RX_TRIG_LEVEL) as u8
    }
}

/// XSPI FIFO status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FifoStatus(u32);

impl FifoStatus {
    const TB_WR: u32 = 0x1 << 31;
    const TB_CNT: u32 = 0x7 << 28;
    const TF_CNT: u32 = 0xFF << 16;
    const RB_WR: u32 = 0x1 << 15;
    const RB_CNT: u32 = 0x7 << 12;
    const RF_CNT: u32 = 0xFF;

    /// Check if tx fifo write buffer is enabled (`TB_WR`).
    #[doc(alias = "TB_WR")]
    #[inline]
    pub const fn is_tx_buffer_write_enabled(self) -> bool {
        self.0 & Self::TB_WR != 0
    }
    /// Get tx fifo write buffer counter (`TB_CNT`).
    #[doc(alias = "TB_CNT")]
    #[inline]
    pub const fn tx_buffer_count(self) -> u8 {
        ((self.0 & Self::TB_CNT) >> 28) as u8
    }
    /// Get tx fifo counter (`TF_CNT`).
    #[doc(alias = "TF_CNT")]
    #[inline]
    pub const fn tx_fifo_count(self) -> u8 {
        ((self.0 & Self::TF_CNT) >> 16) as u8
    }
    /// Check if rx fifo read buffer write is enabled (`RB_WR`).
    #[doc(alias = "RB_WR")]
    #[inline]
    pub const fn is_rx_buffer_write_enabled(self) -> bool {
        self.0 & Self::RB_WR != 0
    }
    /// Get rx fifo read buffer counter (`RB_CNT`).
    #[doc(alias = "RB_CNT")]
    #[inline]
    pub const fn rx_buffer_count(self) -> u8 {
        ((self.0 & Self::RB_CNT) >> 12) as u8
    }
    /// Get rx fifo counter (`RF_CNT`).
    #[doc(alias = "RF_CNT")]
    #[inline]
    pub const fn rx_fifo_count(self) -> u8 {
        (self.0 & Self::RF_CNT) as u8
    }
}

/// XSPI start register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Start(u32);

impl Start {
    const START_CTL: u32 = 0xF;

    /// Set start group (`START_CTL`).
    ///
    /// AHB channel start group:
    /// - 0: start execution from LUT0
    /// - 1: start execution from LUT4
    /// - ...
    /// - 7: start execution from LUT28
    ///
    /// LUT registers are divided into eight groups, with four LUTs per group.
    /// The register will be automatically cleared to 0 after startup is complete.
    #[doc(alias = "START_CTL")]
    #[inline]
    pub const fn set_start_group(self, group: u8) -> Self {
        assert!(group < 0x8, "Start group out of range (expected 0..=7)");
        Self((self.0 & !Self::START_CTL) | (Self::START_CTL & (group as u32)))
    }
}

/// Format selection for simple operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FormatSel {
    /// LUT configuration, AHB/AXI read/write access.
    LutConfig,
    /// COMMAND, 1S.
    Command1S,
    /// COMMAND + 3-byte address, 1S-1S.
    Command3Addr,
    /// COMMAND + 4-byte address, 1S-1S.
    Command4Addr,
    /// COMMAND, 8D.
    Command8D,
    /// COMMAND + COMMAND_EX, 8D.
    CommandEx8D,
    /// COMMAND + COMMAND_EX + 4-byte address, 8D-8D.
    CommandExAddr8D,
}

/// XSPI format configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Format(u32);

impl Format {
    const FORMAT_CMD: u32 = 0xFF << 24;
    const FORMAT_CMD_EX: u32 = 0xFF << 16;
    const FORMAT_SEL: u32 = 0x7;

    /// Set format command (`FORMAT_CMD`).
    #[doc(alias = "FORMAT_CMD")]
    #[inline]
    pub const fn set_format_cmd(self, cmd: u8) -> Self {
        Self((self.0 & !Self::FORMAT_CMD) | (Self::FORMAT_CMD & ((cmd as u32) << 24)))
    }
    /// Get format command.
    #[inline]
    pub const fn format_cmd(self) -> u8 {
        ((self.0 & Self::FORMAT_CMD) >> 24) as u8
    }
    /// Set format command extension (`FORMAT_CMD_EX`).
    #[doc(alias = "FORMAT_CMD_EX")]
    #[inline]
    pub const fn set_format_cmd_ex(self, cmd_ex: u8) -> Self {
        Self((self.0 & !Self::FORMAT_CMD_EX) | (Self::FORMAT_CMD_EX & ((cmd_ex as u32) << 16)))
    }
    /// Get format command extension.
    #[inline]
    pub const fn format_cmd_ex(self) -> u8 {
        ((self.0 & Self::FORMAT_CMD_EX) >> 16) as u8
    }
    /// Set format selection (`FORMAT_SEL`).
    ///
    /// This bit is write-autoclear. Writing a value will trigger the corresponding operation.
    /// The operation will be executed without LUT configuration.
    /// After transmission ends, this bit is automatically cleared.
    #[doc(alias = "FORMAT_SEL")]
    #[inline]
    pub const fn set_format_sel(self, sel: FormatSel) -> Self {
        Self((self.0 & !Self::FORMAT_SEL) | (Self::FORMAT_SEL & (sel as u32)))
    }
    /// Get format selection value.
    #[inline]
    pub const fn format_sel(self) -> FormatSel {
        match self.0 & Self::FORMAT_SEL {
            0 => FormatSel::LutConfig,
            1 => FormatSel::Command1S,
            2 => FormatSel::Command3Addr,
            3 => FormatSel::Command4Addr,
            4 => FormatSel::Command8D,
            5 => FormatSel::CommandEx8D,
            6 => FormatSel::CommandExAddr8D,
            _ => unreachable!(),
        }
    }
}

/// XSPI burst type register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BurstType(u32);

impl BurstType {
    const SPI_BURST_WRAPPED: u32 = 0xFF << 8;
    const SPI_BURST_LINEAR: u32 = 0xFF;

    /// Set SPI burst wrapped command (`SPI_BURST_WRAPPED`).
    ///
    /// When SPI mode XIP is enabled, if AXI burst type switches from linear to wrapped,
    /// this command is triggered for sending.
    #[doc(alias = "SPI_BURST_WRAPPED")]
    #[inline]
    pub const fn set_spi_burst_wrapped(self, cmd: u8) -> Self {
        Self((self.0 & !Self::SPI_BURST_WRAPPED) | (Self::SPI_BURST_WRAPPED & ((cmd as u32) << 8)))
    }
    /// Get SPI burst wrapped command.
    #[inline]
    pub const fn spi_burst_wrapped(self) -> u8 {
        ((self.0 & Self::SPI_BURST_WRAPPED) >> 8) as u8
    }
    /// Set SPI burst linear command (`SPI_BURST_LINEAR`).
    ///
    /// When SPI mode XIP is enabled, if AXI burst type switches from wrapped to linear,
    /// this command is triggered for sending.
    #[doc(alias = "SPI_BURST_LINEAR")]
    #[inline]
    pub const fn set_spi_burst_linear(self, cmd: u8) -> Self {
        Self((self.0 & !Self::SPI_BURST_LINEAR) | (Self::SPI_BURST_LINEAR & (cmd as u32)))
    }
    /// Get SPI burst linear command.
    #[inline]
    pub const fn spi_burst_linear(self) -> u8 {
        (self.0 & Self::SPI_BURST_LINEAR) as u8
    }
}

/// XSPI read command control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct RdCmdControl(u32);

impl RdCmdControl {
    const READ_MODE_BYTE_EN: u32 = 0x1 << 17;
    const RDCMD_BYPASS_EN: u32 = 0x1 << 16;
    const RDCMD_BYPASS_CODE: u32 = 0xFF << 8;
    const RDCMD_NORMAL_CODE: u32 = 0xFF;

    /// Enable read mode byte (`READ_MODE_BYTE_EN`).
    ///
    /// When enabled, 1 byte mode bit is added after the address.
    /// The line width of the mode bit is the same as the address bit.
    /// The setting value of the mode bit is determined by `RDCMD_BYPASS_EN`.
    ///
    /// Note: This bit must be enabled when SPI works in Dual I/O, Quad I/O, QPI, DTR Quad I/O modes.
    #[doc(alias = "READ_MODE_BYTE_EN")]
    #[inline]
    pub const fn enable_read_mode_byte(self) -> Self {
        Self(self.0 | Self::READ_MODE_BYTE_EN)
    }
    /// Disable read mode byte.
    #[inline]
    pub const fn disable_read_mode_byte(self) -> Self {
        Self(self.0 & !Self::READ_MODE_BYTE_EN)
    }
    /// Check if read mode byte is enabled.
    #[inline]
    pub const fn is_read_mode_byte_enabled(self) -> bool {
        self.0 & Self::READ_MODE_BYTE_EN != 0
    }
    /// Enable read command bypass mode (`RDCMD_BYPASS_EN`).
    ///
    /// When enabled, the mode bit setting value is `RDCMD_BYPASS_CODE`, otherwise it is `RDCMD_NORMAL_CODE`.
    /// After enabling, subsequent read operations do not need to send read commands again.
    #[doc(alias = "RDCMD_BYPASS_EN")]
    #[inline]
    pub const fn enable_rdcmd_bypass(self) -> Self {
        Self(self.0 | Self::RDCMD_BYPASS_EN)
    }
    /// Disable read command bypass mode.
    #[inline]
    pub const fn disable_rdcmd_bypass(self) -> Self {
        Self(self.0 & !Self::RDCMD_BYPASS_EN)
    }
    /// Check if read command bypass mode is enabled.
    #[inline]
    pub const fn is_rdcmd_bypass_enabled(self) -> bool {
        self.0 & Self::RDCMD_BYPASS_EN != 0
    }
    /// Set read command bypass code (`RDCMD_BYPASS_CODE`).
    ///
    /// Defines the setting value in read command bypass mode.
    #[doc(alias = "RDCMD_BYPASS_CODE")]
    #[inline]
    pub const fn set_rdcmd_bypass_code(self, code: u8) -> Self {
        Self((self.0 & !Self::RDCMD_BYPASS_CODE) | (Self::RDCMD_BYPASS_CODE & ((code as u32) << 8)))
    }
    /// Get read command bypass code.
    #[inline]
    pub const fn rdcmd_bypass_code(self) -> u8 {
        ((self.0 & Self::RDCMD_BYPASS_CODE) >> 8) as u8
    }
    /// Set read command normal code (`RDCMD_NORMAL_CODE`).
    ///
    /// Defines the setting value in normal mode. Each read operation requires sending a read command.
    #[doc(alias = "RDCMD_NORMAL_CODE")]
    #[inline]
    pub const fn set_rdcmd_normal_code(self, code: u8) -> Self {
        Self((self.0 & !Self::RDCMD_NORMAL_CODE) | (Self::RDCMD_NORMAL_CODE & (code as u32)))
    }
    /// Get read command normal code.
    #[inline]
    pub const fn rdcmd_normal_code(self) -> u8 {
        (self.0 & Self::RDCMD_NORMAL_CODE) as u8
    }
}

/// DMA active signal control mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DmaActiveMode {
    /// Dma active signal is low.
    Low,
    /// Dma active signal is high.
    High,
    /// Dma active signal is controlled by DMA request.
    DmaRequest,
    /// Dma active signal is controlled by controller.
    Controller,
}

/// Active fall behavior control.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ActiveFallBehavior {
    /// Active fall do not care ack.
    DoNotCareAck,
    /// Active fall must after detect ack is high.
    MustAfterAckHigh,
}

/// XSPI DMA mode control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DmaModeControl(u32);

impl DmaModeControl {
    const DMA_ACTIVE_MODE: u32 = 0x3 << 6;
    const ACTIVE_FALL_BEHAVIOR: u32 = 0x1 << 5;
    const DELAY_CLOCKS: u32 = 0x1F;

    /// Set dma active mode (`DMA_ACTIVE_MODE`).
    #[doc(alias = "DMA_ACTIVE_MODE")]
    #[inline]
    pub const fn set_dma_active_mode(self, mode: DmaActiveMode) -> Self {
        Self((self.0 & !Self::DMA_ACTIVE_MODE) | (Self::DMA_ACTIVE_MODE & ((mode as u32) << 6)))
    }
    /// Get dma active mode.
    #[inline]
    pub const fn dma_active_mode(self) -> DmaActiveMode {
        match (self.0 & Self::DMA_ACTIVE_MODE) >> 6 {
            0 => DmaActiveMode::Low,
            1 => DmaActiveMode::High,
            2 => DmaActiveMode::DmaRequest,
            3 => DmaActiveMode::Controller,
            _ => unreachable!(),
        }
    }
    /// Set active fall behavior (`ACTIVE_FALL_BEHAVIOR`).
    #[doc(alias = "ACTIVE_FALL_BEHAVIOR")]
    #[inline]
    pub const fn set_active_fall_behavior(self, behavior: ActiveFallBehavior) -> Self {
        Self(
            (self.0 & !Self::ACTIVE_FALL_BEHAVIOR)
                | (Self::ACTIVE_FALL_BEHAVIOR & ((behavior as u32) << 5)),
        )
    }
    /// Get active fall behavior.
    #[inline]
    pub const fn active_fall_behavior(self) -> ActiveFallBehavior {
        match (self.0 & Self::ACTIVE_FALL_BEHAVIOR) >> 5 {
            0 => ActiveFallBehavior::DoNotCareAck,
            1 => ActiveFallBehavior::MustAfterAckHigh,
            _ => unreachable!(),
        }
    }
    /// Set delay clock count (`DELAY_CLOCKS`).
    ///
    /// Delay clock count from the last DMA signal going high to Dma active signal staying high.
    #[doc(alias = "DELAY_CLOCKS")]
    #[inline]
    pub const fn set_delay_clocks(self, clocks: u8) -> Self {
        assert!(clocks < 0x20, "Delay clocks out of range (expected 0..=31)");
        Self((self.0 & !Self::DELAY_CLOCKS) | (Self::DELAY_CLOCKS & (clocks as u32)))
    }
    /// Get delay clock count.
    #[inline]
    pub const fn delay_clocks(self) -> u8 {
        (self.0 & Self::DELAY_CLOCKS) as u8
    }
}

/// Lock configuration state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LockCfg {
    /// Locked: LUT cannot be configured.
    Locked = 1,
    /// Unlocked: LUT can be configured.
    Unlocked = 2,
}

/// XSPI lock configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LockConfig(u32);

impl LockConfig {
    const LOCK_CFG: u32 = 0x3;

    /// Set lock configuration (`LOCK_CFG`).
    #[doc(alias = "LOCK_CFG")]
    #[inline]
    pub const fn set_lock_cfg(self, cfg: LockCfg) -> Self {
        Self((self.0 & !Self::LOCK_CFG) | (Self::LOCK_CFG & (cfg as u32)))
    }
    /// Get lock configuration.
    #[inline]
    pub const fn lock_cfg(self) -> LockCfg {
        match self.0 & Self::LOCK_CFG {
            1 => LockCfg::Locked,
            2 => LockCfg::Unlocked,
            _ => unreachable!(),
        }
    }
}

/// XSPI look up table update register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LutUp(u32);

impl LutUp {
    const LUT_UP: u32 = 0x1;

    /// Set lut update bit (`LUT_UP`).
    ///
    /// Writing 1 updates the current LUT read/write control information to the AXI channel transfer,
    /// reducing the hardware LUT lookup time. Automatically cleared to 0 after completion.
    ///
    /// Note: This operation must be completed before setting `XIP_EN` to 1.
    #[doc(alias = "LUT_UP")]
    #[inline]
    pub const fn set_lut_update(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::LUT_UP)
        } else {
            Self(self.0 & !Self::LUT_UP)
        }
    }
    /// Get lut update bit.
    #[inline]
    pub const fn lut_update(self) -> bool {
        self.0 & Self::LUT_UP != 0
    }
}

/// Physical data line selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DataLine {
    /// Data line 0.
    D0,
    /// Data line 1.
    D1,
    /// Data line 2.
    D2,
    /// Data line 3.
    D3,
    /// Data line 4.
    D4,
    /// Data line 5.
    D5,
    /// Data line 6.
    D6,
    /// Data line 7.
    D7,
}

/// XSPI chip select sequence register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CsSequence(u32);

impl CsSequence {
    const D7_SELD: u32 = 0x7 << 28;
    const D6_SELD: u32 = 0x7 << 24;
    const D5_SELD: u32 = 0x7 << 20;
    const D4_SELD: u32 = 0x7 << 16;
    const D3_SELD: u32 = 0x7 << 12;
    const D2_SELD: u32 = 0x7 << 8;
    const D1_SELD: u32 = 0x7 << 4;
    const D0_SELD: u32 = 0x7;

    /// Set data line 7 selection (`D7_SELD`).
    #[doc(alias = "D7_SELD")]
    #[inline]
    pub const fn set_d7_sel(self, line: DataLine) -> Self {
        Self((self.0 & !Self::D7_SELD) | (Self::D7_SELD & ((line as u32) << 28)))
    }
    /// Get data line 7 selection.
    #[inline]
    pub const fn d7_sel(self) -> DataLine {
        match (self.0 & Self::D7_SELD) >> 28 {
            0 => DataLine::D0,
            1 => DataLine::D1,
            2 => DataLine::D2,
            3 => DataLine::D3,
            4 => DataLine::D4,
            5 => DataLine::D5,
            6 => DataLine::D6,
            7 => DataLine::D7,
            _ => unreachable!(),
        }
    }
    /// Set data line 6 selection (`D6_SELD`).
    #[doc(alias = "D6_SELD")]
    #[inline]
    pub const fn set_d6_sel(self, line: DataLine) -> Self {
        Self((self.0 & !Self::D6_SELD) | (Self::D6_SELD & ((line as u32) << 24)))
    }
    /// Get data line 6 selection.
    #[inline]
    pub const fn d6_sel(self) -> DataLine {
        match (self.0 & Self::D6_SELD) >> 24 {
            0 => DataLine::D0,
            1 => DataLine::D1,
            2 => DataLine::D2,
            3 => DataLine::D3,
            4 => DataLine::D4,
            5 => DataLine::D5,
            6 => DataLine::D6,
            7 => DataLine::D7,
            _ => unreachable!(),
        }
    }
    /// Set data line 5 selection (`D5_SELD`).
    #[doc(alias = "D5_SELD")]
    #[inline]
    pub const fn set_d5_sel(self, line: DataLine) -> Self {
        Self((self.0 & !Self::D5_SELD) | (Self::D5_SELD & ((line as u32) << 20)))
    }
    /// Get data line 5 selection.
    #[inline]
    pub const fn d5_sel(self) -> DataLine {
        match (self.0 & Self::D5_SELD) >> 20 {
            0 => DataLine::D0,
            1 => DataLine::D1,
            2 => DataLine::D2,
            3 => DataLine::D3,
            4 => DataLine::D4,
            5 => DataLine::D5,
            6 => DataLine::D6,
            7 => DataLine::D7,
            _ => unreachable!(),
        }
    }
    /// Set data line 4 selection (`D4_SELD`).
    #[doc(alias = "D4_SELD")]
    #[inline]
    pub const fn set_d4_sel(self, line: DataLine) -> Self {
        Self((self.0 & !Self::D4_SELD) | (Self::D4_SELD & ((line as u32) << 16)))
    }
    /// Get data line 4 selection.
    #[inline]
    pub const fn d4_sel(self) -> DataLine {
        match (self.0 & Self::D4_SELD) >> 16 {
            0 => DataLine::D0,
            1 => DataLine::D1,
            2 => DataLine::D2,
            3 => DataLine::D3,
            4 => DataLine::D4,
            5 => DataLine::D5,
            6 => DataLine::D6,
            7 => DataLine::D7,
            _ => unreachable!(),
        }
    }
    /// Set data line 3 selection (`D3_SELD`).
    #[doc(alias = "D3_SELD")]
    #[inline]
    pub const fn set_d3_sel(self, line: DataLine) -> Self {
        Self((self.0 & !Self::D3_SELD) | (Self::D3_SELD & ((line as u32) << 12)))
    }
    /// Get data line 3 selection.
    #[inline]
    pub const fn d3_sel(self) -> DataLine {
        match (self.0 & Self::D3_SELD) >> 12 {
            0 => DataLine::D0,
            1 => DataLine::D1,
            2 => DataLine::D2,
            3 => DataLine::D3,
            4 => DataLine::D4,
            5 => DataLine::D5,
            6 => DataLine::D6,
            7 => DataLine::D7,
            _ => unreachable!(),
        }
    }
    /// Set data line 2 selection (`D2_SELD`).
    #[doc(alias = "D2_SELD")]
    #[inline]
    pub const fn set_d2_sel(self, line: DataLine) -> Self {
        Self((self.0 & !Self::D2_SELD) | (Self::D2_SELD & ((line as u32) << 8)))
    }
    /// Get data line 2 selection.
    #[inline]
    pub const fn d2_sel(self) -> DataLine {
        match (self.0 & Self::D2_SELD) >> 8 {
            0 => DataLine::D0,
            1 => DataLine::D1,
            2 => DataLine::D2,
            3 => DataLine::D3,
            4 => DataLine::D4,
            5 => DataLine::D5,
            6 => DataLine::D6,
            7 => DataLine::D7,
            _ => unreachable!(),
        }
    }
    /// Set data line 1 selection (`D1_SELD`).
    #[doc(alias = "D1_SELD")]
    #[inline]
    pub const fn set_d1_sel(self, line: DataLine) -> Self {
        Self((self.0 & !Self::D1_SELD) | (Self::D1_SELD & ((line as u32) << 4)))
    }
    /// Get data line 1 selection.
    #[inline]
    pub const fn d1_sel(self) -> DataLine {
        match (self.0 & Self::D1_SELD) >> 4 {
            0 => DataLine::D0,
            1 => DataLine::D1,
            2 => DataLine::D2,
            3 => DataLine::D3,
            4 => DataLine::D4,
            5 => DataLine::D5,
            6 => DataLine::D6,
            7 => DataLine::D7,
            _ => unreachable!(),
        }
    }
    /// Set data line 0 selection (`D0_SELD`).
    #[doc(alias = "D0_SELD")]
    #[inline]
    pub const fn set_d0_sel(self, line: DataLine) -> Self {
        Self((self.0 & !Self::D0_SELD) | (Self::D0_SELD & (line as u32)))
    }
    /// Get data line 0 selection.
    #[inline]
    pub const fn d0_sel(self) -> DataLine {
        match self.0 & Self::D0_SELD {
            0 => DataLine::D0,
            1 => DataLine::D1,
            2 => DataLine::D2,
            3 => DataLine::D3,
            4 => DataLine::D4,
            5 => DataLine::D5,
            6 => DataLine::D6,
            7 => DataLine::D7,
            _ => unreachable!(),
        }
    }
}

/// XSPI io control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IoControl(u32);

impl IoControl {
    const CS1_IO_CFG: u32 = 0x1 << 1;
    const CS0_IO_CFG: u32 = 0x1;

    /// Enable cs1 io (`CS1_IO_CFG`).
    #[doc(alias = "CS1_IO_CFG")]
    #[inline]
    pub const fn enable_cs1_io(self) -> Self {
        Self(self.0 & !Self::CS1_IO_CFG)
    }
    /// Disable cs1 io.
    #[inline]
    pub const fn disable_cs1_io(self) -> Self {
        Self(self.0 | Self::CS1_IO_CFG)
    }
    /// Check if cs1 io is enabled.
    #[inline]
    pub const fn is_cs1_io_enabled(self) -> bool {
        self.0 & Self::CS1_IO_CFG == 0
    }
    /// Enable cs0 io (`CS0_IO_CFG`).
    #[doc(alias = "CS0_IO_CFG")]
    #[inline]
    pub const fn enable_cs0_io(self) -> Self {
        Self(self.0 & !Self::CS0_IO_CFG)
    }
    /// Disable cs0 io.
    #[inline]
    pub const fn disable_cs0_io(self) -> Self {
        Self(self.0 | Self::CS0_IO_CFG)
    }
    /// Check if cs0 io is enabled.
    #[inline]
    pub const fn is_cs0_io_enabled(self) -> bool {
        self.0 & Self::CS0_IO_CFG == 0
    }
}

/// Pin pull-up/pull-down configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PinPull {
    /// Disabled (high-impedance state).
    Disabled,
    /// Pull-down with 33 KΩ to GND.
    PullDown = 2,
    /// Pull-up with 33 KΩ to VCC-IO.
    PullUp = 3,
}

/// Pin output drive strength configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PinDriveStrength {
    /// Level 0 (weakest drive).
    /// - 3.3V: 180Ω
    /// - 1.8V: 300Ω
    Level0,
    /// Level 1.
    /// - 3.3V: 90Ω
    /// - 1.8V: 150Ω
    Level1,
    /// Level 2.
    /// - 3.3V: 60Ω
    /// - 1.8V: 100Ω
    Level2,
    /// Level 3.
    /// - 3.3V: 45Ω
    /// - 1.8V: 75Ω
    Level3,
    /// Level 4.
    /// - 3.3V: 36Ω
    /// - 1.8V: 60Ω
    Level4,
    /// Level 5.
    /// - 3.3V: 30Ω
    /// - 1.8V: 50Ω
    Level5,
    /// Level 6.
    /// - 3.3V: 26Ω
    /// - 1.8V: 43Ω
    Level6,
    /// Level 7 (strongest drive).
    /// - 3.3V: 23Ω
    /// - 1.8V: 38Ω
    Level7,
}

/// XSPI chip select io configuration 1 register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CsIoConfig1(u32);

impl CsIoConfig1 {
    const D7_PIN_PULL: u32 = 0x3 << 28;
    const D7_PIN_DRV: u32 = 0x7 << 24;
    const D6_PIN_PULL: u32 = 0x3 << 20;
    const D6_PIN_DRV: u32 = 0x7 << 16;
    const D5_PIN_PULL: u32 = 0x3 << 12;
    const D5_PIN_DRV: u32 = 0x7 << 8;
    const D4_PIN_PULL: u32 = 0x3 << 4;
    const D4_PIN_DRV: u32 = 0x7;

    /// Set D7 pin pull configuration (`D7_PIN_PULL`).
    #[doc(alias = "D7_PIN_PULL")]
    #[inline]
    pub const fn set_d7_pin_pull(self, pull: PinPull) -> Self {
        Self((self.0 & !Self::D7_PIN_PULL) | (Self::D7_PIN_PULL & ((pull as u32) << 28)))
    }
    /// Get D7 pin pull configuration.
    #[inline]
    pub const fn d7_pin_pull(self) -> PinPull {
        match (self.0 & Self::D7_PIN_PULL) >> 28 {
            0 => PinPull::Disabled,
            2 => PinPull::PullDown,
            3 => PinPull::PullUp,
            _ => unreachable!(),
        }
    }
    /// Set D7 pin drive strength (`D7_PIN_DRV`).
    #[doc(alias = "D7_PIN_DRV")]
    #[inline]
    pub const fn set_d7_pin_drv(self, drv: PinDriveStrength) -> Self {
        Self((self.0 & !Self::D7_PIN_DRV) | (Self::D7_PIN_DRV & ((drv as u32) << 24)))
    }
    /// Get D7 pin drive strength.
    #[inline]
    pub const fn d7_pin_drv(self) -> PinDriveStrength {
        match (self.0 & Self::D7_PIN_DRV) >> 24 {
            0 => PinDriveStrength::Level0,
            1 => PinDriveStrength::Level1,
            2 => PinDriveStrength::Level2,
            3 => PinDriveStrength::Level3,
            4 => PinDriveStrength::Level4,
            5 => PinDriveStrength::Level5,
            6 => PinDriveStrength::Level6,
            7 => PinDriveStrength::Level7,
            _ => unreachable!(),
        }
    }
    /// Set D6 pin pull configuration (`D6_PIN_PULL`).
    #[doc(alias = "D6_PIN_PULL")]
    #[inline]
    pub const fn set_d6_pin_pull(self, pull: PinPull) -> Self {
        Self((self.0 & !Self::D6_PIN_PULL) | (Self::D6_PIN_PULL & ((pull as u32) << 20)))
    }
    /// Get D6 pin pull configuration.
    #[inline]
    pub const fn d6_pin_pull(self) -> PinPull {
        match (self.0 & Self::D6_PIN_PULL) >> 20 {
            0 => PinPull::Disabled,
            2 => PinPull::PullDown,
            3 => PinPull::PullUp,
            _ => unreachable!(),
        }
    }
    /// Set D6 pin drive strength (`D6_PIN_DRV`).
    #[doc(alias = "D6_PIN_DRV")]
    #[inline]
    pub const fn set_d6_pin_drv(self, drv: PinDriveStrength) -> Self {
        Self((self.0 & !Self::D6_PIN_DRV) | (Self::D6_PIN_DRV & ((drv as u32) << 16)))
    }
    /// Get D6 pin drive strength.
    #[inline]
    pub const fn d6_pin_drv(self) -> PinDriveStrength {
        match (self.0 & Self::D6_PIN_DRV) >> 16 {
            0 => PinDriveStrength::Level0,
            1 => PinDriveStrength::Level1,
            2 => PinDriveStrength::Level2,
            3 => PinDriveStrength::Level3,
            4 => PinDriveStrength::Level4,
            5 => PinDriveStrength::Level5,
            6 => PinDriveStrength::Level6,
            7 => PinDriveStrength::Level7,
            _ => unreachable!(),
        }
    }
    /// Set D5 pin pull configuration (`D5_PIN_PULL`).
    #[doc(alias = "D5_PIN_PULL")]
    #[inline]
    pub const fn set_d5_pin_pull(self, pull: PinPull) -> Self {
        Self((self.0 & !Self::D5_PIN_PULL) | (Self::D5_PIN_PULL & ((pull as u32) << 12)))
    }
    /// Get D5 pin pull configuration.
    #[inline]
    pub const fn d5_pin_pull(self) -> PinPull {
        match (self.0 & Self::D5_PIN_PULL) >> 12 {
            0 => PinPull::Disabled,
            2 => PinPull::PullDown,
            3 => PinPull::PullUp,
            _ => unreachable!(),
        }
    }
    /// Set D5 pin drive strength (`D5_PIN_DRV`).
    #[doc(alias = "D5_PIN_DRV")]
    #[inline]
    pub const fn set_d5_pin_drv(self, drv: PinDriveStrength) -> Self {
        Self((self.0 & !Self::D5_PIN_DRV) | (Self::D5_PIN_DRV & ((drv as u32) << 8)))
    }
    /// Get D5 pin drive strength.
    #[inline]
    pub const fn d5_pin_drv(self) -> PinDriveStrength {
        match (self.0 & Self::D5_PIN_DRV) >> 8 {
            0 => PinDriveStrength::Level0,
            1 => PinDriveStrength::Level1,
            2 => PinDriveStrength::Level2,
            3 => PinDriveStrength::Level3,
            4 => PinDriveStrength::Level4,
            5 => PinDriveStrength::Level5,
            6 => PinDriveStrength::Level6,
            7 => PinDriveStrength::Level7,
            _ => unreachable!(),
        }
    }
    /// Set D4 pin pull configuration (`D4_PIN_PULL`).
    #[doc(alias = "D4_PIN_PULL")]
    #[inline]
    pub const fn set_d4_pin_pull(self, pull: PinPull) -> Self {
        Self((self.0 & !Self::D4_PIN_PULL) | (Self::D4_PIN_PULL & ((pull as u32) << 4)))
    }
    /// Get D4 pin pull configuration.
    #[inline]
    pub const fn d4_pin_pull(self) -> PinPull {
        match (self.0 & Self::D4_PIN_PULL) >> 4 {
            0 => PinPull::Disabled,
            2 => PinPull::PullDown,
            3 => PinPull::PullUp,
            _ => unreachable!(),
        }
    }
    /// Set D4 pin drive strength (`D4_PIN_DRV`).
    #[doc(alias = "D4_PIN_DRV")]
    #[inline]
    pub const fn set_d4_pin_drv(self, drv: PinDriveStrength) -> Self {
        Self((self.0 & !Self::D4_PIN_DRV) | (Self::D4_PIN_DRV & (drv as u32)))
    }
    /// Get D4 pin drive strength.
    #[inline]
    pub const fn d4_pin_drv(self) -> PinDriveStrength {
        match self.0 & Self::D4_PIN_DRV {
            0 => PinDriveStrength::Level0,
            1 => PinDriveStrength::Level1,
            2 => PinDriveStrength::Level2,
            3 => PinDriveStrength::Level3,
            4 => PinDriveStrength::Level4,
            5 => PinDriveStrength::Level5,
            6 => PinDriveStrength::Level6,
            7 => PinDriveStrength::Level7,
            _ => unreachable!(),
        }
    }
}

/// XSPI chip select io configuration 2 register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CsIoConfig2(u32);

impl CsIoConfig2 {
    const D3_PIN_PULL: u32 = 0x3 << 28;
    const D3_PIN_DRV: u32 = 0x7 << 24;
    const D2_PIN_PULL: u32 = 0x3 << 20;
    const D2_PIN_DRV: u32 = 0x7 << 16;
    const D1_PIN_PULL: u32 = 0x3 << 12;
    const D1_PIN_DRV: u32 = 0x7 << 8;
    const D0_PIN_PULL: u32 = 0x3 << 4;
    const D0_PIN_DRV: u32 = 0x7;

    /// Set D3 pin pull configuration (`D3_PIN_PULL`).
    #[doc(alias = "D3_PIN_PULL")]
    #[inline]
    pub const fn set_d3_pin_pull(self, pull: PinPull) -> Self {
        Self((self.0 & !Self::D3_PIN_PULL) | (Self::D3_PIN_PULL & ((pull as u32) << 28)))
    }
    /// Get D3 pin pull configuration.
    #[inline]
    pub const fn d3_pin_pull(self) -> PinPull {
        match (self.0 & Self::D3_PIN_PULL) >> 28 {
            0 => PinPull::Disabled,
            2 => PinPull::PullDown,
            3 => PinPull::PullUp,
            _ => unreachable!(),
        }
    }
    /// Set D3 pin drive strength (`D3_PIN_DRV`).
    #[doc(alias = "D3_PIN_DRV")]
    #[inline]
    pub const fn set_d3_pin_drv(self, drv: PinDriveStrength) -> Self {
        Self((self.0 & !Self::D3_PIN_DRV) | (Self::D3_PIN_DRV & ((drv as u32) << 24)))
    }
    /// Get D3 pin drive strength.
    #[inline]
    pub const fn d3_pin_drv(self) -> PinDriveStrength {
        match (self.0 & Self::D3_PIN_DRV) >> 24 {
            0 => PinDriveStrength::Level0,
            1 => PinDriveStrength::Level1,
            2 => PinDriveStrength::Level2,
            3 => PinDriveStrength::Level3,
            4 => PinDriveStrength::Level4,
            5 => PinDriveStrength::Level5,
            6 => PinDriveStrength::Level6,
            7 => PinDriveStrength::Level7,
            _ => unreachable!(),
        }
    }
    /// Set D2 pin pull configuration (`D2_PIN_PULL`).
    #[doc(alias = "D2_PIN_PULL")]
    #[inline]
    pub const fn set_d2_pin_pull(self, pull: PinPull) -> Self {
        Self((self.0 & !Self::D2_PIN_PULL) | (Self::D2_PIN_PULL & ((pull as u32) << 20)))
    }
    /// Get D2 pin pull configuration.
    #[inline]
    pub const fn d2_pin_pull(self) -> PinPull {
        match (self.0 & Self::D2_PIN_PULL) >> 20 {
            0 => PinPull::Disabled,
            2 => PinPull::PullDown,
            3 => PinPull::PullUp,
            _ => unreachable!(),
        }
    }
    /// Set D2 pin drive strength (`D2_PIN_DRV`).
    #[doc(alias = "D2_PIN_DRV")]
    #[inline]
    pub const fn set_d2_pin_drv(self, drv: PinDriveStrength) -> Self {
        Self((self.0 & !Self::D2_PIN_DRV) | (Self::D2_PIN_DRV & ((drv as u32) << 16)))
    }
    /// Get D2 pin drive strength.
    #[inline]
    pub const fn d2_pin_drv(self) -> PinDriveStrength {
        match (self.0 & Self::D2_PIN_DRV) >> 16 {
            0 => PinDriveStrength::Level0,
            1 => PinDriveStrength::Level1,
            2 => PinDriveStrength::Level2,
            3 => PinDriveStrength::Level3,
            4 => PinDriveStrength::Level4,
            5 => PinDriveStrength::Level5,
            6 => PinDriveStrength::Level6,
            7 => PinDriveStrength::Level7,
            _ => unreachable!(),
        }
    }
    /// Set D1 pin pull configuration (`D1_PIN_PULL`).
    #[doc(alias = "D1_PIN_PULL")]
    #[inline]
    pub const fn set_d1_pin_pull(self, pull: PinPull) -> Self {
        Self((self.0 & !Self::D1_PIN_PULL) | (Self::D1_PIN_PULL & ((pull as u32) << 12)))
    }
    /// Get D1 pin pull configuration.
    #[inline]
    pub const fn d1_pin_pull(self) -> PinPull {
        match (self.0 & Self::D1_PIN_PULL) >> 12 {
            0 => PinPull::Disabled,
            2 => PinPull::PullDown,
            3 => PinPull::PullUp,
            _ => unreachable!(),
        }
    }
    /// Set D1 pin drive strength (`D1_PIN_DRV`).
    #[doc(alias = "D1_PIN_DRV")]
    #[inline]
    pub const fn set_d1_pin_drv(self, drv: PinDriveStrength) -> Self {
        Self((self.0 & !Self::D1_PIN_DRV) | (Self::D1_PIN_DRV & ((drv as u32) << 8)))
    }
    /// Get D1 pin drive strength.
    #[inline]
    pub const fn d1_pin_drv(self) -> PinDriveStrength {
        match (self.0 & Self::D1_PIN_DRV) >> 8 {
            0 => PinDriveStrength::Level0,
            1 => PinDriveStrength::Level1,
            2 => PinDriveStrength::Level2,
            3 => PinDriveStrength::Level3,
            4 => PinDriveStrength::Level4,
            5 => PinDriveStrength::Level5,
            6 => PinDriveStrength::Level6,
            7 => PinDriveStrength::Level7,
            _ => unreachable!(),
        }
    }
    /// Set D0 pin pull configuration (`D0_PIN_PULL`).
    #[doc(alias = "D0_PIN_PULL")]
    #[inline]
    pub const fn set_d0_pin_pull(self, pull: PinPull) -> Self {
        Self((self.0 & !Self::D0_PIN_PULL) | (Self::D0_PIN_PULL & ((pull as u32) << 4)))
    }
    /// Get D0 pin pull configuration.
    #[inline]
    pub const fn d0_pin_pull(self) -> PinPull {
        match (self.0 & Self::D0_PIN_PULL) >> 4 {
            0 => PinPull::Disabled,
            2 => PinPull::PullDown,
            3 => PinPull::PullUp,
            _ => unreachable!(),
        }
    }
    /// Set D0 pin drive strength (`D0_PIN_DRV`).
    #[doc(alias = "D0_PIN_DRV")]
    #[inline]
    pub const fn set_d0_pin_drv(self, drv: PinDriveStrength) -> Self {
        Self((self.0 & !Self::D0_PIN_DRV) | (Self::D0_PIN_DRV & (drv as u32)))
    }
    /// Get D0 pin drive strength.
    #[inline]
    pub const fn d0_pin_drv(self) -> PinDriveStrength {
        match self.0 & Self::D0_PIN_DRV {
            0 => PinDriveStrength::Level0,
            1 => PinDriveStrength::Level1,
            2 => PinDriveStrength::Level2,
            3 => PinDriveStrength::Level3,
            4 => PinDriveStrength::Level4,
            5 => PinDriveStrength::Level5,
            6 => PinDriveStrength::Level6,
            7 => PinDriveStrength::Level7,
            _ => unreachable!(),
        }
    }
}

/// XSPI chip select io configuration 3 register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CsIoConfig3(u32);

impl CsIoConfig3 {
    const CS_PIN_PULL: u32 = 0x3 << 28;
    const CS_PIN_DRV: u32 = 0x7 << 24;
    const DQS_PIN_PULL: u32 = 0x3 << 20;
    const DQS_PIN_DRV: u32 = 0x7 << 16;
    const CK_PIN_PULL: u32 = 0x3 << 12;
    const CK_PIN_DRV: u32 = 0x7 << 8;
    const CKN_PIN_PULL: u32 = 0x3 << 4;
    const CKN_PIN_DRV: u32 = 0x7;

    /// Set CS pin pull configuration (`CS_PIN_PULL`).
    #[doc(alias = "CS_PIN_PULL")]
    #[inline]
    pub const fn set_cs_pin_pull(self, pull: PinPull) -> Self {
        Self((self.0 & !Self::CS_PIN_PULL) | (Self::CS_PIN_PULL & ((pull as u32) << 28)))
    }
    /// Get CS pin pull configuration.
    #[inline]
    pub const fn cs_pin_pull(self) -> PinPull {
        match (self.0 & Self::CS_PIN_PULL) >> 28 {
            0 => PinPull::Disabled,
            2 => PinPull::PullDown,
            3 => PinPull::PullUp,
            _ => unreachable!(),
        }
    }
    /// Set CS pin drive strength (`CS_PIN_DRV`).
    #[doc(alias = "CS_PIN_DRV")]
    #[inline]
    pub const fn set_cs_pin_drv(self, drv: PinDriveStrength) -> Self {
        Self((self.0 & !Self::CS_PIN_DRV) | (Self::CS_PIN_DRV & ((drv as u32) << 24)))
    }
    /// Get CS pin drive strength.
    #[inline]
    pub const fn cs_pin_drv(self) -> PinDriveStrength {
        match (self.0 & Self::CS_PIN_DRV) >> 24 {
            0 => PinDriveStrength::Level0,
            1 => PinDriveStrength::Level1,
            2 => PinDriveStrength::Level2,
            3 => PinDriveStrength::Level3,
            4 => PinDriveStrength::Level4,
            5 => PinDriveStrength::Level5,
            6 => PinDriveStrength::Level6,
            7 => PinDriveStrength::Level7,
            _ => unreachable!(),
        }
    }
    /// Set DQS pin pull configuration (`DQS_PIN_PULL`).
    #[doc(alias = "DQS_PIN_PULL")]
    #[inline]
    pub const fn set_dqs_pin_pull(self, pull: PinPull) -> Self {
        Self((self.0 & !Self::DQS_PIN_PULL) | (Self::DQS_PIN_PULL & ((pull as u32) << 20)))
    }
    /// Get DQS pin pull configuration.
    #[inline]
    pub const fn dqs_pin_pull(self) -> PinPull {
        match (self.0 & Self::DQS_PIN_PULL) >> 20 {
            0 => PinPull::Disabled,
            2 => PinPull::PullDown,
            3 => PinPull::PullUp,
            _ => unreachable!(),
        }
    }
    /// Set DQS pin drive strength (`DQS_PIN_DRV`).
    #[doc(alias = "DQS_PIN_DRV")]
    #[inline]
    pub const fn set_dqs_pin_drv(self, drv: PinDriveStrength) -> Self {
        Self((self.0 & !Self::DQS_PIN_DRV) | (Self::DQS_PIN_DRV & ((drv as u32) << 16)))
    }
    /// Get DQS pin drive strength.
    #[inline]
    pub const fn dqs_pin_drv(self) -> PinDriveStrength {
        match (self.0 & Self::DQS_PIN_DRV) >> 16 {
            0 => PinDriveStrength::Level0,
            1 => PinDriveStrength::Level1,
            2 => PinDriveStrength::Level2,
            3 => PinDriveStrength::Level3,
            4 => PinDriveStrength::Level4,
            5 => PinDriveStrength::Level5,
            6 => PinDriveStrength::Level6,
            7 => PinDriveStrength::Level7,
            _ => unreachable!(),
        }
    }
    /// Set CK pin pull configuration (`CK_PIN_PULL`).
    #[doc(alias = "CK_PIN_PULL")]
    #[inline]
    pub const fn set_ck_pin_pull(self, pull: PinPull) -> Self {
        Self((self.0 & !Self::CK_PIN_PULL) | (Self::CK_PIN_PULL & ((pull as u32) << 12)))
    }
    /// Get CK pin pull configuration.
    #[inline]
    pub const fn ck_pin_pull(self) -> PinPull {
        match (self.0 & Self::CK_PIN_PULL) >> 12 {
            0 => PinPull::Disabled,
            2 => PinPull::PullDown,
            3 => PinPull::PullUp,
            _ => unreachable!(),
        }
    }
    /// Set CK pin drive strength (`CK_PIN_DRV`).
    #[doc(alias = "CK_PIN_DRV")]
    #[inline]
    pub const fn set_ck_pin_drv(self, drv: PinDriveStrength) -> Self {
        Self((self.0 & !Self::CK_PIN_DRV) | (Self::CK_PIN_DRV & ((drv as u32) << 8)))
    }
    /// Get CK pin drive strength.
    #[inline]
    pub const fn ck_pin_drv(self) -> PinDriveStrength {
        match (self.0 & Self::CK_PIN_DRV) >> 8 {
            0 => PinDriveStrength::Level0,
            1 => PinDriveStrength::Level1,
            2 => PinDriveStrength::Level2,
            3 => PinDriveStrength::Level3,
            4 => PinDriveStrength::Level4,
            5 => PinDriveStrength::Level5,
            6 => PinDriveStrength::Level6,
            7 => PinDriveStrength::Level7,
            _ => unreachable!(),
        }
    }
    /// Set CKN pin pull configuration (`CKN_PIN_PULL`).
    #[doc(alias = "CKN_PIN_PULL")]
    #[inline]
    pub const fn set_ckn_pin_pull(self, pull: PinPull) -> Self {
        Self((self.0 & !Self::CKN_PIN_PULL) | (Self::CKN_PIN_PULL & ((pull as u32) << 4)))
    }
    /// Get CKN pin pull configuration.
    #[inline]
    pub const fn ckn_pin_pull(self) -> PinPull {
        match (self.0 & Self::CKN_PIN_PULL) >> 4 {
            0 => PinPull::Disabled,
            2 => PinPull::PullDown,
            3 => PinPull::PullUp,
            _ => unreachable!(),
        }
    }
    /// Set CKN pin drive strength (`CKN_PIN_DRV`).
    #[doc(alias = "CKN_PIN_DRV")]
    #[inline]
    pub const fn set_ckn_pin_drv(self, drv: PinDriveStrength) -> Self {
        Self((self.0 & !Self::CKN_PIN_DRV) | (Self::CKN_PIN_DRV & (drv as u32)))
    }
    /// Get CKN pin drive strength.
    #[inline]
    pub const fn ckn_pin_drv(self) -> PinDriveStrength {
        match self.0 & Self::CKN_PIN_DRV {
            0 => PinDriveStrength::Level0,
            1 => PinDriveStrength::Level1,
            2 => PinDriveStrength::Level2,
            3 => PinDriveStrength::Level3,
            4 => PinDriveStrength::Level4,
            5 => PinDriveStrength::Level5,
            6 => PinDriveStrength::Level6,
            7 => PinDriveStrength::Level7,
            _ => unreachable!(),
        }
    }
}

/// XSPI chip select io configuration 4 register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CsIoConfig4(u32);

impl CsIoConfig4 {
    const DM_PIN_PULL: u32 = 0x3 << 4;
    const DM_PIN_DRV: u32 = 0x7;

    /// Set DM pin pull configuration (`DM_PIN_PULL`).
    #[doc(alias = "DM_PIN_PULL")]
    #[inline]
    pub const fn set_dm_pin_pull(self, pull: PinPull) -> Self {
        Self((self.0 & !Self::DM_PIN_PULL) | (Self::DM_PIN_PULL & ((pull as u32) << 4)))
    }
    /// Get DM pin pull configuration.
    #[inline]
    pub const fn dm_pin_pull(self) -> PinPull {
        match (self.0 & Self::DM_PIN_PULL) >> 4 {
            0 => PinPull::Disabled,
            2 => PinPull::PullDown,
            3 => PinPull::PullUp,
            _ => unreachable!(),
        }
    }
    /// Set DM pin drive strength (`DM_PIN_DRV`).
    #[doc(alias = "DM_PIN_DRV")]
    #[inline]
    pub const fn set_dm_pin_drv(self, drv: PinDriveStrength) -> Self {
        Self((self.0 & !Self::DM_PIN_DRV) | (Self::DM_PIN_DRV & (drv as u32)))
    }
    /// Get DM pin drive strength.
    #[inline]
    pub const fn dm_pin_drv(self) -> PinDriveStrength {
        match self.0 & Self::DM_PIN_DRV {
            0 => PinDriveStrength::Level0,
            1 => PinDriveStrength::Level1,
            2 => PinDriveStrength::Level2,
            3 => PinDriveStrength::Level3,
            4 => PinDriveStrength::Level4,
            5 => PinDriveStrength::Level5,
            6 => PinDriveStrength::Level6,
            7 => PinDriveStrength::Level7,
            _ => unreachable!(),
        }
    }
}

/// Training result calculation method.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TrainingPhaseCal {
    /// (temp1 + temp2) / 2, round down.
    RoundDown,
    /// ((temp1 + temp2) - 1) / 2 + 1, round up.
    RoundUp,
}

/// Training pattern selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TrainingPatternSel {
    /// Stuck address.
    StuckAddress,
    /// Random number.
    Random,
    /// High-low level inversion.
    Inversion,
    /// Custom.
    Custom,
}

/// XSPI training configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TrainingConfig(u32);

impl TrainingConfig {
    const TRAINING_PHASE_CAL: u32 = 0x1 << 24;
    const TRAINING_PATTERN_SEL: u32 = 0xF << 16;
    const DATA_LEN: u32 = 0xFFFF;

    /// Set training phase calculation method (`TRAINING_PHASE_CAL`).
    #[doc(alias = "TRAINING_PHASE_CAL")]
    #[inline]
    pub const fn set_training_phase_cal(self, cal: TrainingPhaseCal) -> Self {
        Self(
            (self.0 & !Self::TRAINING_PHASE_CAL)
                | (Self::TRAINING_PHASE_CAL & ((cal as u32) << 24)),
        )
    }
    /// Get training phase calculation method.
    #[inline]
    pub const fn training_phase_cal(self) -> TrainingPhaseCal {
        match (self.0 & Self::TRAINING_PHASE_CAL) >> 24 {
            0 => TrainingPhaseCal::RoundDown,
            1 => TrainingPhaseCal::RoundUp,
            _ => unreachable!(),
        }
    }
    /// Set training pattern selection (`TRAINING_PATTERN_SEL`).
    #[doc(alias = "TRAINING_PATTERN_SEL")]
    #[inline]
    pub const fn set_training_pattern_sel(self, pattern: TrainingPatternSel) -> Self {
        Self(
            (self.0 & !Self::TRAINING_PATTERN_SEL)
                | (Self::TRAINING_PATTERN_SEL & ((pattern as u32) << 16)),
        )
    }
    /// Get training pattern selection.
    #[inline]
    pub const fn training_pattern_sel(self) -> TrainingPatternSel {
        match (self.0 & Self::TRAINING_PATTERN_SEL) >> 16 {
            0 => TrainingPatternSel::StuckAddress,
            1 => TrainingPatternSel::Random,
            2 => TrainingPatternSel::Inversion,
            3 => TrainingPatternSel::Custom,
            _ => unreachable!(),
        }
    }
    /// Set training data length (`DATA_LEN`).
    ///
    /// Training data length in bytes.
    #[doc(alias = "DATA_LEN")]
    #[inline]
    pub const fn set_data_len(self, len: u16) -> Self {
        Self((self.0 & !Self::DATA_LEN) | (Self::DATA_LEN & (len as u32)))
    }
    /// Get training data length.
    #[inline]
    pub const fn data_len(self) -> u16 {
        (self.0 & Self::DATA_LEN) as u16
    }
}

/// IO configuration for instruction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IoCfg {
    /// 1 IO.
    OneIo,
    /// 2 IO.
    TwoIo,
    /// 4 IO.
    FourIo,
    /// 8 IO.
    EightIo,
}

/// XSPI look up table register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Lut(u32);

impl Lut {
    const INSTR1: u32 = 0x3F << 26;
    const IO_CFG1: u32 = 0x3 << 24;
    const OPERAND1: u32 = 0xFF << 16;
    const INSTR0: u32 = 0x3F << 10;
    const IO_CFG0: u32 = 0x3 << 8;
    const OPERAND0: u32 = 0xFF;

    /// Set instruction1 (`INSTR1`).
    ///
    /// Instruction for the first part of the LUT entry.
    #[doc(alias = "INSTR1")]
    #[inline]
    pub const fn set_instr1(self, instr: u8) -> Self {
        assert!(instr < 0x40, "Instruction1 out of range (expected 0..=63)");
        Self((self.0 & !Self::INSTR1) | (Self::INSTR1 & ((instr as u32) << 26)))
    }
    /// Get instruction1.
    #[inline]
    pub const fn instr1(self) -> u8 {
        ((self.0 & Self::INSTR1) >> 26) as u8
    }
    /// Set instruction1 IO configuration (`IO_CFG1`).
    #[doc(alias = "IO_CFG1")]
    #[inline]
    pub const fn set_io_cfg1(self, cfg: IoCfg) -> Self {
        Self((self.0 & !Self::IO_CFG1) | (Self::IO_CFG1 & ((cfg as u32) << 24)))
    }
    /// Get instruction1 IO configuration.
    #[inline]
    pub const fn io_cfg1(self) -> IoCfg {
        match (self.0 & Self::IO_CFG1) >> 24 {
            0 => IoCfg::OneIo,
            1 => IoCfg::TwoIo,
            2 => IoCfg::FourIo,
            3 => IoCfg::EightIo,
            _ => unreachable!(),
        }
    }
    /// Set instruction1 operand (`OPERAND1`).
    ///
    /// Operand configuration for instruction1.
    #[doc(alias = "OPERAND1")]
    #[inline]
    pub const fn set_operand1(self, operand: u8) -> Self {
        Self((self.0 & !Self::OPERAND1) | (Self::OPERAND1 & ((operand as u32) << 16)))
    }
    /// Get instruction1 operand.
    #[inline]
    pub const fn operand1(self) -> u8 {
        ((self.0 & Self::OPERAND1) >> 16) as u8
    }
    /// Set instruction0 (`INSTR0`).
    ///
    /// Instruction for the second part of the LUT entry.
    #[doc(alias = "INSTR0")]
    #[inline]
    pub const fn set_instr0(self, instr: u8) -> Self {
        assert!(instr < 0x40, "Instruction0 out of range (expected 0..=63)");
        Self((self.0 & !Self::INSTR0) | (Self::INSTR0 & ((instr as u32) << 10)))
    }
    /// Get instruction0.
    #[inline]
    pub const fn instr0(self) -> u8 {
        ((self.0 & Self::INSTR0) >> 10) as u8
    }
    /// Set instruction0 IO configuration (`IO_CFG0`).
    #[doc(alias = "IO_CFG0")]
    #[inline]
    pub const fn set_io_cfg0(self, cfg: IoCfg) -> Self {
        Self((self.0 & !Self::IO_CFG0) | (Self::IO_CFG0 & ((cfg as u32) << 8)))
    }
    /// Get instruction0 IO configuration.
    #[inline]
    pub const fn io_cfg0(self) -> IoCfg {
        match (self.0 & Self::IO_CFG0) >> 8 {
            0 => IoCfg::OneIo,
            1 => IoCfg::TwoIo,
            2 => IoCfg::FourIo,
            3 => IoCfg::EightIo,
            _ => unreachable!(),
        }
    }
    /// Set instruction0 operand (`OPERAND0`).
    ///
    /// Operand configuration for instruction0.
    #[doc(alias = "OPERAND0")]
    #[inline]
    pub const fn set_operand0(self, operand: u8) -> Self {
        Self((self.0 & !Self::OPERAND0) | (Self::OPERAND0 & (operand as u32)))
    }
    /// Get instruction0 operand.
    #[inline]
    pub const fn operand0(self) -> u8 {
        (self.0 & Self::OPERAND0) as u8
    }
}

/// XSPI debug select register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DebugSel(u32);

impl DebugSel {
    const DEBUG_SEL: u32 = 0xF;

    /// Set debug selection (`DEBUG_SEL`).
    ///
    /// Selects the debug signal to be output.
    #[doc(alias = "DEBUG_SEL")]
    #[inline]
    pub const fn set_debug_sel(self, sel: u8) -> Self {
        assert!(sel < 0x10, "Debug selection out of range (expected 0..=15)");
        Self((self.0 & !Self::DEBUG_SEL) | (Self::DEBUG_SEL & (sel as u32)))
    }
    /// Get debug selection.
    #[inline]
    pub const fn debug_sel(self) -> u8 {
        (self.0 & Self::DEBUG_SEL) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_should_panic;
    use core::mem::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, ctrl), 0x0);
        assert_eq!(offset_of!(RegisterBlock, clk), 0x4);
        assert_eq!(offset_of!(RegisterBlock, trans_ctrl), 0x8);
        assert_eq!(offset_of!(RegisterBlock, status), 0xC);
        assert_eq!(offset_of!(RegisterBlock, cs0_ctrl), 0x10);
        assert_eq!(offset_of!(RegisterBlock, cs0_dll_ctrl), 0x14);
        assert_eq!(offset_of!(RegisterBlock, cs1_ctrl), 0x18);
        assert_eq!(offset_of!(RegisterBlock, cs1_dll_ctrl), 0x1C);
        assert_eq!(offset_of!(RegisterBlock, int_en), 0x20);
        assert_eq!(offset_of!(RegisterBlock, int_status), 0x24);
        assert_eq!(offset_of!(RegisterBlock, fifo_ctrl), 0x28);
        assert_eq!(offset_of!(RegisterBlock, fifo_status), 0x2C);
        assert_eq!(offset_of!(RegisterBlock, start), 0x30);
        assert_eq!(offset_of!(RegisterBlock, addr), 0x34);
        assert_eq!(offset_of!(RegisterBlock, fmt_config), 0x38);
        assert_eq!(offset_of!(RegisterBlock, burst_type), 0x40);
        assert_eq!(offset_of!(RegisterBlock, rd_cmd_ctrl), 0x44);
        assert_eq!(offset_of!(RegisterBlock, dma_mode_ctrl), 0x48);
        assert_eq!(offset_of!(RegisterBlock, timeout_config), 0x50);
        assert_eq!(offset_of!(RegisterBlock, lock_config), 0x54);
        assert_eq!(offset_of!(RegisterBlock, lut_up), 0x58);
        assert_eq!(offset_of!(RegisterBlock, cs0_sequence), 0x60);
        assert_eq!(offset_of!(RegisterBlock, cs1_sequence), 0x64);
        assert_eq!(offset_of!(RegisterBlock, io_ctrl), 0x68);
        assert_eq!(offset_of!(RegisterBlock, cs0_io_cfg1), 0x70);
        assert_eq!(offset_of!(RegisterBlock, cs0_io_cfg2), 0x74);
        assert_eq!(offset_of!(RegisterBlock, cs0_io_cfg3), 0x78);
        assert_eq!(offset_of!(RegisterBlock, cs0_io_cfg4), 0x7C);
        assert_eq!(offset_of!(RegisterBlock, cs1_io_cfg1), 0x80);
        assert_eq!(offset_of!(RegisterBlock, cs1_io_cfg2), 0x84);
        assert_eq!(offset_of!(RegisterBlock, cs1_io_cfg3), 0x88);
        assert_eq!(offset_of!(RegisterBlock, cs1_io_cfg4), 0x8C);
        assert_eq!(offset_of!(RegisterBlock, training_cfg), 0x90);
        assert_eq!(offset_of!(RegisterBlock, training_pattern), 0x94);
        assert_eq!(offset_of!(RegisterBlock, luts), 0x100);
        assert_eq!(offset_of!(RegisterBlock, tx_data), 0x200);
        assert_eq!(offset_of!(RegisterBlock, rx_data), 0x300);
        assert_eq!(offset_of!(RegisterBlock, debug), 0x400);
        assert_eq!(offset_of!(RegisterBlock, debug_sel), 0x404);
        assert_eq!(offset_of!(RegisterBlock, version), 0xFFC);
    }

    #[test]
    fn struct_control_functions() {
        let mut val = Control(0x0);

        val = val.enable_axi_arbiter();
        assert!(val.is_axi_arbiter_enabled());
        assert_eq!(val.0, 0x0004_0000);
        val = val.disable_axi_arbiter();
        assert!(!val.is_axi_arbiter_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_col_addr_ctrl(ColAddrCtrl::WordAddr);
        assert_eq!(val.col_addr_ctrl(), ColAddrCtrl::WordAddr);
        assert_eq!(val.0, 0x0002_0000);
        val = val.set_col_addr_ctrl(ColAddrCtrl::ByteAddr);
        assert_eq!(val.col_addr_ctrl(), ColAddrCtrl::ByteAddr);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_pin_width(PinWidth::X16);
        assert_eq!(val.pin_width(), PinWidth::X16);
        assert_eq!(val.0, 0x0001_0000);
        val = val.set_pin_width(PinWidth::X8);
        assert_eq!(val.pin_width(), PinWidth::X8);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_boundary_size(BoundarySize::Size1K);
        assert_eq!(val.boundary_size(), BoundarySize::Size1K);
        assert_eq!(val.0, 0x0000_2000);
        val = val.set_boundary_size(BoundarySize::Size2K);
        assert_eq!(val.boundary_size(), BoundarySize::Size2K);
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_boundary();
        assert!(val.is_boundary_enabled());
        assert_eq!(val.0, 0x0000_1000);
        val = val.disable_boundary();
        assert!(!val.is_boundary_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_reset();
        assert!(val.is_reset_enabled());
        assert_eq!(val.0, 0x0000_0200);
        val = val.disable_reset();
        assert!(!val.is_reset_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_reset_level(true);
        assert!(val.reset_level());
        assert_eq!(val.0, 0x0000_0100);
        val = val.set_reset_level(false);
        assert!(!val.reset_level());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_timeout();
        assert!(val.is_timeout_enabled());
        assert_eq!(val.0, 0x0000_0080);
        val = val.disable_timeout();
        assert!(!val.is_timeout_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_parallel_mode();
        assert!(val.is_parallel_mode_enabled());
        assert_eq!(val.0, 0x0000_0040);
        val = val.disable_parallel_mode();
        assert!(!val.is_parallel_mode_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_xspi_mode(XspiMode::SPI);
        assert_eq!(val.xspi_mode(), XspiMode::SPI);
        assert_eq!(val.0, 0x0000_0030);
        val = val.set_xspi_mode(XspiMode::OPI);
        assert_eq!(val.xspi_mode(), XspiMode::OPI);
        assert_eq!(val.0, 0x0000_0020);
        val = val.set_xspi_mode(XspiMode::Hyperbus);
        assert_eq!(val.xspi_mode(), XspiMode::Hyperbus);
        assert_eq!(val.0, 0x0000_0010);
        val = val.set_xspi_mode(XspiMode::Xccela);
        assert_eq!(val.xspi_mode(), XspiMode::Xccela);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_axi_wrap_burst_ctrl(true);
        assert!(val.axi_wrap_burst_ctrl());
        assert_eq!(val.0, 0x0000_0008);
        val = val.set_axi_wrap_burst_ctrl(false);
        assert!(!val.axi_wrap_burst_ctrl());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_xip();
        assert!(val.is_xip_enabled());
        assert_eq!(val.0, 0x0000_0004);
        val = val.disable_xip();
        assert!(!val.is_xip_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_idle_lp();
        assert!(val.is_idle_lp_enabled());
        assert_eq!(val.0, 0x0000_0002);
        val = val.disable_idle_lp();
        assert!(!val.is_idle_lp_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_xspi();
        assert!(val.is_xspi_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_xspi();
        assert!(!val.is_xspi_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_clock_functions() {
        let mut val = Clock(0x0);

        val = val.set_clock_divider(ClockDivider::Divider2);
        assert_eq!(val.clock_divider(), ClockDivider::Divider2);
        assert_eq!(val.0, 0x0000_1000);
        val = val.set_clock_divider(ClockDivider::Divider1);
        assert_eq!(val.clock_divider(), ClockDivider::Divider1);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_clk_div1(0xF);
        assert_eq!(val.clk_div1(), 0xF);
        assert_eq!(val.0, 0x0000_0F00);

        val = Clock(0x0).set_clk_div2(0xFF);
        assert_eq!(val.clk_div2(), 0xFF);
        assert_eq!(val.0, 0x0000_00FF);
    }

    test_should_panic!((
        test_clock_set_clk_div1_panic,
        Clock(0x0).set_clk_div1(0x10),
        "Clock divider 1 out of range (expected 0..=15)"
    ),);

    #[test]
    fn struct_trans_control_functions() {
        let mut val = TransControl(0x0);

        val = val.set_opi_hold_ex(0xF);
        assert_eq!(val.opi_hold_ex(), 0xF);
        assert_eq!(val.0, 0xF000_0000);

        val = TransControl(0x0).set_dqs_clk_gating(DqsClkGating::Bypass);
        assert_eq!(val.dqs_clk_gating(), DqsClkGating::Bypass);
        assert_eq!(val.0, 0x0300_0000);
        val = val.set_dqs_clk_gating(DqsClkGating::Delay);
        assert_eq!(val.dqs_clk_gating(), DqsClkGating::Delay);
        assert_eq!(val.0, 0x0200_0000);
        val = val.set_dqs_clk_gating(DqsClkGating::Before);
        assert_eq!(val.dqs_clk_gating(), DqsClkGating::Before);
        assert_eq!(val.0, 0x0100_0000);
        val = val.set_dqs_clk_gating(DqsClkGating::Normal);
        assert_eq!(val.dqs_clk_gating(), DqsClkGating::Normal);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_cs_rd_hold(0xF);
        assert_eq!(val.cs_rd_hold(), 0xF);
        assert_eq!(val.0, 0x00F0_0000);

        val = TransControl(0x0).set_cs_wr_hold(0xF);
        assert_eq!(val.cs_wr_hold(), 0xF);
        assert_eq!(val.0, 0x000F_0000);

        val = TransControl(0x0).set_cs_setup(0xF);
        assert_eq!(val.cs_setup(), 0xF);
        assert_eq!(val.0, 0x0000_F000);

        val = TransControl(0x0).enable_jump_ins();
        assert!(val.is_jump_ins_enabled());
        assert_eq!(val.0, 0x0000_0800);
        val = val.disable_jump_ins();
        assert!(!val.is_jump_ins_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_dummy_fill_type(DummyFillType::Fill1);
        assert_eq!(val.dummy_fill_type(), DummyFillType::Fill1);
        assert_eq!(val.0, 0x0000_0100);
        val = val.set_dummy_fill_type(DummyFillType::Fill0);
        assert_eq!(val.dummy_fill_type(), DummyFillType::Fill0);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_cs_level(CsLevel::High);
        assert_eq!(val.cs_level(), CsLevel::High);
        assert_eq!(val.0, 0x0000_0080);
        val = val.set_cs_level(CsLevel::Low);
        assert_eq!(val.cs_level(), CsLevel::Low);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_cs_ctrl_mode(CsCtrlMode::Software);
        assert_eq!(val.cs_ctrl_mode(), CsCtrlMode::Software);
        assert_eq!(val.0, 0x0000_0040);
        val = val.set_cs_ctrl_mode(CsCtrlMode::SpiController);
        assert_eq!(val.cs_ctrl_mode(), CsCtrlMode::SpiController);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_cs_sel(CsSel::Cs1);
        assert_eq!(val.cs_sel(), CsSel::Cs1);
        assert_eq!(val.0, 0x0000_0010);
        val = val.set_cs_sel(CsSel::Cs0);
        assert_eq!(val.cs_sel(), CsSel::Cs0);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_cs_pol(CsPolarity::IdleLow);
        assert_eq!(val.cs_pol(), CsPolarity::IdleLow);
        assert_eq!(val.0, 0x0000_0004);
        val = val.set_cs_pol(CsPolarity::IdleHigh);
        assert_eq!(val.cs_pol(), CsPolarity::IdleHigh);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_clk_pol(Polarity::IdleHigh);
        assert_eq!(val.clk_pol(), Polarity::IdleHigh);
        assert_eq!(val.0, 0x0000_0002);
        val = val.set_clk_pol(Polarity::IdleLow);
        assert_eq!(val.clk_pol(), Polarity::IdleLow);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_clk_pha(Phase::CaptureOnSecondTransition);
        assert_eq!(val.clk_pha(), Phase::CaptureOnSecondTransition);
        assert_eq!(val.0, 0x0000_0001);
        val = val.set_clk_pha(Phase::CaptureOnFirstTransition);
        assert_eq!(val.clk_pha(), Phase::CaptureOnFirstTransition);
        assert_eq!(val.0, 0x0000_0000);
    }

    test_should_panic!(
        (
            test_trans_control_set_opi_hold_ex_panic,
            TransControl(0x0).set_opi_hold_ex(0x10),
            "OPI hold extension out of range (expected 0..=15)"
        ),
        (
            test_trans_control_set_cs_rd_hold_panic,
            TransControl(0x0).set_cs_rd_hold(0x10),
            "CS read hold time out of range (expected 0..=15)"
        ),
        (
            test_trans_control_set_cs_wr_hold_panic,
            TransControl(0x0).set_cs_wr_hold(0x10),
            "CS write hold time out of range (expected 0..=15)"
        ),
        (
            test_trans_control_set_cs_setup_panic,
            TransControl(0x0).set_cs_setup(0x10),
            "CS setup control out of range (expected 0..=15)"
        ),
    );

    #[test]
    fn struct_status_functions() {
        let mut val = Status(0x0000_0004);
        assert!(val.is_ahb_transfer());

        val = Status(0x0000_0002);
        assert!(val.is_axi_transfer());

        val = Status(0x0000_0001);
        assert!(val.is_busy());
    }

    #[test]
    fn struct_cs_control_functions() {
        let mut val = CsControl(0x0);

        val = val.set_wr_delay_chain_sel(0x1F);
        assert_eq!(val.wr_delay_chain_sel(), 0x1F);
        assert_eq!(val.0, 0x1F00_0000);

        val = CsControl(0x0).set_wr_phase_sel(true);
        assert!(val.wr_phase_sel());
        assert_eq!(val.0, 0x0020_0000);
        val = val.set_wr_phase_sel(false);
        assert!(!val.wr_phase_sel());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_wr_delay_chain();
        assert!(val.is_wr_delay_chain_enabled());
        assert_eq!(val.0, 0x0010_0000);
        val = val.disable_wr_delay_chain();
        assert!(!val.is_wr_delay_chain_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_rd_phase(RdPha::Deg360);
        assert_eq!(val.rd_phase(), RdPha::Deg360);
        assert_eq!(val.0, 0x0003_0000);
        val = val.set_rd_phase(RdPha::Deg180);
        assert_eq!(val.rd_phase(), RdPha::Deg180);
        assert_eq!(val.0, 0x0002_0000);
        val = val.set_rd_phase(RdPha::Deg90);
        assert_eq!(val.rd_phase(), RdPha::Deg90);
        assert_eq!(val.0, 0x0001_0000);
        val = val.set_rd_phase(RdPha::Deg0);
        assert_eq!(val.rd_phase(), RdPha::Deg0);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_rd_delay_cycle(0x7);
        assert_eq!(val.rd_delay_cycle(), 0x7);
        assert_eq!(val.0, 0x0000_7000);

        val = CsControl(0x0).set_rd_delay_chain_sel(0x1F);
        assert_eq!(val.rd_delay_chain_sel(), 0x1F);
        assert_eq!(val.0, 0x0000_01F0);

        val = CsControl(0x0).set_rd_valid_control(true);
        assert!(val.rd_valid_control());
        assert_eq!(val.0, 0x0000_0008);
        val = val.set_rd_valid_control(false);
        assert!(!val.rd_valid_control());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_rd_delay_chain();
        assert!(val.is_rd_delay_chain_enabled());
        assert_eq!(val.0, 0x0000_0004);
        val = val.disable_rd_delay_chain();
        assert!(!val.is_rd_delay_chain_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_rd_sample_ctrl(RdSampleCtrl::DqsDelayChain);
        assert_eq!(val.rd_sample_ctrl(), RdSampleCtrl::DqsDelayChain);
        assert_eq!(val.0, 0x0000_0002);
        val = val.set_rd_sample_ctrl(RdSampleCtrl::InternalDelayChain);
        assert_eq!(val.rd_sample_ctrl(), RdSampleCtrl::InternalDelayChain);
        assert_eq!(val.0, 0x0000_0001);
        val = val.set_rd_sample_ctrl(RdSampleCtrl::DqsDll);
        assert_eq!(val.rd_sample_ctrl(), RdSampleCtrl::DqsDll);
        assert_eq!(val.0, 0x0000_0000);
    }

    test_should_panic!(
        (
            test_cs_control_set_wr_delay_chain_sel_panic,
            CsControl(0x0).set_wr_delay_chain_sel(0x20),
            "Write delay chain selection out of range (expected 0..=31)"
        ),
        (
            test_cs_control_set_rd_delay_chain_sel_panic,
            CsControl(0x0).set_rd_delay_chain_sel(0x20),
            "Read delay chain selection out of range (expected 0..=31)"
        ),
    );

    #[test]
    fn struct_cs_dll_control_functions() {
        let mut val = CsDllControl(0x0);

        val = val.set_force_lock(true);
        assert!(val.force_lock());
        assert_eq!(val.0, 0x2000_0000);
        val = val.set_force_lock(false);
        assert!(!val.force_lock());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_atb();
        assert!(val.is_atb_enabled());
        assert_eq!(val.0, 0x1000_0000);
        val = val.disable_atb();
        assert!(!val.is_atb_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_atbsel(0x7);
        assert_eq!(val.atbsel(), 0x7);
        assert_eq!(val.0, 0x0700_0000);

        val = CsDllControl(0x0).set_bypass(Bypass::Range3);
        assert_eq!(val.bypass(), Bypass::Range3);
        assert_eq!(val.0, 0x0030_0000);
        val = val.set_bypass(Bypass::Range2);
        assert_eq!(val.bypass(), Bypass::Range2);
        assert_eq!(val.0, 0x0020_0000);
        val = val.set_bypass(Bypass::Range1);
        assert_eq!(val.bypass(), Bypass::Range1);
        assert_eq!(val.0, 0x0010_0000);
        val = val.set_bypass(Bypass::Range0);
        assert_eq!(val.bypass(), Bypass::Range0);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_delay(0x3);
        assert_eq!(val.delay(), 0x3);
        assert_eq!(val.0, 0x0003_0000);

        val = CsDllControl(0x0).set_icp(Icp::Range200To266M);
        assert_eq!(val.icp(), Icp::Range200To266M);
        assert_eq!(val.0, 0x0000_3000);
        val = val.set_icp(Icp::Range150To200M);
        assert_eq!(val.icp(), Icp::Range150To200M);
        assert_eq!(val.0, 0x0000_2000);
        val = val.set_icp(Icp::Range100To150M);
        assert_eq!(val.icp(), Icp::Range100To150M);
        assert_eq!(val.0, 0x0000_1000);
        val = val.set_icp(Icp::Range50To100M);
        assert_eq!(val.icp(), Icp::Range50To100M);
        assert_eq!(val.0, 0x0000_0000);

        for i in 0..16 {
            let (sel, reg_val) = match i {
                0 => (PhaseSel::Deg22_5, 0x0000_0000),
                1 => (PhaseSel::Deg45, 0x0000_0100),
                2 => (PhaseSel::Deg67_5, 0x0000_0200),
                3 => (PhaseSel::Deg90, 0x0000_0300),
                4 => (PhaseSel::Deg112_5, 0x0000_0400),
                5 => (PhaseSel::Deg135, 0x0000_0500),
                6 => (PhaseSel::Deg157_5, 0x0000_0600),
                7 => (PhaseSel::Deg180, 0x0000_0700),
                8 => (PhaseSel::Deg202_5, 0x0000_0800),
                9 => (PhaseSel::Deg225, 0x0000_0900),
                10 => (PhaseSel::Deg247_5, 0x0000_0A00),
                11 => (PhaseSel::Deg270, 0x0000_0B00),
                12 => (PhaseSel::Deg292_5, 0x0000_0C00),
                13 => (PhaseSel::Deg315, 0x0000_0D00),
                14 => (PhaseSel::Deg337_5, 0x0000_0E00),
                15 => (PhaseSel::Bypass, 0x0000_0F00),
                _ => unreachable!(),
            };

            val = val.set_phase_sel(sel);
            assert_eq!(val.phase_sel(), sel);
            assert_eq!(val.0, reg_val);
        }

        val = CsDllControl(0x0).enable_lvs();
        assert!(val.is_lvs_enabled());
        assert_eq!(val.0, 0x0000_0020);
        val = val.disable_lvs();
        assert!(!val.is_lvs_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_ldo();
        assert!(val.is_ldo_enabled());
        assert_eq!(val.0, 0x0000_0010);
        val = val.disable_ldo();
        assert!(!val.is_ldo_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_bypass();
        assert!(val.is_bypass_enabled());
        assert_eq!(val.0, 0x0000_0008);
        val = val.disable_bypass();
        assert!(!val.is_bypass_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_cp();
        assert!(val.is_cp_enabled());
        assert_eq!(val.0, 0x0000_0004);
        val = val.disable_cp();
        assert!(!val.is_cp_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_vcdl();
        assert!(val.is_vcdl_enabled());
        assert_eq!(val.0, 0x0000_0002);
        val = val.disable_vcdl();
        assert!(!val.is_vcdl_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_dll();
        assert!(val.is_dll_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_dll();
        assert!(!val.is_dll_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    test_should_panic!(
        (
            test_cs_dll_control_set_atbsel_panic,
            CsDllControl(0x0).set_atbsel(0x8),
            "REG_ATBSEL out of range (expected 0..=7)"
        ),
        (
            test_cs_dll_control_set_delay_panic,
            CsDllControl(0x0).set_delay(0x4),
            "Delay out of range (expected 0..=3)"
        ),
    );

    #[test]
    fn struct_int_enable_functions() {
        let mut val = IntEnable(0x0);

        val = val.enable_xip_error();
        assert!(val.is_xip_error_enabled());
        assert_eq!(val.0, 0x0100_0000);
        val = val.disable_xip_error();
        assert!(!val.is_xip_error_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_axi_error();
        assert!(val.is_axi_error_enabled());
        assert_eq!(val.0, 0x0080_0000);
        val = val.disable_axi_error();
        assert!(!val.is_axi_error_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_opi_error();
        assert!(val.is_opi_error_enabled());
        assert_eq!(val.0, 0x0040_0000);
        val = val.disable_opi_error();
        assert!(!val.is_opi_error_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_hyperbus_error();
        assert!(val.is_hyperbus_error_enabled());
        assert_eq!(val.0, 0x0020_0000);
        val = val.disable_hyperbus_error();
        assert!(!val.is_hyperbus_error_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_xccela_error();
        assert!(val.is_xccela_error_enabled());
        assert_eq!(val.0, 0x0010_0000);
        val = val.disable_xccela_error();
        assert!(!val.is_xccela_error_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_axi_trans_error();
        assert!(val.is_axi_trans_error_enabled());
        assert_eq!(val.0, 0x0008_0000);
        val = val.disable_axi_trans_error();
        assert!(!val.is_axi_trans_error_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_ahb_trans_error();
        assert!(val.is_ahb_trans_error_enabled());
        assert_eq!(val.0, 0x0004_0000);
        val = val.disable_ahb_trans_error();
        assert!(!val.is_ahb_trans_error_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_lut_instr_error();
        assert!(val.is_lut_instr_error_enabled());
        assert_eq!(val.0, 0x0002_0000);
        val = val.disable_lut_instr_error();
        assert!(!val.is_lut_instr_error_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_lut_addr_operand_error();
        assert!(val.is_lut_addr_operand_error_enabled());
        assert_eq!(val.0, 0x0001_0000);
        val = val.disable_lut_addr_operand_error();
        assert!(!val.is_lut_addr_operand_error_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_cs1_timeout();
        assert!(val.is_cs1_timeout_enabled());
        assert_eq!(val.0, 0x0000_8000);
        val = val.disable_cs1_timeout();
        assert!(!val.is_cs1_timeout_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_cs0_timeout();
        assert!(val.is_cs0_timeout_enabled());
        assert_eq!(val.0, 0x0000_4000);
        val = val.disable_cs0_timeout();
        assert!(!val.is_cs0_timeout_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_cs1_done();
        assert!(val.is_cs1_done_enabled());
        assert_eq!(val.0, 0x0000_2000);
        val = val.disable_cs1_done();
        assert!(!val.is_cs1_done_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_cs0_done();
        assert!(val.is_cs0_done_enabled());
        assert_eq!(val.0, 0x0000_1000);
        val = val.disable_cs0_done();
        assert!(!val.is_cs0_done_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_tx_fifo_underflow();
        assert!(val.is_tx_fifo_underflow_enabled());
        assert_eq!(val.0, 0x0000_0800);
        val = val.disable_tx_fifo_underflow();
        assert!(!val.is_tx_fifo_underflow_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_tx_fifo_overflow();
        assert!(val.is_tx_fifo_overflow_enabled());
        assert_eq!(val.0, 0x0000_0400);
        val = val.disable_tx_fifo_overflow();
        assert!(!val.is_tx_fifo_overflow_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_rx_fifo_underflow();
        assert!(val.is_rx_fifo_underflow_enabled());
        assert_eq!(val.0, 0x0000_0200);
        val = val.disable_rx_fifo_underflow();
        assert!(!val.is_rx_fifo_underflow_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_rx_fifo_overflow();
        assert!(val.is_rx_fifo_overflow_enabled());
        assert_eq!(val.0, 0x0000_0100);
        val = val.disable_rx_fifo_overflow();
        assert!(!val.is_rx_fifo_overflow_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_tx_fifo_full();
        assert!(val.is_tx_fifo_full_enabled());
        assert_eq!(val.0, 0x0000_0040);
        val = val.disable_tx_fifo_full();
        assert!(!val.is_tx_fifo_full_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_tx_fifo_empty();
        assert!(val.is_tx_fifo_empty_enabled());
        assert_eq!(val.0, 0x0000_0020);
        val = val.disable_tx_fifo_empty();
        assert!(!val.is_tx_fifo_empty_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_tx_fifo_empty_req();
        assert!(val.is_tx_fifo_empty_req_enabled());
        assert_eq!(val.0, 0x0000_0010);
        val = val.disable_tx_fifo_empty_req();
        assert!(!val.is_tx_fifo_empty_req_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_rx_fifo_full();
        assert!(val.is_rx_fifo_full_enabled());
        assert_eq!(val.0, 0x0000_0004);
        val = val.disable_rx_fifo_full();
        assert!(!val.is_rx_fifo_full_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_rx_fifo_empty();
        assert!(val.is_rx_fifo_empty_enabled());
        assert_eq!(val.0, 0x0000_0002);
        val = val.disable_rx_fifo_empty();
        assert!(!val.is_rx_fifo_empty_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_rx_fifo_empty_req();
        assert!(val.is_rx_fifo_empty_req_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_rx_fifo_empty_req();
        assert!(!val.is_rx_fifo_empty_req_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_int_status_functions() {
        let mut val = IntStatus(0x0).clear_xip_err();
        assert!(val.is_xip_err_pending());
        assert_eq!(val.0, 0x0100_0000);

        val = IntStatus(0x0).clear_axi_err();
        assert!(val.is_axi_err_pending());
        assert_eq!(val.0, 0x0080_0000);

        val = IntStatus(0x0).clear_opi_err();
        assert!(val.is_opi_err_pending());
        assert_eq!(val.0, 0x0040_0000);

        val = IntStatus(0x0).clear_hyperbus_err();
        assert!(val.is_hyperbus_err_pending());
        assert_eq!(val.0, 0x0020_0000);

        val = IntStatus(0x0).clear_xccela_err();
        assert!(val.is_xccela_err_pending());
        assert_eq!(val.0, 0x0010_0000);

        val = IntStatus(0x0).clear_axi_trans_err();
        assert!(val.is_axi_trans_err_pending());
        assert_eq!(val.0, 0x0008_0000);

        val = IntStatus(0x0).clear_ahb_trans_err();
        assert!(val.is_ahb_trans_err_pending());
        assert_eq!(val.0, 0x0004_0000);

        val = IntStatus(0x0).clear_lut_instr_err();
        assert!(val.is_lut_instr_err_pending());
        assert_eq!(val.0, 0x0002_0000);

        val = IntStatus(0x0).clear_lut_addr_operand_err();
        assert!(val.is_lut_addr_operand_err_pending());
        assert_eq!(val.0, 0x0001_0000);

        val = IntStatus(0x0).clear_cs1_timeout();
        assert!(val.is_cs1_timeout_pending());
        assert_eq!(val.0, 0x0000_8000);

        val = IntStatus(0x0).clear_cs0_timeout();
        assert!(val.is_cs0_timeout_pending());
        assert_eq!(val.0, 0x0000_4000);

        val = IntStatus(0x0).clear_cs1_done();
        assert!(val.is_cs1_done_pending());
        assert_eq!(val.0, 0x0000_2000);

        val = IntStatus(0x0).clear_cs0_done();
        assert!(val.is_cs0_done_pending());
        assert_eq!(val.0, 0x0000_1000);

        val = IntStatus(0x0).clear_tx_fifo_underflow();
        assert!(val.is_tx_fifo_underflow_pending());
        assert_eq!(val.0, 0x0000_0800);

        val = IntStatus(0x0).clear_tx_fifo_overflow();
        assert!(val.is_tx_fifo_overflow_pending());
        assert_eq!(val.0, 0x0000_0400);

        val = IntStatus(0x0).clear_rx_fifo_underflow();
        assert!(val.is_rx_fifo_underflow_pending());
        assert_eq!(val.0, 0x0000_0200);

        val = IntStatus(0x0).clear_rx_fifo_overflow();
        assert!(val.is_rx_fifo_overflow_pending());
        assert_eq!(val.0, 0x0000_0100);

        val = IntStatus(0x0).clear_tx_fifo_full();
        assert!(val.is_tx_fifo_full_pending());
        assert_eq!(val.0, 0x0000_0040);

        val = IntStatus(0x0).clear_tx_fifo_empty();
        assert!(val.is_tx_fifo_empty_pending());
        assert_eq!(val.0, 0x0000_0020);

        val = IntStatus(0x0).clear_tx_fifo_ready();
        assert!(val.is_tx_fifo_ready_pending());
        assert_eq!(val.0, 0x0000_0010);

        val = IntStatus(0x0).clear_rx_fifo_full();
        assert!(val.is_rx_fifo_full_pending());
        assert_eq!(val.0, 0x0000_0004);

        val = IntStatus(0x0).clear_rx_fifo_empty();
        assert!(val.is_rx_fifo_empty_pending());
        assert_eq!(val.0, 0x0000_0002);

        val = IntStatus(0x0).clear_rx_fifo_ready();
        assert!(val.is_rx_fifo_ready_pending());
        assert_eq!(val.0, 0x0000_0001);
    }

    #[test]
    fn struct_fifo_control_functions() {
        let mut val = FifoControl(0x0);

        val = val.set_tx_fifo_reset(true);
        assert!(val.tx_fifo_reset());
        assert_eq!(val.0, 0x8000_0000);
        val = val.set_tx_fifo_reset(false);
        assert!(!val.tx_fifo_reset());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_tx_dma_req();
        assert!(val.is_tx_dma_req_enabled());
        assert_eq!(val.0, 0x0100_0000);
        val = val.disable_tx_dma_req();
        assert!(!val.is_tx_dma_req_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_tx_trigger_level(0x7F);
        assert_eq!(val.tx_trigger_level(), 0x7F);
        assert_eq!(val.0, 0x007F_0000);

        val = FifoControl(0x0).set_rx_fifo_reset(true);
        assert!(val.rx_fifo_reset());
        assert_eq!(val.0, 0x0000_8000);
        val = val.set_rx_fifo_reset(false);
        assert!(!val.rx_fifo_reset());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_rx_dma_req();
        assert!(val.is_rx_dma_req_enabled());
        assert_eq!(val.0, 0x0000_0100);
        val = val.disable_rx_dma_req();
        assert!(!val.is_rx_dma_req_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_rx_trigger_level(0x7F);
        assert_eq!(val.rx_trigger_level(), 0x7F);
        assert_eq!(val.0, 0x0000_007F);
    }

    test_should_panic!(
        (
            test_fifo_control_set_tx_trigger_level_panic,
            FifoControl(0x0).set_tx_trigger_level(0x80),
            "TX trigger level out of range (expected 0..=127)"
        ),
        (
            test_fifo_control_set_rx_trigger_level_panic,
            FifoControl(0x0).set_rx_trigger_level(0x80),
            "RX trigger level out of range (expected 0..=127)"
        ),
    );

    #[test]
    fn struct_fifo_status_functions() {
        let mut val = FifoStatus(0xF0FF_0000);
        assert!(val.is_tx_buffer_write_enabled());
        assert_eq!(val.tx_buffer_count(), 0x7);
        assert_eq!(val.tx_fifo_count(), 0xFF);
        assert!(!val.is_rx_buffer_write_enabled());
        assert_eq!(val.rx_buffer_count(), 0);
        assert_eq!(val.rx_fifo_count(), 0);

        val = FifoStatus(0x0000_F0FF);
        assert!(!val.is_tx_buffer_write_enabled());
        assert_eq!(val.tx_buffer_count(), 0);
        assert_eq!(val.tx_fifo_count(), 0);
        assert!(val.is_rx_buffer_write_enabled());
        assert_eq!(val.rx_buffer_count(), 0x7);
        assert_eq!(val.rx_fifo_count(), 0xFF);
    }

    #[test]
    fn struct_start_functions() {
        let val = Start(0x0).set_start_group(0x7);
        assert_eq!(val.0, 0x0000_0007);
    }

    test_should_panic!((
        test_start_set_start_group_panic,
        Start(0x0).set_start_group(0x8),
        "Start group out of range (expected 0..=7)"
    ),);

    #[test]
    fn struct_format_functions() {
        let mut val = Format(0x0);

        val = val.set_format_cmd(0xFF);
        assert_eq!(val.format_cmd(), 0xFF);
        assert_eq!(val.0, 0xFF00_0000);

        val = Format(0x0).set_format_cmd_ex(0xFF);
        assert_eq!(val.format_cmd_ex(), 0xFF);
        assert_eq!(val.0, 0xFF_0000);

        val = Format(0x0);
        for i in 0..7 {
            let (sel, reg_val) = match i {
                0 => (FormatSel::LutConfig, 0x0000_0000),
                1 => (FormatSel::Command1S, 0x0000_0001),
                2 => (FormatSel::Command3Addr, 0x0000_0002),
                3 => (FormatSel::Command4Addr, 0x0000_0003),
                4 => (FormatSel::Command8D, 0x0000_0004),
                5 => (FormatSel::CommandEx8D, 0x0000_0005),
                6 => (FormatSel::CommandExAddr8D, 0x0000_0006),
                _ => unreachable!(),
            };

            val = val.set_format_sel(sel);
            assert_eq!(val.format_sel(), sel);
            assert_eq!(val.0, reg_val);
        }
    }

    #[test]
    fn struct_burst_type_functions() {
        let mut val = BurstType(0x0);

        val = val.set_spi_burst_wrapped(0xFF);
        assert_eq!(val.spi_burst_wrapped(), 0xFF);
        assert_eq!(val.0, 0x0000_FF00);

        val = BurstType(0x0).set_spi_burst_linear(0xFF);
        assert_eq!(val.spi_burst_linear(), 0xFF);
        assert_eq!(val.0, 0x0000_00FF);
    }

    #[test]
    fn struct_rd_cmd_control_functions() {
        let mut val = RdCmdControl(0x0);

        val = val.enable_read_mode_byte();
        assert!(val.is_read_mode_byte_enabled());
        assert_eq!(val.0, 0x0002_0000);
        val = val.disable_read_mode_byte();
        assert!(!val.is_read_mode_byte_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_rdcmd_bypass();
        assert!(val.is_rdcmd_bypass_enabled());
        assert_eq!(val.0, 0x0001_0000);
        val = val.disable_rdcmd_bypass();
        assert!(!val.is_rdcmd_bypass_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_rdcmd_bypass_code(0xFF);
        assert_eq!(val.rdcmd_bypass_code(), 0xFF);
        assert_eq!(val.0, 0x0000_FF00);

        val = RdCmdControl(0x0).set_rdcmd_normal_code(0xFF);
        assert_eq!(val.rdcmd_normal_code(), 0xFF);
        assert_eq!(val.0, 0x0000_00FF);
    }

    #[test]
    fn struct_dma_mode_control_functions() {
        let mut val = DmaModeControl(0x0);

        val = val.set_dma_active_mode(DmaActiveMode::Controller);
        assert_eq!(val.dma_active_mode(), DmaActiveMode::Controller);
        assert_eq!(val.0, 0x0000_00C0);
        val = val.set_dma_active_mode(DmaActiveMode::DmaRequest);
        assert_eq!(val.dma_active_mode(), DmaActiveMode::DmaRequest);
        assert_eq!(val.0, 0x0000_0080);
        val = val.set_dma_active_mode(DmaActiveMode::High);
        assert_eq!(val.dma_active_mode(), DmaActiveMode::High);
        assert_eq!(val.0, 0x0000_0040);
        val = val.set_dma_active_mode(DmaActiveMode::Low);
        assert_eq!(val.dma_active_mode(), DmaActiveMode::Low);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_active_fall_behavior(ActiveFallBehavior::MustAfterAckHigh);
        assert_eq!(
            val.active_fall_behavior(),
            ActiveFallBehavior::MustAfterAckHigh
        );
        assert_eq!(val.0, 0x0000_0020);
        val = val.set_active_fall_behavior(ActiveFallBehavior::DoNotCareAck);
        assert_eq!(val.active_fall_behavior(), ActiveFallBehavior::DoNotCareAck);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_delay_clocks(0x1F);
        assert_eq!(val.delay_clocks(), 0x1F);
        assert_eq!(val.0, 0x0000_001F);
    }

    test_should_panic!((
        test_dma_mode_control_set_delay_clocks_panic,
        DmaModeControl(0x0).set_delay_clocks(0x20),
        "Delay clocks out of range (expected 0..=31)"
    ),);

    #[test]
    fn struct_lock_config_functions() {
        let val = LockConfig(0x0).set_lock_cfg(LockCfg::Locked);
        assert_eq!(val.lock_cfg(), LockCfg::Locked);
        assert_eq!(val.0, 0x0000_0001);

        let val = val.set_lock_cfg(LockCfg::Unlocked);
        assert_eq!(val.lock_cfg(), LockCfg::Unlocked);
        assert_eq!(val.0, 0x0000_0002);
    }

    #[test]
    fn struct_lut_up_functions() {
        let mut val = LutUp(0x0).set_lut_update(true);
        assert!(val.lut_update());
        assert_eq!(val.0, 0x0000_0001);
        val = val.set_lut_update(false);
        assert!(!val.lut_update());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_cs_sequence_functions() {
        for i in 0..8 {
            let sel = match i {
                0 => DataLine::D0,
                1 => DataLine::D1,
                2 => DataLine::D2,
                3 => DataLine::D3,
                4 => DataLine::D4,
                5 => DataLine::D5,
                6 => DataLine::D6,
                7 => DataLine::D7,
                _ => unreachable!(),
            };

            let reg_val = sel as u32;
            let mut val = CsSequence(0x0).set_d0_sel(sel);
            assert_eq!(val.d0_sel(), sel);
            assert_eq!(val.0, reg_val);
            val = CsSequence(0x0).set_d1_sel(sel);
            assert_eq!(val.d1_sel(), sel);
            assert_eq!(val.0, reg_val << 4);
            val = CsSequence(0x0).set_d2_sel(sel);
            assert_eq!(val.d2_sel(), sel);
            assert_eq!(val.0, reg_val << 8);
            val = CsSequence(0x0).set_d3_sel(sel);
            assert_eq!(val.d3_sel(), sel);
            assert_eq!(val.0, reg_val << 12);
            val = CsSequence(0x0).set_d4_sel(sel);
            assert_eq!(val.d4_sel(), sel);
            assert_eq!(val.0, reg_val << 16);
            val = CsSequence(0x0).set_d5_sel(sel);
            assert_eq!(val.d5_sel(), sel);
            assert_eq!(val.0, reg_val << 20);
            val = CsSequence(0x0).set_d6_sel(sel);
            assert_eq!(val.d6_sel(), sel);
            assert_eq!(val.0, reg_val << 24);
            val = CsSequence(0x0).set_d7_sel(sel);
            assert_eq!(val.d7_sel(), sel);
            assert_eq!(val.0, reg_val << 28);
        }
    }

    #[test]
    fn struct_io_control_functions() {
        let mut val = IoControl(0x0);

        val = val.disable_cs1_io();
        assert!(!val.is_cs1_io_enabled());
        assert_eq!(val.0, 0x0000_0002);
        val = val.enable_cs1_io();
        assert!(val.is_cs1_io_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.disable_cs0_io();
        assert!(!val.is_cs0_io_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.enable_cs0_io();
        assert!(val.is_cs0_io_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_cs_io_config1_functions() {
        for pull_val in 0..4 {
            if pull_val == 1 {
                continue;
            }

            let pull = match pull_val {
                0 => PinPull::Disabled,
                2 => PinPull::PullDown,
                3 => PinPull::PullUp,
                _ => unreachable!(),
            };

            let pull_reg_val = pull as u32;
            let mut val = CsIoConfig1(0x0).set_d7_pin_pull(pull);
            assert_eq!(val.d7_pin_pull(), pull);
            assert_eq!(val.0, pull_reg_val << 28);
            val = CsIoConfig1(0x0).set_d6_pin_pull(pull);
            assert_eq!(val.d6_pin_pull(), pull);
            assert_eq!(val.0, pull_reg_val << 20);
            val = CsIoConfig1(0x0).set_d5_pin_pull(pull);
            assert_eq!(val.d5_pin_pull(), pull);
            assert_eq!(val.0, pull_reg_val << 12);
            val = CsIoConfig1(0x0).set_d4_pin_pull(pull);
            assert_eq!(val.d4_pin_pull(), pull);
            assert_eq!(val.0, pull_reg_val << 4);
        }

        for drv_val in 0..8 {
            let drv = match drv_val {
                0 => PinDriveStrength::Level0,
                1 => PinDriveStrength::Level1,
                2 => PinDriveStrength::Level2,
                3 => PinDriveStrength::Level3,
                4 => PinDriveStrength::Level4,
                5 => PinDriveStrength::Level5,
                6 => PinDriveStrength::Level6,
                7 => PinDriveStrength::Level7,
                _ => unreachable!(),
            };

            let drv_reg_val = drv as u32;
            let mut val = CsIoConfig1(0x0).set_d7_pin_drv(drv);
            assert_eq!(val.d7_pin_drv(), drv);
            assert_eq!(val.0, drv_reg_val << 24);
            val = CsIoConfig1(0x0).set_d6_pin_drv(drv);
            assert_eq!(val.d6_pin_drv(), drv);
            assert_eq!(val.0, drv_reg_val << 16);
            val = CsIoConfig1(0x0).set_d5_pin_drv(drv);
            assert_eq!(val.d5_pin_drv(), drv);
            assert_eq!(val.0, drv_reg_val << 8);
            val = CsIoConfig1(0x0).set_d4_pin_drv(drv);
            assert_eq!(val.d4_pin_drv(), drv);
            assert_eq!(val.0, drv_reg_val);
        }
    }

    #[test]
    fn struct_cs_io_config2_functions() {
        for pull_val in 0..4 {
            if pull_val == 1 {
                continue;
            }

            let pull = match pull_val {
                0 => PinPull::Disabled,
                2 => PinPull::PullDown,
                3 => PinPull::PullUp,
                _ => unreachable!(),
            };

            let pull_reg_val = pull as u32;
            let mut val = CsIoConfig2(0x0).set_d3_pin_pull(pull);
            assert_eq!(val.d3_pin_pull(), pull);
            assert_eq!(val.0, pull_reg_val << 28);
            val = CsIoConfig2(0x0).set_d2_pin_pull(pull);
            assert_eq!(val.d2_pin_pull(), pull);
            assert_eq!(val.0, pull_reg_val << 20);
            val = CsIoConfig2(0x0).set_d1_pin_pull(pull);
            assert_eq!(val.d1_pin_pull(), pull);
            assert_eq!(val.0, pull_reg_val << 12);
            val = CsIoConfig2(0x0).set_d0_pin_pull(pull);
            assert_eq!(val.d0_pin_pull(), pull);
            assert_eq!(val.0, pull_reg_val << 4);
        }

        for drv_val in 0..8 {
            let drv = match drv_val {
                0 => PinDriveStrength::Level0,
                1 => PinDriveStrength::Level1,
                2 => PinDriveStrength::Level2,
                3 => PinDriveStrength::Level3,
                4 => PinDriveStrength::Level4,
                5 => PinDriveStrength::Level5,
                6 => PinDriveStrength::Level6,
                7 => PinDriveStrength::Level7,
                _ => unreachable!(),
            };

            let drv_reg_val = drv as u32;
            let mut val = CsIoConfig2(0x0).set_d3_pin_drv(drv);
            assert_eq!(val.d3_pin_drv(), drv);
            assert_eq!(val.0, drv_reg_val << 24);
            val = CsIoConfig2(0x0).set_d2_pin_drv(drv);
            assert_eq!(val.d2_pin_drv(), drv);
            assert_eq!(val.0, drv_reg_val << 16);
            val = CsIoConfig2(0x0).set_d1_pin_drv(drv);
            assert_eq!(val.d1_pin_drv(), drv);
            assert_eq!(val.0, drv_reg_val << 8);
            val = CsIoConfig2(0x0).set_d0_pin_drv(drv);
            assert_eq!(val.d0_pin_drv(), drv);
            assert_eq!(val.0, drv_reg_val);
        }
    }

    #[test]
    fn struct_cs_io_config3_functions() {
        for pull_val in 0..4 {
            if pull_val == 1 {
                continue;
            }

            let pull = match pull_val {
                0 => PinPull::Disabled,
                2 => PinPull::PullDown,
                3 => PinPull::PullUp,
                _ => unreachable!(),
            };

            let pull_reg_val = pull as u32;
            let mut val = CsIoConfig3(0x0).set_cs_pin_pull(pull);
            assert_eq!(val.cs_pin_pull(), pull);
            assert_eq!(val.0, pull_reg_val << 28);
            val = CsIoConfig3(0x0).set_dqs_pin_pull(pull);
            assert_eq!(val.dqs_pin_pull(), pull);
            assert_eq!(val.0, pull_reg_val << 20);
            val = CsIoConfig3(0x0).set_ck_pin_pull(pull);
            assert_eq!(val.ck_pin_pull(), pull);
            assert_eq!(val.0, pull_reg_val << 12);
            val = CsIoConfig3(0x0).set_ckn_pin_pull(pull);
            assert_eq!(val.ckn_pin_pull(), pull);
            assert_eq!(val.0, pull_reg_val << 4);
        }

        for drv_val in 0..8 {
            let drv = match drv_val {
                0 => PinDriveStrength::Level0,
                1 => PinDriveStrength::Level1,
                2 => PinDriveStrength::Level2,
                3 => PinDriveStrength::Level3,
                4 => PinDriveStrength::Level4,
                5 => PinDriveStrength::Level5,
                6 => PinDriveStrength::Level6,
                7 => PinDriveStrength::Level7,
                _ => unreachable!(),
            };

            let drv_reg_val = drv as u32;
            let mut val = CsIoConfig3(0x0).set_cs_pin_drv(drv);
            assert_eq!(val.cs_pin_drv(), drv);
            assert_eq!(val.0, drv_reg_val << 24);
            val = CsIoConfig3(0x0).set_dqs_pin_drv(drv);
            assert_eq!(val.dqs_pin_drv(), drv);
            assert_eq!(val.0, drv_reg_val << 16);
            val = CsIoConfig3(0x0).set_ck_pin_drv(drv);
            assert_eq!(val.ck_pin_drv(), drv);
            assert_eq!(val.0, drv_reg_val << 8);
            val = CsIoConfig3(0x0).set_ckn_pin_drv(drv);
            assert_eq!(val.ckn_pin_drv(), drv);
            assert_eq!(val.0, drv_reg_val);
        }
    }

    #[test]
    fn struct_cs_io_config4_functions() {
        for pull_val in 0..4 {
            if pull_val == 1 {
                continue;
            }

            let pull = match pull_val {
                0 => PinPull::Disabled,
                2 => PinPull::PullDown,
                3 => PinPull::PullUp,
                _ => unreachable!(),
            };

            let pull_reg_val = pull as u32;
            let val = CsIoConfig4(0x0).set_dm_pin_pull(pull);
            assert_eq!(val.dm_pin_pull(), pull);
            assert_eq!(val.0, pull_reg_val << 4);
        }

        for drv_val in 0..8 {
            let drv = match drv_val {
                0 => PinDriveStrength::Level0,
                1 => PinDriveStrength::Level1,
                2 => PinDriveStrength::Level2,
                3 => PinDriveStrength::Level3,
                4 => PinDriveStrength::Level4,
                5 => PinDriveStrength::Level5,
                6 => PinDriveStrength::Level6,
                7 => PinDriveStrength::Level7,
                _ => unreachable!(),
            };

            let drv_reg_val = drv as u32;
            let val = CsIoConfig4(0x0).set_dm_pin_drv(drv);
            assert_eq!(val.dm_pin_drv(), drv);
            assert_eq!(val.0, drv_reg_val);
        }
    }

    #[test]
    fn struct_training_config_functions() {
        let mut val = TrainingConfig(0x0);

        val = val.set_training_phase_cal(TrainingPhaseCal::RoundUp);
        assert_eq!(val.training_phase_cal(), TrainingPhaseCal::RoundUp);
        assert_eq!(val.0, 0x0100_0000);
        val = val.set_training_phase_cal(TrainingPhaseCal::RoundDown);
        assert_eq!(val.training_phase_cal(), TrainingPhaseCal::RoundDown);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_training_pattern_sel(TrainingPatternSel::Custom);
        assert_eq!(val.training_pattern_sel(), TrainingPatternSel::Custom);
        assert_eq!(val.0, 0x0003_0000);
        val = val.set_training_pattern_sel(TrainingPatternSel::Inversion);
        assert_eq!(val.training_pattern_sel(), TrainingPatternSel::Inversion);
        assert_eq!(val.0, 0x0002_0000);
        val = val.set_training_pattern_sel(TrainingPatternSel::Random);
        assert_eq!(val.training_pattern_sel(), TrainingPatternSel::Random);
        assert_eq!(val.0, 0x0001_0000);
        val = val.set_training_pattern_sel(TrainingPatternSel::StuckAddress);
        assert_eq!(val.training_pattern_sel(), TrainingPatternSel::StuckAddress);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_data_len(0xFFFF);
        assert_eq!(val.data_len(), 0xFFFF);
        assert_eq!(val.0, 0x0000_FFFF);
    }

    #[test]
    fn struct_lut_functions() {
        let mut val = Lut(0x0);

        val = val.set_instr1(0x3F);
        assert_eq!(val.instr1(), 0x3F);
        assert_eq!(val.0, 0xFC00_0000);

        val = Lut(0x0).set_io_cfg1(IoCfg::EightIo);
        assert_eq!(val.io_cfg1(), IoCfg::EightIo);
        assert_eq!(val.0, 0x0300_0000);
        val = val.set_io_cfg1(IoCfg::FourIo);
        assert_eq!(val.io_cfg1(), IoCfg::FourIo);
        assert_eq!(val.0, 0x0200_0000);
        val = val.set_io_cfg1(IoCfg::TwoIo);
        assert_eq!(val.io_cfg1(), IoCfg::TwoIo);
        assert_eq!(val.0, 0x0100_0000);
        val = val.set_io_cfg1(IoCfg::OneIo);
        assert_eq!(val.io_cfg1(), IoCfg::OneIo);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_operand1(0xFF);
        assert_eq!(val.operand1(), 0xFF);
        assert_eq!(val.0, 0x00FF_0000);

        val = Lut(0x0).set_instr0(0x3F);
        assert_eq!(val.instr0(), 0x3F);
        assert_eq!(val.0, 0x0000_FC00);

        val = Lut(0x0).set_io_cfg0(IoCfg::EightIo);
        assert_eq!(val.io_cfg0(), IoCfg::EightIo);
        assert_eq!(val.0, 0x0000_0300);
        val = val.set_io_cfg0(IoCfg::FourIo);
        assert_eq!(val.io_cfg0(), IoCfg::FourIo);
        assert_eq!(val.0, 0x0000_0200);
        val = val.set_io_cfg0(IoCfg::TwoIo);
        assert_eq!(val.io_cfg0(), IoCfg::TwoIo);
        assert_eq!(val.0, 0x0000_0100);
        val = val.set_io_cfg0(IoCfg::OneIo);
        assert_eq!(val.io_cfg0(), IoCfg::OneIo);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_operand0(0xFF);
        assert_eq!(val.operand0(), 0xFF);
        assert_eq!(val.0, 0x0000_00FF);
    }

    test_should_panic!(
        (
            test_lut_set_instr1_panic,
            Lut(0x0).set_instr1(0x40),
            "Instruction1 out of range (expected 0..=63)"
        ),
        (
            test_lut_set_instr0_panic,
            Lut(0x0).set_instr0(0x40),
            "Instruction0 out of range (expected 0..=63)"
        ),
    );

    #[test]
    fn struct_debug_sel_functions() {
        let val = DebugSel(0x0).set_debug_sel(0xF);
        assert_eq!(val.debug_sel(), 0xF);
        assert_eq!(val.0, 0x0000_000F);
    }

    test_should_panic!((
        test_debug_sel_set_debug_sel_panic,
        DebugSel(0x0).set_debug_sel(0x10),
        "Debug selection out of range (expected 0..=15)"
    ),);
}
