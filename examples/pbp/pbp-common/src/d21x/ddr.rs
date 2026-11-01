//! D21x DRAM bring-up for the first-stage PBP.

use artinchip_hal::cmu::{ClockTree, Cmu};
use artinchip_hal::dram::{self, Dram, DramConfig, DramExt as _};
use artinchip_hal::sid::Sid;
use artinchip_hal::sys_cfg::{SysCfg, V2};
use artinchip_hal::system;
use log::{debug, error};

/// Bring DRAM up and return the size in bytes described by the detected preset.
pub(super) fn bring_up(
    dram: Dram,
    cmu: &mut Cmu,
    syscfg: &mut SysCfg<V2>,
    sid: &mut Sid,
    clocks: &ClockTree,
) -> u32 {
    debug!("Initializing DRAM...");

    let detected = dram::detect(cmu, sid);
    let config = DramConfig::for_detected(&detected);
    if detected.dram_type.is_some() {
        debug!(
            "Using {} {} MiB info in efuse",
            config.dram_type.name(),
            config.memsize_mib()
        );
    } else {
        debug!(
            "No DRAM info in efuse, using {} {} MiB",
            config.dram_type.name(),
            config.memsize_mib()
        );
    }

    let Ok(mut driver) = dram.new_driver(cmu, syscfg, sid, &config, clocks) else {
        error!("DRAM init/status failed; halting before touching DRAM");
        system::halt();
    };
    if driver.probe().is_err() {
        error!("DRAM window probe failed; halting before touching DRAM");
        system::halt();
    }

    debug!("DRAM init OK🦀!");
    config.memsize
}
