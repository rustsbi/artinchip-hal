//! SPI NOR status register bit definitions.

/// Status register 1 bit 0: erase/program/write in progress (`BUSY`).
pub(super) const BUSY: u8 = 1 << 0;
/// Status register 1 bit 1: write-enable latch (`WEL`).
pub(super) const WRITE_ENABLE_LATCH: u8 = 1 << 1;
/// Status register 1 bit 6: quad-enable (`QE`) on Macronix/ISSI-style parts.
pub(super) const QE_SR1: u8 = 1 << 6;
/// Status register 2 bit 1: quad-enable (`QE`) on Winbond/GigaDevice parts.
pub(super) const QE_SR2: u8 = 1 << 1;
/// Status register 1 block-protection mask (`BP0..BP2`, `TB`, `SEC`, `SRP0`).
pub(super) const SR1_PROTECT_MASK: u8 = 0xFC;
/// Status register 2 protection mask (`SRP1`, `CMP`, `SUS` / upper bits).
pub(super) const SR2_PROTECT_MASK: u8 = 0xE0;
