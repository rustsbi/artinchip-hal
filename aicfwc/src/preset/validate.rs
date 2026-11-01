//! Static validation rules for [`Preset`].

use std::collections::{BTreeMap, BTreeSet};

use anyhow::{Context, Result, bail};

use super::schema::{Chip, EXT_ALIGNMENT, NAME_MAX, PartitionSpec, Preset};

impl Preset {
    /// Validate the preset's static invariants.
    pub fn validate(&self) -> Result<()> {
        self.validate_board()?;
        self.validate_partitions()?;
        self.validate_components()?;
        self.validate_apps()?;
        self.validate_boot_chain()?;
        Ok(())
    }

    fn validate_board(&self) -> Result<()> {
        if self.board.chip.default_media() != self.image.media {
            bail!(
                "unsupported platform/media combination: chip {:?} boots from {}, preset says {:?}",
                self.board.chip.name(),
                self.board.chip.default_media().name(),
                self.image.media.name()
            );
        }
        if self.image.platform != self.board.chip.name() {
            bail!(
                "unsupported platform {:?}: [image] platform must name the [board] chip {:?}",
                self.image.platform,
                self.board.chip.name()
            );
        }
        if self.board.profile.is_empty() {
            bail!("[board] profile must not be empty");
        }
        if self.image.boot_app.len() > NAME_MAX {
            bail!(
                "[image] app {:?} is {} bytes, longer than the {NAME_MAX}-byte AIC header field",
                self.image.boot_app,
                self.image.boot_app.len()
            );
        }
        let bits = if self.board.target.contains("riscv32") {
            32
        } else if self.board.target.contains("riscv64") {
            64
        } else {
            bail!(
                "unsupported target triple {:?}: expected a riscv32* or riscv64* bare-metal triple",
                self.board.target
            )
        };
        match (self.board.chip, bits) {
            (Chip::D13x, 32) | (Chip::D21x, 64) => {}
            (Chip::D13x, _) => bail!(
                "unsupported chip/target combination: chip {:?} builds for a riscv32* triple, \
                 preset says {:?}",
                self.board.chip.name(),
                self.board.target
            ),
            (Chip::D21x, _) => bail!(
                "unsupported chip/target combination: chip {:?} builds for a riscv64* triple, \
                 preset says {:?}",
                self.board.chip.name(),
                self.board.target
            ),
        }
        Ok(())
    }

    fn validate_partitions(&self) -> Result<()> {
        if self.partitions.is_empty() {
            bail!("preset declares no [[partition]] entries");
        }
        let erase = self.image.media.erase_size();
        if self.partitions[0].offset != 0 {
            bail!(
                "partition {:?} must start at offset 0: the BootROM reads the first-stage image \
                 there",
                self.partitions[0].name
            );
        }

        let mut names = BTreeSet::new();
        let mut previous_end = 0u32;
        for (index, partition) in self.partitions.iter().enumerate() {
            if partition.name.is_empty() {
                bail!("partition #{index} has an empty name");
            }
            if !names.insert(partition.name.clone()) {
                bail!("duplicate partition name {:?}", partition.name);
            }
            if partition.size == 0 {
                bail!("partition {:?} has a zero size", partition.name);
            }
            if !partition.offset.is_multiple_of(erase) {
                bail!(
                    "partition {:?} offset {:#x} is not aligned to the {erase:#x}-byte erase block \
                     of {}",
                    partition.name,
                    partition.offset,
                    self.image.media.name()
                );
            }
            if !partition.size.is_multiple_of(erase) {
                bail!(
                    "partition {:?} size {:#x} is not a multiple of the {erase:#x}-byte erase \
                     block of {}",
                    partition.name,
                    partition.size,
                    self.image.media.name()
                );
            }
            if index > 0 && partition.offset < previous_end {
                bail!(
                    "partitions are not ordered and non-overlapping: {:?} starts at {:#x}, before \
                     the previous partition ends at {previous_end:#x}",
                    partition.name,
                    partition.offset
                );
            }
            previous_end = partition
                .offset
                .checked_add(partition.size)
                .with_context(|| format!("partition {:?} ends past 4 GiB", partition.name))?;
        }
        Ok(())
    }

    fn validate_boot_chain(&self) -> Result<()> {
        let first = &self.partitions[0].name;
        if !self.components.iter().any(|component| {
            !component.partition.is_empty() && PartitionSpec::name_of(&component.partition) == first
        }) {
            bail!(
                "no `[[component]]` targets the first partition {first:?} at offset 0: the \
                 BootROM reads the first-stage image there"
            );
        }
        Ok(())
    }

    fn validate_components(&self) -> Result<()> {
        if self.components.is_empty() {
            bail!("preset declares no [[component]] entries");
        }
        let mut names = BTreeSet::new();
        for component in &self.components {
            if !names.insert(component.name.clone()) {
                bail!("duplicate component name {:?}", component.name);
            }
            if component.name.len() > NAME_MAX {
                bail!(
                    "component name {:?} is {} bytes, longer than the {NAME_MAX}-byte AIC header \
                     field",
                    component.name,
                    component.name.len()
                );
            }
            match (&component.package, &component.file) {
                (Some(_), Some(_)) => bail!(
                    "component {:?} sets both `package` and `file`; keep exactly one",
                    component.name
                ),
                (None, None) => bail!(
                    "component {:?} names neither a cargo `package` nor an explicit `file`",
                    component.name
                ),
                _ => {}
            }
            if component.pbp.is_some() && component.pbp_package.is_some() {
                bail!(
                    "component {:?} sets both `pbp` and `pbp_package`; keep exactly one",
                    component.name
                );
            }
            if component.updater && component.ram.is_none() {
                bail!(
                    "component {:?} is an updater (`updater = true`) but declares no `ram` load \
                     address",
                    component.name
                );
            }
            if component.partition.is_empty() {
                if !component.updater {
                    bail!(
                        "component {:?} is not an updater and names no `partition`",
                        component.name
                    );
                }
            } else {
                let partition = PartitionSpec::name_of(&component.partition);
                if !self
                    .partitions
                    .iter()
                    .any(|candidate| candidate.name == partition)
                {
                    bail!(
                        "component {:?} references unknown partition {:?}",
                        component.name,
                        partition
                    );
                }
            }
            if !component.ext_reserve.is_multiple_of(EXT_ALIGNMENT) {
                bail!(
                    "component {:?}: `ext_reserve` ({:#x}) must be a multiple of the {EXT_ALIGNMENT:#x}-byte loader alignment",
                    component.name,
                    component.ext_reserve
                );
            }
        }
        self.validate_feature_sets()?;
        Ok(())
    }

    fn validate_apps(&self) -> Result<()> {
        let mut names = BTreeSet::new();
        for app in self.apps.values() {
            if app.name.is_empty() {
                bail!("[apps.{}] has an empty `name`", app.package);
            }
            if app.name.len() > NAME_MAX {
                bail!(
                    "[apps.{}] name {:?} is {} bytes, longer than the {NAME_MAX}-byte AIC header \
                     field",
                    app.package,
                    app.name,
                    app.name.len()
                );
            }
            if app.filename.is_empty() {
                bail!("[apps.{}] has an empty `filename`", app.package);
            }
            if !self
                .partitions
                .iter()
                .any(|partition| partition.name == app.partition)
            {
                bail!(
                    "[apps.{}] references unknown partition {:?}",
                    app.package,
                    app.partition
                );
            }
            if names.iter().any(|name| name == &app.name) {
                bail!(
                    "duplicate application AIC name {:?} in [apps.{}]",
                    app.name,
                    app.package
                );
            }
            names.insert(app.name.clone());
            if let Some(base) = app.load_address
                && !base.is_multiple_of(4)
            {
                bail!(
                    "[apps.{}] load_address {base:#x} is not 4-byte aligned",
                    app.package
                );
            }
        }
        Ok(())
    }

    fn validate_feature_sets(&self) -> Result<()> {
        let mut seen: BTreeMap<&str, &[String]> = BTreeMap::new();
        for component in &self.components {
            for package in [
                component.package.as_deref(),
                component.pbp_package.as_deref(),
            ]
            .into_iter()
            .flatten()
            {
                let features = if component.pbp_package.as_deref() == Some(package) {
                    &component.pbp_features
                } else {
                    &component.features
                };
                if let Some(previous) = seen.insert(package, features)
                    && previous != features.as_slice()
                {
                    bail!(
                        "package {package:?} is required with two feature sets \
                         ({previous:?} and {:?}); give the components distinct packages",
                        features
                    );
                }
            }
        }
        for app in self.apps.values() {
            if let Some(previous) = seen.insert(&app.package, &app.features)
                && previous != app.features.as_slice()
            {
                bail!(
                    "package {:?} is required with two feature sets ({previous:?} and {:?})",
                    app.package,
                    app.features
                );
            }
        }
        Ok(())
    }
}
