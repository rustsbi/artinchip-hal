//! I2C pad.

pub trait I2cPads<const I: u8> {}
pub trait SerialClock<const I: u8> {}
pub trait SerialData<const I: u8> {}

impl<const I: u8, SCL, SDA> I2cPads<I> for (SCL, SDA)
where
    SCL: SerialClock<I>,
    SDA: SerialData<I>,
{
}
