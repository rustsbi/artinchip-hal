//! DRAM driver.

use super::config::{DramConfig, DramType};
use super::error::DramError;
use super::instance::Dram;
use super::register::{ControllerMode, ControllerRegisterBlock, PhyRegisterBlock, PhyStatus};
use super::sequence::DramSequencer;
use crate::cmu::{ClockTree, Cmu, SdmFreq, SdmMode};
use crate::sid::{Efuse, Sid};
use crate::sys_cfg::{Ldo1xVoltage, SysCfg, V2};

/// An initialized DRAM controller and PHY.
pub struct DramDriver<'a> {
    controller: &'a ControllerRegisterBlock,
    phy: &'a PhyRegisterBlock,
    dram_base: usize,
}

impl<'a> DramDriver<'a> {
    /// Bring the controller and PHY up for `config`.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        controller: &'a ControllerRegisterBlock,
        phy: &'a PhyRegisterBlock,
        dram_base: usize,
        cmu: &mut Cmu,
        syscfg: &mut SysCfg<V2>,
        sid: &mut Sid,
        config: &DramConfig,
        clock_tree: &ClockTree,
    ) -> Result<Self, DramError> {
        Self::apply_supply(cmu, syscfg, sid, config);
        Self::apply_dram_clock(cmu, config, clock_tree);

        let sequencer = DramSequencer::new(controller, phy);
        sequencer.apply_init_sequence(cmu, config)?;
        sequencer.retrain_read_eye(config.dram_type)?;

        let status = sequencer.report_status()?;
        if !status.is_ok() {
            return Err(DramError::PhyNotReady(status));
        }

        Ok(Self {
            controller,
            phy,
            dram_base,
        })
    }

    /// Base address of the decoded DRAM memory window.
    #[inline]
    pub const fn dram_base(&self) -> usize {
        self.dram_base
    }

    /// Read `MP_PGSR0` and return the PHY's verdict.
    pub fn status(&self) -> Result<PhyStatus, DramError> {
        DramSequencer::new(self.controller, self.phy).report_status()
    }

    /// Probe the DRAM window with write/read test words.
    pub fn probe(&mut self) -> Result<(), DramError> {
        const PROBE_OFFSETS: &[usize] =
            &[0x1000, 0x1004, 0x2000, 0x10000, 0x100000, 0x0, 0x4, 0x40];
        for &offset in PROBE_OFFSETS {
            let address = self.dram_base + offset;
            let expected = !(offset as u32);

            unsafe { core::ptr::write_volatile(address as *mut u32, expected) };
            let got = unsafe { core::ptr::read_volatile(address as *const u32) };

            if got != expected {
                return Err(DramError::ProbeMismatch {
                    address,
                    expected,
                    actual: got,
                });
            }
        }
        Ok(())
    }

    /// Whether the DRAM rail is on and the controller is in normal operating mode.
    pub fn is_up(&self, syscfg: &SysCfg<V2>) -> bool {
        let rail = syscfg.register_block().ldo1x_cfg.read().is_enabled();
        let status = self.controller.status.read();
        rail && status.mode() == ControllerMode::Normal
    }

    /// Release the driver and return the peripheral instance.
    pub fn free(self) -> Dram {
        Dram::__new(self.controller, self.phy, self.dram_base)
    }

    /// Bring up the `VCC_DRAM` power rail in SYSCFG and copy efuse calibration trim to shadow registers.
    fn apply_supply(cmu: &mut Cmu, syscfg: &mut SysCfg<V2>, sid: &mut Sid, config: &DramConfig) {
        unsafe {
            cmu.register_block()
                .clock_syscfg
                .modify(|val| val.enable_bus_clk());
        }

        let voltage = match config.dram_type {
            DramType::Ddr3 => Ldo1xVoltage::Mv1500,
            DramType::Ddr2 => Ldo1xVoltage::Mv1800,
        };

        unsafe {
            syscfg
                .register_block()
                .ldo1x_cfg
                .modify(|val| val.set_val_fb(4).enable().set_voltage(voltage));
        }

        let efuse = Efuse::new(cmu, sid);
        let cali_a = efuse.read(9);
        let cali_b = efuse.read(10);

        if cali_a == 0 {
            return;
        }

        let syscfg = syscfg.register_block();
        unsafe {
            syscfg.ldo30_cfg.modify(|val| val.set_bg_ctrl(cali_b as u8));
            syscfg
                .ldo25_cfg
                .modify(|val| val.set_bg_ctrl((cali_b >> 8) as u8));
            syscfg
                .ddr_rext
                .modify(|val| val.set_res_cal_val((cali_b >> 24) as u8));
            syscfg
                .usb0_rext
                .modify(|val| val.set_res_cal_val((cali_a >> 8) as u8));
            syscfg
                .usb1_rext
                .modify(|val| val.set_res_cal_val(cali_a as u8));
        }
    }

    /// Retune `PLL_FRA0` and bring the DDR subsystem out of reset.
    fn apply_dram_clock(cmu: &mut Cmu, config: &DramConfig, clock_tree: &ClockTree) {
        let clock_ddr = &cmu.register_block().clock_ddr;

        // 1. Put the DDR subsystem back into reset with its clocks off while PLL_FRA0 is retuned.
        unsafe {
            clock_ddr.modify(|val| {
                val
                    // Controller
                    .disable_ctrl_clk()
                    .enable_ctrl_reset()
                    .disable_ctrl_apb_clk()
                    .enable_ctrl_apb_reset()
                    // PHY
                    .disable_phy_module_clk()
                    .enable_phy_reset()
                    .disable_phy_apb_clk()
                    .enable_phy_apb_bus_reset()
                    // Interconnect buses
                    .disable_ahb_clk()
                    .enable_ahb_bus_reset()
                    .disable_axi_clk()
                    .enable_axi_bus_reset()
            });
        }

        // 2. Configure PLL_FRA0 clock frequency and optional spread-spectrum.
        Self::pll_fra0_set(cmu, config.freq, clock_tree.osc_24m.0);
        Self::pll_fra0_spread_spectrum(cmu, config, clock_tree.osc_24m.0);

        // 3. Set CLK_DDR bits 8..19: every reset deasserted, every clock enabled.
        unsafe {
            clock_ddr.modify(|val| {
                val
                    // Controller
                    .enable_ctrl_clk()
                    .disable_ctrl_reset()
                    .enable_ctrl_apb_clk()
                    .disable_ctrl_apb_reset()
                    // PHY
                    .enable_phy_module_clk()
                    .disable_phy_reset()
                    .enable_phy_apb_clk()
                    .disable_phy_apb_bus_reset()
                    // Interconnect buses
                    .enable_ahb_clk()
                    .disable_ahb_bus_reset()
                    .enable_axi_clk()
                    .disable_axi_bus_reset()
            });
        }
    }

    fn pll_fra0_set_multiplier(cmu: &mut Cmu, field: u8) {
        let fra0 = &cmu.register_block().pll_fra0_general;
        unsafe {
            fra0.modify(|val| {
                val.set_factor_p(0)
                    .set_factor_m(0)
                    .set_factor_n(field)
                    .enable_pll_out2sys()
                    .enable_pll_reg()
            });
        }

        for _ in 0..2 {
            core::hint::spin_loop();
        }

        unsafe {
            fra0.modify(|val| val.enable_pll());
        }

        for _ in 0..200 {
            core::hint::spin_loop();
        }
    }

    fn pll_fra0_set(cmu: &mut Cmu, clock_hz: u32, osc_hz: u32) {
        let mhz = clock_hz / 1_000_000;
        let osc_mhz = (osc_hz / 1_000_000).max(1);

        let field = if clock_hz > 504_999_999 {
            clock_hz / osc_hz - 1
        } else if clock_hz > 264_999_999 {
            (mhz * 2) / osc_mhz - 1
        } else {
            (mhz * 3) / osc_mhz - 1
        };

        if !(14..=199).contains(&field) {
            return;
        }

        Self::pll_fra0_set_multiplier(cmu, field as u8);

        match mhz {
            528 => Self::pll_fra0_set_multiplier(cmu, 32),
            672 => Self::pll_fra0_set_multiplier(cmu, 41),
            _ => {}
        }

        for _ in 0..1000 {
            core::hint::spin_loop();
        }

        let achieved = Self::pll_fra0_hz(cmu, osc_hz);
        let divider = if achieved >= clock_hz {
            (achieved as u64 * 2 / clock_hz as u64 - 1) as u32
        } else {
            0
        };

        let clock_ddr = &cmu.register_block().clock_ddr;
        unsafe {
            clock_ddr.modify(|val| val.set_module_clk_div(divider as u8));
        }
    }

    fn pll_fra0_spread_spectrum(cmu: &mut Cmu, config: &DramConfig, osc_hz: u32) {
        if !config.ssc {
            return;
        }

        let regs = cmu.register_block();
        let fra0 = &regs.pll_fra0_general;
        let reference = osc_hz / (fra0.read().factor_p() as u32 + 1);
        let arg = Self::pll_fra0_hz(cmu, osc_hz) / 100;

        let fine = if arg < reference {
            0x2_0000 - ((arg as u64 * 131_072) / reference as u64) as u32
        } else {
            0
        };
        let step = (((0x2_0000 - fine) as u64 * 66_000 / osc_hz as u64) as u32).min(511);
        let fine = fine.min(0x1_FFFF);

        unsafe {
            fra0.modify(|val| val.set_pll_icp(0));
            regs.pll_fra0_spread_spectrum.modify(|val| {
                val.enable_sdm()
                    .set_sdm_mode(SdmMode::Triangular)
                    .set_sdm_freq(SdmFreq::F33P0KHz)
                    .set_sdm_step(step as u16)
                    .set_sdm_bottom(fine)
            });
        }
    }

    fn pll_fra0_hz(cmu: &Cmu, osc_hz: u32) -> u32 {
        let reg = cmu.register_block();
        let general = reg.pll_fra0_general.read();
        let fra = reg.pll_fra0_config.read();
        let frac = if fra.is_fra_div_enabled() {
            fra.fra_div()
        } else {
            0
        };
        let p = general.factor_p() as u64 + 1;
        let m = general.factor_m() as u64 + 1;
        let n = general.factor_n() as u64 + 1;
        let base = (osc_hz as u64) * n / (p * m);
        let frac_part = (osc_hz as u64) * (frac as u64) / (p * m * (1 << 24));
        (base + frac_part) as u32
    }
}
