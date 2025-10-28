//! Bare-metal ROM runtime for ArtInChip chips.
#![no_std]

pub use artinchip_rt_macros::pbp_entry;

pub mod pbp;
pub mod soc;

#[cfg(feature = "d13x")]
pub use soc::d13x::Peripherals;

#[cfg(not(feature = "d13x"))]
/// Mock peripheral struct for unselected chips.
pub struct Peripherals {}
