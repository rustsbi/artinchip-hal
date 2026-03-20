//! DMA instance.

use super::channel::{DmaChannel, DmaChannels};
use super::dma_ext::DmaExt;
use super::register::RegisterBlock;
use crate::cmu::Cmu;
use core::marker::PhantomData;

/// DMA instance.
pub struct Dma {
    reg: *const RegisterBlock,
    _private: PhantomData<()>,
}

impl Dma {
    /// Create a new DMA instance.
    pub const fn __new(reg: *const RegisterBlock) -> Self {
        Self {
            reg,
            _private: PhantomData,
        }
    }

    /// Get a reference to the register block.
    pub const fn register_block(&self) -> &'static RegisterBlock {
        unsafe { &*self.reg }
    }
}

impl DmaExt for Dma {
    fn split(self, cmu: &mut Cmu) -> DmaChannels {
        DmaChannels {
            ch0: DmaChannel::<0>::__new(self.register_block(), cmu),
            ch1: DmaChannel::<1>::__new(self.register_block(), cmu),
            ch2: DmaChannel::<2>::__new(self.register_block(), cmu),
            ch3: DmaChannel::<3>::__new(self.register_block(), cmu),
            ch4: DmaChannel::<4>::__new(self.register_block(), cmu),
            ch5: DmaChannel::<5>::__new(self.register_block(), cmu),
            ch6: DmaChannel::<6>::__new(self.register_block(), cmu),
            ch7: DmaChannel::<7>::__new(self.register_block(), cmu),
        }
    }
}
