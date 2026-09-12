//! XSPI PSRAM bring-up driver.

use super::config::{Chip, PsramConfig, PsramIoConfig, PsramModel};
use super::error::PsramError;
use super::register::{
    BoundarySize, ClockDivider, CsSel, Icp, IoCfg, LockCfg, PhaseSel, RegisterBlock, XspiMode,
};
use crate::cmu::{Cmu, LdoVoltage};
use crate::sys_cfg::SysCfg;
use embedded_hal::delay::DelayNs;

/// LUT instruction codes (`XSPI_LUTn` instruction field).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum Instruction {
    Stop = 0x00,
    Dummy = 0x10,
    CommandDdr = 0x11,
    AddressDdr = 0x13,
    WriteDdr = 0x14,
    ReadDdr = 0x15,
}

/// TX/RX FIFO depth in 32-bit words.
const FIFO_DEPTH: u8 = 64;

/// Spin budget for the FIFO/transaction polls, so a wedged controller cannot
/// hang the boot.
const POLL_LIMIT: u32 = 1_000_000;

/// XSPI driver created by [`super::XspiExt::new_driver`].
pub struct XspiDriver<'a> {
    /// XSPI register block.
    reg: &'a RegisterBlock,
    /// SoC this driver targets.
    pub chip: Chip,
    /// PSRAM part this driver targets.
    pub model: PsramModel,
    /// Configuration this driver was built with.
    pub cfg: PsramConfig,
}

impl<'a> XspiDriver<'a> {
    /// Create a PSRAM driver for a `chip`/`model` pair and bring it up.
    pub fn __new(
        reg: &'a RegisterBlock,
        chip: Chip,
        model: PsramModel,
        delay: &mut impl DelayNs,
        syscfg: &'a SysCfg,
        cmu: &'a Cmu,
    ) -> Result<Self, PsramError> {
        let cfg = PsramConfig::chip_model(chip, model);

        let parent = parent_hz(cmu);
        let divider = (parent / cfg.clock_hz.0.saturating_mul(2))
            .saturating_sub(1)
            .min(0x1f) as u8;
        let icp = icp_for(parent / (divider as u32 + 1));
        let clock = cmu.register_block();
        let syscfg_reg = syscfg.register_block();
        unsafe {
            // The PLL must have its internal LDO before it can clock a module.
            clock
                .pll_common
                .modify(|v| v.enable_ldo().set_ldo_voltage(LdoVoltage::U1P25V));
            clock.clock_syscfg.modify(|v| v.enable_bus_clk());

            // Reset the XSPI clock domain while programming its divider, then
            // release it and let the dividers settle.
            clock.clock_xspi.modify(|v| {
                v.set_module_clk_div(divider)
                    .enable_module_clk()
                    .enable_bus_clk()
                    .enable_module_reset()
            });
            delay.delay_us(10);
            clock.clock_xspi.modify(|v| v.disable_module_reset());
            delay.delay_us(10);

            // XSPI pad DLL bias current, then the PSRAM I/O rail (voltage first,
            // then enable, so the rail settles before use).
            syscfg_reg
                .ldo25_cfg
                .modify(|v| v.enable_xspi_dllc0_ibias().enable_xspi_dllc1_ibias());
            syscfg_reg
                .ldo18_cfg
                .modify(|v| v.set_ldo18_voltage(cfg.ldo18_voltage));
            syscfg_reg.ldo18_cfg.modify(|v| v.enable_ldo18());
        }
        delay.delay_us(200);

        unsafe {
            reg.clk.modify(|v| {
                v.set_clock_divider(ClockDivider::Divider1)
                    .set_clk_div1(0)
                    .set_clk_div2(0)
            });
            reg.trans_ctrl.modify(|v| {
                v.set_cs_wr_hold(cfg.cs_wr_hold)
                    .set_cs_rd_hold(cfg.cs_rd_hold)
                    .set_clk_pha(cfg.clk_pha)
            });
        }

        for cs in CsSel::ALL {
            configure_io(reg, cs, &cfg.io);
        }

        let mut this = Self {
            reg,
            chip,
            model,
            cfg,
        };

        // OPI mode, split wrap bursts, 1 KiB boundary, single die.
        this.wait_idle();
        unsafe {
            this.reg.ctrl.modify(|v| {
                v.set_xspi_mode(XspiMode::OPI)
                    .set_axi_wrap_burst_ctrl(true)
                    .enable_boundary()
                    .set_boundary_size(BoundarySize::Size1K)
                    .disable_parallel_mode()
            });
        }

        // The DLLs must be up before the module is enabled (vendor order).
        for cs in CsSel::ALL {
            this.set_dll(cs, Icp::Range150To200M, PhaseSel::Deg90, delay);
        }
        this.wait_idle();
        unsafe {
            this.reg.ctrl.modify(|v| v.enable_xspi());
        }

        for &cs in cfg.chip_selects {
            this.wait_idle();
            unsafe {
                this.reg.trans_ctrl.modify(|v| v.set_cs_sel(cs));
            }
            this.program_device(delay)?;
        }

        // The device-reset transactions run with the smaller boundary; XIP needs
        // the 2 KiB one.
        unsafe {
            this.reg
                .ctrl
                .modify(|v| v.enable_boundary().set_boundary_size(BoundarySize::Size2K));
        }
        this.xip_config();
        this.wait_idle();
        unsafe {
            this.reg.ctrl.modify(|v| v.enable_xip());
        }

        for &cs in cfg.chip_selects {
            this.wait_idle();
            unsafe {
                this.reg.trans_ctrl.modify(|v| v.set_cs_sel(cs));
                this.reg.ctrl.modify(|v| v.disable_parallel_mode());
            }
            this.train(cs, icp, delay)?;
        }
        this.wait_idle();
        unsafe {
            this.reg.ctrl.modify(|v| v.enable_parallel_mode());
        }

        Ok(this)
    }

    /// Pattern-test `len` bytes of the XIP window.
    ///
    /// Writes an index pattern and a pointer pattern, and reads both back after
    /// cleaning+invalidating the cache, so a mismatch means PSRAM itself did not
    /// store the value.
    pub fn pattern_test(&self, len: usize) -> bool {
        let base = self.cfg.xip_base;
        let words = len / core::mem::size_of::<u32>();

        for i in 0..words {
            unsafe { ((base + i * 4) as *mut u32).write_volatile(i as u32) };
        }
        unsafe { crate::cache::dcache_clean_invalidate_range(base, len) };
        for i in 0..words {
            if unsafe { ((base + i * 4) as *const u32).read_volatile() } != i as u32 {
                return false;
            }
        }

        for i in 0..words {
            unsafe { ((base + i * 4) as *mut u32).write_volatile((base + i * 4) as u32) };
        }
        unsafe { crate::cache::dcache_clean_invalidate_range(base, len) };
        for i in 0..words {
            if unsafe { ((base + i * 4) as *const u32).read_volatile() } != (base + i * 4) as u32 {
                return false;
            }
        }

        true
    }

    /// Program the DLL input-clock range and delay phase of one chip select.
    fn set_dll(&mut self, cs: CsSel, icp: Icp, phase: PhaseSel, delay: &mut impl DelayNs) {
        self.wait_idle();
        let dctl = match cs {
            CsSel::Cs0 => &self.reg.cs0_dll_ctrl,
            CsSel::Cs1 => &self.reg.cs1_dll_ctrl,
        };
        unsafe {
            dctl.modify(|v| {
                v.set_icp(icp)
                    .set_phase_sel(phase)
                    .enable_ldo()
                    .enable_lvs()
                    .disable_bypass()
            });
        }
        delay.delay_us(5);

        // Bring up DLL, VCDL and charge pump in the vendor's order.
        unsafe {
            dctl.modify(|v| v.enable_dll());
            delay.delay_us(1);
            dctl.modify(|v| v.enable_vcdl());
            delay.delay_us(1);
            dctl.modify(|v| v.enable_cp());
            delay.delay_us(1);
            dctl.modify(|v| v.disable_bypass());
        }
        delay.delay_us(5);
    }

    /// Move one chip select to `phase`, leaving the rest of the DLL setup alone.
    fn set_phase(&mut self, cs: CsSel, phase: PhaseSel) {
        let dctl = match cs {
            CsSel::Cs0 => &self.reg.cs0_dll_ctrl,
            CsSel::Cs1 => &self.reg.cs1_dll_ctrl,
        };
        unsafe {
            dctl.modify(|v| v.set_phase_sel(phase));
        }
    }

    /// Scan the DLL phases of one chip select and settle on the middle of the
    /// working window.
    fn train(&mut self, cs: CsSel, icp: Icp, delay: &mut impl DelayNs) -> Result<u8, PsramError> {
        // The vendor re-arms the DLL at 90 deg right before scanning.
        self.set_dll(cs, icp, PhaseSel::ALL[3], delay);

        let mut rising = None;
        for (i, &phase) in PhaseSel::ALL[..15].iter().enumerate() {
            self.set_phase(cs, phase);
            delay.delay_us(5);
            if self.pattern_test(self.cfg.training_len) {
                rising = Some(i as u8);
                break;
            }
        }
        let Some(rising) = rising else {
            return Err(PsramError::TrainingFailed { chip_select: cs });
        };

        let mut falling = None;
        for (i, &phase) in PhaseSel::ALL[..15].iter().enumerate().rev() {
            self.set_phase(cs, phase);
            delay.delay_us(5);
            if self.pattern_test(self.cfg.training_len) {
                falling = Some(i as u8);
                break;
            }
        }
        let Some(falling) = falling else {
            return Err(PsramError::TrainingFailed { chip_select: cs });
        };

        let settle = (rising + falling) / 2;
        self.set_dll(cs, icp, PhaseSel::ALL[settle as usize], delay);
        Ok(settle)
    }

    /// Reset the device and program its mode registers.
    fn program_device(&mut self, delay: &mut impl DelayNs) -> Result<(), PsramError> {
        // Device reset (`0xFF`, address 0, one payload byte).
        let mut reset = [0u8; 1];
        self.command(0xff, 0x00, 0, false, &mut reset)?;
        delay.delay_us(100);

        // Read `MR4` to pick the write dummy cycles: bit 7 asks the part for the
        // extra latency it needs above 166 MHz, which widens the dummy window.
        let mut mr4 = [0u8; 1];
        self.command(0x40, 0x04, 4, true, &mut mr4)?;
        let dummy = if mr4[0] & 0x80 != 0 { 2 } else { 0 };

        for (address, value) in self.cfg.mode_registers {
            let mut payload = [value];
            self.command(0xc0, address, dummy, false, &mut payload)?;
        }
        Ok(())
    }

    /// Run one LUT-based CPU transfer: `[CMD][ADDR]` then `[DUMMY][READ|WRITE]`.
    ///
    /// `read` selects the direction; `payload` is written or filled in.
    fn command(
        &mut self,
        cmd: u8,
        address: u8,
        dummy: u8,
        read: bool,
        payload: &mut [u8],
    ) -> Result<(), PsramError> {
        if payload.is_empty() {
            return Ok(());
        }
        let len = payload.len();
        let addr_bits = self.cfg.addr_width * 8;

        self.wait_idle();
        self.write_lut(
            0,
            (Instruction::CommandDdr, IoCfg::EightIo, cmd),
            (Instruction::AddressDdr, IoCfg::EightIo, addr_bits),
        );
        self.wait_idle();
        unsafe {
            self.reg.addr.write(address as u32);
        }
        self.write_lut(
            1,
            (Instruction::Dummy, IoCfg::EightIo, dummy),
            (
                if read {
                    Instruction::ReadDdr
                } else {
                    Instruction::WriteDdr
                },
                IoCfg::EightIo,
                (len - 1) as u8,
            ),
        );
        self.write_stop_lut(2);
        self.write_stop_lut(3);

        self.reset_fifo();
        if read {
            self.start();
            self.read_fifo(payload)
        } else {
            self.write_fifo(payload)?;
            self.start();
            self.wait_tx_empty()
        }
    }

    /// Program the XIP read and write LUTs, then publish them.
    fn xip_config(&mut self) {
        let (wr_cmd, wr_dummy, wr_cnt) = self.cfg.xip_write;
        let (rd_cmd, rd_dummy, rd_cnt) = self.cfg.xip_read;
        let addr_bits = self.cfg.addr_width * 8;

        // The LUT bank only accepts writes while it is unlocked.
        self.set_lut_lock(LockCfg::Unlocked);

        // Write transaction: LUT0/LUT1.
        self.write_lut(
            0,
            (Instruction::CommandDdr, IoCfg::EightIo, wr_cmd),
            (Instruction::AddressDdr, IoCfg::EightIo, addr_bits),
        );
        self.write_lut(
            1,
            (Instruction::Dummy, IoCfg::EightIo, wr_dummy),
            (Instruction::WriteDdr, IoCfg::EightIo, (wr_cnt - 1) as u8),
        );
        self.write_stop_lut(2);
        self.write_stop_lut(3);

        // Read transaction: LUT16/LUT17.
        self.write_lut(
            16,
            (Instruction::CommandDdr, IoCfg::EightIo, rd_cmd),
            (Instruction::AddressDdr, IoCfg::EightIo, addr_bits),
        );
        self.write_lut(
            17,
            (Instruction::Dummy, IoCfg::EightIo, rd_dummy),
            (Instruction::ReadDdr, IoCfg::EightIo, (rd_cnt - 1) as u8),
        );
        self.write_stop_lut(18);
        self.write_stop_lut(19);

        unsafe {
            self.reg.lut_up.modify(|v| v.set_lut_update(true));
        }
        self.set_lut_lock(LockCfg::Locked);
    }

    /// Write one two-instruction LUT entry, each half `(instruction, width, operand)`.
    fn write_lut(
        &mut self,
        index: usize,
        high: (Instruction, IoCfg, u8),
        low: (Instruction, IoCfg, u8),
    ) {
        unsafe {
            self.reg.luts[index].modify(|v| {
                v.set_instr1(high.0 as u8)
                    .set_io_cfg1(high.1)
                    .set_operand1(high.2)
                    .set_instr0(low.0 as u8)
                    .set_io_cfg0(low.1)
                    .set_operand0(low.2)
            });
        }
    }

    /// Terminate a LUT sequence at `index`.
    fn write_stop_lut(&mut self, index: usize) {
        self.write_lut(
            index,
            (Instruction::Stop, IoCfg::OneIo, 0),
            (Instruction::Stop, IoCfg::OneIo, 0),
        );
    }

    /// Lock or unlock the LUT bank.
    fn set_lut_lock(&mut self, cfg: LockCfg) {
        unsafe {
            self.reg.lock_config.modify(|v| v.set_lock_cfg(cfg));
        }
    }

    /// Reset both FIFOs.
    fn reset_fifo(&mut self) {
        unsafe {
            self.reg
                .fifo_ctrl
                .modify(|v| v.set_tx_fifo_reset(true).set_rx_fifo_reset(true));
        }
    }

    /// Kick off the LUT0 sequence.
    fn start(&mut self) {
        unsafe {
            self.reg.start.modify(|v| v.set_start_group(0));
        }
    }

    /// Push `payload` into the TX FIFO.
    ///
    /// The port is byte wide, so a 32-bit store would queue four bytes where the
    /// sequence expects one.
    fn write_fifo(&mut self, payload: &[u8]) -> Result<(), PsramError> {
        for &byte in payload {
            let mut spins = 0;
            while self.reg.fifo_status.read().tx_fifo_count() >= FIFO_DEPTH - 8 {
                spins += 1;
                if spins > POLL_LIMIT {
                    log::error!("psram: TX FIFO full timeout");
                    return Err(PsramError::Timeout);
                }
            }
            let ptr = core::ptr::addr_of!(self.reg.tx_data).cast::<u8>() as *mut u8;
            unsafe { ptr.write_volatile(byte) };
        }
        Ok(())
    }

    /// Collect `payload.len()` bytes from the RX FIFO.
    fn read_fifo(&mut self, payload: &mut [u8]) -> Result<(), PsramError> {
        let mut done = 0;
        let mut spins = 0;
        while done < payload.len() {
            // `RX_FIFO_COUNT` is a word count.
            let available = self.reg.fifo_status.read().rx_fifo_count() as usize * 4;
            if available == 0 {
                spins += 1;
                if spins > POLL_LIMIT {
                    log::error!("psram: RX FIFO empty timeout, got {done}/{}", payload.len());
                    return Err(PsramError::Timeout);
                }
                continue;
            }
            for _ in 0..available.min(payload.len() - done) {
                let ptr = core::ptr::addr_of!(self.reg.rx_data).cast::<u8>();
                payload[done] = unsafe { ptr.read_volatile() };
                done += 1;
            }
        }
        Ok(())
    }

    /// Wait for the TX FIFO to drain.
    fn wait_tx_empty(&mut self) -> Result<(), PsramError> {
        let mut spins = 0;
        while self.reg.fifo_status.read().tx_fifo_count() != 0 {
            spins += 1;
            if spins > POLL_LIMIT {
                log::error!("psram: TX not drained timeout");
                return Err(PsramError::Timeout);
            }
        }
        Ok(())
    }

    /// Wait until the controller reports idle.
    fn wait_idle(&mut self) {
        let mut spins = 0;
        while self.reg.status.read().is_busy() {
            spins += 1;
            if spins > POLL_LIMIT {
                log::error!("psram: BUSY stuck, giving up on wait_idle");
                return;
            }
        }
    }
}

/// Program one chip select's IO drive strength and pull settings.
///
/// The vendor asserts the `CSx_IO_CFG` select bit while writing the matching
/// `CSx_IOCFGn` bank and clears it afterwards.
fn configure_io(reg: &RegisterBlock, cs: CsSel, io: &PsramIoConfig) {
    let (cfg1, cfg2, cfg3, cfg4) = match cs {
        CsSel::Cs0 => (
            &reg.cs0_io_cfg1,
            &reg.cs0_io_cfg2,
            &reg.cs0_io_cfg3,
            &reg.cs0_io_cfg4,
        ),
        CsSel::Cs1 => (
            &reg.cs1_io_cfg1,
            &reg.cs1_io_cfg2,
            &reg.cs1_io_cfg3,
            &reg.cs1_io_cfg4,
        ),
    };

    unsafe {
        reg.io_ctrl.modify(|v| match cs {
            CsSel::Cs0 => v.disable_cs0_io(),
            CsSel::Cs1 => v.disable_cs1_io(),
        });
        cfg1.modify(|v| {
            v.set_d7_pin_pull(io.dq.pull)
                .set_d7_pin_drv(io.dq.drv)
                .set_d6_pin_pull(io.dq.pull)
                .set_d6_pin_drv(io.dq.drv)
                .set_d5_pin_pull(io.dq.pull)
                .set_d5_pin_drv(io.dq.drv)
                .set_d4_pin_pull(io.dq.pull)
                .set_d4_pin_drv(io.dq.drv)
        });
        cfg2.modify(|v| {
            v.set_d3_pin_pull(io.dq.pull)
                .set_d3_pin_drv(io.dq.drv)
                .set_d2_pin_pull(io.dq.pull)
                .set_d2_pin_drv(io.dq.drv)
                .set_d1_pin_pull(io.dq.pull)
                .set_d1_pin_drv(io.dq.drv)
                .set_d0_pin_pull(io.dq.pull)
                .set_d0_pin_drv(io.dq.drv)
        });
        cfg3.modify(|v| {
            v.set_cs_pin_pull(io.cs.pull)
                .set_cs_pin_drv(io.cs.drv)
                .set_dqs_pin_pull(io.dqs.pull)
                .set_dqs_pin_drv(io.dqs.drv)
                .set_ck_pin_pull(io.ck.pull)
                .set_ck_pin_drv(io.ck.drv)
                .set_ckn_pin_pull(io.ckn.pull)
                .set_ckn_pin_drv(io.ckn.drv)
        });
        cfg4.modify(|v| v.set_dm_pin_pull(io.dm.pull).set_dm_pin_drv(io.dm.drv));
        reg.io_ctrl.modify(|v| match cs {
            CsSel::Cs0 => v.enable_cs0_io(),
            CsSel::Cs1 => v.enable_cs1_io(),
        });
    }
}

/// XSPI parent clock (`PLL_FRA0`) rate in Hz, or the 24 MHz crystal when the PLL
/// is off (the state the BootROM can leave it in).
fn parent_hz(cmu: &Cmu) -> u32 {
    let general = cmu.register_block().pll_fra0_general.read();
    if !general.is_pll_enabled() {
        return 24_000_000;
    }
    let p = general.factor_p() as u32 + 1;
    let n = general.factor_n() as u32 + 1;
    let m = general.factor_m() as u32 + 1;
    // PLL_O = [24 / (P + 1)] * [(N + 1) / (M + 1)] MHz.
    24_000_000 / p * n / m
}

/// DLL input-clock range for `xspi_hz`.
fn icp_for(xspi_hz: u32) -> Icp {
    if (99_000_000..=198_000_000).contains(&xspi_hz) {
        Icp::Range50To100M
    } else if (198_000_001..=247_500_000).contains(&xspi_hz) {
        Icp::Range100To150M
    } else if (247_500_001..=396_000_000).contains(&xspi_hz) {
        Icp::Range150To200M
    } else {
        Icp::Range50To100M
    }
}
