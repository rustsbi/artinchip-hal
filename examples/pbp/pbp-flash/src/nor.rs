//! SPI-NOR validation routine for `pbp-flash`.

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
#[cfg(any(feature = "quad", feature = "dma"))]
use artinchip_hal::spi_flash::{NOR_PAGE_SIZE, NOR_SECTOR_SIZE};
use artinchip_hal::spi_flash::{NorQuadConfig, SpiNorConfig, SpiNorFlash};
use artinchip_rt::prelude::*;
#[cfg(feature = "d13x")]
use artinchip_rt::soc::d13x::GpioBPads;
#[cfg(feature = "d21x")]
use artinchip_rt::soc::d21x::GpioBPads;
use log::{error, info};

pub fn run(
    qspi0: Qspi<0>,
    gpiob: GpioBPads,
    #[cfg(feature = "dma")] dma: Dma,
    delay: TimerDelay<'static>,
    clocks: &ClockTree,
    cmu: &mut Cmu,
) -> bool {
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
    let sck_hz = bus.sck_hz();
    #[cfg(feature = "dma")]
    let bus = DmaQspi::new(bus, dma.split(cmu).ch0);

    let config = if cfg!(feature = "quad") {
        SpiNorConfig::quad(NorQuadConfig::W25Q128JV)
    } else {
        SpiNorConfig::single()
    };
    let mut flash = match SpiNorFlash::with_config(bus, cs, delay, config) {
        Ok(flash) => flash,
        Err(err) => {
            error!("NOR: init failed: {err:?}");
            return false;
        }
    };

    info!("Partition headers:");
    for offset in [0x0000_0000u32, 0x0000_0100u32] {
        let mut magic = [0u8; 4];
        match flash.read(offset, &mut magic) {
            Ok(()) => info!(
                "  0x{offset:06X}: {:02X?} ({})",
                magic,
                crate::ascii(&magic)
            ),
            Err(err) => error!("  0x{offset:06X}: read failed ({err:?})"),
        }
    }

    let id = flash.id();
    let capacity = flash.device().capacity;
    info!(
        "JEDEC ID={:02X} {:02X} {:02X}, {} MiB, sck={} Hz",
        id.manufacturer,
        id.memory_type,
        id.capacity,
        capacity / (1024 * 1024),
        sck_hz
    );

    #[cfg(any(feature = "quad", feature = "dma"))]
    {
        let offset = (capacity - NOR_SECTOR_SIZE) as u32;
        let mut pattern = crate::DmaBuffer([0u8; NOR_PAGE_SIZE]);
        for (index, byte) in pattern.0.iter_mut().enumerate() {
            *byte = (index as u8) ^ 0x5A;
        }
        let mut readback = crate::DmaBuffer([0u8; NOR_PAGE_SIZE]);

        let write_ok = flash
            .erase_sector(offset)
            .and_then(|_| flash.write(offset, &pattern.0))
            .is_ok();
        let read_ok = flash.read(offset, &mut readback.0).is_ok();
        let passed = write_ok && read_ok && readback.0 == pattern.0;
        info!(
            "Write test ({} B @ 0x{offset:06X}): {}",
            NOR_PAGE_SIZE,
            crate::outcome(passed)
        );
        passed
    }

    #[cfg(not(any(feature = "quad", feature = "dma")))]
    true
}
