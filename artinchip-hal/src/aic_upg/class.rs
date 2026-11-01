//! AIC USB upgrade (`USBUPG`) `embassy-usb` device class.

use core::future::poll_fn;
use core::task::Poll;

use embassy_futures::select::{Either, select};
use embassy_usb::{Builder, Config, UsbDevice, UsbVersion};
use embassy_usb_driver::{EndpointAddress, EndpointIn as _, EndpointOut as _};
use log::error;

use super::protocol::{
    CBW_SIZE, Cbw, Csw, Engine, RESP_FAIL, RESP_OK, TRANSPORT_READ, TRANSPORT_WRITE,
};
use crate::cmu::Cmu;
use crate::interrupt::typelevel;
use crate::sys_cfg::SocSysCfg;
use crate::usbdrd::device::{
    EmbassyEndpointIn, EmbassyEndpointOut, UsbDev, UsbDevDriver, UsbDevError, UsbDevExt as _,
    UsbDevInterrupt,
};
use crate::usbdrd::{UsbConfig, UsbPads};

const VID: u16 = 0x33c3;
const PID: u16 = 0x6677;
const BULK_IN: u8 = 0x81;
const BULK_OUT: u8 = 0x02;
const BULK_MPS: u16 = 512;
const CHUNK: usize = 512;

/// Caller-owned descriptor and EP0 control buffers for [`AicUpgUsb`].
#[repr(C, align(64))]
pub struct AicUpgBuffers {
    config_desc: [u8; 512],
    bos_desc: [u8; 256],
    msos_desc: [u8; 256],
    control_buf: [u8; 256],
}

impl Default for AicUpgBuffers {
    fn default() -> Self {
        Self::new()
    }
}

impl AicUpgBuffers {
    /// Create zero-initialized descriptor and control buffers.
    pub const fn new() -> Self {
        Self {
            config_desc: [0; 512],
            bos_desc: [0; 256],
            msos_desc: [0; 256],
            control_buf: [0; 256],
        }
    }
}

/// Poll `future`, returning `Err(())` immediately if `expired(engine)` becomes true.
macro_rules! with_deadline {
    ($future:expr, $engine:expr, $expired:expr) => {{
        let mut future = core::pin::pin!($future);
        poll_fn(|cx| {
            if let Poll::Ready(result) = future.as_mut().poll(cx) {
                return Poll::Ready(Ok(result));
            }
            if $expired($engine) {
                return Poll::Ready(Err(()));
            }
            cx.waker().wake_by_ref();
            Poll::Pending
        })
        .await
    }};
}

/// ArtInChip USB upgrade (`USBUPG`) device built on `embassy-usb`.
pub struct AicUpgUsb<'d, PAD: UsbPads<0> + 'static> {
    usb: UsbDevice<'d, UsbDevDriver<'static, PAD>>,
    ep_in: EmbassyEndpointIn,
    ep_out: EmbassyEndpointOut,
}

impl<'d, PAD> AicUpgUsb<'d, PAD>
where
    PAD: UsbPads<0> + 'static,
{
    /// Initialize the USB controller and construct the vendor bulk `USBUPG` interface.
    pub fn new<IRQS>(
        usb_dev: UsbDev,
        pads: PAD,
        usb_config: UsbConfig,
        irqs: IRQS,
        buffers: &'d mut AicUpgBuffers,
        cmu: &mut Cmu,
        syscfg: &mut SocSysCfg,
    ) -> Result<Self, UsbDevError>
    where
        IRQS: typelevel::Binding<typelevel::USB_DEV, UsbDevInterrupt>,
    {
        let driver = usb_dev.new_embassy_driver(pads, usb_config, irqs, cmu, syscfg)?;

        let mut config = Config::new(VID, PID);
        config.device_class = 0xff;
        config.device_sub_class = 0xff;
        config.device_protocol = 0xff;
        config.composite_with_iads = false;
        config.bcd_usb = UsbVersion::Two;
        config.device_release = 0x0101;
        config.max_packet_size_0 = 64;
        config.manufacturer = Some("Artinchip");
        config.product = Some("Artinchip Device");
        config.max_power = 500;

        let mut builder = Builder::new(
            driver,
            config,
            &mut buffers.config_desc,
            &mut buffers.bos_desc,
            &mut buffers.msos_desc,
            &mut buffers.control_buf,
        );
        let (ep_in, ep_out) = {
            let mut function = builder.function(0xff, 0xff, 0xff);
            let mut interface = function.interface();
            let mut alt = interface.alt_setting(0xff, 0xff, 0xff, None);
            let ep_in = alt.endpoint_bulk_in(Some(EndpointAddress::from(BULK_IN)), BULK_MPS);
            let ep_out = alt.endpoint_bulk_out(Some(EndpointAddress::from(BULK_OUT)), BULK_MPS);
            (ep_in, ep_out)
        };
        let usb = builder.build();
        Ok(Self { usb, ep_in, ep_out })
    }

    /// Drop the D+ pull-up and disconnect from the USB bus.
    pub async fn disconnect(&mut self) {
        self.usb.disable().await;
    }

    /// Run enumeration and the `USBUPG` bulk transport loop until `expired` or host `reset`.
    pub async fn run(&mut self, engine: &mut Engine<'_>, expired: impl Fn(&Engine<'_>) -> bool) {
        let Self { usb, ep_in, ep_out } = self;
        let protocol = Self::protocol(ep_in, ep_out, engine, &expired);
        match select(usb.run(), protocol).await {
            Either::First(never) => match never {},
            Either::Second(()) => {}
        }
    }

    /// Tear down the USB stack, mute the controller, and release USB clocks/resets.
    pub fn deinit(self, cmu: &mut Cmu) {
        drop(self.usb);
        UsbDevDriver::teardown_embassy(cmu);
    }

    async fn protocol<F>(
        ep_in: &mut EmbassyEndpointIn,
        ep_out: &mut EmbassyEndpointOut,
        engine: &mut Engine<'_>,
        expired: &F,
    ) where
        F: Fn(&Engine<'_>) -> bool,
    {
        let mut cbw_buf = [0u8; CBW_SIZE];
        let mut data = [0u8; CHUNK];

        'session: loop {
            if expired(engine) {
                return;
            }
            let n = match with_deadline!(ep_out.read(&mut cbw_buf), engine, expired) {
                Ok(Ok(n)) => n,
                Ok(Err(_)) => {
                    engine.reset();
                    continue;
                }
                Err(()) => return,
            };
            let Some(cbw) = Cbw::parse(&cbw_buf[..n]) else {
                let failed = Csw::reply(0, RESP_FAIL, 0).to_bytes();
                match with_deadline!(ep_in.write(&failed), engine, expired) {
                    Ok(Ok(())) => {}
                    Ok(Err(_)) => engine.reset(),
                    Err(()) => return,
                }
                continue;
            };

            let tag = cbw.tag;
            let csw = match cbw.command {
                TRANSPORT_WRITE => {
                    let mut remaining = cbw.data_len as usize;
                    let mut status = RESP_OK;
                    if remaining == 0 {
                        status = engine.transport_write(&[]);
                    } else {
                        while remaining > 0 {
                            let want = remaining.min(CHUNK);
                            let n = match with_deadline!(
                                ep_out.read(&mut data[..want]),
                                engine,
                                expired
                            ) {
                                Ok(Ok(n)) => n,
                                Ok(Err(_)) => {
                                    engine.reset();
                                    continue 'session;
                                }
                                Err(()) => return,
                            };
                            if n == 0 {
                                break;
                            }
                            status = engine.transport_write(&data[..n]);
                            remaining -= n;
                        }
                    }
                    Csw::reply(tag, status, remaining as u32)
                }
                TRANSPORT_READ => {
                    let mut remaining = cbw.data_len as usize;
                    while remaining > 0 {
                        let want = remaining.min(CHUNK);
                        let n = engine.transport_read(&mut data[..want]);
                        if n == 0 {
                            break;
                        }
                        match with_deadline!(ep_in.write(&data[..n]), engine, expired) {
                            Ok(Ok(())) => {}
                            Ok(Err(_)) => {
                                engine.reset();
                                continue 'session;
                            }
                            Err(()) => return,
                        }
                        remaining -= n;
                    }
                    Csw::reply(tag, RESP_OK, remaining as u32)
                }
                command => {
                    error!("aic_upg: Unsupported transport command {command:#04x}");
                    Csw::reply(tag, RESP_FAIL, cbw.data_len)
                }
            };

            if expired(engine) {
                return;
            }

            let status = csw.to_bytes();
            match with_deadline!(ep_in.write(&status), engine, expired) {
                Ok(Ok(())) => {}
                Ok(Err(_)) => {
                    engine.reset();
                    continue;
                }
                Err(()) => return,
            }
            if engine.reset_requested() {
                return;
            }
        }
    }
}
