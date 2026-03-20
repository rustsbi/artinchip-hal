//! Extension traits for GPIO pads.

use super::function::Function;
use super::input::Input;
use super::output::Output;

/// Extension of `GpioPad` or `&mut GpioPad`, for GPIO group `G` and number `N`.
pub trait PadExt<'a, const G: char, const N: u8> {
    /// Configures the pad to operate as a pull up output pad.
    fn into_pull_up_output(self) -> Output<'a, G, N>;
    /// Configures the pad to operate as a pull down output pad.
    fn into_pull_down_output(self) -> Output<'a, G, N>;
    /// Configures the pad to operate as a floating output pad.
    fn into_floating_output(self) -> Output<'a, G, N>;
    /// Configures the pad to operate as a pull up input pad.
    fn into_pull_up_input(self) -> Input<'a, G, N>;
    /// Configures the pad to operate as a pull down input pad.
    fn into_pull_down_input(self) -> Input<'a, G, N>;
    /// Configures the pad to operate as a floating input pad.
    fn into_floating_input(self) -> Input<'a, G, N>;
    /// Configures the pad to operate as a function mode pad with function `F`.
    fn into_function<const F: u8>(self) -> Function<'a, G, N, F>;
}
