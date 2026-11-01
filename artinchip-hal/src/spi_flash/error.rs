//! Unified error type for SPI NOR and SPI NAND flash drivers.

use embedded_storage::nor_flash::{NorFlashError, NorFlashErrorKind};

/// Error returned by SPI NOR and SPI NAND flash operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlashError<E> {
    /// Underlying SPI bus returned an error.
    Spi(E),
    /// Part stayed busy past the operation timeout.
    Timeout,
    /// Offset or length is not aligned to the required page/erase unit.
    NotAligned,
    /// Slice length does not match the expected page size.
    InvalidLength,
    /// Offset, page, or block falls outside the device capacity.
    OutOfBounds,
    /// Read-back verification did not match programmed data.
    VerifyFailed,
    /// `WRITE ENABLE` did not set the write-enable latch.
    WriteEnableFailed,
    /// On-die ECC reported an uncorrectable page error.
    UncorrectableEcc,
    /// Hardware status reported an erase failure.
    EraseFailed,
    /// Hardware status reported a page-program failure.
    ProgramFailed,
    /// Requested feature (such as quad mode) is not supported by the part.
    NotSupported,
}

impl<E> FlashError<E> {
    /// Map onto `embedded_storage::nor_flash::NorFlashErrorKind`.
    pub const fn nor_flash_kind(&self) -> NorFlashErrorKind {
        match self {
            Self::NotAligned | Self::InvalidLength => NorFlashErrorKind::NotAligned,
            Self::OutOfBounds => NorFlashErrorKind::OutOfBounds,
            _ => NorFlashErrorKind::Other,
        }
    }
}

impl<E: core::fmt::Debug> NorFlashError for FlashError<E> {
    fn kind(&self) -> NorFlashErrorKind {
        self.nor_flash_kind()
    }
}
