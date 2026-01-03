//! Generic Timer Controller register blocks and registers.

use volatile_register::{RO, RW};

/// GTC configuration register block.
#[repr(C)]
pub struct RegisterBlock {
    /// GTC counter control register (`GTC_CNTCR`).
    #[doc(alias = "GTC_CNTCR")]
    pub cnt_ctrl: RW<CntControl>,
    /// GTC counter status register (`GTC_CNTSR`).
    #[doc(alias = "GTC_CNTSR")]
    pub cnt_status: RO<CntStatus>,
    /// GTC counter value low register (`GTC_CNTVL`).
    #[doc(alias = "GTC_CNTVL")]
    pub cnt_value_low: RW<u32>,
    /// GTC counter value high register (`GTC_CNTVH`).
    #[doc(alias = "GTC_CNTVH")]
    pub cnt_value_high: RW<CntValHigh>,
    _reserved0: [u8; 0x10],
    /// GTC counter frequency ID0 register (`GTC_CNTFID0`).
    #[doc(alias = "GTC_CNTFID0")]
    pub cnt_freq_id0: RO<u32>,
    /// GTC counter frequency ID1 register (`GTC_CNTFID1`).
    #[doc(alias = "GTC_CNTFID1")]
    pub cnt_freq_id1: RO<u32>,
    /// GTC counter frequency ID2 register (`GTC_CNTFID2`).
    #[doc(alias = "GTC_CNTFID2")]
    pub cnt_freq_id2: RO<u32>,
    /// GTC counter frequency ID3 register (`GTC_CNTFID3`).
    #[doc(alias = "GTC_CNTFID3")]
    pub cnt_freq_id3: RO<u32>,
    _reserved1: [u8; 0x90],
    /// GTC configuration register (`GTC_CONFG`).
    #[doc(alias = "GTC_CONFG")]
    pub config: RW<Config>,
    _reserved2: [u8; 0xF38],
    /// GTC version register (`GTC_VERSION`).
    #[doc(alias = "GTC_VERSION")]
    pub version: RO<u32>,
}

/// Counter frequency.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CntFreq {
    Freq4M,
    Freq1M,
    Freq250k,
}

/// GTC counter control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CntControl(u32);

impl CntControl {
    const FCREQ: u32 = 0x3 << 8;
    const CNT_ON_DBG: u32 = 0x1 << 1;
    const EN: u32 = 0x1;

    /// Set the counter frequency (`FCREQ`).
    #[doc(alias = "FCREQ")]
    #[inline]
    pub const fn set_freq(self, freq: CntFreq) -> Self {
        Self((self.0 & !Self::FCREQ) | (Self::FCREQ & ((freq as u32) << 8)))
    }
    /// Get the counter frequency.
    #[inline]
    pub const fn freq(self) -> CntFreq {
        match (self.0 & Self::FCREQ) >> 8 {
            0x0 => CntFreq::Freq4M,
            0x1 => CntFreq::Freq1M,
            0x2 => CntFreq::Freq250k,
            _ => unreachable!(),
        }
    }
    /// Enable continuous counting in debug mode (`CNT_ON_DBG`).
    #[doc(alias = "CNT_ON_DBG")]
    #[inline]
    pub const fn enable_cnt_on_dbg(self) -> Self {
        Self(self.0 | Self::CNT_ON_DBG)
    }
    /// Disable continuous counting in debug mode.
    #[inline]
    pub const fn disable_cnt_on_dbg(self) -> Self {
        Self(self.0 & !Self::CNT_ON_DBG)
    }
    /// Check if continuous counting in debug mode is enabled.
    #[inline]
    pub const fn is_cnt_on_dbg_enabled(self) -> bool {
        (self.0 & Self::CNT_ON_DBG) != 0
    }
    /// Enable counter (`EN`).
    #[doc(alias = "EN")]
    #[inline]
    pub const fn enable_cnt(self) -> Self {
        Self(self.0 | Self::EN)
    }
    /// Disable counter.
    #[inline]
    pub const fn disable_cnt(self) -> Self {
        Self(self.0 & !Self::EN)
    }
    /// Check if counter is enabled.
    #[inline]
    pub const fn is_cnt_enabled(self) -> bool {
        (self.0 & Self::EN) != 0
    }
}

/// GTC counter status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CntStatus(u32);

impl CntStatus {
    const FCACK: u32 = 0x3 << 8;

    /// Get current counter frequency (`FCACK`).
    #[doc(alias = "FCACK")]
    #[inline]
    pub const fn fcack(self) -> CntFreq {
        match (self.0 & Self::FCACK) >> 8 {
            0x0 => CntFreq::Freq4M,
            0x1 => CntFreq::Freq1M,
            0x2 => CntFreq::Freq250k,
            _ => unreachable!(),
        }
    }
}

/// GTC counter value high register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CntValHigh(u32);

impl CntValHigh {
    const CNTVH: u32 = 0xFFFFF;

    /// Set counter value high (`CNTVH`).
    #[doc(alias = "CNTVH")]
    #[inline]
    pub const fn set_cnt_value_high(self, value: u32) -> Self {
        Self((self.0 & !Self::CNTVH) | (value & Self::CNTVH))
    }
    /// Get counter value high.
    #[inline]
    pub const fn cnt_value_high(self) -> u32 {
        self.0 & Self::CNTVH
    }
}

/// GTC configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Config(u32);

impl Config {
    const FDIV: u32 = 0x1F << 8;

    /// Set frequency divider (`FDIV`).
    ///
    /// CNT_MOD_CLK = PCLK / FDIV.
    #[doc(alias = "FDIV")]
    #[inline]
    pub const fn set_fdiv(self, div: u16) -> Self {
        assert!(
            div <= 0x1F,
            "Frequency divider out of range (expected 0..=31)"
        );
        Self((self.0 & !Self::FDIV) | (Self::FDIV & ((div as u32) << 8)))
    }
    /// Get frequency divider.
    #[inline]
    pub const fn fdiv(self) -> u16 {
        ((self.0 & Self::FDIV) >> 8) as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_should_panic;
    use core::mem::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, cnt_ctrl), 0x0);
        assert_eq!(offset_of!(RegisterBlock, cnt_status), 0x4);
        assert_eq!(offset_of!(RegisterBlock, cnt_value_low), 0x8);
        assert_eq!(offset_of!(RegisterBlock, cnt_value_high), 0xC);
        assert_eq!(offset_of!(RegisterBlock, cnt_freq_id0), 0x20);
        assert_eq!(offset_of!(RegisterBlock, cnt_freq_id1), 0x24);
        assert_eq!(offset_of!(RegisterBlock, cnt_freq_id2), 0x28);
        assert_eq!(offset_of!(RegisterBlock, cnt_freq_id3), 0x2C);
        assert_eq!(offset_of!(RegisterBlock, config), 0xC0);
        assert_eq!(offset_of!(RegisterBlock, version), 0xFFC);
    }

    #[test]
    fn struct_cnt_control_functions() {
        let mut val = CntControl(0x0);
        val = val.set_freq(CntFreq::Freq250k);
        assert_eq!(val.freq(), CntFreq::Freq250k);
        assert_eq!(val.0, 0x0000_0200);
        val = val.set_freq(CntFreq::Freq1M);
        assert_eq!(val.freq(), CntFreq::Freq1M);
        assert_eq!(val.0, 0x0000_0100);
        val = val.set_freq(CntFreq::Freq4M);
        assert_eq!(val.freq(), CntFreq::Freq4M);
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_cnt_on_dbg();
        assert!(val.is_cnt_on_dbg_enabled());
        assert_eq!(val.0, 0x0000_0002);
        val = val.disable_cnt_on_dbg();
        assert!(!val.is_cnt_on_dbg_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_cnt();
        assert!(val.is_cnt_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_cnt();
        assert!(!val.is_cnt_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_cnt_status_functions() {
        let mut val = CntStatus(0x0000_0000);
        assert_eq!(val.fcack(), CntFreq::Freq4M);
        assert_eq!(val.0, 0x0000_0000);
        val = CntStatus(0x0000_0100);
        assert_eq!(val.fcack(), CntFreq::Freq1M);
        assert_eq!(val.0, 0x0000_0100);
        val = CntStatus(0x0000_0200);
        assert_eq!(val.fcack(), CntFreq::Freq250k);
        assert_eq!(val.0, 0x0000_0200);
    }

    #[test]
    fn struct_cnt_val_high_functions() {
        let mut val = CntValHigh(0x0);
        val = val.set_cnt_value_high(0x0000_FFFF);
        assert_eq!(val.cnt_value_high(), 0x0000_FFFF);
    }

    #[test]
    fn struct_config_functions() {
        let mut val = Config(0x0);
        val = val.set_fdiv(0x1F);
        assert_eq!(val.fdiv(), 0x1F);
        assert_eq!(val.0, 0x0000_1F00);
    }

    test_should_panic!((
        test_cfg_set_fdiv_panic,
        Config(0x0).set_fdiv(0x20),
        "Frequency divider out of range (expected 0..=31)"
    ));
}
