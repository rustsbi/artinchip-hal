//! SPI NAND flash device geometry and quad configuration.

use super::status::{Configuration, Protection};

/// JEDEC identification returned by `READ ID` (`9Fh` + dummy byte).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NandJedecId {
    /// Manufacturer ID (`EFh` for Winbond).
    pub manufacturer: u8,
    /// Two-byte device ID.
    pub device: u16,
}

impl NandJedecId {
    /// Decode the three identification bytes returned by `READ ID`.
    pub const fn from_bytes(bytes: [u8; 3]) -> Self {
        Self {
            manufacturer: bytes[0],
            device: u16::from_be_bytes([bytes[1], bytes[2]]),
        }
    }
}

/// Geometry of a SPI NAND device.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NandGeometry {
    /// Main-area bytes per page.
    pub page_size: usize,
    /// Spare-area (OOB) bytes per page.
    pub oob_size: usize,
    /// Number of pages per erase block.
    pub pages_per_block: usize,
    /// Number of erase blocks on the device.
    pub block_count: usize,
}

/// Known JEDEC SPI NAND table (`(manufacturer, device, geometry)`).
const KNOWN_NAND_DEVICES: &[(u8, u16, NandGeometry)] = &[
    (0xEF, 0xAA21, NandGeometry::W25N01GV),
    (0xEF, 0xAB21, NandGeometry::W25N02KV),
    (0xC8, 0xD148, NandGeometry::W25N01GV),
    (0xC8, 0xF148, NandGeometry::W25N01GV),
];

impl NandGeometry {
    /// Winbond W25N01GV and compatible 1 Gbit parts (2 KiB page, 64 B OOB, 1024 blocks).
    pub const W25N01GV: Self = Self {
        page_size: 2048,
        oob_size: 64,
        pages_per_block: 64,
        block_count: 1024,
    };

    /// Winbond W25N02KV and compatible 2 Gbit parts (2 KiB page, 128 B OOB, 2048 blocks).
    pub const W25N02KV: Self = Self {
        page_size: 2048,
        oob_size: 128,
        pages_per_block: 64,
        block_count: 2048,
    };

    /// Bytes per erase block (`page_size * pages_per_block`).
    pub const fn block_size(self) -> usize {
        self.page_size * self.pages_per_block
    }

    /// Total main-area pages (`pages_per_block * block_count`).
    pub const fn page_count(self) -> usize {
        self.pages_per_block * self.block_count
    }

    /// Total addressable main-area capacity in bytes.
    pub const fn capacity(self) -> usize {
        self.block_size() * self.block_count
    }

    /// Resolve geometry from JEDEC ID, falling back to [`Self::W25N01GV`].
    pub const fn from_jedec(id: NandJedecId) -> Self {
        let mut idx = 0;
        while idx < KNOWN_NAND_DEVICES.len() {
            let (mfg, dev, geom) = KNOWN_NAND_DEVICES[idx];
            if id.manufacturer == mfg && id.device == dev {
                return geom;
            }
            idx += 1;
        }
        Self::W25N01GV
    }
}

/// Quad cache-transfer parameters for a SPI NAND device.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NandQuadConfig {
    /// Clock-to-output valid time (`tCLQV`) in nanoseconds.
    pub tclqv_ns: u32,
    /// Quad-output cache read opcode (`6Bh` by default).
    pub read_cache_opcode: u8,
    /// Quad-input program load opcode (`32h` by default).
    pub load_program_opcode: u8,
    /// Extra dummy bytes added on top of the computed count.
    pub dummy_margin: u8,
}

impl Default for NandQuadConfig {
    fn default() -> Self {
        Self::W25N01GV
    }
}

impl NandQuadConfig {
    /// Default parameters for W25N01GV-compatible parts (`6Bh` read, `32h` load, `tCLQV` = 10 ns).
    pub const W25N01GV: Self = Self {
        tclqv_ns: 10,
        read_cache_opcode: 0x6B,
        load_program_opcode: 0x32,
        dummy_margin: 0,
    };
}

/// Configuration for [`SpiNandFlash`](super::SpiNandFlash).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SpiNandConfig {
    /// Optional four-line cache transfer parameters (`None` selects single-line mode).
    pub quad: Option<NandQuadConfig>,
    /// Optional explicit device geometry (`None` detects from `READ ID`).
    pub geometry: Option<NandGeometry>,
}

impl SpiNandConfig {
    /// Single-line mode with automatic JEDEC geometry detection.
    pub const fn single() -> Self {
        Self {
            quad: None,
            geometry: None,
        }
    }

    /// Quad cache-transfer mode with automatic JEDEC geometry detection.
    pub const fn quad(quad: NandQuadConfig) -> Self {
        Self {
            quad: Some(quad),
            geometry: None,
        }
    }
}

/// Identification and feature-register snapshot recorded during bring-up.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NandInit {
    /// JEDEC identification read with `READ ID`.
    pub id: NandJedecId,
    /// Configuration register (`B0h`) before bring-up.
    pub configuration_before: Configuration,
    /// Configuration register (`B0h`) after bring-up.
    pub configuration_after: Configuration,
    /// Block-protection register (`A0h`) before bring-up cleared it.
    pub protection_before: Protection,
}

impl NandInit {
    pub(super) const UNKNOWN: Self = Self {
        id: NandJedecId {
            manufacturer: 0,
            device: 0,
        },
        configuration_before: Configuration::from_byte(0),
        configuration_after: Configuration::from_byte(0),
        protection_before: Protection::from_byte(0),
    };
}
