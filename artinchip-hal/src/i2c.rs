//! Inter-Integrated Circuit (I2C).

mod blocking;
mod config;
mod i2c_ext;
mod instance;
mod pad;
mod pin_mux;
mod register;

pub use blocking::*;
pub use config::*;
pub use i2c_ext::I2cExt;
pub use instance::I2c;
pub use register::*;
