//! CLINT instances for the supported RISC-V cores.
//!
//! The standard CLINT exposes `mtime` through its memory-mapped register block.
//! T-Head C906 cores instead read the counter from the `time` CSR, while
//! `mtimecmp` remains in the CLINT register block.

use super::register::RegisterBlock;
use core::marker::PhantomData;
use volatile_register::RW;

/// E907 CLINT instance with a memory-mapped machine timer.
pub struct ClintV1 {
    reg: *const RegisterBlock,
    _private: PhantomData<()>,
}

impl ClintV1 {
    /// Create a new E907 CLINT instance.
    pub const fn __new(reg: *const RegisterBlock) -> Self {
        Self {
            reg,
            _private: PhantomData,
        }
    }

    /// Get a reference to the register block.
    pub const fn register_block(&self) -> &'static RegisterBlock {
        unsafe { &*self.reg }
    }

    /// Machine timer counter, low word.
    const MTIME_LO: usize = 0xBFF8;
    /// Machine timer counter, high word.
    const MTIME_HI: usize = 0xBFFC;

    #[inline]
    fn word(&self, offset: usize) -> &'static RW<u32> {
        unsafe { &*((self.reg as usize + offset) as *const RW<u32>) }
    }

    /// Read the machine timer counter.
    #[inline]
    pub fn mtime(&self) -> u64 {
        loop {
            let high = self.word(Self::MTIME_HI).read();
            let low = self.word(Self::MTIME_LO).read();
            if high == self.word(Self::MTIME_HI).read() {
                return ((high as u64) << 32) | low as u64;
            }
        }
    }

    /// Read the machine timer compare for hart 0.
    #[inline]
    pub fn mtimecmp(&self) -> u64 {
        self.register_block().read_mtimecmp(0)
    }

    /// Write the machine timer compare for hart 0.
    ///
    /// Interrupts are masked while the upstream helper writes the low and high
    /// words; this prevents a transient low-word value from raising MTIP.
    #[inline]
    pub fn set_mtimecmp(&self, at: u64) {
        riscv::interrupt::free(|| self.register_block().write_mtimecmp(0, at));
    }
}

/// C906 CLINT instance.
///
/// The counter is read through the `time` CSR; `mtimecmp` is still part of the
/// CLINT register block.
pub struct ClintV2 {
    reg: *const RegisterBlock,
    _private: PhantomData<()>,
}

impl ClintV2 {
    /// Create a new C906 CLINT instance.
    pub const fn __new(reg: *const RegisterBlock) -> Self {
        Self {
            reg,
            _private: PhantomData,
        }
    }

    /// Get a reference to the register block.
    pub const fn register_block(&self) -> &'static RegisterBlock {
        unsafe { &*self.reg }
    }

    /// Read the machine timer counter.
    #[inline]
    pub fn mtime(&self) -> u64 {
        riscv::register::time::read64()
    }

    /// Read the machine timer compare for hart 0.
    #[inline]
    pub fn mtimecmp(&self) -> u64 {
        self.register_block().read_mtimecmp(0)
    }

    /// Write the machine timer compare for hart 0.
    ///
    /// Interrupts are masked while the upstream helper writes the low and high
    /// words; this prevents a transient low-word value from raising MTIP.
    #[inline]
    pub fn set_mtimecmp(&self, at: u64) {
        riscv::interrupt::free(|| self.register_block().write_mtimecmp(0, at));
    }
}
