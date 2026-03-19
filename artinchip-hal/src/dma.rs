//! Direct Memory Access (DMA).

mod channel;
mod dma_ext;
mod instance;
mod register;
mod task;

pub use channel::*;
pub use dma_ext::DmaExt;
pub use instance::Dma;
pub use register::*;
pub use task::*;
