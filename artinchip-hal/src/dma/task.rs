//! DMA task descriptor.

use super::register::*;
use core::ptr::NonNull;

/// DMA task descriptor - exactly matches SDK struct aic_dma_task
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C, align(8))]
pub struct DmaTask {
    // ========== Hardware-accessible fields (read by DMA controller) ==========
    /// Task configuration
    pub cfg: ChConfig,
    /// Source address.
    pub src: u32,
    /// Destination address.
    pub dst: u32,
    /// Transfer length in bytes.
    pub len: u32,
    /// Delay parameter between tasks.
    pub delay: u32,
    /// Physical address of next task (0xFFFFF800 = end).
    pub p_next: u32,

    // ========== Software-only fields (not directly used by DMA controller) ==========
    /// Handshake mode
    pub mode: ChMode,
    /// Virtual pointer to next task.
    pub v_next: Option<NonNull<DmaTask>>,
}

impl DmaTask {
    /// End of DMA task chain end flag.
    pub const TASK_END: u32 = 0xFFFFF800;
    /// Default delay value.
    pub const DEFAULT_DELAY: u32 = 0x40;

    pub fn append(&mut self, next: &mut DmaTask) {
        let mut last = self;

        for _ in 0..1_000 {
            if let Some(mut next_ptr) = last.v_next {
                last = unsafe { next_ptr.as_mut() };
            } else {
                break;
            }
        }

        if last.v_next.is_some() {
            panic!("DMA task chain too long or contains a cycle");
        }

        last.p_next = next as *const DmaTask as u32;
        last.v_next = Some(NonNull::from(next));
    }
}
