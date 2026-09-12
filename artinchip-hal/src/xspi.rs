//! Expanded Serial Peripheral Interface (XSPI).

mod config;
mod driver;
mod error;
mod instance;
mod register;
mod xspi_ext;

pub use config::*;
pub use driver::*;
pub use error::*;
pub use instance::Xspi;
pub use register::*;
pub use xspi_ext::XspiExt;
