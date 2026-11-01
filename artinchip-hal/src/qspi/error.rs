//! QSPI error types.

/// QSPI bus error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QspiError {
    /// Timeout waiting for hardware.
    Timeout,
    /// The requested mode is not implemented by this controller revision.
    ///
    /// The D21x QSPI is the v1.0 controller, which has no QPI mode.
    Unsupported,
}

impl embedded_hal::spi::Error for QspiError {
    fn kind(&self) -> embedded_hal::spi::ErrorKind {
        embedded_hal::spi::ErrorKind::Other
    }
}
