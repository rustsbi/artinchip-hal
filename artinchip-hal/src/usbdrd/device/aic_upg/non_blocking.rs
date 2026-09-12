//! AIC USB upgrade device.
//!
//! Owns **enumeration** (the control endpoint, the descriptor set and the
//! standard requests) and the vendor bulk endpoints, and drives them from the
//! USB interrupt. Enumeration has to be exactly right — a device that answers
//! even one standard request with the wrong data is rejected by the host, which
//! then resets the bus over and over (the symptom is an endless
//! `bus reset, restarting` loop).
//!
//! Same shape as `uart::non_blocking`: the interrupt handler drives the hardware
//! (it acknowledges the endpoint interrupts and queues the resulting events)
//! and the async task only consumes those events. That split is not cosmetic:
//! the controller's interrupt is *level* triggered, so leaving the
//! acknowledgement to the task keeps the line asserted and the task never runs.
//!
//! The protocol engine (`super::Engine`) and its storage backend
//! (`super::Storage`) sit on top of [`AsyncAicUpg`].

use core::cell::RefCell;
use core::future::poll_fn;
use core::mem;
use core::task::Poll;

use critical_section::Mutex;
use embassy_sync::waitqueue::AtomicWaker;
use log::{debug, error};

use super::super::dev_register::EndpointType;
use super::super::{UsbDevDriver, UsbDevError, UsbDevEvent, UsbEndpointConfig};
use super::protocol::{
    CBW_SIZE, CSW_SIZE, Cbw, Csw, Engine, RESP_FAIL, RESP_OK, TRANSPORT_READ, TRANSPORT_WRITE,
};
use crate::interrupt::clic::typelevel::{self, Interrupt as _};
use crate::usbdrd::UsbPads;

const BULK_OUT: u8 = 0x02;
const BULK_IN: u8 = 0x81;
const EP0: u8 = 0x00;
const EP0_IN: u8 = 0x80;
/// EP0 max packet size, and the value advertised as `bMaxPacketSize0`.
const EP0_MPS: usize = 64;
/// Bulk max packet size. The vendor bootloader is built with `CONFIG_USB_HS`
/// (see `images/rtua.py` in the SDK build output), so the device runs at High
/// Speed and advertises 512-byte bulk endpoints.
const BULK_MPS: u16 = 512;
const VID: u16 = 0x33c3;
const PID: u16 = 0x6677;

/// Descriptor blob aligned to a cache line.
///
/// The controller's DMA writes and reads whole cache lines, so every buffer it
/// touches has to be cache-line aligned: invalidating a range that shares a line
/// with unrelated data would discard it. 64 bytes covers both the 32-byte D13x
/// and the 64-byte D21x cache lines.
#[repr(align(64))]
struct Aligned<const N: usize>([u8; N]);

/// Device descriptor. Advertises `iManufacturer = 1` and `iProduct = 2`, so the
/// strings below are *not* optional: the host will request them.
static DEVICE_DESC: Aligned<18> = Aligned([
    18,
    0x01, // DEVICE
    0x00,
    0x02, // bcdUSB 2.00
    0xff,
    0xff,
    0xff, // vendor specific
    EP0_MPS as u8,
    (VID & 0xff) as u8,
    (VID >> 8) as u8,
    (PID & 0xff) as u8,
    (PID >> 8) as u8,
    0x01,
    0x01, // bcdDevice 1.01, matching the vendor blob
    1,    // iManufacturer
    2,    // iProduct
    0,    // iSerialNumber
    1,    // bNumConfigurations
]);

/// Configuration: 1 interface, 2 vendor bulk endpoints.
///
/// Byte-for-byte identical to the vendor's `usb_upg_descriptor[]`
/// (`cherryusb/demo/aicupg_for_usb.c`), which the USB capture confirms.
static CONFIG_DESC: Aligned<32> = Aligned([
    9,
    0x02, // CONFIGURATION
    32,
    0x00, // wTotalLength
    1,    // bNumInterfaces
    1,    // bConfigurationValue
    0,    // iConfiguration
    0x80, // bus powered
    0xfa, // 500 mA
    9,
    0x04, // INTERFACE
    0,
    0, // interface number / alternate setting
    2, // bNumEndpoints
    0xff,
    0xff,
    0xff, // vendor specific
    0,    // iInterface
    7,
    0x05, // ENDPOINT
    BULK_IN,
    0x02, // bulk
    (BULK_MPS & 0xff) as u8,
    (BULK_MPS >> 8) as u8, // wMaxPacketSize
    0,                     // bInterval
    7,
    0x05, // ENDPOINT
    BULK_OUT,
    0x02, // bulk
    (BULK_MPS & 0xff) as u8,
    (BULK_MPS >> 8) as u8, // wMaxPacketSize
    0,                     // bInterval
]);

/// Device qualifier descriptor, present because the device is High Speed
/// capable. The host asks for it during enumeration and stalls are tolerated,
/// but answering it matches the vendor exactly.
static QUALIFIER_DESC: Aligned<10> = Aligned([
    10,
    0x06, // DEVICE_QUALIFIER
    0x00,
    0x02, // bcdUSB 2.00
    0xff,
    0xff,
    0xff,
    EP0_MPS as u8,
    1, // bNumConfigurations
    0,
]);

/// String descriptor 0: the supported language IDs (en-US).
static STR_LANG: Aligned<4> = Aligned([4, 0x03, 0x09, 0x04]);

/// String descriptor 1: manufacturer, UTF-16LE "Artinchip".
static STR_MFG: Aligned<22> = Aligned([
    22, 0x03, b'A', 0, b'r', 0, b't', 0, b'i', 0, b'n', 0, b'c', 0, b'h', 0, b'i', 0, b'p', 0, 0, 0,
]);

/// String descriptor 2: product, UTF-16LE "Artinchip Device".
static STR_PRODUCT: Aligned<36> = Aligned([
    36, 0x03, b'A', 0, b'r', 0, b't', 0, b'i', 0, b'n', 0, b'c', 0, b'h', 0, b'i', 0, b'p', 0,
    b' ', 0, b'D', 0, b'e', 0, b'v', 0, b'i', 0, b'c', 0, b'e', 0, 0, 0,
]);

/// String descriptor 3: the vendor's build stamp. Unreferenced by the device
/// descriptor, but present in the vendor blob and cheap to reproduce.
static STR_BUILD: Aligned<22> = Aligned([
    22, 0x03, b'2', 0, b'0', 0, b'2', 0, b'2', 0, b'1', 0, b'2', 0, b'3', 0, b'4', 0, b'5', 0,
    b'6', 0,
]);

/// Reply to `GET_STATUS`: two zero bytes.
static STATUS_ZERO: Aligned<2> = Aligned([0, 0]);
/// Reply to `GET_CONFIGURATION`: selected configuration (1 after SET_CONFIGURATION).
static CONFIG_SET: Aligned<1> = Aligned([1]);
/// Reply to `GET_CONFIGURATION` while unconfigured.
static CONFIG_CLEAR: Aligned<1> = Aligned([0]);

#[repr(C, align(64))]
struct Setup([u8; 8]);

/// EP0 SETUP buffer.
///
/// Static, so the DMA address the controller is programmed with stays valid
/// wherever the device struct itself is stored: the SETUP receive is re-armed
/// all the time, including across enumeration.
static mut SETUP: Setup = Setup([0; 8]);

fn setup_buffer() -> &'static mut Setup {
    // SAFETY: there is a single USB device controller, so this is its only user.
    unsafe { &mut *core::ptr::addr_of_mut!(SETUP) }
}

/// AIC USB upgrade device. The upgrade protocol payload is deliberately left to
/// the application; this type owns enumeration, bulk endpoints and DMA IRQs.
pub(crate) struct AicUpg<'a, PAD> {
    driver: UsbDevDriver<'a, PAD>,
    setup: &'static mut Setup,
    configured: bool,
    /// EP0 IN data of the control transfer in progress.
    ep0_in: &'static [u8],
    ep0_in_pos: usize,
    /// The transfer in progress has a data stage.
    ///
    /// That decides who terminates it: with a data stage the host sends a
    /// zero-length OUT packet, without one the zero-length IN we send *is* the
    /// status stage and the transfer is already over.
    ep0_had_data: bool,
}

impl<'a, PAD> AicUpg<'a, PAD>
where
    PAD: UsbPads<0>,
{
    pub(crate) fn new(mut driver: UsbDevDriver<'a, PAD>) -> Result<Self, UsbDevError> {
        Self::open_endpoints(&mut driver)?;
        Ok(Self {
            driver,
            setup: setup_buffer(),
            configured: false,
            ep0_in: &[],
            ep0_in_pos: 0,
            ep0_had_data: false,
        })
    }

    /// Configure the endpoints this device uses.
    ///
    /// Called at construction and again after a bus reset: a reset drops all
    /// endpoint state *and* the per-endpoint interrupt masks, so without this the
    /// device would stop seeing endpoint events after the first reset.
    fn open_endpoints(driver: &mut UsbDevDriver<'a, PAD>) -> Result<(), UsbDevError> {
        driver.open_endpoint(UsbEndpointConfig {
            address: EP0,
            max_packet_size: EP0_MPS as u16,
            ep_type: EndpointType::Control,
        })?;
        driver.open_endpoint(UsbEndpointConfig {
            address: EP0_IN,
            max_packet_size: EP0_MPS as u16,
            ep_type: EndpointType::Control,
        })?;
        driver.open_endpoint(UsbEndpointConfig {
            address: BULK_IN,
            max_packet_size: BULK_MPS,
            ep_type: EndpointType::Bulk,
        })?;
        driver.open_endpoint(UsbEndpointConfig {
            address: BULK_OUT,
            max_packet_size: BULK_MPS,
            ep_type: EndpointType::Bulk,
        })?;
        Ok(())
    }

    pub fn driver_mut(&mut self) -> &mut UsbDevDriver<'a, PAD> {
        &mut self.driver
    }

    /// Start a bulk OUT DMA receive into an application-owned buffer.
    pub fn receive(&mut self, buffer: &'a mut [u8]) -> Result<(), UsbDevError> {
        self.driver.start_read_dma(BULK_OUT, buffer)
    }

    /// Start a bulk IN DMA send from an application-owned buffer.
    pub fn send(&mut self, buffer: &'a [u8]) -> Result<(), UsbDevError> {
        self.driver.start_write_dma(BULK_IN, buffer)
    }

    /// Service one IRQ. Call from the USB DEV interrupt handler or from a task
    /// woken by it.
    pub fn handle_interrupt(&mut self) -> Option<UsbDevEvent> {
        let event = self.driver.handle_interrupt();
        if let Some(event) = event {
            match event {
                UsbDevEvent::Reset => {
                    self.configured = false;
                    self.ep0_in = &[];
                    self.ep0_in_pos = 0;
                    self.ep0_had_data = false;
                    // A reset drops the endpoint configuration (`USB_EP_INT_MSK`
                    // included), so it has to be rebuilt before re-arming EP0.
                    let _ = Self::open_endpoints(&mut self.driver);
                }
                UsbDevEvent::Setup => self.handle_setup(),
                UsbDevEvent::InComplete { ep: EP0_IN, len: _ } => {
                    if self.ep0_in_pos < self.ep0_in.len() {
                        // More of the control-IN payload to send.
                        self.arm_ep0_in();
                    } else if !self.ep0_had_data {
                        // A no-data request is terminated by the zero-length IN
                        // we just sent, which *was* its status stage: the host
                        // sends no OUT status afterwards, so nothing else would
                        // ever re-arm EP0 for the next SETUP. Without this,
                        // SET_ADDRESS (and SET_CONFIGURATION, CLEAR_FEATURE...)
                        // leave EP0 disabled and the host resets the bus.
                        self.driver.request_setup_rearm();
                    }
                }
                _ => {}
            }
        }
        // The controller signals that the next SETUP is expected (at startup, on
        // a bus reset, and whenever a control transfer finished). This must run
        // even when there is no event to report, otherwise EP0 is never armed
        // and the device silently fails to enumerate.
        if self.driver.ep0_needs_rearm() {
            self.arm_ep0_setup();
        }
        event
    }

    /// The controller work the USB interrupt handler has to do.
    ///
    /// Acknowledge the pending endpoint interrupts - that is what deasserts the
    /// controller's level-triggered interrupt line - queue the resulting events
    /// for the task, and re-arm EP0 once a control transfer has finished. Event
    /// *handling* (descriptors, EP0 data stages) deliberately stays in the task;
    /// this is only the part the hardware needs done promptly.
    pub(crate) fn poll_isr(&mut self) {
        self.driver.poll();
        if self.driver.ep0_needs_rearm() {
            self.arm_ep0_setup();
        }
    }

    /// Arm the EP0 SETUP receive.
    fn arm_ep0_setup(&mut self) {
        self.ep0_in = &[];
        self.ep0_in_pos = 0;
        self.ep0_had_data = false;
        if self.driver.start_setup_read(&mut self.setup.0).is_err() {
            self.driver.request_setup_rearm();
        }
    }

    /// Send the next EP0 IN chunk (at most one max packet, as the controller's
    /// EP0 transfer-size register is programmed one packet at a time).
    fn arm_ep0_in(&mut self) {
        let start = self.ep0_in_pos;
        let end = (start + EP0_MPS).min(self.ep0_in.len());
        let chunk = &self.ep0_in[start..end];
        if self.driver.start_write_dma(EP0_IN, chunk).is_ok() {
            self.ep0_in_pos = end;
        }
    }

    fn handle_setup(&mut self) {
        let s = self.setup.0;
        let request_type = s[0];
        let request = s[1];
        let value = u16::from_le_bytes([s[2], s[3]]);
        let length = u16::from_le_bytes([s[6], s[7]]) as usize;
        let direction_in = request_type & 0x80 != 0;
        let standard = request_type & 0x60 == 0;

        // Standard requests only; class/vendor requests are stalled.
        //
        // Requests with a host-to-device data stage are not implemented at all
        // (the upgrade protocol is entirely bulk-based), so they are stalled
        // rather than answered with a bogus status stage.
        let reply: Option<&'static [u8]> = if !standard || !direction_in && length != 0 {
            None
        } else {
            match request {
                // SET_ADDRESS
                5 => {
                    let address = value as u8;
                    let _ = self.driver.set_address(address);
                    Some(&[])
                }
                // SET_CONFIGURATION
                9 => {
                    self.configured = value != 0;
                    Some(&[])
                }
                // GET_DESCRIPTOR
                6 => {
                    let dtype = (value >> 8) as u8;
                    let didx = (value & 0xff) as u8;
                    match dtype {
                        1 => Some(&DEVICE_DESC.0[..]),
                        2 => Some(&CONFIG_DESC.0[..]),
                        3 => match didx {
                            0 => Some(&STR_LANG.0[..]),
                            1 => Some(&STR_MFG.0[..]),
                            2 => Some(&STR_PRODUCT.0[..]),
                            3 => Some(&STR_BUILD.0[..]),
                            _ => None,
                        },
                        // DEVICE_QUALIFIER: only reachable at High Speed.
                        6 => Some(&QUALIFIER_DESC.0[..]),
                        _ => None,
                    }
                }
                // GET_STATUS
                0 => Some(&STATUS_ZERO.0[..]),
                // CLEAR_FEATURE / SET_FEATURE: accept, no data stage.
                1 | 3 => Some(&[]),
                // GET_CONFIGURATION
                8 => Some(if self.configured {
                    &CONFIG_SET.0[..]
                } else {
                    &CONFIG_CLEAR.0[..]
                }),
                // SET_INTERFACE / GET_INTERFACE
                11 => Some(&[]),
                10 => Some(&CONFIG_CLEAR.0[..]),
                _ => None,
            }
        };

        match reply {
            Some(data) => {
                // Host-to-device requests (no data stage) are acknowledged with a
                // zero-length IN as the status stage.
                let count = if direction_in {
                    data.len().min(length)
                } else {
                    0
                };
                self.ep0_in = &data[..count];
                self.ep0_in_pos = 0;
                self.ep0_had_data = count != 0;
                self.arm_ep0_in();
                if direction_in && count != 0 {
                    // The status stage of a control-IN transfer is a zero-length
                    // OUT packet, and EP0 OUT must be armed to accept it — the
                    // controller does not do it on its own. The setup buffer
                    // doubles as scratch, it is not read again for this request.
                    let scratch = &mut self.setup.0[..];
                    let _ = self.driver.start_read_dma(EP0, scratch);
                }
            }
            None => {
                let _ = self.driver.set_stall(EP0_IN);
                self.ep0_in = &[];
                self.ep0_in_pos = 0;
                self.ep0_had_data = false;
                self.driver.request_setup_rearm();
            }
        }
    }
}

/// Bytes transferred per bulk transaction.
const CHUNK: usize = 512;
const BULK_OUT_EP: u8 = 0x02;
const BULK_IN_EP: u8 = 0x81;

/// Woken by [`AsyncUpgHandler`] on every USB device interrupt.
static WAKER: AtomicWaker = AtomicWaker::new();

/// Interrupt-context poll of the driver.
///
/// Monomorphised inside `AsyncAicUpg::<PAD>::new`, the only place the concrete
/// pad type is known; the handler itself only carries an opaque pointer.
type PollFn = unsafe fn(*mut ());

#[derive(Clone, Copy)]
struct Hook {
    state: StatePtr,
    poll: PollFn,
}

/// The driver address the handler carries.
///
/// A raw `*mut ()` is not `Send`/`Sync`, so it is wrapped to let the hook live in
/// a `static`.
#[derive(Clone, Copy)]
struct StatePtr(*mut ());

// Safety: there is one USB device controller on this core. The pointer is only
// dereferenced from the interrupt handler and from the owning task - never from
// another core or thread - and always inside a critical section.
unsafe impl Send for StatePtr {}
unsafe impl Sync for StatePtr {}

/// The installed hook. There is one USB device controller, so one slot.
static HOOK: Mutex<RefCell<Option<Hook>>> = Mutex::new(RefCell::new(None));

/// Fixed storage for the driver value.
///
/// It must live at an address that never changes: the interrupt handler holds a
/// pointer to it for the whole run, so it cannot be a stack local that the
/// caller may move. [`AsyncAicUpg::new`] asserts the value fits before writing.
const STORAGE_SIZE: usize = 1024;
#[repr(C, align(16))]
struct Storage([u8; STORAGE_SIZE]);
static mut STORAGE: Storage = Storage([0; STORAGE_SIZE]);

/// CLIC handler for the USB device interrupt.
///
/// Bind it with `clic_bind_interrupts!`, the vector table setup then happens in
/// the reset code:
///
/// ```ignore
/// clic_bind_interrupts!(struct Irqs {
///     USB_DEV => AsyncUpgHandler;
/// });
/// ```
pub struct AsyncUpgHandler;

impl typelevel::Handler<typelevel::USB_DEV> for AsyncUpgHandler {
    unsafe fn on_interrupt() {
        // Drive the controller here, exactly like the vendor handler and
        // `uart/non_blocking` do: acknowledge the pending endpoint interrupts
        // (which is what deasserts the level-triggered line) and queue the
        // events for the task. Deferring this to the task instead would leave
        // the line asserted, so unmasking the interrupt would re-enter here
        // immediately and the task would never get to run.
        critical_section::with(|cs| match *HOOK.borrow_ref(cs) {
            Some(hook) => unsafe { (hook.poll)(hook.state.0) },
            // Enumeration starts before the hook is installed; keep the source
            // masked until `AsyncAicUpg::new` unmasks it.
            None => typelevel::USB_DEV::disable(),
        });
        typelevel::USB_DEV::clear_pending();
        WAKER.wake();
    }
}

/// Cache-line-aligned DMA buffer.
///
/// The controller DMAs whole cache lines, so a buffer that shares a line with
/// unrelated data would have that data discarded when the range is invalidated.
/// 64 bytes covers both the 32-byte D13x and the 64-byte D21x cache lines.
#[repr(align(64))]
struct DmaBuf<const N: usize>([u8; N]);

static mut CBW_BUF: DmaBuf<CBW_SIZE> = DmaBuf([0; CBW_SIZE]);
static mut CSW_BUF: DmaBuf<CSW_SIZE> = DmaBuf([0; CSW_SIZE]);
static mut OUT_BUF: DmaBuf<CHUNK> = DmaBuf([0; CHUNK]);
static mut IN_BUF: DmaBuf<CHUNK> = DmaBuf([0; CHUNK]);

fn cbw_buf() -> &'static mut [u8] {
    unsafe { &mut (*core::ptr::addr_of_mut!(CBW_BUF)).0 }
}

fn csw_buf() -> &'static mut [u8] {
    unsafe { &mut (*core::ptr::addr_of_mut!(CSW_BUF)).0 }
}

fn out_buf() -> &'static mut [u8] {
    unsafe { &mut (*core::ptr::addr_of_mut!(OUT_BUF)).0 }
}

fn in_buf() -> &'static mut [u8] {
    unsafe { &mut (*core::ptr::addr_of_mut!(IN_BUF)).0 }
}

/// USB transport state machine.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    /// Waiting for a 31-byte CBW.
    Cbw,
    /// Waiting for the next OUT data chunk of a host WRITE.
    OutData,
    /// Waiting for the IN data chunk of a host READ to be sent.
    InData,
    /// Waiting for the CSW to be sent.
    Csw,
}

/// AIC USB upgrade device driven by the USB interrupt.
pub struct AsyncAicUpg<PAD> {
    /// The live [`AicUpg`] inside [`STORAGE`].
    state: *mut AicUpg<'static, PAD>,
}

impl<PAD> AsyncAicUpg<PAD>
where
    PAD: UsbPads<0>,
{
    pub(crate) fn new(driver: UsbDevDriver<'static, PAD>) -> Result<Self, UsbDevError> {
        let upg = AicUpg::new(driver)?;

        // The value is moved into raw storage, so it has to fit.
        assert!(mem::size_of::<AicUpg<'static, PAD>>() <= STORAGE_SIZE);
        assert!(mem::align_of::<AicUpg<'static, PAD>>() <= mem::align_of::<Storage>());

        let storage = unsafe { &mut *core::ptr::addr_of_mut!(STORAGE) };
        let state = storage.0.as_mut_ptr() as *mut AicUpg<'static, PAD>;
        // Safety: `state` is aligned and large enough (asserted above), and this
        // is the only write to `STORAGE`.
        unsafe { state.write(upg) };

        critical_section::with(|cs| {
            // Arm the first EP0 SETUP and install the hook *before* the
            // controller interrupt is unmasked, so no interrupt can arrive with
            // nothing to drive it.
            // Safety: `state` was just written and nothing else references it.
            unsafe { (*state).poll_isr() };
            *HOOK.borrow_ref_mut(cs) = Some(Hook {
                state: StatePtr(state as *mut ()),
                poll: Self::isr_poll,
            });
        });
        typelevel::USB_DEV::enable();

        Ok(Self { state })
    }

    /// Interrupt-context trampoline: recover the driver from the opaque pointer
    /// the handler carries.
    ///
    /// # Safety
    /// `ptr` must be the pointer installed by [`Self::new`].
    unsafe fn isr_poll(ptr: *mut ()) {
        // Safety: see the caller's contract; the address is stable (STORAGE).
        let upg = unsafe { &mut *(ptr as *mut AicUpg<'static, PAD>) };
        upg.poll_isr();
    }

    /// Run `f` on the driver with interrupts disabled, so the handler can never
    /// interleave with it.
    fn with<R>(&mut self, f: impl FnOnce(&mut AicUpg<'static, PAD>) -> R) -> R {
        critical_section::with(|_| {
            // Safety: `self.state` points at the live value in `STORAGE`, and
            // interrupts are off for the duration, so no other reference to it
            // can be created.
            f(unsafe { &mut *self.state })
        })
    }

    /// Drop the USB pull-up so the host sees the device leave the bus.
    ///
    /// Used when the loader decides this is a normal boot and hands over to the
    /// application: leaving the device enumerated would tell the host an upgrade
    /// device is present that nothing will ever answer.
    pub fn disconnect(&mut self) {
        self.with(|upg| upg.driver_mut().set_connected(false));
    }

    /// Wait for the next event the handler queued, or for the caller's deadline.
    ///
    /// The event is checked once more *after* the waker is registered, so one
    /// queued in between is not missed. The deadline is evaluated here, inside the
    /// wait: a pending wait parks `run` at the `await`, so a silent host would
    /// otherwise never let the `expired` check run again.
    ///
    /// Returns `None` once `expired(engine)` reports the session is over.
    async fn wait_event(
        &mut self,
        engine: &Engine<'_>,
        expired: impl Fn(&Engine<'_>) -> bool,
    ) -> Option<UsbDevEvent> {
        poll_fn(|cx| {
            if expired(engine) {
                return Poll::Ready(None);
            }
            if let Some(event) = self.with(|upg| upg.handle_interrupt()) {
                return Poll::Ready(Some(event));
            }
            WAKER.register(cx.waker());
            match self.with(|upg| upg.handle_interrupt()) {
                Some(event) => Poll::Ready(Some(event)),
                // Safety net, same idiom as `uart::non_blocking::flush`: a
                // missed interrupt would otherwise leave the device silent
                // forever, so keep the task coming back to re-poll the
                // controller (which is also what the polling loop this replaced
                // did). The handler still does the hardware work.
                None => {
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            }
        })
        .await
    }

    /// Arm the transfer matching `state`.
    fn arm(&mut self, state: State, out_len: usize, in_len: usize) -> Result<(), UsbDevError> {
        self.with(|upg| match state {
            State::Cbw => upg.receive(cbw_buf()),
            State::OutData => upg.receive(&mut out_buf()[..out_len]),
            State::InData => upg.send(&in_buf()[..in_len]),
            State::Csw => upg.send(csw_buf()),
        })
    }

    /// Run the upgrade command loop until `expired` reports the session is over.
    ///
    /// `expired` is evaluated between iterations and inside the event wait, so a
    /// silent host cannot stop it firing. The policy lives with the caller: a
    /// host-started burn keeps its clock until a state-changing command arrives,
    /// while a flash start's deadline is final. `|_| false` waits forever.
    pub async fn run(&mut self, engine: &mut Engine<'_>, expired: impl Fn(&Engine<'_>) -> bool) {
        let mut state = State::Cbw;
        let mut armed = false;
        let mut tag = 0u32;
        let mut out_remaining = 0usize;
        let mut in_len = 0usize;
        let mut in_remaining = 0u32;
        let mut csw = Csw::default();

        loop {
            if !armed {
                if state == State::Csw {
                    csw_buf().copy_from_slice(&csw.to_bytes());
                }
                match self.arm(state, out_remaining.min(CHUNK), in_len) {
                    Ok(()) => armed = true,
                    // Nothing is queued after a failed arm, so waiting for an
                    // interrupt here would hang: retry instead, as the polling
                    // loop this replaced did. The failure is transient - an
                    // endpoint still holding the previous transfer.
                    Err(err) => {
                        error!("aic_upg: arm({state:?}) failed: {err:?}");
                        continue;
                    }
                }
            }

            // Also checked inside the wait below. Here it covers the moment right
            // after arming, before any interrupt can arrive.
            if expired(engine) {
                return;
            }

            // `wait_event` owns the deadline while it waits: a pending wait parks
            // this loop at the `await`, so this body would not run again until an
            // event arrived.
            let Some(event) = self.wait_event(engine, &expired).await else {
                return;
            };

            // A bus reset aborts every queued transfer, so the armed flag and any
            // half-received frame are stale: restart from the CBW stage.
            match event {
                UsbDevEvent::Reset => {
                    debug!("aic_upg: bus reset, restarting");
                    state = State::Cbw;
                    armed = false;
                    tag = 0;
                    out_remaining = 0;
                    in_len = 0;
                    in_remaining = 0;
                    engine.reset();
                    continue;
                }
                UsbDevEvent::Enumerated(speed) => {
                    debug!("aic_upg: enumerated, speed={speed:?}");
                    continue;
                }
                UsbDevEvent::Error(err) => {
                    error!("aic_upg: USB error {err:?}, restarting");
                    state = State::Cbw;
                    armed = false;
                    out_remaining = 0;
                    in_len = 0;
                    in_remaining = 0;
                    continue;
                }
                _ => {}
            }

            match (state, event) {
                (State::Cbw, UsbDevEvent::OutComplete { ep, .. }) if ep == BULK_OUT_EP => {
                    let Some(cbw) = Cbw::parse(cbw_buf()) else {
                        csw = Csw::reply(tag, RESP_FAIL, 0);
                        state = State::Csw;
                        armed = false;
                        continue;
                    };
                    tag = cbw.tag;
                    match cbw.command {
                        TRANSPORT_WRITE => {
                            out_remaining = cbw.data_len as usize;
                            if out_remaining == 0 {
                                let status = engine.transport_write(&[]);
                                csw = Csw::reply(tag, status, 0);
                                state = State::Csw;
                            } else {
                                state = State::OutData;
                            }
                        }
                        TRANSPORT_READ => {
                            in_remaining = cbw.data_len;
                            let want = (in_remaining as usize).min(CHUNK);
                            let n = engine.transport_read(&mut in_buf()[..want]);
                            in_remaining -= n as u32;
                            in_len = n;
                            csw = Csw::reply(tag, RESP_OK, in_remaining);
                            // The response data has to go out *before* the CSW:
                            // `InData` is the arm that transmits `in_buf`, and the
                            // CSW is only queued once `in_remaining` has drained
                            // to zero. Going straight to `Csw` whenever the whole
                            // response fits in one chunk would drop the payload
                            // and send the host a bare CSW where it is waiting for
                            // data.
                            state = State::InData;
                        }
                        _ => {
                            error!("aic_upg: unsupported transport cmd 0x{:02x}", cbw.command);
                            csw = Csw::reply(tag, RESP_FAIL, cbw.data_len);
                            state = State::Csw;
                        }
                    }
                    armed = false;
                }
                (State::OutData, UsbDevEvent::OutComplete { ep, .. }) if ep == BULK_OUT_EP => {
                    let len = out_remaining.min(CHUNK);
                    let status = engine.transport_write(&out_buf()[..len]);
                    out_remaining -= len;
                    if out_remaining == 0 {
                        csw = Csw::reply(tag, status, 0);
                        state = State::Csw;
                    }
                    armed = false;
                }
                (State::InData, UsbDevEvent::InComplete { ep, .. }) if ep == BULK_IN_EP => {
                    // Continue draining the response so a reply larger than one
                    // buffer is not truncated at the first chunk.
                    let want = (in_remaining as usize).min(CHUNK);
                    if want == 0 {
                        csw = Csw::reply(tag, RESP_OK, 0);
                        state = State::Csw;
                    } else {
                        let n = engine.transport_read(&mut in_buf()[..want]);
                        in_remaining -= n as u32;
                        in_len = n;
                        csw = Csw::reply(tag, RESP_OK, in_remaining);
                        state = if in_remaining == 0 {
                            State::Csw
                        } else {
                            State::InData
                        };
                    }
                    armed = false;
                }
                (State::Csw, UsbDevEvent::InComplete { ep, .. }) if ep == BULK_IN_EP => {
                    state = State::Cbw;
                    armed = false;
                    // The reply is out, so a pending hand-over may proceed: the
                    // host asked us to reset, which means it is done with us.
                    if engine.reset_requested() {
                        return;
                    }
                }
                _ => {}
            }
        }
    }
}
