//! TOML image manifest → [`FwImage`] conversion.
//!
//! A component with `updater = true` and a non-empty `partition` is written once
//! but expands to **two** meta records (`image.updater.<name>` and
//! `image.target.<name>`) - the format needs both.
//!
//! Example:
//!
//! ```toml
//! [image]
//! platform = "d13x"
//! product  = "general-board"
//! version  = "0.1.0"
//! media    = "spi-nor"
//!
//! [[component]]
//! kind = "app"
//! name = "bootloader"
//! file = "../../target/.../release/app-bootloader"
//! partition = "bootloader"
//! attr = "mtd;required"
//! ```

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result, bail};

use super::{
    aic::AicImageBuilder,
    elf,
    img::{FwComponent, FwImage},
    pbp,
    toml_lite::{self, Table, Value},
};

/// How a component's source file is turned into image data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentKind {
    /// Application ELF → AIC loader container (auto load/entry from the ELF).
    App,
    /// Raw binary or PBP file → AIC container with a PBP resource.
    Pbp,
    /// File bytes used as-is.
    Raw,
}

impl ComponentKind {
    fn parse(value: &str) -> Result<Self> {
        match value {
            "app" => Ok(Self::App),
            "pbp" => Ok(Self::Pbp),
            "raw" => Ok(Self::Raw),
            other => bail!("unknown component kind {other:?} (expected app|pbp|raw)"),
        }
    }
}

/// One `[[component]]` entry.
#[derive(Clone, Debug)]
pub struct ComponentSpec {
    pub kind: ComponentKind,
    pub name: String,
    pub file: PathBuf,
    pub partition: String,
    pub attr: String,
    pub filename: String,
    /// Target-role overrides, used when one entry is both an updater and a target
    /// (`updater = true` **and** a non-empty `partition`).
    pub target_attr: Option<String>,
    pub target_filename: Option<String>,
    pub updater: bool,
    /// Cargo package that produces `file` (`cargo build -p <package>`).
    ///
    /// Lets `aicfwc --build` produce the file instead of requiring it built
    /// first; `file` must then point under cargo's `target/`, where the triple and
    /// profile are read from. `None` means "already built, just pack it".
    pub build_package: Option<String>,
    pub ram: Option<u32>,
    pub pbp: Option<PathBuf>,
    pub private: Option<PathBuf>,
    pub load_address: Option<u32>,
    pub entry_point: Option<u32>,
    pub entry_offset: Option<u32>,
    pub header_version: Option<u32>,
    pub with_ext: bool,
}

/// One `[[partition]]` entry: the on-flash layout owned by this project. The
/// device bootloader resolves `fwc_meta.partition` names through it and reports
/// it via `GET_PARTITION_TABLE`; it is independent of vendor `partition.json`.
#[derive(Clone, Debug)]
pub struct PartitionSpec {
    pub name: String,
    pub offset: u32,
    pub size: u32,
}

/// A parsed image manifest.
#[derive(Clone, Debug, Default)]
pub struct ImageManifest {
    pub platform: String,
    pub product: String,
    pub version: String,
    pub media_type: String,
    pub media_dev_id: u32,
    pub media_id: String,
    /// Application name a normal boot jumps to (`[image] app`); empty boots any
    /// valid image in the `app` partition.
    pub boot_app: String,
    pub partitions: Vec<PartitionSpec>,
    pub components: Vec<ComponentSpec>,
}

impl ImageManifest {
    /// Parse a manifest from TOML text.
    pub fn parse(text: &str) -> Result<Self> {
        let root = toml_lite::parse(text).map_err(|e| anyhow::anyhow!("TOML parse error: {e}"))?;

        let image = root
            .get("image")
            .and_then(Value::as_table)
            .context("manifest is missing the [image] table")?;

        let mut manifest = Self {
            platform: string_field(image, "platform")?.unwrap_or_else(|| "d13x".to_string()),
            product: string_field(image, "product")?.unwrap_or_else(|| "unknown".to_string()),
            version: string_field(image, "version")?.unwrap_or_else(|| "0.0.0".to_string()),
            media_type: string_field(image, "media")?
                .or(string_field(image, "media_type")?)
                .unwrap_or_else(|| "spi-nor".to_string()),
            media_dev_id: int_field(image, "media_dev_id")?.unwrap_or(0),
            media_id: string_field(image, "media_id")?.unwrap_or_default(),
            boot_app: string_field(image, "app")?.unwrap_or_default(),
            partitions: Vec::new(),
            components: Vec::new(),
        };

        if let Some(value) = root.get("partition").or_else(|| root.get("partitions")) {
            let entries = value
                .as_array()
                .context("[[partition]] must be an array of tables")?;
            for (index, entry) in entries.iter().enumerate() {
                let table = entry
                    .as_table()
                    .with_context(|| format!("partition #{index} is not a table"))?;
                let name = string_field(table, "name")?
                    .with_context(|| format!("partition #{index} is missing `name`"))?;
                let offset = int_field(table, "offset")?
                    .with_context(|| format!("partition `{name}` is missing `offset`"))?;
                let size = int_field(table, "size")?
                    .with_context(|| format!("partition `{name}` is missing `size`"))?;
                manifest
                    .partitions
                    .push(PartitionSpec { name, offset, size });
            }
        }

        let components = root
            .get("component")
            .or_else(|| root.get("components"))
            .context("manifest is missing [[component]] entries")?;
        let entries = components
            .as_array()
            .context("[[component]] must be an array of tables")?;
        if entries.is_empty() {
            bail!("manifest has no [[component]] entries");
        }

        for (index, entry) in entries.iter().enumerate() {
            let table = entry
                .as_table()
                .with_context(|| format!("component #{index} is not a table"))?;
            manifest.components.push(parse_component(table, index)?);
        }

        Ok(manifest)
    }

    /// Read and parse a manifest file.
    pub fn load(path: &Path) -> Result<Self> {
        let text = fs::read_to_string(path)
            .with_context(|| format!("failed to read manifest {path:?}"))?;
        Self::parse(&text).with_context(|| format!("invalid manifest {path:?}"))
    }

    /// Build the packed [`FwImage`], resolving component files relative to
    /// `base_dir`.
    ///
    /// An entry that is both an updater and a target (`updater = true` **and** a
    /// non-empty `partition`) becomes **two** meta records, because the format
    /// needs both, but it is written once here. Records are emitted
    /// updaters-first, in manifest order within each group, so the host `EXEC`s
    /// the RAM images before writing partitions.
    pub fn build(&self, base_dir: &Path) -> Result<FwImage> {
        let mut updaters = Vec::new();
        let mut targets = Vec::new();
        for spec in &self.components {
            let file_path = base_dir.join(&spec.file);
            let payload = fs::read(&file_path)
                .with_context(|| format!("failed to read component file {file_path:?}"))?;
            let data = build_component(spec, base_dir, payload)?;

            if spec.updater {
                updaters.push(FwComponent {
                    name: spec.name.clone(),
                    partition: String::new(),
                    attr: spec.attr.clone(),
                    filename: spec.filename.clone(),
                    data: data.clone(),
                    updater: true,
                    ram: spec.ram,
                });
            }

            // A target role exists for a pure target, or for the dual entry.
            if !spec.updater || !spec.partition.is_empty() {
                let attr = spec.target_attr.clone().unwrap_or_else(|| {
                    if spec.updater {
                        "mtd;required".to_string()
                    } else {
                        spec.attr.clone()
                    }
                });
                targets.push(FwComponent {
                    name: spec.name.clone(),
                    partition: spec.partition.clone(),
                    attr,
                    filename: spec
                        .target_filename
                        .clone()
                        .unwrap_or_else(|| spec.filename.clone()),
                    data,
                    updater: false,
                    ram: None,
                });
            }
        }

        let mut components = updaters;
        components.extend(targets);

        Ok(FwImage {
            platform: self.platform.clone(),
            product: self.product.clone(),
            version: self.version.clone(),
            media_type: self.media_type.clone(),
            media_dev_id: self.media_dev_id,
            media_id: self.media_id.clone(),
            components,
        })
    }

    /// Build every component that declares `build_package`, before packing.
    ///
    /// Each package gets its **own** `cargo build` invocation: `artinchip-rt`
    /// picks its entry point from crate-level features, and `pbp-common` needs the
    /// PBP entry (no `app`) while the bootloader needs `app`, so they must not
    /// share one. The lockfile keeps this reproducible.
    pub fn build_packages(&self, base_dir: &Path) -> Result<()> {
        let mut built: Vec<&str> = Vec::new();
        for spec in &self.components {
            let Some(package) = spec.build_package.as_deref() else {
                continue;
            };
            if built.contains(&package) {
                continue;
            }
            built.push(package);

            let (target, release) = spec.build_target()?;
            let mut command = Command::new(env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
            command
                .current_dir(base_dir)
                .args(["build", "-p", package, "--target", &target]);
            if release {
                command.arg("--release");
            }

            println!(
                "build  : cargo build -p {package} --target {target}{}",
                if release { " --release" } else { "" }
            );
            let status = command
                .status()
                .with_context(|| format!("failed to run cargo for package {package:?}"))?;
            if !status.success() {
                bail!("cargo build -p {package} failed ({status})");
            }
        }
        Ok(())
    }
}

impl ComponentSpec {
    /// Target triple and profile the component's `file` is produced under, read
    /// from the path (`.../target/<triple>/<profile>/...`) rather than declared
    /// again, so the manifest keeps one description of where the artifact lives.
    fn build_target(&self) -> Result<(String, bool)> {
        let parts: Vec<&str> = self
            .file
            .components()
            .filter_map(|part| part.as_os_str().to_str())
            .collect();
        let index = parts
            .iter()
            .position(|part| *part == "target")
            .with_context(|| {
                format!(
                    "component {:?} sets `build_package` but its `file` ({:?}) is not under a cargo `target/` directory",
                    self.name, self.file
                )
            })?;
        let (Some(target), Some(profile)) = (parts.get(index + 1), parts.get(index + 2)) else {
            bail!(
                "component {:?}: `file` ({:?}) has no <triple>/<profile> after `target`",
                self.name,
                self.file
            );
        };
        Ok(((*target).to_string(), *profile == "release"))
    }
}

fn build_component(spec: &ComponentSpec, base_dir: &Path, payload: Vec<u8>) -> Result<Vec<u8>> {
    let private = read_optional(base_dir, &spec.private)?;
    match spec.kind {
        ComponentKind::Raw => Ok(payload),
        ComponentKind::Pbp => {
            let pbp_bytes = pbp_bytes(payload)?;
            let mut builder = AicImageBuilder::new()
                .pbp(&pbp_bytes)
                .with_ext(spec.with_ext);
            if let Some(version) = spec.header_version {
                builder = builder.header_version(version);
            }
            if let Some(private) = &private {
                builder = builder.private(private);
            }
            Ok(builder.build())
        }
        ComponentKind::App => {
            let elf = elf::parse(&payload)?;
            let load_address = spec.load_address.unwrap_or(elf.load_address);
            let entry_point = spec
                .entry_point
                .or_else(|| {
                    spec.entry_offset
                        .map(|offset| load_address.wrapping_add(offset))
                })
                .unwrap_or(elf.entry);

            let loader = {
                let mut builder =
                    AicImageBuilder::new().loader(&elf.payload, load_address, entry_point);
                // Record the manifest name so the bootloader can match the image
                // it finds in a partition against the app it was told to boot.
                builder = builder.app_name(&spec.name);
                if let Some(version) = spec.header_version {
                    builder = builder.header_version(version);
                }
                if let Some(private) = &private {
                    builder = builder.private(private);
                }
                builder.build()
            };

            // Vendor `bootloader.aic` == pbp_ext.aic ++ loader.aic.
            match &spec.pbp {
                None => Ok(loader),
                Some(pbp_path) => {
                    let raw = fs::read(base_dir.join(pbp_path))
                        .with_context(|| format!("failed to read PBP {pbp_path:?}"))?;
                    let pbp_bytes = pbp_bytes(raw)?;
                    let mut builder = AicImageBuilder::new().pbp(&pbp_bytes).with_ext(true);
                    if let Some(private) = &private {
                        builder = builder.private(private);
                    }
                    let pbp_ext = builder.build();
                    let mut combined = pbp_ext;
                    combined.extend_from_slice(&loader);
                    Ok(combined)
                }
            }
        }
    }
}

/// Normalise a PBP resource, accepting the linker output as well as the raw file.
///
/// A PBP is built like any crate, so the manifest can point straight at the ELF
/// instead of a `rust-objcopy`ed `.pbp`; [`elf::to_binary`] reconstructs that
/// image byte-identically. A file that is already a PBP passes through.
fn pbp_bytes(payload: Vec<u8>) -> Result<Vec<u8>> {
    let raw = if payload.starts_with(b"\x7fELF") {
        elf::to_binary(&payload)?
    } else {
        payload
    };
    pbp::normalize(raw)
}

fn read_optional(base_dir: &Path, path: &Option<PathBuf>) -> Result<Option<Vec<u8>>> {
    match path {
        Some(path) => {
            let full = base_dir.join(path);
            Ok(Some(
                fs::read(&full).with_context(|| format!("failed to read {full:?}"))?,
            ))
        }
        None => Ok(None),
    }
}

fn parse_component(table: &Table, index: usize) -> Result<ComponentSpec> {
    let kind = match string_field(table, "kind")? {
        Some(kind) => ComponentKind::parse(&kind)?,
        None => ComponentKind::Raw,
    };
    let name = string_field(table, "name")?
        .with_context(|| format!("component #{index} is missing `name`"))?;
    let file = string_field(table, "file")?
        .with_context(|| format!("component `{name}` is missing `file`"))?;
    let file = PathBuf::from(file);
    let filename = string_field(table, "filename")?.unwrap_or_else(|| {
        file.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("component.bin")
            .to_string()
    });

    Ok(ComponentSpec {
        kind,
        name,
        file,
        partition: string_field(table, "partition")?.unwrap_or_default(),
        attr: string_field(table, "attr")?.unwrap_or_else(|| "mtd;required".to_string()),
        filename,
        target_attr: string_field(table, "target_attr")?,
        target_filename: string_field(table, "target_filename")?,
        updater: bool_field(table, "updater")?.unwrap_or(false),
        build_package: string_field(table, "build_package")?,
        ram: int_field(table, "ram")?,
        pbp: string_field(table, "pbp")?.map(PathBuf::from),
        private: string_field(table, "private")?.map(PathBuf::from),
        load_address: int_field(table, "load_address")?,
        entry_point: int_field(table, "entry_point")?,
        entry_offset: int_field(table, "entry_offset")?,
        header_version: int_field(table, "header_version")?,
        with_ext: bool_field(table, "with_ext")?.unwrap_or(false),
    })
}

fn string_field(table: &Table, key: &str) -> Result<Option<String>> {
    match table.get(key) {
        None => Ok(None),
        Some(value) => value
            .as_str()
            .map(|s| Some(s.to_string()))
            .with_context(|| format!("`{key}` must be a string")),
    }
}

fn int_field(table: &Table, key: &str) -> Result<Option<u32>> {
    match table.get(key) {
        None => Ok(None),
        Some(value) => {
            let raw = value
                .as_int()
                .with_context(|| format!("`{key}` must be an integer"))?;
            if !(0..=u32::MAX as i64).contains(&raw) {
                bail!("`{key}` is out of range: {raw}");
            }
            Ok(Some(raw as u32))
        }
    }
}

fn bool_field(table: &Table, key: &str) -> Result<Option<bool>> {
    match table.get(key) {
        None => Ok(None),
        Some(value) => value
            .as_bool()
            .map(Some)
            .with_context(|| format!("`{key}` must be a boolean")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_minimal_manifest() {
        let manifest = ImageManifest::parse(
            r#"
[image]
platform = "d13x"
product = "general-board"
version = "0.1.0"
media = "spi-nor"

[[component]]
kind = "app"
name = "bootloader"
file = "app-bootloader"
partition = "bootloader"
attr = "mtd;required"
"#,
        )
        .unwrap();

        assert_eq!(manifest.platform, "d13x");
        assert_eq!(manifest.media_type, "spi-nor");
        assert_eq!(manifest.components.len(), 1);
        assert_eq!(manifest.components[0].name, "bootloader");
        assert_eq!(manifest.components[0].kind, ComponentKind::App);
    }

    #[test]
    fn dual_role_component_expands_to_updater_and_target() {
        let name = "aicfwc-dual-role-test.bin";
        let dir = std::env::temp_dir();
        fs::write(dir.join(name), b"payload").unwrap();

        let manifest = ImageManifest::parse(
            r#"
[image]
platform = "d13x"

[[component]]
kind = "raw"
name = "bootloader"
file = "aicfwc-dual-role-test.bin"
filename = "bootloader.aic"
target_filename = "app-bootloader.aic"
updater = true
ram = 0x40100000
attr = "required;run"
partition = "bootloader"
"#,
        )
        .unwrap();
        let image = manifest.build(&dir).unwrap();
        let _ = fs::remove_file(dir.join(name));

        // One entry -> updater record first, target record second.
        let records: Vec<_> = image
            .components
            .iter()
            .map(|c| (c.updater, c.attr.as_str(), c.partition.as_str()))
            .collect();
        assert_eq!(
            records,
            [
                (true, "required;run", ""),
                (false, "mtd;required", "bootloader")
            ]
        );
        assert_eq!(image.components[0].ram, Some(0x4010_0000));
        assert_eq!(image.components[0].filename, "bootloader.aic");
        assert_eq!(image.components[1].filename, "app-bootloader.aic");
    }
}
