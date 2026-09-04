pub fn checksum_for_words(buf: &[u8], target: u32) -> u32 {
    let sum = buf.as_chunks::<4>().0.iter().fold(0u32, |sum, word| {
        sum.wrapping_add(u32::from_le_bytes(*word))
    });
    target.wrapping_sub(sum)
}

pub fn verify_checksum(buf: &[u8], target: u32) -> bool {
    buf.len().is_multiple_of(4)
        && buf.as_chunks::<4>().0.iter().fold(0u32, |sum, word| {
            sum.wrapping_add(u32::from_le_bytes(*word))
        }) == target
}
