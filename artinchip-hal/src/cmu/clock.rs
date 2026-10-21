//! ArtInChip clock management.

use super::Cmu;
use super::register::{PllFraConfig, PllFraSdm, PllGeneral, PllOutput};
use embedded_time::rate::Hertz;
use volatile_register::RW;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PllParams {
    p: u8,
    n: u8,
    m: u8,
    frac: u32,
}

/// Clock tree.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ClockTree {
    /// PLL input frequency.
    pub osc_24m: Hertz,
    /// CPU PLL.
    pub pll_int0: Hertz,
    /// Main bus and peripheral PLL.
    pub pll_int1: Hertz,
    /// Flash / DRAM PLL.
    pub pll_fra0: Hertz,
    /// Audio PLL. `None` leaves FRA1 untouched (for example on D13x).
    pub pll_fra1: Option<Hertz>,
    /// Display PLL. `None` leaves FRA2 untouched.
    pub pll_fra2: Option<Hertz>,
}

impl ClockTree {
    const VCO_MIN_HZ: u32 = 768_000_000;
    const VCO_MAX_HZ: u32 = 1_560_000_000;
    const FRA2_VCO_MIN_HZ: u32 = 360_000_000;
    const FRA2_VCO_MAX_HZ: u32 = 1_584_000_000;
    const FRAC_DIV_MASK: u32 = 0x1_FFFF;

    /// Software delay in cycles after enabling a PLL, replacing the vendor's
    /// 200 us wait because the timer may be clocked by the PLL being changed.
    const SETTLE_CYCLES: u32 = 50_000;

    /// Create a clock tree using the on-chip 24 MHz oscillator.
    pub const fn new(
        pll_int0: Hertz,
        pll_int1: Hertz,
        pll_fra0: Hertz,
        pll_fra1: Option<Hertz>,
        pll_fra2: Option<Hertz>,
    ) -> Self {
        Self {
            osc_24m: Hertz(24_000_000),
            pll_int0,
            pll_int1,
            pll_fra0,
            pll_fra1,
            pll_fra2,
        }
    }

    /// Program every PLL that is not already running at its requested rate.
    pub fn configure_plls(&self, cmu: &mut Cmu) {
        let reg = cmu.register_block();
        let vco = (Self::VCO_MIN_HZ, Self::VCO_MAX_HZ);

        self.configure_pll(&reg.pll_int0_general, None, self.pll_int0, vco);
        self.configure_pll(&reg.pll_int1_general, None, self.pll_int1, vco);
        self.configure_pll(
            &reg.pll_fra0_general,
            Some((&reg.pll_fra0_config, &reg.pll_fra0_spread_spectrum)),
            self.pll_fra0,
            vco,
        );
        if let Some(rate) = self.pll_fra1 {
            self.configure_pll(
                &reg.pll_fra1_general,
                Some((&reg.pll_fra1_config, &reg.pll_fra1_spread_spectrum)),
                rate,
                vco,
            );
        }
        if let Some(rate) = self.pll_fra2 {
            self.configure_pll(
                &reg.pll_fra2_general,
                Some((&reg.pll_fra2_config, &reg.pll_fra2_spread_spectrum)),
                rate,
                (Self::FRA2_VCO_MIN_HZ, Self::FRA2_VCO_MAX_HZ),
            );
        }
    }

    /// Configure one PLL unless its registers already describe `rate`.
    fn configure_pll(
        &self,
        general: &RW<PllGeneral>,
        fractional: Option<(&RW<PllFraConfig>, &RW<PllFraSdm>)>,
        rate: Hertz,
        vco: (u32, u32),
    ) {
        let val = general.read();
        let p = val.factor_p() as u64 + 1;
        let m = val.factor_m() as u64 + 1;
        let n = val.factor_n() as u64 + 1;
        let frac = match fractional {
            Some((fra, _)) => {
                let fra = fra.read();
                if fra.is_fra_div_enabled() {
                    fra.fra_div() as u64
                } else {
                    0
                }
            }
            None => 0,
        };
        let current = self.osc_24m.0 as u64 * (n * Self::FRAC_DIV_MASK as u64 + frac)
            / (p * m * Self::FRAC_DIV_MASK as u64);

        if val.is_pll_enabled()
            && val.is_pll_out2sys_enabled()
            && val.pll_output_sel() == PllOutput::PllClk
            && current == rate.0 as u64
        {
            return;
        }

        let params =
            Self::calculate_pll_params(rate.0, self.osc_24m.0, vco.0, vco.1, fractional.is_some())
                .expect("PLL rate cannot be generated from the oscillator");

        unsafe {
            general.modify(|val| {
                val.disable_pll()
                    .disable_pll_out2sys()
                    .set_pll_output_sel(PllOutput::Osc24M)
            });
            general.modify(|val| {
                let val = val
                    .set_factor_p(params.p)
                    .set_factor_n(params.n)
                    .set_factor_m(params.m);
                if params.m == 0 {
                    val.disable_facter_m()
                } else {
                    val.enable_facter_m()
                }
            });
            if let Some((fra_config, sdm_config)) = fractional {
                fra_config.modify(|val| {
                    let val = val.set_fra_div(params.frac);
                    if params.frac == 0 {
                        val.disable_fra_div()
                    } else {
                        val.enable_fra_div()
                    }
                });
                sdm_config.modify(|val| val.disable_sdm());
            }
            general.modify(|val| val.enable_pll_reg().enable_pll_out2sys().enable_pll());
        }

        Self::delay_cycles(Self::SETTLE_CYCLES);
        unsafe {
            general.modify(|val| val.set_pll_output_sel(PllOutput::PllClk));
        }
    }

    /// Calculate P/M/N and the FRA coefficient from `rate`.
    fn calculate_pll_params(
        rate: u32,
        oscillator: u32,
        vco_min: u32,
        vco_max: u32,
        fractional: bool,
    ) -> Option<PllParams> {
        if rate == 0 || oscillator == 0 {
            return None;
        }

        let mut m = if rate < vco_min {
            vco_min.div_ceil(rate) - 1
        } else {
            0
        };
        if m == 2 {
            m = 3;
        }
        m = m.min(3);

        let vco = rate as u64 * (m as u64 + 1);
        if vco < vco_min as u64 || vco > vco_max as u64 {
            return None;
        }

        let p = u8::from(!vco.is_multiple_of(oscillator as u64));
        let divisor = (p as u64 + 1) * (m as u64 + 1);
        let scaled = rate as u64 * divisor;
        let n_plus_one = scaled / oscillator as u64;
        let remainder = scaled % oscillator as u64;
        if !(15..=200).contains(&n_plus_one) {
            return None;
        }

        let mut n = n_plus_one - 1;
        let mut frac = 0u32;
        if fractional && remainder != 0 {
            frac = ((remainder * Self::FRAC_DIV_MASK as u64 + oscillator as u64 / 2)
                / oscillator as u64) as u32;
            if frac > Self::FRAC_DIV_MASK {
                frac = 0;
                n += 1;
                if n > 199 {
                    return None;
                }
            }
        }

        Some(PllParams {
            p,
            n: n as u8,
            m: m as u8,
            frac,
        })
    }

    /// Busy-wait for `cycles` `nop` iterations.
    #[inline]
    fn delay_cycles(cycles: u32) {
        for _ in 0..cycles {
            // SAFETY: `nop` has no memory or register side effects.
            unsafe {
                core::arch::asm!("nop", options(nomem, nostack));
            }
        }
    }
}
