//! SoC peripheral support for ArtInChip chips.
#![no_std]

pub mod axi_cfg;
pub mod ce;
pub mod cmu;
pub mod dma;
pub mod gpio;
pub mod sdmc;
pub mod uart;
pub mod wri;

mod macros;

/// Artinchip HAL prelude.
pub mod prelude {
    pub use crate::gpio::PadExt as _;
    pub use embedded_hal::digital::{InputPin as _, OutputPin as _, StatefulOutputPin as _};
}
