//! Public interrupt handling API for ArtInChip SoCs.
//!
//! Controller selection is derived automatically from the active SoC feature:
//! - **CLIC** (`d12x`, `d13x`, `g73x`, `m6800`): hardware-vectored interrupts via `mtvt`.
//! - **PLIC** (`d21x`): machine external interrupt demultiplexed in trap dispatch.
//!
//! Async drivers program interrupts exclusively through [`typelevel`] and
//! [`InterruptSource`], remaining independent of the underlying controller.

#[cfg(not(feature = "d21x"))]
pub mod clic;

#[cfg(feature = "d21x")]
pub mod plic;

/// Type-level interrupt ZSTs and traits for the selected SoC.
#[cfg(not(feature = "d21x"))]
pub use clic::typelevel;
#[cfg(feature = "d21x")]
pub use plic::typelevel;

/// Controller-specific operations used by [`InterruptSource`].
#[cfg(not(feature = "d21x"))]
mod controller {
    use super::clic::{self, MAX_INTERRUPTS};

    pub(super) fn enable(irq: u8) {
        assert!((irq as usize) < MAX_INTERRUPTS);
        unsafe { clic::enable_interrupt(irq) }
    }

    pub(super) fn disable(irq: u8) {
        assert!((irq as usize) < MAX_INTERRUPTS);
        unsafe { clic::disable_interrupt(irq) }
    }

    pub(super) fn set_priority(irq: u8, priority: u8) {
        assert!((irq as usize) < MAX_INTERRUPTS);
        unsafe { clic::set_priority(irq, priority) }
    }

    pub(super) fn is_pending(irq: u8) -> bool {
        assert!((irq as usize) < MAX_INTERRUPTS);
        clic::is_pending(irq)
    }

    pub(super) fn clear_pending(irq: u8) {
        assert!((irq as usize) < MAX_INTERRUPTS);
        unsafe { clic::clear_pending(irq) }
    }
}

/// Controller-specific operations used by [`InterruptSource`].
#[cfg(feature = "d21x")]
mod controller {
    use super::plic::{self, MAX_INTERRUPTS};

    pub(super) fn enable(irq: u8) {
        assert!((irq as usize) < MAX_INTERRUPTS);
        unsafe { plic::enable_interrupt(irq) }
    }

    pub(super) fn disable(irq: u8) {
        assert!((irq as usize) < MAX_INTERRUPTS);
        unsafe { plic::disable_interrupt(irq) }
    }

    pub(super) fn set_priority(irq: u8, priority: u8) {
        assert!((irq as usize) < MAX_INTERRUPTS);
        unsafe { plic::set_priority(irq, priority) }
    }

    pub(super) fn is_pending(irq: u8) -> bool {
        assert!((irq as usize) < MAX_INTERRUPTS);
        plic::is_pending(irq)
    }

    pub(super) fn clear_pending(irq: u8) {
        assert!((irq as usize) < MAX_INTERRUPTS);
        unsafe { plic::clear_pending(irq) }
    }
}

/// Statically known interrupt source generated in [`typelevel`].
pub trait InterruptSource: Sized {
    /// Hardware interrupt number on this SoC.
    const IRQ: u8;

    /// Enable this interrupt source.
    #[inline(always)]
    fn enable() {
        controller::enable(Self::IRQ);
    }

    /// Disable this interrupt source.
    #[inline(always)]
    fn disable() {
        controller::disable(Self::IRQ);
    }

    /// Set this source's priority.
    #[inline(always)]
    fn set_priority(priority: u8) {
        controller::set_priority(Self::IRQ, priority);
    }

    /// Report whether this source has a pending request.
    #[inline(always)]
    fn is_pending() -> bool {
        controller::is_pending(Self::IRQ)
    }

    /// Clear this source's pending request.
    #[inline(always)]
    fn clear_pending() {
        controller::clear_pending(Self::IRQ);
    }
}

/// Trait implemented by driver interrupt handlers.
pub trait Handler<I: InterruptSource> {
    /// Invoked from interrupt context when `I` fires.
    ///
    /// # Safety
    /// Must only be called from the interrupt dispatch path for `I`.
    unsafe fn on_interrupt();
}

/// Compile-time proof and installer binding handler `H` to interrupt `I`.
///
/// # Safety
/// Implementors must guarantee `bind` installs `H` for `I` in the active controller.
pub unsafe trait Binding<I: InterruptSource, H: Handler<I>>: Copy {
    /// Install `H` into the active interrupt dispatch table for `I`.
    fn bind();
}

/// Initialize the active controller and trap/vector table during image startup.
///
/// # Safety
/// Must only be called once during early runtime startup before enabling caches.
#[inline]
pub unsafe fn init_vector_table() {
    #[cfg(not(feature = "d21x"))]
    unsafe {
        clic::init_vector_table();
    }

    #[cfg(feature = "d21x")]
    unsafe {
        plic::init_vector_table();
    }
}

/// Install handler `H` for interrupt `I` in the active controller's dispatch table.
///
/// Does not unmask `I`; callers enable the source once the peripheral is ready.
///
/// # Safety
/// Must be called after [`init_vector_table`] on the target SoC.
#[inline]
pub unsafe fn register<I: InterruptSource, H: Handler<I>>() {
    #[cfg(not(feature = "d21x"))]
    unsafe {
        clic::register::<I, H>();
    }

    #[cfg(feature = "d21x")]
    unsafe {
        plic::register::<I, H>();
    }
}

/// Internal helper macro generating a chip's `typelevel` interrupt module.
macro_rules! define_interrupt_mod {
    ($($irq_name:ident = $irq_num:expr),* $(,)?) => {
        pub mod typelevel {
            pub use $crate::interrupt::{Binding, Handler, InterruptSource as Interrupt};

            $(
                #[allow(non_camel_case_types)]
                pub enum $irq_name {}
                impl $crate::interrupt::InterruptSource for $irq_name {
                    const IRQ: u8 = $irq_num;
                }
            )*
        }
    };
}
pub(crate) use define_interrupt_mod;

/// Bind type-level interrupts to driver handlers.
///
/// Requires neither `#![feature(abi_riscv_interrupt)]` nor external crates (`paste`,
/// `riscv`) in downstream crates, and composes cleanly across modules.
#[macro_export]
macro_rules! bind_interrupts {
    ($vis:vis struct $name:ident { $($irq:ident => $handler:ty;)* }) => {
        #[derive(Copy, Clone)]
        $vis struct $name;

        impl $name {
            /// Explicitly install all handlers listed in this binding table.
            #[allow(dead_code)]
            pub fn register_all() {
                $(
                    <$name as $crate::interrupt::typelevel::Binding<
                        $crate::interrupt::typelevel::$irq,
                        $handler,
                    >>::bind();
                )*
            }
        }

        $(
            unsafe impl $crate::interrupt::typelevel::Binding<
                $crate::interrupt::typelevel::$irq,
                $handler,
            > for $name {
                #[inline]
                fn bind() {
                    unsafe {
                        $crate::interrupt::register::<
                            $crate::interrupt::typelevel::$irq,
                            $handler,
                        >();
                    }
                }
            }
        )*
    };
}
