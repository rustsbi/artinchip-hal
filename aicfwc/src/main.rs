//! AIC firmware converter: a host-side packer for ArtInChip D13x/D21x images.
//!
//! Two flows share this binary:
//!
//! * **Board presets** (`--board <name|path>` or `--toml <file>`): compose an
//!   `AIC.FW` upgrade image from the board's boot chain (`--bootloader`) or from
//!   the boot chain plus one application (`--app <package>`), optionally building
//!   the cargo packages first (`--build`). See the library docs for the preset
//!   schema and the design rationale.
//! * **Raw images** (`<binary> --raw-img --spi-nor|--spi-nand`): wrap an existing
//!   AIC container (or a bare PBP) into a raw flash image, unchanged from the
//!   original flow.

use std::{
    fs,
    path::{Path, PathBuf},
};

use aicfwc::app_img::{pbp, wrap_resource};
use aicfwc::pack::{self, TargetLayout};
use aicfwc::preset::{Preset, Selection};
use aicfwc::raw_img;
use anyhow::{Context, Result, bail};
use clap::{ArgAction, Parser};

/// AIC firmware converter.
#[derive(Parser, Debug)]
#[command(author, version, about = "ArtInChip image converter")]
struct Cli {
    /// The input binary file. Required unless `--board` or `--toml` is given.
    input: Option<PathBuf>,

    /// Board preset name (`boards/<name>.toml`) or a path to a preset file.
    #[arg(long, conflicts_with = "toml")]
    board: Option<String>,

    /// Preset file to pack from (same schema as `boards/`); escape hatch for an
    /// out-of-tree board.
    #[arg(long)]
    toml: Option<PathBuf>,

    /// Cargo package of the application to include (`[apps.<package>]` in the
    /// preset). A product image: boot chain plus application.
    #[arg(long, conflicts_with = "bootloader")]
    app: Option<String>,

    /// Package the boot chain alone (PBP updaters plus the bootloader image), with
    /// no application.
    #[arg(long, action = ArgAction::SetTrue)]
    bootloader: bool,

    /// Build every cargo package the image needs before packing.
    #[arg(long, action = ArgAction::SetTrue)]
    build: bool,

    /// Output directory for the image and its component pieces (default: next to
    /// the artifacts, `<target-dir>/<triple>/<profile>/<app>-out/`).
    #[arg(long)]
    out_dir: Option<PathBuf>,

    /// Cargo target directory (default: `CARGO_TARGET_DIR`, then the workspace
    /// root's `target/`).
    #[arg(long)]
    target_dir: Option<PathBuf>,

    /// Read and build the `debug` profile instead of the preset's profile.
    #[arg(long, conflicts_with = "profile")]
    debug: bool,

    /// Cargo profile to read and build (default: the preset's `[board] profile`,
    /// or `release`).
    #[arg(long)]
    profile: Option<String>,

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
    #[arg(long, default_value_t = 2048)]
    nand_page_size: usize,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    if cli.board.is_some() || cli.toml.is_some() {
        run_preset(cli)
    } else {
        run_raw(cli)
    }
}

/// Pack an image from a board preset.
fn run_preset(cli: Cli) -> Result<()> {
    if cli.input.is_some() {
        bail!("--board/--toml pack a preset and cannot be combined with a positional input binary");
    }
    if cli.raw_img || cli.spi_nor || cli.spi_nand {
        bail!(
            "--raw-img/--spi-nor/--spi-nand belong to the raw-image flow (positional input), not \
             to --board/--toml"
        );
    }
    if cli.nand_page_size != pack::NAND_PAGE_SIZE {
        bail!(
            "--nand-page-size belongs to the raw-image flow; the preset's SPI-NAND image is built \
             for the board's 2 KiB-page part"
        );
    }

    let preset = match (&cli.board, &cli.toml) {
        (Some(name), None) => Preset::load_named(name)?,
        (None, Some(path)) => Preset::load(path)?,
        _ => bail!("--board and --toml are mutually exclusive"),
    };
    let selection = match (&cli.app, cli.bootloader) {
        (Some(app), false) => Selection::App(app.clone()),
        (None, true) => Selection::Bootloader,
        (None, false) => bail!(
            "choose what to package: --app <package> for a product image, or --bootloader for the \
             boot chain only"
        ),
        (Some(_), true) => bail!("--app and --bootloader are mutually exclusive"),
    };
    let plan = preset.plan(&selection)?;

    let target_dir = pack::resolve_target_dir(cli.target_dir.as_deref())?;
    let profile = if cli.debug {
        "debug".to_string()
    } else if let Some(profile) = &cli.profile {
        profile.clone()
    } else {
        preset.board.profile.clone()
    };
    let layout = TargetLayout::new(target_dir, &preset.board.target, profile)?;

    if cli.build {
        pack::build_required(&plan, &layout)?;
    }
    pack::check_artifacts(&plan, &layout)?;
    let packed = pack::pack(&plan, &layout)?;

    let out_dir = match &cli.out_dir {
        Some(dir) => dir.clone(),
        None => layout.output_dir(&plan.stem),
    };
    pack::write(&plan, &packed, &out_dir)
}

/// Wrap one binary into a PBP/AIC and optionally a raw flash image.
fn run_raw(cli: Cli) -> Result<()> {
    if cli.build
        || cli.out_dir.is_some()
        || cli.target_dir.is_some()
        || cli.debug
        || cli.profile.is_some()
        || cli.app.is_some()
        || cli.bootloader
    {
        bail!(
            "--build/--out-dir/--target-dir/--debug/--profile/--app/--bootloader require --board or --toml"
        );
    }

    let input = cli
        .input
        .clone()
        .context("an input binary is required (or package from a preset with --board/--toml)")?;
    if cli.raw_img != (cli.spi_nor || cli.spi_nand) {
        bail!("--raw-img requires exactly one media option: --spi-nor or --spi-nand");
    }

    let bytes = fs::read(&input).with_context(|| format!("failed to read input {input:?}"))?;
    let output_dir = output_dir(&input)?;
    fs::create_dir_all(&output_dir)
        .with_context(|| format!("failed to create output directory {output_dir:?}"))?;

    let stem = file_stem(&input)?;

    let aic = if bytes.starts_with(b"AIC ") {
        bytes
    } else {
        let pbp = pbp::normalize(bytes)?;
        let pbp_path = output_dir.join(format!("{stem}.pbp"));
        fs::write(&pbp_path, &pbp).with_context(|| format!("failed to write {pbp_path:?}"))?;
        wrap_resource(&pbp)
    };

    if cli.raw_img {
        let image = if cli.spi_nor {
            raw_img::build_nor(&aic)
        } else {
            raw_img::nand::build(&aic, cli.nand_page_size)?
        };
        let image_path = output_dir.join(format!("{stem}.img"));
        fs::write(&image_path, image).with_context(|| format!("failed to write {image_path:?}"))?;
    }

    Ok(())
}

fn file_stem(path: &Path) -> Result<String> {
    path.file_stem()
        .and_then(|name| name.to_str())
        .map(str::to_string)
        .context("file name is not valid UTF-8")
}

fn output_dir(input: &Path) -> Result<PathBuf> {
    let parent = input.parent().context("input has no parent directory")?;
    Ok(parent.join(format!("{}-out", file_stem(input)?)))
}
