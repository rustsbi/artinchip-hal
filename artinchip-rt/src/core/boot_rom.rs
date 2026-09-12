//! ArtInChip Boot ROM API.

use super::cache::_disable_cache;
use log::info;
use xuantie_riscv::asm::dcache_ciall;

/// Boot reason (bits [11:8] of boot_param).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BootReason {
    ColdBoot,
    WarmBoot,
}

/// Boot device (bits [3:0] of boot_param).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BootDevice {
    None,
    Sdmc0,
    Sdmc1,
    Sdmc2,
    Spinor,
    Spinand,
    Sdfat32,
    Usb,
    Udisk,
}

/// Boot controller (bits [7:4] of boot_param).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BootController {
    None,
    Sdmc0,
    Sdmc1,
    Sdmc2,
    Spi0,
    Spi1,
    Usb,
}

/// Boot parameter passed from BROM via a0.
///
/// - bits [3:0]   → boot_device
/// - bits [7:4]   → boot_controller
/// - bits [11:8]  → boot_reason
/// - bits [15:12] → boot_image_id
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BootParam(u32);

impl BootParam {
    #[inline]
    pub fn boot_device(self) -> BootDevice {
        match self.0 & 0xF {
            0 => BootDevice::None,
            1 => BootDevice::Sdmc0,
            2 => BootDevice::Sdmc1,
            3 => BootDevice::Sdmc2,
            4 => BootDevice::Spinor,
            5 => BootDevice::Spinand,
            6 => BootDevice::Sdfat32,
            7 => BootDevice::Usb,
            8 => BootDevice::Udisk,
            _ => BootDevice::None,
        }
    }

    #[inline]
    pub fn boot_controller(self) -> BootController {
        match (self.0 >> 4) & 0xF {
            0 => BootController::None,
            1 => BootController::Sdmc0,
            2 => BootController::Sdmc1,
            3 => BootController::Sdmc2,
            4 => BootController::Spi0,
            5 => BootController::Spi1,
            6 => BootController::Usb,
            _ => BootController::None,
        }
    }

    #[inline]
    pub fn boot_reason(self) -> BootReason {
        match (self.0 >> 8) & 0xF {
            0 => BootReason::ColdBoot,
            _ => BootReason::WarmBoot,
        }
    }

    #[inline]
    pub fn boot_image_id(self) -> u8 {
        ((self.0 >> 12) & 0xF) as u8
    }
}

impl BootParam {
    #[inline]
    pub const fn from_raw(v: u32) -> Self {
        BootParam(v)
    }

    #[inline]
    pub const fn as_raw(self) -> u32 {
        self.0
    }
}

/// Print boot information.
pub fn print_boot_info(boot_param: &BootParam) {
    info!("Boot param: 0x{:08X}", boot_param.as_raw());
    info!("Boot device: {:?}", boot_param.boot_device());
    info!("Boot controller: {:?}", boot_param.boot_controller());
    info!("Boot reason: {:?}", boot_param.boot_reason());
    info!("Boot image ID: {}", boot_param.boot_image_id());
}

/// Check startup.
pub fn check_startup(boot_param: &BootParam) {
    check_upg_req(boot_param);
}

/// PA group input-state register (`GEN_IN_STA`).
const PA_INPUT_STATE: u32 = 0x1870_0000;

/// PA0 pin-configuration register (`PIN_CFG` of the PA group, pin 0).
const PA0_PIN_CONFIG: u32 = 0x1870_0080;

/// PA0 as GPIO input with pull-up, the value the vendor PBP's `upgmode_pin_cfg_val`
/// writes (`PIN_FUN = 1`, `PIN_DRV = 2`, `PIN_PULL = 3`, `GEN_IE = 1`).
const PA0_GPIO_INPUT_PULLUP: u32 = 0x0001_0321;

/// Consecutive low samples a held key must produce; a single read can catch a
/// bounce or an uncharged line.
const UPG_KEY_SAMPLES: u32 = 4;

/// Whether the ROM's USB/UART downloader started us: a host-driven start reports
/// `BD_USB` (its downloader stays alive), a flash start the device it booted from.
fn host_driven(boot_param: &BootParam) -> bool {
    boot_param.boot_device() == BootDevice::Usb
}

/// Whether the BOOT key is held at a moment where it should enter upgrade mode.
///
/// The vendor PBP's `upgmode` probe, byte for byte: repurpose PA0 as a GPIO input
/// with pull-up, wait the vendor's settle time, sample the key [`UPG_KEY_SAMPLES`]
/// times, restore the pin. It reports `false` for a host-driven start, where
/// acting would bounce the host's download into the ROM mid-burn.
pub fn upg_requested(boot_param: &BootParam) -> bool {
    if host_driven(boot_param) {
        return false;
    }

    // Safety: fixed GPIO PA group registers, already clocked by the ROM.
    let old = unsafe { core::ptr::read_volatile(PA0_PIN_CONFIG as *const u32) };
    unsafe { core::ptr::write_volatile(PA0_PIN_CONFIG as *mut u32, PA0_GPIO_INPUT_PULLUP) };

    // Pull-up settle time (vendor `upgmode_pin_pullup_dly` = 500 us). No timer is
    // up this early, so this is a plain spin, as in the ROM's own check.
    for _ in 0..100_000 {
        core::hint::spin_loop();
    }

    // Active-low: held is bit 0 clear.
    let held = (0..UPG_KEY_SAMPLES).all(|_| unsafe {
        // Safety: see the read above.
        core::ptr::read_volatile(PA_INPUT_STATE as *const u32) & 1 == 0
    });

    // Safety: always restore, so a negative probe leaves PA0 as its owner set it.
    unsafe { core::ptr::write_volatile(PA0_PIN_CONFIG as *mut u32, old) };

    held
}

/// Clean and disable the caches, then jump to `entry` on `stack`.
///
/// # Safety
///
/// `entry` must be a BootROM entry point that does not return, and `stack` RAM the
/// BootROM is willing to use.
unsafe fn jump_to_brom(entry: u32, stack: u32) -> ! {
    unsafe {
        // Clean + invalidate all, then disable both: the ROM does not know about
        // the dirty lines this image left behind.
        dcache_ciall();
        _disable_cache();
    }

    unsafe {
        // Explicit registers keep the addresses from being formatted as wider
        // register names, which warns on `asm!` operands.
        core::arch::asm!(
            "mv sp, a1",
            "jr a0",
            in("a0") entry,
            in("a1") stack,
            options(noreturn, nomem, nostack),
        )
    }
}

/// BootROM revision -> USB/UART upgrade entry point (E907). The revision byte the
/// vendor `boot_rom.c` switches on lives at `0x3000_0066`.
#[cfg(not(feature = "d21x"))]
fn e907_upg_entry() -> Option<u32> {
    match unsafe { core::ptr::read_volatile(0x3000_0066 as *const u8) } {
        0x33 => Some(0x3000_7BE6),
        0x37 => Some(0x3000_7DD0),
        _ => None,
    }
}

/// BootROM revision -> USB/UART upgrade entry point (C906).
#[cfg(feature = "d21x")]
fn c906_upg_entry() -> Option<u32> {
    match unsafe { core::ptr::read_volatile(0x66 as *const u8) } {
        0x32 => Some(0x5C08),
        _ => None,
    }
}

/// Hand the machine to the BootROM's USB/UART upgrade engine.
///
/// Returns `false` - without touching the machine - for an unknown BootROM
/// revision, so the caller can fall back to an ordinary boot instead of hanging.
/// On success this never returns.
///
/// # Safety
///
/// Only call when [`upg_requested`] reported a held key. Abandons the current
/// image's stack and leaves the caches disabled.
pub unsafe fn enter_upg_mode() -> bool {
    #[cfg(not(feature = "d21x"))]
    let entry = e907_upg_entry();
    #[cfg(feature = "d21x")]
    let entry = c906_upg_entry();

    let Some(entry) = entry else {
        return false;
    };

    // The ROM's upgrade engine runs on a stack near the top of the SRAM the ROM
    // reserves for itself, so this image's stack is abandoned.
    #[cfg(not(feature = "d21x"))]
    const UPG_ENTRY_STACK: u32 = 0x3004_4000;
    #[cfg(feature = "d21x")]
    const UPG_ENTRY_STACK: u32 = 0x0010_3000;

    unsafe { jump_to_brom(entry, UPG_ENTRY_STACK) }
}

/// Jump to BROM USB upgrade mode entry for E907. Prefer [`enter_upg_mode`], which
/// reports an unknown revision instead of spinning.
///
/// # Safety
///
/// Disables caches and jumps to the BROM upgrade entry unconditionally.
#[cfg(not(feature = "d21x"))]
pub unsafe fn jump_to_e907_upg_entry() {
    let Some(entry) = e907_upg_entry() else {
        loop {
            core::hint::spin_loop();
        }
    };

    unsafe { jump_to_brom(entry, 0x3004_4000) }
}

/// Jump to BROM USB upgrade mode entry for C906. Prefer [`enter_upg_mode`].
///
/// # Safety
///
/// Disables caches and jumps to the BROM upgrade entry unconditionally.
#[cfg(feature = "d21x")]
pub unsafe fn jump_to_c906_upg_entry() {
    let Some(entry) = c906_upg_entry() else {
        loop {
            core::hint::spin_loop();
        }
    };

    unsafe { jump_to_brom(entry, 0x0010_3000) }
}

/// Check if upgrade mode is requested by user: if the BOOT key (PA0, active-low)
/// is held - and this is not a host-driven `image.updater.*` call, where the key
/// is held by definition - hand the machine to the BROM and never return.
pub fn check_upg_req(boot_param: &BootParam) {
    if upg_requested(boot_param) {
        // Safety: a held key was just reported, and the BootROM owns the chip now.
        unsafe { enter_upg_mode() };
    }
}
