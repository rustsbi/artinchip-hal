//! DRAM bring-up errors.

use super::config::DramType;
use super::register::PhyStatus;

/// DRAM bring-up error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DramError {
    /// The parameter set's generation and clock are not covered by the configuration
    /// profiles; the controller was left alone.
    UnsupportedConfig {
        /// The requested DRAM generation.
        dram_type: DramType,
        /// The requested DRAM clock in Hz.
        freq: u32,
    },
    /// The controller never reported its DRAM initialisation finished.
    ControllerInitTimeout,
    /// A controller software command never completed.
    CommandTimeout,
    /// The PHY never reported a request done.
    PhyInitTimeout,
    /// The PHY finished but its init-done bit is clear.
    PhyNotReady(PhyStatus),
    /// The PHY reported a training error.
    PhyTrainingError(PhyStatus),
    /// A probe write and read-back disagree; the DRAM window is not trustworthy.
    ProbeMismatch {
        /// The probed address.
        address: usize,
        /// The value that was written.
        expected: u32,
        /// The value that was read back.
        actual: u32,
    },
}

impl core::fmt::Display for DramError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnsupportedConfig { dram_type, freq } => {
                write!(
                    f,
                    "unsupported configuration: {} @ {} MHz",
                    dram_type.name(),
                    freq / 1_000_000
                )
            }
            Self::ControllerInitTimeout => write!(f, "controller init timeout"),
            Self::CommandTimeout => write!(f, "controller command timeout"),
            Self::PhyInitTimeout => write!(f, "PHY init timeout"),
            Self::PhyNotReady(status) => {
                write!(f, "PHY not ready (PGSR0: {:#010X})", status.raw())
            }
            Self::PhyTrainingError(status) => {
                write!(
                    f,
                    "PHY training error (PGSR0: {:#010X}, code: {:#04X})",
                    status.raw(),
                    status.training_error()
                )
            }
            Self::ProbeMismatch {
                address,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "probe mismatch at {address:#010X}: expected {expected:#010X}, got {actual:#010X}"
                )
            }
        }
    }
}

impl core::error::Error for DramError {}
