//! Watch Dog register blocks and registers.

use volatile_register::{RO, RW};

/// WDOG Register Block.
#[repr(C)]
pub struct RegisterBlock {
    /// Control register (`WDOG_CTL`).
    #[doc(alias = "WDOG_CTL")]
    pub ctrl: RW<Control>,
    /// Counter register (`WDOG_CNT`).
    #[doc(alias = "WDOG_CNT")]
    pub cnt: RW<u32>,
    /// Interrupt enable register (`IRQ_EN`).
    #[doc(alias = "IRQ_EN")]
    pub int_enable: RW<IntEnable>,
    /// Interrupt status register (`IRQ_STA`).
    #[doc(alias = "IRQ_STA")]
    pub int_status: RW<IntStatus>,
    _reserved0: [u8; 0x30],
    ///  Threshold registers.
    pub thres: [Thresholds; 4],
    _reserved1: [u8; 0x68],
    /// Operation register (`WDOG_OP`).
    #[doc(alias = "WDOG_OP")]
    pub op: RW<u32>,
    _reserved2: [u8; 0xF0F],
    /// Version register (`WDOG_VER`).
    #[doc(alias = "WDOG_VER")]
    pub version: RO<u32>,
}

/// Threshold registers.
#[repr(C)]
pub struct Thresholds {
    /// Clear threshold register (`CLR_THD`).
    #[doc(alias = "CLR_THD")]
    pub clr_thres: RW<u32>,
    /// Interrupt request threshold register (`IRQ_THD`).
    #[doc(alias = "IRQ_THD")]
    pub int_thres: RW<u32>,
    /// Reset threshold register (`RST_THD`).
    #[doc(alias = "RST_THD")]
    pub rst_thres: RW<u32>,
    _reserved0: [u8; 0x4],
}

/// Register write mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegWrMode {
    /// Write enable.
    WriteEnable,
    /// Write protect.
    /// - Write permission can be restored through `OP_WR_EN`.
    /// - Except for the OP register, other registers are read-only.
    /// - Executing OP write enable will clear this register.
    WriteProtect,
    /// Write disable.
    /// - Except for the OP register, other registers are read-only.
    /// - Registers can only be cleared and made writable again through a reset.
    WriteDisable = 3,
}

/// Control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Control(u32);

impl Control {
    const REG_WR_DIS: u32 = 0x3 << 28;
    const CFG_ID: u32 = 0x3 << 24;
    const DBG_CNT_CONTINUE: u32 = 0x1 << 1;
    const CNT_EN: u32 = 0x1;

    /// Set register write mode (`REG_WR_DIS`).
    #[doc(alias = "REG_WR_DIS")]
    #[inline]
    pub const fn set_reg_wr_mode(self, mode: RegWrMode) -> Self {
        Self((self.0 & !Self::REG_WR_DIS) | (Self::REG_WR_DIS & (mode as u32) << 28))
    }
    /// Get register write mode.
    #[inline]
    pub const fn reg_wr_mode(self) -> RegWrMode {
        match (self.0 & Self::REG_WR_DIS) >> 28 {
            0 => RegWrMode::WriteEnable,
            1 | 2 => RegWrMode::WriteProtect,
            3 => RegWrMode::WriteDisable,
            _ => unreachable!(),
        }
    }
    /// Get configuration ID (`CFG_ID`).
    #[doc(alias = "CFG_ID")]
    #[inline]
    pub const fn cfg_id(self) -> u8 {
        ((self.0 & Self::CFG_ID) >> 24) as u8
    }
    /// Enable debug counter continue (`DBG_CNT_CONTINUE`).
    #[doc(alias = "DBG_CNT_CONTINUE")]
    #[inline]
    pub const fn enable_dbg_cnt_continue(self) -> Self {
        Self(self.0 | Self::DBG_CNT_CONTINUE)
    }
    /// Disable debug counter continue.
    #[inline]
    pub const fn disable_dbg_cnt_continue(self) -> Self {
        Self(self.0 & !Self::DBG_CNT_CONTINUE)
    }
    /// Check if debug counter continue is enabled.
    #[inline]
    pub const fn is_dbg_cnt_continue_enabled(self) -> bool {
        (self.0 & Self::DBG_CNT_CONTINUE) != 0
    }
    /// Enable counter (`CNT_EN`).
    #[doc(alias = "CNT_EN")]
    #[inline]
    pub const fn enable_cnt(self) -> Self {
        Self(self.0 | Self::CNT_EN)
    }
    /// Disable counter.
    #[inline]
    pub const fn disable_cnt(self) -> Self {
        Self(self.0 & !Self::CNT_EN)
    }
    /// Check if counter is enabled.
    #[inline]
    pub const fn is_cnt_enabled(self) -> bool {
        (self.0 & Self::CNT_EN) != 0
    }
}

/// Interrupt enable register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IntEnable(u32);

impl IntEnable {
    const TIMEOUT_IRQ_EN: u32 = 0x1;

    /// Enable timeout interrupt (`TIMEOUT_IRQ_EN`).
    #[doc(alias = "TIMEOUT_IRQ_EN")]
    #[inline]
    pub const fn enable_tmo_int(self) -> Self {
        Self(self.0 | Self::TIMEOUT_IRQ_EN)
    }
    /// Disable timeout interrupt.
    #[inline]
    pub const fn disable_tmo_int(self) -> Self {
        Self(self.0 & !Self::TIMEOUT_IRQ_EN)
    }
    /// Check if timeout interrupt is enabled.
    #[inline]
    pub const fn is_tmo_int_enabled(self) -> bool {
        (self.0 & Self::TIMEOUT_IRQ_EN) != 0
    }
}

/// Interrupt status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IntStatus(u32);

impl IntStatus {
    const TIMEOUT_IRQ_STA: u32 = 0x1;

    /// Check if timeout interrupt is pending (`TIMEOUT_IRQ_STA`).
    #[doc(alias = "TIMEOUT_IRQ_STA")]
    #[inline]
    pub const fn is_tmo_int_pending(self) -> bool {
        (self.0 & Self::TIMEOUT_IRQ_STA) != 0
    }
    /// Clear timeout interrupt status.
    #[inline]
    pub const fn clear_tmo_int_pending(self) -> Self {
        Self(self.0 | Self::TIMEOUT_IRQ_STA)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, ctrl), 0x0);
        assert_eq!(offset_of!(RegisterBlock, cnt), 0x4);
        assert_eq!(offset_of!(RegisterBlock, int_enable), 0x8);
        assert_eq!(offset_of!(RegisterBlock, int_status), 0xC);
        assert_eq!(offset_of!(RegisterBlock, thres), 0x40);
        assert_eq!(offset_of!(RegisterBlock, op), 0xE8);
        assert_eq!(offset_of!(RegisterBlock, version), 0xFFC);
    }

    #[test]
    fn struct_thresholds_offset() {
        assert_eq!(offset_of!(Thresholds, clr_thres), 0x0);
        assert_eq!(offset_of!(Thresholds, int_thres), 0x4);
        assert_eq!(offset_of!(Thresholds, rst_thres), 0x8);
        assert_eq!(size_of::<Thresholds>(), 0x10);
    }

    #[test]
    fn struct_control_functions() {
        let mut val = Control(0x0).set_reg_wr_mode(RegWrMode::WriteDisable);
        assert_eq!(val.reg_wr_mode(), RegWrMode::WriteDisable);
        assert_eq!(val.0, 0x3000_0000);
        val = val.set_reg_wr_mode(RegWrMode::WriteProtect);
        assert_eq!(val.reg_wr_mode(), RegWrMode::WriteProtect);
        assert_eq!(val.0, 0x1000_0000);
        val = val.set_reg_wr_mode(RegWrMode::WriteEnable);
        assert_eq!(val.reg_wr_mode(), RegWrMode::WriteEnable);
        assert_eq!(val.0, 0x0000_0000);

        val = Control(0x0300_0000);
        assert_eq!(val.cfg_id(), 0x3);

        val = Control(0x0).enable_dbg_cnt_continue();
        assert!(val.is_dbg_cnt_continue_enabled());
        assert_eq!(val.0, 0x0000_0002);
        val = val.disable_dbg_cnt_continue();
        assert!(!val.is_dbg_cnt_continue_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_cnt();
        assert!(val.is_cnt_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_cnt();
        assert!(!val.is_cnt_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_int_enable_functions() {
        let mut val = IntEnable(0x0).enable_tmo_int();
        assert!(val.is_tmo_int_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_tmo_int();
        assert!(!val.is_tmo_int_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_int_status_functions() {
        let val = IntStatus(0x0).clear_tmo_int_pending();
        assert!(val.is_tmo_int_pending());
        assert_eq!(val.0, 0x0000_0001);
    }
}
