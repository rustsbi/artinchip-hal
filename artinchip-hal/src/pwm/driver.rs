//! Pwm channel implementation.

use embedded_time::duration::Nanoseconds;
use embedded_time::rate::Hertz;

use super::config::PwmConfig;
use super::instance::PwmChannel;
use super::pad::*;
use super::register::*;
use crate::cmu::Cmu;

/// Pwm channel driver with statically known channel number and pad.
pub struct PwmChDriver<'a, const I: u8, PAD>
where
    PAD: PwmPads<I>,
{
    reg: &'a RegisterBlock,
    pad: PAD,
    config: PwmConfig,
}

impl<'a, const I: u8, PAD> PwmChDriver<'a, I, PAD>
where
    PAD: PwmPads<I>,
{
    const DEFAULT_PWM_CLK: u32 = 48_000_000;

    /// Create a new PWM channel.
    pub fn __new(reg: &'a RegisterBlock, pad: PAD, config: PwmConfig, cmu: &mut Cmu) -> Self {
        let clk = &cmu.register_block().clock_pwm;
        let fix_mod_div = 24;
        if !clk.read().is_bus_clk_enabled() {
            unsafe {
                // Initialize module clock.
                // Reference: https://aicdoc.artinchip.com/topics/ic/cmu/cmu-function2-d13x.html#topic_yvp_f24_4bc__table_qb3_bn5_ydc
                clk.modify(|v| v.set_module_clk_div(fix_mod_div).enable_module_clk());
                clk.modify(|v| v.enable_bus_clk());
                clk.modify(|v| v.enable_module_reset());
                riscv::asm::delay(500);
                clk.modify(|v| v.disable_module_reset());

                // Enable PWM clk.
                reg.ctrl.modify(|v| v.enable());
            }
        }

        unsafe {
            let channel = &reg.channels[I as usize];
            // Enable channel clock.
            reg.ck_ctrl.modify(|v| v.enable_ch_clk(I));
            // Set shadow register load mode.
            channel.cmp_ctrl.modify(|v| {
                v.set_cmp_a_shdw_ld_mode(ShdwLdMode::Mode2)
                    .set_cmp_b_shdw_ld_mode(ShdwLdMode::Mode2)
            });

            // Set clock div.
            let ch_tb_div = (Self::DEFAULT_PWM_CLK.saturating_div(config.tb_clk_rate.0) - 1) as u16;
            channel
                .tb_ctrl
                .modify(|v| v.set_clk_div(ch_tb_div).set_cnt_mode(config.cnt_mode));

            // Set action qualifier control registers.
            if let Some(action) = config.action_0 {
                channel.aq_ctrl_0.modify(|v| {
                    v.set_init_level(config.init_level)
                        .set_cbd_mode(action.cbd)
                        .set_cbu_mode(action.cbu)
                        .set_cad_mode(action.cad)
                        .set_cau_mode(action.cau)
                        .set_prd_mode(action.prd)
                        .set_zro_mode(action.zro)
                });
            }

            if let Some(action) = config.action_1 {
                channel.aq_ctrl_1.modify(|v| {
                    v.set_init_level(config.init_level)
                        .set_cbd_mode(action.cbd)
                        .set_cbu_mode(action.cbu)
                        .set_cad_mode(action.cad)
                        .set_cau_mode(action.cau)
                        .set_prd_mode(action.prd)
                        .set_zro_mode(action.zro)
                });
            }

            reg.int_ctrl.modify(|v| v.disable_ch_int(I));
            reg.int_stat.modify(|v| v.clear_ch_int_pending(I));
        }
        Self { reg, config, pad }
    }

    /// Enable channel.
    pub fn enable(&mut self) {
        unsafe {
            self.reg.m_ctrl.modify(|v| v.enable_ch(I));
        }
    }

    /// Disable channel.
    pub fn disable(&mut self) {
        unsafe {
            self.reg.m_ctrl.modify(|v| v.disable_ch(I));
        }
    }

    /// Convert period in nanoseconds to frequency in Hertz.
    fn period2hertz(&self, period: Nanoseconds) -> Hertz {
        Hertz::new(1_000_000_000 / period.0)
    }

    /// Set period and duty time.
    ///
    /// - `period`: desired period in nanoseconds.
    /// - `duty_a`: desired duty cycle for channel A in nanoseconds.
    /// - `duty_b`: desired duty cycle for channel B in nanoseconds.
    pub fn set_period_and_duty(
        &mut self,
        period: Nanoseconds,
        duty_a: Nanoseconds,
        duty_b: Nanoseconds,
    ) {
        let channel = &self.reg.channels[I as usize];
        self.config.freq = self.period2hertz(period);
        self.config.period = period;
        self.config.duty_a = duty_a;
        self.config.duty_b = duty_b;

        let tb_clk = self.config.tb_clk_rate.0;
        let freq = self.config.freq.0;
        let prd_reg_val = if self.config.cnt_mode == CntMode::CountUpAndDown {
            ((tb_clk / freq) / 2).min(u16::MAX as u32) as u16
        } else {
            ((tb_clk / freq) - 1).min(u16::MAX as u32) as u16
        };

        let cmp_a_val = if prd_reg_val > 0 {
            let cmp = (duty_a.0 as u64 * prd_reg_val as u64) / period.0 as u64;
            if cmp == prd_reg_val as u64 {
                (prd_reg_val as u64 + 1).min(u16::MAX as u64) as u16
            } else {
                cmp as u16
            }
        } else {
            0
        };

        let cmp_b_val = if prd_reg_val > 0 {
            let cmp = (duty_b.0 as u64 * prd_reg_val as u64) / period.0 as u64;
            if cmp == prd_reg_val as u64 {
                (prd_reg_val as u64 + 1).min(u16::MAX as u64) as u16
            } else {
                cmp as u16
            }
        } else {
            0
        };

        unsafe {
            channel.tb_prd.modify(|v| v.set_tb_prd(prd_reg_val));
            channel.cmp_a.modify(|v| v.set_cmp(cmp_a_val));
            channel.cmp_b.modify(|v| v.set_cmp(cmp_b_val));
        }
    }

    /// Set PWM frequency and duty cycle ratio.
    ///
    /// - `freq`: desired frequency in Hertz (must be > 0).
    /// - `ratio_a`: PWM port A duty cycle percentage, clamped to 0.0..100.0.
    /// - `ratio_b`: PWM port B duty cycle percentage, clamped to 0.0..100.0.
    pub fn set_freq_and_ratio(&mut self, freq: Hertz, ratio_a: f32, ratio_b: f32) {
        let ratio_a = ratio_a.clamp(0.0, 100.0);
        let ratio_b = ratio_b.clamp(0.0, 100.0);
        let period_ns = (1_000_000_000 / freq.0).max(1);
        let duty_ns_a = ((period_ns as f32 * ratio_a / 100.0) + 0.5) as u32;
        let duty_ns_b = ((period_ns as f32 * ratio_b / 100.0) + 0.5) as u32;
        let duty_ns_a = duty_ns_a.min(period_ns);
        let duty_ns_b = duty_ns_b.min(period_ns);
        self.config.freq = freq;
        self.config.period = Nanoseconds(period_ns);
        self.config.duty_a = Nanoseconds(duty_ns_a);
        self.config.duty_b = Nanoseconds(duty_ns_b);
        self.set_period_and_duty(self.config.period, self.config.duty_a, self.config.duty_b);
    }

    /// Get current frequency and duty cycle ratio.
    pub fn freq_and_ratio(&self) -> (Hertz, f32, f32) {
        (
            self.config.freq,
            (self.config.duty_a.0 as f32 / self.config.period.0 as f32) * 100.0,
            (self.config.duty_b.0 as f32 / self.config.period.0 as f32) * 100.0,
        )
    }

    /// Get current period and duty cycle.
    pub fn period_and_duty(&self) -> (Nanoseconds, Nanoseconds, Nanoseconds) {
        (self.config.period, self.config.duty_a, self.config.duty_b)
    }

    /// Free the PWM channel and return PwmChannel instance, port A and B pads.
    pub fn free(self, cmu: &Cmu) -> (PwmChannel<I>, PAD) {
        let mut reset_is_needed = true;
        // Check if any channel is still working.
        for ch in 0..4 {
            if self.reg.ck_ctrl.read().is_ch_clk_enabled(ch) {
                reset_is_needed = false;
            }
        }

        if reset_is_needed {
            unsafe {
                let clk = &cmu.register_block().clock_pwm;
                clk.modify(|v| {
                    v.disable_bus_clk()
                        .disable_module_clk()
                        .enable_module_reset()
                });
            }
        }

        (PwmChannel::__new(self.reg), self.pad)
    }
}
