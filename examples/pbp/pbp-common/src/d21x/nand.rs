//! D21x SPI-NAND boot path: load the bootloader from the boot slot.

use artinchip_hal::cmu::{ClockTree, Cmu};
#[cfg(feature = "dma")]
use artinchip_hal::dma::{Dma, DmaExt as _};
use artinchip_hal::gtc::{CntFreq, Gtc};
use artinchip_hal::prelude::*;
#[cfg(feature = "dma")]
use artinchip_hal::qspi::DmaQspi;
#[cfg(feature = "quad")]
use artinchip_hal::qspi::quad_pads;
#[cfg(not(feature = "quad"))]
use artinchip_hal::qspi::single_pads_idle_io;
use artinchip_hal::qspi::{Qspi, QspiConfig};
#[cfg(feature = "quad")]
use artinchip_hal::spi_flash::NandQuadConfig;
use artinchip_hal::spi_flash::{SpiNandConfig, SpiNandFlash};
use artinchip_hal::system::{self, NandRead, Role};
use artinchip_rt::DRAM_BASE;
use artinchip_rt::{prelude::*, soc::d21x::GpioBPads};
use log::{debug, error};

use crate::layout::BOOTLOADER_FLASH_OFFSET;

/// Load the installed bootloader from the SPI-NAND boot slot and jump to it.
pub(super) fn boot_flash(
    qspi0: Qspi<0>,
    gpiob: GpioBPads,
    gtc: Gtc,
    #[cfg(feature = "dma")] dma: Dma,
    clocks: &ClockTree,
    cmu: &mut Cmu,
    boot_param: &BootParam,
    private_data: &[u8],
    from_host: bool,
    dram_bytes: u32,
) -> ! {
    let flash_cs = gpiob.pb2.into_pull_up_output();
    #[cfg(feature = "quad")]
    let pads = quad_pads::<0, _, _, _, _, _>(
        gpiob.pb5.into_qspi0_sck(),
        gpiob.pb4.into_qspi0_mosi(),
        gpiob.pb3.into_qspi0_miso(),
        gpiob.pb1.into_qspi0_wp(),
        gpiob.pb0.into_qspi0_hold(),
    );
    #[cfg(not(feature = "quad"))]
    let pads = single_pads_idle_io::<0, _, _, _, _, _>(
        gpiob.pb5.into_qspi0_sck(),
        gpiob.pb4.into_qspi0_mosi(),
        gpiob.pb3.into_qspi0_miso(),
        gpiob.pb1.into_pull_up_output(),
        gpiob.pb0.into_pull_up_output(),
    );

    let mut delay = gtc.new_timer_delay(CntFreq::Freq4M, cmu);
    let bus = qspi0.new_blocking(pads, QspiConfig::nand_flash(), clocks, cmu);
    #[cfg(feature = "dma")]
    let bus = DmaQspi::new(bus, dma.split(cmu).ch0);
    #[cfg(feature = "quad")]
    let config = SpiNandConfig::quad(NandQuadConfig::W25N01GV);
    #[cfg(not(feature = "quad"))]
    let config = SpiNandConfig::single();
    let nand = SpiNandFlash::with_config(bus, flash_cs, &mut delay, config)
        .unwrap_or_else(|_| system::halt());
    let mut flash = NandRead(nand);

    let loader = match system::resolve(&mut flash, BOOTLOADER_FLASH_OFFSET) {
        Ok(loader) => loader,
        Err(e) => {
            error!("Failed to parse bootloader container from flash: {e}");
            system::halt();
        }
    };
    if !loader.fits_window(DRAM_BASE as u32, dram_bytes) {
        error!(
            "Bootloader specifies memory range 0x{:08X}..0x{:08X} / entry 0x{:08X} outside DRAM window 0x{:08X}",
            loader.dst,
            loader.dst.saturating_add(loader.length),
            loader.entry,
            DRAM_BASE
        );
        system::halt();
    }

    if let Err(e) = system::load(&mut flash, &loader) {
        error!("Failed to copy bootloader payload: {e}");
        system::halt();
    }
    unsafe { dcache_clean_invalidate_range(loader.dst as usize, loader.length as usize) };

    if !private_data.is_empty() {
        unsafe {
            core::ptr::copy_nonoverlapping(
                private_data.as_ptr(),
                (loader.dst as *mut u8).add(loader.length as usize),
                private_data.len(),
            );
        }
    }

    // A host-driven start must serve the USB upgrade session, not boot flash.
    // Only a flash start publishes Boot.
    let role = if from_host { Role::Upgrade } else { Role::Boot };
    system::set_role(role);
    debug!("Jumping to bootloader @ {:#x}", loader.entry);
    unsafe {
        system::jump_to(
            loader.entry,
            boot_param.as_raw(),
            private_data.as_ptr(),
            private_data.len() as u32,
        )
    }
}
