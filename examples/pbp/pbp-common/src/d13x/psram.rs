//! D13x PSRAM bring-up for the first-stage PBP.

use artinchip_hal::cmu::{ClockTree, Cmu};
use artinchip_hal::sys_cfg::{SysCfg, V1};
use artinchip_hal::xspi::{Chip, PsramModel, Xspi, XspiExt as _};
use embedded_hal::delay::DelayNs;
use log::{debug, error};

use crate::layout::UPG_BOOTLOADER_SLOT;

/// Size of the PSRAM host window verified during cold bring-up.
const UPG_BOOTLOADER_WINDOW: usize = 128 * 1024;

/// Maximum PSRAM training attempts before continuing.
const PSRAM_ATTEMPTS: u32 = 4;

/// Bring PSRAM up and verify the host window.
pub(super) fn bring_up(
    xspi: &Xspi,
    delay: &mut impl DelayNs,
    syscfg: &SysCfg<V1>,
    cmu: &Cmu,
    clocks: &ClockTree,
) {
    debug!("Initializing PSRAM...");
    let xspi_reg = xspi.register_block();
    let mut verified = false;
    for attempt in 0..PSRAM_ATTEMPTS {
        match Xspi::__new(xspi_reg).new_driver(
            Chip::D13x,
            PsramModel::Aps3208K,
            delay,
            syscfg,
            cmu,
            clocks,
        ) {
            Ok(mut driver) => {
                driver.cfg.xip_base = UPG_BOOTLOADER_SLOT as usize;
                delay.delay_ms(2);
                if driver.pattern_test(UPG_BOOTLOADER_WINDOW) {
                    verified = true;
                    break;
                }
                error!("Host window verify failed (attempt {attempt})");
            }
            Err(err) => error!("PSRAM init failed: {err:?}"),
        }
        delay.delay_ms(2);
    }
    if verified {
        debug!("PSRAM init OK🦀!");
    } else {
        error!("Host window never verified");
    }
}
