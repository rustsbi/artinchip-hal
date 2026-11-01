//! Single-hart static interior mutability primitivesprimitives.

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};

/// Transparent `Sync` wrapper around [`UnsafeCell<T>`] for single-hart bare-metal state.
///
/// Because it is `#[repr(transparent)]`, a `static SingleCore<T>` has the exact
/// same layout and symbol address as `T`, allowing startup assembly (`sym ...`)
/// and Rust runtime state to avoid `static mut` altogether.
#[repr(transparent)]
pub struct SingleCore<T>(UnsafeCell<T>);

unsafe impl<T> Sync for SingleCore<T> {}

impl<T> SingleCore<T> {
    /// Wrap `value` in a `SingleCore` cell.
    pub const fn new(value: T) -> Self {
        Self(UnsafeCell::new(value))
    }

    /// Return the raw mutable pointer to the inner value.
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut T {
        self.0.get()
    }
}

/// One-shot `'static` storage cell guarded by an [`AtomicBool`].
pub struct StaticSlot<T> {
    taken: AtomicBool,
    cell: SingleCore<T>,
}

impl<T> StaticSlot<T> {
    /// Create an unclaimed `StaticSlot` initialized with `value`.
    pub const fn new(value: T) -> Self {
        Self {
            taken: AtomicBool::new(false),
            cell: SingleCore::new(value),
        }
    }

    /// Claim the exclusive `'static` mutable reference once.
    ///
    /// # Panics
    /// Panics if `take` has already been called on this slot.
    #[allow(clippy::mut_from_ref)]
    pub fn take(&'static self) -> &'static mut T {
        assert!(
            !self.taken.swap(true, Ordering::AcqRel),
            "StaticSlot::take called more than once"
        );
        unsafe { &mut *self.cell.as_ptr() }
    }
}
