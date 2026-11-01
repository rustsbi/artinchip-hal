//! SPI NOR opcodes and timeout constants.

/// Write enable (`06h`).
pub(super) const WRITE_ENABLE: u8 = 0x06;
/// Read status register 1 (`05h`).
pub(super) const READ_STATUS1: u8 = 0x05;
/// Read status register 2 (`35h`).
pub(super) const READ_STATUS2: u8 = 0x35;
/// Write status register 1 (`01h`).
pub(super) const WRITE_STATUS1: u8 = 0x01;
/// Write status register 2 (`31h`).
pub(super) const WRITE_STATUS2: u8 = 0x31;
/// Single-line page program (`02h`).
pub(super) const PAGE_PROGRAM: u8 = 0x02;
/// 4 KiB sector erase (`20h`).
pub(super) const SECTOR_ERASE: u8 = 0x20;
/// 64 KiB block erase (`D8h`).
pub(super) const BLOCK_ERASE: u8 = 0xD8;
/// Single-line read data (`03h`).
pub(super) const READ: u8 = 0x03;
/// Read JEDEC identification (`9Fh`).
pub(super) const READ_ID: u8 = 0x9F;
/// Read 64-bit unique ID (`4Bh`, followed by four dummy bytes).
pub(super) const READ_UNIQUE_ID: u8 = 0x4B;

/// Poll interval while waiting on `BUSY` (`WIP`).
pub(super) const POLL_INTERVAL_US: u32 = 100;
/// Page program timeout (`tPP` <= 3 ms typical).
pub(super) const PAGE_PROGRAM_TIMEOUT_US: u32 = 20_000;
/// Sector erase timeout (`tSE` <= 400 ms typical).
pub(super) const SECTOR_ERASE_TIMEOUT_US: u32 = 1_000_000;
/// Block erase timeout (`tBE` <= 2 s typical).
pub(super) const BLOCK_ERASE_TIMEOUT_US: u32 = 5_000_000;
/// Status register write timeout (`tW` <= 15 ms typical).
pub(super) const STATUS_WRITE_TIMEOUT_US: u32 = 100_000;
/// Maximum payload per QSPI controller transfer.
pub(super) const MAX_BULK_LEN: usize = 64 * 1024;
