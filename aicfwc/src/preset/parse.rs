//! TOML deserialization into [`Preset`].

use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};

use super::schema::{
    AppSpec, BoardInfo, Chip, ComponentKind, ComponentSpec, ImageInfo, Media, PartitionSpec, Preset,
};
use crate::toml_lite::{self, Table, Value};

impl Preset {
    /// Parse a preset from TOML text.
    ///
    /// `path` supplies the default board name and the base directory of relative paths.
    pub fn parse(text: &str, path: &Path) -> Result<Self> {
        let root =
            toml_lite::parse(text).map_err(|error| anyhow::anyhow!("TOML parse error: {error}"))?;

        let board_table = table_field(&root, "board")?;
        let image_table = table_field(&root, "image")?;

        let chip = match (
            string_field(board_table, "chip")?,
            string_field(image_table, "platform")?,
        ) {
            (Some(chip), _) => Chip::parse(&chip)?,
            (None, Some(platform)) => Chip::parse(&platform)?,
            (None, None) => Chip::D13x,
        };

        let name = string_field(board_table, "name")?.unwrap_or_else(|| {
            path.file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("board")
                .to_string()
        });
        let target = string_field(board_table, "target")?
            .unwrap_or_else(|| chip.default_target().to_string());
        let profile =
            string_field(board_table, "profile")?.unwrap_or_else(|| "release".to_string());

        let image = ImageInfo {
            platform: string_field(image_table, "platform")?.unwrap_or_else(|| chip.name().into()),
            product: string_field(image_table, "product")?.unwrap_or_else(|| "unknown".into()),
            version: string_field(image_table, "version")?.unwrap_or_else(|| "0.0.0".into()),
            media: match string_field(image_table, "media")? {
                Some(media) => Media::parse(&media)?,
                None => chip.default_media(),
            },
            media_dev_id: int_field(image_table, "media_dev_id")?.unwrap_or(0),
            media_id: string_field(image_table, "media_id")?.unwrap_or_default(),
            boot_app: string_field(image_table, "app")?.unwrap_or_default(),
        };

        let mut partitions = Vec::new();
        for (index, entry) in array_field(&root, "partition")?.iter().enumerate() {
            let table = entry
                .as_table()
                .with_context(|| format!("partition #{index} is not a table"))?;
            let name = string_field(Some(table), "name")?
                .with_context(|| format!("partition #{index} is missing `name`"))?;
            let offset = int_field(Some(table), "offset")?
                .with_context(|| format!("partition `{name}` is missing `offset`"))?;
            let size = int_field(Some(table), "size")?
                .with_context(|| format!("partition `{name}` is missing `size`"))?;
            partitions.push(PartitionSpec { name, offset, size });
        }

        let mut components = Vec::new();
        for (index, entry) in array_field(&root, "component")?.iter().enumerate() {
            let table = entry
                .as_table()
                .with_context(|| format!("component #{index} is not a table"))?;
            components.push(parse_component(table, index)?);
        }

        let mut apps = BTreeMap::new();
        if let Some(table) = table_field(&root, "apps")? {
            for (package, value) in table {
                let app = value
                    .as_table()
                    .with_context(|| format!("[apps.{package}] is not a table"))?;
                apps.insert(package.clone(), parse_app(package, app)?);
            }
        }

        Ok(Self {
            path: path.to_path_buf(),
            board: BoardInfo {
                name,
                chip,
                target,
                profile,
            },
            image,
            partitions,
            components,
            apps,
        })
    }
}

fn parse_component(table: &Table, index: usize) -> Result<ComponentSpec> {
    let table = Some(table);
    let kind = match string_field(table, "kind")? {
        Some(kind) => ComponentKind::parse(&kind)?,
        None => ComponentKind::Raw,
    };
    let name = string_field(table, "name")?
        .with_context(|| format!("component #{index} is missing `name`"))?;
    let package = string_field(table, "package")?;
    let bin = string_field(table, "bin")?;
    let file = string_field(table, "file")?.map(PathBuf::from);
    let filename = string_field(table, "filename")?.unwrap_or_else(|| {
        bin.clone()
            .or_else(|| {
                file.as_ref().and_then(|file| {
                    file.file_name()
                        .and_then(|name| name.to_str())
                        .map(str::to_string)
                })
            })
            .or_else(|| package.clone())
            .unwrap_or_else(|| "component.bin".to_string())
    });

    let features = string_list_field(table, "features")?;
    let pbp_features = {
        let explicit = string_list_field(table, "pbp_features")?;
        if explicit.is_empty() {
            features.clone()
        } else {
            explicit
        }
    };

    Ok(ComponentSpec {
        kind,
        name,
        package,
        bin,
        file,
        partition: string_field(table, "partition")?.unwrap_or_default(),
        attr: string_field(table, "attr")?.unwrap_or_else(|| "mtd;required".to_string()),
        filename,
        target_attr: string_field(table, "target_attr")?,
        target_filename: string_field(table, "target_filename")?,
        updater: bool_field(table, "updater")?.unwrap_or(false),
        features,
        pbp_features,
        ram: int_field(table, "ram")?,
        pbp: string_field(table, "pbp")?.map(PathBuf::from),
        pbp_package: string_field(table, "pbp_package")?,
        private: string_field(table, "private")?.map(PathBuf::from),
        load_address: int_field(table, "load_address")?,
        entry_point: int_field(table, "entry_point")?,
        entry_offset: int_field(table, "entry_offset")?,
        header_version: int_field(table, "header_version")?,
        with_ext: bool_field(table, "with_ext")?.unwrap_or(false),
        ext_reserve: int_field(table, "ext_reserve")?.unwrap_or(0) as usize,
    })
}

fn parse_app(package: &str, table: &Table) -> Result<AppSpec> {
    let table = Some(table);
    let name = string_field(table, "name")?.unwrap_or_else(|| package.to_string());
    Ok(AppSpec {
        package: package.to_string(),
        partition: string_field(table, "partition")?.unwrap_or_else(|| "app".to_string()),
        filename: string_field(table, "filename")?.unwrap_or_else(|| format!("{name}.aic")),
        attr: string_field(table, "attr")?.unwrap_or_else(|| "mtd;optional".to_string()),
        features: string_list_field(table, "features")?,
        load_address: int_field(table, "load_address")?,
        entry_offset: int_field(table, "entry_offset")?,
        name,
    })
}

fn table_field<'a>(root: &'a Table, key: &str) -> Result<Option<&'a Table>> {
    match root.get(key) {
        None => Ok(None),
        Some(value) => value
            .as_table()
            .map(Some)
            .with_context(|| format!("`{key}` must be a table")),
    }
}

fn array_field<'a>(root: &'a Table, key: &str) -> Result<&'a [Value]> {
    match root.get(key).or_else(|| match key {
        "partition" => root.get("partitions"),
        "component" => root.get("components"),
        _ => None,
    }) {
        None => Ok(&[]),
        Some(value) => value
            .as_array()
            .with_context(|| format!("`[[{key}]]` must be written as an array of tables")),
    }
}

fn string_field(table: Option<&Table>, key: &str) -> Result<Option<String>> {
    match table.and_then(|table| table.get(key)) {
        None => Ok(None),
        Some(value) => value
            .as_str()
            .map(|value| Some(value.to_string()))
            .with_context(|| format!("`{key}` must be a string")),
    }
}

/// Read a list of strings spelled either as one string (`"a"`, `"a b"`, `"a,b"`)
/// or as an array (`["a", "b"]`), sorted and deduplicated.
fn string_list_field(table: Option<&Table>, key: &str) -> Result<Vec<String>> {
    let Some(value) = table.and_then(|table| table.get(key)) else {
        return Ok(Vec::new());
    };
    let mut features: Vec<String> = if let Some(list) = value.as_str() {
        list.split([' ', ','])
            .filter(|item| !item.is_empty())
            .map(str::to_string)
            .collect()
    } else {
        let items = value
            .as_array()
            .with_context(|| format!("`{key}` must be a string or an array of strings"))?;
        items
            .iter()
            .map(|item| {
                item.as_str()
                    .map(str::to_string)
                    .with_context(|| format!("`{key}` must contain only strings"))
            })
            .collect::<Result<_>>()?
    };
    features.sort();
    features.dedup();
    Ok(features)
}

fn int_field(table: Option<&Table>, key: &str) -> Result<Option<u32>> {
    match table.and_then(|table| table.get(key)) {
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

fn bool_field(table: Option<&Table>, key: &str) -> Result<Option<bool>> {
    match table.and_then(|table| table.get(key)) {
        None => Ok(None),
        Some(value) => value
            .as_bool()
            .map(Some)
            .with_context(|| format!("`{key}` must be a boolean")),
    }
}
