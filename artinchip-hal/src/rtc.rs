//! Real Time Clock (RTC).

mod instance;
mod register;
mod rtc_ext;
mod time;

pub use instance::Rtc;
pub use register::*;
pub use rtc_ext::RtcExt;
pub use time::*;
