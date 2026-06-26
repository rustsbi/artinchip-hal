//! CLIC interrupt management.
#![allow(unsafe_op_in_unsafe_fn)]

const CLIC_BASE: u32 = 0x20800000;

#[inline(always)]
fn clic() -> &'static crate::clic::RegisterBlock {
    unsafe { &*(CLIC_BASE as *const crate::clic::RegisterBlock) }
}

/// Initialize the CLIC controller.
///
/// # Safety
/// Must only be called when the CLIC peripheral is present and the
/// memory-mapped register block is valid for the current SoC.
pub unsafe fn clic_init() {
    critical_section::with(|_| {
        let clic = clic();
        clic.cfg.modify(|v| v.enable_vector_mode().set_nlbits(8));
        clic.mint_thresh.modify(|v| v.set_mint_thresh(0));
    });
}

/// Enable a specific interrupt.
///
/// # Safety
/// The interrupt number must refer to a valid CLIC interrupt on the current SoC.
pub unsafe fn enable_interrupt(irq: u8) {
    critical_section::with(|_| {
        let clic = clic();
        clic.interrupts[irq as usize].int_ie.modify(|v| v.enable());
    });
}

/// Disable a specific interrupt.
///
/// # Safety
/// The interrupt number must refer to a valid CLIC interrupt on the current SoC.
pub unsafe fn disable_interrupt(irq: u8) {
    critical_section::with(|_| {
        let clic = clic();
        clic.interrupts[irq as usize].int_ie.modify(|v| v.disable());
    });
}

/// Set the level/priority for a specific interrupt.
///
/// # Safety
/// The interrupt number must refer to a valid CLIC interrupt on the current SoC.
pub unsafe fn set_priority(irq: u8, priority: u8) {
    critical_section::with(|_| {
        let clic = clic();
        clic.interrupts[irq as usize].int_ctl.write(priority);
    });
}

/// Set the attributes of a specific interrupt.
///
/// # Safety
/// The interrupt number must refer to a valid CLIC interrupt on the current SoC.
pub unsafe fn set_interrupt_attribute(irq: u8, shv: bool, trig: u8, mode: u8) {
    critical_section::with(|_| {
        let clic = clic();
        clic.interrupts[irq as usize]
            .int_attr
            .modify(|v| v.set_hardware_vector(shv).set_trig(trig).set_mode(mode));
    });
}

/// Clear the pending status of a specific interrupt.
///
/// # Safety
/// The interrupt number must refer to a valid CLIC interrupt on the current SoC.
pub unsafe fn clear_pending(irq: u8) {
    critical_section::with(|_| {
        let clic = clic();
        clic.interrupts[irq as usize]
            .int_ip
            .modify(|v| v.clear_pending());
    });
}

/// Check if a specific interrupt is currently pending.
pub fn is_pending(irq: u8) -> bool {
    let clic = clic();
    clic.interrupts[irq as usize].int_ip.read().is_pending()
}

/// Generate type-level interrupt module
#[macro_export]
macro_rules! clic_interrupt_mod {
    ($($irq_name:ident = $irq_num:expr),* $(,)?) => {
        pub mod typelevel {
            /// Common Trait for type-level interrupts
            pub trait Interrupt {
                const IRQ: u8;

                #[inline(always)]
                fn enable() { unsafe { $crate::interrupt::clic::enable_interrupt(Self::IRQ) } }

                #[inline(always)]
                fn disable() { unsafe { $crate::interrupt::clic::disable_interrupt(Self::IRQ) } }

                #[inline(always)]
                fn set_priority(prio: u8) { unsafe { $crate::interrupt::clic::set_priority(Self::IRQ, prio) } }

                #[inline(always)]
                fn is_pending() -> bool { $crate::interrupt::clic::is_pending(Self::IRQ) }

                #[inline(always)]
                fn clear_pending() { unsafe { $crate::interrupt::clic::clear_pending(Self::IRQ) } }
            }

            /// Drivers implement this Trait to handle interrupt logic
            pub trait Handler<I: Interrupt> {
                /// Function to be called when the interrupt fires.
                /// # Safety
                /// Must ONLY be called from the interrupt handler context.
                unsafe fn on_interrupt();
            }

            /// Compile-time constraint: Proves the user has bound a Handler to an Interrupt.
            ///
            /// # Safety
            /// Implementors must ensure the binding is valid for the selected interrupt.
            pub unsafe trait Binding<I: Interrupt, H: Handler<I>> {}

            // Generate specific ZST (Zero-Sized Type) for each hardware interrupt
            $(
                #[allow(non_camel_case_types)]
                pub enum $irq_name {}
                impl Interrupt for $irq_name {
                    const IRQ: u8 = $irq_num;
                }
            )*
        }
    };
}

#[macro_export]
macro_rules! clic_bind_interrupts {
    ($vis:vis struct $name:ident { $($irq:ident => $($handler:ty),*;)* }) => {
        #[derive(Copy, Clone)]
        $vis struct $name;

        // #[no_mangle] and extern "C" ensure the linker can resolve it across crates.
        #[unsafe(no_mangle)]
        pub extern "C" fn __artinchip_dispatch_interrupt(irq_id: usize) {
            match irq_id as u8 {
                $(
                    <$crate::interrupt::clic::typelevel::$irq as $crate::interrupt::clic::typelevel::Interrupt>::IRQ => {
                        unsafe {
                            $(
                                <$handler as $crate::interrupt::clic::typelevel::Handler<
                                    $crate::interrupt::clic::typelevel::$irq
                                >>::on_interrupt();
                            )*
                        }
                    }
                )*
                _ => {
                    // Unregistered ghost interrupt, just ignore it. This is a safety measure to prevent unexpected behavior.
                }
            }
        }

        impl $name {
            // One-click initialization for CLIC hardware and bound interrupts
            pub unsafe fn init_clic_and_interrupts() {
                // Import the trampoline address exposed by the RT crate
                unsafe extern "C" { fn AlignedTrapHandler(); }
                let trap_addr = AlignedTrapHandler as *const () as usize;

                // Configure mtvec (Mode 3: CLIC)
                ::core::arch::asm!("csrw mtvec, {}", in(reg) trap_addr | 3);

                // Initialize the CLIC controller
                $crate::interrupt::clic::clic_init();

                // Automatically configure and enable each interrupt bound in the macro
                $(
                    let irq_num = <$crate::interrupt::clic::typelevel::$irq as $crate::interrupt::clic::typelevel::Interrupt>::IRQ;
                    // Default configuration: Max priority(255), Software dispatch(shv=false), Level-triggered(0)
                    $crate::interrupt::clic::set_priority(irq_num, 255);
                    $crate::interrupt::clic::set_interrupt_attribute(irq_num, false, 0, 3);
                    $crate::interrupt::clic::enable_interrupt(irq_num);
                )*

                // Finally, enable global CPU interrupts
                ::riscv::interrupt::enable();
            }
        }

        // Apply Binding Trait constraints for each handler
        $(
            $(
                unsafe impl $crate::interrupt::clic::typelevel::Binding<
                    $crate::interrupt::clic::typelevel::$irq,
                    $handler
                > for $name {}
            )* )*
    };
}

#[cfg(feature = "d12x")]
clic_interrupt_mod! {
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

#[cfg(feature = "d13x")]
clic_interrupt_mod! {
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
clic_interrupt_mod! {
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
clic_interrupt_mod! {
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
