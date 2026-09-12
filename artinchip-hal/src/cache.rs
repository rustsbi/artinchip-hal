//! ArtInChip cache management.

use core::sync::atomic::{Ordering, fence};
use log::error;
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
use xuantie_riscv::asm::{dcache_cipa, dcache_ipa};

/// D-cache line size in bytes: 64 for `d21x`, 32 otherwise.
#[cfg(feature = "d21x")]
pub const CACHE_LINE: usize = 64;
#[cfg(not(feature = "d21x"))]
pub const CACHE_LINE: usize = 32;

/// Clean + invalidate D-cache for a physical range.
///
/// # Safety
/// `addr`/`len` must be physical and valid, and the caller must synchronize with
/// other agents (e.g. DMA).
#[inline]
pub unsafe fn dcache_clean_invalidate_range(addr: usize, len: usize) {
    if len == 0 {
        error!("dcache_clean_invalidate_range called with len=0");
        return;
    }

    let start = addr & !(CACHE_LINE - 1);
    let end = (addr + len + CACHE_LINE - 1) & !(CACHE_LINE - 1);
    let mut p = start;
    while p < end {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        unsafe {
            dcache_cipa(p);
        }
        // Off RISC-V there is no D-cache to touch (host builds/tests only).
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        let _ = p;
        p += CACHE_LINE;
    }
    fence(Ordering::SeqCst);
}

/// Invalidate D-cache for a physical range.
///
/// # Safety
/// `addr`/`len` must be physical and valid, and the caller must call this after
/// external writes (e.g. DMA).
#[inline]
pub unsafe fn dcache_invalidate_range(addr: usize, len: usize) {
    if len == 0 {
        error!("dcache_invalidate_range called with len=0");
        return;
    }

    let start = addr & !(CACHE_LINE - 1);
    let end = (addr + len + CACHE_LINE - 1) & !(CACHE_LINE - 1);
    let mut p = start;
    while p < end {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        unsafe {
            dcache_ipa(p);
        }
        // Off RISC-V there is no D-cache to touch (host builds/tests only).
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        let _ = p;
        p += CACHE_LINE;
    }
    fence(Ordering::SeqCst);
}
