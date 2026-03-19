//! DMA extension traits.

use super::channel::DmaChannels;
use crate::cmu::Cmu;

pub trait DmaExt {
    /// Split the DMA into independent channels.
    fn split(self, cmu: &Cmu) -> DmaChannels;
}
