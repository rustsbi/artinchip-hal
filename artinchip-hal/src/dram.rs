//! DRAM controller and PHY bring-up.

mod config;
mod dram_ext;
mod driver;
mod error;
mod instance;
mod register;
mod sequence;

pub use config::*;
pub use dram_ext::DramExt;
pub use driver::DramDriver;
pub use error::DramError;
pub use instance::Dram;
pub use register::*;
