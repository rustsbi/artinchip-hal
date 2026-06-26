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

// Only look for the external dispatcher if the interrupts feature is active
#[cfg(feature = "interrupts")]
unsafe extern "C" {
    unsafe fn __artinchip_dispatch_interrupt(irq_id: usize);
}

/// The default trap handler for Machine Mode (M-Mode).
///
/// # Safety
///
/// This function must only be invoked by the hardware trap mechanism.
/// It relies on the `"riscv-interrupt-m"` ABI to automatically save and
/// restore the context (registers) before and after execution.
#[unsafe(no_mangle)]
pub unsafe extern "riscv-interrupt-m" fn DefaultTrapHandler() {
    let mcause: usize;
    unsafe {
        core::arch::asm!("csrr {}, mcause", out(reg) mcause);
    }

    let is_interrupt = (mcause as isize) < 0;

    if is_interrupt {
        // Pass to the user's dispatcher
        #[cfg(feature = "interrupts")]
        unsafe {
            __artinchip_dispatch_interrupt(mcause & 0xFFF);
        }

        // If a hardware interrupt somehow fires while the feature
        // is off, safely catch it in a spin loop to prevent unpredictable behavior.
        #[cfg(not(feature = "interrupts"))]
        loop {
            core::hint::spin_loop();
        }
    } else {
        panic!("Hardware Exception Fatal Error! mcause: {:#X}", mcause);
    }
}
