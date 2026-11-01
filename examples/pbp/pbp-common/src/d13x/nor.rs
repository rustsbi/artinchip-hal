//! D13x SPI-NOR boot path: resolve and start the installed bootloader.

use artinchip_hal::cmu::{ClockTree, Cmu};
#[cfg(feature = "dma")]
use artinchip_hal::dma::{Dma, DmaExt as _};
use artinchip_hal::gtc::TimerDelay;
#[cfg(feature = "dma")]
use artinchip_hal::qspi::DmaQspi;
#[cfg(feature = "quad")]
use artinchip_hal::qspi::quad_pads;
#[cfg(not(feature = "quad"))]
use artinchip_hal::qspi::single_pads_idle_io;
use artinchip_hal::qspi::{Qspi, QspiConfig, QspiExt as _};
#[cfg(feature = "quad")]
use artinchip_hal::spi_flash::NorQuadConfig;
use artinchip_hal::spi_flash::{SpiNorConfig, SpiNorFlash};
use artinchip_hal::system::{self, Role};
use artinchip_rt::prelude::*;
use artinchip_rt::soc::d13x::GpioBPads;
use log::{debug, error};

use crate::layout::BOOTLOADER_FLASH_OFFSET;

/// Resolve the bootloader from the SPI-NOR boot slot and jump to it.
pub(super) fn boot(
    qspi0: Qspi<0>,
    gpiob: GpioBPads,
    delay: TimerDelay<'static>,
    #[cfg(feature = "dma")] dma: Dma,
    clocks: &ClockTree,
    cmu: &mut Cmu,
    boot_param: &BootParam,
    private_data: &[u8],
) -> ! {
    let sck = gpiob.pb4.into_qspi0_sck();
    let mosi = gpiob.pb5.into_qspi0_mosi();
    let miso = gpiob.pb1.into_qspi0_miso();
    let cs = gpiob.pb2.into_pull_up_output();

    #[cfg(feature = "quad")]
    let pads = quad_pads::<0, _, _, _, _, _>(
        sck,
        mosi,
        miso,
        gpiob.pb0.into_qspi0_wp(),
        gpiob.pb3.into_qspi0_hold(),
    );
    #[cfg(not(feature = "quad"))]
    let pads = single_pads_idle_io::<0, _, _, _, _, _>(
        sck,
        mosi,
        miso,
        gpiob.pb0.into_pull_up_output(),
        gpiob.pb3.into_pull_up_output(),
    );

    let bus = qspi0.new_blocking(pads, QspiConfig::nor_flash(), clocks, cmu);
    #[cfg(feature = "dma")]
    let bus = // ch1: USB owns GPDMA ch0 during an upgrade session.
    DmaQspi::new(bus, dma.split(cmu).ch1);
    #[cfg(feature = "quad")]
    let config = SpiNorConfig::quad(NorQuadConfig::W25Q128JV);
    #[cfg(not(feature = "quad"))]
    let config = SpiNorConfig::single();
    let mut nor = match SpiNorFlash::with_config(bus, cs, delay, config) {
        Ok(nor) => nor,
        Err(err) => {
            error!("SPI-NOR init failed: {err:?}");
            system::halt();
        }
    };
    let loader = match system::resolve(&mut nor, BOOTLOADER_FLASH_OFFSET) {
        Ok(loader) => loader,
        Err(_) => {
            error!("Bootloader not found");
            system::halt();
        }
    };
    if system::load(&mut nor, &loader).is_err() {
        error!("Bootloader copy failed");
        system::halt();
    }
    unsafe { dcache_clean_invalidate_range(loader.dst as usize, loader.length as usize) };

    system::set_role(Role::Boot);
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
