//! WRI extension traits.

use super::info::ResetInfo;

pub trait WriExt<'a> {
    /// Creates a new ResetInfo instance.
    fn new_reset_info(self) -> ResetInfo<'a>;
}
