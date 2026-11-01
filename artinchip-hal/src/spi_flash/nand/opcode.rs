//! SPI NAND opcodes, feature-register addresses, and busy-wait timings.

/// Reset (`FFh`).
pub(super) const RESET: u8 = 0xFF;
/// Read JEDEC identification (`9Fh` + dummy byte).
pub(super) const READ_ID: u8 = 0x9F;
/// Get feature register (`0Fh` + register address).
pub(super) const GET_FEATURE: u8 = 0x0F;
/// Set feature register (`1Fh` + register address + value).
pub(super) const SET_FEATURE: u8 = 0x1F;
/// Write enable (`06h`).
pub(super) const WRITE_ENABLE: u8 = 0x06;
/// Page read into cache (`13h` + 3-byte row address).
pub(super) const PAGE_READ: u8 = 0x13;
/// Single-line read from cache (`03h` + 2-byte column address + dummy byte).
pub(super) const READ_CACHE: u8 = 0x03;
/// Block erase (`D8h` + 3-byte row address).
pub(super) const BLOCK_ERASE: u8 = 0xD8;
/// Single-line program data load (`02h` + 2-byte column address).
pub(super) const LOAD_PROGRAM_DATA: u8 = 0x02;
/// Program execute (`10h` + 3-byte row address).
pub(super) const PROGRAM_EXECUTE: u8 = 0x10;

/// Block-protection register (`A0h`).
pub(super) const REG_BLOCK_PROTECTION: u8 = 0xA0;
/// Configuration register (`B0h`).
pub(super) const REG_CONFIGURATION: u8 = 0xB0;
/// Status register (`C0h`).
pub(super) const REG_STATUS: u8 = 0xC0;

/// Polling parameters for an operation in microseconds.
#[derive(Clone, Copy, Debug)]
pub(super) struct Timings {
    pub(super) initial_us: u32,
    pub(super) poll_us: u32,
    pub(super) timeout_us: u32,
}

/// Reset (`tRST` <= 500 us).
pub(super) const RESET_TIMINGS: Timings = Timings {
    initial_us: 10,
    poll_us: 10,
    timeout_us: 500_000,
};

/// Array-to-cache page read (`tR` <= 100 us typical).
pub(super) const READ_TIMINGS: Timings = Timings {
    initial_us: 10,
    poll_us: 10,
    timeout_us: 400_000,
};

/// Page program (`tPROG` 300-400 us typical).
pub(super) const WRITE_TIMINGS: Timings = Timings {
    initial_us: 75,
    poll_us: 15,
    timeout_us: 400_000,
};

/// Block erase (`tBERS` 1-4 ms typical).
pub(super) const ERASE_TIMINGS: Timings = Timings {
    initial_us: 250,
    poll_us: 50,
    timeout_us: 400_000,
};

/// Feature-register write settle wait.
pub(super) const WRITE_REGISTER_TIMINGS: Timings = Timings {
    initial_us: 10,
    poll_us: 10,
    timeout_us: 10_000,
};
