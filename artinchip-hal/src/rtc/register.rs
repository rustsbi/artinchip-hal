//! Real Time Clock register blocks and registers.

use volatile_register::{RO, RW};

/// RTC configuration register block.
#[repr(C)]
pub struct RegisterBlock {
    /// RTC control register (`RTC_CTL`).
    #[doc(alias = "RTC_CTL")]
    pub ctrl: RW<Control>,
    /// RTC initialization register (`RTC_INIT`).
    #[doc(alias = "RTC_INIT")]
    pub init: RW<Init>,
    /// RTC always on interrupt enable register (`RTC_AON_IRQ_EN`).
    #[doc(alias = "RTC_AON_IRQ_EN")]
    pub aon_int_en: RW<AonIrqEnable>,
    /// RTC always on interrupt status register (`RTC_AON_IRQ_STA`).
    #[doc(alias = "RTC_AON_IRQ_STA")]
    pub aon_int_status: RW<AonIrqStatus>,
    _reserved0: [u8; 0x10],
    /// RTC time registers (`RTC_TIME`).
    #[doc(alias = "RTC_TIME")]
    pub time: [RW<Time>; 4],
    /// RTC alarm registers (`RTC_ALARM`).
    #[doc(alias = "RTC_ALARM")]
    pub alarm: [RW<Alarm>; 4],
    /// RTC calibration 0 register (`RTC_CALI0`).
    #[doc(alias = "RTC_CALI0")]
    pub cali0: RW<Calibration0>,
    /// RTC calibration 1 register (`RTC_CALI1`).
    #[doc(alias = "RTC_CALI1")]
    pub cali1: RW<Calibration1>,
    _reserved1: [u8; 0x8],
    /// RTC analog 0 register (`RTC_ANALOG0`).
    #[doc(alias = "RTC_ANALOG0")]
    pub analog0: RW<Analog0>,
    /// RTC analog 1 register (`RTC_ANALOG1`).
    #[doc(alias = "RTC_ANALOG1")]
    pub analog1: RW<Analog1>,
    /// RTC analog 2 register (`RTC_ANALOG2`).
    #[doc(alias = "RTC_ANALOG2")]
    pub analog2: RW<Analog2>,
    /// RTC analog 3 register (`RTC_ANALOG3`).
    #[doc(alias = "RTC_ANALOG3")]
    pub analog3: RW<Analog3>,
    _reserved2: [u8; 0x9C],
    /// RTC write key register (`RTC_WR_KEY`).
    #[doc(alias = "RTC_WR_KEY")]
    pub write_key: RW<WriteKey>,
    /// Boot info register (`BOOT_INFO`).
    #[doc(alias = "BOOT_INFO")]
    pub boot_info: RW<BootInfo>,
    /// System backup registers (`SYS_BAK`).
    #[doc(alias = "SYS_BAK")]
    pub sys_backup: [RW<SysBackup>; 15],
    _reserved3: [u8; 0x6C0],
    /// RTC time counter value register (`RTC_TCNT_VAL`).
    #[doc(alias = "RTC_TCNT_VAL")]
    pub tcnt_val: RO<u32>,
    /// RTC 32K detect register (`RTC_32K_DET`).
    #[doc(alias = "RTC_32K_DET")]
    pub detect_32k: RW<Detect32k>,
    _reserved4: [u8; 0xF4],
    /// RTC version register (`RTC_VER`).
    #[doc(alias = "RTC_VER")]
    pub version: RO<u32>,
}

/// IO output selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IoOutputSel {
    /// Disabled output (high impedance).
    Disabled,
    /// Enable output (active low).
    ActiveLow,
    /// Alarm triggered output (low on alarm).
    AlarmTrigger,
    /// 32.768kHz crystal clock output.
    ClockOutputs,
}

/// RTC control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Control(u32);

impl Control {
    const RTC_IO_LEVEL: u32 = 0x1 << 7;
    const RTC_IO_IE: u32 = 0x1 << 6;
    const RTC_IO_SEL: u32 = 0x3 << 4;
    const ALARM_EN: u32 = 0x1 << 2;
    const TCNT_EN: u32 = 0x1;

    /// Get rtc io level (`RTC_IO_LEVEL`).
    #[doc(alias = "RTC_IO_LEVEL")]
    #[inline]
    pub const fn rtc_io_level(self) -> bool {
        self.0 & Self::RTC_IO_LEVEL != 0
    }
    /// Enable rtc io input mode (`RTC_IO_IE`).
    #[doc(alias = "RTC_IO_IE")]
    #[inline]
    pub const fn enable_rtc_io_input(self) -> Self {
        Self(self.0 | Self::RTC_IO_IE)
    }
    /// Disable rtc io input mode.
    #[inline]
    pub const fn disable_rtc_io_input(self) -> Self {
        Self(self.0 & !Self::RTC_IO_IE)
    }
    /// Check if rtc io input mode is enabled.
    #[inline]
    pub const fn is_rtc_io_input_enabled(self) -> bool {
        self.0 & Self::RTC_IO_IE != 0
    }
    /// Set rtc io output selection (`RTC_IO_SEL`).
    #[doc(alias = "RTC_IO_SEL")]
    #[inline]
    pub const fn set_rtc_io_output_selection(self, sel: IoOutputSel) -> Self {
        Self((self.0 & !Self::RTC_IO_SEL) | (Self::RTC_IO_SEL & ((sel as u32) << 4)))
    }
    /// Get rtc io output selection.
    #[inline]
    pub const fn rtc_io_output_selection(self) -> IoOutputSel {
        match (self.0 & Self::RTC_IO_SEL) >> 4 {
            0 => IoOutputSel::Disabled,
            1 => IoOutputSel::ActiveLow,
            2 => IoOutputSel::AlarmTrigger,
            3 => IoOutputSel::ClockOutputs,
            _ => unreachable!(),
        }
    }
    /// Enable alarm (`ALARM_EN`).
    #[doc(alias = "ALARM_EN")]
    #[inline]
    pub const fn enable_alarm(self) -> Self {
        Self(self.0 | Self::ALARM_EN)
    }
    /// Disable alarm.
    #[inline]
    pub const fn disable_alarm(self) -> Self {
        Self(self.0 & !Self::ALARM_EN)
    }
    /// Check if alarm is enabled.
    #[inline]
    pub const fn is_alarm_enabled(self) -> bool {
        self.0 & Self::ALARM_EN != 0
    }
    /// Enable time counter (`TCNT_EN`).
    ///
    /// Each time `TCNT_EN` transitions from 0 to 1, the value of `TCNT_INIT` will be checked,
    /// and if it is 1, the value of `RTC_TIME` will be initialized into TCNT.
    #[doc(alias = "TCNT_EN")]
    #[inline]
    pub const fn enable_time_cnt(self) -> Self {
        Self(self.0 | Self::TCNT_EN)
    }
    /// Disable time counter.
    #[inline]
    pub const fn disable_time_cnt(self) -> Self {
        Self(self.0 & !Self::TCNT_EN)
    }
    /// Check if time counter is enabled.
    #[inline]
    pub const fn is_time_cnt_enabled(self) -> bool {
        self.0 & Self::TCNT_EN != 0
    }
}

/// RTC initialization register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Init(u32);

impl Init {
    const TCNT_INIT: u32 = 0x1;

    /// Set time counter initial bit (`TCNT_INIT`).
    #[doc(alias = "TCNT_INIT")]
    #[inline]
    pub const fn set_time_cnt_init(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::TCNT_INIT)
        } else {
            Self(self.0 & !Self::TCNT_INIT)
        }
    }
    /// Get time counter initial bit.
    #[inline]
    pub const fn time_cnt_init(self) -> bool {
        self.0 & Self::TCNT_INIT != 0
    }
}

/// RTC always on interrupt enable register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AonIrqEnable(u32);

impl AonIrqEnable {
    const RTC_32K_ERR_IRQ_EN: u32 = 0x1 << 2;
    const ALARM_IRQ_EN: u32 = 0x1;

    /// Enable 32k clock error interrupt (`RTC_32K_ERR_IRQ_EN`).
    #[doc(alias = "RTC_32K_ERR_IRQ_EN")]
    #[inline]
    pub const fn enable_rtc_32k_err_irq(self) -> Self {
        Self(self.0 | Self::RTC_32K_ERR_IRQ_EN)
    }
    /// Disable 32k clock error interrupt.
    #[inline]
    pub const fn disable_rtc_32k_err_irq(self) -> Self {
        Self(self.0 & !Self::RTC_32K_ERR_IRQ_EN)
    }
    /// Check if 32k clock error interrupt is enabled.
    #[inline]
    pub const fn is_rtc_32k_err_irq_enabled(self) -> bool {
        self.0 & Self::RTC_32K_ERR_IRQ_EN != 0
    }
    /// Enable alarm interrupt (`ALARM_IRQ_EN`).
    #[doc(alias = "ALARM_IRQ_EN")]
    #[inline]
    pub const fn enable_alarm_irq(self) -> Self {
        Self(self.0 | Self::ALARM_IRQ_EN)
    }
    /// Disable alarm interrupt.
    #[inline]
    pub const fn disable_alarm_irq(self) -> Self {
        Self(self.0 & !Self::ALARM_IRQ_EN)
    }
    /// Check if alarm interrupt is enabled.
    #[inline]
    pub const fn is_alarm_irq_enabled(self) -> bool {
        self.0 & Self::ALARM_IRQ_EN != 0
    }
}

/// RTC always on interrupt status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AonIrqStatus(u32);

impl AonIrqStatus {
    const RTC_32K_ERR_IRQ_STA: u32 = 0x1 << 2;
    const RTC_IO_STA: u32 = 0x1 << 1;
    const ALARM_IRQ_STA: u32 = 0x1;

    /// Get 32k clock error interrupt status (`RTC_32K_ERR_IRQ_STA`).
    #[doc(alias = "RTC_32K_ERR_IRQ_STA")]
    #[inline]
    pub const fn rtc_32k_err_irq_status(self) -> bool {
        self.0 & Self::RTC_32K_ERR_IRQ_STA != 0
    }
    /// Clear 32k clock error interrupt status.
    #[inline]
    pub const fn clear_rtc_32k_err_irq_status(self) -> Self {
        Self(self.0 | Self::RTC_32K_ERR_IRQ_STA)
    }
    /// Get io enable status (`RTC_IO_STA`).
    ///
    /// RTC_IO is designed as an open-drain output and requires a pull-up resistor connected to the power supply.
    /// When this bit is set to 1, the output is enabled, meaning RTC_IO is at a low level.
    #[doc(alias = "RTC_IO_STA")]
    #[inline]
    pub const fn rtc_io_enable_status(self) -> bool {
        self.0 & Self::RTC_IO_STA != 0
    }
    /// Clear io enable status.
    #[inline]
    pub const fn clear_rtc_io_enable_status(self) -> Self {
        Self(self.0 | Self::RTC_IO_STA)
    }
    /// Get alarm interrupt status (`ALARM_IRQ_STA`).
    #[doc(alias = "ALARM_IRQ_STA")]
    #[inline]
    pub const fn alarm_irq_status(self) -> bool {
        self.0 & Self::ALARM_IRQ_STA != 0
    }
    /// Clear alarm interrupt status.
    #[inline]
    pub const fn clear_alarm_irq_status(self) -> Self {
        Self(self.0 | Self::ALARM_IRQ_STA)
    }
}

/// RTC time register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Time(u32);

impl Time {
    const TIME_SET: u32 = 0xFF;

    /// Set time value.
    #[inline]
    pub const fn set_time(self, value: u8) -> Self {
        Self((self.0 & !Self::TIME_SET) | (Self::TIME_SET & value as u32))
    }

    /// Get time value.
    #[inline]
    pub const fn time(self) -> u8 {
        (self.0 & Self::TIME_SET) as u8
    }
}

/// RTC alarm register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Alarm(u32);

impl Alarm {
    const ALARM_SET: u32 = 0xFF;

    /// Set alarm value.
    #[inline]
    pub const fn set_alarm(self, value: u8) -> Self {
        Self((self.0 & !Self::ALARM_SET) | (Self::ALARM_SET & value as u32))
    }

    /// Get alarm value.
    #[inline]
    pub const fn alarm(self) -> u8 {
        (self.0 & Self::ALARM_SET) as u8
    }
}

/// RTC calibration 0 register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Calibration0(u32);

impl Calibration0 {
    const CALI_VAL: u32 = 0xFF;

    /// Set calibration low value (`CALI_VAL`).
    ///
    /// The calibration time is `CALI_VAL` × 1 ppm.
    #[doc(alias = "CALI_VAL")]
    #[inline]
    pub const fn set_cali_value_low(self, value: u8) -> Self {
        Self((self.0 & !Self::CALI_VAL) | (Self::CALI_VAL & value as u32))
    }

    /// Get calibration low value.
    #[inline]
    pub const fn cali_value_low(self) -> u8 {
        (self.0 & Self::CALI_VAL) as u8
    }
}

/// Calibration direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CaliDirection {
    /// Decrease time.
    Decrease,
    /// Increase time.
    Increase,
}

/// RTC calibration 1 register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Calibration1(u32);

impl Calibration1 {
    const CALI_DIR: u32 = 0x1 << 7;
    const CALI_VAL: u32 = 0x3;

    /// Set calibration direction (`CALI_DIR`).
    #[doc(alias = "CALI_DIR")]
    #[inline]
    pub const fn set_cali_direction(self, direction: CaliDirection) -> Self {
        Self((self.0 & !Self::CALI_DIR) | (Self::CALI_DIR & ((direction as u32) << 7)))
    }
    /// Get calibration direction.
    #[inline]
    pub const fn cali_direction(self) -> CaliDirection {
        match (self.0 & Self::CALI_DIR) >> 7 {
            0 => CaliDirection::Decrease,
            1 => CaliDirection::Increase,
            _ => unreachable!(),
        }
    }
    /// Set calibration value high (`CALI_VAL`).
    ///
    /// The calibration time is `CALI_VAL` × 1 ppm.
    #[doc(alias = "CALI_VAL")]
    #[inline]
    pub const fn set_cali_value_high(self, value: u8) -> Self {
        assert!(value < 4, "Calibration value out of range (expected 0..=3)");
        Self((self.0 & !Self::CALI_VAL) | (Self::CALI_VAL & (value as u32)))
    }
    /// Get calibration value high.
    #[inline]
    pub const fn cali_value_high(self) -> u8 {
        (self.0 & Self::CALI_VAL) as u8
    }
}

/// RC1M reference voltage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rc1mIsel {
    IBIAS,
    BG,
}

/// LDO 1.8v voltage selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ldo1P8Vol {
    /// 1.9V.
    V1_9,
    /// 1.8V.
    V1_8,
    /// 1.7V.
    V1_7,
    /// 1.6V.
    V1_6,
    /// 1.5V.
    V1_5,
    /// 1.4V.
    V1_4,
    /// 1.3V.
    V1_3,
    /// 1.2V.
    V1_2,
}

/// RTC analog 0 register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Analog0(u32);

impl Analog0 {
    const RC1M_ISEL: u32 = 0x1 << 7;
    const RC1M_EN: u32 = 0x1 << 6;
    const LDO18_BYPASS: u32 = 0x1 << 4;
    const LDO18_VOL: u32 = 0x7 << 1;
    const LDO18_EN: u32 = 0x1;

    /// Set RC1M reference voltage selection (`RC1M_ISEL`).
    #[doc(alias = "RC1M_ISEL")]
    #[inline]
    pub const fn set_rc1m_isel(self, isel: Rc1mIsel) -> Self {
        Self((self.0 & !Self::RC1M_ISEL) | (Self::RC1M_ISEL & ((isel as u32) << 7)))
    }
    /// Get RC1M reference voltage selection.
    #[inline]
    pub const fn rc1m_isel(self) -> Rc1mIsel {
        match (self.0 & Self::RC1M_ISEL) >> 7 {
            0 => Rc1mIsel::IBIAS,
            1 => Rc1mIsel::BG,
            _ => unreachable!(),
        }
    }
    /// Enable RC1M oscillator (`RC1M_EN`).
    #[doc(alias = "RC1M_EN")]
    #[inline]
    pub const fn enable_rc1m(self) -> Self {
        Self(self.0 | Self::RC1M_EN)
    }
    /// Disable RC1M oscillator.
    #[inline]
    pub const fn disable_rc1m(self) -> Self {
        Self(self.0 & !Self::RC1M_EN)
    }
    /// Check if RC1M oscillator is enabled.
    #[inline]
    pub const fn is_rc1m_enabled(self) -> bool {
        (self.0 & Self::RC1M_EN) != 0
    }
    /// Enable ldo 1.8V bypass mode (`LDO18_BYPASS`).
    ///
    /// When enabled, the XTAL 32K operates in the VCC_RTC domain.
    #[doc(alias = "LDO18_BYPASS")]
    #[inline]
    pub const fn enable_ldo18_bypass(self) -> Self {
        Self(self.0 | Self::LDO18_BYPASS)
    }
    /// Disable ldo 1.8V bypass mode.
    #[inline]
    pub const fn disable_ldo18_bypass(self) -> Self {
        Self(self.0 & !Self::LDO18_BYPASS)
    }
    /// Check if ldo 1.8V bypass mode is enabled.
    #[inline]
    pub const fn is_ldo18_bypass_enabled(self) -> bool {
        (self.0 & Self::LDO18_BYPASS) != 0
    }
    /// Set ldo 1.8V voltage selection (`LDO18_VOL`).
    #[doc(alias = "LDO18_VOL")]
    #[inline]
    pub const fn set_ldo18_vol(self, vol: Ldo1P8Vol) -> Self {
        Self((self.0 & !Self::LDO18_VOL) | (Self::LDO18_VOL & ((vol as u32) << 1)))
    }
    /// Get ldo 1.8V voltage selection.
    #[inline]
    pub const fn ldo18_vol(self) -> Ldo1P8Vol {
        match (self.0 & Self::LDO18_VOL) >> 1 {
            0 => Ldo1P8Vol::V1_9,
            1 => Ldo1P8Vol::V1_8,
            2 => Ldo1P8Vol::V1_7,
            3 => Ldo1P8Vol::V1_6,
            4 => Ldo1P8Vol::V1_5,
            5 => Ldo1P8Vol::V1_4,
            6 => Ldo1P8Vol::V1_3,
            7 => Ldo1P8Vol::V1_2,
            _ => unreachable!(),
        }
    }
    /// Enable ldo 1.8V (`LDO18_EN`).
    #[doc(alias = "LDO18_EN")]
    #[inline]
    pub const fn enable_ldo18(self) -> Self {
        Self(self.0 | Self::LDO18_EN)
    }
    /// Disable ldo 1.8V.
    #[inline]
    pub const fn disable_ldo18(self) -> Self {
        Self(self.0 & !Self::LDO18_EN)
    }
    /// Check if ldo 1.8V is enabled.
    #[inline]
    pub const fn is_ldo18_enabled(self) -> bool {
        (self.0 & Self::LDO18_EN) != 0
    }
}

/// LDO 1.1v current selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ldo1P1CurSel {
    /// 0.5uA.
    I0_5uA,
    /// 1.0uA.
    I1_0uA,
    /// 2.0uA.
    I2_0uA,
    /// 3.0uA.
    I3_0uA,
}

/// LDO 1.1V voltage selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ldo1P1Vol {
    /// 1.1V.
    V1_10,
    /// 1.05V.
    V1_05,
    /// 1.0V.
    V1_00,
    /// 0.95V.
    V0_95,
    /// 0.9V.
    V0_90,
    /// 0.85V.
    V0_85,
    /// 0.8V.
    V0_80,
}

/// RTC analog 1 register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Analog1(u32);

impl Analog1 {
    const PD_CUR_SEL: u32 = 0x3 << 5;
    const PD_CUR_EN: u32 = 0x1 << 4;
    const LDO11_VOL: u32 = 0x7 << 1;
    const LDO11_LPEN: u32 = 0x1;

    /// Set PD current selection (`PD_CUR_SEL`).
    #[doc(alias = "PD_CUR_SEL")]
    #[inline]
    pub const fn set_pd_cur_sel(self, sel: Ldo1P1CurSel) -> Self {
        Self((self.0 & !Self::PD_CUR_SEL) | (Self::PD_CUR_SEL & ((sel as u32) << 5)))
    }
    /// Get ldo 1.1v current selection.
    #[inline]
    pub const fn pd_cur_sel(self) -> Ldo1P1CurSel {
        match (self.0 & Self::PD_CUR_SEL) >> 5 {
            0 => Ldo1P1CurSel::I0_5uA,
            1 => Ldo1P1CurSel::I1_0uA,
            2 => Ldo1P1CurSel::I2_0uA,
            3 => Ldo1P1CurSel::I3_0uA,
            _ => unreachable!(),
        }
    }
    /// Enable ldo 1.1V (`PD_CUR_EN`).
    #[doc(alias = "PD_CUR_EN")]
    #[inline]
    pub const fn enable_ldo11(self) -> Self {
        Self(self.0 | Self::PD_CUR_EN)
    }
    /// Disable ldo 1.1V.
    #[inline]
    pub const fn disable_ldo11(self) -> Self {
        Self(self.0 & !Self::PD_CUR_EN)
    }
    /// Check if ldo 1.1V is enabled.
    #[inline]
    pub const fn is_ldo11_enabled(self) -> bool {
        (self.0 & Self::PD_CUR_EN) != 0
    }
    /// Set ldo 1.1V voltage selection (`LDO11_VOL`).
    #[doc(alias = "LDO11_VOL")]
    #[inline]
    pub const fn set_ldo11_vol(self, vol: Ldo1P1Vol) -> Self {
        Self((self.0 & !Self::LDO11_VOL) | (Self::LDO11_VOL & ((vol as u32) << 1)))
    }
    /// Get ldo 1.1V voltage selection.
    #[inline]
    pub const fn ldo11_vol(self) -> Ldo1P1Vol {
        match (self.0 & Self::LDO11_VOL) >> 1 {
            0 => Ldo1P1Vol::V1_10,
            1 => Ldo1P1Vol::V1_05,
            2 => Ldo1P1Vol::V1_00,
            3 => Ldo1P1Vol::V0_95,
            4 => Ldo1P1Vol::V0_90,
            5 => Ldo1P1Vol::V0_85,
            6 => Ldo1P1Vol::V0_80,
            _ => unreachable!(),
        }
    }
    /// Enable ldo 1.1V low power mode (`LDO11_LPEN`).
    #[doc(alias = "LDO11_LPEN")]
    #[inline]
    pub const fn enable_ldo11_lp(self) -> Self {
        Self(self.0 | Self::LDO11_LPEN)
    }
    /// Disable ldo 1.1V low power mode.
    #[inline]
    pub const fn disable_ldo11_lp(self) -> Self {
        Self(self.0 & !Self::LDO11_LPEN)
    }
    /// Check if ldo 1.1V low power mode is enabled.
    #[inline]
    pub const fn is_ldo11_lp_enabled(self) -> bool {
        (self.0 & Self::LDO11_LPEN) != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AtbSel {
    /// Internal reference voltage.
    Vref,
    /// Oscillator voltage.
    Vosc,
    /// Bandgap reference current (~20 nA).
    IbgIbi,
    /// Voltage detection current (~15.6 nA).
    IbgVdet,
}

/// RTC analog 2 register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Analog2(u32);

impl Analog2 {
    const ATB_SEL: u32 = 0x3 << 5;
    const ATB_EN: u32 = 0x1 << 4;
    const XTAL32K_STRENGTH_UP: u32 = 0x1 << 3;
    const XTAL32K_DRV: u32 = 0x3;

    /// Set analogical selection (`ATB_SEL`).
    #[doc(alias = "ATB_SEL")]
    #[inline]
    pub const fn set_atb_sel(self, sel: AtbSel) -> Self {
        Self((self.0 & !Self::ATB_SEL) | (Self::ATB_SEL & ((sel as u32) << 5)))
    }
    /// Get analogical selection.
    #[inline]
    pub const fn atb_sel(self) -> AtbSel {
        match (self.0 & Self::ATB_SEL) >> 5 {
            0 => AtbSel::Vref,
            1 => AtbSel::Vosc,
            2 => AtbSel::IbgIbi,
            3 => AtbSel::IbgVdet,
            _ => unreachable!(),
        }
    }
    /// Enable analogical test (`ATB_EN`).
    #[doc(alias = "ATB_EN")]
    #[inline]
    pub const fn enable_atb(self) -> Self {
        Self(self.0 | Self::ATB_EN)
    }
    /// Disable analogical test.
    #[inline]
    pub const fn disable_atb(self) -> Self {
        Self(self.0 & !Self::ATB_EN)
    }
    /// Check if analogical test is enabled.
    #[inline]
    pub const fn is_atb_enabled(self) -> bool {
        (self.0 & Self::ATB_EN) != 0
    }
    /// Enable xtal 32k strength up (`XTAL32K_STRENGTH_UP`).
    #[doc(alias = "XTAL32K_STRENGTH_UP")]
    #[inline]
    pub const fn enable_xtal32k_strength_up(self) -> Self {
        Self(self.0 | Self::XTAL32K_STRENGTH_UP)
    }
    /// Disable xtal 32k strength up.
    #[inline]
    pub const fn disable_xtal32k_strength_up(self) -> Self {
        Self(self.0 & !Self::XTAL32K_STRENGTH_UP)
    }
    /// Check if xtal 32k strength up is enabled.
    #[inline]
    pub const fn is_xtal32k_strength_up_enabled(self) -> bool {
        (self.0 & Self::XTAL32K_STRENGTH_UP) != 0
    }
    /// Set xtal 32k driver strength (`XTAL32K_DRV`).
    #[doc(alias = "XTAL32K_DRV")]
    #[inline]
    pub const fn set_xtal32k_drv(self, drv: u8) -> Self {
        assert!(drv < 4, "XTAL32K_DRV out of range (expected 0..=3)");
        Self((self.0 & !Self::XTAL32K_DRV) | (Self::XTAL32K_DRV & (drv as u32)))
    }
}

/// RTC analog 3 register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Analog3(u32);

impl Analog3 {
    const LDO12_XTAL32K_SW: u32 = 0x1 << 1;
    const XTAL32K_EN: u32 = 0x1;

    /// Enable LDO12 XTAL32K switch (`LDO12_XTAL32K_SW`).
    #[doc(alias = "LDO12_XTAL32K_SW")]
    #[inline]
    pub const fn enable_ldo12_xtal32k_sw(self) -> Self {
        Self(self.0 | Self::LDO12_XTAL32K_SW)
    }
    /// Disable LDO12 XTAL32K switch.
    #[inline]
    pub const fn disable_ldo12_xtal32k_sw(self) -> Self {
        Self(self.0 & !Self::LDO12_XTAL32K_SW)
    }
    /// Check if LDO12 XTAL32K switch is enabled.
    #[inline]
    pub const fn is_ldo12_xtal32k_sw_enabled(self) -> bool {
        (self.0 & Self::LDO12_XTAL32K_SW) != 0
    }
    /// Enable XTAL32K (`XTAL32K_EN`).
    #[doc(alias = "XTAL32K_EN")]
    #[inline]
    pub const fn enable_xtal32k(self) -> Self {
        Self(self.0 | Self::XTAL32K_EN)
    }
    /// Disable XTAL32K.
    #[inline]
    pub const fn disable_xtal32k(self) -> Self {
        Self(self.0 & !Self::XTAL32K_EN)
    }
    /// Check if XTAL32K is enabled.
    #[inline]
    pub const fn is_xtal32k_enabled(self) -> bool {
        (self.0 & Self::XTAL32K_EN) != 0
    }
}

/// RTC write key register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct WriteKey(u32);

impl WriteKey {
    const WR_KEY: u32 = 0xFF;

    /// Set write key value (`WR_KEY`).
    ///
    /// - 0xAC: write allowed
    /// - Others: write not allowed.
    #[doc(alias = "WR_KEY")]
    #[inline]
    pub const fn set_write_key(self, key: u8) -> Self {
        Self((self.0 & !Self::WR_KEY) | (Self::WR_KEY & (key as u32)))
    }
    /// Get write key value.
    #[inline]
    pub const fn write_key(self) -> u8 {
        (self.0 & Self::WR_KEY) as u8
    }
}

/// Boot info register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BootInfo(u32);

impl BootInfo {
    const REBOOT_REASON: u32 = 0xF << 4;
    const BOOT_DEV: u32 = 0xF;

    /// Set reboot reason (`REBOOT_REASON`).
    #[doc(alias = "REBOOT_REASON")]
    #[inline]
    pub const fn set_reboot_reason(self, reason: u8) -> Self {
        assert!(reason < 16, "REBOOT_REASON out of range (expected 0..=15)");
        Self((self.0 & !Self::REBOOT_REASON) | ((reason as u32) << 4))
    }
    /// Get reboot reason.
    #[inline]
    pub const fn reboot_reason(self) -> u8 {
        ((self.0 & Self::REBOOT_REASON) >> 4) as u8
    }
    /// Set boot device (`BOOT_DEV`).
    #[doc(alias = "BOOT_DEV")]
    #[inline]
    pub const fn set_boot_dev(self, dev: u8) -> Self {
        assert!(dev < 16, "BOOT_DEV out of range (expected 0..=15)");
        Self((self.0 & !Self::BOOT_DEV) | (dev as u32))
    }
    /// Get boot device.
    #[inline]
    pub const fn boot_dev(self) -> u8 {
        (self.0 & Self::BOOT_DEV) as u8
    }
}

/// System backup register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SysBackup(u32);

impl SysBackup {
    const SYS_BAK: u32 = 0xFF;

    /// Set system backup value (`SYS_BAK`).
    #[doc(alias = "SYS_BAK")]
    #[inline]
    pub const fn set_sys_bak(self, value: u8) -> Self {
        Self((self.0 & !Self::SYS_BAK) | (Self::SYS_BAK & (value as u32)))
    }
    /// Get system backup value.
    #[inline]
    pub const fn sys_bak(self) -> u8 {
        (self.0 & Self::SYS_BAK) as u8
    }
}

/// RTC 32K detect register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Detect32k(u32);

impl Detect32k {
    const DET_LEVEL: u32 = 0x3FF << 16;
    const DET_EN: u32 = 0x1;

    /// Set detection level (`DET_LEVEL`).
    #[doc(alias = "DET_LEVEL")]
    #[inline]
    pub const fn set_det_level(self, level: u16) -> Self {
        assert!(
            level < 0x400,
            "Detection level out of range (expected 0..=0x3FF)"
        );
        Self((self.0 & !Self::DET_LEVEL) | ((level as u32) << 16))
    }
    /// Get detection level.
    #[inline]
    pub const fn det_level(self) -> u16 {
        ((self.0 & Self::DET_LEVEL) >> 16) as u16
    }
    /// Enable 32K detection (`DET_EN`).
    #[doc(alias = "DET_EN")]
    #[inline]
    pub const fn enable(self) -> Self {
        Self(self.0 | Self::DET_EN)
    }
    /// Disable 32K detection.
    #[inline]
    pub const fn disable(self) -> Self {
        Self(self.0 & !Self::DET_EN)
    }
    /// Check if 32K detection is enabled.
    #[inline]
    pub const fn is_enabled(self) -> bool {
        (self.0 & Self::DET_EN) != 0
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
        assert_eq!(offset_of!(RegisterBlock, init), 0x4);
        assert_eq!(offset_of!(RegisterBlock, aon_int_en), 0x8);
        assert_eq!(offset_of!(RegisterBlock, aon_int_status), 0xC);
        assert_eq!(offset_of!(RegisterBlock, time), 0x20);
        assert_eq!(offset_of!(RegisterBlock, alarm), 0x30);
        assert_eq!(offset_of!(RegisterBlock, cali0), 0x40);
        assert_eq!(offset_of!(RegisterBlock, cali1), 0x44);
        assert_eq!(offset_of!(RegisterBlock, analog0), 0x50);
        assert_eq!(offset_of!(RegisterBlock, analog1), 0x54);
        assert_eq!(offset_of!(RegisterBlock, analog2), 0x58);
        assert_eq!(offset_of!(RegisterBlock, analog3), 0x5C);
        assert_eq!(offset_of!(RegisterBlock, write_key), 0xFC);
        assert_eq!(offset_of!(RegisterBlock, boot_info), 0x100);
        assert_eq!(offset_of!(RegisterBlock, sys_backup), 0x104);
        assert_eq!(offset_of!(RegisterBlock, tcnt_val), 0x800);
        assert_eq!(offset_of!(RegisterBlock, detect_32k), 0x804);
        assert_eq!(offset_of!(RegisterBlock, version), 0x8FC);
    }

    #[test]
    fn struct_control_functions() {
        let mut val = Control(0x0000_0080);
        assert!(val.rtc_io_level());

        val = Control(0x0).enable_rtc_io_input();
        assert!(val.is_rtc_io_input_enabled());
        assert_eq!(val.0, 0x0000_0040);
        val = val.disable_rtc_io_input();
        assert!(!val.is_rtc_io_input_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_rtc_io_output_selection(IoOutputSel::ClockOutputs);
        assert_eq!(val.rtc_io_output_selection(), IoOutputSel::ClockOutputs);
        assert_eq!(val.0, 0x0000_0030);
        val = val.set_rtc_io_output_selection(IoOutputSel::AlarmTrigger);
        assert_eq!(val.rtc_io_output_selection(), IoOutputSel::AlarmTrigger);
        assert_eq!(val.0, 0x0000_0020);
        val = val.set_rtc_io_output_selection(IoOutputSel::ActiveLow);
        assert_eq!(val.rtc_io_output_selection(), IoOutputSel::ActiveLow);
        assert_eq!(val.0, 0x0000_0010);
        val = val.set_rtc_io_output_selection(IoOutputSel::Disabled);
        assert_eq!(val.rtc_io_output_selection(), IoOutputSel::Disabled);
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_alarm();
        assert!(val.is_alarm_enabled());
        assert_eq!(val.0, 0x0000_0004);
        val = val.disable_alarm();
        assert!(!val.is_alarm_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_time_cnt();
        assert!(val.is_time_cnt_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_time_cnt();
        assert!(!val.is_time_cnt_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_init_functions() {
        let mut val = Init(0x0).set_time_cnt_init(true);
        assert!(val.time_cnt_init());
        assert_eq!(val.0, 0x0000_0001);
        val = val.set_time_cnt_init(false);
        assert!(!val.time_cnt_init());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_aon_irq_enable_functions() {
        let mut val = AonIrqEnable(0x0);
        val = val.enable_rtc_32k_err_irq();
        assert!(val.is_rtc_32k_err_irq_enabled());
        assert_eq!(val.0, 0x0000_0004);
        val = val.disable_rtc_32k_err_irq();
        assert!(!val.is_rtc_32k_err_irq_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_alarm_irq();
        assert!(val.is_alarm_irq_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_alarm_irq();
        assert!(!val.is_alarm_irq_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_aon_irq_status_functions() {
        let mut val = AonIrqStatus(0x0);
        val = val.clear_rtc_32k_err_irq_status();
        assert!(val.rtc_32k_err_irq_status());
        assert_eq!(val.0, 0x0000_0004);

        val = AonIrqStatus(0x0).clear_rtc_io_enable_status();
        assert!(val.rtc_io_enable_status());
        assert_eq!(val.0, 0x0000_0002);

        val = AonIrqStatus(0x0).clear_alarm_irq_status();
        assert!(val.alarm_irq_status());
        assert_eq!(val.0, 0x0000_0001);
    }

    #[test]
    fn struct_time_functions() {
        let val = Time(0x0).set_time(0xFF);
        assert_eq!(val.time(), 0xFF);
        assert_eq!(val.0, 0x0000_00FF);
    }

    #[test]
    fn struct_alarm_functions() {
        let val = Alarm(0x0).set_alarm(0xFF);
        assert_eq!(val.alarm(), 0xFF);
        assert_eq!(val.0, 0x0000_00FF);
    }

    #[test]
    fn struct_calibration0_functions() {
        let val = Calibration0(0x0).set_cali_value_low(0xFF);
        assert_eq!(val.cali_value_low(), 0xFF);
        assert_eq!(val.0, 0x0000_00FF);
    }

    #[test]
    fn struct_calibration1_functions() {
        let mut val = Calibration1(0x0);

        val = val.set_cali_direction(CaliDirection::Increase);
        assert_eq!(val.cali_direction(), CaliDirection::Increase);
        assert_eq!(val.0, 0x0000_0080);
        val = val.set_cali_direction(CaliDirection::Decrease);
        assert_eq!(val.cali_direction(), CaliDirection::Decrease);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_cali_value_high(0x3);
        assert_eq!(val.cali_value_high(), 0x3);
        assert_eq!(val.0, 0x0000_0003);
    }

    test_should_panic!((
        test_calibration1_set_cali_value_high_panic,
        Calibration1(0x0).set_cali_value_high(0x4),
        "Calibration value out of range (expected 0..=3)"
    ),);

    #[test]
    fn struct_analog0_functions() {
        let mut val = Analog0(0x0);

        val = val.set_rc1m_isel(Rc1mIsel::BG);
        assert_eq!(val.rc1m_isel(), Rc1mIsel::BG);
        assert_eq!(val.0, 0x0000_0080);
        val = val.set_rc1m_isel(Rc1mIsel::IBIAS);
        assert_eq!(val.rc1m_isel(), Rc1mIsel::IBIAS);
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_rc1m();
        assert!(val.is_rc1m_enabled());
        assert_eq!(val.0, 0x0000_0040);
        val = val.disable_rc1m();
        assert!(!val.is_rc1m_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_ldo18_bypass();
        assert!(val.is_ldo18_bypass_enabled());
        assert_eq!(val.0, 0x0000_0010);
        val = val.disable_ldo18_bypass();
        assert!(!val.is_ldo18_bypass_enabled());
        assert_eq!(val.0, 0x0000_0000);

        for i in 0..8 {
            let (volatge, reg_val) = match i {
                0 => (Ldo1P8Vol::V1_9, 0x0000_0000),
                1 => (Ldo1P8Vol::V1_8, 0x0000_0002),
                2 => (Ldo1P8Vol::V1_7, 0x0000_0004),
                3 => (Ldo1P8Vol::V1_6, 0x0000_0006),
                4 => (Ldo1P8Vol::V1_5, 0x0000_0008),
                5 => (Ldo1P8Vol::V1_4, 0x0000_000A),
                6 => (Ldo1P8Vol::V1_3, 0x0000_000C),
                7 => (Ldo1P8Vol::V1_2, 0x0000_000E),
                _ => unreachable!(),
            };

            val = val.set_ldo18_vol(volatge);
            assert_eq!(val.ldo18_vol(), volatge);
            assert_eq!(val.0, reg_val);
        }

        val = Analog0(0x0).enable_ldo18();
        assert!(val.is_ldo18_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_ldo18();
        assert!(!val.is_ldo18_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_analog1_functions() {
        let mut val = Analog1(0x0);

        val = val.set_pd_cur_sel(Ldo1P1CurSel::I3_0uA);
        assert_eq!(val.pd_cur_sel(), Ldo1P1CurSel::I3_0uA);
        assert_eq!(val.0, 0x0000_0060);
        val = val.set_pd_cur_sel(Ldo1P1CurSel::I2_0uA);
        assert_eq!(val.pd_cur_sel(), Ldo1P1CurSel::I2_0uA);
        assert_eq!(val.0, 0x0000_0040);
        val = val.set_pd_cur_sel(Ldo1P1CurSel::I1_0uA);
        assert_eq!(val.pd_cur_sel(), Ldo1P1CurSel::I1_0uA);
        assert_eq!(val.0, 0x0000_0020);
        val = val.set_pd_cur_sel(Ldo1P1CurSel::I0_5uA);
        assert_eq!(val.pd_cur_sel(), Ldo1P1CurSel::I0_5uA);
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_ldo11();
        assert!(val.is_ldo11_enabled());
        assert_eq!(val.0, 0x0000_0010);
        val = val.disable_ldo11();
        assert!(!val.is_ldo11_enabled());
        assert_eq!(val.0, 0x0000_0000);

        for i in 0..7 {
            let (voltage, reg_val) = match i {
                0 => (Ldo1P1Vol::V1_10, 0x0000_0000),
                1 => (Ldo1P1Vol::V1_05, 0x0000_0002),
                2 => (Ldo1P1Vol::V1_00, 0x0000_0004),
                3 => (Ldo1P1Vol::V0_95, 0x0000_0006),
                4 => (Ldo1P1Vol::V0_90, 0x0000_0008),
                5 => (Ldo1P1Vol::V0_85, 0x0000_000A),
                6 => (Ldo1P1Vol::V0_80, 0x0000_000C),
                _ => unreachable!(),
            };

            val = val.set_ldo11_vol(voltage);
            assert_eq!(val.ldo11_vol(), voltage);
            assert_eq!(val.0, reg_val);
        }

        val = Analog1(0x0).enable_ldo11_lp();
        assert!(val.is_ldo11_lp_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_ldo11_lp();
        assert!(!val.is_ldo11_lp_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_analog2_functions() {
        let mut val = Analog2(0x0);

        val = val.set_atb_sel(AtbSel::IbgVdet);
        assert_eq!(val.atb_sel(), AtbSel::IbgVdet);
        assert_eq!(val.0, 0x0000_0060);
        val = val.set_atb_sel(AtbSel::IbgIbi);
        assert_eq!(val.atb_sel(), AtbSel::IbgIbi);
        assert_eq!(val.0, 0x0000_0040);
        val = val.set_atb_sel(AtbSel::Vosc);
        assert_eq!(val.atb_sel(), AtbSel::Vosc);
        assert_eq!(val.0, 0x0000_0020);
        val = val.set_atb_sel(AtbSel::Vref);
        assert_eq!(val.atb_sel(), AtbSel::Vref);
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_atb();
        assert!(val.is_atb_enabled());
        assert_eq!(val.0, 0x0000_0010);
        val = val.disable_atb();
        assert!(!val.is_atb_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_xtal32k_strength_up();
        assert!(val.is_xtal32k_strength_up_enabled());
        assert_eq!(val.0, 0x0000_0008);
        val = val.disable_xtal32k_strength_up();
        assert!(!val.is_xtal32k_strength_up_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_xtal32k_drv(3);
        assert_eq!(val.0, 0x0000_0003);
    }

    test_should_panic!((
        test_analog2_set_xtal32k_drv_panic,
        Analog2(0x0).set_xtal32k_drv(4),
        "XTAL32K_DRV out of range (expected 0..=3)"
    ),);

    #[test]
    fn struct_analog3_functions() {
        let mut val = Analog3(0x0);

        val = val.enable_ldo12_xtal32k_sw();
        assert!(val.is_ldo12_xtal32k_sw_enabled());
        assert_eq!(val.0, 0x0000_0002);
        val = val.disable_ldo12_xtal32k_sw();
        assert!(!val.is_ldo12_xtal32k_sw_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_xtal32k();
        assert!(val.is_xtal32k_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_xtal32k();
        assert!(!val.is_xtal32k_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_write_key_functions() {
        let val = WriteKey(0x0).set_write_key(0xAC);
        assert_eq!(val.write_key(), 0xAC);
        assert_eq!(val.0, 0x0000_00AC);
    }

    #[test]
    fn struct_boot_info_functions() {
        let mut val = BootInfo(0x0);

        val = val.set_reboot_reason(0xF);
        assert_eq!(val.reboot_reason(), 0xF);
        assert_eq!(val.0, 0x0000_00F0);
        val = val.set_reboot_reason(0);
        assert_eq!(val.reboot_reason(), 0);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_boot_dev(0xF);
        assert_eq!(val.boot_dev(), 0xF);
        assert_eq!(val.0, 0x0000_000F);
        val = val.set_boot_dev(0);
        assert_eq!(val.boot_dev(), 0);
        assert_eq!(val.0, 0x0000_0000);
    }
    test_should_panic!(
        (
            test_boot_info_set_reboot_reason_panic,
            BootInfo(0x0).set_reboot_reason(16),
            "REBOOT_REASON out of range (expected 0..=15)"
        ),
        (
            test_boot_info_set_boot_dev_panic,
            BootInfo(0x0).set_boot_dev(16),
            "BOOT_DEV out of range (expected 0..=15)"
        ),
    );

    #[test]
    fn struct_sys_backup_functions() {
        let val = SysBackup(0x0).set_sys_bak(0xFF);
        assert_eq!(val.sys_bak(), 0xFF);
        assert_eq!(val.0, 0x0000_00FF);
    }

    #[test]
    fn struct_detect32k_functions() {
        let mut val = Detect32k(0x0);

        val = val.set_det_level(0x3FF);
        assert_eq!(val.det_level(), 0x3FF);
        assert_eq!(val.0, 0x03FF_0000);
        val = val.set_det_level(0);
        assert_eq!(val.det_level(), 0);
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable();
        assert!(val.is_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable();
        assert!(!val.is_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    test_should_panic!((
        test_detect32k_set_det_level_panic,
        Detect32k(0x0).set_det_level(0x400),
        "Detection level out of range (expected 0..=0x3FF)"
    ),);
}
