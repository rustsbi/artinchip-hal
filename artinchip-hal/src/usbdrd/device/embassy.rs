//! `embassy-usb-driver` implementation for [`UsbDevDriver`].

mod bus;
mod control;
mod endpoint;

use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, AtomicI8, AtomicU8, AtomicUsize, Ordering};

use critical_section::{CriticalSection, Mutex};
use embassy_sync::waitqueue::AtomicWaker;
use embassy_usb_driver::{
    Direction, Driver, EndpointAddress, EndpointAllocError, EndpointInfo, EndpointType,
};

pub use bus::EmbassyBus;
pub use control::EmbassyControlPipe;
pub use endpoint::{EmbassyEndpointIn, EmbassyEndpointOut};

use super::dev_register::RegisterBlock;
use super::driver::{ControllerCore, EP_COUNT};
use super::{UsbDevDriver, UsbDevError, UsbDevEvent};
use crate::cmu::Cmu;
use crate::interrupt::typelevel::{self, Interrupt as _};
use crate::sys_cfg::SocSysCfg;
use crate::usbdrd::{UsbConfig, UsbPads};

/// Maximum bulk packet size supported by High-Speed endpoints.
pub(super) const MAX_PACKET_SIZE: usize = 512;

pub(super) const EP_PENDING: i8 = 0;
pub(super) const EP_DONE: i8 = 1;
pub(super) const EP_DISABLED: i8 = -1;

pub(super) const BUS_POWER: u8 = 1 << 0;
pub(super) const BUS_RESET: u8 = 1 << 1;
pub(super) const BUS_SUSPEND: u8 = 1 << 2;
pub(super) const BUS_RESUME: u8 = 1 << 3;

/// Cache-line-aligned DMA bounce buffer.
#[repr(C, align(64))]
pub(super) struct AlignedBuf<const N: usize>(pub(super) [u8; N]);

impl<const N: usize> AlignedBuf<N> {
    pub(super) const fn new() -> Self {
        Self([0; N])
    }

    pub(super) fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub(super) fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

/// Cache-line-aligned EP0 DMA buffers owned by [`Shared`].
pub(super) struct ControlBuffers {
    pub(super) setup: AlignedBuf<24>,
    pub(super) ctrl_in: AlignedBuf<64>,
    pub(super) ctrl_out: AlignedBuf<64>,
}

impl ControlBuffers {
    const fn new() -> Self {
        Self {
            setup: AlignedBuf::new(),
            ctrl_in: AlignedBuf::new(),
            ctrl_out: AlignedBuf::new(),
        }
    }
}

struct EndpointInfos {
    in_ep: [Option<EndpointInfo>; EP_COUNT],
    out_ep: [Option<EndpointInfo>; EP_COUNT],
}

impl EndpointInfos {
    const fn new() -> Self {
        Self {
            in_ep: [None; EP_COUNT],
            out_ep: [None; EP_COUNT],
        }
    }
}

/// State shared between the `USB_DEV` interrupt handler and async endpoint futures.
pub(super) struct Shared {
    core: Mutex<RefCell<Option<ControllerCore<'static>>>>,
    ctrl_bufs: Mutex<RefCell<ControlBuffers>>,
    ep_info: Mutex<RefCell<EndpointInfos>>,
    pub(super) bus_waker: AtomicWaker,
    pub(super) setup_waker: AtomicWaker,
    pub(super) ctrl_in_waker: AtomicWaker,
    pub(super) ctrl_out_waker: AtomicWaker,
    pub(super) in_waker: [AtomicWaker; EP_COUNT],
    pub(super) out_waker: [AtomicWaker; EP_COUNT],
    pub(super) in_state: [AtomicI8; EP_COUNT],
    pub(super) out_state: [AtomicI8; EP_COUNT],
    pub(super) in_len: [AtomicUsize; EP_COUNT],
    pub(super) out_len: [AtomicUsize; EP_COUNT],
    pub(super) in_enabled: [AtomicBool; EP_COUNT],
    pub(super) out_enabled: [AtomicBool; EP_COUNT],
    pub(super) in_stalled: [AtomicBool; EP_COUNT],
    pub(super) out_stalled: [AtomicBool; EP_COUNT],
    pub(super) in_used: [AtomicBool; EP_COUNT],
    pub(super) out_used: [AtomicBool; EP_COUNT],
    pub(super) setup_ready: AtomicBool,
    pub(super) setup_armed: AtomicBool,
    pub(super) ctrl_out_armed: AtomicBool,
    pub(super) ep0_configured: AtomicBool,
    pub(super) enumerated: AtomicBool,
    pub(super) bus_events: AtomicU8,
    pub(super) control_mps: AtomicUsize,
}

pub(super) static SHARED: Shared = Shared::new();

impl Shared {
    const fn new() -> Self {
        Self {
            core: Mutex::new(RefCell::new(None)),
            ctrl_bufs: Mutex::new(RefCell::new(ControlBuffers::new())),
            ep_info: Mutex::new(RefCell::new(EndpointInfos::new())),
            bus_waker: AtomicWaker::new(),
            setup_waker: AtomicWaker::new(),
            ctrl_in_waker: AtomicWaker::new(),
            ctrl_out_waker: AtomicWaker::new(),
            in_waker: [const { AtomicWaker::new() }; EP_COUNT],
            out_waker: [const { AtomicWaker::new() }; EP_COUNT],
            in_state: [const { AtomicI8::new(EP_PENDING) }; EP_COUNT],
            out_state: [const { AtomicI8::new(EP_PENDING) }; EP_COUNT],
            in_len: [const { AtomicUsize::new(0) }; EP_COUNT],
            out_len: [const { AtomicUsize::new(0) }; EP_COUNT],
            in_enabled: [const { AtomicBool::new(false) }; EP_COUNT],
            out_enabled: [const { AtomicBool::new(false) }; EP_COUNT],
            in_stalled: [const { AtomicBool::new(false) }; EP_COUNT],
            out_stalled: [const { AtomicBool::new(false) }; EP_COUNT],
            in_used: [const { AtomicBool::new(false) }; EP_COUNT],
            out_used: [const { AtomicBool::new(false) }; EP_COUNT],
            setup_ready: AtomicBool::new(false),
            setup_armed: AtomicBool::new(false),
            ctrl_out_armed: AtomicBool::new(false),
            ep0_configured: AtomicBool::new(false),
            enumerated: AtomicBool::new(false),
            bus_events: AtomicU8::new(BUS_POWER),
            control_mps: AtomicUsize::new(64),
        }
    }

    fn reset_session(&self) {
        critical_section::with(|cs| {
            *self.core.borrow_ref_mut(cs) = None;
            *self.ep_info.borrow_ref_mut(cs) = EndpointInfos::new();
            for idx in 0..EP_COUNT {
                self.in_state[idx].store(EP_PENDING, Ordering::Release);
                self.out_state[idx].store(EP_PENDING, Ordering::Release);
                self.in_len[idx].store(0, Ordering::Release);
                self.out_len[idx].store(0, Ordering::Release);
                self.in_enabled[idx].store(false, Ordering::Release);
                self.out_enabled[idx].store(false, Ordering::Release);
                self.in_stalled[idx].store(false, Ordering::Release);
                self.out_stalled[idx].store(false, Ordering::Release);
                self.in_used[idx].store(false, Ordering::Release);
                self.out_used[idx].store(false, Ordering::Release);
            }
            self.setup_ready.store(false, Ordering::Release);
            self.setup_armed.store(false, Ordering::Release);
            self.ctrl_out_armed.store(false, Ordering::Release);
            self.ep0_configured.store(false, Ordering::Release);
            self.enumerated.store(false, Ordering::Release);
            self.bus_events.store(BUS_POWER, Ordering::Release);
            self.control_mps.store(64, Ordering::Release);
        });
    }

    pub(super) fn with_core<R>(
        &self,
        f: impl FnOnce(&mut ControllerCore<'static>) -> R,
    ) -> Option<R> {
        critical_section::with(|cs| {
            let mut slot = self.core.borrow_ref_mut(cs);
            slot.as_mut().map(f)
        })
    }

    pub(super) fn with_core_and_bufs<R>(
        &self,
        f: impl FnOnce(&mut ControllerCore<'static>, &mut ControlBuffers) -> R,
    ) -> Option<R> {
        critical_section::with(|cs| {
            let mut slot = self.core.borrow_ref_mut(cs);
            let mut bufs = self.ctrl_bufs.borrow_ref_mut(cs);
            slot.as_mut().map(|core| f(core, &mut bufs))
        })
    }

    pub(super) fn with_bufs<R>(&self, f: impl FnOnce(&mut ControlBuffers) -> R) -> R {
        critical_section::with(|cs| {
            let mut bufs = self.ctrl_bufs.borrow_ref_mut(cs);
            f(&mut bufs)
        })
    }

    pub(super) fn ep_info(&self, in_dir: bool, idx: usize) -> Option<EndpointInfo> {
        critical_section::with(|cs| {
            let infos = self.ep_info.borrow_ref(cs);
            if in_dir {
                infos.in_ep[idx]
            } else {
                infos.out_ep[idx]
            }
        })
    }

    /// Route one controller event to the waiting futures.
    fn route_in_cs(&self, cs: CriticalSection<'_>, event: UsbDevEvent) {
        match event {
            UsbDevEvent::Setup => {
                self.setup_armed.store(false, Ordering::Release);
                if self.ctrl_out_armed.swap(false, Ordering::AcqRel) {
                    // When a new SETUP preempts an armed EP0 OUT stage, hardware DMA
                    // wrote the 8-byte SETUP packet into `ctrl_out`; copy it to `setup`.
                    let mut bufs = self.ctrl_bufs.borrow_ref_mut(cs);
                    let src = bufs.ctrl_out.0.as_ptr();
                    unsafe { crate::cache::dcache_invalidate_range(src as usize, 8) };
                    let mut packet = [0u8; 8];
                    packet.copy_from_slice(&bufs.ctrl_out.0[..8]);
                    bufs.setup.0[..8].copy_from_slice(&packet);
                }
                self.setup_ready.store(true, Ordering::Release);
                self.setup_waker.wake();
                self.ctrl_in_waker.wake();
                self.ctrl_out_waker.wake();
            }
            UsbDevEvent::InComplete { ep, len } => {
                let idx = (ep & 0x0f) as usize;
                if idx >= EP_COUNT {
                    return;
                }
                self.in_len[idx].store(len, Ordering::Relaxed);
                self.in_state[idx].store(EP_DONE, Ordering::Release);
                self.in_waker[idx].wake();
                if idx == 0 {
                    self.ctrl_in_waker.wake();
                }
            }
            UsbDevEvent::OutComplete { ep, len } => {
                let idx = (ep & 0x0f) as usize;
                if idx >= EP_COUNT {
                    return;
                }
                if idx == 0 {
                    self.ctrl_out_armed.store(false, Ordering::Release);
                }
                self.out_len[idx].store(len, Ordering::Relaxed);
                self.out_state[idx].store(EP_DONE, Ordering::Release);
                self.out_waker[idx].wake();
                if idx == 0 {
                    self.ctrl_out_waker.wake();
                }
            }
            UsbDevEvent::Reset => {
                self.setup_ready.store(false, Ordering::Release);
                self.setup_armed.store(false, Ordering::Release);
                self.ctrl_out_armed.store(false, Ordering::Release);
                self.ep0_configured.store(false, Ordering::Release);
                for idx in 0..EP_COUNT {
                    self.in_enabled[idx].store(false, Ordering::Release);
                    self.out_enabled[idx].store(false, Ordering::Release);
                    self.in_state[idx].store(EP_DISABLED, Ordering::Release);
                    self.out_state[idx].store(EP_DISABLED, Ordering::Release);
                    self.in_waker[idx].wake();
                    self.out_waker[idx].wake();
                }
                self.ctrl_in_waker.wake();
                self.ctrl_out_waker.wake();
                self.setup_waker.wake();
                self.bus_events.fetch_or(BUS_RESET, Ordering::Release);
                self.bus_waker.wake();
            }
            UsbDevEvent::Enumerated(_) => {
                self.enumerated.store(true, Ordering::Release);
                self.bus_waker.wake();
            }
            UsbDevEvent::Suspend => {
                self.bus_events.fetch_or(BUS_SUSPEND, Ordering::Release);
                self.bus_waker.wake();
            }
            UsbDevEvent::Resume => {
                self.bus_events.fetch_or(BUS_RESUME, Ordering::Release);
                self.bus_waker.wake();
            }
            UsbDevEvent::Error(_) => {}
        }
    }
}

fn on_interrupt() {
    critical_section::with(|cs| {
        let mut slot = SHARED.core.borrow_ref_mut(cs);
        match slot.as_mut() {
            Some(core) => {
                while let Some(event) = core.handle_interrupt() {
                    SHARED.route_in_cs(cs, event);
                }
            }
            None => typelevel::USB_DEV::disable(),
        }
    });
    typelevel::USB_DEV::clear_pending();
}

/// Type-level `USB_DEV` interrupt handler for `embassy-usb`.
pub struct UsbDevInterrupt;

impl typelevel::Handler<typelevel::USB_DEV> for UsbDevInterrupt {
    unsafe fn on_interrupt() {
        on_interrupt();
    }
}

impl<PAD> UsbDevDriver<'static, PAD>
where
    PAD: UsbPads<0> + 'static,
{
    /// Initialize the controller in deferred mode for `embassy-usb`.
    pub fn new_embassy<IRQS>(
        reg: &'static RegisterBlock,
        pads: PAD,
        config: UsbConfig,
        _irqs: IRQS,
        cmu: &mut Cmu,
        syscfg: &mut SocSysCfg,
    ) -> Result<Self, UsbDevError>
    where
        IRQS: typelevel::Binding<typelevel::USB_DEV, UsbDevInterrupt>,
    {
        typelevel::USB_DEV::disable();
        typelevel::USB_DEV::clear_pending();
        IRQS::bind();
        SHARED.reset_session();
        Self::new_deferred(reg, pads, config, cmu, syscfg)
    }

    fn alloc_ep(
        &mut self,
        in_dir: bool,
        ep_type: EndpointType,
        ep_addr: Option<EndpointAddress>,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<(usize, EndpointInfo), EndpointAllocError> {
        if max_packet_size == 0 || max_packet_size as usize > MAX_PACKET_SIZE {
            return Err(EndpointAllocError);
        }
        if !in_dir && ep_type == EndpointType::Control {
            return Err(EndpointAllocError);
        }
        let requested = ep_addr.map(|addr| addr.index());
        if requested == Some(0) {
            return Err(EndpointAllocError);
        }
        let used = if in_dir {
            &SHARED.in_used
        } else {
            &SHARED.out_used
        };
        let mut chosen = None;
        match requested {
            Some(idx) => {
                if idx >= EP_COUNT || used[idx].swap(true, Ordering::AcqRel) {
                    return Err(EndpointAllocError);
                }
                chosen = Some(idx);
            }
            None => {
                for (idx, slot) in used.iter().enumerate().skip(1) {
                    if !slot.swap(true, Ordering::AcqRel) {
                        chosen = Some(idx);
                        break;
                    }
                }
            }
        }
        let idx = chosen.ok_or(EndpointAllocError)?;
        let addr = EndpointAddress::from_parts(
            idx,
            if in_dir {
                Direction::In
            } else {
                Direction::Out
            },
        );
        let info = EndpointInfo {
            addr,
            ep_type,
            max_packet_size,
            interval_ms,
        };
        critical_section::with(|cs| {
            let mut infos = SHARED.ep_info.borrow_ref_mut(cs);
            if in_dir {
                infos.in_ep[idx] = Some(info);
            } else {
                infos.out_ep[idx] = Some(info);
            }
        });
        Ok((idx, info))
    }
}

impl UsbDevDriver<'static, ()> {
    /// Tear down the started `embassy-usb` controller instance and release its clocks/resets.
    pub fn teardown_embassy(cmu: &mut Cmu) {
        let core = critical_section::with(|cs| {
            typelevel::USB_DEV::disable();
            typelevel::USB_DEV::clear_pending();
            SHARED.core.borrow_ref_mut(cs).take()
        });
        if let Some(mut core) = core {
            core.mute_controller();
        }
        super::driver::release_clocks_and_reset(cmu);
    }
}

impl<'d, PAD> Driver<'d> for UsbDevDriver<'static, PAD>
where
    PAD: UsbPads<0> + 'static,
{
    type EndpointOut = EmbassyEndpointOut;
    type EndpointIn = EmbassyEndpointIn;
    type ControlPipe = EmbassyControlPipe;
    type Bus = EmbassyBus<PAD>;

    fn alloc_endpoint_out(
        &mut self,
        ep_type: EndpointType,
        ep_addr: Option<EndpointAddress>,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Self::EndpointOut, EndpointAllocError> {
        let (idx, info) = self.alloc_ep(false, ep_type, ep_addr, max_packet_size, interval_ms)?;
        Ok(EmbassyEndpointOut::new(idx, info))
    }

    fn alloc_endpoint_in(
        &mut self,
        ep_type: EndpointType,
        ep_addr: Option<EndpointAddress>,
        max_packet_size: u16,
        interval_ms: u8,
    ) -> Result<Self::EndpointIn, EndpointAllocError> {
        let (idx, info) = self.alloc_ep(true, ep_type, ep_addr, max_packet_size, interval_ms)?;
        Ok(EmbassyEndpointIn::new(idx, info))
    }

    fn start(self, control_max_packet_size: u16) -> (Self::Bus, Self::ControlPipe) {
        let Self { mut core, pads } = self;
        SHARED
            .control_mps
            .store(control_max_packet_size as usize, Ordering::Release);
        core.enable_global_interrupt();
        critical_section::with(|cs| {
            *SHARED.core.borrow_ref_mut(cs) = Some(core);
        });
        typelevel::USB_DEV::enable();
        (EmbassyBus::new(pads), EmbassyControlPipe::new())
    }
}
