//! PWM pad.

pub trait PwmPads<const I: u8> {}
pub trait PortA<const I: u8> {}
pub trait PortB<const I: u8> {}

impl<const I: u8, PA, PB> PwmPads<I> for (Option<PA>, Option<PB>)
where
    PA: PortA<I>,
    PB: PortB<I>,
{
}
