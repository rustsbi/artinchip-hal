//! ArtInChip clock management.

use super::Cmu;
use super::register::{ClockSelectionIntx, PllFraConfig, PllFraSdm, PllGeneral, PllOutput};
use embedded_time::rate::Hertz;
use volatile_register::RW;

/// PLL instance whose output frequency can be read back.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Pll {
    /// CPU PLL (`PLL_INT0`).
    Int0,
    /// Main bus and peripheral PLL (`PLL_INT1`).
    Int1,
    /// Flash / DRAM PLL (`PLL_FRA0`).
    Fra0,
    /// Audio PLL (`PLL_FRA1`). Not present on D13x.
    Fra1,
    /// Display PLL (`PLL_FRA2`).
    Fra2,
}

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
    const VCO_RANGE: (u32, u32) = (Self::VCO_MIN_HZ, Self::VCO_MAX_HZ);
    const FRA2_VCO_MIN_HZ: u32 = 360_000_000;
    const FRA2_VCO_MAX_HZ: u32 = 1_584_000_000;
    /// Hardware fractional divider denominator / modulus (2^17 - 1 = 131,071).
    const FRA_DIV_DENOM: u32 = 0x1_FFFF;

    /// Software delay in cycles after enabling a PLL, replacing the vendor's
    /// 200 us wait because the timer may be clocked by the PLL being changed.
    const SETTLE_CYCLES: u32 = 50_000;

    /// Allowable relative frequency tolerance when checking whether a PLL is
    /// already configured (in parts per million). Defaults to 1,000 PPM (0.1%).
    const FREQ_TOLERANCE_PPM: u32 = 1_000;

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
        self.configure_pll_int0(cmu, self.pll_int0);
        self.configure_pll_int1(cmu, self.pll_int1);
        self.configure_pll_fra0(cmu, self.pll_fra0);
        if let Some(rate) = self.pll_fra1 {
            self.configure_pll_fra1(cmu, rate);
        }
        if let Some(rate) = self.pll_fra2 {
            self.configure_pll_fra2(cmu, rate);
        }
    }

    /// Program `PLL_INT0` (CPU PLL) to `rate`.
    ///
    pub fn configure_pll_int0(&self, cmu: &mut Cmu, rate: Hertz) {
        self.configure_pll(
            &cmu.register_block().pll_int0_general,
            None,
            rate,
            Self::VCO_RANGE,
        );
    }

    /// Switch the CPU from the 24 MHz oscillator to `PLL_INT0` when the BootROM
    /// left it on the oscillator.
    ///
    /// Call this only after consumers that need the BootROM clock state (for
    /// example DRAM PHY training) have finished.
    pub fn switch_cpu_to_pll_int0(&self, cmu: &mut Cmu) {
        let cpu = &cmu.register_block().clock_cpu;
        if cpu.read().bus_clk() == ClockSelectionIntx::Clk24M {
            // SAFETY: `PLL_INT0` has been configured and locked; only the CPU
            // clock mux and divider are changed.
            unsafe {
                cpu.modify(|val| val.set_bus_clk(ClockSelectionIntx::PllIntx).set_clk_div(0));
            }
        }
    }

    /// Program `PLL_INT1` (main bus and peripheral PLL) to `rate`.
    pub fn configure_pll_int1(&self, cmu: &mut Cmu, rate: Hertz) {
        self.configure_pll(
            &cmu.register_block().pll_int1_general,
            None,
            rate,
            Self::VCO_RANGE,
        );
    }

    /// Program `PLL_FRA0` (flash / DRAM PLL) to `rate`.
    pub fn configure_pll_fra0(&self, cmu: &mut Cmu, rate: Hertz) {
        let reg = cmu.register_block();
        self.configure_pll(
            &reg.pll_fra0_general,
            Some((&reg.pll_fra0_config, &reg.pll_fra0_spread_spectrum)),
            rate,
            Self::VCO_RANGE,
        );
    }

    /// Program `PLL_FRA1` (audio PLL) to `rate`.
    pub fn configure_pll_fra1(&self, cmu: &mut Cmu, rate: Hertz) {
        let reg = cmu.register_block();
        self.configure_pll(
            &reg.pll_fra1_general,
            Some((&reg.pll_fra1_config, &reg.pll_fra1_spread_spectrum)),
            rate,
            Self::VCO_RANGE,
        );
    }

    /// Program `PLL_FRA2` (display PLL) to `rate`.
    pub fn configure_pll_fra2(&self, cmu: &mut Cmu, rate: Hertz) {
        let reg = cmu.register_block();
        self.configure_pll(
            &reg.pll_fra2_general,
            Some((&reg.pll_fra2_config, &reg.pll_fra2_spread_spectrum)),
            rate,
            (Self::FRA2_VCO_MIN_HZ, Self::FRA2_VCO_MAX_HZ),
        );
    }

    /// Read the real-time output frequency of `pll` from CMU registers using
    /// this clock tree's oscillator frequency.
    pub fn pll_frequency(&self, cmu: &Cmu, pll: Pll) -> Hertz {
        let reg = cmu.register_block();
        let (general, frac): (&RW<PllGeneral>, Option<&RW<PllFraConfig>>) = match pll {
            Pll::Int0 => (&reg.pll_int0_general, None),
            Pll::Int1 => (&reg.pll_int1_general, None),
            Pll::Fra0 => (&reg.pll_fra0_general, Some(&reg.pll_fra0_config)),
            Pll::Fra1 => (&reg.pll_fra1_general, Some(&reg.pll_fra1_config)),
            Pll::Fra2 => (&reg.pll_fra2_general, Some(&reg.pll_fra2_config)),
        };

        let general = general.read();
        let frac = frac.map_or(0, |fra| {
            let fra = fra.read();
            if fra.is_fra_div_enabled() {
                fra.fra_div()
            } else {
                0
            }
        });

        Hertz(Self::pll_output_rate(
            self.osc_24m.0,
            general.pll_output_sel(),
            general.factor_p(),
            general.factor_n(),
            general.factor_m(),
            frac,
        ))
    }

    /// Read CPU PLL (`PLL_INT0`) real-time output frequency from CMU registers.
    pub fn pll_int0_hz(&self, cmu: &Cmu) -> Hertz {
        self.pll_frequency(cmu, Pll::Int0)
    }

    /// Read main bus and peripheral PLL (`PLL_INT1`) real-time output frequency from CMU registers.
    pub fn pll_int1_hz(&self, cmu: &Cmu) -> Hertz {
        self.pll_frequency(cmu, Pll::Int1)
    }

    /// Read Flash / DRAM PLL (`PLL_FRA0`) real-time output frequency from CMU registers.
    pub fn pll_fra0_hz(&self, cmu: &Cmu) -> Hertz {
        self.pll_frequency(cmu, Pll::Fra0)
    }

    /// Read audio PLL (`PLL_FRA1`) real-time output frequency from CMU registers.
    pub fn pll_fra1_hz(&self, cmu: &Cmu) -> Hertz {
        self.pll_frequency(cmu, Pll::Fra1)
    }

    /// Read display PLL (`PLL_FRA2`) real-time output frequency from CMU registers.
    pub fn pll_fra2_hz(&self, cmu: &Cmu) -> Hertz {
        self.pll_frequency(cmu, Pll::Fra2)
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
        let frac = match fractional {
            Some((fra, _)) => {
                let fra = fra.read();
                if fra.is_fra_div_enabled() {
                    fra.fra_div()
                } else {
                    0
                }
            }
            None => 0,
        };
        // Compare the programmed rate; the mux and enable bits are checked
        // separately below.
        let current = Self::pll_output_rate(
            self.osc_24m.0,
            PllOutput::PllClk,
            val.factor_p(),
            val.factor_n(),
            val.factor_m(),
            frac,
        );

        let diff = current.abs_diff(rate.0);
        let max_diff = ((rate.0 as u64 * Self::FREQ_TOLERANCE_PPM as u64) / 1_000_000) as u32;

        if val.is_pll_enabled()
            && val.is_pll_out2sys_enabled()
            && val.pll_output_sel() == PllOutput::PllClk
            && diff <= max_diff
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
    const fn calculate_pll_params(
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
        if m > 3 {
            m = 3;
        }

        let vco = rate as u64 * (m as u64 + 1);
        if vco < vco_min as u64 || vco > vco_max as u64 {
            return None;
        }

        let p = if !vco.is_multiple_of(oscillator as u64) {
            1
        } else {
            0
        };
        let divisor = (p as u64 + 1) * (m as u64 + 1);
        let scaled = rate as u64 * divisor;
        let n_plus_one = scaled / oscillator as u64;
        let remainder = scaled % oscillator as u64;
        if n_plus_one < 15 || n_plus_one > 200 {
            return None;
        }

        let mut n = n_plus_one - 1;
        let mut frac = 0u32;
        if fractional && remainder != 0 {
            frac = ((remainder * Self::FRA_DIV_DENOM as u64 + oscillator as u64 / 2)
                / oscillator as u64) as u32;
            if frac > Self::FRA_DIV_DENOM {
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

    /// Compute a PLL output rate from the general- and fractional-register fields.
    ///
    /// `oscillator_hz` is the PLL reference; `factor_p`, `factor_n` and `factor_m`
    /// are the raw fields, each one less than the actual divider. `frac` is the FRA
    /// coefficient with a 17-bit denominator. The oscillator bypass is handled
    /// here; spread spectrum is not, because it does not change the mean rate.
    const fn pll_output_rate(
        oscillator_hz: u32,
        output: PllOutput,
        factor_p: u8,
        factor_n: u8,
        factor_m: u8,
        frac: u32,
    ) -> u32 {
        if matches!(output, PllOutput::Osc24M) {
            return oscillator_hz;
        }

        let p = factor_p as u64 + 1;
        let n = factor_n as u64 + 1;
        let m = factor_m as u64 + 1;
        let denom = Self::FRA_DIV_DENOM as u64;

        (oscillator_hz as u64 * (n * denom + frac as u64) / (p * m * denom)) as u32
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
