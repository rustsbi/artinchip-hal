use super::output::Output;

/// Extension of `GpioPad` or `&mut GpioPad`, for GPIO group `P` and number `N`.
pub trait PadExt<'a> {
    /// Configures the pad to operate as an output pad.
    fn into_output(self) -> Output<'a>;
    // TODO into_pull_up_input
    // TODO into_pull_down_input
    // TODO into_floating_input
}
