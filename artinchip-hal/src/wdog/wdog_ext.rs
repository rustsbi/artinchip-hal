//! WDOG extension traits.

use super::driver::WdogDriver;
use crate::cmu::Cmu;

pub trait WdogExt<'a> {
    /// Create a new WDOG driver.
    fn new_driver(self, cmu: &Cmu) -> WdogDriver<'a>;
}
