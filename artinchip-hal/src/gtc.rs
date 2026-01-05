//! Generic Timer Controller (GTC).

mod delay;
mod gtc_ext;
mod instance;
mod register;

pub use delay::*;
pub use gtc_ext::GtcExt;
pub use instance::Gtc;
pub use register::*;
