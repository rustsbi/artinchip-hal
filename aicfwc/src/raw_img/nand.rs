use crate::checksum::*;
use anyhow::{Result, bail};

/// Build a SPI NAND boot image for candidate block 0: Page 0 is AICP and
/// Page 1 onward contains the AIC image.
pub fn build(aic: &[u8], page_size: usize) -> Result<Vec<u8>> {
    if page_size != 2048 && page_size != 4096 {
        bail!("SPI NAND page size must be 2048 or 4096 bytes");
    }

    let image_len = aic.len().next_multiple_of(page_size);
    let page_count = image_len / page_size;
    if page_count + 1 > 101 {
        bail!("SPI NAND image exceeds the 101-entry AICP page table");
    }
    let mut padded_aic = vec![0xff; image_len];
    padded_aic[..aic.len()].copy_from_slice(aic);

    let mut table = vec![0xff; page_size];
    table[..4].copy_from_slice(b"AICP");
    table[4..8].copy_from_slice(&((page_count + 1) as u32).to_le_bytes());
    table[8..10].copy_from_slice(&(page_size as u16).to_le_bytes());
    table[20..24].copy_from_slice(&0u32.to_le_bytes());
    table[36..40].copy_from_slice(&0u32.to_le_bytes());

    for page in 1..=page_count {
        let entry = 20 + page * 20;
        let page_start = (page - 1) * page_size;
        let checksum =
            checksum_for_words(&padded_aic[page_start..page_start + page_size], u32::MAX);
        table[entry..entry + 4].copy_from_slice(&(page as u32).to_le_bytes());
        table[entry + 4..entry + 16].fill(0xff);
        table[entry + 16..entry + 20].copy_from_slice(&checksum.to_le_bytes());
    }
    let checksum = checksum_for_words(&table, u32::MAX);
    table[36..40].copy_from_slice(&checksum.to_le_bytes());

    let mut image = table;
    image.extend_from_slice(&padded_aic);
    Ok(image)
}
