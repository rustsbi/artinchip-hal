//! USB device controller driver.
//!
//! Register-level engine: controller/PHY bring-up, endpoint configuration, and
//! the DMA transfer/event layer the upgrade device drives.

use super::dev_register::*;
use super::error::UsbDevError;
use super::instance::UsbDev;
use crate::cache;
use crate::cmu::Cmu;
use crate::sys_cfg::{DrdMode, SocSysCfg};
use crate::usbdrd::{
    UsbConfig, UsbDmaBurstLength, UsbPads, UsbPhyInterface, UsbUtmiInterfaceWidth,
};

pub(crate) const EP_COUNT: usize = 5;
const XFER_MASK: u32 = 0x7ffff;
const EVENT_QUEUE_LEN: usize = 8;

/// Startup behavior when initializing [`UsbDevDriver`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsbDevOptions {
    /// Drive the D+ pull-up immediately after initialization.
    pub connect: bool,
    /// Unmask the controller's global interrupt output immediately after initialization.
    pub unmask_global_interrupt: bool,
}

impl Default for UsbDevOptions {
    fn default() -> Self {
        Self::CONNECTED
    }
}

impl UsbDevOptions {
    /// Connect D+ and unmask the global interrupt during `new`.
    pub const CONNECTED: Self = Self {
        connect: true,
        unmask_global_interrupt: true,
    };

    /// Keep D+ disconnected and global interrupt masked until the async stack starts.
    pub const DEFERRED: Self = Self {
        connect: false,
        unmask_global_interrupt: false,
    };
}

/// Completion or bus state event reported by [`UsbDevDriver::handle_interrupt`].
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

/// USB endpoint parameters programmed into the controller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsbEndpointConfig {
    /// Endpoint address; bit 7 selects IN, low nibble selects `EP0..EP4`.
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

/// Non-generic register and DMA bookkeeping state of the USB device controller.
pub(crate) struct ControllerCore<'a> {
    pub(crate) reg: &'a RegisterBlock,
    config: UsbConfig,
    tx_len: [usize; EP_COUNT],
    rx_len: [usize; EP_COUNT],
    tx_ptr: [*const u8; EP_COUNT],
    rx_ptr: [*mut u8; EP_COUNT],
    /// Negotiated max packet size per endpoint, used for the DMA packet count.
    mps: [u16; EP_COUNT],
    /// Vendor `np_txfifo_map`: enabled non-periodic IN endpoints.
    np_txfifo_map: u16,
    /// Pending events.
    ///
    /// One hardware interrupt can report several events at once (a bus reset
    /// together with a SETUP, an OUT completion together with the next SETUP).
    /// `USB_INT_STS` is write-1-to-clear, so queue them here instead of dropping
    /// all but the first.
    events: [Option<UsbDevEvent>; EVENT_QUEUE_LEN],
    event_len: usize,
}

unsafe impl Send for ControllerCore<'_> {}

/// Initialized USB device controller driver.
pub struct UsbDevDriver<'a, PAD> {
    pub(crate) core: ControllerCore<'a>,
    pub(crate) pads: PAD,
}

impl<'a, PAD> UsbDevDriver<'a, PAD>
where
    PAD: UsbPads<0>,
{
    /// Initialize clocks, PHY, FIFOs, and DMA/interrupt engines, then connect D+.
    pub fn new(
        reg: &'a RegisterBlock,
        pads: PAD,
        config: UsbConfig,
        cmu: &mut Cmu,
        syscfg: &mut SocSysCfg,
    ) -> Result<Self, UsbDevError> {
        Self::new_with_options(reg, pads, config, UsbDevOptions::CONNECTED, cmu, syscfg)
    }

    /// Initialize the controller leaving D+ disconnected and global interrupts masked.
    pub fn new_deferred(
        reg: &'a RegisterBlock,
        pads: PAD,
        config: UsbConfig,
        cmu: &mut Cmu,
        syscfg: &mut SocSysCfg,
    ) -> Result<Self, UsbDevError> {
        Self::new_with_options(reg, pads, config, UsbDevOptions::DEFERRED, cmu, syscfg)
    }

    /// Initialize the controller with explicit [`UsbDevOptions`].
    pub fn new_with_options(
        reg: &'a RegisterBlock,
        pads: PAD,
        config: UsbConfig,
        options: UsbDevOptions,
        cmu: &mut Cmu,
        syscfg: &mut SocSysCfg,
    ) -> Result<Self, UsbDevError> {
        let core = ControllerCore::new(reg, config, options, cmu, syscfg)?;
        Ok(Self { core, pads })
    }

    /// Return a shared reference to the underlying register block.
    pub const fn register_block(&self) -> &RegisterBlock {
        self.core.reg
    }

    /// Return the controller configuration.
    pub const fn config(&self) -> UsbConfig {
        self.core.config
    }

    /// Release the peripheral instance and USB pads without resetting hardware.
    pub fn free(self) -> (UsbDev, PAD) {
        (
            UsbDev::__new(self.core.reg as *const RegisterBlock),
            self.pads,
        )
    }

    /// Mute the controller, assert module/PHY resets, and gate their clocks.
    pub fn deinit(mut self, cmu: &mut Cmu) {
        self.core.mute_controller();
        let (mut usb, _pads) = self.free();
        usb.release(cmu);
    }

    /// Connect or disconnect the D+ pull-up resistor.
    pub fn set_connected(&mut self, connected: bool) {
        self.core.set_connected(connected);
    }

    /// Unmask the controller's global interrupt output.
    pub fn enable_global_interrupt(&mut self) {
        self.core.enable_global_interrupt();
    }

    /// Mask the controller's global interrupt output.
    pub fn disable_global_interrupt(&mut self) {
        self.core.disable_global_interrupt();
    }

    /// Program the USB device address (`0..=127`).
    pub fn set_address(&mut self, address: u8) -> Result<(), UsbDevError> {
        self.core.set_address(address)
    }

    /// Read the negotiated bus speed.
    pub fn speed(&self) -> UsbDevSpeed {
        self.core.speed()
    }

    /// Reset all endpoints back to post-enumeration defaults.
    pub fn reset_endpoints(&mut self) -> Result<(), UsbDevError> {
        self.core.reset_endpoints()
    }

    /// Arm the 24-byte `EP0` `SETUP` DMA receive descriptor.
    pub fn start_setup_read(&mut self, data: &mut [u8; 24]) -> Result<(), UsbDevError> {
        self.core.start_setup_read(data)
    }

    /// Enable and configure an IN or OUT endpoint.
    pub fn open_endpoint(&mut self, ep: UsbEndpointConfig) -> Result<(), UsbDevError> {
        self.core.open_endpoint(ep)
    }

    /// Disable an IN or OUT endpoint.
    pub fn close_endpoint(&mut self, address: u8) -> Result<(), UsbDevError> {
        self.core.close_endpoint(address)
    }

    /// Stall an endpoint.
    pub fn set_stall(&mut self, address: u8) -> Result<(), UsbDevError> {
        self.core.stall(address, true)
    }

    /// Clear stall on an endpoint.
    pub fn clear_stall(&mut self, address: u8) -> Result<(), UsbDevError> {
        self.core.stall(address, false)
    }

    /// Start an IN DMA transfer; `data` must stay valid until completion.
    pub fn start_write_dma(&mut self, address: u8, data: &[u8]) -> Result<(), UsbDevError> {
        self.core.start_write_dma(address, data)
    }

    /// Start an OUT DMA transfer; `data` must stay valid until completion.
    pub fn start_read_dma(&mut self, address: u8, data: &mut [u8]) -> Result<(), UsbDevError> {
        self.core.start_read_dma(address, data)
    }

    /// Poll the controller and pop the next pending [`UsbDevEvent`].
    pub fn handle_interrupt(&mut self) -> Option<UsbDevEvent> {
        self.core.handle_interrupt()
    }
}

impl<'a> ControllerCore<'a> {
    pub(crate) fn new(
        reg: &'a RegisterBlock,
        config: UsbConfig,
        options: UsbDevOptions,
        cmu: &mut Cmu,
        syscfg: &mut SocSysCfg,
    ) -> Result<Self, UsbDevError> {
        Self::initialize(reg, cmu, syscfg, config, options)?;
        let mut this = Self {
            reg,
            config,
            tx_len: [0; EP_COUNT],
            rx_len: [0; EP_COUNT],
            tx_ptr: [core::ptr::null(); EP_COUNT],
            rx_ptr: [core::ptr::null_mut(); EP_COUNT],
            mps: [0; EP_COUNT],
            np_txfifo_map: 0,
            events: [None; EVENT_QUEUE_LEN],
            event_len: 0,
        };
        this.reset_endpoints()?;
        Ok(this)
    }

    pub(crate) fn set_connected(&mut self, connected: bool) {
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

    pub(crate) fn enable_global_interrupt(&mut self) {
        unsafe {
            self.reg
                .usb_dev_init
                .modify(|v| v.enable_global_interrupt());
        }
    }

    pub(crate) fn disable_global_interrupt(&mut self) {
        unsafe {
            self.reg
                .usb_dev_init
                .modify(|v| v.disable_global_interrupt());
        }
    }

    pub(crate) fn set_address(&mut self, address: u8) -> Result<(), UsbDevError> {
        if address > 127 {
            return Err(UsbDevError::InvalidEndpoint);
        }
        unsafe {
            self.reg.usb_dev_conf.modify(|v| v.set_dev_addr(address));
        }
        Ok(())
    }

    pub(crate) fn speed(&self) -> UsbDevSpeed {
        self.reg.usb_line_sts.read().enum_speed()
    }

    pub(crate) fn reset_endpoints(&mut self) -> Result<(), UsbDevError> {
        Self::reset_controller(self.reg);
        for i in 0..EP_COUNT {
            self.tx_len[i] = 0;
            self.rx_len[i] = 0;
            self.mps[i] = 0;
        }
        self.np_txfifo_map = 0;
        self.event_len = 0;
        Ok(())
    }

    pub(crate) fn start_setup_read(&mut self, data: &mut [u8; 24]) -> Result<(), UsbDevError> {
        let ptr = data.as_mut_ptr();
        if (ptr as usize & 3) != 0 || ptr as usize > u32::MAX as usize {
            return Err(UsbDevError::InvalidDmaBuffer);
        }
        // Hardware requires 3 back-to-back 8-byte SETUP packet slots (24 bytes).
        self.rx_len[0] = 24;
        self.rx_ptr[0] = ptr;
        unsafe {
            cache::dcache_invalidate_range(ptr as usize, 24);
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
        }
        Ok(())
    }

    pub(crate) fn open_endpoint(&mut self, ep: UsbEndpointConfig) -> Result<(), UsbDevError> {
        let idx = ep.index()?;
        if ep.max_packet_size == 0 || ep.max_packet_size > 1024 {
            return Err(UsbDevError::InvalidMaxPacketSize);
        }
        self.mps[idx] = ep.max_packet_size;
        // EP0 encodes max packet size as a 2-bit selector; EP1..EP4 use byte counts.
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
        unsafe {
            self.reg.usb_ep_int_msk.modify(|v| {
                if ep.address & 0x80 != 0 {
                    v.set_in_ep_mask(v.in_ep_mask() | (1 << idx))
                } else {
                    v.set_out_ep_mask(v.out_ep_mask() | (1 << idx))
                }
            });
            if ep.address & 0x80 != 0 {
                self.reg.in_ep_cfg[idx].modify(|v| {
                    v.clear_ep_disable()
                        .set_mps(mps)
                        .set_ep_type(ep.ep_type)
                        .set_tx_fifo_num(TxFifoSelect::NonPeriodic)
                        .set_data0_pid(true)
                        .activate_ep()
                });
                self.np_txfifo_map |= 1 << idx;
                Self::set_dma_next_ep(self.reg, self.np_txfifo_map);
            } else {
                self.reg.out_ep_cfg[idx].modify(|v| {
                    v.clear_ep_disable()
                        .set_mps(mps)
                        .set_ep_type(ep.ep_type)
                        .set_data0_pid(true)
                        .activate_ep()
                });
            }
        }
        Ok(())
    }

    pub(crate) fn close_endpoint(&mut self, address: u8) -> Result<(), UsbDevError> {
        let idx = (address & 0xf) as usize;
        if idx >= EP_COUNT {
            return Err(UsbDevError::InvalidEndpoint);
        }
        unsafe {
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
        }
        Ok(())
    }

    pub(crate) fn stall(&mut self, address: u8, set: bool) -> Result<(), UsbDevError> {
        let idx = (address & 0xf) as usize;
        if idx >= EP_COUNT {
            return Err(UsbDevError::InvalidEndpoint);
        }
        unsafe {
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
        }
        Ok(())
    }

    pub(crate) fn start_write_dma(&mut self, address: u8, data: &[u8]) -> Result<(), UsbDevError> {
        self.start_dma(address, data.as_ptr() as *mut u8, data.len(), true)
    }

    pub(crate) fn start_read_dma(
        &mut self,
        address: u8,
        data: &mut [u8],
    ) -> Result<(), UsbDevError> {
        self.start_dma(address, data.as_mut_ptr(), data.len(), false)
    }

    fn start_dma(
        &mut self,
        address: u8,
        ptr: *mut u8,
        len: usize,
        write: bool,
    ) -> Result<(), UsbDevError> {
        let idx = (address & 0xf) as usize;
        if idx >= EP_COUNT || (address & 0x80 == 0 && write) || (address & 0x80 != 0 && !write) {
            return Err(UsbDevError::InvalidEndpoint);
        }
        // Endpoint DMA address registers are 32 bits wide and require word alignment.
        if len != 0 && ((ptr as usize & 3) != 0 || ptr as usize > u32::MAX as usize) {
            return Err(UsbDevError::InvalidDmaBuffer);
        }
        if len > XFER_MASK as usize {
            return Err(UsbDevError::TransferTooLong);
        }
        let mps = self.mps[idx].max(1) as usize;
        unsafe {
            if write {
                self.tx_len[idx] = len;
                self.tx_ptr[idx] = ptr;
                let (pkt_count, size) = if idx == 0 {
                    (1, len.min(mps))
                } else {
                    (len.div_ceil(mps).max(1) as u16, len)
                };
                if len != 0 {
                    cache::dcache_clean_invalidate_range(ptr as usize, len);
                }
                self.reg.in_ep_tsf_siz[idx]
                    .modify(|v| v.set_pkt_cnt(pkt_count).set_xfr_siz(size as u32));
                if len != 0 {
                    self.reg.in_ep_dma[idx].write(ptr as u32);
                }
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
                // EP0 always receives a valid bounce buffer even on zero-length status stages.
                if len != 0 || idx == 0 {
                    self.reg.out_ep_dma[idx].write(ptr as u32);
                }
                self.reg.out_ep_int[idx].modify(|v| v.clear_all());
                self.reg.out_ep_cfg[idx]
                    .modify(|v| v.clear_ep_disable().clear_nack(true).enable_ep());
            }
        }
        Ok(())
    }

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

    fn set_turnaround_time(&mut self, clock_hz: u32) {
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

    pub(crate) fn handle_interrupt(&mut self) -> Option<UsbDevEvent> {
        if self.event_len == 0 {
            self.poll();
        }
        self.pop_event()
    }

    /// Read controller interrupt status, acknowledge pending bits, and queue events.
    pub(crate) fn poll(&mut self) {
        unsafe {
            let status = self.reg.usb_int_sts.read();
            let mask = self.reg.usb_int_msk.read();

            if status.is_out_ep_int() && mask.is_out_ep_int_enabled() {
                let pending = self.reg.usb_ep_int.read().out_ep_int();
                let enable = self.reg.usb_ep_int_msk.read().out_ep_mask();
                for idx in 0..EP_COUNT {
                    if pending & (1 << idx) == 0 || enable & (1 << idx) == 0 {
                        continue;
                    }
                    let epint = self.reg.out_ep_int[idx].read();
                    self.reg.out_ep_int[idx].write(epint);
                    if idx == 0 {
                        self.handle_ep0_out(epint);
                    } else if epint.is_rx_comp_int() {
                        let armed = self.rx_len[idx];
                        let remaining = self.reg.out_ep_tsf_siz[idx].read().xfr_siz() as usize;
                        let actual = armed.saturating_sub(remaining);
                        self.rx_len[idx] = 0;
                        if armed != 0 {
                            cache::dcache_invalidate_range(self.rx_ptr[idx] as usize, armed);
                        }
                        self.push_event(UsbDevEvent::OutComplete {
                            ep: idx as u8,
                            len: actual,
                        });
                    }
                }
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
            }

            if status.is_usb_reset() && mask.is_usb_reset_enabled() {
                // Write W1C only to `usb_reset` so other pending interrupt bits survive.
                self.reg.usb_int_sts.modify(|v| v.zero().clear_usb_reset());
                self.reg.usb_dev_func.modify(|v| v.disable_remote_wakeup());
                let _ = self.reset_endpoints();
                self.push_event(UsbDevEvent::Reset);
            }

            if status.is_enum_done() && mask.is_enum_done_enabled() {
                self.reg.usb_int_sts.modify(|v| v.zero().clear_enum_done());
                self.set_turnaround_time(self.config.ahb_clock_hz);
                self.reg.usb_dev_func.modify(|v| v.clear_np_in_nack());
                self.push_event(UsbDevEvent::Enumerated(self.speed()));
            }

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

    fn handle_ep0_out(&mut self, epint: OutEpInt) {
        if epint.is_setup_done_int() {
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
            unsafe {
                cache::dcache_invalidate_range(ptr as usize, actual);
            }
        }
        // Report zero-length completions too so control status stages wake their futures.
        self.push_event(UsbDevEvent::OutComplete { ep: 0, len: actual });
    }

    #[inline]
    pub(crate) fn settle() {
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        riscv::asm::delay(200_000);
        #[cfg(not(any(target_arch = "riscv32", target_arch = "riscv64")))]
        for _ in 0..1024 {
            core::hint::spin_loop();
        }
    }

    fn initialize(
        reg: &RegisterBlock,
        cmu: &mut Cmu,
        syscfg: &mut SocSysCfg,
        config: UsbConfig,
        options: UsbDevOptions,
    ) -> Result<(), UsbDevError> {
        let cmu_reg = cmu.register_block();

        unsafe {
            syscfg
                .register_block()
                .usb0_cfg
                .modify(|v| v.set_drd_mode(DrdMode::Device));
        }

        unsafe {
            cmu_reg.clock_usb_phy0.modify(|v| v.enable_module_clk());
            cmu_reg.clock_usb_dev.modify(|v| v.enable_bus_clk());
            Self::settle();

            cmu_reg.clock_usb_phy0.modify(|v| v.enable_module_reset());
            cmu_reg.clock_usb_dev.modify(|v| v.enable_module_reset());
            Self::settle();
            cmu_reg.clock_usb_phy0.modify(|v| v.disable_module_reset());
            cmu_reg.clock_usb_dev.modify(|v| v.disable_module_reset());
            Self::settle();

            reg.usb_dev_func
                .modify(|v| v.set_connection(UsbDeviceConnection::Disconnected));
            reg.usb_dev_init.modify(|v| v.disable_global_interrupt());

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

            // Program PHY interface after soft reset (reset clears PHY selection bits).
            reg.usb_phy_if.modify(|v| {
                v.set_phy_interface(match config.phy_interface {
                    UsbPhyInterface::Utmi => PhyInterface::UtmiPlus,
                    UsbPhyInterface::Ulpi => PhyInterface::Ulpi,
                })
                .set_utmi_phy_interface_width(match config.utmi_interface_width {
                    UsbUtmiInterfaceWidth::Bits8 => UtmiPhyInterfaceWidth::Bits8,
                    UsbUtmiInterfaceWidth::Bits16 => UtmiPhyInterfaceWidth::Bits16,
                })
                .set_timeout_cal(7)
                .set_ta_time(5)
            });

            reg.usb_dev_conf.modify(|v| {
                v.set_dev_speed(UsbDevSpeed::High)
                    .set_per_frame_int(PeriodicFrameInterval::Percent80)
                    .set_dev_addr(0)
            });

            Self::set_dma_next_ep(reg, 0);

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

            reg.inep_int_msk.modify(|v| v.clear_all());
            reg.outep_int_msk.modify(|v| v.clear_all());
            reg.usb_ep_int_msk.modify(|v| v.clear_all());
            for i in 0..EP_COUNT {
                reg.in_ep_int[i].modify(|v| v.clear_all());
                reg.out_ep_int[i].modify(|v| v.clear_all());
            }

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

            Self::reset_controller(reg);
            if options.unmask_global_interrupt {
                reg.usb_dev_init.modify(|v| v.enable_global_interrupt());
            }
            if options.connect {
                reg.usb_dev_func
                    .modify(|v| v.set_connection(UsbDeviceConnection::Connected));
            }
        }
        Ok(())
    }

    /// Disconnect D+, mask all controller interrupts, and stop endpoint DMA engines.
    pub(crate) fn mute_controller(&mut self) {
        unsafe {
            self.reg
                .usb_dev_func
                .modify(|v| v.set_connection(UsbDeviceConnection::Disconnected));
            self.reg
                .usb_dev_init
                .modify(|v| v.disable_global_interrupt());
            self.reg.usb_int_msk.modify(|v| v.clear_all());
            self.reg.inep_int_msk.modify(|v| v.clear_all());
            self.reg.outep_int_msk.modify(|v| v.clear_all());

            Self::reset_controller(self.reg);
            self.reg.usb_ep_int_msk.modify(|v| v.clear_all());
            Self::settle();
        }
    }

    fn reset_controller(reg: &RegisterBlock) {
        unsafe {
            reg.usb_dev_conf.modify(|v| v.set_dev_addr(0));

            Self::flush_txfifo(reg);
            Self::flush_rxfifo(reg);

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

            Self::set_dma_next_ep(reg, 0);
        }
    }

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
}

/// Assert USB_DEV and PHY0 module resets and gate their clocks.
pub(crate) fn release_clocks_and_reset(cmu: &mut Cmu) {
    unsafe {
        let cmu_reg = cmu.register_block();

        cmu_reg.clock_usb_dev.modify(|v| v.enable_module_reset());
        cmu_reg.clock_usb_phy0.modify(|v| v.enable_module_reset());
        ControllerCore::settle();

        cmu_reg.clock_usb_dev.modify(|v| v.disable_bus_clk());
        cmu_reg.clock_usb_phy0.modify(|v| v.disable_module_clk());
    }
}
