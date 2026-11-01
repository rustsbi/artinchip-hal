//! Quad Serial Peripheral Interface (QSPI).

mod blocking;
mod bus;
mod config;
mod dma;
mod error;
mod instance;
mod pad;
mod qspi_ext;
mod register;

pub use blocking::*;
pub use bus::QuadSpiBus;
pub use config::*;
pub use dma::*;
pub use error::*;
pub use instance::Qspi;
pub use pad::*;
pub use qspi_ext::QspiExt;
pub use register::*;
