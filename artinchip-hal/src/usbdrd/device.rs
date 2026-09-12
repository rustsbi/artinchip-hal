//! USB device.

mod aic_upg;
pub mod dev_register;
mod device_ext;
mod driver;
mod error;
mod instance;

pub use aic_upg::*;
pub use device_ext::UsbDevExt;
pub use driver::*;
pub use error::UsbDevError;
pub use instance::UsbDev;
