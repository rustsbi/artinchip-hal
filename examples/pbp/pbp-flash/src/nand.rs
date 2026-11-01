//! SPI-NAND validation routine for `pbp-flash`.

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
use artinchip_hal::spi_flash::{NandQuadConfig, SpiNandConfig, SpiNandFlash};
use artinchip_rt::prelude::*;
#[cfg(feature = "d13x")]
use artinchip_rt::soc::d13x::GpioBPads;
#[cfg(feature = "d21x")]
use artinchip_rt::soc::d21x::GpioBPads;
use log::{error, info};

#[cfg(any(feature = "quad", feature = "dma"))]
const MAX_NAND_PAGE: usize = 4096;

pub fn run(
    qspi0: Qspi<0>,
    gpiob: GpioBPads,
    #[cfg(feature = "dma")] dma: Dma,
    delay: TimerDelay<'static>,
    clocks: &ClockTree,
    cmu: &mut Cmu,
) -> bool {
    let sck = gpiob.pb5.into_qspi0_sck();
    let mosi = gpiob.pb4.into_qspi0_mosi();
    let miso = gpiob.pb3.into_qspi0_miso();
    let cs = gpiob.pb2.into_pull_up_output();

    #[cfg(feature = "quad")]
    let pads = quad_pads::<0, _, _, _, _, _>(
        sck,
        mosi,
        miso,
        gpiob.pb1.into_qspi0_wp(),
        gpiob.pb0.into_qspi0_hold(),
    );
    #[cfg(not(feature = "quad"))]
    let pads = single_pads_idle_io::<0, _, _, _, _, _>(
        sck,
        mosi,
        miso,
        gpiob.pb1.into_pull_up_output(),
        gpiob.pb0.into_pull_up_output(),
    );

    let bus = qspi0.new_blocking(pads, QspiConfig::nand_flash(), clocks, cmu);
    let sck_hz = bus.sck_hz();
    #[cfg(feature = "dma")]
    let bus = DmaQspi::new(bus, dma.split(cmu).ch0);

    let config = if cfg!(feature = "quad") {
        SpiNandConfig::quad(NandQuadConfig::W25N01GV)
    } else {
        SpiNandConfig::single()
    };
    let mut flash = match SpiNandFlash::with_config(bus, cs, delay, config) {
        Ok(flash) => flash,
        Err(err) => {
            error!("NAND: init failed: {err:?}");
            return false;
        }
    };

    let geom = flash.geometry();
    info!("Partition headers:");
    for offset in [0usize, geom.page_size] {
        let mut magic = [0u8; 4];
        match flash.read(offset, &mut magic) {
            Ok(()) => info!(
                "  0x{offset:08X}: {:02X?} ({})",
                magic,
                crate::ascii(&magic)
            ),
            Err(err) => error!("  0x{offset:08X}: read failed ({err:?})"),
        }
    }

    let id = flash.info().id;
    info!(
        "JEDEC ID=0x{:02X}{:04X}, sck={} Hz",
        id.manufacturer, id.device, sck_hz
    );

    #[cfg(any(feature = "quad", feature = "dma"))]
    {
        let block = geom.block_count - 1;
        let page = block * geom.pages_per_block;
        let page_size = geom.page_size;
        let mut pattern = crate::DmaBuffer([0u8; MAX_NAND_PAGE]);
        for (index, byte) in pattern.0[..page_size].iter_mut().enumerate() {
            *byte = (index as u8) ^ 0xA5;
        }
        let mut readback = crate::DmaBuffer([0u8; MAX_NAND_PAGE]);

        let write_ok = flash
            .erase_block(block)
            .and_then(|_| flash.write_page(page, &pattern.0[..page_size]))
            .is_ok();
        let read_ok = flash.read_page(page, &mut readback.0[..page_size]).is_ok();
        let passed = write_ok && read_ok && readback.0[..page_size] == pattern.0[..page_size];
        info!(
            "Write test ({page_size} B @ block {block}): {}",
            crate::outcome(passed)
        );
        passed
    }

    #[cfg(not(any(feature = "quad", feature = "dma")))]
    true
}
