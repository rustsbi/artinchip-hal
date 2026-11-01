//! Artifact resolution, cargo builds and on-disk image packing.
//!
//! [`TargetLayout`] knows where cargo writes a package's binary
//! (`<target-dir>/<triple>/<profile>/<bin>`); [`pack`] composes the planned image
//! from those binaries and writes the `AIC.FW` image plus one piece per record.
//!
//! The target directory is never guessed from an artifact path. It comes from
//! `--target-dir`, then `CARGO_TARGET_DIR`, then the workspace root's `target/`,
//! in that order.

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result, bail};

use crate::{
    app_img::{AicImageBuilder, FwComponent, FwImage, elf, pbp},
    preset::{ComponentKind, ComponentSpec, ImagePlan, Media, Source},
    raw_img,
};

/// BootROM page size of the SPI-NAND parts this tool wraps for.
pub const NAND_PAGE_SIZE: usize = 2048;

/// Where cargo writes build artifacts: `<target_dir>/<triple>/<profile>/`.
#[derive(Clone, Debug)]
pub struct TargetLayout {
    pub target_dir: PathBuf,
    pub triple: String,
    /// Cargo profile name: `release`, `debug`, or a custom profile.
    pub profile: String,
}

impl TargetLayout {
    /// Create a layout, rejecting a profile that cannot be a cargo profile name.
    pub fn new(
        target_dir: impl Into<PathBuf>,
        triple: impl Into<String>,
        profile: impl Into<String>,
    ) -> Result<Self> {
        let layout = Self {
            target_dir: target_dir.into(),
            triple: triple.into(),
            profile: profile.into(),
        };
        if layout.triple.is_empty() {
            bail!("board preset declares no cargo target triple");
        }
        if layout.profile.is_empty() {
            bail!("cargo profile name must not be empty");
        }
        if layout.profile.contains(['/', '\\']) {
            bail!(
                "cargo profile name must not be a path: {:?}",
                layout.profile
            );
        }
        Ok(layout)
    }

    /// The directory name cargo uses for the profile.
    ///
    /// The `dev` profile lands in `debug/`, everything else in a directory named
    /// after the profile.
    pub fn profile_dir(&self) -> &str {
        match self.profile.as_str() {
            "dev" => "debug",
            profile => profile,
        }
    }

    /// The directory cargo writes the triple's artifacts to.
    pub fn artifact_dir(&self) -> PathBuf {
        self.target_dir.join(&self.triple).join(self.profile_dir())
    }

    /// Path of the binary a cargo package produces.
    pub fn artifact(&self, bin: &str) -> PathBuf {
        self.artifact_dir().join(bin)
    }

    /// Default output directory of an image and its pieces.
    pub fn output_dir(&self, stem: &str) -> PathBuf {
        self.artifact_dir().join(format!("{stem}-out"))
    }

    /// The `cargo build` argument vector (without the program name) that produces
    /// a package's artifact in this layout.
    pub fn build_arguments(&self, package: &str, features: &[String]) -> Vec<String> {
        let mut arguments = vec![
            "build".to_string(),
            "-p".to_string(),
            package.to_string(),
            "--target".to_string(),
            self.triple.clone(),
            "--target-dir".to_string(),
            self.target_dir.display().to_string(),
        ];
        match self.profile.as_str() {
            "release" => arguments.push("--release".to_string()),
            "debug" | "dev" => {}
            profile => {
                arguments.push("--profile".to_string());
                arguments.push(profile.to_string());
            }
        }
        if !features.is_empty() {
            arguments.push("--no-default-features".to_string());
            arguments.push("--features".to_string());
            arguments.push(features.join(","));
        }
        arguments
    }

    /// The exact `cargo build` command line for a package, for logs and errors.
    pub fn build_command(&self, package: &str, features: &[String]) -> String {
        format!(
            "cargo {}",
            self.build_arguments(package, features).join(" ")
        )
    }
}

/// Locate the cargo target directory.
///
/// Order: `--target-dir`, `CARGO_TARGET_DIR` (relative to the current
/// directory), the workspace root's `target/`.
pub fn resolve_target_dir(explicit: Option<&Path>) -> Result<PathBuf> {
    let cwd = env::current_dir().context("failed to read the current directory")?;
    let env_target = env::var_os("CARGO_TARGET_DIR").map(PathBuf::from);
    let workspace_root = workspace_root(&cwd);
    choose_target_dir(
        &cwd,
        explicit,
        env_target.as_deref(),
        workspace_root.as_deref(),
    )
    .context(
        "cannot locate the cargo target directory: pass --target-dir, set CARGO_TARGET_DIR, or \
         run this inside the workspace",
    )
}

/// Pure precedence rule behind [`resolve_target_dir`].
fn choose_target_dir(
    cwd: &Path,
    explicit: Option<&Path>,
    env_target: Option<&Path>,
    workspace_root: Option<&Path>,
) -> Option<PathBuf> {
    explicit
        .map(|dir| absolutize(cwd, dir))
        .or_else(|| env_target.map(|dir| absolutize(cwd, dir)))
        .or_else(|| workspace_root.map(|root| root.join("target")))
}

/// Resolve a possibly relative path against `cwd`.
fn absolutize(cwd: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        cwd.join(path)
    }
}

/// The workspace root: the closest ancestor whose `Cargo.toml` has a
/// `[workspace]` table.
fn workspace_root(cwd: &Path) -> Option<PathBuf> {
    for ancestor in cwd.ancestors() {
        let manifest = ancestor.join("Cargo.toml");
        let Ok(text) = fs::read_to_string(&manifest) else {
            continue;
        };
        if text.lines().any(|line| line.trim() == "[workspace]") {
            return Some(ancestor.to_path_buf());
        }
    }
    None
}

fn cargo_program() -> std::ffi::OsString {
    env::var_os("CARGO").unwrap_or_else(|| "cargo".into())
}

/// The directory `cargo build` runs in: the workspace root when there is one, the
/// current directory otherwise.
fn build_dir() -> PathBuf {
    env::current_dir()
        .ok()
        .and_then(|cwd| workspace_root(&cwd))
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Run `cargo build` for every package the image needs, one invocation per
/// package.
pub fn build_required(plan: &ImagePlan, layout: &TargetLayout) -> Result<()> {
    let mut built: Vec<(String, Vec<String>)> = Vec::new();
    for requirement in &plan.required_builds {
        if built.iter().any(|(package, features)| {
            package == &requirement.package && features == &requirement.features
        }) {
            continue;
        }
        built.push((requirement.package.clone(), requirement.features.clone()));

        let arguments = layout.build_arguments(&requirement.package, &requirement.features);
        let command = format!("cargo {}", arguments.join(" "));
        println!("build  : {command}");
        let status = Command::new(cargo_program())
            .current_dir(build_dir())
            .args(&arguments)
            .status()
            .with_context(|| format!("failed to run `{command}`"))?;
        if !status.success() {
            bail!("`{command}` failed ({status})");
        }
    }
    Ok(())
}

/// Fail before packing when a component's binary has not been built, naming the
/// exact command that produces it.
pub fn check_artifacts(plan: &ImagePlan, layout: &TargetLayout) -> Result<()> {
    for component in &plan.components {
        for source in std::iter::once(&component.source).chain(component.pbp_source.iter()) {
            let Source::Package {
                package,
                bin,
                features,
            } = source
            else {
                continue;
            };
            let path = layout.artifact(bin);
            if !path.is_file() {
                bail!(
                    "missing artifact for component {:?}: {path:?}\n\
                     build it with:\n  {}",
                    component.spec.name,
                    layout.build_command(package, features)
                );
            }
        }
        if let Some(private) = &component.spec.private
            && !private.is_file()
        {
            bail!(
                "missing private resource for component {:?}: {private:?}",
                component.spec.name
            );
        }
    }
    Ok(())
}

/// A component's payload bytes, ready for conversion.
#[derive(Clone, Debug)]
pub struct ResolvedComponent {
    pub spec: ComponentSpec,
    pub payload: Vec<u8>,
    pub pbp_payload: Option<Vec<u8>>,
}

/// Read every component payload a plan needs.
pub fn resolve_payloads(plan: &ImagePlan, layout: &TargetLayout) -> Result<Vec<ResolvedComponent>> {
    plan.components
        .iter()
        .map(|component| {
            let payload = read_source(&component.source, layout, &component.spec)?;
            let pbp_payload = match &component.pbp_source {
                Some(source) => Some(read_source(source, layout, &component.spec)?),
                None => None,
            };
            Ok(ResolvedComponent {
                spec: component.spec.clone(),
                payload,
                pbp_payload,
            })
        })
        .collect()
}

fn read_source(source: &Source, layout: &TargetLayout, spec: &ComponentSpec) -> Result<Vec<u8>> {
    let path = match source {
        Source::File(path) => path.clone(),
        Source::Package { bin, .. } => layout.artifact(bin),
    };
    fs::read(&path).with_context(|| {
        format!(
            "failed to read the payload for component {:?}: {path:?}",
            spec.name
        )
    })
}

/// Convert one component's payload into the bytes the image carries.
///
/// The conversion depends on the kind: an application ELF becomes an AIC loader
/// (load address/entry from the ELF, optionally appended after a PBP container),
/// a PBP becomes an AIC container with a PBP resource, and a raw file is used
/// as-is.
///
/// An application that declares a `load_address` (from `[apps.<package>]`) is
/// checked against the ELF: the linker and the packer must agree on where the
/// bootloader will copy the image, and a mismatch means the application was built
/// by an out-of-date `build.rs` or the preset changed after the build.
pub fn convert_component(
    spec: &ComponentSpec,
    payload: &[u8],
    pbp_payload: Option<&[u8]>,
) -> Result<Vec<u8>> {
    let private = match &spec.private {
        Some(path) => Some(fs::read(path).with_context(|| {
            format!(
                "failed to read the private resource for component {:?}: {path:?}",
                spec.name
            )
        })?),
        None => None,
    };

    match spec.kind {
        ComponentKind::Raw => Ok(payload.to_vec()),
        ComponentKind::Pbp => {
            let pbp_bytes = pbp_bytes(payload.to_vec())?;
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
            let elf = elf::parse(payload)?;
            if let Some(declared) = spec.load_address
                && declared != elf.load_address
            {
                bail!(
                    "component {:?}: the ELF is linked at {:#x} but the board preset declares \
                     `load_address = {declared:#x}`; rebuild the application so its build script \
                     picks the preset up, or fix the preset",
                    spec.name,
                    elf.load_address
                );
            }
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
                // Record the preset name so the bootloader can match the image
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
            match pbp_payload {
                None => Ok(loader),
                Some(raw) => {
                    let pbp_bytes = pbp_bytes(raw.to_vec())?;
                    let mut builder = AicImageBuilder::new().pbp(&pbp_bytes).with_ext(true);
                    if let Some(private) = &private {
                        builder = builder.private(private);
                    }
                    if spec.ext_reserve != 0 {
                        builder = builder.ext_reserve(spec.ext_reserve);
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
fn pbp_bytes(payload: Vec<u8>) -> Result<Vec<u8>> {
    let raw = if payload.starts_with(b"\x7fELF") {
        elf::to_binary(&payload)?
    } else {
        payload
    };
    pbp::normalize(raw)
}

/// Compose the packed image records from resolved payloads.
///
/// A component with `updater = true` becomes one `image.updater.<name>` record;
/// a component with a partition becomes one `image.target.<name>` record. An entry
/// that is both expands to two records (it is still written once).
pub fn compose(plan: &ImagePlan, resolved: &[ResolvedComponent]) -> Result<FwImage> {
    let mut updaters = Vec::new();
    let mut targets = Vec::new();
    for component in resolved {
        let spec = &component.spec;
        let data = convert_component(spec, &component.payload, component.pbp_payload.as_deref())?;

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
                data: wrap_target_for_media(plan, spec, data)?,
                updater: false,
                ram: None,
            });
        }
    }

    let mut components = updaters;
    components.extend(targets);

    // A payload that does not fit its partition would burn a truncated image.
    for component in &components {
        if component.partition.is_empty() {
            continue;
        }
        if let Some(partition) = plan.partition(&component.partition)
            && component.data.len() > partition.size as usize
        {
            bail!(
                "component {:?} produces {} bytes, which does not fit partition {:?} ({} bytes)",
                component.name,
                component.data.len(),
                partition.name,
                partition.size
            );
        }
    }

    Ok(FwImage {
        platform: plan.image.platform.clone(),
        product: plan.image.product.clone(),
        version: plan.image.version.clone(),
        media_type: plan.image.media.name().to_string(),
        media_dev_id: plan.image.media_dev_id,
        media_id: plan.image.media_id.clone(),
        components,
    })
}

/// Wrap the target copy of the first-stage component in the BootROM's SPI-NAND
/// page table.
///
/// On SPI-NAND the BootROM boots the first slot through an `AICP` page table (see
/// [`raw_img::nand`]), so the bytes that have to reach flash are the wrapped ones.
/// The same component's RAM copy (`image.updater.*`, which the BootROM itself
/// parses) stays the bare container the file holds - that difference between the
/// two roles is why this is applied here rather than to the file.
fn wrap_target_for_media(plan: &ImagePlan, spec: &ComponentSpec, data: Vec<u8>) -> Result<Vec<u8>> {
    if plan.image.media != Media::SpiNand {
        return Ok(data);
    }
    let at_slot_zero = plan
        .partition(&spec.partition)
        .is_some_and(|partition| partition.offset == 0);
    if !at_slot_zero {
        return Ok(data);
    }
    raw_img::nand::build(&data, NAND_PAGE_SIZE)
}

/// An image ready to be written: the packed bytes and the pieces it is made of.
#[derive(Clone, Debug)]
pub struct PackedImage {
    pub bytes: Vec<u8>,
    /// `(filename, bytes)` for every record, in image order.
    pub pieces: Vec<(String, Vec<u8>)>,
    /// Flash-layout image for direct programming (0xFF-filled gaps).
    pub raw: Vec<u8>,
}

/// Read the artifacts and compose the packed image.
pub fn pack(plan: &ImagePlan, layout: &TargetLayout) -> Result<PackedImage> {
    let resolved = resolve_payloads(plan, layout)?;
    let image = compose(plan, &resolved)?;
    let pieces = image
        .components
        .iter()
        .map(|component| (component.filename.clone(), component.data.clone()))
        .collect();
    Ok(PackedImage {
        bytes: image.build(),
        pieces,
        raw: raw_image(plan, &image)?,
    })
}

/// Lay target components into their partition offsets for a direct flash write.
fn raw_image(plan: &ImagePlan, image: &FwImage) -> Result<Vec<u8>> {
    let mut slots: Vec<(usize, &[u8])> = Vec::new();
    let mut len = 0usize;
    for component in image.components.iter().filter(|c| !c.updater) {
        let partition = plan
            .partition(&component.partition)
            .with_context(|| format!("unknown partition {:?}", component.partition))?;
        let offset = partition.offset as usize;
        let end = offset + component.data.len();
        len = len.max(end);
        slots.push((offset, &component.data));
    }
    let mut raw = vec![0xFFu8; len];
    for (offset, data) in slots {
        raw[offset..offset + data.len()].copy_from_slice(data);
    }
    Ok(raw)
}

/// Write the image and its pieces into `out_dir`.
pub fn write(plan: &ImagePlan, packed: &PackedImage, out_dir: &Path) -> Result<()> {
    fs::create_dir_all(out_dir)
        .with_context(|| format!("failed to create output directory {out_dir:?}"))?;
    let image_path = out_dir.join(format!("{}.img", plan.stem));
    fs::write(&image_path, &packed.bytes)
        .with_context(|| format!("failed to write {image_path:?}"))?;
    println!(
        "image  : {image_path:?} ({} bytes, {} components)",
        packed.bytes.len(),
        packed.pieces.len()
    );

    // The pieces themselves, so a board that is programmed without the host tool
    // still has each file the image is made of (the combined `bootloader.aic`, the
    // application's AIC, the PBP). Same bytes as the meta records carry.
    for (filename, data) in &packed.pieces {
        let path = out_dir.join(filename);
        fs::write(&path, data).with_context(|| format!("failed to write {path:?}"))?;
        println!("piece  : {path:?} ({} bytes)", data.len());
    }

    // Flash-layout image for a programmer that writes raw media, not AIC.FW.
    let media = if plan.image.media == Media::SpiNand {
        "nand"
    } else {
        "nor"
    };
    let raw_path = out_dir.join(format!("{}-raw-{media}.img", plan.stem));
    fs::write(&raw_path, &packed.raw).with_context(|| format!("failed to write {raw_path:?}"))?;
    println!("raw    : {raw_path:?} ({} bytes)", packed.raw.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::preset::{Preset, Selection};

    fn plan(text: &str) -> ImagePlan {
        let preset = Preset::parse(text, &PathBuf::from("boards/test.toml")).unwrap();
        preset.plan(&Selection::Bootloader).unwrap()
    }

    /// Compose with explicit payloads: no target directory is touched.
    fn compose_with(plan: &ImagePlan, payloads: &[&[u8]]) -> Result<FwImage> {
        let resolved: Vec<ResolvedComponent> = plan
            .components
            .iter()
            .zip(payloads)
            .map(|(component, payload)| ResolvedComponent {
                spec: component.spec.clone(),
                payload: payload.to_vec(),
                pbp_payload: None,
            })
            .collect();
        compose(plan, &resolved)
    }

    const RAW_BOARD: &str = r#"
[board]
name = "test"
chip = "d13x"

[[partition]]
name = "bootloader"
offset = 0x000000
size = 0x080000

[[partition]]
name = "app"
offset = 0x080000
size = 0x400000

[[component]]
kind = "raw"
name = "bootloader"
package = "app-bootloader"
updater = true
ram = 0x40100000
partition = "bootloader"
attr = "required;run"
filename = "bootloader.aic"
target_filename = "app-bootloader.aic"
load_address = 0x406c0000
"#;

    #[test]
    fn target_directory_precedence_is_explicit_env_workspace() {
        let cwd = Path::new("/work/ws");
        let explicit = Path::new("out");
        let env = Path::new("/cache/target");
        let workspace = Path::new("/work/ws");

        assert_eq!(
            choose_target_dir(cwd, Some(explicit), Some(env), Some(workspace)),
            Some(PathBuf::from("/work/ws/out"))
        );
        assert_eq!(
            choose_target_dir(cwd, None, Some(env), Some(workspace)),
            Some(PathBuf::from("/cache/target"))
        );
        assert_eq!(
            choose_target_dir(cwd, None, None, Some(workspace)),
            Some(PathBuf::from("/work/ws/target"))
        );
        assert_eq!(choose_target_dir(cwd, None, None, None), None);
    }

    /// A dual-role entry is written once but expands to two records: the updater
    /// first, then the target.
    #[test]
    fn dual_role_component_expands_to_updater_and_target() {
        let plan = plan(RAW_BOARD);
        let image = compose_with(&plan, &[b"payload"]).unwrap();

        let records: Vec<_> = image
            .components
            .iter()
            .map(|component| {
                (
                    component.updater,
                    component.attr.as_str(),
                    component.partition.as_str(),
                )
            })
            .collect();
        assert_eq!(
            records,
            [
                (true, "required;run", ""),
                (false, "mtd;required", "bootloader"),
            ]
        );
        assert_eq!(image.components[0].ram, Some(0x4010_0000));
        assert_eq!(image.components[0].filename, "bootloader.aic");
        assert_eq!(image.components[1].filename, "app-bootloader.aic");
    }

    #[test]
    fn spi_nand_target_at_slot_zero_is_wrapped_for_the_bootrom() {
        let plan = plan(
            r#"
[board]
chip = "d21x"

[[partition]]
name = "bootloader"
offset = 0x000000
size = 0x100000

[[partition]]
name = "app"
offset = 0x100000
size = 0x400000

[[component]]
kind = "raw"
name = "bootloader"
package = "app-bootloader"
updater = true
ram = 0x40800000
partition = "bootloader"
attr = "required;run"
"#,
        );

        // An AIC container, like the combined `bootloader.aic`.
        let mut aic = vec![0u8; 0x100];
        aic[..4].copy_from_slice(b"AIC ");
        aic.extend_from_slice(&[0x5a; 3000]);
        let image = compose_with(&plan, &[&aic]).unwrap();

        // The RAM copy stays the bare container the BootROM parses...
        assert_eq!(&image.components[0].data[..4], b"AIC ");
        // ...while the flash copy carries the `AICP` page table in front of it.
        let target = &image.components[1].data;
        assert_eq!(&target[..4], b"AICP");
        assert_eq!(&target[2048..2052], b"AIC ");
        assert_eq!(target.len(), 2048 + aic.len().next_multiple_of(2048));
    }
}
