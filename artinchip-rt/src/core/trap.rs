//! ArtInChip RT trap and vector-table startup hooks.

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
use core::arch::global_asm;
use log::error;
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
use riscv::register::*;

// CLIC SoCs require 64-byte alignment on `mtvec`; D21x/C906 uses 4-byte aligned direct mode.
#[cfg(all(
    any(target_arch = "riscv32", target_arch = "riscv64"),
    not(feature = "d21x")
))]
global_asm!(
    "
    .align 6
    .global AlignedTrapHandler
    AlignedTrapHandler:
        j DefaultTrapHandler
    "
);

#[cfg(all(
    any(target_arch = "riscv32", target_arch = "riscv64"),
    feature = "d21x"
))]
global_asm!(
    "
    .balign 4
    .global AlignedTrapHandler
    AlignedTrapHandler:
        j DefaultTrapHandler
    "
);

/// Default M-mode trap handler.
///
/// # Safety
/// Must only be entered via the hardware `mtvec` trampoline.
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
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
    }

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

#[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
#[doc(hidden)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DefaultTrapHandler() {
    error!("TRAP: DefaultTrapHandler invoked on host");
    loop {
        core::hint::spin_loop();
    }
}

/// Initialize the active interrupt controller and trap/vector table during `_start`.
///
/// # Safety
/// Must only be called once from `_start` before caches are enabled.
#[cfg(feature = "interrupts")]
#[doc(hidden)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _init_vector_table() {
    unsafe { artinchip_hal::interrupt::init_vector_table() };
}

/// Minimal trap setup for builds without the `interrupts` feature.
#[cfg(not(feature = "interrupts"))]
mod placeholder {
    #[repr(C, align(64))]
    #[allow(dead_code)]
    struct ClicVectorTable([u32; 0]);

    #[unsafe(link_section = ".clic.vector_table")]
    static _PLACEHOLDER: ClicVectorTable = ClicVectorTable([0; 0]);

    /// Install `AlignedTrapHandler` into `mtvec` when full interrupt support is disabled.
    ///
    /// # Safety
    /// Must only be called once from `_start`.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn _init_vector_table() {
        #[cfg(all(
            any(target_arch = "riscv32", target_arch = "riscv64"),
            not(feature = "d21x")
        ))]
        unsafe {
            unsafe extern "C" {
                fn AlignedTrapHandler();
            }
            let trap_addr = (AlignedTrapHandler as *const () as usize & !0x3) | 3;
            core::arch::asm!("csrw mtvec, {}", in(reg) trap_addr);
            riscv::interrupt::enable();
        }

        #[cfg(all(
            any(target_arch = "riscv32", target_arch = "riscv64"),
            feature = "d21x"
        ))]
        unsafe {
            unsafe extern "C" {
                fn AlignedTrapHandler();
            }
            let trap_addr = AlignedTrapHandler as *const () as usize & !0x3;
            core::arch::asm!("csrw mtvec, {}", in(reg) trap_addr);
        }
    }
}
