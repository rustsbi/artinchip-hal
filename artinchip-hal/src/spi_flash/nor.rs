//! SPI NOR flash device driver.

mod config;
mod driver;
mod opcode;
mod status;

pub use config::{
    NOR_BLOCK_SIZE_64K, NOR_PAGE_SIZE, NOR_READ_SIZE, NOR_SECTOR_SIZE, NorDeviceInfo, NorJedecId,
    NorQuadConfig, SpiNorConfig,
};
pub use driver::SpiNorFlash;
