//! Real time.

use super::instance::Rtc;
use super::register::*;
use crate::cmu::Cmu;

/// Real time.
pub struct RealTime<'a> {
    reg: &'a RegisterBlock,
}

impl<'a> RealTime<'a> {
    const WRITE_EN_KEY: u8 = 0xAC;
    /// Create a new RealTime instance.
    pub fn new(reg: &'a RegisterBlock, cmu: &Cmu) -> Self {
        let clk = &cmu.register_block().clock_rtc;
        unsafe {
            // Initialize module clock.
            // Reference: https://aicdoc.artinchip.com/topics/ic/cmu/cmu-function2-d13x.html#topic_yvp_f24_4bc__table_qb3_bn5_ydc
            clk.modify(|v| v.enable_bus_clk());

            // Enable write.
            reg.write_key
                .modify(|v| v.set_write_key(Self::WRITE_EN_KEY));

            // Clear poweroff alarm status.
            reg.aon_int_status.modify(|v| {
                v.clear_alarm_irq_status()
                    .clear_rtc_32k_err_irq_status()
                    .clear_rtc_io_enable_status()
            });

            // Enable RTC.
            reg.ctrl.modify(|v| v.enable_time_cnt());

            // Configure lowpower settings.
            reg.analog0.modify(|v| {
                v.enable_rc1m()
                    .enable_ldo18()
                    .set_ldo18_vol(Ldo1P8Vol::V1_2)
            });
            reg.analog1.modify(|v| {
                v.set_pd_cur_sel(Ldo1P1CurSel::I2_0uA)
                    .set_ldo11_vol(Ldo1P1Vol::V0_80)
                    .enable_ldo11_lp()
            });

            // Disable write.
            reg.write_key.modify(|v| v.set_write_key(0));
        }
        Self { reg }
    }

    /// Set time.
    pub fn set_time(&self, time: u32) {
        unsafe {
            // Enable write.
            self.reg
                .write_key
                .modify(|v| v.set_write_key(Self::WRITE_EN_KEY));

            // Disable RTC.
            self.reg.ctrl.modify(|v| v.disable_time_cnt());

            // Set value.
            for i in 0..4 {
                self.reg.time[i].modify(|v| v.set_time(((time >> (i * 8)) & 0xFF) as u8));
            }

            // Init value.
            self.reg.init.modify(|v| v.set_time_cnt_init(true));

            // Enable RTC.
            self.reg.ctrl.modify(|v| v.enable_time_cnt());

            // Disable write.
            self.reg.write_key.modify(|v| v.set_write_key(0));
        }
    }

    /// Get time.
    pub fn time(&self) -> u32 {
        self.reg.tcnt_val.read()
    }

    /// Set alarm.
    pub fn set_alarm(&self, alarm: u32) {
        unsafe {
            // Enable write.
            self.reg
                .write_key
                .modify(|v| v.set_write_key(Self::WRITE_EN_KEY));

            // Disable alarm.
            self.reg.ctrl.modify(|v| v.disable_alarm());

            // Set value.
            for i in 0..4 {
                self.reg.alarm[i].modify(|v| v.set_alarm(((alarm >> (i * 8)) & 0xFF) as u8));
            }

            // Enable alarm and interrupt.
            self.reg.ctrl.modify(|v| v.enable_alarm());
            self.reg
                .aon_int_en
                .modify(|v| v.enable_alarm_irq().enable_rtc_32k_err_irq());

            // Disable write.
            self.reg.write_key.modify(|v| v.set_write_key(0));
        }
    }

    /// Read alarm.
    pub fn alarm(&self) -> u32 {
        let mut alarm: u32 = 0;
        for i in 0..4 {
            alarm |= (self.reg.alarm[i].read().alarm() as u32) << (i * 8);
        }
        alarm
    }

    /// Free the RealTime and return RTC instance.
    pub fn free(self, cmu: &Cmu) -> Rtc {
        let clk = &cmu.register_block().clock_rtc;
        unsafe {
            // Disable module clock.
            clk.modify(|v| v.disable_bus_clk());
        }
        Rtc::__new(self.reg)
    }
}
