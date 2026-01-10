//! Timer delay source.

use super::register::{CntFreq, RegisterBlock};
use crate::cmu;

/// Timer delay source.
pub struct TimerDelay<'a> {
    reg: &'a RegisterBlock,
}

impl<'a> TimerDelay<'a> {
    /// Create a new TimerDelay instance.
    pub fn new(reg: &'a RegisterBlock, freq: CntFreq, cmu: &cmu::RegisterBlock) -> Self {
        let clk = &cmu.clock_gtc;
        unsafe {
            // Initialize module clock.
            // Reference: https://aicdoc.artinchip.com/topics/ic/cmu/cmu-function2-d13x.html#topic_yvp_f24_4bc__table_qb3_bn5_ydc
            clk.modify(|v| v.enable_bus_clk());
            clk.modify(|v| v.enable_module_reset());
            riscv::asm::delay(500);
            clk.modify(|v| v.disable_module_reset());

            // Configure GTC.
            reg.config.modify(|cfg| cfg.set_fdiv(6));
            reg.cnt_ctrl
                .modify(|ctrl| ctrl.disable_cnt_on_dbg().set_freq(freq).enable_cnt());
        }
        Self { reg }
    }

    /// Get current tick.
    pub fn get_tick(&self) -> u64 {
        let low = self.reg.cnt_value_low.read() as u64;
        let high = ((self.reg.cnt_value_high.read().cnt_value_high() & 0xFFFFF) as u64) << 32;
        high | low
    }

    /// Delay us.
    pub fn delay_us(&self, us: u64) {
        let one_sec_tick: u64 = match self.reg.cnt_status.read().fcack() {
            CntFreq::Freq4M => 4_000_000,
            CntFreq::Freq1M => 1_000_000,
            CntFreq::Freq250k => 250_000,
        };

        let start_tick = self.get_tick();
        let target_ticks = us * one_sec_tick / 1_000_000;

        loop {
            let elapsed_tick = self.get_tick().wrapping_sub(start_tick);
            if elapsed_tick >= target_ticks {
                break;
            }
        }
    }

    /// Delay ms.
    pub fn delay_ms(&self, ms: u64) {
        self.delay_us(ms * 1_000);
    }
}
