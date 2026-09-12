//! USBDRD host register blocks and registers.

use volatile_register::{RO, RW};

/// EHCI host controller operational registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Capability registers (`CAPLENGTH`).
    #[doc(alias = "CAPLENGTH")]
    pub cap_length_hci_version: RO<CapLengthHciVersion>,
    /// Host controller structural parameters (`HCSPARAMS`).
    #[doc(alias = "HCSPARAMS")]
    pub hcs_params: RO<HcsParams>,
    /// Host controller capability parameters (`HCCPARAMS`).
    #[doc(alias = "HCCPARAMS")]
    pub hcc_params: RO<HccParams>,
    /// Companion port route description (`HCSP_PORTROUTE`).
    ///
    /// Standard EHCI capability register not described in the D13x user
    /// manual; kept for EHCI spec compliance.
    #[doc(alias = "HCSP_PORTROUTE")]
    pub hcs_port_route: RO<HcsPortRoute>,
    /// USB command register (`USBCMD`).
    #[doc(alias = "USBCMD")]
    pub usb_cmd: RW<UsbCmd>,
    /// USB status register (`USBSTS`).
    #[doc(alias = "USBSTS")]
    pub usb_sts: RW<UsbSts>,
    /// USB interrupt enable register (`USBINTR`).
    #[doc(alias = "USBINTR")]
    pub usb_intr: RW<UsbIntr>,
    /// Frame index register (`FRINDEX`).
    #[doc(alias = "FRINDEX")]
    pub frame_index: RW<FrameIndex>,
    /// Control data structure segment register (`CTRLDSSEGMENT`).
    ///
    /// Standard EHCI operational register not described in the D13x user
    /// manual; kept for EHCI spec compliance and used by the C host driver.
    #[doc(alias = "CTRLDSSEGMENT")]
    pub ctrl_ds_segment: RW<CtrlDsSegment>,
    /// Periodic frame list base address register (`PERIODICLISTBASE`).
    #[doc(alias = "PERIODICLISTBASE")]
    pub periodic_list_base: RW<PeriodicListBase>,
    /// Asynchronous list address register (`ASYNCLISTADDR`).
    #[doc(alias = "ASYNCLISTADDR")]
    pub async_list_addr: RW<AsyncListAddr>,
    _reserved0: [u8; 0x24],
    /// Configure flag register (`CONFIGFLAG`).
    #[doc(alias = "CONFIGFLAG")]
    pub config_flag: RW<ConfigFlag>,
    /// Port status and control register (`PORTSC`).
    #[doc(alias = "PORTSC")]
    pub port_sc: RW<PortSc>,
    _reserved1: [u8; 0x3C],
    /// AHB to STBUS interface register 01 (`AHB2STBUS_INSREG01`).
    #[doc(alias = "AHB2STBUS_INSREG01")]
    pub ahb2stbus_insreg01: RW<Ahb2StbusInsreg01>,
    _reserved1b: [u8; 0x368],
    /// OHCI register block.
    pub ohci: OhciRegisterBlock,
    _reserved2: [u8; 0x3A8],
    /// AIC host control register (`USB_HOST_CTL`).
    #[doc(alias = "USB_HOST_CTL")]
    pub usb_host_ctl: RW<UsbHostCtl>,
}

/// OHCI operational registers.
#[repr(C)]
pub struct OhciRegisterBlock {
    /// OHCI revision register (`HcRevision`).
    pub revision: RO<OhciRevision>,
    /// OHCI control register (`HcControl`).
    pub control: RW<OhciControl>,
    /// OHCI command status register (`HcCommandStatus`).
    pub command_status: RW<OhciCommandStatus>,
    /// OHCI interrupt status register (`HcInterruptStatus`).
    pub interrupt_status: RW<OhciInterruptStatus>,
    /// OHCI interrupt enable register (`HcInterruptEnable`).
    pub interrupt_enable: RW<OhciInterruptEnable>,
    /// OHCI interrupt disable register (`HcInterruptDisable`).
    pub interrupt_disable: RW<OhciInterruptDisable>,
    /// Host controller communication area register (`HcHCCA`).
    pub hcca: RW<OhciHcca>,
    /// Periodic current endpoint descriptor register (`HcPeriodCurrentED`).
    pub periodic_current_ed: RW<OhciEdPointer>,
    /// Control head endpoint descriptor register (`HcControlHeadED`).
    pub control_head_ed: RW<OhciEdPointer>,
    /// Control current endpoint descriptor register (`HcControlCurrentED`).
    pub control_current_ed: RW<OhciEdPointer>,
    /// Bulk head endpoint descriptor register (`HcBulkHeadED`).
    pub bulk_head_ed: RW<OhciEdPointer>,
    /// Bulk current endpoint descriptor register (`HcBulkCurrentED`).
    pub bulk_current_ed: RW<OhciEdPointer>,
    /// Done head register (`HcDoneHead`).
    pub done_head: RO<OhciDoneHead>,
    /// Frame interval register (`HcFmInterval`).
    pub frame_interval: RW<OhciFrameInterval>,
    /// Frame remaining register (`HcFmRemaining`).
    pub frame_remaining: RO<OhciFrameRemaining>,
    /// Frame number register (`HcFmNumber`).
    pub frame_number: RO<OhciFrameNumber>,
    /// Periodic start register (`HcPeriodicStart`).
    pub periodic_start: RW<OhciPeriodicStart>,
    /// Low-speed threshold register (`HcLSThreshold`).
    pub low_speed_threshold: RW<OhciLsThreshold>,
    /// Root hub descriptor A register (`HcRhDescriptorA`).
    pub root_hub_descriptor_a: RW<OhciRhDescriptorA>,
    /// Root hub descriptor B register (`HcRhDescriptorB`).
    pub root_hub_descriptor_b: RW<OhciRhDescriptorB>,
    /// Root hub status register (`HcRhStatus`).
    pub root_hub_status: RW<OhciRhStatus>,
    /// Root hub port status register (`HcRhPortStatus`).
    pub root_hub_port_status: RW<OhciRhPortStatus>,
}

/// EHCI capability length and interface version register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CapLengthHciVersion(u32);

impl CapLengthHciVersion {
    const HCI_VERSION: u32 = 0xFFFF << 16;
    const CAP_LENGTH: u32 = 0xFF;

    /// Get EHCI interface version (`HCI_VERSION`).
    #[doc(alias = "HCIVERSION")]
    #[inline]
    pub const fn hci_version(self) -> u16 {
        ((self.0 & Self::HCI_VERSION) >> 16) as u16
    }
    /// Get operational register offset (`CAP_LENGTH`).
    #[doc(alias = "CAPLENGTH")]
    #[inline]
    pub const fn cap_length(self) -> u8 {
        (self.0 & Self::CAP_LENGTH) as u8
    }
}

/// EHCI structural parameters register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct HcsParams(u32);

impl HcsParams {
    const DEBUG_PORT_NUMBER: u32 = 0xF << 20;
    const PORT_INDICATOR: u32 = 1 << 16;
    const N_CC: u32 = 0xF << 12;
    const N_PCC: u32 = 0xF << 8;
    const PORT_ROUTING_RULES: u32 = 1 << 7;
    const PPC: u32 = 1 << 4;
    const N_PORTS: u32 = 0xF;

    /// Get debug port number (`DEBUG_PORT_NUMBER`).
    #[doc(alias = "DEBUG_PORT_NUMBER")]
    #[inline]
    pub const fn debug_port_number(self) -> u8 {
        ((self.0 & Self::DEBUG_PORT_NUMBER) >> 20) as u8
    }
    /// Check if port indicators are supported (`P_INDICATOR`).
    #[doc(alias = "P_INDICATOR")]
    #[inline]
    pub const fn has_port_indicator(self) -> bool {
        (self.0 & Self::PORT_INDICATOR) != 0
    }
    /// Get number of companion controllers (`N_CC`).
    #[doc(alias = "N_CC")]
    #[inline]
    pub const fn companion_controller_count(self) -> u8 {
        ((self.0 & Self::N_CC) >> 12) as u8
    }
    /// Get number of ports per companion controller (`N_PCC`).
    #[doc(alias = "N_PCC")]
    #[inline]
    pub const fn ports_per_companion_controller(self) -> u8 {
        ((self.0 & Self::N_PCC) >> 8) as u8
    }
    /// Check if explicit port routing is used (`PORT_ROUTING_RULES`).
    #[doc(alias = "PORT_ROUTING_RULES")]
    #[inline]
    pub const fn has_explicit_port_routing(self) -> bool {
        (self.0 & Self::PORT_ROUTING_RULES) != 0
    }
    /// Check if per-port power control is supported (`PPC`).
    #[doc(alias = "PPC")]
    #[inline]
    pub const fn has_port_power_control(self) -> bool {
        (self.0 & Self::PPC) != 0
    }
    /// Get number of downstream ports (`N_PORTS`).
    #[doc(alias = "N_PORTS")]
    #[inline]
    pub const fn port_count(self) -> u8 {
        (self.0 & Self::N_PORTS) as u8
    }
}

/// EHCI capability parameters register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct HccParams(u32);

impl HccParams {
    const EECP: u32 = 0xFF << 8;
    const ISOCHRONOUS_SCHEDULING_THRESHOLD: u32 = 0xF << 4;
    const ASYNC_SCHEDULE_PARK_CAPABILITY: u32 = 1 << 2;
    const PROGRAMMABLE_FRAME_LIST_FLAG: u32 = 1 << 1;
    const ADDRESSING_CAPABILITY_64: u32 = 1;

    /// Get EHCI extended capabilities pointer (`EECP`).
    #[doc(alias = "EECP")]
    #[inline]
    pub const fn eecp(self) -> u8 {
        ((self.0 & Self::EECP) >> 8) as u8
    }
    /// Get isochronous scheduling threshold (`ISOCHRONOUS_SCHEDULING_THRESHOLD`).
    #[doc(alias = "ISOCHRONOUS_SCHEDULING_THRESHOLD")]
    #[inline]
    pub const fn isochronous_scheduling_threshold(self) -> u8 {
        ((self.0 & Self::ISOCHRONOUS_SCHEDULING_THRESHOLD) >> 4) as u8
    }
    /// Check if asynchronous schedule parking is supported (`ASYNC_SCHEDULE_PARK_CAPABILITY`).
    #[doc(alias = "ASYNC_SCHEDULE_PARK_CAPABILITY")]
    #[inline]
    pub const fn has_async_schedule_park(self) -> bool {
        (self.0 & Self::ASYNC_SCHEDULE_PARK_CAPABILITY) != 0
    }
    /// Check if frame list size is programmable (`PROGRAMMABLE_FRAME_LIST_FLAG`).
    #[doc(alias = "PROGRAMMABLE_FRAME_LIST_FLAG")]
    #[inline]
    pub const fn has_programmable_frame_list(self) -> bool {
        (self.0 & Self::PROGRAMMABLE_FRAME_LIST_FLAG) != 0
    }
    /// Check if 64-bit addressing is supported (`ADDRESSING_CAPABILITY_64`).
    #[doc(alias = "ADDRESSING_CAPABILITY_64")]
    #[inline]
    pub const fn has_64bit_addressing(self) -> bool {
        (self.0 & Self::ADDRESSING_CAPABILITY_64) != 0
    }
}

/// EHCI companion port route description register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct HcsPortRoute(u32);

impl HcsPortRoute {
    /// Get companion controller route for a port.
    #[inline]
    pub const fn port_route(self, port: u8) -> u8 {
        assert!(port < 8, "Port number out of range (expected 0..=7)");
        ((self.0 >> (port as u32 * 4)) & 0xF) as u8
    }
}

/// Periodic frame-list size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FrameListSize {
    /// 1024 frames.
    Frames1024,
    /// 512 frames.
    Frames512,
    /// 256 frames.
    Frames256,
}

/// Interrupt threshold interval in microframes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum InterruptThreshold {
    /// 1 microframe.
    Microframes1 = 1,
    /// 2 microframes.
    Microframes2 = 2,
    /// 4 microframes.
    Microframes4 = 4,
    /// 8 microframes.
    Microframes8 = 8,
    /// 16 microframes.
    Microframes16 = 0x10,
    /// 32 microframes.
    Microframes32 = 0x20,
    /// 64 microframes.
    Microframes64 = 0x40,
}

/// Asynchronous schedule park mode transaction count.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AsyncScheduleParkModeCount {
    /// 1 transaction.
    Transactions1 = 1,
    /// 2 transactions.
    Transactions2,
    /// 3 transactions.
    Transactions3,
}

/// USB Command register.
///
/// The async schedule park mode fields follow the cherryusb EHCI reference and
/// are not described in the D13x user manual; this controller does not support
/// park mode (`HCCPARAMS.ASPC` is 0).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UsbCmd(u32);

impl UsbCmd {
    const INT_THRESHOLD_CTL: u32 = 0xFF << 16;
    const ASYNC_SCHEDULE_PARK_MODE_ENABLE: u32 = 1 << 11;
    const ASYNC_SCHEDULE_PARK_MODE_COUNT: u32 = 0x3 << 8;
    const LIGHT_HOST_CONTROLLER_RESET: u32 = 1 << 7;
    const INT_ASYNC_ADVANCE_DOORBELL: u32 = 1 << 6;
    const ASYNC_SCHEDULE_ENABLE: u32 = 1 << 5;
    const PERIODIC_SCHEDULE_ENABLE: u32 = 1 << 4;
    const FLIST_SIZE: u32 = 0x3 << 2;
    const HC_RESET: u32 = 1 << 1;
    const RUN_STOP: u32 = 1;

    /// Set interrupt threshold control (`INT_THRESHOLD_CTL`).
    #[doc(alias = "INT_THRESHOLD_CTL")]
    #[inline]
    pub const fn set_int_threshold(self, threshold: InterruptThreshold) -> Self {
        Self(
            (self.0 & !Self::INT_THRESHOLD_CTL)
                | (((threshold as u32) << 16) & Self::INT_THRESHOLD_CTL),
        )
    }
    /// Get interrupt threshold control.
    #[inline]
    pub const fn int_threshold(self) -> InterruptThreshold {
        match (self.0 & Self::INT_THRESHOLD_CTL) >> 16 {
            1 => InterruptThreshold::Microframes1,
            2 => InterruptThreshold::Microframes2,
            4 => InterruptThreshold::Microframes4,
            8 => InterruptThreshold::Microframes8,
            0x10 => InterruptThreshold::Microframes16,
            0x20 => InterruptThreshold::Microframes32,
            0x40 => InterruptThreshold::Microframes64,
            _ => panic!("Invalid interrupt threshold"),
        }
    }
    /// Enable async schedule park mode (`ASYNC_SCHEDULE_PARK_MODE_ENABLE`).
    #[doc(alias = "ASYNC_SCHEDULE_PARK_MODE_ENABLE")]
    #[inline]
    pub const fn enable_park_mode(self) -> Self {
        Self(self.0 | Self::ASYNC_SCHEDULE_PARK_MODE_ENABLE)
    }
    /// Disable async schedule park mode.
    #[inline]
    pub const fn disable_park_mode(self) -> Self {
        Self(self.0 & !Self::ASYNC_SCHEDULE_PARK_MODE_ENABLE)
    }
    /// Check if park mode is enabled.
    #[inline]
    pub const fn is_park_mode_enabled(self) -> bool {
        (self.0 & Self::ASYNC_SCHEDULE_PARK_MODE_ENABLE) != 0
    }
    /// Set async schedule park mode count (`ASYNC_SCHEDULE_PARK_MODE_COUNT`).
    #[doc(alias = "ASYNC_SCHEDULE_PARK_MODE_COUNT")]
    #[inline]
    pub const fn set_park_mode_count(self, count: AsyncScheduleParkModeCount) -> Self {
        Self(
            (self.0 & !Self::ASYNC_SCHEDULE_PARK_MODE_COUNT)
                | (((count as u32) << 8) & Self::ASYNC_SCHEDULE_PARK_MODE_COUNT),
        )
    }
    /// Get park mode count.
    #[inline]
    pub const fn park_mode_count(self) -> AsyncScheduleParkModeCount {
        match (self.0 & Self::ASYNC_SCHEDULE_PARK_MODE_COUNT) >> 8 {
            1 => AsyncScheduleParkModeCount::Transactions1,
            2 => AsyncScheduleParkModeCount::Transactions2,
            3 => AsyncScheduleParkModeCount::Transactions3,
            _ => panic!("Invalid async schedule park mode count"),
        }
    }
    /// Assert Light Host Controller Reset (`LIGHT_HOST_CONTROLLER_RESET`).
    #[doc(alias = "LIGHT_HOST_CONTROLLER_RESET")]
    #[inline]
    pub const fn light_hc_reset(self) -> Self {
        Self(self.0 | Self::LIGHT_HOST_CONTROLLER_RESET)
    }
    /// Check if light HC reset is asserted.
    #[inline]
    pub const fn is_light_hc_reset(self) -> bool {
        (self.0 & Self::LIGHT_HOST_CONTROLLER_RESET) != 0
    }
    /// Set interrupt on async advance doorbell (`INT_ASYNC_ADVANCE_DOORBELL`).
    #[doc(alias = "INT_ASYNC_ADVANCE_DOORBELL")]
    #[inline]
    pub const fn set_int_async_advance_doorbell(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::INT_ASYNC_ADVANCE_DOORBELL)
        } else {
            Self(self.0 & !Self::INT_ASYNC_ADVANCE_DOORBELL)
        }
    }
    /// Get interrupt on async advance doorbell bit.
    #[inline]
    pub const fn int_async_advance_doorbell(self) -> bool {
        (self.0 & Self::INT_ASYNC_ADVANCE_DOORBELL) != 0
    }
    /// Enable async schedule (`ASYNC_SCHEDULE_ENABLE`).
    #[doc(alias = "ASYNC_SCHEDULE_ENABLE")]
    #[inline]
    pub const fn enable_async_schedule(self) -> Self {
        Self(self.0 | Self::ASYNC_SCHEDULE_ENABLE)
    }
    /// Disable async schedule.
    #[inline]
    pub const fn disable_async_schedule(self) -> Self {
        Self(self.0 & !Self::ASYNC_SCHEDULE_ENABLE)
    }
    /// Check if async schedule is enabled.
    #[inline]
    pub const fn is_async_schedule_enabled(self) -> bool {
        (self.0 & Self::ASYNC_SCHEDULE_ENABLE) != 0
    }
    /// Enable periodic schedule (`PERIODIC_SCHEDULE_ENABLE`).
    #[doc(alias = "PERIODIC_SCHEDULE_ENABLE")]
    #[inline]
    pub const fn enable_periodic_schedule(self) -> Self {
        Self(self.0 | Self::PERIODIC_SCHEDULE_ENABLE)
    }
    /// Disable periodic schedule.
    #[inline]
    pub const fn disable_periodic_schedule(self) -> Self {
        Self(self.0 & !Self::PERIODIC_SCHEDULE_ENABLE)
    }
    /// Check if periodic schedule is enabled.
    #[inline]
    pub const fn is_periodic_schedule_enabled(self) -> bool {
        (self.0 & Self::PERIODIC_SCHEDULE_ENABLE) != 0
    }
    /// Get frame list size (`FLIST_SIZE`).
    ///
    /// Read-only on this controller: the frame list size is not programmable
    /// (`HCCPARAMS.PFLF` is 0), so it always reports 1024 frames.
    #[doc(alias = "FLIST_SIZE")]
    #[inline]
    pub const fn flist_size(self) -> FrameListSize {
        match (self.0 & Self::FLIST_SIZE) >> 2 {
            0 => FrameListSize::Frames1024,
            1 => FrameListSize::Frames512,
            2 => FrameListSize::Frames256,
            _ => panic!("Invalid frame-list size"),
        }
    }
    /// Assert Host Controller Reset (`HC_RESET`).
    #[doc(alias = "HC_RESET")]
    #[inline]
    pub const fn hc_reset(self) -> Self {
        Self(self.0 | Self::HC_RESET)
    }
    /// Check if HC reset is asserted.
    #[inline]
    pub const fn is_hc_reset(self) -> bool {
        (self.0 & Self::HC_RESET) != 0
    }
    /// Set Run/Stop (1 = Run, 0 = Stop) (`RUN_STOP`).
    #[doc(alias = "RUN_STOP")]
    #[inline]
    pub const fn set_run_stop(self, run: bool) -> Self {
        if run {
            Self(self.0 | Self::RUN_STOP)
        } else {
            Self(self.0 & !Self::RUN_STOP)
        }
    }
    /// Check if HC is running.
    #[inline]
    pub const fn is_running(self) -> bool {
        (self.0 & Self::RUN_STOP) != 0
    }
}

/// USB Status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UsbSts(u32);

impl UsbSts {
    const ASYNC_SCHEDULE_STATUS: u32 = 1 << 15;
    const PERIODIC_SCHEDULE_STATUS: u32 = 1 << 14;
    const RECLAMATION: u32 = 1 << 13;
    const HCHALTED: u32 = 1 << 12;
    const INT_ASYNC: u32 = 1 << 5;
    const SYSTEM_ERROR: u32 = 1 << 4;
    const FLIST_ROLLOVER: u32 = 1 << 3;
    const PORT_CHANGE_DETECT: u32 = 1 << 2;
    const USB_ERR_INT: u32 = 1 << 1;
    const USB_INT: u32 = 1;

    /// Check async schedule status (`ASYNC_SCHEDULE_STATUS`).
    #[doc(alias = "ASYNC_SCHEDULE_STATUS")]
    #[inline]
    pub const fn is_async_schedule_status(self) -> bool {
        (self.0 & Self::ASYNC_SCHEDULE_STATUS) != 0
    }
    /// Check periodic schedule status (`PERIODIC_SCHEDULE_STATUS`).
    #[doc(alias = "PERIODIC_SCHEDULE_STATUS")]
    #[inline]
    pub const fn is_periodic_schedule_status(self) -> bool {
        (self.0 & Self::PERIODIC_SCHEDULE_STATUS) != 0
    }
    /// Check Reclamation (`RECLAMATION`).
    #[doc(alias = "RECLAMATION")]
    #[inline]
    pub const fn is_reclamation(self) -> bool {
        (self.0 & Self::RECLAMATION) != 0
    }
    /// Check HC Halted (`HCHALTED`).
    #[doc(alias = "HCHALTED")]
    #[inline]
    pub const fn is_hchalted(self) -> bool {
        (self.0 & Self::HCHALTED) != 0
    }
    /// Check if interrupt on async advance is pending (`INT_ASYNC`).
    #[doc(alias = "INT_ASYNC")]
    #[inline]
    pub const fn is_int_async(self) -> bool {
        (self.0 & Self::INT_ASYNC) != 0
    }
    /// Clear interrupt on async advance status (Write-1-to-Clear).
    #[inline]
    pub const fn clear_int_async(self) -> Self {
        Self(self.0 | Self::INT_ASYNC)
    }
    /// Check if system error is pending (`SYSTEM_ERROR`).
    #[doc(alias = "SYSTEM_ERROR")]
    #[inline]
    pub const fn is_system_error(self) -> bool {
        (self.0 & Self::SYSTEM_ERROR) != 0
    }
    /// Clear system error status (Write-1-to-Clear).
    #[inline]
    pub const fn clear_system_error(self) -> Self {
        Self(self.0 | Self::SYSTEM_ERROR)
    }
    /// Check if frame list rollover is pending (`FLIST_ROLLOVER`).
    #[doc(alias = "FLIST_ROLLOVER")]
    #[inline]
    pub const fn is_flist_rollover(self) -> bool {
        (self.0 & Self::FLIST_ROLLOVER) != 0
    }
    /// Clear frame list rollover status (Write-1-to-Clear).
    #[inline]
    pub const fn clear_flist_rollover(self) -> Self {
        Self(self.0 | Self::FLIST_ROLLOVER)
    }
    /// Check if port change detect is pending (`PORT_CHANGE_DETECT`).
    #[doc(alias = "PORT_CHANGE_DETECT")]
    #[inline]
    pub const fn is_port_change_detect(self) -> bool {
        (self.0 & Self::PORT_CHANGE_DETECT) != 0
    }
    /// Clear port change detect status (Write-1-to-Clear).
    #[inline]
    pub const fn clear_port_change_detect(self) -> Self {
        Self(self.0 | Self::PORT_CHANGE_DETECT)
    }
    /// Check if USB error interrupt is pending (`USB_ERR_INT`).
    #[doc(alias = "USB_ERR_INT")]
    #[inline]
    pub const fn is_usb_err_int(self) -> bool {
        (self.0 & Self::USB_ERR_INT) != 0
    }
    /// Clear USB error interrupt status (Write-1-to-Clear).
    #[inline]
    pub const fn clear_usb_err_int(self) -> Self {
        Self(self.0 | Self::USB_ERR_INT)
    }
    /// Check if USB interrupt is pending (`ASYNC_SCHEDULE_STATUS`).
    #[doc(alias = "USB_INT")]
    #[inline]
    pub const fn is_usb_int(self) -> bool {
        (self.0 & Self::USB_INT) != 0
    }
    /// Clear USB interrupt status (Write-1-to-Clear).
    #[inline]
    pub const fn clear_usb_int(self) -> Self {
        Self(self.0 | Self::USB_INT)
    }
}

/// USB Interrupt Enable register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UsbIntr(u32);

impl UsbIntr {
    const INT_ASYNC: u32 = 1 << 5;
    const SYSTEM_ERROR: u32 = 1 << 4;
    const FLIST_ROLLOVER: u32 = 1 << 3;
    const PORT_CHANGE_DETECT: u32 = 1 << 2;
    const USB_ERR_INT: u32 = 1 << 1;
    const USB_INT: u32 = 1;

    /// Enable interrupt on async advance (`INT_ASYNC`).
    #[doc(alias = "INT_ASYNC")]
    #[inline]
    pub const fn enable_int_async(self) -> Self {
        Self(self.0 | Self::INT_ASYNC)
    }
    /// Disable interrupt on async advance.
    #[inline]
    pub const fn disable_int_async(self) -> Self {
        Self(self.0 & !Self::INT_ASYNC)
    }
    /// Check if interrupt on async advance is enabled.
    #[inline]
    pub const fn is_int_async_enabled(self) -> bool {
        (self.0 & Self::INT_ASYNC) != 0
    }
    /// Enable system error interrupt (`SYSTEM_ERROR`).
    #[doc(alias = "SYSTEM_ERROR")]
    #[inline]
    pub const fn enable_system_error(self) -> Self {
        Self(self.0 | Self::SYSTEM_ERROR)
    }
    /// Disable system error interrupt.
    #[inline]
    pub const fn disable_system_error(self) -> Self {
        Self(self.0 & !Self::SYSTEM_ERROR)
    }
    /// Check if system error interrupt is enabled.
    #[inline]
    pub const fn is_system_error_enabled(self) -> bool {
        (self.0 & Self::SYSTEM_ERROR) != 0
    }
    /// Enable frame list rollover interrupt (`FLIST_ROLLOVER`).
    #[doc(alias = "FLIST_ROLLOVER")]
    #[inline]
    pub const fn enable_flist_rollover(self) -> Self {
        Self(self.0 | Self::FLIST_ROLLOVER)
    }
    /// Disable frame list rollover interrupt.
    #[inline]
    pub const fn disable_flist_rollover(self) -> Self {
        Self(self.0 & !Self::FLIST_ROLLOVER)
    }
    /// Check if frame list rollover interrupt is enabled.
    #[inline]
    pub const fn is_flist_rollover_enabled(self) -> bool {
        (self.0 & Self::FLIST_ROLLOVER) != 0
    }
    /// Enable port change detect interrupt (`PORT_CHANGE_DETECT`).
    #[doc(alias = "PORT_CHANGE_DETECT")]
    #[inline]
    pub const fn enable_port_change_detect(self) -> Self {
        Self(self.0 | Self::PORT_CHANGE_DETECT)
    }
    /// Disable port change detect interrupt.
    #[inline]
    pub const fn disable_port_change_detect(self) -> Self {
        Self(self.0 & !Self::PORT_CHANGE_DETECT)
    }
    /// Check if port change detect interrupt is enabled.
    #[inline]
    pub const fn is_port_change_detect_enabled(self) -> bool {
        (self.0 & Self::PORT_CHANGE_DETECT) != 0
    }
    /// Enable USB error interrupt (`USB_ERR_INT`).
    #[doc(alias = "USB_ERR_INT")]
    #[inline]
    pub const fn enable_usb_err_int(self) -> Self {
        Self(self.0 | Self::USB_ERR_INT)
    }
    /// Disable USB error interrupt.
    #[inline]
    pub const fn disable_usb_err_int(self) -> Self {
        Self(self.0 & !Self::USB_ERR_INT)
    }
    /// Check if USB error interrupt is enabled.
    #[inline]
    pub const fn is_usb_err_int_enabled(self) -> bool {
        (self.0 & Self::USB_ERR_INT) != 0
    }
    /// Enable USB interrupt (`INT_ASYNC`).
    #[doc(alias = "USB_INT")]
    #[inline]
    pub const fn enable_usb_int(self) -> Self {
        Self(self.0 | Self::USB_INT)
    }
    /// Disable USB interrupt.
    #[inline]
    pub const fn disable_usb_int(self) -> Self {
        Self(self.0 & !Self::USB_INT)
    }
    /// Check if USB interrupt is enabled.
    #[inline]
    pub const fn is_usb_int_enabled(self) -> bool {
        (self.0 & Self::USB_INT) != 0
    }
}

/// EHCI frame index register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FrameIndex(u32);

impl FrameIndex {
    const FRAME_INDEX: u32 = 0x3FFF;

    /// Set frame index (`FRAME_INDEX`).
    #[doc(alias = "FRINDEX")]
    #[inline]
    pub const fn set_frame_index(self, index: u16) -> Self {
        assert!(
            index < 0x4000,
            "Frame index out of range (expected 0..=16383)"
        );
        Self((self.0 & !Self::FRAME_INDEX) | index as u32)
    }
    /// Get frame index.
    #[inline]
    pub const fn frame_index(self) -> u16 {
        (self.0 & Self::FRAME_INDEX) as u16
    }
}

/// EHCI control data structure segment register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CtrlDsSegment(u32);

impl CtrlDsSegment {
    /// Set upper 32 address bits for control data structures.
    #[inline]
    pub const fn set_upper_address(self, address: u32) -> Self {
        Self(address)
    }
    /// Get upper 32 address bits for control data structures.
    #[inline]
    pub const fn upper_address(self) -> u32 {
        self.0
    }
}

/// EHCI periodic frame list base register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PeriodicListBase(u32);

impl PeriodicListBase {
    const BASE_ADDRESS: u32 = 0xFFFFF << 12;

    /// Set periodic frame list base address (`BASE_ADDRESS`).
    #[doc(alias = "PERIODICLISTBASE")]
    #[inline]
    pub const fn set_base_address(self, address: u32) -> Self {
        assert!(address & 0xFFF == 0, "Base address must be 4 KiB aligned");
        Self((self.0 & !Self::BASE_ADDRESS) | (address & Self::BASE_ADDRESS))
    }
    /// Get periodic frame list base address.
    #[inline]
    pub const fn base_address(self) -> u32 {
        self.0 & Self::BASE_ADDRESS
    }
}

/// EHCI asynchronous list address register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AsyncListAddr(u32);

impl AsyncListAddr {
    const LINK_POINTER_LOW: u32 = 0x7FFFFFF << 5;

    /// Set asynchronous queue head address (`LINK_POINTER_LOW`).
    #[doc(alias = "ASYNCLISTADDR")]
    #[inline]
    pub const fn set_queue_head_address(self, address: u32) -> Self {
        assert!(
            address & 0x1F == 0,
            "Queue head address must be 32-byte aligned"
        );
        Self((self.0 & !Self::LINK_POINTER_LOW) | (address & Self::LINK_POINTER_LOW))
    }
    /// Get asynchronous queue head address.
    #[inline]
    pub const fn queue_head_address(self) -> u32 {
        self.0 & Self::LINK_POINTER_LOW
    }
}

/// EHCI configure flag register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ConfigFlag(u32);

impl ConfigFlag {
    const CONFIGURE: u32 = 1;

    /// Select EHCI ownership for port 0 (`CONFIGURE`).
    #[doc(alias = "CONFIGURE")]
    #[inline]
    pub const fn select_ehci(self) -> Self {
        Self(self.0 | Self::CONFIGURE)
    }
    /// Select OHCI ownership for port 0.
    #[inline]
    pub const fn select_ohci(self) -> Self {
        Self(self.0 & !Self::CONFIGURE)
    }
    /// Check if EHCI owns port 0.
    #[inline]
    pub const fn is_ehci_selected(self) -> bool {
        (self.0 & Self::CONFIGURE) != 0
    }
    /// Check if OHCI owns port 0.
    #[inline]
    pub const fn is_ohci_selected(self) -> bool {
        (self.0 & Self::CONFIGURE) == 0
    }
}

/// USB line status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PortLineStatus {
    /// Single-ended zero.
    Se0,
    /// K state.
    KState,
    /// J state.
    JState,
}

/// Port indicator control.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PortIndicatorControl {
    /// Indicator off.
    Off,
    /// Amber indicator.
    Amber,
    /// Green indicator.
    Green,
}

/// Port test control.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PortTestControl {
    /// Test mode disabled.
    Disabled,
    /// Force J state.
    J,
    /// Force K state.
    K,
    /// Force SE0 with NAK.
    Se0Nak,
    /// Force test packet.
    Packet,
    /// Force enable.
    ForceEnable,
}

/// Port Status and Control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PortSc(u32);

impl PortSc {
    const WAKE_ON_OVERCURRENT_ENABLE: u32 = 1 << 22;
    const WAKE_ON_DISCONNECT_ENABLE: u32 = 1 << 21;
    const WAKE_ON_CONNECT_ENABLE: u32 = 1 << 20;
    const PORT_TEST_CONTROL: u32 = 0xF << 16;
    const PORT_OWNER: u32 = 1 << 13;
    const PORT_POWER: u32 = 1 << 12;
    const LINE_STATUS: u32 = 0x3 << 10;
    const PORT_RESET: u32 = 1 << 8;
    const SUSPEND: u32 = 1 << 7;
    const FORCE_PORT_RESUME: u32 = 1 << 6;
    const OVER_CURRENT_CHANGE: u32 = 1 << 5;
    const OVER_CURRENT_ACTIVE: u32 = 1 << 4;
    const PORT_ENABLE_CHANGE: u32 = 1 << 3;
    const PORT_ENABLE: u32 = 1 << 2;
    const CONNECT_STATUS_CHANGE: u32 = 1 << 1;
    const CURRENT_CONNECT_STATUS: u32 = 1;

    /// Enable wake on overcurrent (`WAKE_ON_OVERCURRENT_ENABLE`).
    #[doc(alias = "WAKE_ON_OVERCURRENT_ENABLE")]
    #[inline]
    pub const fn enable_wake_on_overcurrent(self) -> Self {
        Self(self.0 | Self::WAKE_ON_OVERCURRENT_ENABLE)
    }
    /// Disable wake on overcurrent.
    #[inline]
    pub const fn disable_wake_on_overcurrent(self) -> Self {
        Self(self.0 & !Self::WAKE_ON_OVERCURRENT_ENABLE)
    }
    /// Check if wake on overcurrent is enabled.
    #[inline]
    pub const fn is_wake_on_overcurrent_enabled(self) -> bool {
        (self.0 & Self::WAKE_ON_OVERCURRENT_ENABLE) != 0
    }
    /// Enable wake on disconnect (`WAKE_ON_DISCONNECT_ENABLE`).
    #[doc(alias = "WAKE_ON_DISCONNECT_ENABLE")]
    #[inline]
    pub const fn enable_wake_on_disconnect(self) -> Self {
        Self(self.0 | Self::WAKE_ON_DISCONNECT_ENABLE)
    }
    /// Disable wake on disconnect.
    #[inline]
    pub const fn disable_wake_on_disconnect(self) -> Self {
        Self(self.0 & !Self::WAKE_ON_DISCONNECT_ENABLE)
    }
    /// Check if wake on disconnect is enabled.
    #[inline]
    pub const fn is_wake_on_disconnect_enabled(self) -> bool {
        (self.0 & Self::WAKE_ON_DISCONNECT_ENABLE) != 0
    }
    /// Enable wake on connect (`WAKE_ON_CONNECT_ENABLE`).
    #[doc(alias = "WAKE_ON_CONNECT_ENABLE")]
    #[inline]
    pub const fn enable_wake_on_connect(self) -> Self {
        Self(self.0 | Self::WAKE_ON_CONNECT_ENABLE)
    }
    /// Disable wake on connect.
    #[inline]
    pub const fn disable_wake_on_connect(self) -> Self {
        Self(self.0 & !Self::WAKE_ON_CONNECT_ENABLE)
    }
    /// Check if wake on connect is enabled.
    #[inline]
    pub const fn is_wake_on_connect_enabled(self) -> bool {
        (self.0 & Self::WAKE_ON_CONNECT_ENABLE) != 0
    }
    /// Set port test control (`PORT_TEST_CONTROL`).
    #[doc(alias = "PORT_TEST_CONTROL")]
    #[inline]
    pub const fn set_port_test_control(self, val: PortTestControl) -> Self {
        Self((self.0 & !Self::PORT_TEST_CONTROL) | (((val as u32) << 16) & Self::PORT_TEST_CONTROL))
    }
    /// Get port test control.
    #[inline]
    pub const fn port_test_control(self) -> PortTestControl {
        match (self.0 & Self::PORT_TEST_CONTROL) >> 16 {
            0 => PortTestControl::Disabled,
            1 => PortTestControl::J,
            2 => PortTestControl::K,
            3 => PortTestControl::Se0Nak,
            4 => PortTestControl::Packet,
            5 => PortTestControl::ForceEnable,
            _ => panic!("Invalid port test control"),
        }
    }
    /// Set port owner bit (`PORT_OWNER`).
    #[doc(alias = "PORT_OWNER")]
    #[inline]
    pub const fn set_port_owner(self, companion: bool) -> Self {
        if companion {
            Self(self.0 | Self::PORT_OWNER)
        } else {
            Self(self.0 & !Self::PORT_OWNER)
        }
    }
    /// Get port owner bit.
    #[inline]
    pub const fn is_port_owner(self) -> bool {
        (self.0 & Self::PORT_OWNER) != 0
    }
    /// Set port power bit (`PORT_POWER`).
    #[doc(alias = "PORT_POWER")]
    #[inline]
    pub const fn set_port_power(self, enable: bool) -> Self {
        if enable {
            Self(self.0 | Self::PORT_POWER)
        } else {
            Self(self.0 & !Self::PORT_POWER)
        }
    }
    /// Get port power bit.
    #[inline]
    pub const fn is_port_power(self) -> bool {
        (self.0 & Self::PORT_POWER) != 0
    }
    /// Get line status (`LINE_STATUS`).
    #[doc(alias = "LINE_STATUS")]
    #[inline]
    pub const fn line_status(self) -> PortLineStatus {
        match (self.0 & Self::LINE_STATUS) >> 10 {
            0 => PortLineStatus::Se0,
            1 => PortLineStatus::KState,
            2 => PortLineStatus::JState,
            _ => panic!("Invalid port line status"),
        }
    }
    /// Assert port reset (`PORT_RESET`).
    #[doc(alias = "PORT_RESET")]
    #[inline]
    pub const fn port_reset(self) -> Self {
        Self(self.0 | Self::PORT_RESET)
    }
    /// Deassert port reset.
    #[inline]
    pub const fn clear_port_reset(self) -> Self {
        Self(self.0 & !Self::PORT_RESET)
    }
    /// Check if port reset is asserted.
    #[inline]
    pub const fn is_port_reset(self) -> bool {
        (self.0 & Self::PORT_RESET) != 0
    }
    /// Set suspend bit (`SUSPEND`).
    #[doc(alias = "SUSPEND")]
    #[inline]
    pub const fn set_suspend(self, enable: bool) -> Self {
        if enable {
            Self(self.0 | Self::SUSPEND)
        } else {
            Self(self.0 & !Self::SUSPEND)
        }
    }
    /// Get suspend bit.
    #[inline]
    pub const fn is_suspended(self) -> bool {
        (self.0 & Self::SUSPEND) != 0
    }
    /// Set force port resume bit (`FORCE_PORT_RESUME`).
    #[doc(alias = "FORCE_PORT_RESUME")]
    #[inline]
    pub const fn force_port_resume(self) -> Self {
        Self(self.0 | Self::FORCE_PORT_RESUME)
    }
    /// Check over-current change (`OVER_CURRENT_CHANGE`).
    #[doc(alias = "OVER_CURRENT_CHANGE")]
    #[inline]
    pub const fn is_over_current_change(self) -> bool {
        (self.0 & Self::OVER_CURRENT_CHANGE) != 0
    }
    /// Clear over-current change (Write-1-to-Clear).
    #[inline]
    pub const fn clear_over_current_change(self) -> Self {
        Self(self.0 | Self::OVER_CURRENT_CHANGE)
    }
    /// Check over-current active (`OVER_CURRENT_ACTIVE`).
    #[doc(alias = "OVER_CURRENT_ACTIVE")]
    #[inline]
    pub const fn is_over_current_active(self) -> bool {
        (self.0 & Self::OVER_CURRENT_ACTIVE) != 0
    }
    /// Check port enable change (`PORT_ENABLE_CHANGE`).
    #[doc(alias = "PORT_ENABLE_CHANGE")]
    #[inline]
    pub const fn is_port_enable_change(self) -> bool {
        (self.0 & Self::PORT_ENABLE_CHANGE) != 0
    }
    /// Clear port enable change (Write-1-to-Clear).
    #[inline]
    pub const fn clear_port_enable_change(self) -> Self {
        Self(self.0 | Self::PORT_ENABLE_CHANGE)
    }
    /// Check port enable (`PORT_ENABLE`).
    #[doc(alias = "PORT_ENABLE")]
    #[inline]
    pub const fn is_port_enable(self) -> bool {
        (self.0 & Self::PORT_ENABLE) != 0
    }
    /// Check connect status change (`CONNECT_STATUS_CHANGE`).
    #[doc(alias = "CONNECT_STATUS_CHANGE")]
    #[inline]
    pub const fn is_connect_status_change(self) -> bool {
        (self.0 & Self::CONNECT_STATUS_CHANGE) != 0
    }
    /// Clear connect status change (Write-1-to-Clear).
    #[inline]
    pub const fn clear_connect_status_change(self) -> Self {
        Self(self.0 | Self::CONNECT_STATUS_CHANGE)
    }
    /// Check current connect status (`CURRENT_CONNECT_STATUS`).
    #[doc(alias = "CURRENT_CONNECT_STATUS")]
    #[inline]
    pub const fn is_current_connect_status(self) -> bool {
        (self.0 & Self::CURRENT_CONNECT_STATUS) != 0
    }
}

/// AHB to STBUS interface register 01.
///
/// Configures the EHCI packet buffer IN/OUT thresholds in DWORDs. The OUT
/// threshold must be raised (FIFO size minus 4) to avoid data underrun.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Ahb2StbusInsreg01(u32);

impl Ahb2StbusInsreg01 {
    const OUT_THRESHOLD: u32 = 0xFFFF << 16;
    const IN_THRESHOLD: u32 = 0xFFFF;

    /// Set the OUT packet buffer threshold (`OUT_THRESHOLD`).
    #[doc(alias = "OUT_THRESHOLD")]
    #[inline]
    pub const fn set_out_threshold(self, value: u16) -> Self {
        Self((self.0 & !Self::OUT_THRESHOLD) | ((value as u32) << 16))
    }
    /// Get the OUT packet buffer threshold.
    #[inline]
    pub const fn out_threshold(self) -> u16 {
        (self.0 >> 16) as u16
    }
    /// Set the IN packet buffer threshold (`IN_THRESHOLD`).
    #[doc(alias = "IN_THRESHOLD")]
    #[inline]
    pub const fn set_in_threshold(self, value: u16) -> Self {
        Self((self.0 & !Self::IN_THRESHOLD) | value as u32)
    }
    /// Get the IN packet buffer threshold.
    #[inline]
    pub const fn in_threshold(self) -> u16 {
        self.0 as u16
    }
}

/// OHCI revision register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OhciRevision(u32);

impl OhciRevision {
    const LEGACY_SUPPORT: u32 = 1 << 8;
    const REVISION: u32 = 0xFF;

    /// Check if legacy support is implemented (`LEGACY_SUPPORT`).
    #[doc(alias = "LEGACY_SUPPORT")]
    #[inline]
    pub const fn has_legacy_support(self) -> bool {
        (self.0 & Self::LEGACY_SUPPORT) != 0
    }
    /// Get OHCI revision in BCD (`REVISION`).
    #[doc(alias = "REVISION")]
    #[inline]
    pub const fn revision(self) -> u8 {
        (self.0 & Self::REVISION) as u8
    }
}

/// OHCI host controller functional state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OhciFunctionalState {
    /// USB reset state.
    Reset = 0,
    /// USB resume state.
    Resume = 1,
    /// USB operational state.
    Operational = 2,
    /// USB suspend state.
    Suspend = 3,
}

/// OHCI control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OhciControl(u32);

impl OhciControl {
    const REMOTE_WAKEUP_ENABLE: u32 = 1 << 10;
    const REMOTE_WAKEUP_CONNECTED: u32 = 1 << 9;
    const INTERRUPT_ROUTING: u32 = 1 << 8;
    const HC_FUNCTIONAL_STATE: u32 = 0x3 << 6;
    const BULK_LIST_ENABLE: u32 = 1 << 5;
    const CONTROL_LIST_ENABLE: u32 = 1 << 4;
    const ISOCHRONOUS_ENABLE: u32 = 1 << 3;
    const PERIODIC_LIST_ENABLE: u32 = 1 << 2;
    const CONTROL_BULK_SERVICE_RATIO: u32 = 0x3;

    /// Enable remote wakeup (`RWE`).
    #[doc(alias = "RWE")]
    #[inline]
    pub const fn enable_remote_wakeup(self) -> Self {
        Self(self.0 | Self::REMOTE_WAKEUP_ENABLE)
    }
    /// Disable remote wakeup.
    #[inline]
    pub const fn disable_remote_wakeup(self) -> Self {
        Self(self.0 & !Self::REMOTE_WAKEUP_ENABLE)
    }
    /// Check if remote wakeup is enabled.
    #[inline]
    pub const fn is_remote_wakeup_enabled(self) -> bool {
        (self.0 & Self::REMOTE_WAKEUP_ENABLE) != 0
    }
    /// Set remote wakeup connected bit (`RWC`).
    #[doc(alias = "RWC")]
    #[inline]
    pub const fn set_remote_wakeup_connected(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::REMOTE_WAKEUP_CONNECTED)
        } else {
            Self(self.0 & !Self::REMOTE_WAKEUP_CONNECTED)
        }
    }
    /// Get remote wakeup connected bit.
    #[inline]
    pub const fn remote_wakeup_connected(self) -> bool {
        (self.0 & Self::REMOTE_WAKEUP_CONNECTED) != 0
    }
    /// Set interrupt routing bit (`IR`).
    #[doc(alias = "IR")]
    #[inline]
    pub const fn set_interrupt_routing(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::INTERRUPT_ROUTING)
        } else {
            Self(self.0 & !Self::INTERRUPT_ROUTING)
        }
    }
    /// Get interrupt routing bit.
    #[inline]
    pub const fn interrupt_routing(self) -> bool {
        (self.0 & Self::INTERRUPT_ROUTING) != 0
    }
    /// Set host controller functional state (`HCFS`).
    #[doc(alias = "HCFS")]
    #[inline]
    pub const fn set_functional_state(self, state: OhciFunctionalState) -> Self {
        Self((self.0 & !Self::HC_FUNCTIONAL_STATE) | ((state as u32) << 6))
    }
    /// Get host controller functional state.
    #[inline]
    pub const fn functional_state(self) -> OhciFunctionalState {
        match ((self.0 & Self::HC_FUNCTIONAL_STATE) >> 6) as u8 {
            0 => OhciFunctionalState::Reset,
            1 => OhciFunctionalState::Resume,
            2 => OhciFunctionalState::Operational,
            _ => OhciFunctionalState::Suspend,
        }
    }
    /// Enable bulk list processing (`BLE`).
    #[doc(alias = "BLE")]
    #[inline]
    pub const fn enable_bulk_list(self) -> Self {
        Self(self.0 | Self::BULK_LIST_ENABLE)
    }
    /// Disable bulk list processing.
    #[inline]
    pub const fn disable_bulk_list(self) -> Self {
        Self(self.0 & !Self::BULK_LIST_ENABLE)
    }
    /// Check if bulk list processing is enabled.
    #[inline]
    pub const fn is_bulk_list_enabled(self) -> bool {
        (self.0 & Self::BULK_LIST_ENABLE) != 0
    }
    /// Enable control list processing (`CLE`).
    #[doc(alias = "CLE")]
    #[inline]
    pub const fn enable_control_list(self) -> Self {
        Self(self.0 | Self::CONTROL_LIST_ENABLE)
    }
    /// Disable control list processing.
    #[inline]
    pub const fn disable_control_list(self) -> Self {
        Self(self.0 & !Self::CONTROL_LIST_ENABLE)
    }
    /// Check if control list processing is enabled.
    #[inline]
    pub const fn is_control_list_enabled(self) -> bool {
        (self.0 & Self::CONTROL_LIST_ENABLE) != 0
    }
    /// Enable isochronous endpoint processing (`IE`).
    #[doc(alias = "IE")]
    #[inline]
    pub const fn enable_isochronous(self) -> Self {
        Self(self.0 | Self::ISOCHRONOUS_ENABLE)
    }
    /// Disable isochronous endpoint processing.
    #[inline]
    pub const fn disable_isochronous(self) -> Self {
        Self(self.0 & !Self::ISOCHRONOUS_ENABLE)
    }
    /// Check if isochronous endpoint processing is enabled.
    #[inline]
    pub const fn is_isochronous_enabled(self) -> bool {
        (self.0 & Self::ISOCHRONOUS_ENABLE) != 0
    }
    /// Enable periodic list processing (`PLE`).
    #[doc(alias = "PLE")]
    #[inline]
    pub const fn enable_periodic_list(self) -> Self {
        Self(self.0 | Self::PERIODIC_LIST_ENABLE)
    }
    /// Disable periodic list processing.
    #[inline]
    pub const fn disable_periodic_list(self) -> Self {
        Self(self.0 & !Self::PERIODIC_LIST_ENABLE)
    }
    /// Check if periodic list processing is enabled.
    #[inline]
    pub const fn is_periodic_list_enabled(self) -> bool {
        (self.0 & Self::PERIODIC_LIST_ENABLE) != 0
    }
    /// Set control-to-bulk service ratio (`CBSR`).
    #[doc(alias = "CBSR")]
    #[inline]
    pub const fn set_control_bulk_service_ratio(self, ratio: u8) -> Self {
        assert!(
            ratio < 4,
            "Control/bulk service ratio out of range (expected 0..=3)"
        );
        Self((self.0 & !Self::CONTROL_BULK_SERVICE_RATIO) | ratio as u32)
    }
    /// Get control-to-bulk service ratio.
    #[inline]
    pub const fn control_bulk_service_ratio(self) -> u8 {
        (self.0 & Self::CONTROL_BULK_SERVICE_RATIO) as u8
    }
}

/// OHCI command status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OhciCommandStatus(u32);

impl OhciCommandStatus {
    const SCHEDULING_OVERRUN_COUNT: u32 = 0x3 << 16;
    const OWNERSHIP_CHANGE_REQUEST: u32 = 1 << 3;
    const BULK_LIST_FILLED: u32 = 1 << 2;
    const CONTROL_LIST_FILLED: u32 = 1 << 1;
    const HOST_CONTROLLER_RESET: u32 = 1;

    /// Get scheduling overrun count (`SOC`).
    #[doc(alias = "SOC")]
    #[inline]
    pub const fn scheduling_overrun_count(self) -> u8 {
        ((self.0 & Self::SCHEDULING_OVERRUN_COUNT) >> 16) as u8
    }
    /// Set ownership change request (`OCR`).
    #[doc(alias = "OCR")]
    #[inline]
    pub const fn set_ownership_change_request(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::OWNERSHIP_CHANGE_REQUEST)
        } else {
            Self(self.0 & !Self::OWNERSHIP_CHANGE_REQUEST)
        }
    }
    /// Get ownership change request bit.
    #[inline]
    pub const fn ownership_change_request(self) -> bool {
        (self.0 & Self::OWNERSHIP_CHANGE_REQUEST) != 0
    }
    /// Set bulk list filled (`BLF`).
    #[doc(alias = "BLF")]
    #[inline]
    pub const fn set_bulk_list_filled(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::BULK_LIST_FILLED)
        } else {
            Self(self.0 & !Self::BULK_LIST_FILLED)
        }
    }
    /// Get bulk list filled bit.
    #[inline]
    pub const fn bulk_list_filled(self) -> bool {
        (self.0 & Self::BULK_LIST_FILLED) != 0
    }
    /// Set control list filled (`CLF`).
    #[doc(alias = "CLF")]
    #[inline]
    pub const fn set_control_list_filled(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::CONTROL_LIST_FILLED)
        } else {
            Self(self.0 & !Self::CONTROL_LIST_FILLED)
        }
    }
    /// Get control list filled bit.
    #[inline]
    pub const fn control_list_filled(self) -> bool {
        (self.0 & Self::CONTROL_LIST_FILLED) != 0
    }
    /// Set host controller reset (`HCR`).
    #[doc(alias = "HCR")]
    #[inline]
    pub const fn set_host_controller_reset(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::HOST_CONTROLLER_RESET)
        } else {
            Self(self.0 & !Self::HOST_CONTROLLER_RESET)
        }
    }
    /// Get host controller reset bit.
    #[inline]
    pub const fn host_controller_reset(self) -> bool {
        (self.0 & Self::HOST_CONTROLLER_RESET) != 0
    }
}

/// OHCI interrupt status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OhciInterruptStatus(u32);

impl OhciInterruptStatus {
    const MASTER_INTERRUPT_ENABLE: u32 = 1 << 31;
    const OWNERSHIP_CHANGE: u32 = 1 << 30;
    const ROOT_HUB_STATUS_CHANGE: u32 = 1 << 6;
    const FRAME_NUMBER_OVERFLOW: u32 = 1 << 5;
    const UNRECOVERABLE_ERROR: u32 = 1 << 4;
    const RESUME_DETECTED: u32 = 1 << 3;
    const START_OF_FRAME: u32 = 1 << 2;
    const WRITEBACK_DONE_HEAD: u32 = 1 << 1;
    const SCHEDULING_OVERRUN: u32 = 1;

    /// Check master interrupt enable (`MIE`).
    ///
    /// The D13x user manual marks bit 31 as reserved, while the generic OHCI
    /// reference defines it as the master interrupt enable.
    #[doc(alias = "MIE")]
    #[inline]
    pub const fn is_master_interrupt_enabled(self) -> bool {
        self.0 & Self::MASTER_INTERRUPT_ENABLE != 0
    }
    /// Check ownership change (`OC`).
    #[doc(alias = "OC")]
    #[inline]
    pub const fn ownership_change(self) -> bool {
        self.0 & Self::OWNERSHIP_CHANGE != 0
    }
    /// Clear ownership change (Write-1-to-Clear).
    #[inline]
    pub const fn clear_ownership_change(self) -> Self {
        Self(self.0 | Self::OWNERSHIP_CHANGE)
    }
    /// Check root hub status change (`RHSC`).
    #[doc(alias = "RHSC")]
    #[inline]
    pub const fn root_hub_status_change(self) -> bool {
        self.0 & Self::ROOT_HUB_STATUS_CHANGE != 0
    }
    /// Clear root hub status change (Write-1-to-Clear).
    #[inline]
    pub const fn clear_root_hub_status_change(self) -> Self {
        Self(self.0 | Self::ROOT_HUB_STATUS_CHANGE)
    }
    /// Check frame number overflow (`FNO`).
    #[doc(alias = "FNO")]
    #[inline]
    pub const fn frame_number_overflow(self) -> bool {
        self.0 & Self::FRAME_NUMBER_OVERFLOW != 0
    }
    /// Clear frame number overflow (Write-1-to-Clear).
    #[inline]
    pub const fn clear_frame_number_overflow(self) -> Self {
        Self(self.0 | Self::FRAME_NUMBER_OVERFLOW)
    }
    /// Check unrecoverable error (`UE`).
    #[doc(alias = "UE")]
    #[inline]
    pub const fn unrecoverable_error(self) -> bool {
        self.0 & Self::UNRECOVERABLE_ERROR != 0
    }
    /// Clear unrecoverable error (Write-1-to-Clear).
    #[inline]
    pub const fn clear_unrecoverable_error(self) -> Self {
        Self(self.0 | Self::UNRECOVERABLE_ERROR)
    }
    /// Check resume detected (`RD`).
    #[doc(alias = "RD")]
    #[inline]
    pub const fn resume_detected(self) -> bool {
        self.0 & Self::RESUME_DETECTED != 0
    }
    /// Clear resume detected (Write-1-to-Clear).
    #[inline]
    pub const fn clear_resume_detected(self) -> Self {
        Self(self.0 | Self::RESUME_DETECTED)
    }
    /// Check start of frame (`SF`).
    #[doc(alias = "SF")]
    #[inline]
    pub const fn start_of_frame(self) -> bool {
        self.0 & Self::START_OF_FRAME != 0
    }
    /// Clear start of frame (Write-1-to-Clear).
    #[inline]
    pub const fn clear_start_of_frame(self) -> Self {
        Self(self.0 | Self::START_OF_FRAME)
    }
    /// Check writeback done head (`WDH`).
    #[doc(alias = "WDH")]
    #[inline]
    pub const fn writeback_done_head(self) -> bool {
        self.0 & Self::WRITEBACK_DONE_HEAD != 0
    }
    /// Clear writeback done head (Write-1-to-Clear).
    #[inline]
    pub const fn clear_writeback_done_head(self) -> Self {
        Self(self.0 | Self::WRITEBACK_DONE_HEAD)
    }
    /// Check scheduling overrun (`SO`).
    #[doc(alias = "SO")]
    #[inline]
    pub const fn scheduling_overrun(self) -> bool {
        self.0 & Self::SCHEDULING_OVERRUN != 0
    }
    /// Clear scheduling overrun (Write-1-to-Clear).
    #[inline]
    pub const fn clear_scheduling_overrun(self) -> Self {
        Self(self.0 | Self::SCHEDULING_OVERRUN)
    }
}

/// OHCI interrupt enable register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OhciInterruptEnable(u32);

impl OhciInterruptEnable {
    const MASTER_INTERRUPT_ENABLE: u32 = 1 << 31;
    const OWNERSHIP_CHANGE: u32 = 1 << 30;
    const ROOT_HUB_STATUS_CHANGE: u32 = 1 << 6;
    const FRAME_NUMBER_OVERFLOW: u32 = 1 << 5;
    const UNRECOVERABLE_ERROR: u32 = 1 << 4;
    const RESUME_DETECTED: u32 = 1 << 3;
    const START_OF_FRAME: u32 = 1 << 2;
    const WRITEBACK_DONE_HEAD: u32 = 1 << 1;
    const SCHEDULING_OVERRUN: u32 = 1;

    /// Enable master interrupts (`MIE`).
    #[doc(alias = "MIE")]
    #[inline]
    pub const fn enable_master_interrupt(self) -> Self {
        Self(self.0 | Self::MASTER_INTERRUPT_ENABLE)
    }
    /// Disable master interrupts.
    #[inline]
    pub const fn disable_master_interrupt(self) -> Self {
        Self(self.0 & !Self::MASTER_INTERRUPT_ENABLE)
    }
    /// Check whether master interrupts are enabled.
    #[inline]
    pub const fn is_master_interrupt_enabled(self) -> bool {
        self.0 & Self::MASTER_INTERRUPT_ENABLE != 0
    }
    /// Enable ownership change interrupt (`OC`).
    #[doc(alias = "OC")]
    #[inline]
    pub const fn enable_ownership_change(self) -> Self {
        Self(self.0 | Self::OWNERSHIP_CHANGE)
    }
    /// Disable ownership change interrupt.
    #[inline]
    pub const fn disable_ownership_change(self) -> Self {
        Self(self.0 & !Self::OWNERSHIP_CHANGE)
    }
    /// Check whether ownership change interrupt is enabled.
    #[inline]
    pub const fn is_ownership_change_enabled(self) -> bool {
        self.0 & Self::OWNERSHIP_CHANGE != 0
    }
    /// Enable root hub status change interrupt (`RHSC`).
    #[doc(alias = "RHSC")]
    #[inline]
    pub const fn enable_root_hub_status_change(self) -> Self {
        Self(self.0 | Self::ROOT_HUB_STATUS_CHANGE)
    }
    /// Disable root hub status change interrupt.
    #[inline]
    pub const fn disable_root_hub_status_change(self) -> Self {
        Self(self.0 & !Self::ROOT_HUB_STATUS_CHANGE)
    }
    /// Check whether root hub status change interrupt is enabled.
    #[inline]
    pub const fn is_root_hub_status_change_enabled(self) -> bool {
        self.0 & Self::ROOT_HUB_STATUS_CHANGE != 0
    }
    /// Enable frame number overflow interrupt (`FNO`).
    #[doc(alias = "FNO")]
    #[inline]
    pub const fn enable_frame_number_overflow(self) -> Self {
        Self(self.0 | Self::FRAME_NUMBER_OVERFLOW)
    }
    /// Disable frame number overflow interrupt.
    #[inline]
    pub const fn disable_frame_number_overflow(self) -> Self {
        Self(self.0 & !Self::FRAME_NUMBER_OVERFLOW)
    }
    /// Check whether frame number overflow interrupt is enabled.
    #[inline]
    pub const fn is_frame_number_overflow_enabled(self) -> bool {
        self.0 & Self::FRAME_NUMBER_OVERFLOW != 0
    }
    /// Enable unrecoverable error interrupt (`UE`).
    #[doc(alias = "UE")]
    #[inline]
    pub const fn enable_unrecoverable_error(self) -> Self {
        Self(self.0 | Self::UNRECOVERABLE_ERROR)
    }
    /// Disable unrecoverable error interrupt.
    #[inline]
    pub const fn disable_unrecoverable_error(self) -> Self {
        Self(self.0 & !Self::UNRECOVERABLE_ERROR)
    }
    /// Check whether unrecoverable error interrupt is enabled.
    #[inline]
    pub const fn is_unrecoverable_error_enabled(self) -> bool {
        self.0 & Self::UNRECOVERABLE_ERROR != 0
    }
    /// Enable resume detected interrupt (`RD`).
    #[doc(alias = "RD")]
    #[inline]
    pub const fn enable_resume_detected(self) -> Self {
        Self(self.0 | Self::RESUME_DETECTED)
    }
    /// Disable resume detected interrupt.
    #[inline]
    pub const fn disable_resume_detected(self) -> Self {
        Self(self.0 & !Self::RESUME_DETECTED)
    }
    /// Check whether resume detected interrupt is enabled.
    #[inline]
    pub const fn is_resume_detected_enabled(self) -> bool {
        self.0 & Self::RESUME_DETECTED != 0
    }
    /// Enable start-of-frame interrupt (`SF`).
    #[doc(alias = "SF")]
    #[inline]
    pub const fn enable_start_of_frame(self) -> Self {
        Self(self.0 | Self::START_OF_FRAME)
    }
    /// Disable start-of-frame interrupt.
    #[inline]
    pub const fn disable_start_of_frame(self) -> Self {
        Self(self.0 & !Self::START_OF_FRAME)
    }
    /// Check whether start-of-frame interrupt is enabled.
    #[inline]
    pub const fn is_start_of_frame_enabled(self) -> bool {
        self.0 & Self::START_OF_FRAME != 0
    }
    /// Enable writeback done head interrupt (`WDH`).
    #[doc(alias = "WDH")]
    #[inline]
    pub const fn enable_writeback_done_head(self) -> Self {
        Self(self.0 | Self::WRITEBACK_DONE_HEAD)
    }
    /// Disable writeback done head interrupt.
    #[inline]
    pub const fn disable_writeback_done_head(self) -> Self {
        Self(self.0 & !Self::WRITEBACK_DONE_HEAD)
    }
    /// Check whether writeback done head interrupt is enabled.
    #[inline]
    pub const fn is_writeback_done_head_enabled(self) -> bool {
        self.0 & Self::WRITEBACK_DONE_HEAD != 0
    }
    /// Enable scheduling overrun interrupt (`SO`).
    #[doc(alias = "SO")]
    #[inline]
    pub const fn enable_scheduling_overrun(self) -> Self {
        Self(self.0 | Self::SCHEDULING_OVERRUN)
    }
    /// Disable scheduling overrun interrupt.
    #[inline]
    pub const fn disable_scheduling_overrun(self) -> Self {
        Self(self.0 & !Self::SCHEDULING_OVERRUN)
    }
    /// Check whether scheduling overrun interrupt is enabled.
    #[inline]
    pub const fn is_scheduling_overrun_enabled(self) -> bool {
        self.0 & Self::SCHEDULING_OVERRUN != 0
    }
}

/// OHCI interrupt disable register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OhciInterruptDisable(u32);

impl OhciInterruptDisable {
    const MASTER_INTERRUPT_ENABLE: u32 = 1 << 31;
    const OWNERSHIP_CHANGE: u32 = 1 << 30;
    const ROOT_HUB_STATUS_CHANGE: u32 = 1 << 6;
    const FRAME_NUMBER_OVERFLOW: u32 = 1 << 5;
    const UNRECOVERABLE_ERROR: u32 = 1 << 4;
    const RESUME_DETECTED: u32 = 1 << 3;
    const START_OF_FRAME: u32 = 1 << 2;
    const WRITEBACK_DONE_HEAD: u32 = 1 << 1;
    const SCHEDULING_OVERRUN: u32 = 1;

    /// Disable master interrupts (`MIE`).
    #[doc(alias = "MIE")]
    #[inline]
    pub const fn disable_master_interrupt(self) -> Self {
        Self(self.0 | Self::MASTER_INTERRUPT_ENABLE)
    }
    /// Disable ownership change interrupt (`OC`).
    #[doc(alias = "OC")]
    #[inline]
    pub const fn disable_ownership_change(self) -> Self {
        Self(self.0 | Self::OWNERSHIP_CHANGE)
    }
    /// Disable root hub status change interrupt (`RHSC`).
    #[doc(alias = "RHSC")]
    #[inline]
    pub const fn disable_root_hub_status_change(self) -> Self {
        Self(self.0 | Self::ROOT_HUB_STATUS_CHANGE)
    }
    /// Disable frame number overflow interrupt (`FNO`).
    #[doc(alias = "FNO")]
    #[inline]
    pub const fn disable_frame_number_overflow(self) -> Self {
        Self(self.0 | Self::FRAME_NUMBER_OVERFLOW)
    }
    /// Disable unrecoverable error interrupt (`UE`).
    #[doc(alias = "UE")]
    #[inline]
    pub const fn disable_unrecoverable_error(self) -> Self {
        Self(self.0 | Self::UNRECOVERABLE_ERROR)
    }
    /// Disable resume detected interrupt (`RD`).
    #[doc(alias = "RD")]
    #[inline]
    pub const fn disable_resume_detected(self) -> Self {
        Self(self.0 | Self::RESUME_DETECTED)
    }
    /// Disable start-of-frame interrupt (`SF`).
    #[doc(alias = "SF")]
    #[inline]
    pub const fn disable_start_of_frame(self) -> Self {
        Self(self.0 | Self::START_OF_FRAME)
    }
    /// Disable writeback done head interrupt (`WDH`).
    #[doc(alias = "WDH")]
    #[inline]
    pub const fn disable_writeback_done_head(self) -> Self {
        Self(self.0 | Self::WRITEBACK_DONE_HEAD)
    }
    /// Disable scheduling overrun interrupt (`SO`).
    #[doc(alias = "SO")]
    #[inline]
    pub const fn disable_scheduling_overrun(self) -> Self {
        Self(self.0 | Self::SCHEDULING_OVERRUN)
    }
}

/// OHCI host controller communication area register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OhciHcca(u32);

impl OhciHcca {
    const HCCA: u32 = 0xFFFFFF << 8;
    /// Set host controller communication area base address (`HCCA`).
    #[doc(alias = "HCCA")]
    #[inline]
    pub const fn set_address(self, address: u32) -> Self {
        assert!(address & 0xFF == 0, "HCCA address must be 256-byte aligned");
        Self((self.0 & !Self::HCCA) | (address & Self::HCCA))
    }
    /// Get host controller communication area base address.
    #[inline]
    pub const fn address(self) -> u32 {
        self.0 & Self::HCCA
    }
}

/// OHCI endpoint descriptor pointer register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OhciEdPointer(u32);

impl OhciEdPointer {
    const ADDRESS: u32 = 0xFFFFFFF << 4;
    /// Set endpoint descriptor address (`ED`).
    #[doc(alias = "ED")]
    #[inline]
    pub const fn set_address(self, address: u32) -> Self {
        assert!(
            address & 0xF == 0,
            "Endpoint descriptor address must be 16-byte aligned"
        );
        Self((self.0 & !Self::ADDRESS) | (address & Self::ADDRESS))
    }
    /// Get endpoint descriptor address.
    #[inline]
    pub const fn address(self) -> u32 {
        self.0 & Self::ADDRESS
    }
}

/// OHCI done head register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OhciDoneHead(u32);

impl OhciDoneHead {
    const ADDRESS: u32 = 0xFFFFFFF << 4;

    /// Set completed transfer descriptor address (`DH`).
    #[doc(alias = "DH")]
    #[inline]
    pub const fn set_address(self, address: u32) -> Self {
        assert!(
            address & 0xF == 0,
            "Done head address must be 16-byte aligned"
        );
        Self((self.0 & !Self::ADDRESS) | (address & Self::ADDRESS))
    }
    /// Get completed transfer descriptor address.
    #[inline]
    pub const fn address(self) -> u32 {
        self.0 & Self::ADDRESS
    }
}

/// OHCI frame interval register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OhciFrameInterval(u32);

impl OhciFrameInterval {
    const FRAME_INTERVAL_TOGGLE: u32 = 1 << 31;
    const FS_LARGEST_DATA_PACKET: u32 = 0x7FFF << 16;
    const FRAME_INTERVAL: u32 = 0x3FFF;

    /// Set frame interval toggle (`FIT`).
    #[doc(alias = "FIT")]
    #[inline]
    pub const fn set_frame_interval_toggle(self, set: bool) -> Self {
        Self(
            (self.0 & !Self::FRAME_INTERVAL_TOGGLE)
                | if set { Self::FRAME_INTERVAL_TOGGLE } else { 0 },
        )
    }
    /// Get frame interval toggle bit.
    #[inline]
    pub const fn frame_interval_toggle(self) -> bool {
        self.0 & Self::FRAME_INTERVAL_TOGGLE != 0
    }
    /// Set largest data packet (`FSMPS`).
    #[doc(alias = "FSMPS")]
    #[inline]
    pub const fn set_fs_largest_data_packet(self, value: u16) -> Self {
        assert!(
            value <= 0x7FFF,
            "Largest data packet out of range (expected 0..=32767)"
        );
        Self((self.0 & !Self::FS_LARGEST_DATA_PACKET) | ((value as u32) << 16))
    }
    /// Get largest data packet.
    #[inline]
    pub const fn fs_largest_data_packet(self) -> u16 {
        ((self.0 & Self::FS_LARGEST_DATA_PACKET) >> 16) as u16
    }
    /// Set frame interval (`FI`).
    #[doc(alias = "FI")]
    #[inline]
    pub const fn set_frame_interval(self, value: u16) -> Self {
        assert!(
            value <= 0x3FFF,
            "Frame interval out of range (expected 0..=16383)"
        );
        Self((self.0 & !Self::FRAME_INTERVAL) | value as u32)
    }
    /// Get frame interval.
    #[inline]
    pub const fn frame_interval(self) -> u16 {
        (self.0 & Self::FRAME_INTERVAL) as u16
    }
}

/// OHCI frame remaining register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OhciFrameRemaining(u32);

impl OhciFrameRemaining {
    const FRAME_REMAINING_TOGGLE: u32 = 1 << 31;
    const FRAME_REMAINING: u32 = 0x3FFF;

    /// Check frame remaining toggle (`FRT`).
    #[doc(alias = "FRT")]
    #[inline]
    pub const fn frame_remaining_toggle(self) -> bool {
        self.0 & Self::FRAME_REMAINING_TOGGLE != 0
    }
    /// Get frame remaining (`FR`).
    #[doc(alias = "FR")]
    #[inline]
    pub const fn frame_remaining(self) -> u16 {
        (self.0 & Self::FRAME_REMAINING) as u16
    }
}

/// OHCI frame number register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OhciFrameNumber(u32);

impl OhciFrameNumber {
    const FRAME_NUMBER: u32 = 0xFFFF;

    /// Get frame number (`FN`).
    #[doc(alias = "FN")]
    #[inline]
    pub const fn frame_number(self) -> u16 {
        (self.0 & Self::FRAME_NUMBER) as u16
    }
}

/// OHCI periodic start register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OhciPeriodicStart(u32);

impl OhciPeriodicStart {
    const PERIODIC_START: u32 = 0x3FFF;

    /// Set periodic start (`PS`).
    #[doc(alias = "PS")]
    #[inline]
    pub const fn set_periodic_start(self, value: u16) -> Self {
        assert!(
            value <= 0x3FFF,
            "Periodic start out of range (expected 0..=16383)"
        );
        Self((self.0 & !Self::PERIODIC_START) | value as u32)
    }
    /// Get periodic start.
    #[inline]
    pub const fn periodic_start(self) -> u16 {
        (self.0 & Self::PERIODIC_START) as u16
    }
}

/// OHCI low-speed threshold register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OhciLsThreshold(u32);

impl OhciLsThreshold {
    const LOW_SPEED_THRESHOLD: u32 = 0xFFF;

    /// Set low-speed threshold (`LST`).
    #[doc(alias = "LST")]
    #[inline]
    pub const fn set_threshold(self, value: u16) -> Self {
        assert!(
            value <= 0xFFF,
            "Low-speed threshold out of range (expected 0..=4095)"
        );
        Self((self.0 & !Self::LOW_SPEED_THRESHOLD) | value as u32)
    }
    /// Get low-speed threshold.
    #[inline]
    pub const fn threshold(self) -> u16 {
        (self.0 & Self::LOW_SPEED_THRESHOLD) as u16
    }
}

/// OHCI root hub descriptor A register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OhciRhDescriptorA(u32);

impl OhciRhDescriptorA {
    const POWER_ON_TO_POWER_GOOD_TIME: u32 = 0xFF << 24;
    const NO_OVER_CURRENT_PROTECTION: u32 = 1 << 12;
    const OVER_CURRENT_PROTECTION_MODE: u32 = 1 << 11;
    const DEVICE_TYPE: u32 = 1 << 10;
    const NO_POWER_SWITCHING: u32 = 1 << 9;
    const POWER_SWITCHING_MODE: u32 = 1 << 8;
    const NUMBER_DOWNSTREAM_PORTS: u32 = 0xFF;

    /// Set power-on-to-power-good time (`POTPGT`).
    #[doc(alias = "POTPGT")]
    #[inline]
    pub const fn set_power_on_to_power_good_time(self, value: u8) -> Self {
        Self((self.0 & !Self::POWER_ON_TO_POWER_GOOD_TIME) | ((value as u32) << 24))
    }
    /// Get power-on-to-power-good time.
    #[inline]
    pub const fn power_on_to_power_good_time(self) -> u8 {
        (self.0 >> 24) as u8
    }
    /// Set no over-current protection (`NOCP`).
    #[doc(alias = "NOCP")]
    #[inline]
    pub const fn set_no_over_current_protection(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::NO_OVER_CURRENT_PROTECTION)
        } else {
            Self(self.0 & !Self::NO_OVER_CURRENT_PROTECTION)
        }
    }
    /// Get no over-current protection bit.
    #[inline]
    pub const fn no_over_current_protection(self) -> bool {
        self.0 & Self::NO_OVER_CURRENT_PROTECTION != 0
    }
    /// Set over-current protection mode (`OCPM`).
    #[doc(alias = "OCPM")]
    #[inline]
    pub const fn set_over_current_protection_mode(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::OVER_CURRENT_PROTECTION_MODE)
        } else {
            Self(self.0 & !Self::OVER_CURRENT_PROTECTION_MODE)
        }
    }
    /// Get over-current protection mode bit.
    #[inline]
    pub const fn over_current_protection_mode(self) -> bool {
        self.0 & Self::OVER_CURRENT_PROTECTION_MODE != 0
    }
    /// Set device type (`DT`).
    #[doc(alias = "DT")]
    #[inline]
    pub const fn set_device_type(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::DEVICE_TYPE)
        } else {
            Self(self.0 & !Self::DEVICE_TYPE)
        }
    }
    /// Get device type bit.
    #[inline]
    pub const fn device_type(self) -> bool {
        self.0 & Self::DEVICE_TYPE != 0
    }
    /// Set no power switching (`NPS`).
    #[doc(alias = "NPS")]
    #[inline]
    pub const fn set_no_power_switching(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::NO_POWER_SWITCHING)
        } else {
            Self(self.0 & !Self::NO_POWER_SWITCHING)
        }
    }
    /// Get no power switching bit.
    #[inline]
    pub const fn no_power_switching(self) -> bool {
        self.0 & Self::NO_POWER_SWITCHING != 0
    }
    /// Set power switching mode (`PSM`).
    #[doc(alias = "PSM")]
    #[inline]
    pub const fn set_power_switching_mode(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::POWER_SWITCHING_MODE)
        } else {
            Self(self.0 & !Self::POWER_SWITCHING_MODE)
        }
    }
    /// Get power switching mode bit.
    #[inline]
    pub const fn power_switching_mode(self) -> bool {
        self.0 & Self::POWER_SWITCHING_MODE != 0
    }
    /// Set downstream port count (`NDP`).
    #[doc(alias = "NDP")]
    #[inline]
    pub const fn set_downstream_port_count(self, value: u8) -> Self {
        Self((self.0 & !Self::NUMBER_DOWNSTREAM_PORTS) | value as u32)
    }
    /// Get downstream port count.
    #[inline]
    pub const fn downstream_port_count(self) -> u8 {
        (self.0 & Self::NUMBER_DOWNSTREAM_PORTS) as u8
    }
}

/// OHCI root hub descriptor B register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OhciRhDescriptorB(u32);

impl OhciRhDescriptorB {
    const PORT_POWER_CONTROL_MASK: u32 = 0xFFFF << 16;
    const DEVICE_REMOVABLE: u32 = 0xFFFF;

    /// Set port power control mask (`PPCM`).
    #[doc(alias = "PPCM")]
    #[inline]
    pub const fn set_port_power_control_mask(self, value: u16) -> Self {
        Self((self.0 & !Self::PORT_POWER_CONTROL_MASK) | ((value as u32) << 16))
    }
    /// Get port power control mask.
    #[inline]
    pub const fn port_power_control_mask(self) -> u16 {
        (self.0 >> 16) as u16
    }
    /// Set device removable mask (`DR`).
    #[doc(alias = "DR")]
    #[inline]
    pub const fn set_device_removable(self, value: u16) -> Self {
        Self((self.0 & !Self::DEVICE_REMOVABLE) | value as u32)
    }
    /// Get device removable mask.
    #[inline]
    pub const fn device_removable(self) -> u16 {
        self.0 as u16
    }
}

/// OHCI root hub status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OhciRhStatus(u32);

impl OhciRhStatus {
    const CLEAR_REMOTE_WAKEUP_ENABLE: u32 = 1 << 31;
    const OVER_CURRENT_INDICATOR_CHANGE: u32 = 1 << 17;
    const SET_GLOBAL_POWER: u32 = 1 << 16;
    const LOCAL_POWER_STATUS_CHANGE: u32 = 1 << 16;
    const DEVICE_REMOTE_WAKEUP_ENABLE: u32 = 1 << 15;
    const OVER_CURRENT_INDICATOR: u32 = 1 << 1;
    const CLEAR_GLOBAL_POWER: u32 = 1;
    const LOCAL_POWER_STATUS: u32 = 1;

    /// Clear remote wakeup enable (`CRWE`).
    #[doc(alias = "CRWE")]
    #[inline]
    pub const fn clear_remote_wakeup_enable(self) -> Self {
        Self(self.0 | Self::CLEAR_REMOTE_WAKEUP_ENABLE)
    }
    /// Check over-current indicator change (`OCIC`).
    #[doc(alias = "OCIC")]
    #[inline]
    pub const fn over_current_indicator_change(self) -> bool {
        self.0 & Self::OVER_CURRENT_INDICATOR_CHANGE != 0
    }
    /// Clear over-current indicator change (Write-1-to-Clear).
    #[inline]
    pub const fn clear_over_current_indicator_change(self) -> Self {
        Self(self.0 | Self::OVER_CURRENT_INDICATOR_CHANGE)
    }
    /// Set global power (`SGP`).
    #[doc(alias = "SGP")]
    #[inline]
    pub const fn set_global_power(self) -> Self {
        Self(self.0 | Self::SET_GLOBAL_POWER)
    }
    /// Check local power status change (`LPSC`).
    #[doc(alias = "LPSC")]
    #[inline]
    pub const fn local_power_status_change(self) -> bool {
        self.0 & Self::LOCAL_POWER_STATUS_CHANGE != 0
    }
    /// Check device remote wakeup enable (`DRWE`).
    #[doc(alias = "DRWE")]
    #[inline]
    pub const fn device_remote_wakeup_enabled(self) -> bool {
        self.0 & Self::DEVICE_REMOTE_WAKEUP_ENABLE != 0
    }
    /// Check over-current indicator (`OCI`).
    #[doc(alias = "OCI")]
    #[inline]
    pub const fn over_current_indicator(self) -> bool {
        self.0 & Self::OVER_CURRENT_INDICATOR != 0
    }
    /// Clear global power (`CGP`).
    #[doc(alias = "CGP")]
    #[inline]
    pub const fn clear_global_power(self) -> Self {
        Self(self.0 | Self::CLEAR_GLOBAL_POWER)
    }
    /// Check local power status (`LPS`).
    #[doc(alias = "LPS")]
    #[inline]
    pub const fn local_power_status(self) -> bool {
        self.0 & Self::LOCAL_POWER_STATUS != 0
    }
}

/// OHCI root hub port status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OhciRhPortStatus(u32);

impl OhciRhPortStatus {
    const PORT_RESET_STATUS_CHANGE: u32 = 1 << 20;
    const PORT_OVER_CURRENT_INDICATOR_CHANGE: u32 = 1 << 19;
    const PORT_SUSPEND_STATUS_CHANGE: u32 = 1 << 18;
    const PORT_ENABLE_STATUS_CHANGE: u32 = 1 << 17;
    const CONNECT_STATUS_CHANGE: u32 = 1 << 16;
    const LOW_SPEED_DEVICE_ATTACHED: u32 = 1 << 9;
    const PORT_POWER_STATUS: u32 = 1 << 8;
    const PORT_RESET_STATUS: u32 = 1 << 4;
    const PORT_OVER_CURRENT_INDICATOR: u32 = 1 << 3;
    const PORT_SUSPEND_STATUS: u32 = 1 << 2;
    const PORT_ENABLE_STATUS: u32 = 1 << 1;
    const CURRENT_CONNECT_STATUS: u32 = 1;

    /// Check port reset status change (`PRSC`).
    #[doc(alias = "PRSC")]
    #[inline]
    pub const fn port_reset_status_change(self) -> bool {
        self.0 & Self::PORT_RESET_STATUS_CHANGE != 0
    }
    /// Clear port reset status change (Write-1-to-Clear).
    #[inline]
    pub const fn clear_port_reset_status_change(self) -> Self {
        Self(self.0 | Self::PORT_RESET_STATUS_CHANGE)
    }
    /// Check port over-current indicator change (`OCIC`).
    #[doc(alias = "OCIC")]
    #[inline]
    pub const fn port_over_current_indicator_change(self) -> bool {
        self.0 & Self::PORT_OVER_CURRENT_INDICATOR_CHANGE != 0
    }
    /// Clear port over-current indicator change (Write-1-to-Clear).
    #[inline]
    pub const fn clear_port_over_current_indicator_change(self) -> Self {
        Self(self.0 | Self::PORT_OVER_CURRENT_INDICATOR_CHANGE)
    }
    /// Check port suspend status change (`PSSC`).
    #[doc(alias = "PSSC")]
    #[inline]
    pub const fn port_suspend_status_change(self) -> bool {
        self.0 & Self::PORT_SUSPEND_STATUS_CHANGE != 0
    }
    /// Clear port suspend status change (Write-1-to-Clear).
    #[inline]
    pub const fn clear_port_suspend_status_change(self) -> Self {
        Self(self.0 | Self::PORT_SUSPEND_STATUS_CHANGE)
    }
    /// Check port enable status change (`PESC`).
    #[doc(alias = "PESC")]
    #[inline]
    pub const fn port_enable_status_change(self) -> bool {
        self.0 & Self::PORT_ENABLE_STATUS_CHANGE != 0
    }
    /// Clear port enable status change (Write-1-to-Clear).
    #[inline]
    pub const fn clear_port_enable_status_change(self) -> Self {
        Self(self.0 | Self::PORT_ENABLE_STATUS_CHANGE)
    }
    /// Check connect status change (`CSC`).
    #[doc(alias = "CSC")]
    #[inline]
    pub const fn connect_status_change(self) -> bool {
        self.0 & Self::CONNECT_STATUS_CHANGE != 0
    }
    /// Clear connect status change (Write-1-to-Clear).
    #[inline]
    pub const fn clear_connect_status_change(self) -> Self {
        Self(self.0 | Self::CONNECT_STATUS_CHANGE)
    }
    /// Check low-speed device attached (`LSDA`).
    #[doc(alias = "LSDA")]
    #[inline]
    pub const fn low_speed_device_attached(self) -> bool {
        self.0 & Self::LOW_SPEED_DEVICE_ATTACHED != 0
    }
    /// Check port power status (`PPS`).
    #[doc(alias = "PPS")]
    #[inline]
    pub const fn port_power_status(self) -> bool {
        self.0 & Self::PORT_POWER_STATUS != 0
    }
    /// Set port power status (`PPS`).
    #[inline]
    pub const fn set_port_power_status(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::PORT_POWER_STATUS)
        } else {
            Self(self.0 & !Self::PORT_POWER_STATUS)
        }
    }
    /// Check port reset status (`PRS`).
    #[doc(alias = "PRS")]
    #[inline]
    pub const fn port_reset_status(self) -> bool {
        self.0 & Self::PORT_RESET_STATUS != 0
    }
    /// Check port over-current indicator (`POCI`).
    #[doc(alias = "POCI")]
    #[inline]
    pub const fn port_over_current_indicator(self) -> bool {
        self.0 & Self::PORT_OVER_CURRENT_INDICATOR != 0
    }
    /// Check port suspend status (`PSS`).
    #[doc(alias = "PSS")]
    #[inline]
    pub const fn port_suspend_status(self) -> bool {
        self.0 & Self::PORT_SUSPEND_STATUS != 0
    }
    /// Check port enable status (`PES`).
    #[doc(alias = "PES")]
    #[inline]
    pub const fn port_enable_status(self) -> bool {
        self.0 & Self::PORT_ENABLE_STATUS != 0
    }
    /// Check current connect status (`CCS`).
    #[doc(alias = "CCS")]
    #[inline]
    pub const fn current_connect_status(self) -> bool {
        self.0 & Self::CURRENT_CONNECT_STATUS != 0
    }
}

/// UTMI host PHY interface width.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HostUtmiInterfaceWidth {
    /// 8-bit interface.
    Bits8,
    /// 16-bit interface.
    Bits16,
}

/// Host PHY interface selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HostPhyInterface {
    /// ULPI interface.
    Ulpi,
    /// UTMI interface.
    Utmi,
}

/// USB Host Control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UsbHostCtl(u32);

impl UsbHostCtl {
    const OHCI_CNTSEL_N: u32 = 1 << 25;
    const SIMULATION_MODE: u32 = 1 << 24;
    const APP_PRT_OVRCUR: u32 = 1 << 19;
    const OHCI_SUSP_LGCY: u32 = 1 << 15;
    const APP_START_CLK: u32 = 1 << 14;
    const AUTO_PPD_ON_OVERCUR: u32 = 1 << 13;
    const ULPI_PP2VBUS: u32 = 1 << 12;
    const EN_INCR16: u32 = 1 << 11;
    const EN_INCR8: u32 = 1 << 10;
    const EN_INCR4: u32 = 1 << 9;
    const EN_INCRX_ALIGN: u32 = 1 << 8;
    const UTMI_WORD_IF: u32 = 1 << 4;
    const ULPI_BYPASS: u32 = 1;

    /// Set OHCI count select bit (`OHCI_CNTSEL_N`).
    #[doc(alias = "OHCI_CNTSEL_N")]
    #[inline]
    pub const fn set_ohci_cntsel_n(self, set: bool) -> Self {
        Self((self.0 & !Self::OHCI_CNTSEL_N) | if set { Self::OHCI_CNTSEL_N } else { 0 })
    }
    /// Get OHCI count select bit.
    #[inline]
    pub const fn ohci_cntsel_n(self) -> bool {
        self.0 & Self::OHCI_CNTSEL_N != 0
    }
    /// Set simulation mode bit (`SIMULATION_MODE`).
    #[doc(alias = "SIMULATION_MODE")]
    #[inline]
    pub const fn set_simulation_mode(self, set: bool) -> Self {
        Self((self.0 & !Self::SIMULATION_MODE) | if set { Self::SIMULATION_MODE } else { 0 })
    }
    /// Get simulation mode bit.
    #[inline]
    pub const fn simulation_mode(self) -> bool {
        self.0 & Self::SIMULATION_MODE != 0
    }
    /// Set application over-current indication bit (`APP_PRT_OVRCUR`).
    #[doc(alias = "APP_PRT_OVRCUR")]
    #[inline]
    pub const fn set_app_prt_ovrcur(self, set: bool) -> Self {
        Self((self.0 & !Self::APP_PRT_OVRCUR) | if set { Self::APP_PRT_OVRCUR } else { 0 })
    }
    /// Get application over-current indication bit.
    #[inline]
    pub const fn app_prt_ovrcur(self) -> bool {
        self.0 & Self::APP_PRT_OVRCUR != 0
    }
    /// Set OHCI suspend legacy mode bit (`OHCI_SUSP_LGCY`).
    #[doc(alias = "OHCI_SUSP_LGCY")]
    #[inline]
    pub const fn set_ohci_susp_lgcy(self, set: bool) -> Self {
        Self((self.0 & !Self::OHCI_SUSP_LGCY) | if set { Self::OHCI_SUSP_LGCY } else { 0 })
    }
    /// Get OHCI suspend legacy mode bit.
    #[inline]
    pub const fn ohci_susp_lgcy(self) -> bool {
        self.0 & Self::OHCI_SUSP_LGCY != 0
    }
    /// Set application start clock bit (`APP_START_CLK`).
    #[doc(alias = "APP_START_CLK")]
    #[inline]
    pub const fn set_app_start_clk(self, set: bool) -> Self {
        Self((self.0 & !Self::APP_START_CLK) | if set { Self::APP_START_CLK } else { 0 })
    }
    /// Get application start clock bit.
    #[inline]
    pub const fn app_start_clk(self) -> bool {
        self.0 & Self::APP_START_CLK != 0
    }
    /// Enable automatic port power-down on over-current (`AUTO_PPD_ON_OVERCUR`).
    #[doc(alias = "AUTO_PPD_ON_OVERCUR")]
    #[inline]
    pub const fn enable_auto_port_power_down_on_overcurrent(self) -> Self {
        Self(self.0 | Self::AUTO_PPD_ON_OVERCUR)
    }
    /// Disable automatic port power-down on over-current.
    #[inline]
    pub const fn disable_auto_port_power_down_on_overcurrent(self) -> Self {
        Self(self.0 & !Self::AUTO_PPD_ON_OVERCUR)
    }
    /// Check if automatic port power-down on over-current is enabled.
    #[inline]
    pub const fn is_auto_port_power_down_on_overcurrent_enabled(self) -> bool {
        self.0 & Self::AUTO_PPD_ON_OVERCUR != 0
    }
    /// Enable PHY port-power to VBUS connection (`ULPI_PP2VBUS`).
    #[doc(alias = "ULPI_PP2VBUS")]
    #[inline]
    pub const fn enable_phy_port_power_to_vbus_connection(self) -> Self {
        Self(self.0 | Self::ULPI_PP2VBUS)
    }
    /// Disable PHY port-power to VBUS connection.
    #[inline]
    pub const fn disable_phy_port_power_to_vbus_connection(self) -> Self {
        Self(self.0 & !Self::ULPI_PP2VBUS)
    }
    /// Check if PHY port-power to VBUS connection is enabled.
    #[inline]
    pub const fn is_phy_port_power_to_vbus_connection_enabled(self) -> bool {
        self.0 & Self::ULPI_PP2VBUS != 0
    }
    /// Enable INCR16 support (`EN_INCR16`).
    #[doc(alias = "EN_INCR16")]
    #[inline]
    pub const fn enable_incr16(self) -> Self {
        Self(self.0 | Self::EN_INCR16)
    }
    /// Disable INCR16 support.
    #[inline]
    pub const fn disable_incr16(self) -> Self {
        Self(self.0 & !Self::EN_INCR16)
    }
    /// Check if INCR16 support is enabled.
    #[inline]
    pub const fn is_incr16_enabled(self) -> bool {
        self.0 & Self::EN_INCR16 != 0
    }
    /// Enable INCR8 support (`EN_INCR8`).
    #[doc(alias = "EN_INCR8")]
    #[inline]
    pub const fn enable_incr8(self) -> Self {
        Self(self.0 | Self::EN_INCR8)
    }
    /// Disable INCR8 support.
    #[inline]
    pub const fn disable_incr8(self) -> Self {
        Self(self.0 & !Self::EN_INCR8)
    }
    /// Check if INCR8 support is enabled.
    #[inline]
    pub const fn is_incr8_enabled(self) -> bool {
        self.0 & Self::EN_INCR8 != 0
    }
    /// Enable INCR4 support (`EN_INCR4`).
    #[doc(alias = "EN_INCR4")]
    #[inline]
    pub const fn enable_incr4(self) -> Self {
        Self(self.0 | Self::EN_INCR4)
    }
    /// Disable INCR4 support.
    #[inline]
    pub const fn disable_incr4(self) -> Self {
        Self(self.0 & !Self::EN_INCR4)
    }
    /// Check if INCR4 support is enabled.
    #[inline]
    pub const fn is_incr4_enabled(self) -> bool {
        self.0 & Self::EN_INCR4 != 0
    }
    /// Enable INCR burst alignment (`EN_INCRX_ALIGN`).
    #[doc(alias = "EN_INCRX_ALIGN")]
    #[inline]
    pub const fn enable_incr_burst_alignment(self) -> Self {
        Self(self.0 | Self::EN_INCRX_ALIGN)
    }
    /// Disable INCR burst alignment.
    #[inline]
    pub const fn disable_incr_burst_alignment(self) -> Self {
        Self(self.0 & !Self::EN_INCRX_ALIGN)
    }
    /// Check if INCR burst alignment is enabled.
    #[inline]
    pub const fn is_incr_burst_alignment_enabled(self) -> bool {
        self.0 & Self::EN_INCRX_ALIGN != 0
    }
    /// Set UTMI interface width (`UTMI_WORD_IF`).
    #[doc(alias = "UTMI_WORD_IF")]
    #[inline]
    pub const fn set_utmi_interface_width(self, width: HostUtmiInterfaceWidth) -> Self {
        Self((self.0 & !Self::UTMI_WORD_IF) | ((width as u32) << 4))
    }
    /// Get UTMI interface width.
    #[inline]
    pub const fn utmi_interface_width(self) -> HostUtmiInterfaceWidth {
        match (self.0 & Self::UTMI_WORD_IF) >> 4 {
            0 => HostUtmiInterfaceWidth::Bits8,
            _ => HostUtmiInterfaceWidth::Bits16,
        }
    }
    /// Set host PHY interface (`ULPI_BYPASS`).
    #[doc(alias = "ULPI_BYPASS")]
    #[inline]
    pub const fn set_phy_interface(self, interface: HostPhyInterface) -> Self {
        Self((self.0 & !Self::ULPI_BYPASS) | interface as u32)
    }
    /// Get host PHY interface.
    #[inline]
    pub const fn phy_interface(self) -> HostPhyInterface {
        match self.0 & Self::ULPI_BYPASS {
            0 => HostPhyInterface::Ulpi,
            _ => HostPhyInterface::Utmi,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::offset_of;

    #[test]
    fn test_register_block_offsets() {
        assert_eq!(offset_of!(RegisterBlock, cap_length_hci_version), 0x000);
        assert_eq!(offset_of!(RegisterBlock, hcs_params), 0x004);
        assert_eq!(offset_of!(RegisterBlock, hcc_params), 0x008);
        assert_eq!(offset_of!(RegisterBlock, hcs_port_route), 0x00C);
        assert_eq!(offset_of!(RegisterBlock, usb_cmd), 0x010);
        assert_eq!(offset_of!(RegisterBlock, usb_sts), 0x014);
        assert_eq!(offset_of!(RegisterBlock, usb_intr), 0x018);
        assert_eq!(offset_of!(RegisterBlock, frame_index), 0x01C);
        assert_eq!(offset_of!(RegisterBlock, ctrl_ds_segment), 0x020);
        assert_eq!(offset_of!(RegisterBlock, periodic_list_base), 0x024);
        assert_eq!(offset_of!(RegisterBlock, async_list_addr), 0x028);
        assert_eq!(offset_of!(RegisterBlock, config_flag), 0x050);
        assert_eq!(offset_of!(RegisterBlock, port_sc), 0x054);
        assert_eq!(offset_of!(RegisterBlock, ahb2stbus_insreg01), 0x094);
        assert_eq!(offset_of!(RegisterBlock, ohci), 0x400);
        assert_eq!(offset_of!(RegisterBlock, usb_host_ctl), 0x800);
    }

    #[test]
    fn test_ohci_register_block_offsets() {
        assert_eq!(offset_of!(OhciRegisterBlock, revision), 0x000);
        assert_eq!(offset_of!(OhciRegisterBlock, control), 0x004);
        assert_eq!(offset_of!(OhciRegisterBlock, command_status), 0x008);
        assert_eq!(offset_of!(OhciRegisterBlock, interrupt_status), 0x00C);
        assert_eq!(offset_of!(OhciRegisterBlock, interrupt_enable), 0x010);
        assert_eq!(offset_of!(OhciRegisterBlock, interrupt_disable), 0x014);
        assert_eq!(offset_of!(OhciRegisterBlock, hcca), 0x018);
        assert_eq!(offset_of!(OhciRegisterBlock, periodic_current_ed), 0x01C);
        assert_eq!(offset_of!(OhciRegisterBlock, control_head_ed), 0x020);
        assert_eq!(offset_of!(OhciRegisterBlock, control_current_ed), 0x024);
        assert_eq!(offset_of!(OhciRegisterBlock, bulk_head_ed), 0x028);
        assert_eq!(offset_of!(OhciRegisterBlock, bulk_current_ed), 0x02C);
        assert_eq!(offset_of!(OhciRegisterBlock, done_head), 0x030);
        assert_eq!(offset_of!(OhciRegisterBlock, frame_interval), 0x034);
        assert_eq!(offset_of!(OhciRegisterBlock, frame_remaining), 0x038);
        assert_eq!(offset_of!(OhciRegisterBlock, frame_number), 0x03C);
        assert_eq!(offset_of!(OhciRegisterBlock, periodic_start), 0x040);
        assert_eq!(offset_of!(OhciRegisterBlock, low_speed_threshold), 0x044);
        assert_eq!(offset_of!(OhciRegisterBlock, root_hub_descriptor_a), 0x048);
        assert_eq!(offset_of!(OhciRegisterBlock, root_hub_descriptor_b), 0x04C);
        assert_eq!(offset_of!(OhciRegisterBlock, root_hub_status), 0x050);
        assert_eq!(offset_of!(OhciRegisterBlock, root_hub_port_status), 0x054);
    }
}
