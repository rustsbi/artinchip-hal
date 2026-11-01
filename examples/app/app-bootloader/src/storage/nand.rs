//! SPI-NAND upgrade storage.

use core::convert::Infallible;

use artinchip_hal::aic_upg::NandUpgStorage;
use artinchip_hal::gtc::TimerDelay;
use artinchip_hal::qspi::QuadSpiBus;
#[cfg(feature = "quad")]
use artinchip_hal::spi_flash::NandQuadConfig;
use artinchip_hal::spi_flash::{SpiNandConfig, SpiNandFlash};
use artinchip_hal::system;
use artinchip_rt::core::boot_rom::{BootDevice, BootParam};
use embedded_hal::digital::OutputPin;
use log::{debug, error};

use crate::layout::FLASH_LAYOUT;

/// Upgrade storage backed by a SPI-NAND flash.
pub type Storage<B, C> = NandUpgStorage<'static, SpiNandFlash<B, C, TimerDelay<'static>>>;

/// Bring up the NAND and wrap it as upgrade storage.
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
    let config = SpiNandConfig::quad(NandQuadConfig::W25N01GV);
    #[cfg(not(feature = "quad"))]
    let config = SpiNandConfig::single();

    let flash = match SpiNandFlash::with_config(bus, cs, delay, config) {
        Ok(flash) => flash,
        Err(err) => {
            error!("app-bootloader: SPI-NAND bring-up failed: {err:?}, halting");
            system::halt();
        }
    };
    let init = flash.info();

    let boot_device = boot_param.boot_device();
    if boot_device == BootDevice::Spinand {
        debug!(
            "app-bootloader: Boot device {boot_device} (ID=0x{:02X} {:04X})",
            init.id.manufacturer, init.id.device
        );
    } else {
        debug!("app-bootloader: Boot device {boot_device}");
    }

    NandUpgStorage::new(flash, scratch, FLASH_LAYOUT)
}
