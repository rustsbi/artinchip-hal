//! PLIC interrupt controller backend (D21x / C906).

use core::num::NonZeroU32;
use core::sync::atomic::{AtomicUsize, Ordering};

use plic::{HartContext, InterruptSource};
use xuantie_riscv::peripheral::plic::Plic;

use super::define_interrupt_mod;

const PLIC_BASE: usize = 0x2000_0000;

/// Machine timer interrupt number (`CORET_IRQn`), routed via `mie.MTIE` rather than PLIC.
const CORE_TIMER_IRQ: u8 = 7;

/// D21x implements sources `0..=95` (CIR = 95); one handler slot per source.
pub const MAX_INTERRUPTS: usize = 96;

static HANDLERS: [AtomicUsize; MAX_INTERRUPTS] = [const { AtomicUsize::new(0) }; MAX_INTERRUPTS];

/// A PLIC hart context: context 0 is hart 0 M-mode, context 1 is hart 0 S-mode.
#[derive(Clone, Copy)]
struct Hart(usize);
impl HartContext for Hart {
    fn index(self) -> usize {
        self.0
    }
}

const H0_M: Hart = Hart(0);

/// A non-zero PLIC source ID.
#[derive(Clone, Copy)]
struct Source(u32);
impl InterruptSource for Source {
    fn id(self) -> NonZeroU32 {
        NonZeroU32::new(self.0).expect("PLIC source id must be non-zero")
    }
}

#[inline(always)]
fn plic() -> &'static Plic {
    // SAFETY: the D21x PLIC is memory-mapped at this fixed base for the image's lifetime.
    unsafe { &*(PLIC_BASE as *const Plic) }
}

#[inline(always)]
fn source(irq: u8) -> Source {
    Source(irq as u32)
}

/// Disable and drain every source of one context.
fn clear_context(p: &Plic, ctx: Hart) {
    for id in 1..MAX_INTERRUPTS as u32 {
        p.disable(Source(id), ctx);
    }
    p.set_threshold(ctx, 0);
    for id in 1..MAX_INTERRUPTS as u32 {
        p.complete(ctx, Source(id));
    }
}

/// Clear pending bits; the upstream PLIC API has no pending-clear operation.
#[inline]
unsafe fn clear_pending_bits() {
    for index in 0..MAX_INTERRUPTS.div_ceil(32) {
        unsafe {
            (PLIC_BASE as *mut u32).add(0x400 + index).write_volatile(0);
        }
    }
}

/// Initialize the PLIC to the vendor `interrupt_init` baseline.
///
/// # Safety
/// The PLIC peripheral must be mapped and owned by this image.
pub unsafe fn plic_init() {
    critical_section::with(|_| {
        let p = plic();
        for id in 1..MAX_INTERRUPTS as u32 {
            p.set_priority(Source(id), 31);
        }
        unsafe { clear_pending_bits() };
        for ctx in [H0_M, Hart(1)] {
            clear_context(p, ctx);
        }
        p.enable_supervisor_access();

        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        unsafe {
            riscv::register::mie::set_mext()
        };
    });
}

/// Mask every PLIC source and the machine timer, clearing stale claims.
///
/// # Safety
/// The PLIC peripheral must be mapped and owned by this image.
pub unsafe fn mask_all_interrupts() {
    critical_section::with(|_| {
        let p = plic();
        unsafe { clear_pending_bits() };
        for ctx in [H0_M, Hart(1)] {
            clear_context(p, ctx);
        }

        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        unsafe {
            riscv::register::mie::clear_mtimer()
        };
    });
}

/// Enable one interrupt source (either a PLIC line or `CPU_TIMER`).
///
/// # Safety
/// The PLIC peripheral must be mapped and `irq` must refer to a valid interrupt.
pub unsafe fn enable_interrupt(irq: u8) {
    if irq == CORE_TIMER_IRQ {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        unsafe {
            riscv::register::mie::set_mtimer();
        }
        return;
    }

    critical_section::with(|_| plic().enable(source(irq), H0_M));
}

/// Disable one interrupt source (either a PLIC line or `CPU_TIMER`).
///
/// # Safety
/// The PLIC peripheral must be mapped and `irq` must refer to a valid interrupt.
pub unsafe fn disable_interrupt(irq: u8) {
    if irq == CORE_TIMER_IRQ {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        unsafe {
            riscv::register::mie::clear_mtimer();
        }
        return;
    }

    critical_section::with(|_| plic().disable(source(irq), H0_M));
}

/// Report whether `irq` is enabled in context 0 (or `mie.MTIE` for `CPU_TIMER`).
pub fn is_enabled(irq: u8) -> bool {
    if irq == CORE_TIMER_IRQ {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        {
            return riscv::register::mie::read().mtimer();
        }
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        {
            return false;
        }
    }

    plic().is_enabled(source(irq), H0_M)
}

/// Set `irq`'s priority in the PLIC (`0..=31`; ignored for `CPU_TIMER`).
///
/// # Safety
/// The PLIC peripheral must be mapped and `irq` must refer to a valid interrupt.
pub unsafe fn set_priority(irq: u8, priority: u8) {
    if irq == CORE_TIMER_IRQ {
        return;
    }

    critical_section::with(|_| plic().set_priority(source(irq), priority as u32));
}

/// Report whether `irq` is pending in the PLIC (or `mip.MTIP` for `CPU_TIMER`).
pub fn is_pending(irq: u8) -> bool {
    if irq == CORE_TIMER_IRQ {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        {
            return riscv::register::mip::read().mtimer();
        }
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        {
            return false;
        }
    }

    plic().is_pending(source(irq))
}

/// No-op on PLIC (pending state clears via claim/peripheral acknowledgement).
///
/// # Safety
/// Kept `unsafe` for symmetry with the CLIC backend.
pub unsafe fn clear_pending(_irq: u8) {}

/// Claim the highest-priority pending PLIC source for context 0 (`0` if none).
#[inline]
pub fn claim() -> u32 {
    plic().claim(H0_M).map_or(0, NonZeroU32::get)
}

/// Signal completion of a source previously returned by [`claim`].
///
/// # Safety
/// `irq` must be a non-zero source ID obtained from [`claim`] on context 0.
#[inline]
pub unsafe fn complete(irq: u32) {
    if irq != 0 {
        plic().complete(H0_M, Source(irq));
    }
}

fn irq_trampoline<I: super::InterruptSource, H: super::Handler<I>>() {
    unsafe { H::on_interrupt() };
}

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64", test))]
#[inline]
fn dispatch_registered(irq: usize) {
    if irq < MAX_INTERRUPTS {
        let addr = HANDLERS[irq].load(Ordering::Acquire);
        if addr != 0 {
            let handler: fn() = unsafe { core::mem::transmute(addr) };
            handler();
        }
    }
}

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
core::arch::global_asm!(
    ".balign 4
     .global __plic_trap_vector
     .type __plic_trap_vector, @function
 __plic_trap_vector:
     j __plic_trap_entry
     .size __plic_trap_vector, . - __plic_trap_vector"
);

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
#[doc(hidden)]
#[unsafe(no_mangle)]
pub extern "riscv-interrupt-m" fn __plic_trap_entry() {
    let mcause = riscv::register::mcause::read();

    if mcause.is_interrupt() {
        match mcause.code() {
            7 => dispatch_registered(CORE_TIMER_IRQ as usize),
            _ => {
                let irq = claim();
                if irq != 0 {
                    dispatch_registered(irq as usize);
                    unsafe { complete(irq) };
                }
            }
        }
        return;
    }

    let mepc = riscv::register::mepc::read();
    let mtval = riscv::register::mtval::read();
    let mstatus = riscv::register::mstatus::read();
    let mtvec = riscv::register::mtvec::read();
    let mip = riscv::register::mip::read();
    let (mepc_addr, mhcr, mhint, mxstatus): (usize, usize, usize, usize);
    unsafe {
        core::arch::asm!("csrr {}, mepc", out(reg) mepc_addr);
        core::arch::asm!("csrr {}, 0x7C1", out(reg) mhcr);
        core::arch::asm!("csrr {}, 0x7C5", out(reg) mhint);
        core::arch::asm!("csrr {}, 0x7C0", out(reg) mxstatus);
    }
    let insn = unsafe { core::ptr::read_unaligned(mepc_addr as *const u32) };
    log::error!(
        "TRAP: Exception at {:#010X}\n  \
         MCAUSE   {}\n  \
         MTVAL    {:#010X}\n  \
         INSN     {:#010X}\n  \
         MSTATUS  {:#010X}\n  \
         MTVEC    {:#010X}\n  \
         MIP      {:#010X}\n  \
         MHCR     {:#010X}\n  \
         MHINT    {:#010X}\n  \
         MXSTATUS {:#010X}",
        mepc,
        mcause.code(),
        mtval,
        insn,
        mstatus.bits(),
        mtvec.bits(),
        mip.bits(),
        mhcr,
        mhint,
        mxstatus
    );
    loop {
        core::hint::spin_loop();
    }
}

/// Initialize the PLIC, clear dispatch entries, install `mtvec`, and enable M-mode interrupts.
///
/// # Safety
/// Must only be called once during early image startup on a D21x SoC.
pub unsafe fn init_vector_table() {
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    riscv::interrupt::disable();

    for slot in &HANDLERS {
        slot.store(0, Ordering::Release);
    }

    unsafe {
        plic_init();
    }
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    unsafe {
        riscv::register::mie::clear_mtimer();
        unsafe extern "C" {
            fn __plic_trap_vector();
        }
        let trap_addr = __plic_trap_vector as *const () as usize & !0x3;
        core::arch::asm!("csrw mtvec, {}", in(reg) trap_addr);
        riscv::interrupt::enable();
    }
}

/// Register handler `H` for interrupt `I` in [`HANDLERS`].
///
/// # Safety
/// Must be called on a D21x SoC with `I::IRQ < MAX_INTERRUPTS`.
pub unsafe fn register<I: super::InterruptSource, H: super::Handler<I>>() {
    let idx = I::IRQ as usize;
    assert!(idx < MAX_INTERRUPTS, "PLIC interrupt index out of range");
    HANDLERS[idx].store(
        irq_trampoline::<I, H> as *const () as usize,
        Ordering::Release,
    );
}

define_interrupt_mod! {
    CPU_TIMER = 7,
    SYSCFG = 16,
    CPM = 20,
    SDFM = 21,
    HCL = 22,
    CORDIC = 23,
    PWMCS_FAULT = 24,
    PWMCS_EPWM = 25,
    PWMCS_CAP = 26,
    PWMCS_QEP = 27,
    PSADC = 28,
    DMA = 32,
    CE = 33,
    USB_DEV = 34,
    USB_HOST0_EHCI = 35,
    USB_HOST0_OHCI = 36,
    USB_HOST1_EHCI = 37,
    USB_HOST1_OHCI = 38,
    GMAC0 = 39,
    GMAC1 = 40,
    SPI_ENC = 41,
    QSPI2 = 42,
    QSPI3 = 43,
    QSPI0 = 44,
    QSPI1 = 45,
    SDMC0 = 46,
    SDMC1 = 47,
    SDMC2 = 48,
    XSPI = 49,
    RTC = 50,
    MTOP = 51,
    I2S0 = 52,
    I2S1 = 53,
    AUDIO = 54,
    LCD = 55,
    DSI = 56,
    DVP = 57,
    DE = 59,
    GE = 60,
    VE = 61,
    WDOG = 64,
    GPIO_GRP_A = 68,
    GPIO_GRP_B = 69,
    GPIO_GRP_C = 70,
    GPIO_GRP_D = 71,
    GPIO_GRP_E = 72,
    GPIO_GRP_F = 73,
    UART0 = 76,
    UART1 = 77,
    UART2 = 78,
    UART3 = 79,
    UART4 = 80,
    UART5 = 81,
    UART6 = 82,
    UART7 = 83,
    I2C0 = 84,
    I2C1 = 85,
    I2C2 = 86,
    I2C3 = 87,
    CAN0 = 88,
    CAN1 = 89,
    PWM = 90,
    GPAI = 92,
    RTP = 93,
    THS = 94,
    CIR = 95,
}

#[cfg(test)]
mod tests {
    use core::sync::atomic::{AtomicUsize, Ordering};

    use super::{HANDLERS, dispatch_registered, register, typelevel};
    use crate::interrupt::Handler;

    static COUNT: AtomicUsize = AtomicUsize::new(0);

    struct TestHandler;

    impl Handler<typelevel::UART0> for TestHandler {
        unsafe fn on_interrupt() {
            COUNT.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn register_and_dispatch_invokes_handler() {
        COUNT.store(0, Ordering::SeqCst);
        unsafe { register::<typelevel::UART0, TestHandler>() };
        dispatch_registered(76);
        assert_eq!(COUNT.load(Ordering::SeqCst), 1);
        HANDLERS[76].store(0, Ordering::Release);
        dispatch_registered(76);
        assert_eq!(COUNT.load(Ordering::SeqCst), 1);
    }
}
