//! Warm Reset Information register blocks and registers.

use volatile_register::{RO, RW};

/// WRI configuration register block.
#[repr(C)]
pub struct RegisterBlock {
    /// Reset flag register (`RST_FLAG`).
    #[doc(alias = "RST_FLAG")]
    pub reset_flag: RW<u32>,
    _reserved0: [u8; 0xFF8],
    /// WRI version register (`WRI_VER`).
    #[doc(alias = "WRI_VER")]
    pub version: RO<u32>,
}

/// Reset flag register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct RstFlag(u32);

impl RstFlag {
    const CMP_RST_FLAG: u32 = 0x1 << 12;
    const THS_RST_FLAG: u32 = 0x1 << 11;
    const WDOG_RST_FLAG: u32 = 0x1 << 10;
    const CPU_DM_RST_FLAG: u32 = 0x1 << 9;
    const PIN_RST_FLAG: u32 = 0x1 << 8;
    const RTC_POR_FLAG: u32 = 0x1 << 1;
    const SYS_POR_FLAG: u32 = 0x1;

    /// Check if voltage comparator reset flag is set (`CMP_RST_FLAG`).
    #[doc(alias = "CMP_RST_FLAG")]
    #[inline]
    pub const fn cmp_rst_flag(self) -> bool {
        (self.0 & Self::CMP_RST_FLAG) != 0
    }
    /// Clear voltage comparator reset flag.
    #[inline]
    pub const fn clear_cmp_rst_flag(self) -> Self {
        Self(self.0 | Self::CMP_RST_FLAG)
    }
    /// Check if thermal shutdown reset flag is set (`THS_RST_FLAG`).
    #[doc(alias = "THS_RST_FLAG")]
    #[inline]
    pub const fn ths_rst_flag(self) -> bool {
        (self.0 & Self::THS_RST_FLAG) != 0
    }
    /// Clear thermal shutdown reset flag.
    #[inline]
    pub const fn clear_ths_rst_flag(self) -> Self {
        Self(self.0 | Self::THS_RST_FLAG)
    }
    /// Check if watchdog reset flag is set (`WDOG_RST_FLAG`).
    #[doc(alias = "WDOG_RST_FLAG")]
    #[inline]
    pub const fn wdog_rst_flag(self) -> bool {
        (self.0 & Self::WDOG_RST_FLAG) != 0
    }
    /// Clear watchdog reset flag.
    #[inline]
    pub const fn clear_wdog_rst_flag(self) -> Self {
        Self(self.0 | Self::WDOG_RST_FLAG)
    }
    /// Check if CPU debug module reset flag is set (`CPU_DM_RST_FLAG`).
    #[doc(alias = "CPU_DM_RST_FLAG")]
    #[inline]
    pub const fn cpu_dm_rst_flag(self) -> bool {
        (self.0 & Self::CPU_DM_RST_FLAG) != 0
    }
    /// Clear CPU debug module reset flag.
    #[inline]
    pub const fn clear_cpu_dm_rst_flag(self) -> Self {
        Self(self.0 | Self::CPU_DM_RST_FLAG)
    }
    /// Check if pin reset flag is set (`PIN_RST_FLAG`).
    #[doc(alias = "PIN_RST_FLAG")]
    #[inline]
    pub const fn pin_rst_flag(self) -> bool {
        (self.0 & Self::PIN_RST_FLAG) != 0
    }
    /// Clear pin reset flag.
    #[inline]
    pub const fn clear_pin_rst_flag(self) -> Self {
        Self(self.0 | Self::PIN_RST_FLAG)
    }
    /// Check if RTC power-on reset flag is set (`RTC_POR_FLAG`).
    #[doc(alias = "RTC_POR_FLAG")]
    #[inline]
    pub const fn rtc_por_flag(self) -> bool {
        (self.0 & Self::RTC_POR_FLAG) != 0
    }
    /// Clear RTC power-on reset flag.
    #[inline]
    pub const fn clear_rtc_por_flag(self) -> Self {
        Self(self.0 | Self::RTC_POR_FLAG)
    }
    /// Check if system power-on reset flag is set (`SYS_POR_FLAG`).
    #[doc(alias = "SYS_POR_FLAG")]
    #[inline]
    pub const fn sys_por_flag(self) -> bool {
        (self.0 & Self::SYS_POR_FLAG) != 0
    }
    /// Clear system power-on reset flag.
    #[inline]
    pub const fn clear_sys_por_flag(self) -> Self {
        Self(self.0 | Self::SYS_POR_FLAG)
    }
}

#[cfg(test)]
mod tests {
    use super::{RegisterBlock, RstFlag};
    use core::mem::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, reset_flag), 0x0);
        assert_eq!(offset_of!(RegisterBlock, version), 0xFFC);
    }

    #[test]
    fn struct_rst_flag_functions() {
        let mut flag = RstFlag(0);
        assert!(!flag.cmp_rst_flag());
        flag = flag.clear_cmp_rst_flag();
        assert!(flag.cmp_rst_flag());
        assert_eq!(flag.0, 0x0000_1000);

        flag = RstFlag(0);
        assert!(!flag.ths_rst_flag());
        flag = flag.clear_ths_rst_flag();
        assert!(flag.ths_rst_flag());
        assert_eq!(flag.0, 0x0000_0800);

        flag = RstFlag(0);
        assert!(!flag.wdog_rst_flag());
        flag = flag.clear_wdog_rst_flag();
        assert!(flag.wdog_rst_flag());
        assert_eq!(flag.0, 0x0000_0400);

        flag = RstFlag(0);
        assert!(!flag.cpu_dm_rst_flag());
        flag = flag.clear_cpu_dm_rst_flag();
        assert!(flag.cpu_dm_rst_flag());
        assert_eq!(flag.0, 0x0000_0200);

        flag = RstFlag(0);
        assert!(!flag.pin_rst_flag());
        flag = flag.clear_pin_rst_flag();
        assert!(flag.pin_rst_flag());
        assert_eq!(flag.0, 0x0000_0100);

        flag = RstFlag(0);
        assert!(!flag.rtc_por_flag());
        flag = flag.clear_rtc_por_flag();
        assert!(flag.rtc_por_flag());
        assert_eq!(flag.0, 0x0000_0002);

        flag = RstFlag(0);
        assert!(!flag.sys_por_flag());
        flag = flag.clear_sys_por_flag();
        assert!(flag.sys_por_flag());
        assert_eq!(flag.0, 0x0000_0001);
    }
}
