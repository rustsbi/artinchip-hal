//! SPL / AIC boot image support.
//!
//! Locating a loader inside an `"AIC "` image, copying it to its load address,
//! and jumping to it across first-stage PBPs and second-stage bootloaders.

mod boot;
mod image;
mod role;

pub use boot::{boot_from, boot_loader, halt, jump_to};
pub use image::{
    AIC_HEADER_SIZE, APP_NAME_MAX, Error, Loader, NandRead, Ram, ReadAt, load, loader_fits_window,
    resolve,
};
pub use role::{Role, role, set_role};
