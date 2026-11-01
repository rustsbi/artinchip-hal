//! Bare-metal ROM and application runtime for ArtInChip chips.
#![no_std]
#![cfg_attr(
    any(target_arch = "riscv32", target_arch = "riscv64"),
    feature(abi_riscv_interrupt)
)]

// Ensure `riscv` (`critical-section-single-hart`) is linked for downstream crates.
use riscv as _;

pub use artinchip_rt_macros::{app_entry, boot_entry, pbp_entry};

#[macro_use]
#[cfg(any(feature = "d13x", feature = "d21x"))]
pub mod macros;
pub mod app;
pub mod core;
pub mod entry;
pub mod gpio;
pub use entry::pbp;
pub mod soc;
pub mod sync;
#[cfg(all(feature = "async", any(feature = "d13x", feature = "d21x")))]
pub mod time;

#[doc(hidden)]
#[cfg(not(feature = "d21x"))]
pub use crate::{
    __app_startup_d13x as __rt_app_startup, __boot_startup_d13x as __rt_boot_startup,
    __pbp_startup_d13x as __rt_pbp_startup,
};
#[doc(hidden)]
#[cfg(feature = "d21x")]
pub use crate::{
    __app_startup_d21x as __rt_app_startup, __boot_startup_d21x as __rt_boot_startup,
    __pbp_startup_d21x as __rt_pbp_startup,
};

#[cfg(feature = "async")]
pub use embassy_executor;
#[cfg(feature = "async")]
pub use embassy_time;

/// ArtInChip RT prelude.
pub mod prelude {
    pub use crate::core::{boot_rom::*, cache::*};
    pub use crate::gpio::PadExt as _;
}

#[cfg(feature = "d13x")]
pub use soc::d13x::{Peripherals, default_clk_tree};

#[cfg(feature = "d21x")]
pub use soc::d21x::{DRAM_BASE, Peripherals, default_clk_tree};

#[cfg(not(any(feature = "d13x", feature = "d21x")))]
/// Mock peripheral struct for unselected chips.
pub struct Peripherals {}
