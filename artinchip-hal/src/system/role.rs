//! Boot-role handshake between the first stage and the loader it starts.
//!
//! `_start` clobbers `a0` (via `_init_vector_table`) before `main`, so the role
//! cannot travel in registers. The first stage leaves it in a fixed RAM word
//! just below the loader; undefined memory after power-up reads as a normal boot.

/// Role word below the loader: D13x PSRAM, D21x DRAM.
#[cfg(feature = "d21x")]
const ROLE_ADDR: u32 = 0x40f0_0000;
#[cfg(not(feature = "d21x"))]
const ROLE_ADDR: u32 = 0x406b_f000;

/// `"BOOT"`.
const ROLE_BOOT: u32 = 0x424f_4f54;

/// `"UPGR"`.
const ROLE_UPGRADE: u32 = 0x5550_4752;

/// How the first stage started the loader.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    /// Flash start: no host is expected, so the loader may time out and boot.
    Boot,
    /// The container the host just `EXEC`ed: serve it without a deadline.
    Upgrade,
}

/// Publish `role` for the image about to be started.
///
/// The store is cleaned out of the D-cache by hand so the write survives the
/// hand-over, rather than depending on `jump_to`'s flush - a dropped line would
/// leave a *stale* role behind.
pub fn set_role(role: Role) {
    let value = match role {
        Role::Boot => ROLE_BOOT,
        Role::Upgrade => ROLE_UPGRADE,
    };
    unsafe {
        (ROLE_ADDR as *mut u32).write_volatile(value);
        crate::cache::dcache_clean_invalidate_range(ROLE_ADDR as usize, 4);
    }
}

/// Role the first stage published for this image.
///
/// Anything but `"UPGR"` - including undefined PSRAM - is [`Role::Boot`]. The
/// line is invalidated first: the word is written by a *different* image, and a
/// stale line is what once made cold starts wait for an absent host.
pub fn role() -> Role {
    unsafe {
        crate::cache::dcache_invalidate_range(ROLE_ADDR as usize, 4);
        match core::ptr::read_volatile(ROLE_ADDR as *const u32) {
            ROLE_UPGRADE => Role::Upgrade,
            _ => Role::Boot,
        }
    }
}
