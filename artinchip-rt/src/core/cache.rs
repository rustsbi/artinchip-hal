//! ArtInChip cache management.

use core::sync::atomic::{Ordering, fence};
use xuantie_riscv::asm::{dcache_cipa, dcache_ipa};

#[cfg(any(
    feature = "d12x",
    feature = "d13x",
    feature = "g73x",
    feature = "m6800"
))]
pub const CACHE_LINE: usize = 32;
#[cfg(feature = "d21x")]
pub const CACHE_LINE: usize = 64;

/// Clean + invalidate D-cache for a physical range.
///
/// # Safety
/// - `addr`/`len` are physical and valid.
/// - Requires privilege to execute `th.dcache.cipa`.
/// - Caller must synchronize with other agents (e.g. DMA).
#[inline]
pub unsafe fn dcache_clean_invalidate_range(addr: usize, len: usize) {
    if len == 0 {
        return;
    }

    let start = addr & !(CACHE_LINE - 1);
    let end = (addr + len + CACHE_LINE - 1) & !(CACHE_LINE - 1);
    let mut p = start;
    while p < end {
        unsafe {
            dcache_cipa(p);
        }
        p += CACHE_LINE;
    }
    fence(Ordering::SeqCst);
}

/// Invalidate D-cache for a physical range.
///
/// # Safety
/// - `addr`/`len` are physical and valid.
/// - Requires privilege to execute `th.dcache.ipa`.
/// - Call after external writes (e.g. DMA).
#[inline]
pub unsafe fn dcache_invalidate_range(addr: usize, len: usize) {
    if len == 0 {
        return;
    }

    let start = addr & !(CACHE_LINE - 1);
    let end = (addr + len + CACHE_LINE - 1) & !(CACHE_LINE - 1);
    let mut p = start;
    while p < end {
        unsafe {
            dcache_ipa(p);
        }
        p += CACHE_LINE;
    }
    fence(Ordering::SeqCst);
}
