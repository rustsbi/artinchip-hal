//! Board preset schema types and lookup helpers.

use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};

use super::{builtin_boards_dir, known_presets};

/// Erase/sector size of the SPI-NOR parts the D13x boards carry.
pub const NOR_ERASE_SIZE: u32 = 4 * 1024;
/// Erase-block size of the SPI-NAND parts the D21x boards carry.
pub const NAND_ERASE_SIZE: u32 = 128 * 1024;
/// Alignment when a concatenated (`with_ext`) loader is appended.
pub const EXT_ALIGNMENT: usize = 512;
/// Maximum application name length in bytes in the AIC header.
pub const NAME_MAX: usize = 32;

/// A chip a preset can describe.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chip {
    /// D13x: RV32 E907, boots from SPI-NOR.
    D13x,
    /// D21x: RV64 C906, boots from SPI-NAND.
    D21x,
}

impl Chip {
    /// Both supported chips.
    pub const ALL: [Self; 2] = [Self::D13x, Self::D21x];

    /// Parse the chip name used in presets (`d13x` | `d21x`).
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "d13x" => Ok(Self::D13x),
            "d21x" => Ok(Self::D21x),
            other => bail!("unsupported chip {other:?} (expected d13x | d21x)"),
        }
    }

    /// The preset name of the chip.
    pub fn name(self) -> &'static str {
        match self {
            Self::D13x => "d13x",
            Self::D21x => "d21x",
        }
    }

    /// The bare-metal cargo target triple the chip's images are built for.
    pub fn default_target(self) -> &'static str {
        match self {
            Self::D13x => "riscv32imafc-unknown-none-elf",
            Self::D21x => "riscv64gc-unknown-none-elf",
        }
    }

    /// The boot media the chip's boards carry.
    pub fn default_media(self) -> Media {
        match self {
            Self::D13x => Media::SpiNor,
            Self::D21x => Media::SpiNand,
        }
    }

    /// The media's erase block, which partition offsets and sizes must align to.
    pub fn erase_size(self) -> u32 {
        self.default_media().erase_size()
    }
}

/// The on-flash media of a board.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Media {
    /// SPI-NOR: 4 KiB sectors, BootROM boots a bare `"AIC "` container from offset 0.
    SpiNor,
    /// SPI-NAND: 2 KiB pages in 128 KiB blocks, BootROM reads an `AICP` table at offset 0.
    SpiNand,
}

impl Media {
    /// Parse the media name used in presets (`spi-nor` | `spi-nand`).
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "spi-nor" => Ok(Self::SpiNor),
            "spi-nand" => Ok(Self::SpiNand),
            other => bail!("unsupported flash media {other:?} (expected spi-nor | spi-nand)"),
        }
    }

    /// The preset name of the media.
    pub fn name(self) -> &'static str {
        match self {
            Self::SpiNor => "spi-nor",
            Self::SpiNand => "spi-nand",
        }
    }

    /// The media's erase block.
    pub fn erase_size(self) -> u32 {
        match self {
            Self::SpiNor => NOR_ERASE_SIZE,
            Self::SpiNand => NAND_ERASE_SIZE,
        }
    }
}

/// The `[board]` section: what the image is built for and where artifacts come from.
#[derive(Clone, Debug)]
pub struct BoardInfo {
    pub name: String,
    pub chip: Chip,
    pub target: String,
    pub profile: String,
}

/// The `[image]` section: the `AIC.FW` header metadata.
#[derive(Clone, Debug)]
pub struct ImageInfo {
    pub platform: String,
    pub product: String,
    pub version: String,
    pub media: Media,
    pub media_dev_id: u32,
    pub media_id: String,
    /// Application name a normal boot jumps to; empty boots the first valid image.
    pub boot_app: String,
}

/// One `[[partition]]` entry: the on-flash layout owned by this project.
#[derive(Clone, Debug)]
pub struct PartitionSpec {
    pub name: String,
    pub offset: u32,
    pub size: u32,
}

impl PartitionSpec {
    /// The partition name without the `attr;` suffix the meta format allows.
    pub(crate) fn name_of(reference: &str) -> &str {
        reference.split(';').next().unwrap_or(reference).trim()
    }
}

/// How a component's payload is turned into image data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentKind {
    /// Application ELF -> AIC loader container (load address/entry from the ELF).
    App,
    /// Raw binary or PBP file -> AIC container with a PBP resource.
    Pbp,
    /// File bytes used as-is.
    Raw,
}

impl ComponentKind {
    pub(crate) fn parse(value: &str) -> Result<Self> {
        match value {
            "app" => Ok(Self::App),
            "pbp" => Ok(Self::Pbp),
            "raw" => Ok(Self::Raw),
            other => bail!("unknown component kind {other:?} (expected app | pbp | raw)"),
        }
    }
}

/// One `[[component]]` entry: a boot-chain piece of the image.
#[derive(Clone, Debug)]
pub struct ComponentSpec {
    pub kind: ComponentKind,
    /// AIC/header name matched by the runtime boot chain (`psram`, `ddr`, `bootloader`).
    pub name: String,
    /// Cargo package that produces the payload. Exactly one of `package` and `file` is set.
    pub package: Option<String>,
    /// Artifact file name when it differs from the package name.
    pub bin: Option<String>,
    /// Explicit payload path, resolved relative to the preset's directory.
    pub file: Option<PathBuf>,
    /// Partition the target role is burned to; empty for an updater-only entry.
    pub partition: String,
    pub attr: String,
    pub filename: String,
    /// Target-role overrides for dual-role entries (`updater = true` + `partition`).
    pub target_attr: Option<String>,
    pub target_filename: Option<String>,
    pub updater: bool,
    /// Cargo features for the component's own `package`.
    pub features: Vec<String>,
    /// Cargo features for `pbp_package`; defaults to `features` when omitted.
    pub pbp_features: Vec<String>,
    pub ram: Option<u32>,
    /// Explicit PBP path (escape hatch); non-cargo PBPs only.
    pub pbp: Option<PathBuf>,
    /// Cargo package that produces the PBP resource (e.g. `pbp-common`).
    pub pbp_package: Option<String>,
    pub private: Option<PathBuf>,
    pub load_address: Option<u32>,
    pub entry_point: Option<u32>,
    pub entry_offset: Option<u32>,
    pub header_version: Option<u32>,
    pub with_ext: bool,
    /// Bytes reserved between the PBP image and the concatenated loader.
    pub ext_reserve: usize,
}

/// One `[apps.<package>]` table: the application-specific half of an image.
#[derive(Clone, Debug)]
pub struct AppSpec {
    /// Cargo package name (the table key).
    pub package: String,
    /// AIC header name the bootloader matches (`blinky`).
    pub name: String,
    pub partition: String,
    pub filename: String,
    pub attr: String,
    pub features: Vec<String>,
    /// Link base (`APP_BASE`) the application's `build.rs` links it at.
    pub load_address: Option<u32>,
    pub entry_offset: Option<u32>,
}

impl AppSpec {
    /// Convert `[apps.<package>]` into a target-only [`ComponentSpec`].
    pub(crate) fn component(&self) -> ComponentSpec {
        ComponentSpec {
            kind: ComponentKind::App,
            name: self.name.clone(),
            package: Some(self.package.clone()),
            bin: None,
            file: None,
            partition: self.partition.clone(),
            attr: self.attr.clone(),
            filename: self.filename.clone(),
            target_attr: None,
            target_filename: None,
            updater: false,
            features: self.features.clone(),
            pbp_features: Vec::new(),
            ram: None,
            pbp: None,
            pbp_package: None,
            private: None,
            load_address: self.load_address,
            entry_point: None,
            entry_offset: self.entry_offset,
            header_version: None,
            with_ext: false,
            ext_reserve: 0,
        }
    }
}

/// A parsed board preset.
#[derive(Clone, Debug)]
pub struct Preset {
    /// File the preset was read from; relative paths resolve against its directory.
    pub path: PathBuf,
    pub board: BoardInfo,
    pub image: ImageInfo,
    pub partitions: Vec<PartitionSpec>,
    pub components: Vec<ComponentSpec>,
    pub apps: BTreeMap<String, AppSpec>,
}

impl Preset {
    /// Read and parse a preset file.
    pub fn load(path: &Path) -> Result<Self> {
        let text = fs::read_to_string(path)
            .with_context(|| format!("failed to read board preset {path:?}"))?;
        Self::parse(&text, path).with_context(|| format!("invalid board preset {path:?}"))
    }

    /// The directory relative paths in the preset resolve against.
    pub fn dir(&self) -> &Path {
        self.path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."))
    }

    /// Resolve a `--board` argument: an explicit file path if it names one,
    /// otherwise `boards/<name>.toml` below the working tree.
    pub fn load_named(name: &str) -> Result<Self> {
        let direct = Path::new(name);
        if direct.is_file() {
            return Self::load(direct);
        }

        let boards_dir = builtin_boards_dir().with_context(|| {
            format!(
                "unknown board preset {name:?}: no `boards/` directory found above the current \
                 directory; name a preset file with a path instead"
            )
        })?;
        let file = if name.ends_with(".toml") {
            boards_dir.join(name)
        } else {
            boards_dir.join(format!("{name}.toml"))
        };
        if !file.is_file() {
            let known: Vec<String> = known_presets(&boards_dir)
                .into_iter()
                .map(|(name, _)| name)
                .collect();
            bail!(
                "unknown board preset {name:?}: {file:?} does not exist (available: {})",
                if known.is_empty() {
                    "none".to_string()
                } else {
                    known.join(", ")
                }
            );
        }
        Self::load(&file)
    }

    /// Find and validate the default preset describing `chip`.
    pub fn for_chip(boards_dir: &Path, chip: Chip) -> Result<Self> {
        let mut candidates = known_presets(boards_dir);
        candidates.sort();
        for (_, path) in candidates {
            let preset = Self::load(&path)?;
            if preset.board.chip == chip {
                preset.validate()?;
                return Ok(preset);
            }
        }
        bail!(
            "no board preset for chip {} in {boards_dir:?}; expected boards/{}.toml",
            chip.name(),
            match chip {
                Chip::D13x => "d13x-nor",
                Chip::D21x => "d21x-nand",
            }
        )
    }

    /// The application declared by `[apps.<package>]`.
    pub fn app(&self, package: &str) -> Result<&AppSpec> {
        self.apps.get(package).with_context(|| {
            format!(
                "board preset {:?} declares no `[apps.{package}]` (declared: {})",
                self.board.name,
                if self.apps.is_empty() {
                    "none".to_string()
                } else {
                    self.apps.keys().cloned().collect::<Vec<_>>().join(", ")
                }
            )
        })
    }

    /// The `APP_BASE` of the boot-chain application image (the bootloader).
    pub fn boot_chain_app_base(&self) -> Result<u32> {
        self.components
            .iter()
            .find(|component| component.kind == ComponentKind::App)
            .and_then(|component| component.load_address)
            .with_context(|| {
                format!(
                    "board preset {:?} declares no boot-chain `kind = \"app\"` component with a \
                     `load_address`",
                    self.board.name
                )
            })
    }
}
