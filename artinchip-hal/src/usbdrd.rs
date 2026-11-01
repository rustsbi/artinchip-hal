//! ArtInChip USB (dual-role device/host) support.
//!
//! # Inferred IP
//! - **Device**: Synopsys DesignWare Cores USB 2.0 Hi-Speed OTG Controller (DWC2).
//! - **Host**: STMicroelectronics EHCI 2.0 + OHCI 1.1 companion host controller.
//! - **PHY**: Integrated USB 2.0 High-Speed analog PHY (UTMI+).

mod config;
pub mod device;
pub mod host;
mod pad;

pub use config::*;
pub use pad::*;
