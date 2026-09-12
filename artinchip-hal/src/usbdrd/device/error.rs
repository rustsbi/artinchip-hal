//! USB device errors.

use core::fmt;

/// USB device controller operation error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbDevError {
    /// A controller operation did not complete before its polling limit.
    Timeout,
    /// The endpoint address is outside the controller's EP0..EP4 range.
    InvalidEndpoint,
    /// The endpoint maximum packet size is not valid for its type.
    InvalidMaxPacketSize,
    /// The endpoint is already processing a transfer.
    EndpointBusy,
    /// DMA requires a non-null, 32-bit aligned buffer.
    InvalidDmaBuffer,
    /// The transfer is larger than the endpoint transfer-size register.
    TransferTooLong,
    /// The controller reported an AHB access error.
    Ahb,
    /// The USB setup request is unsupported.
    UnsupportedRequest,
}

impl fmt::Display for UsbDevError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Timeout => "USB device operation timed out",
            Self::InvalidEndpoint => "invalid USB endpoint",
            Self::InvalidMaxPacketSize => "invalid USB endpoint maximum packet size",
            Self::EndpointBusy => "USB endpoint is busy",
            Self::InvalidDmaBuffer => "invalid USB DMA buffer",
            Self::TransferTooLong => "USB transfer is too long",
            Self::Ahb => "USB controller AHB error",
            Self::UnsupportedRequest => "unsupported USB setup request",
        })
    }
}

impl core::error::Error for UsbDevError {}
