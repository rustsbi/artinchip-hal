//! Bare-metal ROM runtime for ArtInChip chips.
#![no_std]
#![feature(abi_riscv_interrupt)]

pub use artinchip_rt_macros::{app_entry, pbp_entry};

#[macro_use]
#[cfg(any(feature = "d13x", feature = "d21x"))]
pub mod macros;
/// Application image runtime (loader-style entry).
#[cfg(feature = "app")]
pub mod app;
pub mod core;
pub mod gpio;
pub mod pbp;
pub mod soc;

/// ArtInChip RT prelude.
pub mod prelude {
    pub use crate::core::{boot_rom::*, cache::*};
    pub use crate::gpio::PadExt as _;
}

#[cfg(feature = "d13x")]
pub use soc::d13x::Peripherals;

#[cfg(feature = "d21x")]
pub use soc::d21x::Peripherals;

#[cfg(not(any(feature = "d13x", feature = "d21x")))]
/// Mock peripheral struct for unselected chips.
pub struct Peripherals {}
