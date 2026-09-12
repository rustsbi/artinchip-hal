use aicfwc::app_img::{ComponentKind, ImageManifest, pbp, wrap_resource};
use aicfwc::raw_img::*;
use anyhow::{Context, Result, bail};
use clap::{ArgAction, Parser};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// AIC firmware converter.
#[derive(Parser, Debug)]
#[command(author, version, about = "ArtInChip image converter")]
struct Cli {
    /// The input binary file. Required unless `--toml` is given.
    input: Option<PathBuf>,

    /// Build a packed `AIC.FW` image from a TOML manifest. May name the manifest
    /// file, or a directory holding exactly one, so an image can be built from
    /// where it lives.
    #[arg(long)]
    toml: Option<PathBuf>,

    /// Build each component's `build_package` first (`cargo build`), then pack.
    #[arg(long, action = ArgAction::SetTrue, requires = "toml")]
    build: bool,

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
    if let Some(manifest) = cli.toml.clone() {
        if cli.input.is_some() {
            bail!("--toml cannot be combined with a positional input file");
        }
        let manifest = resolve_manifest(&manifest)?;
        return build_from_toml(&manifest, cli.build);
    }

    let input = cli
        .input
        .clone()
        .context("an input binary is required (or use --toml)")?;
    if cli.raw_img != (cli.spi_nor || cli.spi_nand) {
        bail!("--raw-img requires exactly one media option: --spi-nor or --spi-nand");
    }

    let bytes = fs::read(&input).with_context(|| format!("failed to read input {input:?}"))?;
    let pbp = pbp::normalize(bytes)?;
    let output_dir = output_dir(&input)?;
    fs::create_dir_all(&output_dir)
        .with_context(|| format!("failed to create output directory {output_dir:?}"))?;

    let stem = file_stem(&input)?;
    let pbp_path = output_dir.join(format!("{stem}.pbp"));
    fs::write(&pbp_path, &pbp).with_context(|| format!("failed to write {pbp_path:?}"))?;

    if cli.raw_img {
        let aic = wrap_resource(&pbp);
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

/// Resolve `--toml` to the manifest file it names.
///
/// A directory is accepted as well as a file, the manifest inside it picked when
/// unambiguous, so `cargo run -p aicfwc -- --toml <dir>` works. `Cargo.toml` is
/// never a candidate.
fn resolve_manifest(path: &Path) -> Result<PathBuf> {
    if path.is_file() {
        return Ok(path.to_path_buf());
    }
    if !path.is_dir() {
        bail!("{path:?} is neither a manifest file nor a directory");
    }

    let mut candidates: Vec<PathBuf> = fs::read_dir(path)
        .with_context(|| format!("failed to read directory {path:?}"))?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|entry| {
            entry.is_file()
                && entry
                    .extension()
                    .is_some_and(|extension| extension == "toml")
                && entry.file_name().is_some_and(|name| name != "Cargo.toml")
        })
        .collect();
    candidates.sort();

    match candidates.as_slice() {
        [only] => Ok(only.clone()),
        [] => bail!("no manifest (*.toml other than Cargo.toml) in {path:?}"),
        many => bail!(
            "{path:?} holds {} candidate manifests ({}); name one with --toml",
            many.len(),
            many.iter()
                .map(|candidate| candidate.display().to_string())
                .collect::<Vec<_>>()
                .join(", "),
        ),
    }
}

/// Build a packed `AIC.FW` image from a TOML manifest.
fn build_from_toml(manifest_path: &Path, build_packages: bool) -> Result<()> {
    let manifest = ImageManifest::load(manifest_path)?;
    let base_dir = manifest_path.parent().unwrap_or_else(|| Path::new("."));
    if build_packages {
        manifest.build_packages(base_dir)?;
    }
    let image = manifest.build(base_dir)?;
    let bytes = image.build();

    let stem = file_stem(manifest_path)?;
    // Emit next to the component binaries (cargo's target directory), like the
    // `--raw-img` flow, instead of beside the manifest.
    let output_dir = component_dir(&manifest, base_dir)
        .unwrap_or_else(|| base_dir.to_path_buf())
        .join(format!("{stem}-out"));
    fs::create_dir_all(&output_dir)
        .with_context(|| format!("failed to create output directory {output_dir:?}"))?;

    let image_path = output_dir.join(format!("{stem}.img"));
    fs::write(&image_path, &bytes).with_context(|| format!("failed to write {image_path:?}"))?;
    println!(
        "image  : {image_path:?} ({} bytes, {} components)",
        bytes.len(),
        image.components.len()
    );

    Ok(())
}

fn file_stem(path: &Path) -> Result<String> {
    path.file_stem()
        .and_then(|name| name.to_str())
        .map(str::to_string)
        .context("file name is not valid UTF-8")
}

/// Directory holding the first `app` component, used as the output location.
///
/// Manifests point `file` at the cargo target directory
/// (`../../../target/<triple>/<profile>/<name>`), so `<stem>-out/` lands there
/// too, the same place the PBP flow writes to.
fn component_dir(manifest: &ImageManifest, base_dir: &Path) -> Option<PathBuf> {
    manifest
        .components
        .iter()
        .find(|component| component.kind == ComponentKind::App)
        .and_then(|component| {
            base_dir
                .join(&component.file)
                .parent()
                .map(Path::to_path_buf)
        })
}

fn output_dir(input: &Path) -> Result<PathBuf> {
    let parent = input.parent().context("input has no parent directory")?;
    Ok(parent.join(format!("{}-out", file_stem(input)?)))
}
