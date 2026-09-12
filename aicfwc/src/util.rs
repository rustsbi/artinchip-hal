//! Small shared helpers.

/// Round `value` up to a multiple of `align` (a power of two).
pub const fn round_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_up_rounds_to_multiples() {
        assert_eq!(round_up(0, 4), 0);
        assert_eq!(round_up(1, 4), 4);
        assert_eq!(round_up(4, 4), 4);
        assert_eq!(round_up(100, 32), 128);
        assert_eq!(round_up(2049, 2048), 4096);
    }
}
