//! SPI-NOR upgrade storage.

use core::convert::Infallible;

use artinchip_hal::aic_upg::{NorUpgStorage, SECTOR_SIZE};
use artinchip_hal::gtc::TimerDelay;
use artinchip_hal::qspi::QuadSpiBus;
use artinchip_hal::spi_flash::{SpiNorConfig, SpiNorFlash};
use artinchip_hal::system;
use artinchip_rt::core::boot_rom::{BootDevice, BootParam};
use artinchip_rt::sync::StaticSlot;
use embedded_hal::digital::OutputPin;
use log::{debug, error};

use crate::DmaBuf;
use crate::layout::FLASH_LAYOUT;

#[cfg(feature = "quad")]
use artinchip_hal::spi_flash::NorQuadConfig;

static SECTOR_BUF: StaticSlot<DmaBuf<SECTOR_SIZE>> = StaticSlot::new(DmaBuf([0; SECTOR_SIZE]));

/// Upgrade storage backed by a SPI-NOR flash.
pub type Storage<B, C> = NorUpgStorage<'static, SpiNorFlash<B, C, TimerDelay<'static>>>;

/// Bring up the NOR and wrap it as upgrade storage.
pub fn init<B, C>(
    bus: B,
    cs: C,
    delay: TimerDelay<'static>,
    boot_param: BootParam,
    scratch: &'static mut [u8],
) -> Storage<B, C>
where
    B: QuadSpiBus + 'static,
    B::Error: core::fmt::Debug,
    C: OutputPin<Error = Infallible> + 'static,
{
    #[cfg(feature = "quad")]
    let config = SpiNorConfig::quad(NorQuadConfig::W25Q128JV);
    #[cfg(not(feature = "quad"))]
    let config = SpiNorConfig::single();

    let mut nor = match SpiNorFlash::with_config(bus, cs, delay, config) {
        Ok(flash) => flash,
        Err(err) => {
            error!("app-bootloader: SPI-NOR init failed: {err:?}, halting");
            system::halt();
        }
    };

    let boot_device = boot_param.boot_device();
    if boot_device == BootDevice::Spinor {
        match nor.unique_id() {
            Ok(uid) => debug!("app-bootloader: Boot device {boot_device} (UID=0x{uid:016x})"),
            Err(err) => {
                error!("app-bootloader: Boot device {boot_device} (UID read failed: {err:?})")
            }
        }
    } else {
        debug!("app-bootloader: Boot device {boot_device}");
    }

    match nor.unprotect() {
        Ok((before, after)) if before != after => {
            debug!("app-bootloader: Cleared NOR block protection 0x{before:02X} -> 0x{after:02X}")
        }
        Ok(_) => {}
        Err(err) => error!("app-bootloader: NOR unprotect failed: {err:?}"),
    }

    let sector_buf = &mut SECTOR_BUF.take().0;
    NorUpgStorage::new(nor, sector_buf, scratch, FLASH_LAYOUT)
}
