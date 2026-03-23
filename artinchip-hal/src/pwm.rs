//! Pulse Width Modulation (PWM).

mod config;
mod driver;
mod instance;
mod pad;
mod pwm_ext;
mod register;

pub use config::*;
pub use driver::*;
pub use instance::*;
pub use pad::*;
pub use pwm_ext::PwmExt;
pub use register::*;
