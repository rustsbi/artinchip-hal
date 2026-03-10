//! Timer delay source.

use super::instance::Gtc;
use super::register::{CntFreq, RegisterBlock};
use crate::cmu::Cmu;

/// Timer delay source.
pub struct TimerDelay<'a> {
    reg: &'a RegisterBlock,
}

impl<'a> TimerDelay<'a> {
    /// Create a new TimerDelay instance.
    pub fn new(reg: &'a RegisterBlock, freq: CntFreq, cmu: &Cmu) -> Self {
        let clk = &cmu.register_block().clock_gtc;
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

    /// Free the TimerDelay and return GTC instance.
    pub fn free(self, cmu: &Cmu) -> Gtc {
        unsafe {
            let clk = &cmu.register_block().clock_gtc;
            clk.modify(|v| v.disable_bus_clk().enable_module_reset());
        }
        Gtc::__new(self.reg)
    }
}

impl embedded_hal::delay::DelayNs for TimerDelay<'_> {
    fn delay_ns(&mut self, ns: u32) {
        // Calculate the precision of each tick (in nanoseconds).
        let tick_precision: u64 = match self.reg.cnt_status.read().fcack() {
            CntFreq::Freq4M => 250,     // 4 MHz -> 250 ns per tick.
            CntFreq::Freq1M => 1_000,   // 1 MHz -> 1 µs per tick.
            CntFreq::Freq250k => 4_000, // 250 KHz -> 4 µs per tick.
        };

        // Adjust the requested time 'ns' to the nearest valid non-zero tick_multiple.
        // Ensure a minimum delay of one tick if ns is 0 or smaller than tick_precision.
        let adjusted_ns = if ns == 0 {
            tick_precision // Minimum one tick if ns is 0.
        } else {
            // Round up to the nearest tick multiple using div_ceil.
            (ns as u64).div_ceil(tick_precision) * tick_precision
        };

        // Convert the adjusted delay from nanoseconds to target ticks.
        let target_ticks = adjusted_ns / tick_precision;

        // Save the starting tick value.
        let start_tick = self.get_tick();

        // Poll the counter until the elapsed ticks meet or exceed the target ticks.
        loop {
            let elapsed_tick = self.get_tick().wrapping_sub(start_tick);
            if elapsed_tick >= target_ticks {
                break;
            }
        }
    }
}
