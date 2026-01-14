//! SPI ENC register blocks and registers.

use volatile_register::{RO, RW};

/// Serial Peripheral Interface Encryption registers.
#[repr(C)]
pub struct RegisterBlock {
    /// SPI ENC control register (`SPIE_CTL`).
    #[doc(alias = "SPIE_CTL")]
    pub ctrl: RW<Control>,
    /// SPI ENC interrupt control register (`SPIE_ICR`).
    #[doc(alias = "SPIE_ICR")]
    pub int_ctrl: RW<IntControl>,
    /// SPI ENC interrupt status register (`SPIE_ISR`).
    #[doc(alias = "SPIE_ISR")]
    pub int_status: RW<IntStatus>,
    /// SPI ENC key counter register (`SPIE_KCNT`).
    #[doc(alias = "SPIE_KCNT")]
    pub key_cnt: RO<u32>,
    /// SPI ENC output counter register (`SPIE_OCNT`).
    #[doc(alias = "SPIE_OCNT")]
    pub output_cnt: RO<u32>,
    /// SPI ENC data address register (`SPIE_ADDR`).
    #[doc(alias = "SPIE_ADDR")]
    pub addr: RW<u32>,
    /// SPI ENC tweak register (`SPIE_TWEAK`).
    #[doc(alias = "SPIE_TWEAK")]
    pub tweak: RW<u32>,
    /// SPI ENC ciphertext position register (`SPIE_CPOS`).
    #[doc(alias = "SPIE_CPOS")]
    pub cipher_pos: RW<u32>,
    /// SPI ENC ciphertext length register (`SPIE_CLEN`).
    #[doc(alias = "SPIE_CLEN")]
    pub cipher_len: RW<u32>,
    _reserved0: [u8; 0xFD8],
    /// SPI ENC version register (`SPIE_VER`).
    #[doc(alias = "SPIE_VER")]
    pub version: RO<u32>,
}

/// Encryption bus connection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EncBus {
    /// Bypass, not connected to any bus.
    Bypass,
    /// Connected to SPI0 bus.
    Spi0,
    /// Connected to SPI1 bus.
    Spi1,
}

/// SPI ENC control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Control(u32);

impl Control {
    const XIP_EN: u32 = 0x1 << 16;
    const SPI_SEL: u32 = 0x3 << 12;
    const KEY_START: u32 = 0x1;

    /// Enable spi xip encryption (`XIP_EN`).
    ///
    /// Once enabled, the hardware automatically connects to the SPI controller.
    #[doc(alias = "XIP_EN")]
    #[inline]
    pub const fn enable_xip_enc(self) -> Self {
        Self(self.0 | Self::XIP_EN)
    }
    /// Disable spi xip encryption.
    #[inline]
    pub const fn disable_xip_enc(self) -> Self {
        Self(self.0 & !Self::XIP_EN)
    }
    /// Check if spi xip encryption is enabled.
    #[inline]
    pub const fn is_xip_enc_enabled(self) -> bool {
        self.0 & Self::XIP_EN != 0
    }
    /// Set encryption bus (`SPI_SEL`).
    ///
    /// When XIP encryption is enabled, setting the encryption bus is invalid.
    #[doc(alias = "SPI_SEL")]
    #[inline]
    pub const fn set_enc_bus(self, bus: EncBus) -> Self {
        Self((self.0 & !Self::SPI_SEL) | (Self::SPI_SEL & ((bus as u32) << 12)))
    }
    /// Get encryption bus.
    #[inline]
    pub const fn enc_bus(self) -> EncBus {
        match (self.0 & Self::SPI_SEL) >> 12 {
            0 => EncBus::Bypass,
            1 => EncBus::Spi0,
            2 => EncBus::Spi1,
            _ => unreachable!(),
        }
    }
    /// Set key start bit (`KEY_START`).
    ///
    /// Before or during data encryption and decryption, the group key must be calculated first;
    /// otherwise, the encryption and decryption unit will keep waiting for the key to be generated, leading to transmission failure.
    /// After the transmission is complete, the key must be cleared to stop the key calculation.
    ///
    /// When XIP encryption is enabled, setting the key start bit is invalid.
    #[doc(alias = "KEY_START")]
    #[inline]
    pub const fn set_key_start(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::KEY_START)
        } else {
            Self(self.0 & !Self::KEY_START)
        }
    }
    /// Get key start bit.
    #[inline]
    pub const fn key_start(self) -> bool {
        self.0 & Self::KEY_START != 0
    }
}

/// SPI ENC interrupt control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IntControl(u32);

impl IntControl {
    const KEY_OVF: u32 = 0x1 << 5;
    const KEY_UDF: u32 = 0x1 << 4;
    const READ_HALF_EMPTY: u32 = 0x1 << 3;
    const READ_EMPTY_DATA: u32 = 0x1 << 2;
    const ENC_DEC_FINISHED: u32 = 0x1 << 1;
    const KEY_GEN: u32 = 0x1;

    /// Enable group key fifo overflow interrupt (`KEY_OVF`).
    #[doc(alias = "KEY_OVF")]
    #[inline]
    pub const fn enable_key_ovf_int(self) -> Self {
        Self(self.0 | Self::KEY_OVF)
    }
    /// Disable group key fifo overflow interrupt.
    #[inline]
    pub const fn disable_key_ovf_int(self) -> Self {
        Self(self.0 & !Self::KEY_OVF)
    }
    /// Check if group key fifo overflow interrupt is enabled.
    #[inline]
    pub const fn is_key_ovf_int_enabled(self) -> bool {
        self.0 & Self::KEY_OVF != 0
    }
    /// Enable group key fifo underflow interrupt (`KEY_UDF`).
    #[doc(alias = "KEY_UDF")]
    #[inline]
    pub const fn enable_key_udf_int(self) -> Self {
        Self(self.0 | Self::KEY_UDF)
    }
    /// Disable group key fifo underflow interrupt.
    #[inline]
    pub const fn disable_key_udf_int(self) -> Self {
        Self(self.0 & !Self::KEY_UDF)
    }
    /// Check if group key fifo underflow interrupt is enabled.
    #[inline]
    pub const fn is_key_udf_int_enabled(self) -> bool {
        self.0 & Self::KEY_UDF != 0
    }
    /// Enable read half empty interrupt (`READ_HALF_EMPTY`).
    #[doc(alias = "READ_HALF_EMPTY")]
    #[inline]
    pub const fn enable_read_half_empty_int(self) -> Self {
        Self(self.0 | Self::READ_HALF_EMPTY)
    }
    /// Disable read half empty interrupt.
    #[inline]
    pub const fn disable_read_half_empty_int(self) -> Self {
        Self(self.0 & !Self::READ_HALF_EMPTY)
    }
    /// Check if read half empty interrupt is enabled.
    #[inline]
    pub const fn is_read_half_empty_int_enabled(self) -> bool {
        self.0 & Self::READ_HALF_EMPTY != 0
    }
    /// Enable read empty data interrupt (`READ_EMPTY_DATA`).
    #[doc(alias = "READ_EMPTY_DATA")]
    #[inline]
    pub const fn enable_read_empty_data_int(self) -> Self {
        Self(self.0 | Self::READ_EMPTY_DATA)
    }
    /// Disable read empty data interrupt.
    #[inline]
    pub const fn disable_read_empty_data_int(self) -> Self {
        Self(self.0 & !Self::READ_EMPTY_DATA)
    }
    /// Check if read empty data interrupt is enabled.
    #[inline]
    pub const fn is_read_empty_data_int_enabled(self) -> bool {
        self.0 & Self::READ_EMPTY_DATA != 0
    }
    /// Enable encryption/decryption finished interrupt (`ENC_DEC_FINISHED`).
    #[doc(alias = "ENC_DEC_FINISHED")]
    #[inline]
    pub const fn enable_enc_dec_finished_int(self) -> Self {
        Self(self.0 | Self::ENC_DEC_FINISHED)
    }
    /// Disable encryption/decryption finished interrupt.
    #[inline]
    pub const fn disable_enc_dec_finished_int(self) -> Self {
        Self(self.0 & !Self::ENC_DEC_FINISHED)
    }
    /// Check if encryption/decryption finished interrupt is enabled.
    #[inline]
    pub const fn is_enc_dec_finished_int_enabled(self) -> bool {
        self.0 & Self::ENC_DEC_FINISHED != 0
    }
    /// Enable the first group key generation interrupt (`KEY_GEN`).
    #[doc(alias = "KEY_GEN")]
    #[inline]
    pub const fn enable_key_gen_int(self) -> Self {
        Self(self.0 | Self::KEY_GEN)
    }
    /// Disable the first group key generation interrupt.
    #[inline]
    pub const fn disable_key_gen_int(self) -> Self {
        Self(self.0 & !Self::KEY_GEN)
    }
    /// Check if the first group key generation interrupt is enabled.
    #[inline]
    pub const fn is_key_gen_int_enabled(self) -> bool {
        self.0 & Self::KEY_GEN != 0
    }
}

/// SPI ENC interrupt status register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IntStatus(u32);

impl IntStatus {
    const KEY_OVF: u32 = 0x1 << 5;
    const KEY_UDF: u32 = 0x1 << 4;
    const READ_HALF_EMPTY: u32 = 0x1 << 3;
    const READ_EMPTY_DATA: u32 = 0x1 << 2;
    const ENC_DEC_FINISHED: u32 = 0x1 << 1;
    const KEY_GEN: u32 = 0x1;

    /// Check if group key fifo overflow interrupt is pending (`KEY_OVF`).
    #[doc(alias = "KEY_OVF")]
    #[inline]
    pub const fn is_key_ovf_int_pending(self) -> bool {
        self.0 & Self::KEY_OVF != 0
    }
    /// Clear group key fifo overflow interrupt.
    #[inline]
    pub const fn clear_key_ovf_int(self) -> Self {
        Self(self.0 | Self::KEY_OVF)
    }
    /// Check if group key fifo underflow interrupt is pending (`KEY_UDF`).
    #[doc(alias = "KEY_UDF")]
    #[inline]
    pub const fn is_key_udf_int_pending(self) -> bool {
        self.0 & Self::KEY_UDF != 0
    }
    /// Clear group key fifo underflow interrupt.
    #[inline]
    pub const fn clear_key_udf_int(self) -> Self {
        Self(self.0 | Self::KEY_UDF)
    }
    /// Check if read half empty interrupt is pending (`READ_HALF_EMPTY`).
    #[doc(alias = "READ_HALF_EMPTY")]
    #[inline]
    pub const fn is_read_half_empty_int_pending(self) -> bool {
        self.0 & Self::READ_HALF_EMPTY != 0
    }
    /// Clear read half empty interrupt.
    #[inline]
    pub const fn clear_read_half_empty_int(self) -> Self {
        Self(self.0 | Self::READ_HALF_EMPTY)
    }
    /// Check if read empty data interrupt is pending (`READ_EMPTY_DATA`).
    #[doc(alias = "READ_EMPTY_DATA")]
    #[inline]
    pub const fn is_read_empty_data_int_pending(self) -> bool {
        self.0 & Self::READ_EMPTY_DATA != 0
    }
    /// Clear read empty data interrupt.
    #[inline]
    pub const fn clear_read_empty_data_int(self) -> Self {
        Self(self.0 | Self::READ_EMPTY_DATA)
    }
    /// Check if encryption/decryption finished interrupt is pending (`ENC_DEC_FINISHED`).
    #[doc(alias = "ENC_DEC_FINISHED")]
    #[inline]
    pub const fn is_enc_dec_finished_int_pending(self) -> bool {
        self.0 & Self::ENC_DEC_FINISHED != 0
    }
    /// Clear encryption/decryption finished interrupt.
    #[inline]
    pub const fn clear_enc_dec_finished_int(self) -> Self {
        Self(self.0 | Self::ENC_DEC_FINISHED)
    }
    /// Check if the first group key generation interrupt is pending (`KEY_GEN`).
    #[doc(alias = "KEY_GEN")]
    #[inline]
    pub const fn is_key_gen_int_pending(self) -> bool {
        self.0 & Self::KEY_GEN != 0
    }
    /// Clear the first group key generation interrupt.
    #[inline]
    pub const fn clear_key_gen_int(self) -> Self {
        Self(self.0 | Self::KEY_GEN)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, ctrl), 0x0);
        assert_eq!(offset_of!(RegisterBlock, int_ctrl), 0x4);
        assert_eq!(offset_of!(RegisterBlock, int_status), 0x8);
        assert_eq!(offset_of!(RegisterBlock, key_cnt), 0xC);
        assert_eq!(offset_of!(RegisterBlock, output_cnt), 0x10);
        assert_eq!(offset_of!(RegisterBlock, addr), 0x14);
        assert_eq!(offset_of!(RegisterBlock, tweak), 0x18);
        assert_eq!(offset_of!(RegisterBlock, cipher_pos), 0x1C);
        assert_eq!(offset_of!(RegisterBlock, cipher_len), 0x20);
        assert_eq!(offset_of!(RegisterBlock, version), 0xFFC);
    }

    #[test]
    fn struct_control_functions() {
        let mut val = Control(0x0);

        val = val.enable_xip_enc();
        assert!(val.is_xip_enc_enabled());
        assert_eq!(val.0, 0x0001_0000);
        val = val.disable_xip_enc();
        assert!(!val.is_xip_enc_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_enc_bus(EncBus::Spi1);
        assert_eq!(val.enc_bus(), EncBus::Spi1);
        assert_eq!(val.0, 0x0000_2000);
        val = val.set_enc_bus(EncBus::Spi0);
        assert_eq!(val.enc_bus(), EncBus::Spi0);
        assert_eq!(val.0, 0x0000_1000);
        val = val.set_enc_bus(EncBus::Bypass);
        assert_eq!(val.enc_bus(), EncBus::Bypass);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_key_start(true);
        assert!(val.key_start());
        assert_eq!(val.0, 0x0000_0001);
        val = val.set_key_start(false);
        assert!(!val.key_start());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_int_control_functions() {
        let mut val = IntControl(0x0);

        val = val.enable_key_ovf_int();
        assert!(val.is_key_ovf_int_enabled());
        assert_eq!(val.0, 0x0000_0020);
        val = val.disable_key_ovf_int();
        assert!(!val.is_key_ovf_int_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_key_udf_int();
        assert!(val.is_key_udf_int_enabled());
        assert_eq!(val.0, 0x0000_0010);
        val = val.disable_key_udf_int();
        assert!(!val.is_key_udf_int_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_read_half_empty_int();
        assert!(val.is_read_half_empty_int_enabled());
        assert_eq!(val.0, 0x0000_0008);
        val = val.disable_read_half_empty_int();
        assert!(!val.is_read_half_empty_int_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_read_empty_data_int();
        assert!(val.is_read_empty_data_int_enabled());
        assert_eq!(val.0, 0x0000_0004);
        val = val.disable_read_empty_data_int();
        assert!(!val.is_read_empty_data_int_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_enc_dec_finished_int();
        assert!(val.is_enc_dec_finished_int_enabled());
        assert_eq!(val.0, 0x0000_0002);
        val = val.disable_enc_dec_finished_int();
        assert!(!val.is_enc_dec_finished_int_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_key_gen_int();
        assert!(val.is_key_gen_int_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_key_gen_int();
        assert!(!val.is_key_gen_int_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_int_status_functions() {
        let mut val = IntStatus(0x0).clear_key_ovf_int();
        assert!(val.is_key_ovf_int_pending());
        assert_eq!(val.0, 0x0000_0020);

        val = IntStatus(0x0).clear_key_udf_int();
        assert!(val.is_key_udf_int_pending());
        assert_eq!(val.0, 0x0000_0010);

        val = IntStatus(0x0).clear_read_half_empty_int();
        assert!(val.is_read_half_empty_int_pending());
        assert_eq!(val.0, 0x0000_0008);

        val = IntStatus(0x0).clear_read_empty_data_int();
        assert!(val.is_read_empty_data_int_pending());
        assert_eq!(val.0, 0x0000_0004);

        val = IntStatus(0x0).clear_enc_dec_finished_int();
        assert!(val.is_enc_dec_finished_int_pending());
        assert_eq!(val.0, 0x0000_0002);

        val = IntStatus(0x0).clear_key_gen_int();
        assert!(val.is_key_gen_int_pending());
        assert_eq!(val.0, 0x0000_0001);
    }
}
