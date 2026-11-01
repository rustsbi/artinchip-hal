//! Pre-Boot Program execution and return context.

/// Pre-Boot Program header structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct PbpHeader {
    /// Magic number, should be ASCII "PBP ".
    pub magic: [u8; 4],
    /// PBP checksum filled by `aicfwc`.
    pub checksum: u32,
}

impl PbpHeader {
    pub const fn new() -> Self {
        Self {
            magic: *b"PBP ",
            checksum: 0,
        }
    }
}

impl Default for PbpHeader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "d21x")]
pub const STACK_SIZE: usize = 12 * 1024;
#[cfg(not(feature = "d21x"))]
pub const STACK_SIZE: usize = 16 * 1024;

/// 16-byte aligned PBP stack storage.
#[doc(hidden)]
#[repr(align(16))]
pub struct PbpStack(pub [u8; STACK_SIZE]);

impl PbpStack {
    pub const fn new() -> Self {
        Self([0; STACK_SIZE])
    }
}

impl Default for PbpStack {
    fn default() -> Self {
        Self::new()
    }
}

/// Caller registers and CSRs saved across a return-capable PBP invocation.
#[doc(hidden)]
#[repr(C, align(16))]
pub struct ReturnContext {
    pub ra: u32,
    pub sp: u32,
    pub gp: u32,
    pub tp: u32,
    pub s0: u32,
    pub s1: u32,
    pub s2: u32,
    pub s3: u32,
    pub s4: u32,
    pub s5: u32,
    pub s6: u32,
    pub s7: u32,
    pub s8: u32,
    pub s9: u32,
    pub s10: u32,
    pub s11: u32,
    pub a0: u32,
    pub a1: u32,
    pub a2: u32,
    pub mstatus: u32,
    pub mie: u32,
    pub mip: u32,
    pub mtvec: u32,
    pub mhcr: u32,
    pub mxstatus: u32,
}

impl ReturnContext {
    pub const fn new() -> Self {
        Self {
            ra: 0,
            sp: 0,
            gp: 0,
            tp: 0,
            s0: 0,
            s1: 0,
            s2: 0,
            s3: 0,
            s4: 0,
            s5: 0,
            s6: 0,
            s7: 0,
            s8: 0,
            s9: 0,
            s10: 0,
            s11: 0,
            a0: 0,
            a1: 0,
            a2: 0,
            mstatus: 0,
            mie: 0,
            mip: 0,
            mtvec: 0,
            mhcr: 0,
            mxstatus: 0,
        }
    }
}

impl Default for ReturnContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{align_of, size_of};

    #[test]
    fn pbp_header_magic() {
        let header = PbpHeader::new();
        assert_eq!(&header.magic, b"PBP ");
        assert_eq!(header.checksum, 0);
        assert_eq!(size_of::<PbpHeader>(), 8);
    }

    #[test]
    fn return_context_layout() {
        assert_eq!(align_of::<ReturnContext>(), 16);
        assert_eq!(size_of::<ReturnContext>(), 112);
    }
}
