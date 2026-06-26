//! Generic ring buffer.

use core::mem::MaybeUninit;

/// A generic, fixed-size ring buffer suitable for `no_std` environments.
/// Can store any type `T` without requiring `Copy` or `Default` bounds.
pub struct RingBuffer<T, const N: usize> {
    buf: [MaybeUninit<T>; N],
    head: usize,
    tail: usize,
    full: bool,
}

// Ensure the buffer can be safely sent across threads if T is Send
unsafe impl<T: Send, const N: usize> Send for RingBuffer<T, N> {}

impl<T, const N: usize> RingBuffer<T, N> {
    /// Create a new empty ring buffer.
    pub const fn new() -> Self {
        Self {
            // Safely initialize an array of uninitialized memory.
            // This is sound because the array elements are `MaybeUninit<T>`,
            // which explicitly do not require initialization.
            buf: unsafe { MaybeUninit::uninit().assume_init() },
            head: 0,
            tail: 0,
            full: false,
        }
    }

    /// Check if the buffer is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.head == self.tail && !self.full
    }

    /// Check if the buffer is full.
    #[inline]
    pub fn is_full(&self) -> bool {
        self.full
    }

    /// Push an item into the buffer.
    /// Returns `Err(item)` if the buffer is full, giving ownership back to the caller.
    #[inline]
    pub fn push(&mut self, item: T) -> Result<(), T> {
        if self.full {
            return Err(item); // Return the item to avoid dropping it unintentionally
        }

        // Write the item into the uninitialized memory slot
        self.buf[self.head].write(item);
        self.head = (self.head + 1) % N;

        if self.head == self.tail {
            self.full = true;
        }

        Ok(())
    }

    /// Pop an item from the buffer.
    /// Returns `None` if the buffer is empty.
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        // Extract the value and take ownership of the memory slot
        let item = unsafe { self.buf[self.tail].assume_init_read() };
        self.tail = (self.tail + 1) % N;
        self.full = false;

        Some(item)
    }
}

impl<T, const N: usize> Default for RingBuffer<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Drop implementation to ensure elements remaining in the buffer are correctly
/// destroyed when the RingBuffer goes out of scope.
impl<T, const N: usize> Drop for RingBuffer<T, N> {
    fn drop(&mut self) {
        // Pop and drop all remaining valid elements
        while self.pop().is_some() {}
    }
}
