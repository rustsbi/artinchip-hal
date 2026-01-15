//! SID register blocks and registers.

use volatile_register::{RO, RW};

/// Secure ID (SID) registers.
#[repr(C)]
pub struct RegisterBlock {
    /// EFUSE control register (`EFUSE_CTL`).
    #[doc(alias = "EFUSE_CTL")]
    pub ctrl: RW<Control>,
    /// EFUSE address register (`EFUSE_ADDR`).
    #[doc(alias = "EFUSE_ADDR")]
    pub addr: RW<Address>,
    /// EFUSE write data register (`EFUSE_WDATA`).
    #[doc(alias = "EFUSE_WDATA")]
    pub wdata: RW<u32>,
    /// EFUSE read data register (`EFUSE_RDATA`).
    #[doc(alias = "EFUSE_RDATA")]
    pub rdata: RO<u32>,
    /// EFUSE timing low register (`EFUSE_TIMING_LOW`).
    #[doc(alias = "EFUSE_TIMING_LOW")]
    pub timing_low: RW<TimingLow>,
    _reserved0: [u8; 0x6C],
    /// BROM privilege register (`BROM_PRIV`).
    #[doc(alias = "BROM_PRIV")]
    pub brom_priv: RW<BromPriv>,
    _reserved1: [u8; 0x78],
    /// SID version register (`SID_VER`).
    #[doc(alias = "SID_VER")]
    pub version: RO<u32>,
    _reserved2: [u8; 0x100],
    /// EFUSE data buffer register (`EFUSE_BUFFER`).
    #[doc(alias = "EFUSE_BUFFER")]
    pub buffer: [RW<u32>; 64],
}

/// Efuse state machine status.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EfuseStatus {
    /// Boot auto-reading in progress.
    BootReading = 1,
    /// Idle state.
    Idle = 2,
    /// Programming/burning in progress.
    Programming = 4,
    /// Reading in progress.
    Reading = 8,
}

/// EFUSE control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Control(u32);

impl Control {
    const SID_ERROR: u32 = 0x1 << 29;
    const BROM_PRIV_LOCK: u32 = 0x1 << 28;
    const EFUSE_OP_CODE: u32 = 0xFFF << 16;
    const EFUSE_STATUS: u32 = 0x1F << 8;
    const EFUSE_READ_START: u32 = 0x1 << 4;
    const EFUSE_WRITE_START: u32 = 0x1;

    /// Set the sid error bit (`SID_ERROR`).
    ///
    /// - 0: Normal.
    /// - 1: Start reading and writing when busy.
    #[doc(alias = "SID_ERROR")]
    #[inline]
    pub const fn set_sid_error(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::SID_ERROR)
        } else {
            Self(self.0 & !Self::SID_ERROR)
        }
    }
    /// Get the sid error bit.
    #[inline]
    pub const fn sid_error(self) -> bool {
        (self.0 & Self::SID_ERROR) != 0
    }
    /// Set the brom privilege lock bit (`BROM_PRIV_LOCK`).
    ///
    /// - 0: BROM privileges are available, and the software can use the related privileged functions.
    /// - 1: BROM privileges are disabled, and the software cannot use the related privileged functions.
    ///
    /// Once set to 1, it cannot be reset to 0 before the next POR.
    ///
    /// BROM privilege lock privileges include:
    /// - The JTAG unlocking function can be enabled.
    /// - The CE computation result of the eFuse SSK can be output to SRAM.
    ///
    /// This bit is configured as 1 in BROM to ensure that the related privileged functions are only available in BROM and cannot be modified by other software.
    #[doc(alias = "BROM_PRIV_LOCK")]
    #[inline]
    pub const fn set_brom_priv_lock(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::BROM_PRIV_LOCK)
        } else {
            Self(self.0 & !Self::BROM_PRIV_LOCK)
        }
    }
    /// Get the brom privilege lock bit.
    #[inline]
    pub const fn brom_priv_lock(self) -> bool {
        (self.0 & Self::BROM_PRIV_LOCK) != 0
    }
    /// Set the efuse operation code (`EFUSE_OP_CODE`).
    ///
    /// To prevent accidental operations, set it to write 0xA1C before allowing eFuse to perform read and write operations.
    #[inline]
    pub const fn set_efuse_op_code(self, code: u16) -> Self {
        assert!(
            code < 0x1000,
            "EFUSE_OP_CODE out of range (expected 0..=0xFFF)"
        );
        Self((self.0 & !Self::EFUSE_OP_CODE) | (Self::EFUSE_OP_CODE & ((code as u32) << 16)))
    }
    /// Get the efuse operation code.
    #[inline]
    pub const fn efuse_op_code(self) -> u16 {
        ((self.0 & Self::EFUSE_OP_CODE) >> 16) as u16
    }
    /// Get the efuse state machine status (`EFUSE_STATUS`).
    ///
    /// - It is recommended to check EFUSE_DATA_READY when in idle state.
    /// - It is recommended to check EFUSE_WRITE_START when the write operation is completed.
    /// - It is recommended to check EFUSE_READ_START when the read operation is completed.
    #[doc(alias = "EFUSE_STATUS")]
    #[inline]
    pub const fn efuse_status(self) -> EfuseStatus {
        match (self.0 & Self::EFUSE_STATUS) >> 8 {
            1 => EfuseStatus::BootReading,
            2 => EfuseStatus::Idle,
            4 => EfuseStatus::Programming,
            8 => EfuseStatus::Reading,
            _ => unreachable!(),
        }
    }
    /// Set the efuse read start bit (`EFUSE_READ_START`).
    ///
    /// - 0: No operation
    /// - 1: Start eFuse read operation
    ///
    ///  When writing to this bit, it is written simultaneously with EFUSE_OP_CODE = 0xA1C.
    ///  Writing 1 initiates the eFuse read, and this bit automatically clears once the read is complete.
    #[doc(alias = "EFUSE_READ_START")]
    #[inline]
    pub const fn set_efuse_read_start(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::EFUSE_READ_START)
        } else {
            Self(self.0 & !Self::EFUSE_READ_START)
        }
    }
    /// Get the efuse read start bit.
    #[inline]
    pub const fn efuse_read_start(self) -> bool {
        (self.0 & Self::EFUSE_READ_START) != 0
    }
    /// Set the efuse write start bit (`EFUSE_WRITE_START`).
    ///
    /// - 0: No operation
    /// - 1: Start eFuse write operation
    ///
    ///  When writing to this bit, it is written simultaneously with EFUSE_OP_CODE = 0xA1C.
    ///  Writing 1 initiates the eFuse write, and this bit automatically clears once the write is complete.
    ///  The LDO will automatically turn on and off during this period, and there is no need to configure the LDO switch.
    #[doc(alias = "EFUSE_WRITE_START")]
    #[inline]
    pub const fn set_efuse_write_start(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::EFUSE_WRITE_START)
        } else {
            Self(self.0 & !Self::EFUSE_WRITE_START)
        }
    }
    /// Get the efuse write start bit.
    #[inline]
    pub const fn efuse_write_start(self) -> bool {
        (self.0 & Self::EFUSE_WRITE_START) != 0
    }
}

/// EFUSE address register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Address(u32);

impl Address {
    const EFUSE_ADDR: u32 = 0x1FF;

    /// Set the efuse address (`EFUSE_ADDR`).
    ///
    /// Read and write operations cover the entire space in byte units.
    /// Since the operations are performed in 32-bit increments, the lowest two bits need to be set to 0.
    #[doc(alias = "EFUSE_ADDR")]
    #[inline]
    pub const fn set_efuse_addr(self, addr: u16) -> Self {
        assert!(addr < 0x200, "EFUSE_ADDR out of range (expected 0..=0x1FF)");
        Self(Self::EFUSE_ADDR & (addr as u32))
    }
    /// Get the efuse address.
    #[inline]
    pub const fn efuse_addr(self) -> u16 {
        (self.0 & Self::EFUSE_ADDR) as u16
    }
}

/// EFUSE timing low register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TimingLow(u32);

impl TimingLow {
    const TAEN_RD: u32 = 0xFF << 24;
    const TRD: u32 = 0xFF << 16;
    const TAEN_PGM: u32 = 0xFF << 8;
    const TPGM: u32 = 0xFF;

    /// Set the read data processing time (`TAEN_RD`).
    #[doc(alias = "TAEN_RD")]
    #[inline]
    pub const fn set_rd_data_proc_time(self, cycles: u8) -> Self {
        Self((self.0 & !Self::TAEN_RD) | (Self::TAEN_RD & ((cycles as u32) << 24)))
    }
    /// Get the read data processing time.
    #[inline]
    pub const fn rd_data_proc_time(self) -> u8 {
        ((self.0 & Self::TAEN_RD) >> 24) as u8
    }
    /// Set the read data high level duration (`TRD`).
    #[doc(alias = "TRD")]
    #[inline]
    pub const fn set_rd_high_level_duration(self, cycles: u8) -> Self {
        Self((self.0 & !Self::TRD) | (Self::TRD & ((cycles as u32) << 16)))
    }
    /// Get the read data high level duration.
    #[inline]
    pub const fn rd_high_level_duration(self) -> u8 {
        ((self.0 & Self::TRD) >> 16) as u8
    }
    /// Set the write data processing time (`TAEN_PGM`).
    #[doc(alias = "TAEN_PGM")]
    #[inline]
    pub const fn set_wr_data_proc_time(self, cycles: u8) -> Self {
        Self((self.0 & !Self::TAEN_PGM) | (Self::TAEN_PGM & ((cycles as u32) << 8)))
    }
    /// Get the write data processing time.
    #[inline]
    pub const fn wr_data_proc_time(self) -> u8 {
        ((self.0 & Self::TAEN_PGM) >> 8) as u8
    }
    /// Set the write data high level duration (`TPGM`).
    #[doc(alias = "TPGM")]
    #[inline]
    pub const fn set_wr_high_level_duration(self, cycles: u8) -> Self {
        Self((self.0 & !Self::TPGM) | (Self::TPGM & (cycles as u32)))
    }
    /// Get the write data high level duration.
    #[inline]
    pub const fn wr_high_level_duration(self) -> u8 {
        (self.0 & Self::TPGM) as u8
    }
}

/// BROM privilege register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BromPriv(u32);

impl BromPriv {
    const JTAG_UNLOCK: u32 = 0x1;

    /// Enable jtag unlock (`JTAG_UNLOCK`).
    ///
    /// Effective only when JTAG_LOCK = 1.
    ///
    /// Has a higher priority than JTAG_LOCK (from eFuse); the unlock function is only effective when BROM_PRIVILEGE_LOCK is 0.
    ///
    /// If BROM_PRIVILEGE_LOCK is 1, JTAG unlocking will not be enabled even if JTAG_UNLOCK is set to 1.
    #[doc(alias = "JTAG_UNLOCK")]
    #[inline]
    pub const fn enable_jtag_unlock(self) -> Self {
        Self(self.0 | Self::JTAG_UNLOCK)
    }
    /// Disable jtag unlock.
    #[inline]
    pub const fn disable_jtag_unlock(self) -> Self {
        Self(self.0 & !Self::JTAG_UNLOCK)
    }
    /// Check if jtag unlock is enabled.
    #[inline]
    pub const fn is_jtag_unlock_enabled(self) -> bool {
        (self.0 & Self::JTAG_UNLOCK) != 0
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
        assert_eq!(offset_of!(RegisterBlock, addr), 0x4);
        assert_eq!(offset_of!(RegisterBlock, wdata), 0x8);
        assert_eq!(offset_of!(RegisterBlock, rdata), 0xC);
        assert_eq!(offset_of!(RegisterBlock, timing_low), 0x10);
        assert_eq!(offset_of!(RegisterBlock, brom_priv), 0x80);
        assert_eq!(offset_of!(RegisterBlock, version), 0xFC);
        assert_eq!(offset_of!(RegisterBlock, buffer), 0x200);
    }

    #[test]
    fn struct_control_functions() {
        let mut val = Control(0x0);

        val = val.set_sid_error(true);
        assert!(val.sid_error());
        assert_eq!(val.0, 0x2000_0000);
        val = val.set_sid_error(false);
        assert!(!val.sid_error());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_brom_priv_lock(true);
        assert!(val.brom_priv_lock());
        assert_eq!(val.0, 0x1000_0000);
        val = val.set_brom_priv_lock(false);
        assert!(!val.brom_priv_lock());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_efuse_op_code(0xFFF);
        assert_eq!(val.efuse_op_code(), 0xFFF);
        assert_eq!(val.0, 0x0FFF_0000);

        val = Control(0x0000_0100);
        assert_eq!(val.efuse_status(), EfuseStatus::BootReading);
        val = Control(0x0000_0200);
        assert_eq!(val.efuse_status(), EfuseStatus::Idle);
        val = Control(0x0000_0400);
        assert_eq!(val.efuse_status(), EfuseStatus::Programming);
        val = Control(0x0000_0800);
        assert_eq!(val.efuse_status(), EfuseStatus::Reading);

        val = Control(0x0).set_efuse_read_start(true);
        assert!(val.efuse_read_start());
        assert_eq!(val.0, 0x0000_0010);
        val = val.set_efuse_read_start(false);
        assert!(!val.efuse_read_start());
        assert_eq!(val.0, 0x0000_0000);

        val = Control(0x0).set_efuse_write_start(true);
        assert!(val.efuse_write_start());
        assert_eq!(val.0, 0x0000_0001);
        val = val.set_efuse_write_start(false);
        assert!(!val.efuse_write_start());
        assert_eq!(val.0, 0x0000_0000);
    }

    test_should_panic!((
        test_control_set_efuse_op_panic,
        Control(0x0).set_efuse_op_code(0x1000),
        "EFUSE_OP_CODE out of range (expected 0..=0xFFF)"
    ),);

    #[test]
    fn struct_address_functions() {
        let val = Address(0x0).set_efuse_addr(0x1FF);
        assert_eq!(val.efuse_addr(), 0x1FF);
        assert_eq!(val.0, 0x0000_01FF);
    }

    test_should_panic!((
        test_address_set_efuse_addr_panic,
        Address(0x0).set_efuse_addr(0x200),
        "EFUSE_ADDR out of range (expected 0..=0x1FF)"
    ),);

    #[test]
    fn struct_timing_low_functions() {
        let mut val = TimingLow(0x0);

        val = val.set_rd_data_proc_time(0xFF);
        assert_eq!(val.rd_data_proc_time(), 0xFF);
        assert_eq!(val.0, 0xFF00_0000);

        val = val.set_rd_high_level_duration(0xFF);
        assert_eq!(val.rd_high_level_duration(), 0xFF);
        assert_eq!(val.0, 0xFFFF_0000);

        val = val.set_wr_data_proc_time(0xFF);
        assert_eq!(val.wr_data_proc_time(), 0xFF);
        assert_eq!(val.0, 0xFFFF_FF00);

        val = val.set_wr_high_level_duration(0xFF);
        assert_eq!(val.wr_high_level_duration(), 0xFF);
        assert_eq!(val.0, 0xFFFF_FFFF);
    }

    #[test]
    fn struct_brom_priv_functions() {
        let mut val = BromPriv(0x0);

        val = val.enable_jtag_unlock();
        assert!(val.is_jtag_unlock_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_jtag_unlock();
        assert!(!val.is_jtag_unlock_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }
}
