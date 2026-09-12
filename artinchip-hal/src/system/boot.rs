//! Loader bring-up and hand-over.

use core::convert::Infallible;

use super::image::{self, Error, Loader, ReadAt};
use crate::cache::dcache_clean_invalidate_range;

/// Resolve, copy and jump to the AIC loader at `base` in `src`; never returns on
/// success.
///
/// # Safety
/// `base` must point at a valid AIC container whose loader load address is
/// writable, executable RAM.
pub unsafe fn boot_from(src: &mut impl ReadAt, base: u32) -> Result<Infallible, Error> {
    let loader = image::resolve(src, base)?;
    unsafe { boot_loader(src, &loader) }
}

/// Copy a resolved `loader` and jump to it.
///
/// # Safety
/// See [`boot_from`].
pub unsafe fn boot_loader(src: &mut impl ReadAt, loader: &Loader) -> Result<Infallible, Error> {
    image::load(src, loader)?;
    unsafe { dcache_clean_invalidate_range(loader.dst as usize, loader.length as usize) };
    unsafe { jump_to(loader.entry) }
}

/// Jump to `entry` after flushing caches and masking interrupts.
///
/// Caches are cleaned then disabled, the state the boot chain wants at hand-over;
/// interrupts are masked too, since `mstatus.MIE` and the CLIC enables survive the
/// jump and the next image must not take one through this vector table.
///
/// # Safety
/// `entry` must point at executable code; this never returns.
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub unsafe fn jump_to(entry: u32) -> ! {
    unsafe {
        riscv::interrupt::disable();
        xuantie_riscv::asm::dcache_ciall();
        xuantie_riscv::register::mhcr::clear_ie();
        xuantie_riscv::register::mhcr::clear_de();

        core::arch::asm!(
            "jr {entry}",
            // `reg` is pointer-wide, so widen the 32-bit entry even on RV64.
            entry = in(reg) entry as usize,
            options(noreturn, nomem, nostack),
        );
    }
}

/// Non-RISC-V (host) stub: the boot chain is unreachable there.
///
/// # Safety
/// Never call this off the RISC-V boot chain.
#[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
pub unsafe fn jump_to(_entry: u32) -> ! {
    unreachable!("jump_to is only implemented for RISC-V");
}
