//! SYSCFG peripheral instance and version markers.

use core::marker::PhantomData;

/// SYSCFG revision 1 marker (D12x / D13x).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum V1 {}

/// SYSCFG revision 2 marker (D21x).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum V2 {}

/// SYSCFG peripheral instance parameterized by hardware revision `V` ([`V1`] or [`V2`]).
pub struct SysCfg<V> {
    reg: *const (),
    _marker: PhantomData<V>,
}

impl SysCfg<V1> {
    /// Create a new SYSCFG v1 instance.
    #[inline]
    pub const fn __new(reg: *const super::register_v1::RegisterBlock) -> Self {
        Self {
            reg: reg as *const (),
            _marker: PhantomData,
        }
    }

    /// Get a reference to the SYSCFG v1 register block.
    #[inline]
    pub const fn register_block(&self) -> &'static super::register_v1::RegisterBlock {
        unsafe { &*(self.reg as *const super::register_v1::RegisterBlock) }
    }
}

impl SysCfg<V2> {
    /// Create a new SYSCFG v2 instance.
    #[inline]
    pub const fn __new(reg: *const super::register_v2::RegisterBlock) -> Self {
        Self {
            reg: reg as *const (),
            _marker: PhantomData,
        }
    }

    /// Get a reference to the SYSCFG v2 register block.
    #[inline]
    pub const fn register_block(&self) -> &'static super::register_v2::RegisterBlock {
        unsafe { &*(self.reg as *const super::register_v2::RegisterBlock) }
    }
}

/// SYSCFG handle selected for the active SoC revision.
#[cfg(feature = "d21x")]
pub type SocSysCfg = SysCfg<V2>;
#[cfg(not(feature = "d21x"))]
pub type SocSysCfg = SysCfg<V1>;
