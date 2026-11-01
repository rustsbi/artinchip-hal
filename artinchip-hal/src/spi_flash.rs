//! SPI NOR and SPI NAND flash device drivers.

mod error;
pub mod nand;
pub mod nor;

pub use error::FlashError;
pub use nand::*;
pub use nor::*;
