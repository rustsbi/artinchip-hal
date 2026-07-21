//! ArtInChip RT Trap Handler.

use core::arch::global_asm;

// 64-byte aligned trampoline for hardware vectoring requirement
global_asm!(
    "
    .align 6
    .global AlignedTrapHandler
    AlignedTrapHandler:
        j DefaultTrapHandler
    "
);

/// The default trap handler for Machine Mode (M-Mode).
///
/// # Safety
///
/// This function must only be invoked by the hardware trap mechanism.
/// It relies on the `"riscv-interrupt-m"` ABI to automatically save and
/// restore the context (registers) before and after execution.
#[unsafe(no_mangle)]
pub unsafe extern "riscv-interrupt-m" fn DefaultTrapHandler() {
    loop {
        core::hint::spin_loop();
    }
}

/// Placeholder vector table for non‑interrupt builds.
/// Required by the linker script (`.clic.vector_table` with `KEEP`).
/// Zero‑sized — merely marks the section so the linker doesn't error.
#[cfg(not(feature = "interrupts"))]
mod placeholder {
    #[repr(C, align(64))]
    #[allow(dead_code)]
    struct ClicVectorTable([u32; 0]);

    #[unsafe(link_section = ".clic.vector_table")]
    static _PLACEHOLDER: ClicVectorTable = ClicVectorTable([0; 0]);

    /// Empty implementation for when interrupts are not enabled.
    ///
    /// # Safety
    ///
    /// This function does nothing, only serves as a placeholder
    /// to satisfy the linker when interrupts are disabled.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn _init_vector_table() {
        // Do nothing if CLIC interrupts are not enabled in Cargo.toml
    }
}
