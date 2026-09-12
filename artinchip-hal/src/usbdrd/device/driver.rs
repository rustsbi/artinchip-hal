//! USB device controller driver.
//!
//! Register-level engine: controller/PHY bring-up, endpoint configuration, and
//! the DMA transfer/event layer the upgrade device drives.

use super::dev_register::*;
use super::error::UsbDevError;
use crate::cache;
use crate::cmu::Cmu;
use crate::sys_cfg::{DrdMode, SysCfg};
use crate::usbdrd::{
    UsbConfig, UsbDev, UsbDmaBurstLength, UsbPads, UsbPhyInterface, UsbUtmiInterfaceWidth,
};

const EP_COUNT: usize = 5;
const XFER_MASK: u32 = 0x7ffff;

/// A completion or bus event reported by [`UsbDevDriver::handle_interrupt`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsbDevEvent {
    Reset,
    Enumerated(UsbDevSpeed),
    Suspend,
    Resume,
    Setup,
    InComplete { ep: u8, len: usize },
    OutComplete { ep: u8, len: usize },
    Error(UsbDevError),
}

/// USB endpoint descriptor information needed by the controller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsbEndpointConfig {
    /// Endpoint address; bit 7 selects IN, low nibble selects EP0..EP4.
    pub address: u8,
    /// Maximum packet size in bytes.
    pub max_packet_size: u16,
    /// USB transfer type.
    pub ep_type: EndpointType,
}

impl UsbEndpointConfig {
    fn index(self) -> Result<usize, UsbDevError> {
        let idx = (self.address & 0x0f) as usize;
        if idx >= EP_COUNT {
            Err(UsbDevError::InvalidEndpoint)
        } else {
            Ok(idx)
        }
    }
}

/// Initialized USB device controller.
pub struct UsbDevDriver<'a, PAD> {
    reg: &'a RegisterBlock,
    pads: PAD,
    config: UsbConfig,
    tx_len: [usize; EP_COUNT],
    rx_len: [usize; EP_COUNT],
    tx_ptr: [*const u8; EP_COUNT],
    rx_ptr: [*mut u8; EP_COUNT],
    /// Negotiated max packet size per endpoint, used for the DMA packet count.
    mps: [u16; EP_COUNT],
    /// Vendor `np_txfifo_map`: enabled non-periodic IN endpoints.
    np_txfifo_map: u16,
    /// Set when EP0 is idle and waiting for the application to arm the next
    /// SETUP receive (a bus reset or a completed control transfer).
    ep0_needs_rearm: bool,
    /// Pending events.
    ///
    /// A single hardware interrupt can report several events at once (a bus
    /// reset together with a SETUP, an OUT completion together with the next
    /// SETUP...). Returning only one of them would silently drop the others,
    /// because `USB_INT_STS` is write-1-to-clear, so they are queued here.
    events: [Option<UsbDevEvent>; EVENT_QUEUE_LEN],
    event_len: usize,
}

const EVENT_QUEUE_LEN: usize = 8;

impl<'a, PAD> UsbDevDriver<'a, PAD>
where
    PAD: UsbPads<0>,
{
    /// Initialize clocks, PHY, controller FIFOs and DMA/interrupt engines.
    ///
    /// Mirrors the vendor `usb_dc_low_level_init()` + `usb_dc_init()` sequence.
    pub fn new(
        reg: &'a RegisterBlock,
        pads: PAD,
        config: UsbConfig,
        cmu: &mut Cmu,
        syscfg: &mut SysCfg,
    ) -> Result<Self, UsbDevError> {
        initialize(reg, cmu, syscfg, config)?;

        let mut this = Self {
            reg,
            pads,
            config,
            tx_len: [0; EP_COUNT],
            rx_len: [0; EP_COUNT],
            tx_ptr: [core::ptr::null(); EP_COUNT],
            rx_ptr: [core::ptr::null_mut(); EP_COUNT],
            mps: [0; EP_COUNT],
            np_txfifo_map: 0,
            ep0_needs_rearm: true,
            events: [None; EVENT_QUEUE_LEN],
            event_len: 0,
        };
        this.reset_endpoints()?;
        Ok(this)
    }

    pub const fn register_block(&self) -> &'a RegisterBlock {
        self.reg
    }

    pub const fn config(&self) -> UsbConfig {
        self.config
    }

    pub fn free(self) -> (UsbDev, PAD) {
        (UsbDev::__new(self.reg), self.pads)
    }

    /// Connect or disconnect the pull-up resistor.
    pub fn set_connected(&mut self, connected: bool) {
        unsafe {
            self.reg.usb_dev_func.modify(|v| {
                v.set_connection(if connected {
                    UsbDeviceConnection::Connected
                } else {
                    UsbDeviceConnection::Disconnected
                })
            });
        }
    }

    pub fn set_address(&mut self, address: u8) -> Result<(), UsbDevError> {
        unsafe {
            if address > 127 {
                return Err(UsbDevError::InvalidEndpoint);
            }
            self.reg.usb_dev_conf.modify(|v| v.set_dev_addr(address));
            Ok(())
        }
    }

    pub fn speed(&self) -> UsbDevSpeed {
        self.reg.usb_line_sts.read().enum_speed()
    }

    /// Vendor `usb_dc_rst()`: reset the controller back to a just-enumerated
    /// state and let the application re-arm EP0.
    ///
    /// Called at init time and on every bus reset. The endpoint configurations
    /// are *not* re-opened here — [`Self::open_endpoint`] does that, because a bus
    /// reset also drops `USB_EP_INT_MSK`.
    pub fn reset_endpoints(&mut self) -> Result<(), UsbDevError> {
        reset_controller(self.reg);

        for i in 0..EP_COUNT {
            self.tx_len[i] = 0;
            self.rx_len[i] = 0;
            self.mps[i] = 0;
        }
        self.np_txfifo_map = 0;
        self.ep0_needs_rearm = true;
        self.event_len = 0;
        Ok(())
    }

    /// `true` while the controller is waiting for the application to re-arm the
    /// EP0 SETUP receive via [`Self::start_setup_read`].
    pub const fn ep0_needs_rearm(&self) -> bool {
        self.ep0_needs_rearm
    }

    /// Arm the EP0 SETUP receive, exactly as the vendor
    /// `aic_ep0_start_read_setup()` does.
    ///
    /// This differs from a plain [`Self::start_read_dma`] call: the controller
    /// expects a three-packet, 24-byte transfer descriptor for the SETUP stage,
    /// and the pending EP0 interrupt has to be acknowledged before the endpoint
    /// is re-enabled.
    pub fn start_setup_read(&mut self, data: &mut [u8; 8]) -> Result<(), UsbDevError> {
        unsafe {
            let ptr = data.as_mut_ptr();
            if (ptr as usize & 3) != 0 {
                return Err(UsbDevError::InvalidDmaBuffer);
            }
            crate::cache::dcache_invalidate_range(ptr as usize, 8);
            // The transfer descriptor is three 8-byte packets; the hardware
            // reports the eight bytes actually received as
            // `armed - remaining`, so the armed length has to be recorded.
            self.rx_len[0] = 24;
            self.rx_ptr[0] = ptr;
            self.reg.out_ep_tsf_siz[0]
                .modify(|v| v.zero().set_sup_pkt_cnt(1).set_pkt_cnt(3).set_xfr_siz(24));
            self.reg.out_ep_dma[0].write(ptr as u32);
            self.reg.out_ep_int[0].modify(|v| v.clear_all());
            self.reg.out_ep_cfg[0].modify(|v| {
                v.clear_ep_disable()
                    .clear_nack(true)
                    .enable_ep()
                    .activate_ep()
            });
            self.ep0_needs_rearm = false;
            Ok(())
        }
    }

    /// Mark EP0 as idle so the application arms the next SETUP receive.
    pub fn request_setup_rearm(&mut self) {
        self.ep0_needs_rearm = true;
    }

    pub fn open_endpoint(&mut self, ep: UsbEndpointConfig) -> Result<(), UsbDevError> {
        unsafe {
            let idx = ep.index()?;
            if ep.max_packet_size == 0 || ep.max_packet_size > 1024 {
                return Err(UsbDevError::InvalidMaxPacketSize);
            }
            self.mps[idx] = ep.max_packet_size;
            self.reg.usb_ep_int_msk.modify(|v| {
                if ep.address & 0x80 != 0 {
                    v.set_in_ep_mask(v.in_ep_mask() | (1 << idx))
                } else {
                    v.set_out_ep_mask(v.out_ep_mask() | (1 << idx))
                }
            });
            if ep.address & 0x80 != 0 {
                // EP0 encodes its max packet size as a 2-bit value; every other
                // endpoint uses the plain byte count.
                let mps = if idx == 0 {
                    match ep.max_packet_size {
                        64 => 0,
                        32 => 1,
                        16 => 2,
                        _ => 3,
                    }
                } else {
                    ep.max_packet_size
                };
                // Dedicated non-periodic TXFIFO for this endpoint. `EPDIS` is
                // cleared explicitly: it must never be left pending alongside
                // the `EPENA` that `start_dma` writes, or the endpoint will not
                // transmit.
                self.reg.in_ep_cfg[idx].modify(|v| {
                    v.clear_ep_disable()
                        .set_mps(mps)
                        .set_ep_type(ep.ep_type)
                        .set_tx_fifo_num(TxFifoSelect::NonPeriodic)
                        .set_data0_pid(true)
                        .activate_ep()
                });
                self.np_txfifo_map |= 1 << idx;
                // The internal DMA walks the IN endpoints in NXT_TX_EP order.
                set_dma_next_ep(self.reg, self.np_txfifo_map);
            } else {
                let mps = if idx == 0 {
                    match ep.max_packet_size {
                        64 => 0,
                        32 => 1,
                        16 => 2,
                        _ => 3,
                    }
                } else {
                    ep.max_packet_size
                };
                self.reg.out_ep_cfg[idx].modify(|v| {
                    v.clear_ep_disable()
                        .set_mps(mps)
                        .set_ep_type(ep.ep_type)
                        .set_data0_pid(true)
                        .activate_ep()
                });
            }
            Ok(())
        }
    }

    pub fn close_endpoint(&mut self, address: u8) -> Result<(), UsbDevError> {
        unsafe {
            let idx = (address & 0xf) as usize;
            if idx >= EP_COUNT {
                return Err(UsbDevError::InvalidEndpoint);
            }
            self.reg.usb_ep_int_msk.modify(|v| {
                if address & 0x80 != 0 {
                    v.set_in_ep_mask(v.in_ep_mask() & !(1 << idx))
                } else {
                    v.set_out_ep_mask(v.out_ep_mask() & !(1 << idx))
                }
            });
            if address & 0x80 != 0 {
                self.reg.in_ep_cfg[idx].modify(|v| v.disable_ep());
            } else {
                self.reg.out_ep_cfg[idx].modify(|v| v.disable_ep());
            }
            Ok(())
        }
    }

    pub fn set_stall(&mut self, address: u8) -> Result<(), UsbDevError> {
        self.stall(address, true)
    }

    pub fn clear_stall(&mut self, address: u8) -> Result<(), UsbDevError> {
        self.stall(address, false)
    }

    fn stall(&mut self, address: u8, set: bool) -> Result<(), UsbDevError> {
        unsafe {
            let idx = (address & 0xf) as usize;
            if idx >= EP_COUNT {
                return Err(UsbDevError::InvalidEndpoint);
            }
            if address & 0x80 != 0 {
                self.reg.in_ep_cfg[idx].modify(|v| {
                    if set {
                        v.enable_stall()
                    } else {
                        v.disable_stall()
                    }
                });
            } else {
                self.reg.out_ep_cfg[idx].modify(|v| {
                    if set {
                        v.enable_stall()
                    } else {
                        v.disable_stall()
                    }
                });
            }
            Ok(())
        }
    }

    /// Start a controller DMA transfer. Buffers must remain valid until completion.
    pub fn start_write_dma(&mut self, address: u8, data: &[u8]) -> Result<(), UsbDevError> {
        self.start_dma(address, data.as_ptr() as *mut u8, data.len(), true)
    }

    /// Start a controller DMA receive transfer. Buffers must remain valid until completion.
    pub fn start_read_dma(&mut self, address: u8, data: &mut [u8]) -> Result<(), UsbDevError> {
        self.start_dma(address, data.as_mut_ptr(), data.len(), false)
    }

    fn start_dma(
        &mut self,
        address: u8,
        ptr: *mut u8,
        len: usize,
        write: bool,
    ) -> Result<(), UsbDevError> {
        unsafe {
            let idx = (address & 0xf) as usize;
            if idx >= EP_COUNT || address & 0x80 == 0 && write || address & 0x80 != 0 && !write {
                return Err(UsbDevError::InvalidEndpoint);
            }
            if len != 0 && (ptr as usize & 3) != 0 {
                return Err(UsbDevError::InvalidDmaBuffer);
            }
            if len > XFER_MASK as usize {
                return Err(UsbDevError::TransferTooLong);
            }
            // A zero-length OUT transfer still consumes a packet's worth of
            // descriptor space, so it is armed with the max packet size. A
            // zero-length IN transfer is the opposite: it must be armed with a
            // transfer size of *zero*, otherwise the controller waits for data
            // that will never come and the status stage never completes (the
            // host then gives up and resets the bus).
            let mps = self.mps[idx].max(1) as usize;
            if write {
                self.tx_len[idx] = len;
                self.tx_ptr[idx] = ptr;
                let (pkt_count, size) = if idx == 0 {
                    // EP0 is programmed one control packet at a time.
                    (1, len.min(mps))
                } else {
                    (len.div_ceil(mps).max(1) as u16, len)
                };
                if len != 0 {
                    cache::dcache_clean_invalidate_range(ptr as usize, len);
                }
                self.reg.in_ep_tsf_siz[idx]
                    .modify(|v| v.set_pkt_cnt(pkt_count).set_xfr_siz(size as u32));
                // A zero-length transfer has no buffer to point at; the vendor
                // likewise leaves the address register alone when `data` is
                // null.
                if len != 0 {
                    self.reg.in_ep_dma[idx].write(ptr as u32);
                }
                // Acknowledge any stale endpoint interrupt before re-enabling.
                self.reg.in_ep_int[idx].modify(|v| v.clear_all());
                self.reg.in_ep_cfg[idx]
                    .modify(|v| v.clear_ep_disable().clear_nack(true).enable_ep());
            } else {
                self.rx_len[idx] = len;
                self.rx_ptr[idx] = ptr;
                let (pkt_count, size) = if idx == 0 {
                    let size = if len == 0 { mps } else { len.min(mps) };
                    (1, size)
                } else if len == 0 {
                    (1, mps)
                } else {
                    (len.div_ceil(mps) as u16, len)
                };
                if len != 0 {
                    cache::dcache_invalidate_range(ptr as usize, len);
                }
                self.reg.out_ep_tsf_siz[idx].modify(|v| {
                    let v = v.set_pkt_cnt(pkt_count).set_xfr_siz(size as u32);
                    if idx == 0 { v.set_sup_pkt_cnt(1) } else { v }
                });
                if len != 0 {
                    self.reg.out_ep_dma[idx].write(ptr as u32);
                }
                self.reg.out_ep_int[idx].modify(|v| v.clear_all());
                self.reg.out_ep_cfg[idx]
                    .modify(|v| v.clear_ep_disable().clear_nack(true).enable_ep());
            }
            Ok(())
        }
    }

    /// Queue one event for the application to pick up.
    fn push_event(&mut self, event: UsbDevEvent) {
        if self.event_len < EVENT_QUEUE_LEN {
            self.events[self.event_len] = Some(event);
            self.event_len += 1;
        }
    }

    fn pop_event(&mut self) -> Option<UsbDevEvent> {
        if self.event_len == 0 {
            return None;
        }
        let event = self.events[0].take()?;
        self.events.copy_within(1..self.event_len, 0);
        self.event_len -= 1;
        Some(event)
    }

    /// Vendor `aic_set_turnaroundtime()`: the USB turnaround time depends on the
    /// AHB clock, and it is only known once the device speed is known.
    fn set_turnaround_time(&self, clock_hz: u32) {
        let trd = match self.speed() {
            UsbDevSpeed::High => 9,
            UsbDevSpeed::Full => {
                if clock_hz >= 32_000_000 {
                    6
                } else if clock_hz >= 27_700_000 {
                    7
                } else if clock_hz >= 24_000_000 {
                    8
                } else if clock_hz >= 21_800_000 {
                    9
                } else if clock_hz >= 20_000_000 {
                    10
                } else if clock_hz >= 18_500_000 {
                    11
                } else if clock_hz >= 17_200_000 {
                    12
                } else if clock_hz >= 16_000_000 {
                    13
                } else if clock_hz >= 15_000_000 {
                    14
                } else {
                    15
                }
            }
        };
        unsafe {
            self.reg
                .usb_phy_if
                .modify(|v| v.set_timeout_cal(7).set_ta_time(trd));
        }
    }

    /// Poll the controller and return the next pending event.
    ///
    /// All events visible in one pass are queued, because `USB_INT_STS` is
    /// write-1-to-clear: acknowledging a bit for an event the caller cannot
    /// report yet would lose it forever.
    pub fn handle_interrupt(&mut self) -> Option<UsbDevEvent> {
        if self.event_len == 0 {
            self.poll();
        }
        self.pop_event()
    }

    /// Read the controller status, acknowledge every pending endpoint interrupt
    /// and queue the resulting events.
    ///
    /// Split out of [`Self::handle_interrupt`] so the interrupt handler can run
    /// just this part: acknowledging here is what deasserts the controller's
    /// level-triggered interrupt line, while event *handling* stays with the
    /// caller that pops from the queue.
    ///
    /// Every enabled global source is acknowledged here, including the two
    /// isochronous bits this device never otherwise services; a single
    /// unacknowledged enabled bit would keep the level-triggered line asserted.
    pub(crate) fn poll(&mut self) {
        unsafe {
            let status = self.reg.usb_int_sts.read();
            let mask = self.reg.usb_int_msk.read();

            // OUT endpoints first, matching the vendor interrupt order.
            if status.is_out_ep_int() && mask.is_out_ep_int_enabled() {
                let pending = self.reg.usb_ep_int.read().out_ep_int();
                let enable = self.reg.usb_ep_int_msk.read().out_ep_mask();
                for idx in 0..EP_COUNT {
                    if pending & (1 << idx) == 0 || enable & (1 << idx) == 0 {
                        continue;
                    }
                    // Acknowledge the whole pending set; masked-off sources
                    // have no handler.
                    let epint = self.reg.out_ep_int[idx].read();
                    self.reg.out_ep_int[idx].write(epint);
                    if idx == 0 {
                        self.handle_ep0_out(epint);
                    } else if epint.is_rx_comp_int() {
                        // Report what the host actually sent, not what was armed:
                        // the hardware leaves the un-received byte count in the
                        // transfer-size register.
                        let armed = self.rx_len[idx];
                        let remaining = self.reg.out_ep_tsf_siz[idx].read().xfr_siz() as usize;
                        let actual = armed.saturating_sub(remaining);
                        self.rx_len[idx] = 0;
                        if armed != 0 {
                            cache::dcache_invalidate_range(self.rx_ptr[idx] as usize, armed);
                        }
                        if actual != 0 {
                            self.push_event(UsbDevEvent::OutComplete {
                                ep: idx as u8,
                                len: actual,
                            });
                        }
                    }
                }
                // `USB_INT_STS.OUT_EP_INT` is a level bit that clears itself once
                // every endpoint interrupt has been acknowledged, so it must not
                // be written back (it is not write-1-to-clear).
            }

            if status.is_in_ep_int() && mask.is_in_ep_int_enabled() {
                let pending = self.reg.usb_ep_int.read().in_ep_int();
                let enable = self.reg.usb_ep_int_msk.read().in_ep_mask();
                for idx in 0..EP_COUNT {
                    if pending & (1 << idx) == 0 || enable & (1 << idx) == 0 {
                        continue;
                    }
                    let epint = self.reg.in_ep_int[idx].read();
                    self.reg.in_ep_int[idx].write(epint);
                    if epint.is_tx_comp_int() {
                        let len = self.tx_len[idx];
                        self.tx_len[idx] = 0;
                        self.push_event(UsbDevEvent::InComplete {
                            ep: (idx as u8) | 0x80,
                            len,
                        });
                    }
                }
                // `IN_EP_INT` is level-triggered as well: drained above already.
            }

            if status.is_usb_reset() && mask.is_usb_reset_enabled() {
                // Only the reset bit may be written back, otherwise every other
                // pending interrupt is lost to the write-1-to-clear semantics.
                self.reg.usb_int_sts.modify(|v| v.zero().clear_usb_reset());
                // The core may have re-armed the remote-wakeup signalling.
                self.reg.usb_dev_func.modify(|v| v.disable_remote_wakeup());
                let _ = self.reset_endpoints();
                self.push_event(UsbDevEvent::Reset);
            }

            if status.is_enum_done() && mask.is_enum_done_enabled() {
                self.reg.usb_int_sts.modify(|v| v.zero().clear_enum_done());
                // The turnaround time is only meaningful once the speed is
                // known, and the global non-periodic IN NAK has to be released
                // or the device can never answer an IN token.
                self.set_turnaround_time(self.config.ahb_clock_hz);
                self.reg.usb_dev_func.modify(|v| v.clear_np_in_nack());
                self.push_event(UsbDevEvent::Enumerated(self.speed()));
            }

            // These two sources are enabled at init but have no endpoint to
            // service (this device has no isochronous transfers). They still
            // must be acknowledged like every other enabled bit: USB_INT_STS is
            // write-1-to-clear and the controller's interrupt line is level
            // triggered, so a single unacknowledged bit keeps the line asserted
            // and the handler re-enters forever. The vendor ISR clears both
            // unconditionally, and so does this one.
            if status.is_incomp_iso_out_int() && mask.is_incomp_iso_out_int_enabled() {
                self.reg
                    .usb_int_sts
                    .modify(|v| v.zero().clear_incomp_iso_out_int());
            }
            if status.is_incomp_iso_in_int() && mask.is_incomp_iso_in_int_enabled() {
                self.reg
                    .usb_int_sts
                    .modify(|v| v.zero().clear_incomp_iso_in_int());
            }

            if status.is_usb_sus() && mask.is_usb_sus_enabled() {
                self.reg.usb_int_sts.modify(|v| v.zero().clear_usb_sus());
                self.push_event(UsbDevEvent::Suspend);
            }
            if status.is_wakeup_int() && mask.is_wakeup_int_enabled() {
                self.reg.usb_int_sts.modify(|v| v.zero().clear_wakeup_int());
                self.push_event(UsbDevEvent::Resume);
            }
        }
    }

    /// Vendor `usbd_irq_handler()` EP0 branch.
    ///
    /// A control transfer is delimited by `SETUP_PHASE_DONE` (a SETUP packet was
    /// DMA'd into the setup buffer) and by later OUT-side activity. The number of
    /// bytes actually received is `armed - remaining`, the same accounting the
    /// vendor uses, because EP0's status stage is a zero-length OUT packet that
    /// must not be mistaken for a data stage.
    unsafe fn handle_ep0_out(&mut self, epint: OutEpInt) {
        unsafe {
            if epint.is_setup_done_int() {
                // The controller only raises this after it has DMA'd a real
                // SETUP packet, so it is safe to surface unconditionally.
                self.rx_len[0] = 0;
                self.push_event(UsbDevEvent::Setup);
                return;
            }
            if !(epint.is_rx_comp_int() || epint.is_sts_phase_rx_int()) {
                return;
            }
            let remaining = self.reg.out_ep_tsf_siz[0].read().xfr_siz() as usize;
            let actual = self.rx_len[0].saturating_sub(remaining);
            let ptr = self.rx_ptr[0];
            self.rx_len[0] = 0;
            if actual != 0 {
                cache::dcache_invalidate_range(ptr as usize, actual);
                self.push_event(UsbDevEvent::OutComplete { ep: 0, len: actual });
            } else {
                // Zero-length status stage: the transfer is over.
                self.ep0_needs_rearm = true;
            }
        }
    }
}

/// Let the PHY/controller clocks and resets settle (~hundreds of microseconds).
#[inline]
fn settle() {
    riscv::asm::delay(200_000);
}

/// Perform the controller/PHY initialization removed from [`UsbDevDriver::new`].
///
/// Mirrors the vendor `usb_dc_low_level_init()` + `usb_dc_init()` sequence.
fn initialize(
    reg: &RegisterBlock,
    cmu: &mut Cmu,
    syscfg: &mut SysCfg,
    config: UsbConfig,
) -> Result<(), UsbDevError> {
    unsafe {
        let cmu_reg = cmu.register_block();

        // PHY switch to device mode before the controller is brought up.
        syscfg
            .register_block()
            .usb0_cfg
            .modify(|v| v.set_drd_mode(DrdMode::Device));

        // Enable the PHY and controller clocks.
        cmu_reg.clock_usb_phy0.modify(|v| v.enable_module_clk());
        cmu_reg.clock_usb_dev.modify(|v| v.enable_bus_clk());
        settle();

        // Pulse the PHY and controller resets.
        //
        // This matters when taking over from the BootROM (or from the RAM helper
        // it downloads): their USB engine can still have an endpoint DMA armed,
        // and the core soft reset below never completes while the AHB master is
        // busy. Resetting the whole controller clears that state.
        cmu_reg.clock_usb_phy0.modify(|v| v.enable_module_reset());
        cmu_reg.clock_usb_dev.modify(|v| v.enable_module_reset());
        settle();
        cmu_reg.clock_usb_phy0.modify(|v| v.disable_module_reset());
        cmu_reg.clock_usb_dev.modify(|v| v.disable_module_reset());
        settle();

        reg.usb_dev_func
            .modify(|v| v.set_connection(UsbDeviceConnection::Disconnected));
        reg.usb_dev_init.modify(|v| v.disable_global_interrupt());

        // Vendor `aic_reset()` order: wait for the AHB master to become idle
        // *before* asserting the core soft reset, then wait for it to self-clear.
        let mut count = 0;
        while !reg.ahb_basic.read().is_ahb_idle() {
            count += 1;
            if count > 200_000 {
                return Err(UsbDevError::Timeout);
            }
        }
        reg.usb_dev_init.modify(|v| v.soft_reset());
        count = 0;
        while reg.usb_dev_init.read().is_soft_reset() {
            count += 1;
            if count > 200_000 {
                return Err(UsbDevError::Timeout);
            }
        }

        // The PHY interface has to be programmed *after* the soft reset — the
        // reset would otherwise clear it.
        reg.usb_phy_if.modify(|v| {
            let v = v
                .set_phy_interface(match config.phy_interface {
                    UsbPhyInterface::Utmi => PhyInterface::UtmiPlus,
                    UsbPhyInterface::Ulpi => PhyInterface::Ulpi,
                })
                .set_utmi_phy_interface_width(match config.utmi_interface_width {
                    UsbUtmiInterfaceWidth::Bits8 => UtmiPhyInterfaceWidth::Bits8,
                    UsbUtmiInterfaceWidth::Bits16 => UtmiPhyInterfaceWidth::Bits16,
                })
                .set_timeout_cal(7);
            v.set_ta_time(5)
        });

        // Device configuration: speed, periodic-frame interrupt point and device
        // address. The vendor bootloader is built with `CONFIG_USB_HS` (see
        // `images/rtua.py` in the SDK build output), so the device runs at
        // **High Speed**; the core soft reset above clears this register, so it
        // has to be programmed afterwards.
        reg.usb_dev_conf.modify(|v| {
            v.set_dev_speed(UsbDevSpeed::High)
                .set_per_frame_int(PeriodicFrameInterval::Percent80)
                .set_dev_addr(0)
        });

        // Vendor `aic_core_init()`: the DMA next-endpoint pointers are derived
        // from the endpoint enable map, so they are (re)programmed right after
        // the PHY interface is set up. No IN endpoint is open yet.
        set_dma_next_ep(reg, 0);

        reg.usb_dev_init.modify(|v| v.clear_rxfifo());
        count = 0;
        while reg.usb_dev_init.read().is_rxfifo_flushing() {
            count += 1;
            if count > 200_000 {
                return Err(UsbDevError::Timeout);
            }
        }
        reg.rxfifo_siz.modify(|v| v.set_rxfifo_depth(0x119));
        reg.nptxfifo_siz
            .modify(|v| v.set_start_addr(0x119).set_size(0x100));
        reg.ptxfifo_siz[0].modify(|v| v.set_start_addr(0x219).set_size(0x100));
        reg.ptxfifo_siz[1].modify(|v| v.set_start_addr(0x319).set_size(0xdd));

        // Clear every endpoint interrupt and mask, including USB_EP_INT_MSK,
        // which `usbd_ep_open()` re-populates bit by bit.
        reg.inep_int_msk.modify(|v| v.clear_all());
        reg.outep_int_msk.modify(|v| v.clear_all());
        reg.usb_ep_int_msk.modify(|v| v.clear_all());
        for i in 0..EP_COUNT {
            reg.in_ep_int[i].modify(|v| v.clear_all());
            reg.out_ep_int[i].modify(|v| v.clear_all());
        }

        // Interrupt masks. The exact set matters: `CTRL_OUT_EP_STATUS_PHASE_RCVD`
        // and `NON_ISO_IN_EP_TIMEOUT`/`INTKNEPMIS` are what let the EP0 state
        // machine finish a control transfer and re-arm for the next SETUP.
        reg.outep_int_msk.modify(|v| {
            v.clear_all()
                .enable_rx_comp_int()
                .enable_setup_done_int()
                .enable_sts_phase_rx_int()
        });
        reg.inep_int_msk.modify(|v| {
            v.clear_all()
                .enable_tx_comp_int()
                .enable_time_out_int()
                .enable_intoken_mis_int()
        });

        // Disable every interrupt source, clear anything already pending, then
        // enable the device-mode sources only.
        reg.usb_int_msk.modify(|v| v.clear_all());
        reg.usb_int_sts.modify(|v| v.clear_all());
        reg.usb_int_msk.modify(|v| {
            v.clear_all()
                .enable_out_ep_int()
                .enable_in_ep_int()
                .enable_usb_reset()
                .enable_enum_done()
                .enable_incomp_iso_in_int()
                .enable_incomp_iso_out_int()
                .enable_usb_sus()
                .enable_wakeup_int()
        });

        let mut init =
            reg.usb_dev_init
                .read()
                .set_dma_burst_length(match config.dma_burst_length {
                    UsbDmaBurstLength::Single => DmaBurstLength::Single,
                    UsbDmaBurstLength::Incr => DmaBurstLength::INCR,
                    UsbDmaBurstLength::Incr4 => DmaBurstLength::INCR4,
                    UsbDmaBurstLength::Incr8 => DmaBurstLength::INCR8,
                    UsbDmaBurstLength::Incr16 => DmaBurstLength::INCR16,
                });
        init = if config.enable_dma {
            init.enable_dma()
        } else {
            init.disable_dma()
        };
        init = if config.enable_global_interrupt {
            init.enable_global_interrupt()
        } else {
            init.disable_global_interrupt()
        };
        reg.usb_dev_init.write(init);

        // Vendor `usb_dc_init()` finishes with `usb_dc_rst()`, which arms the
        // first EP0 SETUP receive, and only then enables the global interrupt
        // and connects the pull-up.
        reset_controller(reg);
        reg.usb_dev_init.modify(|v| v.enable_global_interrupt());
        reg.usb_dev_func
            .modify(|v| v.set_connection(UsbDeviceConnection::Connected));
    }
    Ok(())
}

/// Vendor `usb_dc_rst()`: bring every endpoint interrupt register back to a known
/// state so the driver can re-arm EP0.
///
/// This runs both at init time and on every bus reset. Skipping it leaves the
/// RX/TX FIFOs unflushed and the per-direction endpoint interrupt masks stale,
/// after which the device silently stops seeing control transfers. The caller is
/// responsible for re-arming the EP0 SETUP receive afterwards.
fn reset_controller(reg: &RegisterBlock) {
    unsafe {
        reg.usb_dev_conf.modify(|v| v.set_dev_addr(0));

        flush_txfifo(reg);
        flush_rxfifo(reg);

        // EP0 is re-armed below; every other endpoint is closed. The vendor
        // writes a bare `SNAK` into the EP0 config registers and zeroes the
        // other endpoints' config registers, which also wipes the max packet
        // size/type fields; `open_endpoint()` programs those.
        //
        // NOTE: `disable_ep()` must *not* be used here. It sets `EPDIS`, and
        // `open_endpoint()`/`start_dma()` later set `EPENA` on the same
        // register without clearing it — `EPENA` together with a pending `EPDIS`
        // is undefined, and on this controller it makes an IN endpoint silently
        // never transmit (bulk OUT keeps working, so the failure looks like
        // ``the CSW is never sent'').
        reg.in_ep_cfg[0].modify(|v| v.set_nack(true));
        reg.out_ep_cfg[0].modify(|v| v.set_nack(true));
        for i in 1..EP_COUNT {
            reg.in_ep_cfg[i].modify(|v| v.zero());
            reg.out_ep_cfg[i].modify(|v| v.zero());
        }
        for i in 0..EP_COUNT {
            reg.in_ep_tsf_siz[i].modify(|v| v.zero());
            reg.in_ep_int[i].modify(|v| v.clear_all());
            reg.out_ep_tsf_siz[i].modify(|v| v.zero());
            reg.out_ep_int[i].modify(|v| v.clear_all());
        }

        // The per-endpoint interrupt enable map is rebuilt by `open_endpoint()`,
        // but the per-direction masks must be restored here.
        reg.usb_ep_int_msk.modify(|v| v.clear_all());
        reg.outep_int_msk.modify(|v| {
            v.clear_all()
                .enable_rx_comp_int()
                .enable_setup_done_int()
                .enable_sts_phase_rx_int()
        });
        reg.inep_int_msk.modify(|v| {
            v.clear_all()
                .enable_tx_comp_int()
                .enable_time_out_int()
                .enable_intoken_mis_int()
        });

        set_dma_next_ep(reg, 0);
    }
}

/// Vendor `aic_set_dma_nextep()`: program the internal DMA next-endpoint chain.
///
/// `map` is the vendor's `np_txfifo_map` — the set of enabled non-periodic IN
/// endpoints. The DMA walks them in index order, so each one points at the next
/// enabled endpoint with a higher index.
fn set_dma_next_ep(reg: &RegisterBlock, map: u16) {
    unsafe {
        for (i, cfg) in reg.in_ep_cfg.iter().enumerate() {
            let mut next = 0u8;
            for j in (i + 1)..EP_COUNT {
                if map & (1 << j) == 0 {
                    continue;
                }
                next = j as u8;
                break;
            }
            cfg.modify(|v| v.set_next_tx_ep(next));
        }
    }
}

/// Vendor `aic_flush_txfifo(0x10)` — flush every TX FIFO.
fn flush_txfifo(reg: &RegisterBlock) {
    unsafe {
        reg.usb_dev_init
            .modify(|v| v.set_clear_txfifo_number(TxFifoToClear::All).clear_txfifo());
        for _ in 0..200_000 {
            if !reg.usb_dev_init.read().is_txfifo_flushing() {
                return;
            }
        }
    }
}

/// Vendor `aic_flush_rxfifo()`.
fn flush_rxfifo(reg: &RegisterBlock) {
    unsafe {
        reg.usb_dev_init.modify(|v| v.clear_rxfifo());
        for _ in 0..200_000 {
            if !reg.usb_dev_init.read().is_rxfifo_flushing() {
                return;
            }
        }
    }
}
