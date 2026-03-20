//! WDOG driver.

use super::instance::Wdog;
use super::register::*;
use crate::cmu::Cmu;

/// WDOG driver.
pub struct WdogDriver<'a> {
    reg: &'a RegisterBlock,
}

impl<'a> WdogDriver<'a> {
    const OP_CNT_CLR_CODE0: u32 = 0xA1C55555;
    const OP_CNT_CLR_CODE1: u32 = 0xA1CAAAAA;
    const OP_CH_SW_CODE0: u32 = 0xA1C5A5A0;
    const OP_CH_SW_CODE1: u32 = 0xA1CA5A50;
    const OP_WR_EN_CODE0: u32 = 0xA1C99999;
    const OP_WR_EN_CODE1: u32 = 0xA1C66666;
    const FIXED_CLOCK: u32 = 32_000;

    /// Create a new WDOG driver.
    pub fn new(reg: &'a RegisterBlock, cmu: &Cmu) -> Self {
        let clk = &cmu.register_block().clock_wdog;
        unsafe {
            // Initialize module clock.
            // Reference: https://aicdoc.artinchip.com/topics/ic/cmu/cmu-function2-d13x.html#topic_yvp_f24_4bc__table_qb3_bn5_ydc
            clk.modify(|v| v.enable_bus_clk());
            clk.modify(|v| v.enable_module_clk());
            clk.modify(|v| v.enable_wdog_rst());
            riscv::asm::delay(500);
            clk.modify(|v| v.disable_wdog_rst());

            reg.ctrl
                .modify(|v| v.enable_cnt().set_reg_wr_mode(RegWrMode::WriteProtect));
        }

        WdogDriver { reg }
    }

    /// Get current WDOG channel id.
    pub fn channel_id(&self) -> u8 {
        self.reg.ctrl.read().cfg_id()
    }

    /// Set write mode.
    pub fn set_wr_mode(&mut self, mode: RegWrMode) {
        unsafe {
            self.reg.ctrl.modify(|v| v.set_reg_wr_mode(mode));
        }
    }

    /// Get write mode.
    pub fn wr_mode(&self) -> RegWrMode {
        self.reg.ctrl.read().reg_wr_mode()
    }

    /// Execute counter clear operation.
    pub fn op_cnt_clr(&mut self) {
        unsafe {
            self.reg.op.write(Self::OP_CNT_CLR_CODE0);
            self.reg.op.write(Self::OP_CNT_CLR_CODE1);
        }
    }

    /// Execute configuration switch operation.
    pub fn op_cfg_sw(&mut self, cfg: u8) {
        assert!(cfg < 4, "Configuration number must be less than 4.");
        unsafe {
            self.reg.op.write(Self::OP_CH_SW_CODE0 | (cfg as u32));
            self.reg.op.write(Self::OP_CH_SW_CODE1 | (cfg as u32));
        }
    }

    /// Execute write enable operation.
    pub fn op_wr_en(&mut self) {
        unsafe {
            self.reg.op.write(Self::OP_WR_EN_CODE0);
            self.reg.op.write(Self::OP_WR_EN_CODE1);
        }
    }

    /// Convert seconds to hardware counter ticks.
    #[inline]
    fn sec_to_cnt(sec: u32) -> u32 {
        sec.wrapping_mul(Self::FIXED_CLOCK)
    }

    /// Convert hardware counter ticks to seconds.
    #[inline]
    fn cnt_to_sec(cnt: u32) -> u32 {
        cnt / Self::FIXED_CLOCK
    }

    /// Set thresholds for a configuration (CLR, IRQ, RST) in seconds.
    /// CLR < IRQ < RST:
    /// - CLR: lowest threshold; allows CPU to clear the counter (OP_CNT_CLR).
    /// - IRQ: middle threshold; raises a timeout interrupt for recovery.
    /// - RST: highest threshold; causes a hardware reset when exceeded.
    pub fn set_thd(&mut self, cfg: u8, clr_seconds: u32, irq_seconds: u32, rst_seconds: u32) {
        assert!(cfg < 4, "Configuration number must be less than 4.");

        // Convert seconds to hardware ticks.
        let clr_val = Self::sec_to_cnt(clr_seconds);
        let irq_val = Self::sec_to_cnt(irq_seconds);
        let rst_val = Self::sec_to_cnt(rst_seconds);

        unsafe {
            // Write all three threshold registers.
            self.reg.thres[cfg as usize].clr_thres.write(clr_val);
            self.reg.thres[cfg as usize].int_thres.write(irq_val);
            self.reg.thres[cfg as usize].rst_thres.write(rst_val);
        }
    }

    /// Get threshold values for configuration `cfg`.
    /// Returns a tuple of (clear, interrupt, reset) thresholds in seconds.
    pub fn thd(&self, cfg: u8) -> (u32, u32, u32) {
        assert!(cfg < 4, "Configuration number must be less than 4.");
        let clr = Self::cnt_to_sec(self.reg.thres[cfg as usize].clr_thres.read());
        let int = Self::cnt_to_sec(self.reg.thres[cfg as usize].int_thres.read());
        let rst = Self::cnt_to_sec(self.reg.thres[cfg as usize].rst_thres.read());
        (clr, int, rst)
    }

    /// Enable or disable watchdog IRQ (write 1 to enable, 0 to disable).
    pub fn int_enable(&mut self, enable: bool) {
        unsafe {
            if enable {
                self.reg.int_enable.modify(|v| v.enable_tmo_int());
            } else {
                self.reg.int_enable.modify(|v| v.disable_tmo_int());
            }
        }
    }

    /// Read the IRQ enable register.
    pub fn is_int_pending(&self) -> bool {
        self.reg.int_status.read().is_tmo_int_pending()
    }

    /// Clear pending IRQs and return the previous IRQ status.
    pub fn clear_int_pending(&mut self) {
        unsafe {
            self.reg.int_status.modify(|v| v.clear_tmo_int_pending());
        }
    }

    // Apply a full scene atomically and commit it.
    // Sequence:
    // 1) OP_WR_EN  (unlock write window)
    // 2) write CLR/IRQ/RST
    // 3) readback verify (retry once)
    // 4) OP_SW_CFGn (switch to the scene)      <-- move commit BEFORE write-protect
    // 5) OP_CNT_CLR  (clear counter to start new round)
    // 6) set REG_WR_DIS = WriteProtect (protect)  <-- only after commit
    pub fn configure_scene_and_apply(
        &mut self,
        cfg: u8,
        clr_seconds: u32,
        irq_seconds: u32,
        rst_seconds: u32,
    ) {
        assert!(cfg < 4, "Configuration number must be less than 4.");

        let clr_val = Self::sec_to_cnt(clr_seconds);
        let irq_val = Self::sec_to_cnt(irq_seconds);
        let rst_val = Self::sec_to_cnt(rst_seconds);

        // 1) unlock
        self.op_wr_en();

        unsafe {
            // 2) write three registers in the unlocked window
            self.reg.thres[cfg as usize].clr_thres.write(clr_val);
            self.reg.thres[cfg as usize].int_thres.write(irq_val);
            self.reg.thres[cfg as usize].rst_thres.write(rst_val);
        }

        // 3) readback verify; if mismatch, retry once
        unsafe {
            let r_clr = self.reg.thres[cfg as usize].clr_thres.read();
            let r_irq = self.reg.thres[cfg as usize].int_thres.read();
            let r_rst = self.reg.thres[cfg as usize].rst_thres.read();

            if r_clr != clr_val || r_irq != irq_val || r_rst != rst_val {
                // retry: unlock and write again
                self.op_wr_en();
                self.reg.thres[cfg as usize].clr_thres.write(clr_val);
                self.reg.thres[cfg as usize].int_thres.write(irq_val);
                self.reg.thres[cfg as usize].rst_thres.write(rst_val);
            }
        }

        // 4) commit the configuration while still in the logical unlocked/commit phase
        self.op_cfg_sw(cfg);

        // 5) clear counter so the new scene takes effect immediately
        self.op_cnt_clr();

        // 6) optionally restore write-protect for safety
        self.set_wr_mode(RegWrMode::WriteProtect);
    }

    /// Free the driver and return WDOG instance.
    pub fn free(self, cmu: &Cmu) -> Wdog {
        unsafe {
            let clk = &cmu.register_block().clock_wdog;
            clk.modify(|v| v.disable_bus_clk().disable_module_clk().enable_wdog_rst());
        }
        Wdog::__new(self.reg as *const RegisterBlock)
    }
}
