//! SPI NOR flash device parameters and quad configuration.

/// Standard SPI NOR program page size in bytes.
pub const NOR_PAGE_SIZE: usize = 256;
/// Standard SPI NOR erase sector size in bytes.
pub const NOR_SECTOR_SIZE: usize = 4096;
/// Standard SPI NOR 64 KiB erase block size in bytes.
pub const NOR_BLOCK_SIZE_64K: usize = 65536;
/// Minimum read unit in bytes.
pub const NOR_READ_SIZE: usize = 1;

/// JEDEC identification returned by `READ ID` (`9Fh`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NorJedecId {
    /// Manufacturer ID (`EFh` for Winbond).
    pub manufacturer: u8,
    /// Memory type (`40h` for Winbond W25QJV).
    pub memory_type: u8,
    /// Capacity code (`2^capacity` bytes).
    pub capacity: u8,
}

impl NorJedecId {
    /// Decode capacity in bytes when `capacity` is a standard JEDEC power-of-two code.
    pub const fn capacity_bytes(self) -> Option<usize> {
        if self.capacity >= 16 && self.capacity <= 24 {
            Some(1usize << self.capacity)
        } else {
            None
        }
    }
}

/// Geometry and capacity of a SPI NOR device.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NorDeviceInfo {
    /// Total addressable capacity in bytes.
    pub capacity: usize,
    /// Sector erase granularity in bytes.
    pub sector_size: usize,
    /// Block erase granularity in bytes.
    pub block_size: usize,
    /// Page program granularity in bytes.
    pub page_size: usize,
}

/// Known JEDEC SPI NOR table (`(manufacturer, memory_type, capacity_code, info)`).
const KNOWN_NOR_DEVICES: &[(u8, u8, u8, NorDeviceInfo)] = &[
    (0xEF, 0x40, 0x17, NorDeviceInfo::W25Q64JV),
    (0xEF, 0x40, 0x18, NorDeviceInfo::W25Q128JV),
    (0xC8, 0x40, 0x18, NorDeviceInfo::W25Q128JV),
];

impl NorDeviceInfo {
    /// Winbond W25Q64JV (8 MiB, 3-byte addressing).
    pub const W25Q64JV: Self = Self {
        capacity: 8 * 1024 * 1024,
        sector_size: NOR_SECTOR_SIZE,
        block_size: NOR_BLOCK_SIZE_64K,
        page_size: NOR_PAGE_SIZE,
    };

    /// Winbond W25Q128JV (16 MiB, 3-byte addressing).
    pub const W25Q128JV: Self = Self {
        capacity: 16 * 1024 * 1024,
        sector_size: NOR_SECTOR_SIZE,
        block_size: NOR_BLOCK_SIZE_64K,
        page_size: NOR_PAGE_SIZE,
    };

    /// Resolve device geometry from JEDEC ID, falling back to [`Self::W25Q128JV`].
    pub const fn from_jedec(id: NorJedecId) -> Self {
        let mut idx = 0;
        while idx < KNOWN_NOR_DEVICES.len() {
            let (mfg, mem, cap, info) = KNOWN_NOR_DEVICES[idx];
            if id.manufacturer == mfg && id.memory_type == mem && id.capacity == cap {
                return info;
            }
            idx += 1;
        }
        if let Some(capacity) = id.capacity_bytes() {
            Self {
                capacity,
                sector_size: NOR_SECTOR_SIZE,
                block_size: NOR_BLOCK_SIZE_64K,
                page_size: NOR_PAGE_SIZE,
            }
        } else {
            Self::W25Q128JV
        }
    }
}

/// Quad data-phase parameters for a SPI NOR device.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NorQuadConfig {
    /// Clock-to-output valid time (`tCLQV`) in nanoseconds.
    pub tclqv_ns: u32,
    /// Quad-output fast read opcode (`6Bh` by default).
    pub read_opcode: u8,
    /// Quad-input page program opcode (`32h` by default).
    pub page_program_opcode: u8,
    /// Extra dummy bytes added on top of the computed count.
    pub dummy_margin: u8,
}

impl Default for NorQuadConfig {
    fn default() -> Self {
        Self::W25Q128JV
    }
}

impl NorQuadConfig {
    /// Default parameters for W25Q128JV-compatible parts (`6Bh` read, `32h` program, `tCLQV` = 8 ns).
    pub const W25Q128JV: Self = Self {
        tclqv_ns: 8,
        read_opcode: 0x6B,
        page_program_opcode: 0x32,
        dummy_margin: 0,
    };
}

/// Configuration for [`SpiNorFlash`](super::SpiNorFlash).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SpiNorConfig {
    /// Optional four-line data phase parameters (`None` selects single-line mode).
    pub quad: Option<NorQuadConfig>,
    /// Optional explicit device geometry (`None` detects from `READ ID`).
    pub device: Option<NorDeviceInfo>,
}

impl SpiNorConfig {
    /// Single-line mode with automatic JEDEC geometry detection.
    pub const fn single() -> Self {
        Self {
            quad: None,
            device: None,
        }
    }

    /// Quad data-phase mode with automatic JEDEC geometry detection.
    pub const fn quad(quad: NorQuadConfig) -> Self {
        Self {
            quad: Some(quad),
            device: None,
        }
    }
}
