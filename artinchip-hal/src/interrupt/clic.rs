//! CLIC interrupt controller backend (D12x / D13x / G73x / M6800).

use core::cell::UnsafeCell;
use core::sync::atomic::{Ordering, fence};

use super::define_interrupt_mod;

const CLIC_BASE: usize = 0x2080_0000;

/// Number of hardware vector-table entries (`mtvt + 4 * ID`), covering `0..=109`.
pub const MAX_INTERRUPTS: usize = 128;

#[repr(C, align(64))]
struct ClicVectorTable(UnsafeCell<[u32; MAX_INTERRUPTS]>);

unsafe impl Sync for ClicVectorTable {}

#[unsafe(link_section = ".clic.vector_table")]
static VECTOR_TABLE: ClicVectorTable = ClicVectorTable(UnsafeCell::new([0; MAX_INTERRUPTS]));

#[inline(always)]
fn clic() -> &'static crate::clic::RegisterBlock {
    unsafe { &*(CLIC_BASE as *const crate::clic::RegisterBlock) }
}

/// Initialize the CLIC controller in hardware-vectored mode.
///
/// # Safety
/// Must only be called when the CLIC peripheral is mapped on the current SoC.
pub unsafe fn clic_init() {
    critical_section::with(|_| {
        let clic = clic();
        unsafe {
            clic.cfg.modify(|v| v.enable_vector_mode().set_nlbits(8));
            clic.mint_thresh.modify(|v| v.set_mint_thresh(0));
        }
    });
}

/// Enable a specific CLIC interrupt source.
///
/// # Safety
/// `irq` must refer to a valid CLIC interrupt index (`< MAX_INTERRUPTS`).
pub unsafe fn enable_interrupt(irq: u8) {
    critical_section::with(|_| {
        let clic = clic();
        unsafe {
            clic.interrupts[irq as usize].int_ie.modify(|v| v.enable());
        }
    });
}

/// Disable a specific CLIC interrupt source.
///
/// # Safety
/// `irq` must refer to a valid CLIC interrupt index (`< MAX_INTERRUPTS`).
pub unsafe fn disable_interrupt(irq: u8) {
    critical_section::with(|_| {
        let clic = clic();
        unsafe {
            clic.interrupts[irq as usize].int_ie.modify(|v| v.disable());
        }
    });
}

/// Set the priority byte for a specific CLIC interrupt source.
///
/// # Safety
/// `irq` must refer to a valid CLIC interrupt index (`< MAX_INTERRUPTS`).
pub unsafe fn set_priority(irq: u8, priority: u8) {
    critical_section::with(|_| {
        let clic = clic();
        unsafe {
            clic.interrupts[irq as usize].int_ctl.write(priority);
        }
    });
}

/// Set hardware-vector, trigger, and privilege-mode attributes for `irq`.
///
/// # Safety
/// `irq` must refer to a valid CLIC interrupt index (`< MAX_INTERRUPTS`).
pub unsafe fn set_interrupt_attribute(irq: u8, shv: bool, trig: u8, mode: u8) {
    critical_section::with(|_| {
        let clic = clic();
        unsafe {
            clic.interrupts[irq as usize]
                .int_attr
                .modify(|v| v.set_hardware_vector(shv).set_trig(trig).set_mode(mode));
        }
    });
}

/// Clear the pending status of a specific CLIC interrupt source.
///
/// # Safety
/// `irq` must refer to a valid CLIC interrupt index (`< MAX_INTERRUPTS`).
pub unsafe fn clear_pending(irq: u8) {
    critical_section::with(|_| {
        let clic = clic();
        unsafe {
            clic.interrupts[irq as usize]
                .int_ip
                .modify(|v| v.clear_pending());
        }
    });
}

/// Check whether `irq` is currently pending.
pub fn is_pending(irq: u8) -> bool {
    clic().interrupts[irq as usize].int_ip.read().is_pending()
}

/// Mask every CLIC source and clear any pending requests inherited from earlier stages.
///
/// # Safety
/// The CLIC peripheral must be present and mapped on the current SoC.
pub unsafe fn mask_all_interrupts() {
    critical_section::with(|_| {
        let clic = clic();
        for interrupt in clic.interrupts.iter() {
            unsafe {
                interrupt.int_ie.modify(|v| v.disable());
                interrupt.int_ip.modify(|v| v.clear_pending());
            }
        }
    });
}

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
extern "riscv-interrupt-m" fn default_irq_handler() {
    log::error!("Unhandled interrupt in CLIC vector table");
    loop {
        core::hint::spin_loop();
    }
}

#[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
extern "C" fn default_irq_handler() {
    loop {
        core::hint::spin_loop();
    }
}

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
extern "riscv-interrupt-m" fn irq_trampoline<I: super::InterruptSource, H: super::Handler<I>>() {
    unsafe { H::on_interrupt() };
}

#[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
extern "C" fn irq_trampoline<I: super::InterruptSource, H: super::Handler<I>>() {
    unsafe { H::on_interrupt() };
}

/// Initialize CLIC hardware, install `mtvec`/`mtvt`, and populate default vectors.
///
/// # Safety
/// Must only be called once during early image startup before enabling caches.
pub unsafe fn init_vector_table() {
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    riscv::interrupt::disable();

    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    unsafe {
        unsafe extern "C" {
            fn AlignedTrapHandler();
        }
        let trap_addr = (AlignedTrapHandler as *const () as usize & !0x3) | 3;
        core::arch::asm!("csrw mtvec, {}", in(reg) trap_addr);
    }

    unsafe {
        clic_init();
        mask_all_interrupts();
    }

    let default_addr = default_irq_handler as *const () as usize as u32;
    unsafe {
        let table = &mut *VECTOR_TABLE.0.get();
        table.fill(default_addr);
    }

    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    unsafe {
        let vt_addr = VECTOR_TABLE.0.get() as usize;
        core::arch::asm!("csrw 0x307, {}", in(reg) vt_addr);
    }

    fence(Ordering::SeqCst);

    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    unsafe {
        riscv::interrupt::enable();
    }
}

/// Install handler `H` into `VECTOR_TABLE[I::IRQ]` and configure its CLIC attributes.
///
/// # Safety
/// Must be called after [`init_vector_table`] on a CLIC SoC.
pub unsafe fn register<I: super::InterruptSource, H: super::Handler<I>>() {
    let irq = I::IRQ;
    let idx = irq as usize;
    assert!(idx < MAX_INTERRUPTS, "CLIC interrupt index out of range");
    let entry = irq_trampoline::<I, H> as *const () as usize as u32;

    critical_section::with(|_| {
        let slot_addr = unsafe {
            let table = &mut *VECTOR_TABLE.0.get();
            table[idx] = entry;
            core::ptr::addr_of!(table[idx]) as usize
        };
        unsafe {
            crate::cache::dcache_clean_invalidate_range(slot_addr, core::mem::size_of::<u32>());
            set_priority(irq, 255);
            set_interrupt_attribute(irq, true, 0, 0);
        }
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        unsafe {
            core::arch::asm!("fence.i");
        }
    });
}

#[cfg(feature = "d12x")]
define_interrupt_mod! {
    CPU_TIMER = 7,
    DCE = 31,
    DMA = 32,
    SPI_ENC = 41,
    QSPI0 = 44,
    QSPI1 = 45,
    SDMC0 = 46,
    SDMC1 = 47,
    XSPI = 49,
    RTC = 50,
    MTOP = 51,
    AUDIO = 54,
    LCD = 55,
    DE = 59,
    GE = 60,
    VE = 61,
    WDOG = 64,
    GPIO_GRP_A = 68,
    GPIO_GRP_B = 69,
    GPIO_GRP_C = 70,
    GPIO_GRP_D = 71,
    GPIO_GRP_E = 72,
    UART0 = 76,
    UART1 = 77,
    UART2 = 78,
    UART3 = 79,
    I2C0 = 84,
    I2C1 = 85,
    CAN0 = 88,
    CAN1 = 89,
    PWM = 90,
    GPAI = 92,
    RTP = 93,
    THS = 94,
    CIR = 95,
}

#[cfg(any(
    feature = "d13x",
    not(any(feature = "d12x", feature = "g73x", feature = "m6800"))
))]
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
    PWMCS_QOUT = 29,
    DMA = 32,
    CE = 33,
    USB_DEV = 34,
    USB_HOST0_EHCI = 35,
    USB_HOST0_OHCI = 36,
    GMAC = 39,
    SPI_ENC = 41,
    QSPI2 = 42,
    QSPI3 = 43,
    QSPI0 = 44,
    QSPI1 = 45,
    SDMC0 = 46,
    SDMC1 = 47,
    XSPI = 49,
    RTC = 50,
    MTOP = 51,
    I2S = 52,
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
    CAN0 = 88,
    CAN1 = 89,
    PWM = 90,
    GPAI = 92,
    RTP = 93,
    THS = 94,
    CIR = 95,
    TA0_IF = 104,
    TA1_IF = 105,
    EDAT0_IF = 106,
    EDAT1_IF = 107,
    BIS0_IF = 108,
    BIS_IF = 109,
}

#[cfg(feature = "g73x")]
define_interrupt_mod! {
    CPU_TIMER = 7,
    SYSCFG = 16,
    CPM = 20,
    SDFM = 21,
    HCL = 22,
    CORDIC = 23,
    PWMCS_FAULT = 24,
    PWMCS_PWMCS = 25,
    PWMCS_CAP = 26,
    PWMCS_QEP = 27,
    PSADC = 28,
    PWMCS_QOUT = 29,
    DMA = 32,
    CE = 33,
    USB_DEV = 34,
    USB_HOST0_EHCI = 35,
    USB_HOST0_OHCI = 36,
    GMAC = 39,
    SPI_ENC = 41,
    QSPI2 = 42,
    QSPI3 = 43,
    QSPI0 = 44,
    QSPI1 = 45,
    SDMC0 = 46,
    SDMC1 = 47,
    XSPI = 49,
    RTC = 50,
    MTOP = 51,
    I2S = 52,
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
    CAN0 = 88,
    CAN1 = 89,
    PWM = 90,
    GPAI = 92,
    RTP = 93,
    THS = 94,
    CIR = 95,
    TA0_IF = 104,
    TA1_IF = 105,
    EDT0_IF = 106,
    EDT1_IF = 107,
    BISS0_IF = 108,
    BISS_IF = 109,
}

#[cfg(feature = "m6800")]
define_interrupt_mod! {
    CPU_TIMER = 7,
    SYSCFG = 16,
    CPM = 20,
    SDFM = 21,
    HCL = 22,
    CORDIC = 23,
    EPWM_FAULT = 24,
    EPWM = 25,
    CAP = 26,
    QEP = 27,
    ADC = 28,
    QOUT = 29,
    DMA = 32,
    CE = 33,
    USB_DEV = 34,
    EMAC = 39,
    SPI_ENC = 41,
    QSPI2 = 42,
    QSPI3 = 43,
    QSPI0 = 44,
    QSPI1 = 45,
    WDOG = 64,
    GPIO_GRP_A = 68,
    GPIO_GRP_B = 69,
    GPIO_GRP_C = 70,
    GPIO_GRP_D = 71,
    GPIO_GRP_E = 72,
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
    CAN0 = 88,
    CAN1 = 89,
    PWM = 90,
    THS = 94,
    TA0_IF = 104,
    TA1_IF = 105,
    EDAT0_IF = 106,
    EDAT1_IF = 107,
    BIS0_IF = 108,
    BIS_IF = 109,
}
