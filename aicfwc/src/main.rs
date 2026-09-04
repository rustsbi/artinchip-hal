use aicfwc::checksum::*;
use aicfwc::raw_img::*;
use anyhow::{Context, Result, bail};
use clap::{ArgAction, Parser};
use md5::{Digest, Md5};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// AIC firmware converter.
#[derive(Parser, Debug)]
#[command(author, version, about = "ArtInChip image converter")]
struct Cli {
    /// The input binary file.
    input: PathBuf,

    /// Generate a bootable raw image in addition to the repaired PBP file.
    #[arg(long, action = ArgAction::SetTrue)]
    raw_img: bool,

    /// Target SPI NOR media. Required with --raw-img.
    #[arg(long, action = ArgAction::SetTrue, conflicts_with = "spi_nand")]
    spi_nor: bool,

    /// Target SPI NAND media. Required with --raw-img.
    #[arg(long, action = ArgAction::SetTrue, conflicts_with = "spi_nor")]
    spi_nand: bool,

    /// SPI NAND page size in bytes (2048 or 4096).
    #[arg(long, default_value_t = 2048, requires = "spi_nand")]
    nand_page_size: usize,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    if cli.raw_img != (cli.spi_nor || cli.spi_nand) {
        bail!("--raw-img requires exactly one media option: --spi-nor or --spi-nand");
    }

    let input =
        fs::read(&cli.input).with_context(|| format!("failed to read input {:?}", cli.input))?;
    let pbp = if input.starts_with(b"PBP ") {
        repair_pbp(input)?
    } else {
        build_pbp(&input)
    };
    let output_dir = output_dir(&cli.input)?;
    fs::create_dir_all(&output_dir)
        .with_context(|| format!("failed to create output directory {output_dir:?}"))?;

    let stem = cli
        .input
        .file_stem()
        .and_then(|name| name.to_str())
        .context("input file name is not valid UTF-8")?;
    let pbp_path = output_dir.join(format!("{stem}.pbp"));
    fs::write(&pbp_path, &pbp).with_context(|| format!("failed to write {pbp_path:?}"))?;

    if cli.raw_img {
        let aic = pack_aic(&pbp);
        let image = if cli.spi_nor {
            nor::build(&aic)
        } else {
            nand::build(&aic, cli.nand_page_size)?
        };
        let image_path = output_dir.join(format!("{stem}.img"));
        fs::write(&image_path, image).with_context(|| format!("failed to write {image_path:?}"))?;
    }

    Ok(())
}

fn output_dir(input: &Path) -> Result<PathBuf> {
    let parent = input.parent().context("input has no parent directory")?;
    let stem = input
        .file_stem()
        .and_then(|name| name.to_str())
        .context("input file name is not valid UTF-8")?;
    Ok(parent.join(format!("{stem}-out")))
}

/// Validate and repair the checksum of a complete, externally produced PBP.
fn repair_pbp(mut pbp: Vec<u8>) -> Result<Vec<u8>> {
    if pbp.len() < 8 || &pbp[..4] != b"PBP " {
        bail!("input must be a PBP binary beginning with the 8-byte PBP header");
    }
    pbp.resize(pbp.len().next_multiple_of(4), 0);
    pbp[4..8].fill(0);
    let checksum = checksum_for_words(&pbp, u32::MAX);
    pbp[4..8].copy_from_slice(&checksum.to_le_bytes());
    debug_assert!(verify_checksum(&pbp, u32::MAX));
    Ok(pbp)
}

fn build_pbp(binary: &[u8]) -> Vec<u8> {
    let mut pbp = Vec::with_capacity((binary.len() + 11).next_multiple_of(4));
    pbp.extend_from_slice(b"PBP ");
    pbp.extend_from_slice(&[0; 4]);
    pbp.extend_from_slice(binary);
    pbp.resize(pbp.len().next_multiple_of(4), 0);
    let checksum = checksum_for_words(&pbp, u32::MAX);
    pbp[4..8].copy_from_slice(&checksum.to_le_bytes());
    pbp
}

/// Build an unsigned AIC image whose only resource is the repaired PBP.
fn pack_aic(pbp: &[u8]) -> Vec<u8> {
    const AIC_HEADER_SIZE: usize = 256;
    const RESOURCE_ALIGNMENT: usize = 32;
    const IMAGE_ALIGNMENT: usize = 256;
    const MD5_SIZE: usize = 16;

    let resource_len = pbp.len().next_multiple_of(RESOURCE_ALIGNMENT);
    let signed_len = (AIC_HEADER_SIZE + resource_len).next_multiple_of(IMAGE_ALIGNMENT);
    let image_len = signed_len + MD5_SIZE;
    let mut image = vec![0u8; image_len];

    image[..4].copy_from_slice(b"AIC ");
    image[8..12].copy_from_slice(&0x0001_0001u32.to_le_bytes());
    image[12..16].copy_from_slice(&(image_len as u32).to_le_bytes());
    image[40..44].copy_from_slice(&(signed_len as u32).to_le_bytes());
    image[44..48].copy_from_slice(&(MD5_SIZE as u32).to_le_bytes());
    image[72..76].copy_from_slice(&(AIC_HEADER_SIZE as u32).to_le_bytes());
    image[76..80].copy_from_slice(&(pbp.len() as u32).to_le_bytes());
    image[AIC_HEADER_SIZE..AIC_HEADER_SIZE + pbp.len()].copy_from_slice(pbp);

    let digest = Md5::digest(&image[8..signed_len]);
    image[signed_len..].copy_from_slice(&digest);
    let checksum = checksum_for_words(&image, u32::MAX);
    image[4..8].copy_from_slice(&checksum.to_le_bytes());
    debug_assert!(verify_checksum(&image, u32::MAX));
    image
}
