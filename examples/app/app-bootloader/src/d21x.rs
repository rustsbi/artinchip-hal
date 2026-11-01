//! D21x QSPI bus and pin mapping for `app-bootloader`.
//!
//! The flash media is chosen by the `nor` / `nand` feature in `storage`; this
//! module only knows the D21x QSPI pins and DMA channel.

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
use artinchip_hal::usbdrd::device::UsbDev;
use artinchip_rt::gpio::{Function, Output};
use artinchip_rt::prelude::*;
use artinchip_rt::soc::d21x::GpioBPads;

use crate::{AppArguments, BootChip, HostPolicy};

#[cfg(feature = "quad")]
type D21xQspiPads = QuadPads<
    Function<'static, 'B', 5, 3>,
    Function<'static, 'B', 4, 3>,
    Function<'static, 'B', 3, 3>,
    Function<'static, 'B', 1, 3>,
    Function<'static, 'B', 0, 3>,
>;
#[cfg(not(feature = "quad"))]
type D21xQspiPads = SinglePadsIdleIo<
    Function<'static, 'B', 5, 3>,
    Function<'static, 'B', 4, 3>,
    Function<'static, 'B', 3, 3>,
    Output<'static, 'B', 1>,
    Output<'static, 'B', 0>,
>;

#[cfg(feature = "dma")]
type D21xQspiBus = DmaQspi<'static, 0, D21xQspiPads, 0>;
#[cfg(not(feature = "dma"))]
type D21xQspiBus = BlockingQspi<'static, 0, D21xQspiPads>;

pub struct Board;

impl BootChip for Board {
    type Bus = D21xQspiBus;
    type Cs = Output<'static, 'B', 2>;

    const HOST_POLICY: HostPolicy = HostPolicy::Unbounded;

    fn init_bus(
        qspi0: Qspi<0>,
        gpiob: GpioBPads,
        #[cfg(feature = "dma")] dma: Dma,
        clocks: &ClockTree,
        cmu: &mut Cmu,
        config: QspiConfig,
    ) -> (Self::Bus, Self::Cs) {
        // QSPI0 on D21x: SCK PB5, MOSI PB4, MISO PB3, CS PB2; /WP PB1 and /HOLD PB0.
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

        let bus = qspi0.new_blocking(pads, config, clocks, cmu);
        #[cfg(feature = "dma")]
        let bus = DmaQspi::new(bus, dma.split(cmu).ch0);
        (bus, cs)
    }

    fn before_usb(usb: &mut UsbDev, cmu: &mut Cmu) {
        // Reset any PHY/controller state inherited from the BootROM/PBP USB session.
        usb.release(cmu);
    }

    fn app_args<'a>(boot_param: BootParam, private: &'a [u8]) -> AppArguments<'a> {
        AppArguments {
            boot_param,
            private,
        }
    }
}
