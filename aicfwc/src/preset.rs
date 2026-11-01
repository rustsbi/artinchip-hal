//! Board presets: declarative firmware image descriptions.
//!
//! A preset is the single source of truth for one board: its chip and cargo
//! target, flash media, on-flash partition table, boot chain, and per-application
//! properties (partition, AIC name, link address, features). Shipped presets live
//! in `boards/` at the repository root (`boards/d13x-nor.toml`,
//! `boards/d21x-nand.toml`).
//!
//! # Schema
//!
//! ```toml
//! [board]                                  # optional: chip, target and profile
//! name    = "d13x-nor"                     # default: file stem
//! chip    = "d13x"                         # d13x | d21x
//! target  = "riscv32imafc-unknown-none-elf"# default: the chip's triple
//! profile = "release"                      # default: release
//!
//! [image]                                  # optional: image header metadata
//! platform     = "d13x"                    # default: the chip name
//! product      = "general-board"
//! version      = "0.1.0"
//! media        = "spi-nor"                 # default: the chip's media
//! media_dev_id = 0
//! media_id     = ""                        # e.g. "P=2K,B=128K" for SPI-NAND
//! app          = ""                        # boot target name; empty = first valid
//!
//! [[partition]]                            # the on-flash layout, in order
//! name   = "bootloader"
//! offset = 0x000000
//! size   = 0x080000
//!
//! [[component]]                            # the boot chain, in pack order
//! kind     = "app"                         # app | pbp | raw (default raw)
//! name     = "bootloader"                  # AIC/header name the runtime matches
//! package  = "app-bootloader"              # cargo package; resolved under target/
//! filename = "bootloader.aic"
//!
//! [apps.app-blinky]                        # one table per application package
//! name         = "blinky"                  # AIC header name
//! partition    = "app"
//! filename     = "app-blinky.aic"
//! load_address = 0x40700000                # link base (`APP_BASE`)
//! features     = ["d13x"]                  # cargo features, empty = defaults
//! ```

mod parse;
mod plan;
mod schema;
mod validate;

pub use plan::*;
pub use schema::*;

use std::{
    env, fs,
    path::{Path, PathBuf},
};

/// Find the repository's `boards/` directory above `start`.
pub fn boards_dir_from(start: &Path) -> Option<PathBuf> {
    for ancestor in start.ancestors() {
        let boards = ancestor.join("boards");
        if boards.is_dir() {
            return Some(boards);
        }
    }
    None
}

/// Locate the `boards/` directory for a `--board <name>` argument.
///
/// Search order: `AICFW_BOARDS_DIR`, the current directory's ancestors, then the
/// directory this crate was compiled in.
pub fn builtin_boards_dir() -> Option<PathBuf> {
    if let Some(dir) = env::var_os("AICFW_BOARDS_DIR") {
        let dir = PathBuf::from(dir);
        if dir.is_dir() {
            return Some(dir);
        }
    }
    if let Ok(cwd) = env::current_dir()
        && let Some(boards) = boards_dir_from(&cwd)
    {
        return Some(boards);
    }
    option_env!("CARGO_MANIFEST_DIR").and_then(|dir| boards_dir_from(Path::new(dir)))
}

pub(crate) fn known_presets(boards_dir: &Path) -> Vec<(String, PathBuf)> {
    let Ok(entries) = fs::read_dir(boards_dir) else {
        return Vec::new();
    };
    entries
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file()
                && path
                    .extension()
                    .is_some_and(|extension| extension == "toml")
        })
        .filter_map(|path| {
            let name = path.file_stem()?.to_str()?.to_string();
            Some((name, path))
        })
        .collect()
}
