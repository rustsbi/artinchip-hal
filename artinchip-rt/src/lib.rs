//! Bare-metal ROM runtime for ArtInChip chips.
#![no_std]

pub use artinchip_rt_macros::pbp_entry;

#[macro_use]
pub mod macros;
pub mod core;
pub mod gpio;
pub mod pbp;
pub mod soc;

/// ArtInChip RT prelude.
pub mod prelude {
    pub use crate::gpio::PadExt as _;
}

#[cfg(feature = "d13x")]
pub use soc::d13x::Peripherals;

#[cfg(not(feature = "d13x"))]
/// Mock peripheral struct for unselected chips.
pub struct Peripherals {}
