//! ArtInChip RT Trap Handler.

use core::arch::global_asm;
use log::error;
use riscv::register::*;

// The trampoline is used as an `mtvec` target.  CLIC-style chips (D13x and
// siblings) run it in vectored mode, which requires 64-byte alignment.
// D21x/C906 uses standard RISC-V direct mode instead, so 4-byte alignment is
// enough there; a 64-byte demand would also fight the fixed D21x `.text` VMA.
#[cfg(not(feature = "d21x"))]
global_asm!(
    "
    .align 6
    .global AlignedTrapHandler
    AlignedTrapHandler:
        j DefaultTrapHandler
    "
);

#[cfg(feature = "d21x")]
global_asm!(
    "
    .balign 4
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
    let mcause = mcause::read();
    let mepc = mepc::read();
    let mtval = mtval::read();
    let mstatus = mstatus::read();
    let mtvec = mtvec::read();
    let mip = mip::read();
    let mhcr: usize;
    let mhint: usize;
    unsafe {
        core::arch::asm!("csrr {}, 0x7C1", out(reg) mhcr);
        core::arch::asm!("csrr {}, 0x7C5", out(reg) mhint);
    };

    let kind = if mcause.is_interrupt() {
        "Interrupt"
    } else {
        "Exception"
    };

    error!(
        "TRAP: {} (MCAUSE={}, MEPC={:#010X}, MTVAL={:#010X})",
        kind,
        mcause.code(),
        mepc,
        mtval
    );
    error!("MSTATUS={:#010X}", mstatus.bits());
    error!("MTVEC={:#010X}", mtvec.bits());
    error!("MIP={:#010X}", mip.bits());
    error!("MHCR={:#010X}", mhcr);
    error!("MHINT={:#010X}", mhint);
    #[cfg(any(
        feature = "d12x",
        feature = "d13x",
        feature = "g73x",
        feature = "m6800"
    ))]
    {
        let mexstatus: usize;
        unsafe {
            core::arch::asm!("csrr {}, 0x7E1", out(reg) mexstatus);
        }
        error!("MEXSTATUS={:#010X}", mexstatus);
    }

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
    /// This function does nothing, only set the default trap.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn _init_vector_table() {
        #[cfg(any(
            feature = "d12x",
            feature = "d13x",
            feature = "g73x",
            feature = "m6800"
        ))]
        unsafe {
            // Set mtvec to AlignedTrapHandler in CLIC mode (MODE=3),
            // matching the interrupts path.
            unsafe extern "C" {
                fn AlignedTrapHandler();
            }
            let trap_addr = (AlignedTrapHandler as *const () as usize & !0x3) | 3;
            core::arch::asm!("csrw mtvec, {}", in(reg) trap_addr);

            riscv::interrupt::enable();
        }

        #[cfg(feature = "d21x")]
        unsafe {
            // C906 uses standard RISC-V mtvec direct mode here.  A PBP must
            // not inherit the BROM trap vector: a synchronous fault would
            // otherwise return control to ROM instead of stopping in PBP.
            unsafe extern "C" {
                fn AlignedTrapHandler();
            }
            let trap_addr = AlignedTrapHandler as *const () as usize & !0x3;
            core::arch::asm!("csrw mtvec, {}", in(reg) trap_addr);
        }
    }
}
