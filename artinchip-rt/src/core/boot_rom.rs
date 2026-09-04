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

/// Boot parameter passed from BROM via a0 register.
///
/// Bit layout:
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
    /// Construct from raw `u32` value received via BROM's a0 register.
    #[inline]
    pub const fn from_raw(v: u32) -> Self {
        BootParam(v)
    }

    /// Get the underlying raw `u32` value.
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

/// Jump to BROM USB upgrade mode entry for E907 series.
///
/// # Safety
///
/// This function will disable caches and jump to BROM upgrade entry point unconditionally.
pub unsafe fn jump_to_e907_upg_entry() {
    // 1. Read BROM version magic number to select upgrade entry
    let brom_ver: u8 = unsafe { core::ptr::read_volatile(0x3000_0066 as *const u8) };
    let entry: u32 = match brom_ver {
        0x33 => 0x3000_7BE6,
        0x37 => 0x3000_7DD0,
        _ => {
            // Unknown BROM version, fall into dead loop
            loop {
                core::hint::spin_loop();
            }
        }
    };

    unsafe {
        // 2. dcache clean + invalidate all
        dcache_ciall();

        // 3. Disable D-Cache and I-Cache
        _disable_cache();

        // 4. Switch to BROM stack space and jump to upgrade entry
        core::arch::asm!(
            "li sp, 0x30044000",
            "jr a0",
            in("a0") entry,
            options(noreturn, nomem, nostack),
        );
    }
}

/// Jump to BROM USB upgrade mode entry for C906 series.
///
/// # Safety
///
/// This function will disable caches and jump to BROM upgrade entry point unconditionally.
pub unsafe fn jump_to_c906_upg_entry() {
    // 1. Read BROM version magic number to select upgrade entry
    let brom_ver = unsafe { core::ptr::read_volatile(0x66 as *const u8) };
    let entry: usize = match brom_ver {
        0x32 => 0x5c08,
        _ => {
            // Unknown BROM version, fall into dead loop
            loop {
                core::hint::spin_loop();
            }
        }
    };

    unsafe {
        // 2. dcache clean + invalidate all
        dcache_ciall();

        // 3. Disable D-Cache and I-Cache
        _disable_cache();

        // 4. Switch to BROM stack space and jump to upgrade entry
        core::arch::asm!(
            "li sp, 0x103000",
            "jr a0",
            in("a0") entry,
            options(noreturn, nomem, nostack),
        );
    }
}

/// Check if upgrade mode is requested by user.
///
/// If BOOT button (PA0, active-low) is held down on cold boot,
/// this function will jump to BROM upgrade entry point unconditionally.
pub fn check_upg_req(boot_param: &BootParam) {
    // PA group base: 0x18700000
    //   +0x00: input_state (GEN_IN_STA)
    //   +0x80: pin_config[0]  (PIN_CFG)
    const PA_INPUT_STA: u32 = 0x1870_0000;
    const PA0_PIN_CFG: u32 = 0x1870_0080;

    // Only check BOOT button on cold boot
    if boot_param.boot_reason() != BootReason::ColdBoot {
        return;
    }

    // Configure PA0: PIN_FUN=1 (GPIO), GEN_IE=1 (input enable), PIN_PULL=PullUp
    // PinConfig layout: bit16=GEN_IE, bit9:8=PIN_PULL(3=PullUp), bit3:0=PIN_FUN(1)
    unsafe {
        core::ptr::write_volatile(PA0_PIN_CFG as *mut u32, 0x0001_0301);
    }

    // Wait for button state to stabilize
    for _ in 0..100_000 {
        core::hint::spin_loop();
    }

    // Debounced read: PA0 is active-low (pressed = input_state bit0 == 0)
    let state_0 = unsafe {
        let in_sta = core::ptr::read_volatile(PA_INPUT_STA as *const u32);
        in_sta & 1 == 0
    };

    for _ in 0..100_000 {
        core::hint::spin_loop();
    }

    let state_1 = unsafe {
        let in_sta = core::ptr::read_volatile(PA_INPUT_STA as *const u32);
        in_sta & 1 == 0
    };

    if state_0 && state_1 {
        #[cfg(not(feature = "d21x"))]
        unsafe {
            jump_to_e907_upg_entry()
        };
        #[cfg(feature = "d21x")]
        unsafe {
            jump_to_c906_upg_entry()
        };
    }
}
