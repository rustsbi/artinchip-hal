//! Image generation support: `"AIC "` boot containers and packed `AIC.FW`
//! multi-partition images.
//!
//! * [`aic`] builds the 256-byte `"AIC "` header and the container used by the
//!   BootROM and `loader.aic`.
//! * [`elf`] turns an application or PBP ELF into a flat payload directly.
//! * [`img`] builds the packed `AIC.FW` image that the AiBurn host tool burns.
//! * [`pbp`] builds and repairs `"PBP "` resource payloads.

pub mod aic;
pub mod elf;
pub mod img;
pub mod pbp;

pub use aic::{AicHeader, AicImageBuilder, build_aic_header, wrap_resource};
pub use elf::ElfImage;
pub use img::{FwComponent, FwImage};
