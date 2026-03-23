//! PWM register blocks and registers.

use volatile_register::{RO, RW};

/// Pulse Width Modulation Register Block.
#[repr(C)]
pub struct RegisterBlock {
    /// Control register (`PWM_CTL`).
    #[doc(alias = "PWM_CTL")]
    pub ctrl: RW<Control>,
    /// Module control register (`PWM_MCTL`)
    #[doc(alias = "PWM_MCTL")]
    pub m_ctrl: RW<ModuleControl>,
    /// Clock control register (`PWM_CKCTL`).
    #[doc(alias = "PWM_CKCTL")]
    pub ck_ctrl: RW<ClkControl>,
    /// Interrupt control register (`PWM_INTCTL`)
    #[doc(alias = "PWM_INTCTL")]
    pub int_ctrl: RW<IntControl>,
    /// Interrupt status register (`PWM_INTSTS`)
    #[doc(alias = "PWM_INTSTS")]
    pub int_stat: RW<IntStatus>,
    _reserved0: [u8; 0x2EC],
    /// Channels.
    pub channels: [Channel; 4],
    _reserved1: [u8; 0x8FC],
    /// Version registers (`PWM_VERSION`).
    #[doc(alias = "PWM_VERSION")]
    pub version: RO<u32>,
}

/// PWM channel Register Block.
#[repr(C)]
pub struct Channel {
    /// Time-base control register (`PWM_TBCTL`).
    #[doc(alias = "PWM_TBCTL")]
    pub tb_ctrl: RW<TBControl>,
    /// Time-base status register (`PWM_TBSTS`).
    #[doc(alias = "PWM_TBSTS")]
    pub tb_stat: RW<TBStatus>,
    _reserved0: [u8; 0x8],
    /// Time-base counter register (`PWM_TBCTR`).
    #[doc(alias = "PWM_TBCTR")]
    pub tb_cnt: RW<TBCounter>,
    /// Time-base period register (`PWM_TBPRD`).
    #[doc(alias = "PWM_TBPRD")]
    pub tb_prd: RW<TBPeriod>,
    /// Comparator control register (`PWM_CMPCTL`).
    #[doc(alias = "PWM_CMPCTL")]
    pub cmp_ctrl: RW<CmpControl>,
    _reserved1: [u8; 0x4],
    /// Comparator A register (`PWM_CMPA`).
    #[doc(alias = "PWM_CMPA")]
    pub cmp_a: RW<Comparator>,
    /// Comparator B register (`PWM_CMPB`).
    #[doc(alias = "PWM_CMPB")]
    pub cmp_b: RW<Comparator>,
    /// Action qualifier control 0 register (`PWM_AQCTLA`).
    #[doc(alias = "PWM_AQCTLA")]
    pub aq_ctrl_0: RW<AqControl>,
    /// Action qualifier control 1 register (`PWM_AQCTLB`).
    #[doc(alias = "PWM_AQCTLB")]
    pub aq_ctrl_1: RW<AqControl>,
    _reserved2: [u8; 0x14],
    /// Event trigger selection register (`PWM_ETSEL`)
    #[doc(alias = "PWM_ETSEL")]
    pub et_sel: RW<EvtTrigSel>,
    /// Debug register (`PWM_CNT_DIS`)
    #[doc(alias = "PWM_CNT_DIS")]
    pub cnt_dis: RO<CntDis>,
    _reserved3: [u8; 0xB4],
}

/// Control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Control(u32);

impl Control {
    const EN: u32 = 0x1;

    /// Enable PWM (`EN`).
    #[doc(alias = "EN")]
    #[inline]
    pub const fn enable(self) -> Self {
        Self(self.0 | Self::EN)
    }
    /// Disable PWM.
    #[inline]
    pub const fn disable(self) -> Self {
        Self(self.0 & !Self::EN)
    }
    /// Check if PWM is enabled.
    #[inline]
    pub const fn is_enabled(self) -> bool {
        self.0 & Self::EN != 0
    }
}

/// Module control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ModuleControl(u32);

impl ModuleControl {
    /// Enable channel.
    #[inline]
    pub const fn enable_ch(self, ch: u8) -> Self {
        assert!(ch < 4, "Channel out of range (expected 0..=3)");
        Self(self.0 | (0x1 << ch))
    }
    /// Disable channel.
    #[inline]
    pub const fn disable_ch(self, ch: u8) -> Self {
        assert!(ch < 4, "Channel out of range (expected 0..=3)");
        Self(self.0 & !(0x1 << ch))
    }
    /// Check if channel is enabled.
    #[inline]
    pub const fn is_ch_enabled(self, ch: u8) -> bool {
        assert!(ch < 4, "Channel out of range (expected 0..=3)");
        self.0 & (0x1 << ch) != 0
    }
}

/// Clock control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ClkControl(u32);

impl ClkControl {
    /// Enable channel clock.
    #[inline]
    pub const fn enable_ch_clk(self, ch: u8) -> Self {
        assert!(ch < 4, "Channel out of range (expected 0..=3)");
        Self(self.0 | (0x1 << ch))
    }
    /// Disable channel clock.
    #[inline]
    pub const fn disable_ch_clk(self, ch: u8) -> Self {
        assert!(ch < 4, "Channel out of range (expected 0..=3)");
        Self(self.0 & !(0x1 << ch))
    }
    /// Check if channel clock is enabled.
    #[inline]
    pub const fn is_ch_clk_enabled(self, ch: u8) -> bool {
        assert!(ch < 4, "Channel out of range (expected 0..=3)");
        self.0 & (0x1 << ch) != 0
    }
}

/// Interrupt control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IntControl(u32);

impl IntControl {
    /// Enable channel interrupt.
    #[inline]
    pub const fn enable_ch_int(self, ch: u8) -> Self {
        assert!(ch < 4, "Channel out of range (expected 0..=3)");
        Self(self.0 | (0x1 << ch))
    }
    /// Disable channel interrupt.
    #[inline]
    pub const fn disable_ch_int(self, ch: u8) -> Self {
        assert!(ch < 4, "Channel out of range (expected 0..=3)");
        Self(self.0 & !(0x1 << ch))
    }
    /// Check if channel interrupt is enabled.
    #[inline]
    pub const fn is_ch_int_enabled(self, ch: u8) -> bool {
        assert!(ch < 4, "Channel out of range (expected 0..=3)");
        self.0 & (0x1 << ch) != 0
    }
}

/// Interrupt status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IntStatus(u32);

impl IntStatus {
    /// Check if channel interrupt is pending.
    #[inline]
    pub const fn is_ch_int_pending(self, ch: u8) -> bool {
        assert!(ch < 4, "Channel out of range (expected 0..=3)");
        self.0 & (0x1 << ch) != 0
    }
    /// Clear channel interrupt pending.
    #[inline]
    pub const fn clear_ch_int_pending(self, ch: u8) -> Self {
        assert!(ch < 4, "Channel out of range (expected 0..=3)");
        Self(self.0 | (0x1 << ch))
    }
}

/// Time-base period register operation mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PrdRegOpMode {
    /// Shadow mode.
    ///
    /// When the time-base counter (`TBCTR`) equals 0, the value of the time-base period register (`TBPRD`)
    /// is loaded from its shadow register. Reads and writes to the time-base period register (`TBPRD`)
    /// actually operate on its shadow register.
    Shadow,
    /// Immediate mode.
    ///
    /// The value of the time-base period register (`TBPRD`) does not need to be loaded through its shadow register.
    /// Reads and writes to the time-base period register (`TBPRD`) operate directly on `TBPRD` itself.
    Immediate,
}

/// Time-base counter mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CntMode {
    CountUp,
    CountDown,
    CountUpAndDown,
}

/// Time-base control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TBControl(u32);

impl TBControl {
    const CLK_DIV: u32 = 0xFFF << 16;
    const PRDLD: u32 = 0x1 << 3;
    const CTRMODE: u32 = 0x3;

    /// Set time-base clock divider (`CLK_DIV`).
    ///
    /// TBCLK = SYSCLKOUT / (CLKDIV + 1).
    #[doc(alias = "CLK_DIV")]
    #[inline]
    pub const fn set_clk_div(self, div: u16) -> Self {
        assert!(
            div < 0x1000,
            "Clock divider out of range (expected 0..=0xFFF)"
        );
        Self((self.0 & !Self::CLK_DIV) | (Self::CLK_DIV & ((div as u32) << 16)))
    }
    /// Get time-base clock divider.
    #[inline]
    pub const fn clk_div(self) -> u16 {
        ((self.0 & Self::CLK_DIV) >> 16) as u16
    }
    /// Set time-base period register operation mode (`PRDLD`).
    #[doc(alias = "PRDLD")]
    #[inline]
    pub const fn set_prd_reg_op_mode(self, mode: PrdRegOpMode) -> Self {
        Self((self.0 & !Self::PRDLD) | (Self::PRDLD & ((mode as u32) << 3)))
    }
    /// Get time-base period register operation mode.
    #[inline]
    pub const fn prd_reg_op_mode(self) -> PrdRegOpMode {
        match (self.0 & Self::PRDLD) >> 3 {
            0 => PrdRegOpMode::Shadow,
            1 => PrdRegOpMode::Immediate,
            _ => unreachable!(),
        }
    }
    /// Set time-base counter mode (`CTRMODE`).
    #[doc(alias = "CTRMODE")]
    #[inline]
    pub const fn set_cnt_mode(self, mode: CntMode) -> Self {
        Self((self.0 & !Self::CTRMODE) | (Self::CTRMODE & (mode as u32)))
    }
    /// Get time-base counter mode.
    #[inline]
    pub const fn cnt_mode(self) -> CntMode {
        match self.0 & Self::CTRMODE {
            0 => CntMode::CountUp,
            1 => CntMode::CountDown,
            2 => CntMode::CountUpAndDown,
            _ => unreachable!(),
        }
    }
}

/// Time-base status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TBStatus(u32);

impl TBStatus {
    const CTRMAX: u32 = 0x1 << 2;
    const CTRDIR: u32 = 0x1;

    /// Check if time-base counter reach maximum (`CTRMAX`).
    #[doc(alias = "CTRMAX")]
    #[inline]
    pub const fn is_ctr_max(self) -> bool {
        self.0 & Self::CTRMAX != 0
    }
    /// Clear time-base counter maximum.
    #[inline]
    pub const fn clear_ctr_max(self) -> Self {
        Self(self.0 | Self::CTRMAX)
    }
    /// Get counter direction (`CTRDIR`).
    #[doc(alias = "CTRDIR")]
    #[inline]
    pub const fn cnt_dir(self) -> CntMode {
        match self.0 & Self::CTRDIR {
            0 => CntMode::CountDown,
            1 => CntMode::CountUp,
            _ => unreachable!(),
        }
    }
}

/// Time-base counter register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TBCounter(u32);

impl TBCounter {
    const TBCTR: u32 = 0xFFFF;

    /// Set time-base counter (`TBCTR`).
    #[doc(alias = "TBCTR")]
    #[inline]
    pub const fn set_tb_cnt(self, cnt: u16) -> Self {
        Self((self.0 & !Self::TBCTR) | (Self::TBCTR & (cnt as u32)))
    }
    /// Get time-base counter.
    #[inline]
    pub const fn tb_cnt(self) -> u16 {
        (self.0 & Self::TBCTR) as u16
    }
}

/// Time-base period register
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TBPeriod(u32);

impl TBPeriod {
    const TBPRD: u32 = 0xFFFF;

    /// Set time-base period (`TBPRD`).
    #[doc(alias = "TBPRD")]
    #[inline]
    pub const fn set_tb_prd(self, prd: u16) -> Self {
        Self((self.0 & !Self::TBPRD) | (Self::TBPRD & (prd as u32)))
    }
    /// Get time-base period.
    #[inline]
    pub const fn tb_prd(self) -> u16 {
        (self.0 & Self::TBPRD) as u16
    }
}

/// Shadow register load mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ShdwLdMode {
    /// Load when `CTR` = 0.
    Mode0,
    /// Load when `CTR` = `PRD`.
    Mode1,
    /// Load when `CTR` = 0 or `CTR` = `PRD`.
    Mode2,
    /// Freeze.
    Freeze,
}

/// Comparator control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CmpControl(u32);

impl CmpControl {
    const SHDWBMODE: u32 = 0x1 << 6;
    const SHDWAMODE: u32 = 0x1 << 4;
    const LOADBMODE: u32 = 0x3 << 2;
    const LOADAMODE: u32 = 0x3;

    /// Set comparator b op mode (`SHDWBMODE`).
    #[doc(alias = "SHDWBMODE")]
    #[inline]
    pub const fn set_cmp_b_op_mode(self, mode: PrdRegOpMode) -> Self {
        Self((self.0 & !Self::SHDWBMODE) | (Self::SHDWBMODE & ((mode as u32) << 6)))
    }
    /// Get comparator b op mode.
    #[inline]
    pub const fn cmp_b_op_mode(self) -> PrdRegOpMode {
        match (self.0 & Self::SHDWBMODE) >> 6 {
            0 => PrdRegOpMode::Shadow,
            1 => PrdRegOpMode::Immediate,
            _ => unreachable!(),
        }
    }
    /// Set comparator a op mode (`SHDWAMODE`).
    #[doc(alias = "SHDWAMODE")]
    #[inline]
    pub const fn set_cmp_a_op_mode(self, mode: PrdRegOpMode) -> Self {
        Self((self.0 & !Self::SHDWAMODE) | (Self::SHDWAMODE & ((mode as u32) << 4)))
    }
    /// Get comparator a op mode (`SHDWAMODE`).
    #[inline]
    pub const fn cmp_a_op_mode(self) -> PrdRegOpMode {
        match (self.0 & Self::SHDWAMODE) >> 4 {
            0 => PrdRegOpMode::Shadow,
            1 => PrdRegOpMode::Immediate,
            _ => unreachable!(),
        }
    }
    /// Set comparator b shadow load mode (`LOADBMODE`).
    #[doc(alias = "LOADBMODE")]
    #[inline]
    pub const fn set_cmp_b_shdw_ld_mode(self, mode: ShdwLdMode) -> Self {
        Self((self.0 & !Self::LOADBMODE) | (Self::LOADBMODE & ((mode as u32) << 2)))
    }
    /// Get comparator b shadow load mode.
    #[inline]
    pub const fn cmp_b_shdw_ld_mode(self) -> ShdwLdMode {
        match (self.0 & Self::LOADBMODE) >> 2 {
            0 => ShdwLdMode::Mode0,
            1 => ShdwLdMode::Mode1,
            2 => ShdwLdMode::Mode2,
            3 => ShdwLdMode::Freeze,
            _ => unreachable!(),
        }
    }
    /// Set comparator a shadow load mode (`LOADAMODE`).
    #[doc(alias = "LOADAMODE")]
    #[inline]
    pub const fn set_cmp_a_shdw_ld_mode(self, mode: ShdwLdMode) -> Self {
        Self((self.0 & !Self::LOADAMODE) | (Self::LOADAMODE & (mode as u32)))
    }
    /// Get comparator a shadow load mode.
    #[inline]
    pub const fn cmp_a_shdw_ld_mode(self) -> ShdwLdMode {
        match self.0 & Self::LOADAMODE {
            0 => ShdwLdMode::Mode0,
            1 => ShdwLdMode::Mode1,
            2 => ShdwLdMode::Mode2,
            3 => ShdwLdMode::Freeze,
            _ => unreachable!(),
        }
    }
}

/// Comparator register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Comparator(u32);

impl Comparator {
    const CMP: u32 = 0xFFFF;

    /// Set comparator value.
    #[doc(alias = "CMP")]
    #[inline]
    pub const fn set_cmp(self, cmp: u16) -> Self {
        Self((self.0 & !Self::CMP) | (Self::CMP & (cmp as u32)))
    }
    /// Get comparator value.
    #[inline]
    pub const fn cmp(self) -> u16 {
        (self.0 & Self::CMP) as u16
    }
}

/// PWM initial level.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum InitLevel {
    Low,
    High,
}

/// PWM action mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ActionMode {
    /// No operation.
    NoOp,
    /// Set the line to low level.
    SetLow,
    /// Set the line to high level.
    SetHigh,
    /// Toggle the line level.
    Toggle,
}

/// Action qualifier control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AqControl(u32);

impl AqControl {
    const PWM_INIT: u32 = 0x1 << 16;
    const CBD: u32 = 0x3 << 10;
    const CBU: u32 = 0x3 << 8;
    const CAD: u32 = 0x3 << 6;
    const CAU: u32 = 0x3 << 4;
    const PRD: u32 = 0x3 << 2;
    const ZRO: u32 = 0x3;

    /// Set PWM initial level (`PWM_INIT`).
    #[doc(alias = "PWM_INIT")]
    #[inline]
    pub const fn set_init_level(self, level: InitLevel) -> Self {
        Self((self.0 & !Self::PWM_INIT) | (Self::PWM_INIT & ((level as u32) << 16)))
    }
    /// Get PWM initial level.
    #[inline]
    pub const fn init_level(self) -> InitLevel {
        match (self.0 & Self::PWM_INIT) >> 16 {
            0 => InitLevel::Low,
            1 => InitLevel::High,
            _ => unreachable!(),
        }
    }
    /// Set time-base counter counts down and `TBCTR` = `CMPB` action mode (`CBD`).
    ///
    /// - Notice: In the up/down counting mode, when the counter equals zero, the counting direction is defined as 1 (upward direction).
    #[doc(alias = "CBD")]
    #[inline]
    pub const fn set_cbd_mode(self, mode: ActionMode) -> Self {
        Self((self.0 & !Self::CBD) | (Self::CBD & ((mode as u32) << 10)))
    }
    /// Get time-base counter counts down and `TBCTR` = `CMPB` action mode.
    #[inline]
    pub const fn cbd_mode(self) -> ActionMode {
        match (self.0 & Self::CBD) >> 10 {
            0 => ActionMode::NoOp,
            1 => ActionMode::SetLow,
            2 => ActionMode::SetHigh,
            3 => ActionMode::Toggle,
            _ => unreachable!(),
        }
    }
    /// Set time-base counter counts up and `TBCTR` = `CMPB` action mode (`CBU`).
    ///
    /// - Notice: In the up/down counting mode, when the counter equals zero, the counting direction is defined as 1 (upward direction).
    #[doc(alias = "CBU")]
    #[inline]
    pub const fn set_cbu_mode(self, mode: ActionMode) -> Self {
        Self((self.0 & !Self::CBU) | (Self::CBU & ((mode as u32) << 8)))
    }
    /// Get time-base counter counts up and `TBCTR` = `CMPB` action mode.
    #[inline]
    pub const fn cbu_mode(self) -> ActionMode {
        match (self.0 & Self::CBU) >> 8 {
            0 => ActionMode::NoOp,
            1 => ActionMode::SetLow,
            2 => ActionMode::SetHigh,
            3 => ActionMode::Toggle,
            _ => unreachable!(),
        }
    }
    /// Set time-base counter counts down and `TBCTR` = `CMPA` action mode (`CAD`).
    ///
    /// - Notice: In the up/down counting mode, when the counter equals zero, the counting direction is defined as 1 (upward direction).
    #[doc(alias = "CAD")]
    #[inline]
    pub const fn set_cad_mode(self, mode: ActionMode) -> Self {
        Self((self.0 & !Self::CAD) | (Self::CAD & ((mode as u32) << 6)))
    }
    /// Get time-base counter counts down and `TBCTR` = `CMPA` action mode.
    #[inline]
    pub const fn cad_mode(self) -> ActionMode {
        match (self.0 & Self::CAD) >> 6 {
            0 => ActionMode::NoOp,
            1 => ActionMode::SetLow,
            2 => ActionMode::SetHigh,
            3 => ActionMode::Toggle,
            _ => unreachable!(),
        }
    }
    /// Set time-base counter counts up and `TBCTR` = `CMPA` action mode (`CAU`).
    ///
    /// - Notice: In the up/down counting mode, when the counter equals zero, the counting direction is defined as 1 (upward direction).
    #[doc(alias = "CAU")]
    #[inline]
    pub const fn set_cau_mode(self, mode: ActionMode) -> Self {
        Self((self.0 & !Self::CAU) | (Self::CAU & ((mode as u32) << 4)))
    }
    /// Get time-base counter counts up and `TBCTR` = `CMPA` action mode.
    #[inline]
    pub const fn cau_mode(self) -> ActionMode {
        match (self.0 & Self::CAU) >> 4 {
            0 => ActionMode::NoOp,
            1 => ActionMode::SetLow,
            2 => ActionMode::SetHigh,
            3 => ActionMode::Toggle,
            _ => unreachable!(),
        }
    }
    /// Set `TBCTR` = `PRD` action mode (`PRD`).
    ///
    /// - Notice: In the up/down counting mode, when the counter equals zero, the counting direction is defined as 1 (upward direction).
    #[doc(alias = "PRD")]
    #[inline]
    pub const fn set_prd_mode(self, mode: ActionMode) -> Self {
        Self((self.0 & !Self::PRD) | (Self::PRD & ((mode as u32) << 2)))
    }
    /// Get `TBCTR` = `PRD` action mode.
    #[inline]
    pub const fn prd_mode(self) -> ActionMode {
        match (self.0 & Self::PRD) >> 2 {
            0 => ActionMode::NoOp,
            1 => ActionMode::SetLow,
            2 => ActionMode::SetHigh,
            3 => ActionMode::Toggle,
            _ => unreachable!(),
        }
    }
    /// Set `TBCTR` = 0 action mode (`ZRO`).
    ///
    /// - Notice: In the up/down counting mode, when the counter equals zero, the counting direction is defined as 1 (upward direction).
    #[doc(alias = "ZRO")]
    #[inline]
    pub const fn set_zro_mode(self, mode: ActionMode) -> Self {
        Self((self.0 & !Self::ZRO) | (Self::ZRO & (mode as u32)))
    }
    /// Get `TBCTR` = 0 action mode.
    #[inline]
    pub const fn zro_mode(self) -> ActionMode {
        match self.0 & Self::ZRO {
            0 => ActionMode::NoOp,
            1 => ActionMode::SetLow,
            2 => ActionMode::SetHigh,
            3 => ActionMode::Toggle,
            _ => unreachable!(),
        }
    }
}

/// PWM interrupt event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntEvent {
    /// `TBCTR` = 0.
    Event0 = 1,
    /// `TBCTR` = `PRD`.
    Event1 = 2,
    /// `TBCTR` = `CMPA` and counter counts up.
    Event2 = 4,
    /// `TBCTR` = `CMPA` and counter counts down.
    Event3 = 5,
    /// `TBCTR` = `CMPB` and counter counts up.
    Event4 = 6,
    /// `TBCTR` = `CMPB` and counter counts down.
    Event5 = 7,
}

/// Event trigger selection register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct EvtTrigSel(u32);

impl EvtTrigSel {
    const INTEN: u32 = 0x1 << 3;
    const INTSEL: u32 = 0x7;

    /// Enable PWM interrupt (`INTEN`).
    #[doc(alias = "INTEN")]
    #[inline]
    pub const fn enable_int(self) -> Self {
        Self(self.0 | Self::INTEN)
    }
    /// Disable PWM interrupt.
    #[inline]
    pub const fn disable_int(self) -> Self {
        Self(self.0 & !Self::INTEN)
    }
    /// Check if PWM interrupt.
    #[inline]
    pub const fn is_int_enabled(self) -> bool {
        self.0 & Self::INTEN != 0
    }
    /// Set interrupt event (`INTSEL`).
    #[doc(alias = "INTSEL")]
    #[inline]
    pub const fn set_int_event(self, event: IntEvent) -> Self {
        Self((self.0 & !Self::INTSEL) | (Self::INTSEL & (event as u32)))
    }
    /// Get interrupt event.
    #[inline]
    pub const fn int_event(self) -> IntEvent {
        match self.0 & Self::INTSEL {
            1 => IntEvent::Event0,
            2 => IntEvent::Event1,
            4 => IntEvent::Event2,
            5 => IntEvent::Event3,
            6 => IntEvent::Event4,
            7 => IntEvent::Event5,
            _ => unreachable!(),
        }
    }
}

/// Debug register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CntDis(u32);

impl CntDis {
    const PWM_CNT_CK_DIS: u32 = 0x1;

    /// Get PWM counter clock disable bit (`PWM_CNT_CK_DIS`).
    ///
    /// - 0: When no action is configured for a triggered event, the time-base counter is disabled, and no compare events occur.
    ///   CMPA and CMPB can be configured with the same value or not configured at all.
    ///   The default is 0, and the compare actions follow the higher priority configured action.
    /// - 1: When no action is configured, the time-base counter is enabled, and compare events occur.
    ///   Due to priority reasons, CMPA and CMPB cannot be configured with equal values, nor can CMPA or CMPB be configured as 0.
    #[doc(alias = "PWM_CNT_CK_DIS")]
    #[inline]
    pub const fn cnt_dis(self) -> bool {
        self.0 & Self::PWM_CNT_CK_DIS != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_should_panic;
    use core::mem::{offset_of, size_of};

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, ctrl), 0x0);
        assert_eq!(offset_of!(RegisterBlock, m_ctrl), 0x4);
        assert_eq!(offset_of!(RegisterBlock, ck_ctrl), 0x8);
        assert_eq!(offset_of!(RegisterBlock, int_ctrl), 0xC);
        assert_eq!(offset_of!(RegisterBlock, int_stat), 0x10);
        assert_eq!(offset_of!(RegisterBlock, channels), 0x300);
        assert_eq!(offset_of!(RegisterBlock, version), 0xFFC);
    }

    #[test]
    fn struct_channel_offset() {
        assert_eq!(offset_of!(Channel, tb_ctrl), 0x0);
        assert_eq!(offset_of!(Channel, tb_stat), 0x4);
        assert_eq!(offset_of!(Channel, tb_cnt), 0x10);
        assert_eq!(offset_of!(Channel, tb_prd), 0x14);
        assert_eq!(offset_of!(Channel, cmp_ctrl), 0x18);
        assert_eq!(offset_of!(Channel, cmp_a), 0x20);
        assert_eq!(offset_of!(Channel, cmp_b), 0x24);
        assert_eq!(offset_of!(Channel, aq_ctrl_0), 0x28);
        assert_eq!(offset_of!(Channel, aq_ctrl_1), 0x2C);
        assert_eq!(offset_of!(Channel, et_sel), 0x44);
        assert_eq!(offset_of!(Channel, cnt_dis), 0x48);
        assert_eq!(size_of::<Channel>(), 0x100);
    }

    #[test]
    fn struct_control_functions() {
        let mut val = Control(0x0).enable();
        assert!(val.is_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable();
        assert!(!val.is_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_module_control_functions() {
        let val = ModuleControl(0x0).disable_ch(0).enable_ch(3);
        assert!(val.is_ch_enabled(3));
        assert!(!val.is_ch_enabled(0));
        assert_eq!(val.0, 0x0000_0008);
    }

    test_should_panic!(
        (
            test_module_control_enable_ch_panic,
            ModuleControl(0x0).enable_ch(0x4),
            "Channel out of range (expected 0..=3)"
        ),
        (
            test_module_control_disable_ch_panic,
            ModuleControl(0x0).disable_ch(0x4),
            "Channel out of range (expected 0..=3)"
        ),
        (
            test_module_control_is_ch_enabled_panic,
            ModuleControl(0x0).is_ch_enabled(0x4),
            "Channel out of range (expected 0..=3)"
        ),
    );

    #[test]
    fn struct_clk_control_functions() {
        let val = ClkControl(0x0).disable_ch_clk(0).enable_ch_clk(3);
        assert!(val.is_ch_clk_enabled(3));
        assert!(!val.is_ch_clk_enabled(0));
        assert_eq!(val.0, 0x0000_0008);
    }

    test_should_panic!(
        (
            test_clk_control_enable_ch_clk_panic,
            ClkControl(0x0).enable_ch_clk(0x4),
            "Channel out of range (expected 0..=3)"
        ),
        (
            test_clk_control_disable_ch_clk_panic,
            ClkControl(0x0).disable_ch_clk(0x4),
            "Channel out of range (expected 0..=3)"
        ),
        (
            test_clk_control_is_ch_clk_enabled_panic,
            ClkControl(0x0).is_ch_clk_enabled(0x4),
            "Channel out of range (expected 0..=3)"
        ),
    );

    #[test]
    fn struct_int_control_functions() {
        let val = IntControl(0x0).disable_ch_int(0).enable_ch_int(3);
        assert!(val.is_ch_int_enabled(3));
        assert!(!val.is_ch_int_enabled(0));
        assert_eq!(val.0, 0x0000_0008);
    }

    test_should_panic!(
        (
            test_int_control_enable_ch_int_panic,
            IntControl(0x0).enable_ch_int(0x4),
            "Channel out of range (expected 0..=3)"
        ),
        (
            test_int_control_disable_ch_int_panic,
            IntControl(0x0).disable_ch_int(0x4),
            "Channel out of range (expected 0..=3)"
        ),
        (
            test_int_control_is_ch_int_enabled_panic,
            IntControl(0x0).is_ch_int_enabled(0x4),
            "Channel out of range (expected 0..=3)"
        ),
    );

    #[test]
    fn struct_int_status_functions() {
        let val = IntStatus(0x0).clear_ch_int_pending(3);
        assert!(val.is_ch_int_pending(3));
        assert_eq!(val.0, 0x0000_0008);
    }

    test_should_panic!(
        (
            test_int_status_is_ch_int_pending_panic,
            IntStatus(0x0).is_ch_int_pending(0x4),
            "Channel out of range (expected 0..=3)"
        ),
        (
            test_int_status_clear_ch_int_pending_panic,
            IntStatus(0x0).clear_ch_int_pending(0x4),
            "Channel out of range (expected 0..=3)"
        ),
    );

    #[test]
    fn struct_tb_control_functions() {
        let mut val = TBControl(0x0).set_clk_div(0xFFF);
        assert_eq!(val.clk_div(), 0xFFF);
        assert_eq!(val.0, 0x0FFF_0000);

        val = TBControl(0x0).set_prd_reg_op_mode(PrdRegOpMode::Immediate);
        assert_eq!(val.prd_reg_op_mode(), PrdRegOpMode::Immediate);
        assert_eq!(val.0, 0x0000_0008);
        val = val.set_prd_reg_op_mode(PrdRegOpMode::Shadow);
        assert_eq!(val.prd_reg_op_mode(), PrdRegOpMode::Shadow);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_cnt_mode(CntMode::CountUpAndDown);
        assert_eq!(val.cnt_mode(), CntMode::CountUpAndDown);
        assert_eq!(val.0, 0x0000_0002);
        val = val.set_cnt_mode(CntMode::CountDown);
        assert_eq!(val.cnt_mode(), CntMode::CountDown);
        assert_eq!(val.0, 0x0000_0001);
        val = val.set_cnt_mode(CntMode::CountUp);
        assert_eq!(val.cnt_mode(), CntMode::CountUp);
        assert_eq!(val.0, 0x0000_0000);
    }

    test_should_panic!((
        test_tb_control_set_clk_div_panic,
        TBControl(0x0).set_clk_div(0x1000),
        "Clock divider out of range (expected 0..=0xFFF)"
    ),);

    #[test]
    fn struct_tb_status_functions() {
        let mut val = TBStatus(0x0).clear_ctr_max();
        assert!(val.is_ctr_max());
        assert_eq!(val.0, 0x0000_0004);

        val = TBStatus(0x0000_0001);
        assert_eq!(val.cnt_dir(), CntMode::CountUp);
        val = TBStatus(0x0000_0000);
        assert_eq!(val.cnt_dir(), CntMode::CountDown);
    }

    #[test]
    fn struct_tb_counter_functions() {
        let val = TBCounter(0x0).set_tb_cnt(0xFFFF);
        assert_eq!(val.tb_cnt(), 0xFFFF);
        assert_eq!(val.0, 0x0000_FFFF);
    }

    #[test]
    fn struct_tb_period_functions() {
        let val = TBPeriod(0x0).set_tb_prd(0xFFFF);
        assert_eq!(val.tb_prd(), 0xFFFF);
        assert_eq!(val.0, 0x0000_FFFF);
    }

    #[test]
    fn struct_cmp_control_functions() {
        let mut val = CmpControl(0x0).set_cmp_b_op_mode(PrdRegOpMode::Immediate);
        assert_eq!(val.cmp_b_op_mode(), PrdRegOpMode::Immediate);
        assert_eq!(val.0, 0x0000_0040);
        val = val.set_cmp_b_op_mode(PrdRegOpMode::Shadow);
        assert_eq!(val.cmp_b_op_mode(), PrdRegOpMode::Shadow);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_cmp_a_op_mode(PrdRegOpMode::Immediate);
        assert_eq!(val.cmp_a_op_mode(), PrdRegOpMode::Immediate);
        assert_eq!(val.0, 0x0000_0010);
        val = val.set_cmp_a_op_mode(PrdRegOpMode::Shadow);
        assert_eq!(val.cmp_a_op_mode(), PrdRegOpMode::Shadow);
        assert_eq!(val.0, 0x0000_0000);

        for i in 0..4 {
            let (mode, reg_val) = match i {
                0 => (ShdwLdMode::Freeze, 0x0000_000C),
                1 => (ShdwLdMode::Mode2, 0x0000_0008),
                2 => (ShdwLdMode::Mode1, 0x0000_0004),
                3 => (ShdwLdMode::Mode0, 0x0000_0000),
                _ => unreachable!(),
            };
            val = val.set_cmp_b_shdw_ld_mode(mode);
            assert_eq!(val.cmp_b_shdw_ld_mode(), mode);
            assert_eq!(val.0, reg_val);
        }

        for i in 0..4 {
            let (mode, reg_val) = match i {
                0 => (ShdwLdMode::Freeze, 0x0000_0003),
                1 => (ShdwLdMode::Mode2, 0x0000_0002),
                2 => (ShdwLdMode::Mode1, 0x0000_0001),
                3 => (ShdwLdMode::Mode0, 0x0000_0000),
                _ => unreachable!(),
            };
            val = val.set_cmp_a_shdw_ld_mode(mode);
            assert_eq!(val.cmp_a_shdw_ld_mode(), mode);
            assert_eq!(val.0, reg_val);
        }
    }

    #[test]
    fn struct_comparator_functions() {
        let val = Comparator(0x0).set_cmp(0xFFFF);
        assert_eq!(val.cmp(), 0xFFFF);
        assert_eq!(val.0, 0x0000_FFFF);
    }

    #[test]
    fn struct_aq_control_functions() {
        let mut val = AqControl(0x0).set_init_level(InitLevel::High);
        assert_eq!(val.init_level(), InitLevel::High);
        assert_eq!(val.0, 0x0001_0000);
        val = val.set_init_level(InitLevel::Low);
        assert_eq!(val.init_level(), InitLevel::Low);
        assert_eq!(val.0, 0x0000_0000);

        for i in 0..4 {
            let (mode, reg_val) = match i {
                0 => (ActionMode::Toggle, 0x0000_0C00),
                1 => (ActionMode::SetHigh, 0x0000_0800),
                2 => (ActionMode::SetLow, 0x0000_0400),
                3 => (ActionMode::NoOp, 0x0000_0000),
                _ => unreachable!(),
            };
            val = val.set_cbd_mode(mode);
            assert_eq!(val.cbd_mode(), mode);
            assert_eq!(val.0, reg_val);
        }

        for i in 0..4 {
            let (mode, reg_val) = match i {
                0 => (ActionMode::Toggle, 0x0000_0300),
                1 => (ActionMode::SetHigh, 0x0000_0200),
                2 => (ActionMode::SetLow, 0x0000_0100),
                3 => (ActionMode::NoOp, 0x0000_0000),
                _ => unreachable!(),
            };
            val = val.set_cbu_mode(mode);
            assert_eq!(val.cbu_mode(), mode);
            assert_eq!(val.0, reg_val);
        }

        val = AqControl(0x0);
        for i in 0..4 {
            let (mode, reg_val) = match i {
                0 => (ActionMode::Toggle, 0x0000_00C0),
                1 => (ActionMode::SetHigh, 0x0000_0080),
                2 => (ActionMode::SetLow, 0x0000_0040),
                3 => (ActionMode::NoOp, 0x0000_0000),
                _ => unreachable!(),
            };
            val = val.set_cad_mode(mode);
            assert_eq!(val.cad_mode(), mode);
            assert_eq!(val.0, reg_val);
        }

        for i in 0..4 {
            let (mode, reg_val) = match i {
                0 => (ActionMode::Toggle, 0x0000_0030),
                1 => (ActionMode::SetHigh, 0x0000_0020),
                2 => (ActionMode::SetLow, 0x0000_0010),
                3 => (ActionMode::NoOp, 0x0000_0000),
                _ => unreachable!(),
            };
            val = val.set_cau_mode(mode);
            assert_eq!(val.cau_mode(), mode);
            assert_eq!(val.0, reg_val);
        }

        for i in 0..4 {
            let (mode, reg_val) = match i {
                0 => (ActionMode::Toggle, 0x0000_000C),
                1 => (ActionMode::SetHigh, 0x0000_0008),
                2 => (ActionMode::SetLow, 0x0000_0004),
                3 => (ActionMode::NoOp, 0x0000_0000),
                _ => unreachable!(),
            };
            val = val.set_prd_mode(mode);
            assert_eq!(val.prd_mode(), mode);
            assert_eq!(val.0, reg_val);
        }

        for i in 0..4 {
            let (mode, reg_val) = match i {
                0 => (ActionMode::Toggle, 0x0000_0003),
                1 => (ActionMode::SetHigh, 0x0000_0002),
                2 => (ActionMode::SetLow, 0x0000_0001),
                3 => (ActionMode::NoOp, 0x0000_0000),
                _ => unreachable!(),
            };
            val = val.set_zro_mode(mode);
            assert_eq!(val.zro_mode(), mode);
            assert_eq!(val.0, reg_val);
        }
    }

    #[test]
    fn struct_evt_trig_sel_functions() {
        let mut val = EvtTrigSel(0x0).enable_int();
        assert!(val.is_int_enabled());
        assert_eq!(val.0, 0x0000_0008);
        val = val.disable_int();
        assert!(!val.is_int_enabled());
        assert_eq!(val.0, 0x0000_0000);

        for i in 0..6 {
            let (event, reg_val) = match i {
                0 => (IntEvent::Event5, 0x0000_0007),
                1 => (IntEvent::Event4, 0x0000_0006),
                2 => (IntEvent::Event3, 0x0000_0005),
                3 => (IntEvent::Event2, 0x0000_0004),
                4 => (IntEvent::Event1, 0x0000_0002),
                5 => (IntEvent::Event0, 0x0000_0001),
                _ => unreachable!(),
            };
            val = val.set_int_event(event);
            assert_eq!(val.int_event(), event);
            assert_eq!(val.0, reg_val);
        }
    }

    #[test]
    fn struct_cnt_dis_functions() {
        let val = CntDis(0x0000_0001);
        assert!(val.cnt_dis());
    }
}
