//! Media-specific upgrade storage for `app-bootloader`.
//!
//! NOR and NAND are selected by the `nor` / `nand` features, independently of
//! the chip. The chip module only provides the QSPI bus and chip-select pin.

#[cfg(all(feature = "nor", feature = "nand"))]
compile_error!("select exactly one flash media: `nor` or `nand`");
#[cfg(not(any(feature = "nor", feature = "nand")))]
compile_error!("select a flash media: `nor` or `nand`");

#[cfg(feature = "nand")]
mod nand;
#[cfg(feature = "nor")]
mod nor;

#[cfg(feature = "nand")]
pub use nand::{Storage, init};
#[cfg(feature = "nor")]
pub use nor::{Storage, init};

use artinchip_hal::qspi::QspiConfig;

/// QSPI controller configuration for the selected media.
pub fn qspi_config() -> QspiConfig {
    #[cfg(feature = "nor")]
    {
        QspiConfig::nor_flash()
    }
    #[cfg(feature = "nand")]
    {
        QspiConfig::nand_flash()
    }
}
