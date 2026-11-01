//! DRAM controller and PHY initialization sequence.
//!
//! The register values below were captured from a working board, not taken from a
//! public vendor sequence.

use crate::cmu::Cmu;

use super::config::{DramConfig, DramType};
use super::error::DramError;
use super::register::*;

/// One controller port configuration set.
#[derive(Clone, Copy)]
struct PortProfile {
    config_read: PortConfigRead,
    read_qos: PortReadQos0,
    config_write: PortConfigWrite,
    write_qos: PortWriteQos0,
}

impl PortProfile {
    fn default_port() -> Self {
        Self {
            config_read: PortConfigRead::from_bits(0x0010_0E0C),
            read_qos: PortReadQos0::from_bits(0x000F_000F),
            config_write: PortConfigWrite::from_bits(0x0110_0E0C),
            write_qos: PortWriteQos0::from_bits(0x000F_000F),
        }
    }
}

/// Target-specific DRAM timing, mode registers, and PHY parameters.
struct DramHardwareProfile {
    init0: u32,
    init4: u32,
    init5: u32,
    master_control: u32,
    refresh0: u32,
    timing0: u32,
    timing1: u32,
    timing2: u32,
    timing3: u32,
    timing4: u32,
    timing5: u32,
    timing8: u32,
    mapping0: u32,
    dfi_timing0_ports: u32,
    dfi_timing0_phy: u32,
    phy_mr0: u32,
    phy_mr2: u32,
    phy_dcr: u32,
    phy_dtpr0: u32,
    phy_dtpr1: u32,
    phy_dtpr2: u32,
    phy_ptr0: u32,
    phy_ptr1: u32,
    dsgcr_bits: u32,
    zq_impedance: u8,
    pll_frequency_select: bool,
    pir_init: u32,
    lcdlr0_preserve_lsb: bool,
    lcdlr0_pass1_bits: u32,
    lcdlr1_and: u32,
    lcdlr0_pass2_bits: u32,
}

impl DramHardwareProfile {
    fn ddr3() -> Self {
        Self {
            // Controller init words: skip timer/precharge delays.
            init0: 0x0008_0808,
            init4: 0x0707_0707,
            init5: 0x0F0F_0F07,
            // Master control: 16-bit DRAM, BL8.
            master_control: 0x0004_0401,
            refresh0: 0x0600_0400,
            // Controller core timings (tRAS/tRC/tRCD/tRP/...).
            timing0: 0x0B0F_0A0C,
            timing1: 0x0008_0310,
            timing2: 0x0000_0509,
            timing3: 0x0000_400C,
            timing4: 0x0502_0305,
            timing5: 0x0505_0403,
            timing8: 0x0000_0803,
            // Address decode and DFI timing.
            mapping0: 0x0204_0102,
            dfi_timing0_ports: 0x0024_004E,
            dfi_timing0_phy: 0x0024_004E,
            // PHY mode registers MR0/MR2.
            phy_mr0: 0x0000_1A70,
            phy_mr2: 0x0000_0018,
            // PHY timing and clock control (DCR, DTPR0..2, PTR0..1).
            phy_dcr: 0x1000_040B,
            phy_dtpr0: 0x8198_AA66,
            phy_dtpr1: 0x2284_E3D8,
            phy_dtpr2: 0xD003_4200,
            phy_ptr0: 0x0A34_93E1,
            phy_ptr1: 0x0965_D4C1,
            // PHY update control, ZQ impedance and PLL selection.
            dsgcr_bits: 0x8000_0924,
            zq_impedance: 0x5D,
            pll_frequency_select: false,
            // PHY training request.
            pir_init: 0x0000_07F2,
            // Data-slice leveling delay-line values.
            lcdlr0_preserve_lsb: true,
            lcdlr0_pass1_bits: 0xFFFF_FFF8,
            lcdlr1_and: 0xFFFF_FFF9,
            lcdlr0_pass2_bits: 0xFFFF_F9F8,
        }
    }

    fn ddr2() -> Self {
        Self {
            // Controller init words: skip timer/precharge delays.
            init0: 0x003F_0808,
            init4: 0x0606_0606,
            init5: 0x0F0F_0F06,
            // Master control: 16-bit DRAM, BL8.
            master_control: 0x0004_0400,
            refresh0: 0x0500_0400,
            // Controller core timings (tRAS/tRC/tRCD/tRP/...).
            timing0: 0x090D_110D,
            timing1: 0x0005_0411,
            timing2: 0x0000_0408,
            timing3: 0x0000_200C,
            timing4: 0x0602_0306,
            timing5: 0x0505_0303,
            timing8: 0x0000_0402,
            // Address decode and DFI timing.
            mapping0: 0x0202_0101,
            dfi_timing0_ports: 0x001E_0033,
            dfi_timing0_phy: 0x001E_0042,
            // PHY mode registers MR0/MR2.
            phy_mr0: 0x0000_0E73,
            phy_mr2: 0x0000_0000,
            // PHY timing and clock control (DCR, DTPR0..2, PTR0..1).
            phy_dcr: 0x1000_0402,
            phy_dtpr0: 0x899A_CC68,
            phy_dtpr1: 0x2283_3354,
            phy_dtpr2: 0xC803_2900,
            phy_ptr0: 0x0CA1_89C1,
            phy_ptr1: 0x07E5_89C1,
            // PHY update control, ZQ impedance and PLL selection.
            dsgcr_bits: 0x8000_07AE,
            zq_impedance: 0x4B,
            pll_frequency_select: true,
            // PHY training request.
            pir_init: 0x0000_0572,
            // Data-slice leveling delay-line values.
            lcdlr0_preserve_lsb: true,
            lcdlr0_pass1_bits: 0xFFFF_FFF6,
            lcdlr1_and: 0xFFFF_FFF5,
            lcdlr0_pass2_bits: 0xFFFF_F5F6,
        }
    }

    fn lcdlr0(&self, current: u32, second_pass: bool) -> u32 {
        let bits = if second_pass {
            self.lcdlr0_pass2_bits
        } else {
            self.lcdlr0_pass1_bits
        };
        if self.lcdlr0_preserve_lsb {
            (current & 0x1) | bits
        } else {
            bits
        }
    }
}

/// DRAM initialization sequencer coordinating controller and PHY register programming.
pub(crate) struct DramSequencer<'a> {
    controller: &'a ControllerRegisterBlock,
    phy: &'a PhyRegisterBlock,
}

impl<'a> DramSequencer<'a> {
    /// Create a new sequencer for controller and PHY blocks.
    pub const fn new(controller: &'a ControllerRegisterBlock, phy: &'a PhyRegisterBlock) -> Self {
        Self { controller, phy }
    }

    /// Check whether the specified configuration has a verified initialization sequence.
    pub const fn is_supported_config(config: &DramConfig) -> bool {
        matches!(
            (config.dram_type, config.freq),
            (DramType::Ddr3, 600_000_000) | (DramType::Ddr2, 504_000_000)
        )
    }

    /// Execute the full controller and PHY initialization sequence.
    pub fn apply_init_sequence(&self, cmu: &mut Cmu, config: &DramConfig) -> Result<(), DramError> {
        if !Self::is_supported_config(config) {
            return Err(DramError::UnsupportedConfig {
                dram_type: config.dram_type,
                freq: config.freq,
            });
        }

        let profile = match config.dram_type {
            DramType::Ddr3 => DramHardwareProfile::ddr3(),
            DramType::Ddr2 => DramHardwareProfile::ddr2(),
        };

        // Stage 1: Disable all controller ports before reconfiguration
        self.set_ports_enabled(false);

        // Stage 2: Hold controller core and interconnects in reset during port mapping
        Self::set_controller_reset(cmu, true);
        self.configure_controller_ports(&profile)?;
        Self::set_controller_reset(cmu, false);

        // Stage 3: Configure timing, mode registers, PHY, and calibrate
        self.configure_timing_and_phy(&profile)?;

        // Stage 4: Re-enable controller ports
        self.set_ports_enabled(true);

        Ok(())
    }

    fn set_ports_enabled(&self, enabled: bool) {
        let value = PortEnable::from_bits(u32::from(enabled));
        unsafe {
            for port in &self.controller.ports {
                port.enable.write(value);
            }
        }
    }

    #[inline]
    fn open_controller_command(&self) {
        unsafe {
            self.controller.command.write(CommandControl::idle());
        }
    }

    #[inline]
    fn commit_controller_command(&self) -> Result<(), DramError> {
        unsafe {
            self.controller.command.write(CommandControl::commit());
        }
        if !self.poll_until(
            || self.controller.command_status.read().is_done(),
            1_000_000,
        ) {
            return Err(DramError::CommandTimeout);
        }
        Ok(())
    }

    #[inline]
    fn commit_and_reopen_controller_command(&self) -> Result<(), DramError> {
        self.commit_controller_command()?;
        self.open_controller_command();
        Ok(())
    }

    #[inline]
    fn execute_phy_init_request(&self, request: PhyInitRequest) -> Result<(), DramError> {
        unsafe {
            self.phy.pir.write(request);
            self.phy.pir.write(request.enable_init());
        }
        if !self.poll_until(|| self.phy.pgsr0.read().is_request_done(), 100_000_000) {
            return Err(DramError::PhyInitTimeout);
        }
        Ok(())
    }

    fn enable_address_remap(&self) {
        unsafe {
            self.controller
                .address_map2
                .modify(|v| (v & 0x3FFF_FFFF) | 0xC000_0000);
            self.controller
                .address_map
                .modify(|v| (v & 0x3FFF_FFFF) | 0xC000_0000);
        }
    }

    /// Apply the DFI mode/calibration writes captured from the board.
    fn configure_mode_calibration(&self) {
        unsafe {
            self.controller.mode[0].modify(|v| (v & !0x1) | 0x1);
            self.controller.mode[2].modify(|v| (v & 0xFFFF_EFFF) | 0x1000);
            self.controller.mode[2].modify(|v| (v & 0xFFFF_EC00) | 0x1000);
            self.controller.mode[2].modify(|v| (v & 0xFFFF_EC00) | 0x0000_1004);
            self.controller.ports[1].calibration[1].modify(|v| (v & 0xFFFF_EFFF) | 0x1000);
            self.controller.ports[1].calibration[1].modify(|v| (v & 0xFFFF_EC00) | 0x1000);
            self.controller.ports[1].calibration[1].modify(|v| (v & 0xFFFF_EC00) | 0x0000_1004);
        }
        self.open_controller_command();
        unsafe {
            self.controller.mode[1].modify(|v| v);
            self.controller.mode[2].modify(|v| (v & 0xFFFF_EC00) | 0x0000_1004);
        }
    }

    /// Port-specific calibration words captured from the board.
    fn configure_port_calibration(&self) {
        unsafe {
            self.controller.ports[0].calibration[0].modify(|v| v);
            self.controller.ports[0].calibration[1].modify(|v| v);
            self.controller.ports[1].calibration[0].modify(|v| v);
            self.controller.ports[1].calibration[1].modify(|v| (v & 0xFFFF_EC00) | 0x0000_1004);
            self.controller.ports[2].calibration[0].modify(|v| (v & 0xFFFF_DFFF) | 0x0000_2000);
            self.controller.ports[2].calibration[1].modify(|v| v);
            self.controller.ports[3].calibration[0].modify(|v| v);
            self.controller.ports[3].calibration[1].modify(|v| v);
        }
    }

    fn configure_scheduler(&self) {
        unsafe {
            self.controller
                .scheduler
                .write(Scheduler::from_bits(0x0000_1F05));
            self.controller.scheduler1.write(0);
        }
    }

    /// Pulse the DFI initialisation start bit.
    fn start_dfi_init(&self) {
        unsafe {
            self.controller.init_trigger.modify(|v| (v & !0x2) | 0x2);
            self.controller.init_trigger.modify(|v| v & !0x2);
        }
    }

    fn set_dfi_init_complete(&self) {
        unsafe {
            self.controller
                .init_trigger
                .modify(|v| (v & 0xFFFF_FFFC) | 0x1);
        }
    }

    fn clear_dfi_init(&self) {
        unsafe { self.controller.init_trigger.modify(|v| v & 0xFFFF_FFFC) };
    }

    fn configure_controller_ports(&self, profile: &DramHardwareProfile) -> Result<(), DramError> {
        let port_default = PortProfile::default_port();
        let port2 = PortProfile {
            config_write: PortConfigWrite::from_bits(0x0111_0E0C),
            ..port_default
        };
        let port3 = PortProfile {
            config_read: PortConfigRead::from_bits(0x0010_0E08),
            ..port_default
        };
        unsafe {
            self.controller.init_words[1].write(0);
            self.controller.init_words[2].write(0);
            self.controller.init_words[3].write(0x0000_1F1F);
            self.controller.init_words[0].write(profile.init0);
            self.controller.init_words[4].write(profile.init4);
            self.controller.init_words[5].write(profile.init5);
            self.controller
                .master_control
                .write(MasterControl::from_bits(profile.master_control));
            self.controller.refresh[1].write(1);
            self.open_controller_command();
            self.controller.refresh[0].write(profile.refresh0);
            self.commit_and_reopen_controller_command()?;

            self.controller
                .dram_timing0
                .write(DramTiming0::from_bits(profile.timing0));
            self.controller
                .dram_timing1
                .write(DramTiming1::from_bits(profile.timing1));
            self.controller
                .dram_timing2
                .write(DramTiming2::from_bits(profile.timing2));
            self.controller
                .dram_timing3
                .write(DramTiming3::from_bits(profile.timing3));
            self.controller
                .dram_timing4
                .write(DramTiming4::from_bits(profile.timing4));
            self.controller
                .dram_timing5
                .write(DramTiming5::from_bits(profile.timing5));
            self.controller
                .dram_timing8
                .write(DramTiming8::from_bits(profile.timing8));
            self.commit_and_reopen_controller_command()?;

            self.controller
                .rank_control
                .modify(|v| (v & 0x3FFF_FFFF) | 0x4000_0000);
            self.controller.address_map3.write(0);
            self.commit_and_reopen_controller_command()?;

            self.controller.mapping[0].write(profile.mapping0);
            self.controller.mapping[1].write(0x0004_0101);
            self.controller
                .init_parameter
                .write(profile.dfi_timing0_ports);
            self.commit_and_reopen_controller_command()?;

            self.enable_address_remap();
            self.configure_mode_calibration();

            // Port 0..4 timing and QoS configuration
            let port_profiles = [port_default, port_default, port2, port3, port_default];
            for (port, settings) in self.controller.ports.iter().zip(port_profiles) {
                port.config_read.write(settings.config_read);
                port.read_qos.write(settings.read_qos);
                port.config_write.write(settings.config_write);
                port.write_qos.write(settings.write_qos);
            }
            self.configure_port_calibration();

            self.commit_controller_command()?;
            self.commit_controller_command()?;
            self.configure_scheduler();
            Ok(())
        }
    }

    fn configure_timing_and_phy(&self, profile: &DramHardwareProfile) -> Result<(), DramError> {
        unsafe {
            self.controller
                .init_parameter
                .write(profile.dfi_timing0_phy);
            self.start_dfi_init();

            // Program PHY Mode Registers and timing parameters
            self.phy
                .dcr
                .write(PhyDramConfig::from_bits(profile.phy_dcr));
            self.phy.mr[2].write(profile.phy_mr2);
            self.phy.mr[3].write(0);
            self.phy.mr[1].write(0x0000_0040);
            self.phy.mr[0].write(profile.phy_mr0);
            self.phy.dtpr[0].write(profile.phy_dtpr0);
            self.phy.dtpr[1].write(profile.phy_dtpr1);
            self.phy.dtpr[2].write(profile.phy_dtpr2);
            self.phy.ptr[0].write(profile.phy_ptr0);
            self.phy.ptr[1].write(profile.phy_ptr1);

            // Analog timing & update control
            self.phy.aciocr.modify(|v| (v & !0x1) | 0x1);
            self.phy.pgcr2.modify(|v| (v & 0x7FFF_FFFF) | 0x8000_0000);
            self.phy.dxccr.modify(|v| (v & 0xFF3F_FFFF) | 0x00C0_0000);
            self.phy.pgcr1.modify(|v| v & 0xFFFF_FE7F);
            self.phy.pgcr0.modify(|v| (v & 0xFDFF_FFFF) | 0x0200_0000);
            self.phy
                .pgcr2
                .modify(|v| (v & 0x7FFC_0000) | profile.dsgcr_bits);
            self.phy.pgcr1.modify(|v| v & 0xFBFF_FE7F);
            self.phy.dxccr.modify(|v| (v & 0xFF3F_FFFB) | 0x00C0_0004);
            self.phy.pgcr1.modify(|v| (v & 0xFBFF_FC7F) | 0x0000_0200);
            self.phy.dxccr.modify(|v| (v & 0xFF3F_FE1B) | 0x00C0_0184);
            self.phy.dxccr.modify(|v| (v & 0xFF3F_E01B) | 0x00C0_0984);
            self.phy.dsgcr.modify(|v| v & 0xFFFB_FFFE);

            // Data slices power and clock gating
            for dx in &self.phy.dx {
                dx.gcr.modify(|v| (v & 0xFFFF_C7FF) | 0x0000_2000);
                dx.gcr.modify(|v| (v & 0xFFFF_C7F9) | 0x0000_2006);
            }

            // PHY initialization timers
            self.phy.dtar.modify(|v| (v & 0xF0FF_FFFF) | 0x0100_0000);
            self.phy.odtcr.write(0x0001_0000);
            self.phy.dtar.modify(|v| (v & 0x00FF_FFFF) | 0x1100_0000);
            self.phy
                .zq
                .modify(|v| v.set_impedance(profile.zq_impedance));
            self.phy.dtar.modify(|v| (v & 0x00FF_FFF0) | 0x1100_0000);
            self.phy.dtar.modify(|v| (v & 0x00FF_FFF0) | 0x1100_000F);
            self.phy
                .pllcr
                .modify(|v| v.set_frequency_select(profile.pll_frequency_select));

            // Execute PHY leveling and initialization routine
            self.execute_phy_init_request(PhyInitRequest::from_bits(profile.pir_init))?;

            // Address map synchronization
            self.open_controller_command();
            self.controller.address_map3.write(1);
            self.commit_controller_command()?;

            // Wait for DRAM initialization complete
            if !self.poll_until(
                || self.controller.status.read().is_dram_init_done(),
                100_000_000,
            ) {
                return Err(DramError::ControllerInitTimeout);
            }

            self.set_dfi_init_complete();

            self.open_controller_command();
            self.controller.address_map3.write(0);
            self.commit_controller_command()?;

            // Execute eye training
            self.execute_phy_init_request(PhyInitRequest::from_bits(0x0000_F000))?;

            // Data slices post-training delay line configuration
            for dx in &self.phy.dx {
                dx.gcr.modify(|v| (v & 0xFFFF_C1F9) | 0x0000_2000);
                for bdlr in &dx.bdlr[..3] {
                    bdlr.modify(|v| v);
                }
                dx.lcdlr[0].modify(|v| profile.lcdlr0(v, false));
            }

            self.phy.pgcr1.modify(|v| (v & 0xF9FF_FC7F) | 0x0000_0200);
            for dx in &self.phy.dx {
                for bdlr in &dx.bdlr[3..] {
                    bdlr.modify(|v| v);
                }
                dx.bdlr[2].modify(|v| v & 0xFFFC_0FFF);
                dx.lcdlr[1].modify(|v| v & profile.lcdlr1_and);
                dx.lcdlr[0].modify(|v| profile.lcdlr0(v, true));
            }

            self.phy.pgcr1.modify(|v| (v & 0xF9FF_FC7F) | 0x0200_0200);
            self.phy.acbdlr.modify(|v| v);
            self.phy.acbdlr.modify(|v| v);
            self.phy.pgcr1.modify(|v| (v & 0xF9FF_FC7F) | 0x0600_0200);
            self.phy.pgcr0.modify(|v| (v & 0xFDFF_FFC0) | 0x0200_0000);
            self.phy.pgcr0.modify(|v| v & 0xFDFF_FFC0);

            self.open_controller_command();
            self.controller.address_map3.write(0);
            self.commit_controller_command()?;

            self.clear_dfi_init();
            Ok(())
        }
    }

    /// Poll a status bit until it reads true or the budget runs out.
    fn poll_until(&self, mut done: impl FnMut() -> bool, polls: u32) -> bool {
        for _ in 0..polls {
            if done() {
                return true;
            }
            core::hint::spin_loop();
        }
        false
    }

    /// Retrain PHY read-eye delay lines via PIR request.
    pub fn retrain_read_eye(&self, dram_type: DramType) -> Result<(), DramError> {
        let pir = match dram_type {
            DramType::Ddr3 => DramHardwareProfile::ddr3().pir_init,
            DramType::Ddr2 => DramHardwareProfile::ddr2().pir_init,
        };
        self.execute_phy_init_request(PhyInitRequest::from_bits(pir))?;
        self.execute_phy_init_request(PhyInitRequest::from_bits(0x0000_F000))?;

        Ok(())
    }

    /// Read MP_PGSR0 and report PHY status.
    pub fn report_status(&self) -> Result<PhyStatus, DramError> {
        let status = self.phy.pgsr0.read();

        if status.training_error() != 0 {
            Err(DramError::PhyTrainingError(status))
        } else if !status.is_init_done() {
            Err(DramError::PhyNotReady(status))
        } else {
            Ok(status)
        }
    }

    /// Hold or release the controller core, AHB and AXI resets.
    fn set_controller_reset(cmu: &mut Cmu, hold: bool) {
        unsafe {
            cmu.register_block().clock_ddr.modify(|val| {
                if hold {
                    val.enable_ctrl_reset()
                        .enable_ahb_bus_reset()
                        .enable_axi_bus_reset()
                } else {
                    val.disable_ctrl_reset()
                        .disable_ahb_bus_reset()
                        .disable_axi_bus_reset()
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_support_covers_shipped_sets() {
        assert!(DramSequencer::is_supported_config(&DramConfig::DDR3_128MB));
        assert!(DramSequencer::is_supported_config(&DramConfig::DDR2_64MB));

        let mut unsupported = DramConfig::DDR3_128MB;
        unsupported.freq = 500_000_000;
        assert!(!DramSequencer::is_supported_config(&unsupported));
    }
}
