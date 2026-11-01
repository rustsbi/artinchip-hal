//! `embassy-usb-driver` [`Bus`] implementation.

use core::future::poll_fn;
use core::sync::atomic::Ordering;
use core::task::Poll;

use embassy_usb_driver::{Bus, EndpointAddress, EndpointType, Event, Unsupported};

use super::{BUS_POWER, BUS_RESET, BUS_RESUME, BUS_SUSPEND, SHARED};
use crate::usbdrd::UsbPads;
use crate::usbdrd::device::UsbEndpointConfig;
use crate::usbdrd::device::dev_register::EndpointType as DevEndpointType;
use crate::usbdrd::device::driver::EP_COUNT;

/// `embassy-usb` bus handle owning the configured USB pins.
pub struct EmbassyBus<PAD: UsbPads<0> + 'static> {
    _pads: PAD,
}

impl<PAD: UsbPads<0> + 'static> EmbassyBus<PAD> {
    pub(super) const fn new(pads: PAD) -> Self {
        Self { _pads: pads }
    }

    const fn to_device_ep_type(ep_type: EndpointType) -> DevEndpointType {
        match ep_type {
            EndpointType::Control => DevEndpointType::Control,
            EndpointType::Isochronous => DevEndpointType::Isochronous,
            EndpointType::Bulk => DevEndpointType::Bulk,
            EndpointType::Interrupt => DevEndpointType::Interrupt,
        }
    }
}

impl<PAD: UsbPads<0> + 'static> Bus for EmbassyBus<PAD> {
    async fn enable(&mut self) {
        SHARED.with_core(|core| core.set_connected(true));
    }

    async fn disable(&mut self) {
        SHARED.with_core(|core| core.set_connected(false));
        SHARED.bus_events.fetch_or(BUS_POWER, Ordering::Release);
        SHARED.bus_waker.wake();
    }

    async fn poll(&mut self) -> Event {
        poll_fn(|cx| {
            SHARED.bus_waker.register(cx.waker());
            SHARED.enumerated.swap(false, Ordering::AcqRel);
            let flags = SHARED.bus_events.load(Ordering::Acquire);
            let (bit, event) = if flags & BUS_POWER != 0 {
                (BUS_POWER, Event::PowerDetected)
            } else if flags & BUS_RESET != 0 {
                (BUS_RESET, Event::Reset)
            } else if flags & BUS_SUSPEND != 0 {
                (BUS_SUSPEND, Event::Suspend)
            } else if flags & BUS_RESUME != 0 {
                (BUS_RESUME, Event::Resume)
            } else {
                return Poll::Pending;
            };
            SHARED.bus_events.fetch_and(!bit, Ordering::AcqRel);
            Poll::Ready(event)
        })
        .await
    }

    fn endpoint_set_enabled(&mut self, ep_addr: EndpointAddress, enabled: bool) {
        let idx = ep_addr.index();
        if idx == 0 || idx >= EP_COUNT {
            return;
        }
        if enabled {
            let Some(info) = SHARED.ep_info(ep_addr.is_in(), idx) else {
                return;
            };
            let config = UsbEndpointConfig {
                address: u8::from(info.addr),
                max_packet_size: info.max_packet_size,
                ep_type: Self::to_device_ep_type(info.ep_type),
            };
            if !matches!(
                SHARED.with_core(|core| core.open_endpoint(config)),
                Some(Ok(()))
            ) {
                return;
            }
            if ep_addr.is_in() {
                SHARED.in_enabled[idx].store(true, Ordering::Release);
            } else {
                SHARED.out_enabled[idx].store(true, Ordering::Release);
            }
        } else {
            let _ = SHARED.with_core(|core| core.close_endpoint(u8::from(ep_addr)));
            if ep_addr.is_in() {
                SHARED.in_enabled[idx].store(false, Ordering::Release);
            } else {
                SHARED.out_enabled[idx].store(false, Ordering::Release);
            }
        }
        if ep_addr.is_in() {
            SHARED.in_waker[idx].wake();
        } else {
            SHARED.out_waker[idx].wake();
        }
        SHARED.bus_waker.wake();
    }

    fn endpoint_set_stalled(&mut self, ep_addr: EndpointAddress, stalled: bool) {
        let idx = ep_addr.index();
        if idx >= EP_COUNT {
            return;
        }
        let _ = SHARED.with_core(|core| core.stall(u8::from(ep_addr), stalled));
        if ep_addr.is_in() {
            SHARED.in_stalled[idx].store(stalled, Ordering::Release);
        } else {
            SHARED.out_stalled[idx].store(stalled, Ordering::Release);
        }
    }

    fn endpoint_is_stalled(&mut self, ep_addr: EndpointAddress) -> bool {
        let idx = ep_addr.index();
        if idx >= EP_COUNT {
            return false;
        }
        if ep_addr.is_in() {
            SHARED.in_stalled[idx].load(Ordering::Acquire)
        } else {
            SHARED.out_stalled[idx].load(Ordering::Acquire)
        }
    }

    async fn remote_wakeup(&mut self) -> Result<(), Unsupported> {
        Err(Unsupported)
    }
}
