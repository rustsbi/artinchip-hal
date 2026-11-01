//! D13x QSPI bus and pin mapping for `app-bootloader`.
//!
//! The flash media is chosen by the `nor` / `nand` feature in `storage`; this
//! module only knows the D13x QSPI pins and DMA channel.

use artinchip_hal::cmu::{ClockTree, Cmu};
#[cfg(feature = "dma")]
use artinchip_hal::dma::{Dma, DmaExt as _};
#[cfg(not(feature = "dma"))]
use artinchip_hal::qspi::BlockingQspi;
#[cfg(feature = "dma")]
use artinchip_hal::qspi::DmaQspi;
use artinchip_hal::qspi::{Qspi, QspiConfig, QspiExt as _};
#[cfg(feature = "quad")]
use artinchip_hal::qspi::{QuadPads, quad_pads};
#[cfg(not(feature = "quad"))]
use artinchip_hal::qspi::{SinglePadsIdleIo, single_pads_idle_io};
use artinchip_rt::gpio::{Function, Output};
use artinchip_rt::prelude::*;
use artinchip_rt::soc::d13x::GpioBPads;

use crate::{AppArguments, BootChip, HostPolicy};

/// Wait up to 30 s for AiBurn to initiate an upgrade before falling back to flash boot.
const UPGRADE_HOST_DELAY_MS: u32 = 30_000;

#[cfg(feature = "quad")]
type D13xQspiPads = QuadPads<
    Function<'static, 'B', 4, 2>,
    Function<'static, 'B', 5, 2>,
    Function<'static, 'B', 1, 2>,
    Function<'static, 'B', 0, 2>,
    Function<'static, 'B', 3, 2>,
>;
#[cfg(not(feature = "quad"))]
type D13xQspiPads = SinglePadsIdleIo<
    Function<'static, 'B', 4, 2>,
    Function<'static, 'B', 5, 2>,
    Function<'static, 'B', 1, 2>,
    Output<'static, 'B', 0>,
    Output<'static, 'B', 3>,
>;

#[cfg(feature = "dma")]
type D13xQspiBus = DmaQspi<'static, 0, D13xQspiPads, 1>;
#[cfg(not(feature = "dma"))]
type D13xQspiBus = BlockingQspi<'static, 0, D13xQspiPads>;

pub struct Board;

impl BootChip for Board {
    type Bus = D13xQspiBus;
    type Cs = Output<'static, 'B', 2>;

    const HOST_POLICY: HostPolicy = HostPolicy::Window {
        ms: UPGRADE_HOST_DELAY_MS,
    };

    fn init_bus(
        qspi0: Qspi<0>,
        gpiob: GpioBPads,
        #[cfg(feature = "dma")] dma: Dma,
        clocks: &ClockTree,
        cmu: &mut Cmu,
        config: QspiConfig,
    ) -> (Self::Bus, Self::Cs) {
        // QSPI0 on D13x: SCK PB4, MOSI PB5, MISO PB1, CS PB2; /WP PB0 and /HOLD PB3.
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

        let bus = qspi0.new_blocking(pads, config, clocks, cmu);
        #[cfg(feature = "dma")]
        let bus = DmaQspi::new(bus, dma.split(cmu).ch1);
        (bus, cs)
    }

    fn app_args(_boot_param: BootParam, _private: &[u8]) -> AppArguments<'_> {
        AppArguments {
            boot_param: BootParam::from_raw(0),
            private: &[],
        }
    }
}
