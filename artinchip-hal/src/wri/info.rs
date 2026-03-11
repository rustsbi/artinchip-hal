//! Reset info.

use super::instance::Wri;
use super::reason::ResetReason;
use super::register::RegisterBlock;

/// Reset info.
pub struct ResetInfo<'a> {
    reg: &'a RegisterBlock,
}

impl<'a> ResetInfo<'a> {
    /// Create a new ResetInfo instance.
    pub fn new(reg: &'a RegisterBlock) -> Self {
        Self { reg }
    }

    /// Get the reset reason.
    pub fn reason(&self) -> ResetReason {
        let flag = self.reg.reset_flag.read();
        let reason = if flag.sys_por_flag() {
            ResetReason::PowerOnReset
        } else if flag.rtc_por_flag() {
            ResetReason::RtcPowerOnReset
        } else if flag.pin_rst_flag() {
            ResetReason::PinReset
        } else if flag.cpu_dm_rst_flag() {
            ResetReason::CpuDebugReset
        } else if flag.wdog_rst_flag() {
            ResetReason::WatchdogReset
        } else if flag.ths_rst_flag() {
            ResetReason::ThermalReset
        } else if flag.cmp_rst_flag() {
            ResetReason::ComparatorReset
        } else {
            unreachable!();
        };

        self.clear_flag(reason);
        reason
    }

    /// Clear the reset flag.
    pub fn clear_flag(&self, reason: ResetReason) {
        unsafe {
            self.reg.reset_flag.modify(|v| match reason {
                ResetReason::PowerOnReset => v.clear_sys_por_flag(),
                ResetReason::RtcPowerOnReset => v.clear_rtc_por_flag(),
                ResetReason::PinReset => v.clear_pin_rst_flag(),
                ResetReason::CpuDebugReset => v.clear_cpu_dm_rst_flag(),
                ResetReason::WatchdogReset => v.clear_wdog_rst_flag(),
                ResetReason::ThermalReset => v.clear_ths_rst_flag(),
                ResetReason::ComparatorReset => v.clear_cmp_rst_flag(),
            });
        }
    }

    /// Clear all flags.
    pub fn clear_all_flags(&self) {
        unsafe {
            self.reg.reset_flag.modify(|v| v.clear_all());
        }
    }

    /// Free the ResetInfo and return WRI instance.
    pub fn free(self) -> Wri {
        Wri::__new(self.reg)
    }
}
