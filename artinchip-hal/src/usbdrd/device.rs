//! USB device controller and `embassy-usb-driver` implementation.

pub mod dev_register;
mod device_ext;
mod driver;
#[cfg(feature = "async-usb")]
pub mod embassy;
mod error;
mod instance;

pub use super::{UsbConfig, UsbPads};
pub use device_ext::UsbDevExt;
pub use driver::*;
#[cfg(feature = "async-usb")]
pub use embassy::*;
pub use error::UsbDevError;
pub use instance::UsbDev;
