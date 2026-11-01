//! `embassy-usb-driver` IN and OUT endpoint implementations.

use core::future::poll_fn;
use core::sync::atomic::Ordering;
use core::task::Poll;

use embassy_usb_driver::{Endpoint, EndpointError, EndpointIn, EndpointInfo, EndpointOut};

use super::{AlignedBuf, EP_DISABLED, EP_DONE, EP_PENDING, MAX_PACKET_SIZE, SHARED};

/// IN endpoint handle for [`UsbDevDriver`](crate::usbdrd::device::UsbDevDriver).
pub struct EmbassyEndpointIn {
    idx: usize,
    info: EndpointInfo,
    buf: AlignedBuf<MAX_PACKET_SIZE>,
}

impl EmbassyEndpointIn {
    pub(super) const fn new(idx: usize, info: EndpointInfo) -> Self {
        Self {
            idx,
            info,
            buf: AlignedBuf::new(),
        }
    }

    async fn wait_in_enabled(idx: usize) {
        poll_fn(|cx| {
            SHARED.in_waker[idx].register(cx.waker());
            SHARED.bus_waker.register(cx.waker());
            if SHARED.in_enabled[idx].load(Ordering::Acquire) {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await
    }

    async fn wait_in_complete(idx: usize) -> Result<usize, EndpointError> {
        poll_fn(|cx| {
            SHARED.in_waker[idx].register(cx.waker());
            match SHARED.in_state[idx].load(Ordering::Acquire) {
                EP_DONE => Poll::Ready(Ok(SHARED.in_len[idx].load(Ordering::Relaxed))),
                EP_DISABLED => Poll::Ready(Err(EndpointError::Disabled)),
                _ => Poll::Pending,
            }
        })
        .await
    }
}

impl Endpoint for EmbassyEndpointIn {
    fn info(&self) -> &EndpointInfo {
        &self.info
    }

    async fn wait_enabled(&mut self) {
        Self::wait_in_enabled(self.idx).await;
    }
}

impl EndpointIn for EmbassyEndpointIn {
    async fn write(&mut self, buf: &[u8]) -> Result<(), EndpointError> {
        let idx = self.idx;
        if buf.len() > self.info.max_packet_size as usize {
            return Err(EndpointError::BufferOverflow);
        }
        Self::wait_in_enabled(idx).await;
        let len = buf.len();
        self.buf.as_mut_slice()[..len].copy_from_slice(buf);
        let addr = u8::from(self.info.addr);
        let started = SHARED.with_core(|core| {
            SHARED.in_state[idx].store(EP_PENDING, Ordering::Release);
            core.start_write_dma(addr, &self.buf.as_slice()[..len])
        });
        if !matches!(started, Some(Ok(()))) {
            return Err(EndpointError::Disabled);
        }
        Self::wait_in_complete(idx).await.map(|_| ())
    }
}

/// OUT endpoint handle for [`UsbDevDriver`](crate::usbdrd::device::UsbDevDriver).
pub struct EmbassyEndpointOut {
    idx: usize,
    info: EndpointInfo,
    buf: AlignedBuf<MAX_PACKET_SIZE>,
}

impl EmbassyEndpointOut {
    pub(super) const fn new(idx: usize, info: EndpointInfo) -> Self {
        Self {
            idx,
            info,
            buf: AlignedBuf::new(),
        }
    }

    async fn wait_out_enabled(idx: usize) {
        poll_fn(|cx| {
            SHARED.out_waker[idx].register(cx.waker());
            SHARED.bus_waker.register(cx.waker());
            if SHARED.out_enabled[idx].load(Ordering::Acquire) {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await
    }

    async fn wait_out_complete(idx: usize) -> Result<usize, EndpointError> {
        poll_fn(|cx| {
            SHARED.out_waker[idx].register(cx.waker());
            match SHARED.out_state[idx].load(Ordering::Acquire) {
                EP_DONE => Poll::Ready(Ok(SHARED.out_len[idx].load(Ordering::Relaxed))),
                EP_DISABLED => Poll::Ready(Err(EndpointError::Disabled)),
                _ => Poll::Pending,
            }
        })
        .await
    }
}

impl Endpoint for EmbassyEndpointOut {
    fn info(&self) -> &EndpointInfo {
        &self.info
    }

    async fn wait_enabled(&mut self) {
        Self::wait_out_enabled(self.idx).await;
    }
}

impl EndpointOut for EmbassyEndpointOut {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, EndpointError> {
        let idx = self.idx;
        let mps = self.info.max_packet_size as usize;
        let want = buf.len().min(mps);
        Self::wait_out_enabled(idx).await;
        let addr = u8::from(self.info.addr);
        let started = SHARED.with_core(|core| {
            SHARED.out_state[idx].store(EP_PENDING, Ordering::Release);
            core.start_read_dma(addr, &mut self.buf.as_mut_slice()[..want])
        });
        if !matches!(started, Some(Ok(()))) {
            return Err(EndpointError::Disabled);
        }
        let actual = Self::wait_out_complete(idx).await?.min(buf.len());
        buf[..actual].copy_from_slice(&self.buf.as_slice()[..actual]);
        Ok(actual)
    }
}
