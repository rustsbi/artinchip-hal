//! SPI NAND flash device driver.

mod config;
mod driver;
mod opcode;
mod status;

pub use config::{NandGeometry, NandInit, NandJedecId, NandQuadConfig, SpiNandConfig};
pub use driver::{RawNandFlash, SpiNandFlash};
pub use status::{Configuration, EccVerdict, Protection, Status};
