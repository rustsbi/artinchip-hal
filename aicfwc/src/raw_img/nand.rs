use crate::checksum::*;
use crate::util::round_up;
use anyhow::{Result, bail};

/// Page-table entries; each lists two image pages (up to 202 pages).
const PAGE_TABLE_MAX_ENTRY: usize = 101;

/// Pages per block for both supported geometries (64).
const PAGES_PER_BLOCK: u8 = 64;

/// Wrap an AIC image for a SPI-NAND boot block: page 0 is the `AICP` page
/// table, pages 1.. carry the image.
///
/// Entry `i` lists physical pages `i` and `101 + i` with the checksum of the
/// image words each holds; the boot ROM verifies them. Entry 0 describes the
/// table page itself and carries the table's own `~sum`.
pub fn build(aic: &[u8], page_size: usize) -> Result<Vec<u8>> {
    if page_size != 2048 && page_size != 4096 {
        bail!("SPI NAND page size must be 2048 or 4096 bytes");
    }

    let image_len = round_up(aic.len(), page_size);
    let page_cnt = image_len / page_size;
    let last_page = page_cnt + 1;
    if last_page > 2 * PAGE_TABLE_MAX_ENTRY {
        bail!(
            "SPI NAND image exceeds the {}-page AICP table",
            2 * PAGE_TABLE_MAX_ENTRY
        );
    }

    // Sum of the image words of page `index` (0-based), with the words past the
    // image's end reading as zero.
    let page_sum = |index: usize| -> u32 {
        let start = index * page_size;
        let mut sum = 0u32;
        if start < aic.len() {
            let end = (start + page_size).min(aic.len());
            let mut offset = start;
            while offset < end {
                let n = (end - offset).min(4);
                let mut word = [0u8; 4];
                word[..n].copy_from_slice(&aic[offset..offset + n]);
                sum = sum.wrapping_add(u32::from_le_bytes(word));
                offset += n;
            }
        }
        sum
    };
    // A page's checksum in the same convention as the AIC header: `~sum`.
    let page_checksum = |index: usize| u32::MAX.wrapping_sub(page_sum(index));

    let mut table = vec![0xff; page_size];
    table[..4].copy_from_slice(b"AICP");
    table[4..8].copy_from_slice(&((page_cnt + 1) as u32).to_le_bytes());
    table[8..10].copy_from_slice(&(page_size as u16).to_le_bytes());
    table[10] = PAGES_PER_BLOCK;
    table[11] = 0xff; // blocks: unknown here
    table[12] = 1; // planes
    // Entry 0 is the table's own page: page 0, and a checksum field that holds
    // the table's own checksum (patched in below).
    table[20..24].copy_from_slice(&0u32.to_le_bytes());
    table[36..40].copy_from_slice(&0u32.to_le_bytes());

    for i in 0..PAGE_TABLE_MAX_ENTRY {
        let entry = 20 + i * 20;

        // The two physical pages this entry lists.
        if i <= last_page {
            table[entry..entry + 4].copy_from_slice(&(i as u32).to_le_bytes());
        } else {
            table[entry..entry + 4].copy_from_slice(&u32::MAX.to_le_bytes());
        }

        let second = PAGE_TABLE_MAX_ENTRY + i;
        if second <= last_page {
            table[entry + 4..entry + 8].copy_from_slice(&(second as u32).to_le_bytes());
            table[entry + 8..entry + 12].copy_from_slice(&page_checksum(second - 1).to_le_bytes());
            table[entry + 12..entry + 16].copy_from_slice(&u32::MAX.to_le_bytes());
        } else {
            table[entry + 4..entry + 16].fill(0xff);
        }

        // The first column's checksum: the image page this physical page holds
        // (`i - 1`), or `~0` for the table's own page and for pages past the
        // image.
        if i == 0 {
            // The table's own page; the checksum field is patched in below.
            table[entry + 16..entry + 20].copy_from_slice(&0u32.to_le_bytes());
        } else {
            table[entry + 16..entry + 20].copy_from_slice(&page_checksum(i - 1).to_le_bytes());
        }
    }
    let checksum = checksum_for_words(&table, u32::MAX);
    table[36..40].copy_from_slice(&checksum.to_le_bytes());

    let mut image = table;
    // The last page is padded with `0xff` like the rest of a freshly erased
    // block; the checksums above already described the words the image has.
    let mut padded = vec![0xff; image_len];
    padded[..aic.len()].copy_from_slice(aic);
    image.extend_from_slice(&padded);
    Ok(image)
}
