//! `embassy-usb-driver` `EP0` [`ControlPipe`] implementation.

use core::future::poll_fn;
use core::sync::atomic::Ordering;
use core::task::Poll;

use embassy_usb_driver::{ControlPipe, EndpointError};

use super::{EP_DISABLED, EP_DONE, EP_PENDING, SHARED};
use crate::usbdrd::device::UsbEndpointConfig;
use crate::usbdrd::device::dev_register::EndpointType as DevEndpointType;

/// `EP0` control pipe handle for [`UsbDevDriver`](crate::usbdrd::device::UsbDevDriver).
pub struct EmbassyControlPipe {
    _private: (),
}

impl EmbassyControlPipe {
    pub(super) const fn new() -> Self {
        Self { _private: () }
    }

    fn arm_setup() {
        SHARED.with_core_and_bufs(|core, bufs| {
            if SHARED.setup_armed.load(Ordering::Acquire) {
                return;
            }
            let mps = SHARED.control_mps.load(Ordering::Relaxed) as u16;
            if !SHARED.ep0_configured.load(Ordering::Acquire) {
                let config = |address: u8| UsbEndpointConfig {
                    address,
                    max_packet_size: mps,
                    ep_type: DevEndpointType::Control,
                };
                let _ = core.open_endpoint(config(0x00));
                let _ = core.open_endpoint(config(0x80));
                SHARED.ep0_configured.store(true, Ordering::Release);
            }
            let _ = core.stall(0x00, false);
            let _ = core.stall(0x80, false);
            SHARED.ctrl_out_armed.store(false, Ordering::Release);
            if core.start_setup_read(&mut bufs.setup.0).is_ok() {
                SHARED.setup_armed.store(true, Ordering::Release);
            }
        });
    }

    fn arm_ctrl_in(data: &[u8]) -> Result<(), EndpointError> {
        let mps = SHARED.control_mps.load(Ordering::Relaxed);
        let len = data.len().min(mps);
        SHARED.setup_armed.store(false, Ordering::Release);
        let started = SHARED.with_core_and_bufs(|core, bufs| {
            SHARED.in_state[0].store(EP_PENDING, Ordering::Release);
            bufs.ctrl_in.0[..len].copy_from_slice(&data[..len]);
            core.start_write_dma(0x80, &bufs.ctrl_in.0[..len])
        });
        match started {
            Some(Ok(())) => Ok(()),
            _ => Err(EndpointError::Disabled),
        }
    }

    fn arm_ctrl_out(buf: &mut [u8]) -> Result<usize, EndpointError> {
        let mps = SHARED.control_mps.load(Ordering::Relaxed);
        let len = buf.len().min(mps);
        SHARED.setup_armed.store(false, Ordering::Release);
        SHARED.ctrl_out_armed.store(false, Ordering::Release);
        let started = SHARED.with_core_and_bufs(|core, bufs| {
            SHARED.out_state[0].store(EP_PENDING, Ordering::Release);
            let result = core.start_read_dma(0x00, &mut bufs.ctrl_out.0[..len]);
            if result.is_ok() {
                SHARED.ctrl_out_armed.store(true, Ordering::Release);
            }
            result
        });
        match started {
            Some(Ok(())) => Ok(len),
            _ => Err(EndpointError::Disabled),
        }
    }

    async fn wait_ctrl_in_complete() -> Result<usize, EndpointError> {
        poll_fn(|cx| {
            SHARED.ctrl_in_waker.register(cx.waker());
            if SHARED.setup_ready.load(Ordering::Acquire) {
                return Poll::Ready(Err(EndpointError::Disabled));
            }
            match SHARED.in_state[0].load(Ordering::Acquire) {
                EP_DONE => Poll::Ready(Ok(SHARED.in_len[0].load(Ordering::Relaxed))),
                EP_DISABLED => Poll::Ready(Err(EndpointError::Disabled)),
                _ => Poll::Pending,
            }
        })
        .await
    }

    async fn wait_ctrl_out_complete() -> Result<usize, EndpointError> {
        poll_fn(|cx| {
            SHARED.ctrl_out_waker.register(cx.waker());
            if SHARED.setup_ready.load(Ordering::Acquire) {
                return Poll::Ready(Err(EndpointError::Disabled));
            }
            match SHARED.out_state[0].load(Ordering::Acquire) {
                EP_DONE => Poll::Ready(Ok(SHARED.out_len[0].load(Ordering::Relaxed))),
                EP_DISABLED => Poll::Ready(Err(EndpointError::Disabled)),
                _ => Poll::Pending,
            }
        })
        .await
    }

    async fn status_in(&mut self) -> Result<(), EndpointError> {
        if SHARED.setup_ready.load(Ordering::Acquire) {
            return Err(EndpointError::Disabled);
        }
        Self::arm_ctrl_in(&[])?;
        Self::wait_ctrl_in_complete().await.map(|_| ())
    }

    async fn status_out(&mut self) -> Result<(), EndpointError> {
        Self::arm_ctrl_out(&mut [])?;
        Self::wait_ctrl_out_complete().await.map(|_| ())
    }
}

impl ControlPipe for EmbassyControlPipe {
    fn max_packet_size(&self) -> usize {
        SHARED.control_mps.load(Ordering::Relaxed)
    }

    async fn setup(&mut self) -> [u8; 8] {
        poll_fn(|cx| {
            SHARED.setup_waker.register(cx.waker());
            if SHARED.setup_ready.load(Ordering::Acquire) {
                SHARED.setup_ready.store(false, Ordering::Release);
                let packet = SHARED.with_bufs(|bufs| {
                    let mut out = [0u8; 8];
                    out.copy_from_slice(&bufs.setup.0[..8]);
                    out
                });
                return Poll::Ready(packet);
            }
            Self::arm_setup();
            Poll::Pending
        })
        .await
    }

    async fn data_out(
        &mut self,
        buf: &mut [u8],
        _first: bool,
        _last: bool,
    ) -> Result<usize, EndpointError> {
        if SHARED.setup_ready.load(Ordering::Acquire) {
            return Err(EndpointError::Disabled);
        }
        let armed = Self::arm_ctrl_out(buf)?;
        let actual = Self::wait_ctrl_out_complete()
            .await?
            .min(armed)
            .min(buf.len());
        SHARED.with_bufs(|bufs| {
            buf[..actual].copy_from_slice(&bufs.ctrl_out.0[..actual]);
        });
        Ok(actual)
    }

    async fn data_in(
        &mut self,
        data: &[u8],
        _first: bool,
        last: bool,
    ) -> Result<(), EndpointError> {
        if SHARED.setup_ready.load(Ordering::Acquire) {
            return Err(EndpointError::Disabled);
        }
        Self::arm_ctrl_in(data)?;
        Self::wait_ctrl_in_complete().await?;
        if last {
            self.status_out().await?;
        }
        Ok(())
    }

    async fn accept(&mut self) {
        let _ = self.status_in().await;
    }

    async fn reject(&mut self) {
        let _ = SHARED.with_core(|core| {
            let _ = core.stall(0x00, true);
            core.stall(0x80, true)
        });
        SHARED.setup_armed.store(false, Ordering::Release);
        SHARED.ep0_configured.store(false, Ordering::Release);
    }

    async fn accept_set_address(&mut self, addr: u8) {
        // DWC2 latches the device address before sending the status-stage IN ZLP.
        let _ = SHARED.with_core(|core| core.set_address(addr));
        let _ = self.status_in().await;
    }
}
