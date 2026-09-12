//! PBP resource helpers shared by the CLI and the image manifest.
//!
//! A PBP file is a `"PBP "` header (8 bytes: magic + 32-bit ones-complement word
//! sum) followed by the executable. The word sum over the whole file must reach
//! `0xFFFF_FFFF`.

use anyhow::{Result, bail};

use crate::checksum::{checksum_for_words, verify_checksum};
use crate::util::round_up;

/// Wrap a raw binary into a PBP (`"PBP "` header + checksum).
pub fn build(binary: &[u8]) -> Vec<u8> {
    let mut pbp = Vec::with_capacity(round_up(binary.len() + 11, 4));
    pbp.extend_from_slice(b"PBP ");
    pbp.extend_from_slice(&[0; 4]);
    pbp.extend_from_slice(binary);
    pbp.resize(round_up(pbp.len(), 4), 0);
    let checksum = checksum_for_words(&pbp, u32::MAX);
    pbp[4..8].copy_from_slice(&checksum.to_le_bytes());
    pbp
}

/// Validate and repair the checksum of an already built PBP.
pub fn repair(mut pbp: Vec<u8>) -> Result<Vec<u8>> {
    if pbp.len() < 8 || &pbp[..4] != b"PBP " {
        bail!("input must be a PBP binary beginning with the 8-byte PBP header");
    }
    pbp.resize(round_up(pbp.len(), 4), 0);
    pbp[4..8].fill(0);
    let checksum = checksum_for_words(&pbp, u32::MAX);
    pbp[4..8].copy_from_slice(&checksum.to_le_bytes());
    debug_assert!(verify_checksum(&pbp, u32::MAX));
    Ok(pbp)
}

/// Normalize a file into a valid PBP: keep an existing PBP (repairing its
/// checksum) or build one around a raw binary.
pub fn normalize(bytes: Vec<u8>) -> Result<Vec<u8>> {
    if bytes.starts_with(b"PBP ") {
        repair(bytes)
    } else {
        Ok(build(&bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_and_verifies() {
        let pbp = build(&[1, 2, 3, 4, 5]);
        assert!(pbp.starts_with(b"PBP "));
        assert!(verify_checksum(&pbp, u32::MAX));
        assert!(repair(pbp).is_ok());
    }
}
