//! Quad Serial Peripheral Interface (QSPI).

mod blocking;
mod config;
mod instance;
mod pad;
mod pin_mux;
mod qspi_ext;
mod register;

pub use blocking::*;
pub use config::*;
pub use instance::Qspi;
pub use pad::*;
pub use qspi_ext::QspiExt;
pub use register::*;
