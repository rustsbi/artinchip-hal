//! DRAM parameters, detection and efuse decoding.

use crate::cmu::Cmu;
use crate::sid::{Efuse, Sid};

/// An identified DRAM generation as read out of the `SID` block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DramType {
    /// DDR2 SDRAM (1.80 V rail).
    Ddr2,
    /// DDR3 SDRAM (1.50 V rail).
    Ddr3,
}

impl DramType {
    /// The vendor's numerical code: 2 for DDR2, 3 for DDR3.
    #[inline]
    pub const fn code(self) -> u8 {
        match self {
            Self::Ddr2 => 2,
            Self::Ddr3 => 3,
        }
    }

    /// Printable name of the generation.
    #[inline]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Ddr2 => "DDR2",
            Self::Ddr3 => "DDR3",
        }
    }
}

/// The result of probing the `SID` block for DRAM configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Detection {
    /// The raw 32-bit efuse word 0.
    pub raw: u32,
    /// The extracted 4-bit DRAM field (`word0[23:20]`).
    pub field: u8,
    /// The decoded generation, or `None` if the field names no generation.
    pub dram_type: Option<DramType>,
    /// The detected capacity in bytes (0 if unprogrammed).
    pub bytes: u32,
    /// A human-readable label for the detected part.
    pub label: &'static str,
}

impl Detection {
    /// Decode the efuse word 0 into a structured detection result.
    #[inline]
    pub const fn decode(raw: u32) -> Self {
        let field = ((raw >> 20) & 0x0F) as u8;
        match field {
            0x1 => Self {
                raw,
                field,
                dram_type: Some(DramType::Ddr2),
                bytes: 0x0200_0000,
                label: "DDR2 32MB",
            },
            0x2 => Self {
                raw,
                field,
                dram_type: Some(DramType::Ddr2),
                bytes: 0x0400_0000,
                label: "DDR2 64MB",
            },
            0x3 => Self {
                raw,
                field,
                dram_type: Some(DramType::Ddr2),
                bytes: 0x0800_0000,
                label: "DDR2 128MB",
            },
            0x4 => Self {
                raw,
                field,
                dram_type: Some(DramType::Ddr3),
                bytes: 0x0800_0000,
                label: "DDR3 128MB",
            },
            0x5 => Self {
                raw,
                field,
                dram_type: Some(DramType::Ddr3),
                bytes: 0x1000_0000,
                label: "DDR3 256MB",
            },
            0x6 => Self {
                raw,
                field,
                dram_type: Some(DramType::Ddr3),
                bytes: 0x2000_0000,
                label: "DDR3 512MB",
            },
            0x7 => Self {
                raw,
                field,
                dram_type: Some(DramType::Ddr3),
                bytes: 0x4000_0000,
                label: "DDR3 1GB",
            },
            0x8 => Self {
                raw,
                field,
                dram_type: Some(DramType::Ddr3),
                bytes: 0x8000_0000,
                label: "DDR3 2GB",
            },
            _ => Self {
                raw,
                field,
                dram_type: None,
                bytes: 0,
                label: "No DDR info",
            },
        }
    }
}

/// Read the `SID` block and decode the DRAM configuration from efuse word 0.
pub fn detect(cmu: &mut Cmu, sid: &mut Sid) -> Detection {
    let efuse = Efuse::new(cmu, sid);
    Detection::decode(efuse.read(0))
}

/// A target DRAM configuration: generation, size, target clock, and spread-spectrum flag.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DramConfig {
    /// The DRAM generation.
    pub dram_type: DramType,
    /// The part's size in bytes.
    pub memsize: u32,
    /// The DRAM clock in Hz.
    pub freq: u32,
    /// Whether `PLL_FRA0` runs with spread spectrum.
    pub ssc: bool,
}

impl DramConfig {
    /// DDR3 128 MiB @ 600 MHz parameter configuration.
    pub const DDR3_128MB: Self = Self {
        dram_type: DramType::Ddr3,
        memsize: 0x0800_0000,
        freq: 600_000_000,
        ssc: false,
    };

    /// DDR2 64 MiB @ 504 MHz parameter configuration.
    pub const DDR2_64MB: Self = Self {
        dram_type: DramType::Ddr2,
        memsize: 0x0400_0000,
        freq: 504_000_000,
        ssc: false,
    };

    /// The set the efuse names, or the DDR3 128 MiB default when it names none.
    #[inline]
    pub const fn for_detected(detected: &Detection) -> Self {
        match detected.dram_type {
            Some(DramType::Ddr2) => Self::DDR2_64MB,
            Some(DramType::Ddr3) | None => Self::DDR3_128MB,
        }
    }

    /// The size in MiB, for log lines.
    #[inline]
    pub const fn memsize_mib(&self) -> u32 {
        self.memsize >> 20
    }

    /// The DRAM clock in MHz, for log lines.
    #[inline]
    pub const fn freq_mhz(&self) -> u32 {
        self.freq / 1_000_000
    }
}
