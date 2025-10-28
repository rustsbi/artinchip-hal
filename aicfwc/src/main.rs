use anyhow::{Context, Result, bail};
use clap::{ArgAction, Parser};
use md5::{Digest, Md5};
use std::{fs, io::Write, path::PathBuf};

/// AIC firmware converter.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// The input binary file.
    input: PathBuf,

    /// Output as PBP (Pre-Boot Program) format.
    ///
    /// PBP format:
    /// [0..=3]  = b"PBP ",
    /// [4..=7]  = checksum (u32 LE),
    /// [8..end] = original binary 4-byte aligned with zero paddings.
    #[arg(long = "pbp", action = ArgAction::SetTrue)]
    pbp: bool,

    /// Output file path.
    #[arg(short = 'o', long = "output")]
    output: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Read the input binary file
    let bin_data = fs::read(&cli.input)
        .with_context(|| format!("Failed to read input file {:?}", cli.input))?;

    // Currently only supports -pbp mode; error if not specified
    if !cli.pbp {
        bail!("Currently only supports -pbp preprocessing, please add -pbp flag");
    }

    let pbp_bytes = build_pbp(&bin_data)?;

    // Write the PBP file
    let mut f = fs::File::create(&cli.output)
        .with_context(|| format!("Failed to create output file {:?}", cli.output))?;
    f.write_all(&pbp_bytes)
        .with_context(|| format!("Failed to write output file {:?}", cli.output))?;

    // Pack into full image format and write .pk_pbp file
    let image_bytes = pack_pbp(&pbp_bytes)?;
    let pk_pbp_path = cli.output.with_extension("pk_pbp");
    let mut f_pk = fs::File::create(&pk_pbp_path)
        .with_context(|| format!("Failed to create .pk_pbp file {pk_pbp_path:?}"))?;
    f_pk.write_all(&image_bytes)
        .with_context(|| format!("Failed to write .pk_pbp file {pk_pbp_path:?}"))?;

    Ok(())
}

/// Build PBP format:
/// 0..=3: "PBP "
/// 4..=7: checksum (written back later)
/// 8..end: bin content + zero padding to 4-byte alignment
///
/// Checksum calculation:
///   Sum all words of the final PBP file as u32 little-endian (mod 2^32),
///   requiring the final sum == 0x0fffffff.
///   First calculate partial_sum with checksum position as 0,
///   then derive the checksum.
fn build_pbp(bin: &[u8]) -> Result<Vec<u8>> {
    // 1. Align bin to 4 bytes
    let mut aligned = bin.to_vec();
    while !aligned.len().is_multiple_of(4) {
        aligned.push(0);
    }

    // 2. Assemble a placeholder PBP: magic + checksum placeholder + data
    let total_len = aligned.len();
    let mut out = Vec::with_capacity(total_len);

    // Data
    out.extend_from_slice(&aligned);

    // 3. Calculate the current 32-bit sum (with placeholder checksum=0)
    let mut sum: u64 = 0;
    for chunk in out.chunks_exact(4) {
        let word = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        sum = (sum + word as u64) & 0xffff_ffff;
    }

    // 4. Derive the actual checksum
    let target: u32 = 0xffff_ffff;
    let checksum = target.wrapping_sub(sum as u32);

    // 5. Write back checksum (little-endian)
    out[4..8].copy_from_slice(&checksum.to_le_bytes());

    // 6. Optional: Assert correctness (for debug)
    debug_assert!(verify_checksum(&out, target));

    Ok(out)
}

/// Verify if the 32-bit sum of the final file is the target
fn verify_checksum(buf: &[u8], target: u32) -> bool {
    if !buf.len().is_multiple_of(4) {
        return false;
    }
    let mut sum: u64 = 0;
    for chunk in buf.chunks_exact(4) {
        let word = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        sum = (sum + word as u64) & 0xffff_ffff;
    }
    (sum as u32) == target
}

/// Pack PBP data into a complete Artinchip boot image format
/// Format: HEAD1 + HEAD2 + DATA1 (PBP) + DATA2 (empty) + SIGN (MD5)
fn pack_pbp(pbp_data: &[u8]) -> Result<Vec<u8>> {
    // Constants
    const HEAD1_SIZE: usize = 8;
    const HEAD2_SIZE: usize = 248;
    const SIGN_SIZE: usize = 16; // MD5
    const ALIGNMENT: usize = 256;

    // Calculate sizes
    let pbp_total_len = pbp_data.len(); // PBP header + content
    let data1_len = pbp_total_len.div_ceil(ALIGNMENT) * ALIGNMENT; // Align to 256 bytes
    let data2_len = 0; // TODO: Implement DATA2
    let signed_area_len = HEAD1_SIZE + HEAD2_SIZE + data1_len + data2_len;
    let total_len = signed_area_len + SIGN_SIZE;

    // Build HEAD1
    let mut head1 = vec![0u8; HEAD1_SIZE];
    head1[0..4].copy_from_slice(b"AIC "); // Magic
    // Checksum will be calculated later

    // Build HEAD2
    let mut head2 = vec![0u8; HEAD2_SIZE];
    // Header version: 0x00010001
    head2[0..4].copy_from_slice(&0x00010001u32.to_le_bytes());
    // Image length
    head2[4..8].copy_from_slice(&(total_len as u32).to_le_bytes());
    // Firmware version: 0.0.0 (anti_rollback=0, revision=0, minor=0, major=0)
    // Already 0
    // Loader length: 0
    // Load address: 0
    // Entry point: 0
    // Sign algo: 0 (no signature)
    // Enc algo: 0
    // Sign result offset
    head2[32..36].copy_from_slice(&(signed_area_len as u32).to_le_bytes());
    // Sign result length: 16
    head2[36..40].copy_from_slice(&16u32.to_le_bytes());
    // Other offsets: 0
    // PBP offset: 256
    head2[64..68].copy_from_slice(&256u32.to_le_bytes());
    // PBP length: pbp_data.len()
    head2[68..72].copy_from_slice(&(pbp_data.len() as u32).to_le_bytes());
    // Padding: already 0

    // Build DATA1: PBP + padding
    let mut data1 = vec![0u8; data1_len];
    data1[0..pbp_data.len()].copy_from_slice(pbp_data);

    // TODO DATA2: empty

    // SIGN: MD5 of HEAD2 + DATA1 + DATA2
    let signed_data = [head2.as_slice(), data1.as_slice()].concat();
    let mut hasher = Md5::new();
    hasher.update(&signed_data);
    let sign = hasher.finalize().to_vec();

    let signed_area = [head1.as_slice(), head2.as_slice(), data1.as_slice()].concat();

    let mut result = vec![0u8; total_len];
    result[0..signed_area_len].copy_from_slice(&signed_area);
    result[signed_area_len..].copy_from_slice(&sign);

    // Calculate HEAD1 checksum
    let mut sum: u64 = 0;
    for chunk in result.chunks_exact(4) {
        let word = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        sum = (sum + word as u64) & 0xffff_ffff;
    }

    let target: u32 = 0xffff_ffff;
    let head1_checksum = target.wrapping_sub(sum as u32);

    result[4..8].copy_from_slice(&head1_checksum.to_le_bytes());

    Ok(result)
}
