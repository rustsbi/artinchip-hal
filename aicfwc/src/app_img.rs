//! Image generation support: `"AIC "` boot containers and packed `AIC.FW`
//! multi-partition images.
//!
//! * [`aic`] builds the 256-byte `"AIC "` header and the container used by the
//!   BootROM/`loader.aic` (this is the "buildAIC header" piece).
//! * [`elf`] turns an application ELF into a loader payload directly.
//! * [`img`] builds the packed `AIC.FW` image that the AiBurn host tool burns.
//! * [`manifest`] + [`toml_lite`] turn a TOML description into an image, so no
//!   intermediate `--raw-img` file needs to be generated and patched.

pub mod aic;
pub mod elf;
pub mod img;
pub mod manifest;
pub mod pbp;
pub mod toml_lite;

pub use aic::{AicHeader, AicImageBuilder, build_aic_header, wrap_resource};
pub use elf::ElfImage;
pub use img::{FwComponent, FwImage};
pub use manifest::{ComponentKind, ComponentSpec, ImageManifest, PartitionSpec};
