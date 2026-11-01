//! Raw flash boot image builders for SPI-NOR and SPI-NAND.

pub mod nand;

/// SPI-NOR stores the AIC container directly at offset 0.
pub fn build_nor(aic: &[u8]) -> Vec<u8> {
    aic.to_vec()
}
