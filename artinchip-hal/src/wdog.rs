//! Watch Dog (WDOG).

mod driver;
mod instance;
mod register;
mod wdog_ext;

pub use driver::*;
pub use instance::Wdog;
pub use register::*;
pub use wdog_ext::WdogExt;
