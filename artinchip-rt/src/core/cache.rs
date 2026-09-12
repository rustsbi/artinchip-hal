//! ArtInChip cache management.

use core::sync::atomic::{Ordering, fence};
use log::error;
use xuantie_riscv::asm::{dcache_ciall, dcache_cipa, dcache_ipa, icache_iall};
use xuantie_riscv::register::mhcr;

/// Enable I-Cache and D-Cache.
///
/// Must be called after vector table initialization and before main.
///
/// # Safety
/// Caller must ensure vector table is already written to memory
/// and I-cache will be coherent after the subsequent `fence.i`.
#[unsafe(no_mangle)]
pub(crate) extern "C" fn _enable_cache() {
    unsafe {
        dcache_ciall();
        icache_iall();
        mhcr::set_ie();
        mhcr::set_de();
    }
}

/// Disable I-Cache and D-Cache.
///
/// # Safety
/// Caller must ensure that no code will be executed from I-cache after this call.
#[unsafe(no_mangle)]
pub(crate) extern "C" fn _disable_cache() {
    unsafe {
        mhcr::clear_ie();
        mhcr::clear_de();
    }
}

/// Clean + invalidate both caches.
///
/// Used by the D13x PBP entry between `pbp_main` and restoring the caller's
/// `mhcr`: the run may have left dirty lines behind, and the caller resumes with
/// its own cache configuration.
#[cfg(not(feature = "d21x"))]
#[unsafe(no_mangle)]
pub(crate) extern "C" fn _flush_cache() {
    unsafe {
        dcache_ciall();
        icache_iall();
    }
}

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
        error!("dcache_clean_invalidate_range called with len=0");
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
        error!("dcache_invalidate_range called with len=0");
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
