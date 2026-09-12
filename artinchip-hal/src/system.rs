//! SPL / AIC boot image support.
//!
//! Locating a loader inside an `"AIC "` image, copying it to its load address
//! and jumping to it. This is the piece the first-stage PBP and the bootloader
//! share: both need to resolve a loader from flash and hand control to it.

mod boot;
mod image;
mod role;

pub use boot::{boot_from, boot_loader, jump_to};
pub use image::{AIC_HEADER_SIZE, APP_NAME_MAX, Error, Loader, ReadAt, load, resolve};
pub use role::{Role, role, set_role};
