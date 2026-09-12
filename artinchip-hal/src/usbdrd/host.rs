//! USB host.
//!
//! Only the register block and the controller instance are kept for now; the
//! host mode driver/extension traits were removed while the device-side upgrade
//! work is in progress.

pub mod host_register;
mod instance;

pub use instance::UsbHost;
