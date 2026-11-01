//! Image entry points.
//!
//! Each image kind has its own startup path:
//!
//! * [`pbp`] — first-stage Pre-Boot Program, return-capable;
//! * [`app`] — application image entry;
//! * [`bootloader`] — boot-chain bootloader entry, fixed to the bootloader
//!   partition.

pub mod app;
pub mod bootloader;
pub mod pbp;
