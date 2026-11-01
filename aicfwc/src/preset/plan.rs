//! Image planning: composing a [`Preset`] and [`Selection`] into an [`ImagePlan`].

use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};

use anyhow::{Result, bail};

use super::schema::{BoardInfo, ComponentKind, ComponentSpec, ImageInfo, PartitionSpec, Preset};

/// What to package from a preset.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Selection {
    /// The boot chain alone: PBP updaters plus the first-stage bootloader image.
    Bootloader,
    /// The boot chain plus the `[apps.<package>]` application (a product image).
    App(String),
}

/// Where a component's payload comes from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Source {
    /// A cargo package built for the preset's triple and profile.
    Package {
        package: String,
        bin: String,
        features: Vec<String>,
    },
    /// A file relative to the preset (already joined to an absolute path).
    File(PathBuf),
}

/// A component of an [`ImagePlan`], with its payload source resolved.
#[derive(Clone, Debug)]
pub struct PlannedComponent {
    pub spec: ComponentSpec,
    pub source: Source,
    pub pbp_source: Option<Source>,
}

/// A cargo build a plan needs, deduplicated by package.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuildRequirement {
    pub package: String,
    pub bin: String,
    pub features: Vec<String>,
}

/// A preset composed with a [`Selection`]: everything needed to build and pack one image.
#[derive(Clone, Debug)]
pub struct ImagePlan {
    pub path: PathBuf,
    pub board: BoardInfo,
    pub image: ImageInfo,
    pub partitions: Vec<PartitionSpec>,
    /// Components in composition order: the boot chain in preset order, with the
    /// application inserted before the first boot-chain target so it becomes the
    /// first target record.
    pub components: Vec<PlannedComponent>,
    /// Cargo packages `--build` has to produce, in first-use order.
    pub required_builds: Vec<BuildRequirement>,
    /// File-name stem of the output image.
    pub stem: String,
}

impl ImagePlan {
    /// The directory relative paths in the plan resolve against.
    pub fn base_dir(&self) -> &Path {
        self.path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."))
    }

    /// The partition a meta `partition` field refers to.
    pub fn partition(&self, reference: &str) -> Option<&PartitionSpec> {
        let name = PartitionSpec::name_of(reference);
        self.partitions
            .iter()
            .find(|partition| partition.name == name)
    }
}

impl Preset {
    /// Compose the image a [`Selection`] asks for.
    pub fn plan(&self, selection: &Selection) -> Result<ImagePlan> {
        self.validate()?;

        let app_component = match selection {
            Selection::Bootloader => None,
            Selection::App(package) => {
                let app = self.app(package)?;
                Some(app.component())
            }
        };

        let mut names = BTreeSet::new();
        for component in self.components.iter().chain(app_component.iter()) {
            if !names.insert(component.name.clone()) {
                bail!(
                    "application {:?} reuses the boot-chain AIC name {:?}",
                    selection,
                    component.name
                );
            }
        }

        let mut required_builds: Vec<BuildRequirement> = Vec::new();
        for component in self.components.iter().chain(app_component.iter()) {
            for source in sources(component, self.dir()) {
                let Source::Package {
                    package,
                    bin,
                    features,
                } = source
                else {
                    continue;
                };
                if required_builds
                    .iter()
                    .any(|requirement| requirement.package == package)
                {
                    continue;
                }
                required_builds.push(BuildRequirement {
                    package,
                    bin,
                    features,
                });
            }
        }

        let app_component = app_component.map(|component| self.planned(&component));
        let mut components: Vec<PlannedComponent> = Vec::new();
        let mut app_inserted = false;
        for component in &self.components {
            if !app_inserted
                && !component.partition.is_empty()
                && let Some(app) = &app_component
            {
                components.push(app.clone());
                app_inserted = true;
            }
            components.push(self.planned(component));
        }
        if let Some(app) = app_component
            && !app_inserted
        {
            components.push(app);
        }

        Ok(ImagePlan {
            path: self.path.clone(),
            board: self.board.clone(),
            image: self.image.clone(),
            partitions: self.partitions.clone(),
            components,
            required_builds,
            stem: self.output_stem(selection),
        })
    }

    fn planned(&self, component: &ComponentSpec) -> PlannedComponent {
        let mut spec = component.clone();
        let source = if let Some(package) = spec.package.clone() {
            Source::Package {
                package,
                bin: spec.bin.clone().unwrap_or_else(|| {
                    spec.package
                        .clone()
                        .expect("checked above: the component names a package")
                }),
                features: spec.features.clone(),
            }
        } else if let Some(file) = spec.file.take() {
            Source::File(self.dir().join(file))
        } else {
            unreachable!("validated: a component names either `package` or `file`");
        };
        let pbp_source = match (&spec.pbp, &spec.pbp_package) {
            (Some(pbp), _) => Some(Source::File(self.dir().join(pbp))),
            (None, Some(package)) => Some(Source::Package {
                package: package.clone(),
                bin: package.clone(),
                features: spec.pbp_features.clone(),
            }),
            (None, None) => None,
        };
        spec.pbp = None;
        spec.private = spec.private.take().map(|private| self.dir().join(private));
        PlannedComponent {
            spec,
            source,
            pbp_source,
        }
    }

    fn output_stem(&self, selection: &Selection) -> String {
        match selection {
            Selection::App(package) => package.clone(),
            Selection::Bootloader => self
                .components
                .iter()
                .find(|component| {
                    component.kind == ComponentKind::App && !component.partition.is_empty()
                })
                .and_then(|component| {
                    component
                        .package
                        .clone()
                        .or_else(|| component.bin.clone())
                        .or_else(|| {
                            component
                                .file
                                .as_ref()
                                .and_then(|file| file.file_stem())
                                .and_then(|stem| stem.to_str())
                                .map(str::to_string)
                        })
                })
                .unwrap_or_else(|| self.board.name.clone()),
        }
    }
}

fn sources<'a>(component: &'a ComponentSpec, base_dir: &'a Path) -> Vec<Source> {
    let mut sources = Vec::new();
    if let Some(package) = &component.package {
        sources.push(Source::Package {
            package: package.clone(),
            bin: component.bin.clone().unwrap_or_else(|| package.clone()),
            features: component.features.clone(),
        });
    }
    if let Some(file) = &component.file {
        sources.push(Source::File(base_dir.join(file)));
    }
    if let Some(package) = &component.pbp_package {
        sources.push(Source::Package {
            package: package.clone(),
            bin: package.clone(),
            features: component.pbp_features.clone(),
        });
    }
    if let Some(pbp) = &component.pbp {
        sources.push(Source::File(base_dir.join(pbp)));
    }
    sources
}
