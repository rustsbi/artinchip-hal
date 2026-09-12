//! XSPI error types.

use super::register::CsSel;

/// PSRAM bring-up error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PsramError {
    /// A LUT/FIFO transaction never completed.
    Timeout,
    /// No DLL phase read the XIP window back correctly for this chip select.
    TrainingFailed {
        /// Chip select that could not be trained.
        chip_select: CsSel,
    },
}
