//! USB pad traits.

/// USB data-minus pad.
pub trait UsbDm<const I: u8> {}

/// USB data-plus pad.
pub trait UsbDp<const I: u8> {}

/// USB pads.
pub trait UsbPads<const I: u8> {}

impl<const I: u8, DM, DP> UsbPads<I> for (DM, DP)
where
    DM: UsbDm<I>,
    DP: UsbDp<I>,
{
}
