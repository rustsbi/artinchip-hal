//! ArtInChip cache management.

use core::sync::atomic::{Ordering, fence};
use log::error;
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
use xuantie_riscv::asm::{dcache_ciall, dcache_cipa, dcache_ipa, icache_iall};
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
use xuantie_riscv::register::mhcr;

/// Enable I-Cache and D-Cache.
///
/// Must be called after vector table initialization and before `main`.
#[doc(hidden)]
#[unsafe(no_mangle)]
pub extern "C" fn _enable_cache() {
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    unsafe {
        dcache_ciall();
        icache_iall();
        mhcr::set_ie();
        mhcr::set_de();
    }
}

/// Disable I-Cache and D-Cache.
#[doc(hidden)]
#[unsafe(no_mangle)]
pub extern "C" fn _disable_cache() {
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    unsafe {
        mhcr::clear_ie();
        mhcr::clear_de();
    }
}

/// Clean + invalidate both caches before PBP service-call return.
#[doc(hidden)]
#[unsafe(no_mangle)]
pub extern "C" fn _flush_cache() {
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    unsafe {
        dcache_ciall();
        icache_iall();
    }
}

#[cfg(not(feature = "d21x"))]
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
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        unsafe {
            dcache_cipa(p);
        }
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        let _ = p;
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
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        unsafe {
            dcache_ipa(p);
        }
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        let _ = p;
        p += CACHE_LINE;
    }
    fence(Ordering::SeqCst);
}
