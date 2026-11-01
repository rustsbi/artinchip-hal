//! Decoded views of SPI NAND feature registers (`A0h`, `B0h`, `C0h`).

/// Decoded status register (`C0h`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Status(u8);

impl Status {
    /// Operation in progress (`OIP`).
    pub const BUSY: u8 = 1 << 0;
    /// Write-enable latch (`WEL`).
    pub const WRITE_ENABLED: u8 = 1 << 1;
    /// Erase failed (`EFAIL`).
    pub const ERASE_FAILED: u8 = 1 << 2;
    /// Program failed (`PFAIL`).
    pub const PROGRAM_FAILED: u8 = 1 << 3;
    /// ECC verdict mask (bits 5:4).
    pub const ECC_MASK: u8 = 0b11 << 4;

    /// Decode a raw register byte.
    pub const fn from_byte(byte: u8) -> Self {
        Self(byte)
    }

    /// Raw register byte.
    pub const fn byte(self) -> u8 {
        self.0
    }

    /// Return whether an array operation is in progress.
    pub const fn busy(self) -> bool {
        self.0 & Self::BUSY != 0
    }

    /// Return whether the write-enable latch is set.
    pub const fn write_enabled(self) -> bool {
        self.0 & Self::WRITE_ENABLED != 0
    }

    /// Return whether the last block erase failed.
    pub const fn erase_failed(self) -> bool {
        self.0 & Self::ERASE_FAILED != 0
    }

    /// Return whether the last page program failed.
    pub const fn program_failed(self) -> bool {
        self.0 & Self::PROGRAM_FAILED != 0
    }

    /// Decode the on-die ECC verdict from bits 5:4.
    pub const fn ecc_verdict(self) -> EccVerdict {
        match (self.0 & Self::ECC_MASK) >> 4 {
            0 => EccVerdict::NoBitFlips,
            1 => EccVerdict::Corrected,
            2 => EccVerdict::Uncorrectable,
            _ => EccVerdict::Reserved,
        }
    }
}

/// On-die ECC verdict reported in status register bits 5:4 after a page read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EccVerdict {
    /// Page contained no bit flips.
    NoBitFlips,
    /// Bit flips were detected and corrected by on-die ECC.
    Corrected,
    /// Bit flips exceeded ECC capacity.
    Uncorrectable,
    /// Reserved encoding (`0b11`).
    Reserved,
}

impl EccVerdict {
    /// Short human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NoBitFlips => "no bit flips",
            Self::Corrected => "corrected",
            Self::Uncorrectable => "uncorrectable",
            Self::Reserved => "reserved",
        }
    }
}

/// Decoded configuration register (`B0h`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Configuration(u8);

impl Configuration {
    /// Quad-enable bit (`QE`, bit 0).
    pub const QUAD_ENABLE: u8 = 1 << 0;
    /// Buffer read mode (`BUF`, bit 3).
    pub const BUFFER_READ: u8 = 1 << 3;
    /// On-die ECC enable (`ECC-E`, bit 4).
    pub const ECC_ENABLE: u8 = 1 << 4;
    /// OTP access enable (`OTP-E`, bit 6).
    pub const OTP_ENABLE: u8 = 1 << 6;

    /// Standard operational mode: on-die ECC enabled and buffer read mode active.
    pub const OPERATIONAL: Self = Self(Self::ECC_ENABLE | Self::BUFFER_READ);

    /// Decode a raw register byte.
    pub const fn from_byte(byte: u8) -> Self {
        Self(byte)
    }

    /// Raw register byte.
    pub const fn byte(self) -> u8 {
        self.0
    }

    /// Return whether on-die ECC is enabled.
    pub const fn ecc_enabled(self) -> bool {
        self.0 & Self::ECC_ENABLE != 0
    }

    /// Return whether buffer read mode is enabled.
    pub const fn buffer_read(self) -> bool {
        self.0 & Self::BUFFER_READ != 0
    }

    /// Return whether OTP access is enabled.
    pub const fn otp_enabled(self) -> bool {
        self.0 & Self::OTP_ENABLE != 0
    }

    /// Return whether quad mode is enabled.
    pub const fn quad_enabled(self) -> bool {
        self.0 & Self::QUAD_ENABLE != 0
    }

    /// Return a copy with `mask` bits replaced from `value`.
    pub const fn with_bits(self, mask: u8, value: u8) -> Self {
        Self((self.0 & !mask) | (mask & value))
    }
}

/// Decoded block-protection register (`A0h`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Protection(u8);

impl Protection {
    /// Block-protect field (`BP3:BP0`, bits 6:3).
    pub const BLOCK_PROTECT_MASK: u8 = 0b1111 << 3;

    /// Decode a raw register byte.
    pub const fn from_byte(byte: u8) -> Self {
        Self(byte)
    }

    /// Raw register byte.
    pub const fn byte(self) -> u8 {
        self.0
    }

    /// Extract the `BP3:BP0` protection field.
    pub const fn block_protect_bits(self) -> u8 {
        (self.0 & Self::BLOCK_PROTECT_MASK) >> 3
    }

    /// Return whether all blocks are unlocked.
    pub const fn is_unlocked(self) -> bool {
        self.block_protect_bits() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_decodes_ecc_verdict() {
        assert_eq!(
            Status::from_byte(0b00 << 4).ecc_verdict(),
            EccVerdict::NoBitFlips
        );
        assert_eq!(
            Status::from_byte(0b01 << 4).ecc_verdict(),
            EccVerdict::Corrected
        );
        assert_eq!(
            Status::from_byte(0b10 << 4).ecc_verdict(),
            EccVerdict::Uncorrectable
        );
        assert_eq!(
            Status::from_byte(0b11 << 4).ecc_verdict(),
            EccVerdict::Reserved
        );
        assert_eq!(
            Status::from_byte(Status::BUSY | (0b10 << 4)).ecc_verdict(),
            EccVerdict::Uncorrectable
        );
    }
}
