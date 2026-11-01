//! ArtInChip firmware image packer and board preset library.

#[cfg(feature = "cli")]
pub mod app_img;
#[cfg(feature = "cli")]
mod checksum;
#[cfg(feature = "cli")]
pub mod pack;
pub mod preset;
#[cfg(feature = "cli")]
pub mod raw_img;
mod toml_lite;
#[cfg(feature = "cli")]
mod util;
