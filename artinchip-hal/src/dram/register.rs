//! Register blocks of the DRAM subsystem.
//!
//! # Deduced IP Architecture (unoffical)
//! - **Controller**: Synopsys DesignWare uMCTL2 (Universal DDR Memory Controller).
//! - **PHY**: Synopsys DesignWare DDR2/3 PHY utility block (PUB).

use volatile_register::{RO, RW};

/// DDR controller register block.
#[repr(C)]
pub struct ControllerRegisterBlock {
    /// Master control word (`MSTR`).
    #[doc(alias = "MSTR")]
    pub master_control: RW<MasterControl>,
    /// Controller operating-mode status (`STAT`).
    #[doc(alias = "STAT")]
    pub status: RO<ControllerStatus>,
    _reserved0: [u8; 0x28],
    /// Low-power control (`PWRCTL`).
    #[doc(alias = "PWRCTL")]
    pub power_control: RW<PowerControl>,
    _reserved1: [u8; 0x2C],
    /// DFI initialisation trigger.
    pub init_trigger: RW<u32>,
    /// DFI initialisation parameter.
    pub init_parameter: RW<u32>,
    _reserved2: [u8; 0x68],
    /// Rank/axis configuration word; the sequence sets bit 30.
    pub rank_control: RW<u32>,
    _reserved3: [u8; 0x2C],
    /// DRAM timing 0 (`DRAMTMG0`).
    pub dram_timing0: RW<DramTiming0>,
    /// DRAM timing 1 (`DRAMTMG1`).
    pub dram_timing1: RW<DramTiming1>,
    /// DRAM timing 2 (`DRAMTMG2`).
    pub dram_timing2: RW<DramTiming2>,
    /// DRAM timing 3 (`DRAMTMG3`).
    pub dram_timing3: RW<DramTiming3>,
    /// DRAM timing 4 (`DRAMTMG4`).
    pub dram_timing4: RW<DramTiming4>,
    /// DRAM timing 5 (`DRAMTMG5`).
    pub dram_timing5: RW<DramTiming5>,
    /// DRAM timing 6 (`DRAMTMG6`).
    pub dram_timing6: RW<u32>,
    /// DRAM timing 7 (`DRAMTMG7`).
    pub dram_timing7: RW<u32>,
    /// DRAM timing 8 (`DRAMTMG8`).
    pub dram_timing8: RW<DramTiming8>,
    _reserved4: [u8; 0x5C],
    /// Address-map control word.
    pub address_map: RW<u32>,
    _reserved5: [u8; 0xC],
    /// Address-map words.
    pub mapping: [RW<u32>; 2],
    _reserved6: [u8; 0x8],
    /// Address-map control word.
    pub address_map2: RW<u32>,
    _reserved7: [u8; 0xC],
    /// Address-map control word.
    pub address_map3: RW<u32>,
    _reserved8: [u8; 0x50],
    /// Controller initialisation words.
    pub init_words: [RW<u32>; 6],
    _reserved9: [u8; 0x24],
    /// Refresh control and timing words.
    pub refresh: [RW<u32>; 2],
    _reserved10: [u8; 0x8],
    /// Scheduler control (`SCHED`).
    #[doc(alias = "SCHED")]
    pub scheduler: RW<Scheduler>,
    /// Scheduler control 1 (`SCHED1`).
    #[doc(alias = "SCHED1")]
    pub scheduler1: RW<u32>,
    _reserved11: [u8; 0xC8],
    /// Software-command register (`SWCTL`).
    #[doc(alias = "SWCTL")]
    pub command: RW<CommandControl>,
    /// Software-command status register (`SWSTAT`).
    #[doc(alias = "SWSTAT")]
    pub command_status: RO<CommandStatus>,
    _reserved12: [u8; 0xD4],
    /// Port status: zero while no port has traffic outstanding.
    pub port_status: RO<PortStatus>,
    /// Controller/DFI mode words.
    pub mode: [RW<u32>; 3],
    _reserved13: [u8; 0x84],
    /// Port 0..4 register blocks.
    pub ports: [PortRegisterBlock; 5],
}

/// Controller operating mode (`STAT[2:0]`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ControllerMode {
    /// The controller is running its DRAM initialisation.
    Init,
    /// The controller is in its normal operating state.
    Normal,
    /// The controller has put the DRAM into self-refresh.
    SelfRefresh,
    /// A value the vendor decoder does not name.
    Other,
}

/// Controller status register (`STAT`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ControllerStatus(u32);

impl ControllerStatus {
    const MODE: u32 = 0x7;
    const DRAM_INIT_DONE: u32 = 0x1;

    /// Raw register value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Whether controller DRAM initialisation has finished (`STAT[0]`).
    #[inline]
    pub const fn is_dram_init_done(self) -> bool {
        (self.0 & Self::DRAM_INIT_DONE) != 0
    }
    /// Controller operating mode (`STAT[2:0]`).
    #[inline]
    pub const fn mode(self) -> ControllerMode {
        match self.0 & Self::MODE {
            0 => ControllerMode::Init,
            1 => ControllerMode::Normal,
            3 => ControllerMode::SelfRefresh,
            _ => ControllerMode::Other,
        }
    }
}

/// Controller power-control register (`PWRCTL`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PowerControl(u32);

impl PowerControl {
    const DRAM_CLOCK_DISABLE: u32 = 0x1 << 3;
    const SELF_REFRESH: u32 = 0x1;

    /// Raw register value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Whether the DRAM clock is held off (`PWRCTL[3]`).
    #[inline]
    pub const fn is_dram_clock_disabled(self) -> bool {
        (self.0 & Self::DRAM_CLOCK_DISABLE) != 0
    }
    /// Enable the self-refresh request (`PWRCTL[0]`).
    #[inline]
    pub const fn enable_self_refresh(self) -> Self {
        Self(self.0 | Self::SELF_REFRESH)
    }
    /// Disable the self-refresh request.
    #[inline]
    pub const fn disable_self_refresh(self) -> Self {
        Self(self.0 & !Self::SELF_REFRESH)
    }
    /// Whether the self-refresh request is set (`PWRCTL[0]`).
    #[inline]
    pub const fn is_self_refresh_enabled(self) -> bool {
        (self.0 & Self::SELF_REFRESH) != 0
    }
}
/// Controller software-command register (`SWCTL`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CommandControl(u32);

impl CommandControl {
    const START: u32 = 0x1;

    /// Idle command value written before programming quasi-dynamic registers.
    #[inline]
    pub const fn idle() -> Self {
        Self(0)
    }

    /// Commit value that starts the programmed command.
    #[inline]
    pub const fn commit() -> Self {
        Self(Self::START)
    }

    /// Raw register value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Start the controller software command.
    #[inline]
    pub const fn start(self) -> Self {
        Self(self.0 | Self::START)
    }
    /// Clear the start bit to arm the next command.
    #[inline]
    pub const fn clear(self) -> Self {
        Self(self.0 & !Self::START)
    }
    /// Whether the start bit is set.
    #[inline]
    pub const fn is_start_enabled(self) -> bool {
        (self.0 & Self::START) != 0
    }
}

/// Controller software-command status register (`SWSTAT`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CommandStatus(u32);

impl CommandStatus {
    /// Raw register value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Whether the software command completed.
    #[inline]
    pub const fn is_done(self) -> bool {
        self.0 == 1
    }
}

/// Controller port status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PortStatus(u32);

impl PortStatus {
    /// Raw register value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Whether every port is idle (the register reads zero).
    #[inline]
    pub const fn is_idle(self) -> bool {
        self.0 == 0
    }
}

/// One controller port register block.
#[repr(C)]
pub struct PortRegisterBlock {
    /// Port enable word.
    pub enable: RW<PortEnable>,
    /// Port read configuration (`PCFGR`).
    pub config_read: RW<PortConfigRead>,
    /// Port read QoS (`PCFGQOS0`).
    pub read_qos: RW<PortReadQos0>,
    /// Port write configuration (`PCFGW`).
    pub config_write: RW<PortConfigWrite>,
    /// Port write QoS (`PCFGQOS0` write).
    pub write_qos: RW<PortWriteQos0>,
    _reserved0: [u8; 0x10],
    /// Port calibration words.
    pub calibration: [RW<u32>; 2],
    _reserved1: [u8; 0x84],
}

/// Controller port enable word.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PortEnable(u32);

impl PortEnable {
    const ENABLE: u32 = 0x1;

    /// Build a port-enable value from raw bits.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Raw register value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Enable the port.
    #[inline]
    pub const fn enable(self) -> Self {
        Self(self.0 | Self::ENABLE)
    }
    /// Disable the port.
    #[inline]
    pub const fn disable(self) -> Self {
        Self(self.0 & !Self::ENABLE)
    }
    /// Whether the port is enabled.
    #[inline]
    pub const fn is_enabled(self) -> bool {
        (self.0 & Self::ENABLE) != 0
    }
}

/// DDR PHY register block.
#[repr(C)]
pub struct PhyRegisterBlock {
    /// PHY revision identification (`RIDR`).
    #[doc(alias = "RIDR")]
    pub revision: RO<u32>,
    /// PHY initialisation request register (`PIR`).
    #[doc(alias = "PIR")]
    pub pir: RW<PhyInitRequest>,
    /// PHY general configuration 0 (`PGCR0`).
    #[doc(alias = "PGCR0")]
    pub pgcr0: RW<u32>,
    /// PHY general configuration 1 (`PGCR1`).
    #[doc(alias = "PGCR1")]
    pub pgcr1: RW<u32>,
    /// PHY general status 0 (`PGSR0`).
    #[doc(alias = "PGSR0")]
    #[doc(alias = "MP_PGSR0")]
    pub pgsr0: RO<PhyGeneralStatus>,
    /// PHY general status 1 (`PGSR1`).
    #[doc(alias = "PGSR1")]
    pub pgsr1: RO<u32>,
    /// PHY PLL control register (`PLLCR`).
    #[doc(alias = "PLLCR")]
    pub pllcr: RW<PllControl>,
    _reserved0: [u8; 0xC],
    /// PHY timing registers (`PTR0..1`).
    pub ptr: [RW<u32>; 2],
    _reserved1: [u8; 0x4],
    /// AC bit-delay line register (`ACBDLR`).
    #[doc(alias = "ACBDLR")]
    pub acbdlr: RW<u32>,
    /// AC I/O configuration register (`ACIOCR`).
    #[doc(alias = "ACIOCR")]
    pub aciocr: RW<u32>,
    /// Data-slice common configuration (`DXCCR`).
    #[doc(alias = "DXCCR")]
    pub dxccr: RW<u32>,
    /// DDR system general configuration (`DSGCR`).
    #[doc(alias = "DSGCR")]
    pub dsgcr: RW<u32>,
    /// DRAM configuration (`DCR`).
    #[doc(alias = "DCR")]
    pub dcr: RW<PhyDramConfig>,
    /// DRAM timing parameters (`DTPR0..2`).
    pub dtpr: [RW<u32>; 3],
    /// DRAM mode registers (`MR0..3`).
    pub mr: [RW<u32>; 4],
    /// ODT control (`ODTCR`).
    #[doc(alias = "ODTCR")]
    pub odtcr: RW<u32>,
    /// Data-training address (`DTAR`).
    #[doc(alias = "DTAR")]
    pub dtar: RW<u32>,
    _reserved2: [u8; 0x20],
    /// PHY general configuration 2 (`PGCR2`).
    #[doc(alias = "PGCR2")]
    pub pgcr2: RW<u32>,
    _reserved3: [u8; 0xF4],
    /// PHY ZQ calibration control (`ZQ0CR1`).
    #[doc(alias = "ZQ0CR1")]
    pub zq: RW<ZqControl>,
    _reserved4: [u8; 0x38],
    /// Data-slice blocks (`DX0..1`).
    pub dx: [DxRegisterBlock; 2],
}

/// PHY initialisation request (`PIR`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PhyInitRequest(u32);

impl PhyInitRequest {
    const STAGE15: u32 = 0x1 << 15;
    const STAGE14: u32 = 0x1 << 14;
    const WRITE_EYE_TRAINING: u32 = 0x1 << 13;
    const READ_EYE_TRAINING: u32 = 0x1 << 12;
    const WRITE_DESKEW: u32 = 0x1 << 11;
    const READ_DESKEW: u32 = 0x1 << 10;
    const WRITE_LEVELING_ADJUST: u32 = 0x1 << 9;
    const DQS_GATE_TRAINING: u32 = 0x1 << 8;
    const WRITE_LEVELING: u32 = 0x1 << 7;
    const DRAM_INIT: u32 = 0x1 << 6;
    const DRAM_RESET: u32 = 0x1 << 5;
    const ITM_SOFT_RESET: u32 = 0x1 << 4;
    const ZQ_CALIBRATION: u32 = 0x1 << 3;
    const DLL_LOCK: u32 = 0x1 << 2;
    const DLL_SOFT_RESET: u32 = 0x1 << 1;
    const INIT: u32 = 0x1;

    /// Build a request from raw bits.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Raw register value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Enable the training stage 15 stage.
    #[inline]
    pub const fn enable_stage15(self) -> Self {
        Self(self.0 | Self::STAGE15)
    }
    /// Disable the training stage 15 stage.
    #[inline]
    pub const fn disable_stage15(self) -> Self {
        Self(self.0 & !Self::STAGE15)
    }
    /// Whether the training stage 15 stage is selected.
    #[inline]
    pub const fn is_stage15_enabled(self) -> bool {
        (self.0 & Self::STAGE15) != 0
    }
    /// Enable the training stage 14 stage.
    #[inline]
    pub const fn enable_stage14(self) -> Self {
        Self(self.0 | Self::STAGE14)
    }
    /// Disable the training stage 14 stage.
    #[inline]
    pub const fn disable_stage14(self) -> Self {
        Self(self.0 & !Self::STAGE14)
    }
    /// Whether the training stage 14 stage is selected.
    #[inline]
    pub const fn is_stage14_enabled(self) -> bool {
        (self.0 & Self::STAGE14) != 0
    }
    /// Enable the write-eye training stage.
    #[inline]
    pub const fn enable_write_eye_training(self) -> Self {
        Self(self.0 | Self::WRITE_EYE_TRAINING)
    }
    /// Disable the write-eye training stage.
    #[inline]
    pub const fn disable_write_eye_training(self) -> Self {
        Self(self.0 & !Self::WRITE_EYE_TRAINING)
    }
    /// Whether the write-eye training stage is selected.
    #[inline]
    pub const fn is_write_eye_training_enabled(self) -> bool {
        (self.0 & Self::WRITE_EYE_TRAINING) != 0
    }
    /// Enable the read-eye training stage.
    #[inline]
    pub const fn enable_read_eye_training(self) -> Self {
        Self(self.0 | Self::READ_EYE_TRAINING)
    }
    /// Disable the read-eye training stage.
    #[inline]
    pub const fn disable_read_eye_training(self) -> Self {
        Self(self.0 & !Self::READ_EYE_TRAINING)
    }
    /// Whether the read-eye training stage is selected.
    #[inline]
    pub const fn is_read_eye_training_enabled(self) -> bool {
        (self.0 & Self::READ_EYE_TRAINING) != 0
    }
    /// Enable the write deskew stage.
    #[inline]
    pub const fn enable_write_deskew(self) -> Self {
        Self(self.0 | Self::WRITE_DESKEW)
    }
    /// Disable the write deskew stage.
    #[inline]
    pub const fn disable_write_deskew(self) -> Self {
        Self(self.0 & !Self::WRITE_DESKEW)
    }
    /// Whether the write deskew stage is selected.
    #[inline]
    pub const fn is_write_deskew_enabled(self) -> bool {
        (self.0 & Self::WRITE_DESKEW) != 0
    }
    /// Enable the read deskew stage.
    #[inline]
    pub const fn enable_read_deskew(self) -> Self {
        Self(self.0 | Self::READ_DESKEW)
    }
    /// Disable the read deskew stage.
    #[inline]
    pub const fn disable_read_deskew(self) -> Self {
        Self(self.0 & !Self::READ_DESKEW)
    }
    /// Whether the read deskew stage is selected.
    #[inline]
    pub const fn is_read_deskew_enabled(self) -> bool {
        (self.0 & Self::READ_DESKEW) != 0
    }
    /// Enable the write-leveling adjust stage.
    #[inline]
    pub const fn enable_write_leveling_adjust(self) -> Self {
        Self(self.0 | Self::WRITE_LEVELING_ADJUST)
    }
    /// Disable the write-leveling adjust stage.
    #[inline]
    pub const fn disable_write_leveling_adjust(self) -> Self {
        Self(self.0 & !Self::WRITE_LEVELING_ADJUST)
    }
    /// Whether the write-leveling adjust stage is selected.
    #[inline]
    pub const fn is_write_leveling_adjust_enabled(self) -> bool {
        (self.0 & Self::WRITE_LEVELING_ADJUST) != 0
    }
    /// Enable the DQS gate training stage.
    #[inline]
    pub const fn enable_dqs_gate_training(self) -> Self {
        Self(self.0 | Self::DQS_GATE_TRAINING)
    }
    /// Disable the DQS gate training stage.
    #[inline]
    pub const fn disable_dqs_gate_training(self) -> Self {
        Self(self.0 & !Self::DQS_GATE_TRAINING)
    }
    /// Whether the DQS gate training stage is selected.
    #[inline]
    pub const fn is_dqs_gate_training_enabled(self) -> bool {
        (self.0 & Self::DQS_GATE_TRAINING) != 0
    }
    /// Enable the write leveling stage.
    #[inline]
    pub const fn enable_write_leveling(self) -> Self {
        Self(self.0 | Self::WRITE_LEVELING)
    }
    /// Disable the write leveling stage.
    #[inline]
    pub const fn disable_write_leveling(self) -> Self {
        Self(self.0 & !Self::WRITE_LEVELING)
    }
    /// Whether the write leveling stage is selected.
    #[inline]
    pub const fn is_write_leveling_enabled(self) -> bool {
        (self.0 & Self::WRITE_LEVELING) != 0
    }
    /// Enable the DRAM initialisation stage.
    #[inline]
    pub const fn enable_dram_init(self) -> Self {
        Self(self.0 | Self::DRAM_INIT)
    }
    /// Disable the DRAM initialisation stage.
    #[inline]
    pub const fn disable_dram_init(self) -> Self {
        Self(self.0 & !Self::DRAM_INIT)
    }
    /// Whether the DRAM initialisation stage is selected.
    #[inline]
    pub const fn is_dram_init_enabled(self) -> bool {
        (self.0 & Self::DRAM_INIT) != 0
    }
    /// Enable the DRAM reset drive stage.
    #[inline]
    pub const fn enable_dram_reset(self) -> Self {
        Self(self.0 | Self::DRAM_RESET)
    }
    /// Disable the DRAM reset drive stage.
    #[inline]
    pub const fn disable_dram_reset(self) -> Self {
        Self(self.0 & !Self::DRAM_RESET)
    }
    /// Whether the DRAM reset drive stage is selected.
    #[inline]
    pub const fn is_dram_reset_enabled(self) -> bool {
        (self.0 & Self::DRAM_RESET) != 0
    }
    /// Enable the internal timing module soft reset stage.
    #[inline]
    pub const fn enable_itm_soft_reset(self) -> Self {
        Self(self.0 | Self::ITM_SOFT_RESET)
    }
    /// Disable the internal timing module soft reset stage.
    #[inline]
    pub const fn disable_itm_soft_reset(self) -> Self {
        Self(self.0 & !Self::ITM_SOFT_RESET)
    }
    /// Whether the internal timing module soft reset stage is selected.
    #[inline]
    pub const fn is_itm_soft_reset_enabled(self) -> bool {
        (self.0 & Self::ITM_SOFT_RESET) != 0
    }
    /// Enable the impedance calibration stage.
    #[inline]
    pub const fn enable_zq_calibration(self) -> Self {
        Self(self.0 | Self::ZQ_CALIBRATION)
    }
    /// Disable the impedance calibration stage.
    #[inline]
    pub const fn disable_zq_calibration(self) -> Self {
        Self(self.0 & !Self::ZQ_CALIBRATION)
    }
    /// Whether the impedance calibration stage is selected.
    #[inline]
    pub const fn is_zq_calibration_enabled(self) -> bool {
        (self.0 & Self::ZQ_CALIBRATION) != 0
    }
    /// Enable the DLL lock stage.
    #[inline]
    pub const fn enable_dll_lock(self) -> Self {
        Self(self.0 | Self::DLL_LOCK)
    }
    /// Disable the DLL lock stage.
    #[inline]
    pub const fn disable_dll_lock(self) -> Self {
        Self(self.0 & !Self::DLL_LOCK)
    }
    /// Whether the DLL lock stage is selected.
    #[inline]
    pub const fn is_dll_lock_enabled(self) -> bool {
        (self.0 & Self::DLL_LOCK) != 0
    }
    /// Enable the DLL soft reset stage.
    #[inline]
    pub const fn enable_dll_soft_reset(self) -> Self {
        Self(self.0 | Self::DLL_SOFT_RESET)
    }
    /// Disable the DLL soft reset stage.
    #[inline]
    pub const fn disable_dll_soft_reset(self) -> Self {
        Self(self.0 & !Self::DLL_SOFT_RESET)
    }
    /// Whether the DLL soft reset stage is selected.
    #[inline]
    pub const fn is_dll_soft_reset_enabled(self) -> bool {
        (self.0 & Self::DLL_SOFT_RESET) != 0
    }
    /// Enable the initialisation/training engine stage.
    #[inline]
    pub const fn enable_init(self) -> Self {
        Self(self.0 | Self::INIT)
    }
    /// Disable the initialisation/training engine stage.
    #[inline]
    pub const fn disable_init(self) -> Self {
        Self(self.0 & !Self::INIT)
    }
    /// Whether the initialisation/training engine stage is selected.
    #[inline]
    pub const fn is_init_enabled(self) -> bool {
        (self.0 & Self::INIT) != 0
    }
}
/// PHY general status 0 (`PGSR0`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PhyGeneralStatus(u32);

/// Alias kept for the driver error types.
pub type PhyStatus = PhyGeneralStatus;

impl PhyGeneralStatus {
    const REQUEST_DONE: u32 = 0x1;
    const ZQ_ERROR: u32 = 0x1 << 20;
    const TRAINING_ERROR: u32 = 0xFF << 20;

    /// Build a status value from raw bits.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Raw register value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Whether PHY initialisation finished.
    #[inline]
    pub const fn is_init_done(self) -> bool {
        self.is_request_done()
    }
    /// Whether PHY initialisation finished cleanly.
    #[inline]
    pub const fn is_ok(self) -> bool {
        self.is_request_done() && self.training_error() == 0
    }
    /// Training error flags (`PGSR0[27:20]`, zero means all selected trainings passed).
    #[inline]
    pub const fn training_error(self) -> u8 {
        ((self.0 & Self::TRAINING_ERROR) >> 20) as u8
    }
    /// Whether impedance calibration failed (`PGSR0[20]`).
    #[inline]
    pub const fn is_zq_error(self) -> bool {
        (self.0 & Self::ZQ_ERROR) != 0
    }
    /// Whether the last PHY request completed (`PGSR0[0]`).
    #[inline]
    pub const fn is_request_done(self) -> bool {
        (self.0 & Self::REQUEST_DONE) != 0
    }
}

/// PHY PLL control (`PLLCR`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PllControl(u32);

impl PllControl {
    const FRQSEL_LSB: u32 = 0x1 << 18;

    /// Build a control value from raw bits.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Raw register value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Select the PLL frequency range bit (`PLLCR[18]`).
    #[inline]
    pub const fn set_frequency_select(self, high: bool) -> Self {
        if high {
            Self(self.0 | Self::FRQSEL_LSB)
        } else {
            Self(self.0 & !Self::FRQSEL_LSB)
        }
    }
    /// Whether the high PLL frequency range is selected.
    #[inline]
    pub const fn is_frequency_select_enabled(self) -> bool {
        (self.0 & Self::FRQSEL_LSB) != 0
    }
}

/// PHY ZQ impedance control (`ZQ0CR1`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ZqControl(u32);

impl ZqControl {
    const IMPEDANCE: u32 = 0xFF;

    /// Build a control value from raw bits.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Raw register value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Set the ZQ calibration impedance code (`ZQ0CR1[7:0]`).
    #[inline]
    pub const fn set_impedance(self, code: u8) -> Self {
        Self((self.0 & !Self::IMPEDANCE) | (code as u32))
    }
    /// ZQ calibration impedance code (`ZQ0CR1[7:0]`).
    #[inline]
    pub const fn impedance(self) -> u8 {
        (self.0 & Self::IMPEDANCE) as u8
    }
}

/// One PHY data-slice (DATX8) register block.
#[repr(C)]
pub struct DxRegisterBlock {
    /// Data-slice general configuration (`DXnGCR`).
    pub gcr: RW<u32>,
    _reserved0: [u8; 0x8],
    /// Byte delay-line registers (`DXnBDLR0..4`).
    pub bdlr: [RW<u32>; 5],
    _reserved1: [u8; 0x4],
    /// Leveling-code delay-line registers (`DXnLCDLR0..1`).
    pub lcdlr: [RW<u32>; 2],
    _reserved2: [u8; 0x14],
}

/// Controller master control (`MSTR`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MasterControl(u32);

impl MasterControl {
    const ACTIVE_RANKS: u32 = 0xF << 24;
    const BURST_RDWR: u32 = 0xF << 16;
    const DATA_BUS_WIDTH: u32 = 0x3 << 12;
    const EN_2T_TIMING_MODE: u32 = 0x1 << 10;
    const BURST_CHOP: u32 = 0x1 << 9;
    const DDR3: u32 = 0x1 << 0;

    /// Build a register value from raw bits.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Raw register value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Set the active ranks.
    #[inline]
    pub const fn set_active_ranks(self, value: u8) -> Self {
        Self((self.0 & !Self::ACTIVE_RANKS) | ((value as u32) << 24) & Self::ACTIVE_RANKS)
    }
    /// Get the active ranks.
    #[inline]
    pub const fn active_ranks(self) -> u8 {
        ((self.0 & Self::ACTIVE_RANKS) >> 24) as u8
    }
    /// Set the burst read/write type.
    #[inline]
    pub const fn set_burst_rdwr(self, value: u8) -> Self {
        Self((self.0 & !Self::BURST_RDWR) | ((value as u32) << 16) & Self::BURST_RDWR)
    }
    /// Get the burst read/write type.
    #[inline]
    pub const fn burst_rdwr(self) -> u8 {
        ((self.0 & Self::BURST_RDWR) >> 16) as u8
    }
    /// Set the data bus width.
    #[inline]
    pub const fn set_data_bus_width(self, value: u8) -> Self {
        Self((self.0 & !Self::DATA_BUS_WIDTH) | ((value as u32) << 12) & Self::DATA_BUS_WIDTH)
    }
    /// Get the data bus width.
    #[inline]
    pub const fn data_bus_width(self) -> u8 {
        ((self.0 & Self::DATA_BUS_WIDTH) >> 12) as u8
    }
    /// Enable the 2T timing mode.
    #[inline]
    pub const fn enable_en_2t_timing_mode(self) -> Self {
        Self(self.0 | Self::EN_2T_TIMING_MODE)
    }
    /// Disable the 2T timing mode.
    #[inline]
    pub const fn disable_en_2t_timing_mode(self) -> Self {
        Self(self.0 & !Self::EN_2T_TIMING_MODE)
    }
    /// Whether the 2T timing mode is enabled.
    #[inline]
    pub const fn is_en_2t_timing_mode_enabled(self) -> bool {
        (self.0 & Self::EN_2T_TIMING_MODE) != 0
    }
    /// Enable the burst chop.
    #[inline]
    pub const fn enable_burst_chop(self) -> Self {
        Self(self.0 | Self::BURST_CHOP)
    }
    /// Disable the burst chop.
    #[inline]
    pub const fn disable_burst_chop(self) -> Self {
        Self(self.0 & !Self::BURST_CHOP)
    }
    /// Whether the burst chop is enabled.
    #[inline]
    pub const fn is_burst_chop_enabled(self) -> bool {
        (self.0 & Self::BURST_CHOP) != 0
    }
    /// Enable the DDR3 mode select.
    #[inline]
    pub const fn enable_ddr3(self) -> Self {
        Self(self.0 | Self::DDR3)
    }
    /// Disable the DDR3 mode select.
    #[inline]
    pub const fn disable_ddr3(self) -> Self {
        Self(self.0 & !Self::DDR3)
    }
    /// Whether the DDR3 mode select is enabled.
    #[inline]
    pub const fn is_ddr3_enabled(self) -> bool {
        (self.0 & Self::DDR3) != 0
    }
}

/// DRAM timing 0 (`DRAMTMG0`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DramTiming0(u32);

impl DramTiming0 {
    const WR2PRE: u32 = 0x7F << 24;
    const T_FAW: u32 = 0x3F << 16;
    const T_RAS_MAX: u32 = 0x7F << 8;
    const T_RAS_MIN: u32 = 0x3F;

    /// Build a register value from raw bits.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Raw register value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Set the write to precharge.
    #[inline]
    pub const fn set_wr2pre(self, value: u8) -> Self {
        Self((self.0 & !Self::WR2PRE) | ((value as u32) << 24) & Self::WR2PRE)
    }
    /// Get the write to precharge.
    #[inline]
    pub const fn wr2pre(self) -> u8 {
        ((self.0 & Self::WR2PRE) >> 24) as u8
    }
    /// Set the four activate window.
    #[inline]
    pub const fn set_t_faw(self, value: u8) -> Self {
        Self((self.0 & !Self::T_FAW) | ((value as u32) << 16) & Self::T_FAW)
    }
    /// Get the four activate window.
    #[inline]
    pub const fn t_faw(self) -> u8 {
        ((self.0 & Self::T_FAW) >> 16) as u8
    }
    /// Set the maximum RAS active time.
    #[inline]
    pub const fn set_t_ras_max(self, value: u8) -> Self {
        Self((self.0 & !Self::T_RAS_MAX) | ((value as u32) << 8) & Self::T_RAS_MAX)
    }
    /// Get the maximum RAS active time.
    #[inline]
    pub const fn t_ras_max(self) -> u8 {
        ((self.0 & Self::T_RAS_MAX) >> 8) as u8
    }
    /// Set the minimum RAS active time.
    #[inline]
    pub const fn set_t_ras_min(self, value: u8) -> Self {
        Self((self.0 & !Self::T_RAS_MIN) | (value as u32) & Self::T_RAS_MIN)
    }
    /// Get the minimum RAS active time.
    #[inline]
    pub const fn t_ras_min(self) -> u8 {
        (self.0 & Self::T_RAS_MIN) as u8
    }
}

/// DRAM timing 1 (`DRAMTMG1`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DramTiming1(u32);

impl DramTiming1 {
    const T_XP: u32 = 0x1F << 16;
    const RD2PRE: u32 = 0x3F << 8;
    const T_RC: u32 = 0x7F;

    /// Build a register value from raw bits.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Raw register value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Set the power-down exit.
    #[inline]
    pub const fn set_t_xp(self, value: u8) -> Self {
        Self((self.0 & !Self::T_XP) | ((value as u32) << 16) & Self::T_XP)
    }
    /// Get the power-down exit.
    #[inline]
    pub const fn t_xp(self) -> u8 {
        ((self.0 & Self::T_XP) >> 16) as u8
    }
    /// Set the read to precharge.
    #[inline]
    pub const fn set_rd2pre(self, value: u8) -> Self {
        Self((self.0 & !Self::RD2PRE) | ((value as u32) << 8) & Self::RD2PRE)
    }
    /// Get the read to precharge.
    #[inline]
    pub const fn rd2pre(self) -> u8 {
        ((self.0 & Self::RD2PRE) >> 8) as u8
    }
    /// Set the RAS cycle time.
    #[inline]
    pub const fn set_t_rc(self, value: u8) -> Self {
        Self((self.0 & !Self::T_RC) | (value as u32) & Self::T_RC)
    }
    /// Get the RAS cycle time.
    #[inline]
    pub const fn t_rc(self) -> u8 {
        (self.0 & Self::T_RC) as u8
    }
}

/// DRAM timing 2 (`DRAMTMG2`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DramTiming2(u32);

impl DramTiming2 {
    const RD2WR: u32 = 0x3F << 8;
    const WR2RD: u32 = 0x3F;

    /// Build a register value from raw bits.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Raw register value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Set the read to write delay.
    #[inline]
    pub const fn set_rd2wr(self, value: u8) -> Self {
        Self((self.0 & !Self::RD2WR) | ((value as u32) << 8) & Self::RD2WR)
    }
    /// Get the read to write delay.
    #[inline]
    pub const fn rd2wr(self) -> u8 {
        ((self.0 & Self::RD2WR) >> 8) as u8
    }
    /// Set the write to read delay.
    #[inline]
    pub const fn set_wr2rd(self, value: u8) -> Self {
        Self((self.0 & !Self::WR2RD) | (value as u32) & Self::WR2RD)
    }
    /// Get the write to read delay.
    #[inline]
    pub const fn wr2rd(self) -> u8 {
        (self.0 & Self::WR2RD) as u8
    }
}

/// DRAM timing 3 (`DRAMTMG3`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DramTiming3(u32);

impl DramTiming3 {
    const T_MRD: u32 = 0x3F << 12;
    const T_MOD: u32 = 0x3FF;

    /// Build a register value from raw bits.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Raw register value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Set the mode register set command delay.
    #[inline]
    pub const fn set_t_mrd(self, value: u8) -> Self {
        Self((self.0 & !Self::T_MRD) | ((value as u32) << 12) & Self::T_MRD)
    }
    /// Get the mode register set command delay.
    #[inline]
    pub const fn t_mrd(self) -> u8 {
        ((self.0 & Self::T_MRD) >> 12) as u8
    }
    /// Set the mode register set time.
    #[inline]
    pub const fn set_t_mod(self, value: u16) -> Self {
        Self((self.0 & !Self::T_MOD) | (value as u32) & Self::T_MOD)
    }
    /// Get the mode register set time.
    #[inline]
    pub const fn t_mod(self) -> u16 {
        (self.0 & Self::T_MOD) as u16
    }
}

/// DRAM timing 4 (`DRAMTMG4`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DramTiming4(u32);

impl DramTiming4 {
    const T_RCD: u32 = 0x1F << 24;
    const T_CCD: u32 = 0xF << 16;
    const T_RRD: u32 = 0xF << 8;
    const T_RP: u32 = 0x1F;

    /// Build a register value from raw bits.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Raw register value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Set the RAS to CAS delay.
    #[inline]
    pub const fn set_t_rcd(self, value: u8) -> Self {
        Self((self.0 & !Self::T_RCD) | ((value as u32) << 24) & Self::T_RCD)
    }
    /// Get the RAS to CAS delay.
    #[inline]
    pub const fn t_rcd(self) -> u8 {
        ((self.0 & Self::T_RCD) >> 24) as u8
    }
    /// Set the column to column delay.
    #[inline]
    pub const fn set_t_ccd(self, value: u8) -> Self {
        Self((self.0 & !Self::T_CCD) | ((value as u32) << 16) & Self::T_CCD)
    }
    /// Get the column to column delay.
    #[inline]
    pub const fn t_ccd(self) -> u8 {
        ((self.0 & Self::T_CCD) >> 16) as u8
    }
    /// Set the row to row delay.
    #[inline]
    pub const fn set_t_rrd(self, value: u8) -> Self {
        Self((self.0 & !Self::T_RRD) | ((value as u32) << 8) & Self::T_RRD)
    }
    /// Get the row to row delay.
    #[inline]
    pub const fn t_rrd(self) -> u8 {
        ((self.0 & Self::T_RRD) >> 8) as u8
    }
    /// Set the precharge time.
    #[inline]
    pub const fn set_t_rp(self, value: u8) -> Self {
        Self((self.0 & !Self::T_RP) | (value as u32) & Self::T_RP)
    }
    /// Get the precharge time.
    #[inline]
    pub const fn t_rp(self) -> u8 {
        (self.0 & Self::T_RP) as u8
    }
}

/// DRAM timing 5 (`DRAMTMG5`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DramTiming5(u32);

impl DramTiming5 {
    const T_CKSRX: u32 = 0xF << 24;
    const T_CKSRE: u32 = 0xF << 16;
    const T_CKESR: u32 = 0x3F << 8;
    const T_CKE: u32 = 0x1F;

    /// Build a register value from raw bits.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Raw register value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Set the self-refresh exit to read.
    #[inline]
    pub const fn set_t_cksrx(self, value: u8) -> Self {
        Self((self.0 & !Self::T_CKSRX) | ((value as u32) << 24) & Self::T_CKSRX)
    }
    /// Get the self-refresh exit to read.
    #[inline]
    pub const fn t_cksrx(self) -> u8 {
        ((self.0 & Self::T_CKSRX) >> 24) as u8
    }
    /// Set the self-refresh exit to CKE.
    #[inline]
    pub const fn set_t_cksre(self, value: u8) -> Self {
        Self((self.0 & !Self::T_CKSRE) | ((value as u32) << 16) & Self::T_CKSRE)
    }
    /// Get the self-refresh exit to CKE.
    #[inline]
    pub const fn t_cksre(self) -> u8 {
        ((self.0 & Self::T_CKSRE) >> 16) as u8
    }
    /// Set the CKE to self-refresh.
    #[inline]
    pub const fn set_t_ckesr(self, value: u8) -> Self {
        Self((self.0 & !Self::T_CKESR) | ((value as u32) << 8) & Self::T_CKESR)
    }
    /// Get the CKE to self-refresh.
    #[inline]
    pub const fn t_ckesr(self) -> u8 {
        ((self.0 & Self::T_CKESR) >> 8) as u8
    }
    /// Set the CKE minimum pulse width.
    #[inline]
    pub const fn set_t_cke(self, value: u8) -> Self {
        Self((self.0 & !Self::T_CKE) | (value as u32) & Self::T_CKE)
    }
    /// Get the CKE minimum pulse width.
    #[inline]
    pub const fn t_cke(self) -> u8 {
        (self.0 & Self::T_CKE) as u8
    }
}

/// DRAM timing 8 (`DRAMTMG8`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DramTiming8(u32);

impl DramTiming8 {
    const T_XS_DLL_X32: u32 = 0x7F << 8;
    const T_XS_X32: u32 = 0x7F;

    /// Build a register value from raw bits.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Raw register value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Set the self-refresh exit with DLL.
    #[inline]
    pub const fn set_t_xs_dll_x32(self, value: u8) -> Self {
        Self((self.0 & !Self::T_XS_DLL_X32) | ((value as u32) << 8) & Self::T_XS_DLL_X32)
    }
    /// Get the self-refresh exit with DLL.
    #[inline]
    pub const fn t_xs_dll_x32(self) -> u8 {
        ((self.0 & Self::T_XS_DLL_X32) >> 8) as u8
    }
    /// Set the self-refresh exit delay.
    #[inline]
    pub const fn set_t_xs_x32(self, value: u8) -> Self {
        Self((self.0 & !Self::T_XS_X32) | (value as u32) & Self::T_XS_X32)
    }
    /// Get the self-refresh exit delay.
    #[inline]
    pub const fn t_xs_x32(self) -> u8 {
        (self.0 & Self::T_XS_X32) as u8
    }
}

/// Controller scheduler control (`SCHED`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Scheduler(u32);

impl Scheduler {
    const LPR_NUM_ENTRIES: u32 = 0x7F << 8;
    const PAGECLOSE: u32 = 0x1 << 2;
    const PREFER_WRITE: u32 = 0x1 << 1;
    const FORCE_LOW_PRI_N: u32 = 0x1 << 0;

    /// Build a register value from raw bits.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Raw register value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Set the low-priority queue entries.
    #[inline]
    pub const fn set_lpr_num_entries(self, value: u8) -> Self {
        Self((self.0 & !Self::LPR_NUM_ENTRIES) | ((value as u32) << 8) & Self::LPR_NUM_ENTRIES)
    }
    /// Get the low-priority queue entries.
    #[inline]
    pub const fn lpr_num_entries(self) -> u8 {
        ((self.0 & Self::LPR_NUM_ENTRIES) >> 8) as u8
    }
    /// Enable the page close policy.
    #[inline]
    pub const fn enable_pageclose(self) -> Self {
        Self(self.0 | Self::PAGECLOSE)
    }
    /// Disable the page close policy.
    #[inline]
    pub const fn disable_pageclose(self) -> Self {
        Self(self.0 & !Self::PAGECLOSE)
    }
    /// Whether the page close policy is enabled.
    #[inline]
    pub const fn is_pageclose_enabled(self) -> bool {
        (self.0 & Self::PAGECLOSE) != 0
    }
    /// Enable the prefer write.
    #[inline]
    pub const fn enable_prefer_write(self) -> Self {
        Self(self.0 | Self::PREFER_WRITE)
    }
    /// Disable the prefer write.
    #[inline]
    pub const fn disable_prefer_write(self) -> Self {
        Self(self.0 & !Self::PREFER_WRITE)
    }
    /// Whether the prefer write is enabled.
    #[inline]
    pub const fn is_prefer_write_enabled(self) -> bool {
        (self.0 & Self::PREFER_WRITE) != 0
    }
    /// Enable the force low priority.
    #[inline]
    pub const fn enable_force_low_pri_n(self) -> Self {
        Self(self.0 | Self::FORCE_LOW_PRI_N)
    }
    /// Disable the force low priority.
    #[inline]
    pub const fn disable_force_low_pri_n(self) -> Self {
        Self(self.0 & !Self::FORCE_LOW_PRI_N)
    }
    /// Whether the force low priority is enabled.
    #[inline]
    pub const fn is_force_low_pri_n_enabled(self) -> bool {
        (self.0 & Self::FORCE_LOW_PRI_N) != 0
    }
}

/// Port read configuration (`PCFGR`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PortConfigRead(u32);

impl PortConfigRead {
    const RDWR_ORDERED_EN: u32 = 0x1 << 16;
    const RD_PORT_PAGEMATCH_EN: u32 = 0x1 << 14;
    const RD_PORT_URGENT_EN: u32 = 0x1 << 13;
    const RD_PORT_AGING_EN: u32 = 0x1 << 12;
    const RD_PORT_PRIORITY: u32 = 0x3FF;

    /// Build a register value from raw bits.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Raw register value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Enable the read/write ordering.
    #[inline]
    pub const fn enable_rdwr_ordered_en(self) -> Self {
        Self(self.0 | Self::RDWR_ORDERED_EN)
    }
    /// Disable the read/write ordering.
    #[inline]
    pub const fn disable_rdwr_ordered_en(self) -> Self {
        Self(self.0 & !Self::RDWR_ORDERED_EN)
    }
    /// Whether the read/write ordering is enabled.
    #[inline]
    pub const fn is_rdwr_ordered_en_enabled(self) -> bool {
        (self.0 & Self::RDWR_ORDERED_EN) != 0
    }
    /// Enable the read port page match.
    #[inline]
    pub const fn enable_rd_port_pagematch_en(self) -> Self {
        Self(self.0 | Self::RD_PORT_PAGEMATCH_EN)
    }
    /// Disable the read port page match.
    #[inline]
    pub const fn disable_rd_port_pagematch_en(self) -> Self {
        Self(self.0 & !Self::RD_PORT_PAGEMATCH_EN)
    }
    /// Whether the read port page match is enabled.
    #[inline]
    pub const fn is_rd_port_pagematch_en_enabled(self) -> bool {
        (self.0 & Self::RD_PORT_PAGEMATCH_EN) != 0
    }
    /// Enable the read port urgent.
    #[inline]
    pub const fn enable_rd_port_urgent_en(self) -> Self {
        Self(self.0 | Self::RD_PORT_URGENT_EN)
    }
    /// Disable the read port urgent.
    #[inline]
    pub const fn disable_rd_port_urgent_en(self) -> Self {
        Self(self.0 & !Self::RD_PORT_URGENT_EN)
    }
    /// Whether the read port urgent is enabled.
    #[inline]
    pub const fn is_rd_port_urgent_en_enabled(self) -> bool {
        (self.0 & Self::RD_PORT_URGENT_EN) != 0
    }
    /// Enable the read port aging.
    #[inline]
    pub const fn enable_rd_port_aging_en(self) -> Self {
        Self(self.0 | Self::RD_PORT_AGING_EN)
    }
    /// Disable the read port aging.
    #[inline]
    pub const fn disable_rd_port_aging_en(self) -> Self {
        Self(self.0 & !Self::RD_PORT_AGING_EN)
    }
    /// Whether the read port aging is enabled.
    #[inline]
    pub const fn is_rd_port_aging_en_enabled(self) -> bool {
        (self.0 & Self::RD_PORT_AGING_EN) != 0
    }
    /// Set the read port priority.
    #[inline]
    pub const fn set_rd_port_priority(self, value: u16) -> Self {
        Self((self.0 & !Self::RD_PORT_PRIORITY) | (value as u32) & Self::RD_PORT_PRIORITY)
    }
    /// Get the read port priority.
    #[inline]
    pub const fn rd_port_priority(self) -> u16 {
        (self.0 & Self::RD_PORT_PRIORITY) as u16
    }
}

/// Port write configuration (`PCFGW`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PortConfigWrite(u32);

impl PortConfigWrite {
    const WR_PORT_PAGEMATCH_EN: u32 = 0x1 << 14;
    const WR_PORT_URGENT_EN: u32 = 0x1 << 13;
    const WR_PORT_AGING_EN: u32 = 0x1 << 12;
    const WR_PORT_PRIORITY: u32 = 0x3FF;

    /// Build a register value from raw bits.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Raw register value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Enable the write port page match.
    #[inline]
    pub const fn enable_wr_port_pagematch_en(self) -> Self {
        Self(self.0 | Self::WR_PORT_PAGEMATCH_EN)
    }
    /// Disable the write port page match.
    #[inline]
    pub const fn disable_wr_port_pagematch_en(self) -> Self {
        Self(self.0 & !Self::WR_PORT_PAGEMATCH_EN)
    }
    /// Whether the write port page match is enabled.
    #[inline]
    pub const fn is_wr_port_pagematch_en_enabled(self) -> bool {
        (self.0 & Self::WR_PORT_PAGEMATCH_EN) != 0
    }
    /// Enable the write port urgent.
    #[inline]
    pub const fn enable_wr_port_urgent_en(self) -> Self {
        Self(self.0 | Self::WR_PORT_URGENT_EN)
    }
    /// Disable the write port urgent.
    #[inline]
    pub const fn disable_wr_port_urgent_en(self) -> Self {
        Self(self.0 & !Self::WR_PORT_URGENT_EN)
    }
    /// Whether the write port urgent is enabled.
    #[inline]
    pub const fn is_wr_port_urgent_en_enabled(self) -> bool {
        (self.0 & Self::WR_PORT_URGENT_EN) != 0
    }
    /// Enable the write port aging.
    #[inline]
    pub const fn enable_wr_port_aging_en(self) -> Self {
        Self(self.0 | Self::WR_PORT_AGING_EN)
    }
    /// Disable the write port aging.
    #[inline]
    pub const fn disable_wr_port_aging_en(self) -> Self {
        Self(self.0 & !Self::WR_PORT_AGING_EN)
    }
    /// Whether the write port aging is enabled.
    #[inline]
    pub const fn is_wr_port_aging_en_enabled(self) -> bool {
        (self.0 & Self::WR_PORT_AGING_EN) != 0
    }
    /// Set the write port priority.
    #[inline]
    pub const fn set_wr_port_priority(self, value: u16) -> Self {
        Self((self.0 & !Self::WR_PORT_PRIORITY) | (value as u32) & Self::WR_PORT_PRIORITY)
    }
    /// Get the write port priority.
    #[inline]
    pub const fn wr_port_priority(self) -> u16 {
        (self.0 & Self::WR_PORT_PRIORITY) as u16
    }
}

/// Port read QoS 0 (`PCFGQOS0`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PortReadQos0(u32);

impl PortReadQos0 {
    const RQOS_MAP_REGION2: u32 = 0x3 << 24;
    const RQOS_MAP_REGION1: u32 = 0x3 << 20;
    const RQOS_MAP_REGION0: u32 = 0x3 << 16;
    const RQOS_MAP_LEVEL2: u32 = 0xF << 8;
    const RQOS_MAP_LEVEL1: u32 = 0xF;

    /// Build a register value from raw bits.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Raw register value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Set the QoS region 2.
    #[inline]
    pub const fn set_rqos_map_region2(self, value: u8) -> Self {
        Self((self.0 & !Self::RQOS_MAP_REGION2) | ((value as u32) << 24) & Self::RQOS_MAP_REGION2)
    }
    /// Get the QoS region 2.
    #[inline]
    pub const fn rqos_map_region2(self) -> u8 {
        ((self.0 & Self::RQOS_MAP_REGION2) >> 24) as u8
    }
    /// Set the QoS region 1.
    #[inline]
    pub const fn set_rqos_map_region1(self, value: u8) -> Self {
        Self((self.0 & !Self::RQOS_MAP_REGION1) | ((value as u32) << 20) & Self::RQOS_MAP_REGION1)
    }
    /// Get the QoS region 1.
    #[inline]
    pub const fn rqos_map_region1(self) -> u8 {
        ((self.0 & Self::RQOS_MAP_REGION1) >> 20) as u8
    }
    /// Set the QoS region 0.
    #[inline]
    pub const fn set_rqos_map_region0(self, value: u8) -> Self {
        Self((self.0 & !Self::RQOS_MAP_REGION0) | ((value as u32) << 16) & Self::RQOS_MAP_REGION0)
    }
    /// Get the QoS region 0.
    #[inline]
    pub const fn rqos_map_region0(self) -> u8 {
        ((self.0 & Self::RQOS_MAP_REGION0) >> 16) as u8
    }
    /// Set the QoS level 2.
    #[inline]
    pub const fn set_rqos_map_level2(self, value: u8) -> Self {
        Self((self.0 & !Self::RQOS_MAP_LEVEL2) | ((value as u32) << 8) & Self::RQOS_MAP_LEVEL2)
    }
    /// Get the QoS level 2.
    #[inline]
    pub const fn rqos_map_level2(self) -> u8 {
        ((self.0 & Self::RQOS_MAP_LEVEL2) >> 8) as u8
    }
    /// Set the QoS level 1.
    #[inline]
    pub const fn set_rqos_map_level1(self, value: u8) -> Self {
        Self((self.0 & !Self::RQOS_MAP_LEVEL1) | (value as u32) & Self::RQOS_MAP_LEVEL1)
    }
    /// Get the QoS level 1.
    #[inline]
    pub const fn rqos_map_level1(self) -> u8 {
        (self.0 & Self::RQOS_MAP_LEVEL1) as u8
    }
}

/// Port write QoS 0 (`PCFGQOS0` write).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PortWriteQos0(u32);

impl PortWriteQos0 {
    const WQOS_MAP_REGION2: u32 = 0x3 << 24;
    const WQOS_MAP_REGION1: u32 = 0x3 << 20;
    const WQOS_MAP_REGION0: u32 = 0x3 << 16;
    const WQOS_MAP_LEVEL2: u32 = 0xF << 8;
    const WQOS_MAP_LEVEL1: u32 = 0xF;

    /// Build a register value from raw bits.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Raw register value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Set the QoS region 2.
    #[inline]
    pub const fn set_wqos_map_region2(self, value: u8) -> Self {
        Self((self.0 & !Self::WQOS_MAP_REGION2) | ((value as u32) << 24) & Self::WQOS_MAP_REGION2)
    }
    /// Get the QoS region 2.
    #[inline]
    pub const fn wqos_map_region2(self) -> u8 {
        ((self.0 & Self::WQOS_MAP_REGION2) >> 24) as u8
    }
    /// Set the QoS region 1.
    #[inline]
    pub const fn set_wqos_map_region1(self, value: u8) -> Self {
        Self((self.0 & !Self::WQOS_MAP_REGION1) | ((value as u32) << 20) & Self::WQOS_MAP_REGION1)
    }
    /// Get the QoS region 1.
    #[inline]
    pub const fn wqos_map_region1(self) -> u8 {
        ((self.0 & Self::WQOS_MAP_REGION1) >> 20) as u8
    }
    /// Set the QoS region 0.
    #[inline]
    pub const fn set_wqos_map_region0(self, value: u8) -> Self {
        Self((self.0 & !Self::WQOS_MAP_REGION0) | ((value as u32) << 16) & Self::WQOS_MAP_REGION0)
    }
    /// Get the QoS region 0.
    #[inline]
    pub const fn wqos_map_region0(self) -> u8 {
        ((self.0 & Self::WQOS_MAP_REGION0) >> 16) as u8
    }
    /// Set the QoS level 2.
    #[inline]
    pub const fn set_wqos_map_level2(self, value: u8) -> Self {
        Self((self.0 & !Self::WQOS_MAP_LEVEL2) | ((value as u32) << 8) & Self::WQOS_MAP_LEVEL2)
    }
    /// Get the QoS level 2.
    #[inline]
    pub const fn wqos_map_level2(self) -> u8 {
        ((self.0 & Self::WQOS_MAP_LEVEL2) >> 8) as u8
    }
    /// Set the QoS level 1.
    #[inline]
    pub const fn set_wqos_map_level1(self, value: u8) -> Self {
        Self((self.0 & !Self::WQOS_MAP_LEVEL1) | (value as u32) & Self::WQOS_MAP_LEVEL1)
    }
    /// Get the QoS level 1.
    #[inline]
    pub const fn wqos_map_level1(self) -> u8 {
        (self.0 & Self::WQOS_MAP_LEVEL1) as u8
    }
}

/// PHY DRAM configuration (`DCR`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PhyDramConfig(u32);

impl PhyDramConfig {
    const DDR_2T: u32 = 0x1 << 28;
    const BYTE_MASK: u32 = 0xFF << 10;
    const DDR_8_BANK: u32 = 0x1 << 3;
    const DDR_MODE: u32 = 0x7;

    /// Build a register value from raw bits.
    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Raw register value.
    #[inline]
    pub const fn raw(self) -> u32 {
        self.0
    }

    /// Enable the 2T timing.
    #[inline]
    pub const fn enable_ddr_2t(self) -> Self {
        Self(self.0 | Self::DDR_2T)
    }
    /// Disable the 2T timing.
    #[inline]
    pub const fn disable_ddr_2t(self) -> Self {
        Self(self.0 & !Self::DDR_2T)
    }
    /// Whether the 2T timing is enabled.
    #[inline]
    pub const fn is_ddr_2t_enabled(self) -> bool {
        (self.0 & Self::DDR_2T) != 0
    }
    /// Set the byte mask.
    #[inline]
    pub const fn set_byte_mask(self, value: u8) -> Self {
        Self((self.0 & !Self::BYTE_MASK) | ((value as u32) << 10) & Self::BYTE_MASK)
    }
    /// Get the byte mask.
    #[inline]
    pub const fn byte_mask(self) -> u8 {
        ((self.0 & Self::BYTE_MASK) >> 10) as u8
    }
    /// Enable the eight-bank DRAM.
    #[inline]
    pub const fn enable_ddr_8_bank(self) -> Self {
        Self(self.0 | Self::DDR_8_BANK)
    }
    /// Disable the eight-bank DRAM.
    #[inline]
    pub const fn disable_ddr_8_bank(self) -> Self {
        Self(self.0 & !Self::DDR_8_BANK)
    }
    /// Whether the eight-bank DRAM is enabled.
    #[inline]
    pub const fn is_ddr_8_bank_enabled(self) -> bool {
        (self.0 & Self::DDR_8_BANK) != 0
    }
    /// Set the DRAM mode.
    #[inline]
    pub const fn set_ddr_mode(self, value: u8) -> Self {
        Self((self.0 & !Self::DDR_MODE) | (value as u32) & Self::DDR_MODE)
    }
    /// Get the DRAM mode.
    #[inline]
    pub const fn ddr_mode(self) -> u8 {
        (self.0 & Self::DDR_MODE) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::offset_of;

    #[test]
    fn controller_block_offsets() {
        assert_eq!(offset_of!(ControllerRegisterBlock, master_control), 0x000);
        assert_eq!(offset_of!(ControllerRegisterBlock, status), 0x004);
        assert_eq!(offset_of!(ControllerRegisterBlock, power_control), 0x030);
        assert_eq!(offset_of!(ControllerRegisterBlock, init_trigger), 0x060);
        assert_eq!(offset_of!(ControllerRegisterBlock, init_parameter), 0x064);
        assert_eq!(offset_of!(ControllerRegisterBlock, rank_control), 0x0D0);
        assert_eq!(offset_of!(ControllerRegisterBlock, dram_timing0), 0x100);
        assert_eq!(offset_of!(ControllerRegisterBlock, address_map), 0x180);
        assert_eq!(offset_of!(ControllerRegisterBlock, mapping), 0x190);
        assert_eq!(offset_of!(ControllerRegisterBlock, address_map2), 0x1A0);
        assert_eq!(offset_of!(ControllerRegisterBlock, address_map3), 0x1B0);
        assert_eq!(offset_of!(ControllerRegisterBlock, init_words), 0x204);
        assert_eq!(offset_of!(ControllerRegisterBlock, refresh), 0x240);
        assert_eq!(offset_of!(ControllerRegisterBlock, scheduler), 0x250);
        assert_eq!(offset_of!(ControllerRegisterBlock, command), 0x320);
        assert_eq!(offset_of!(ControllerRegisterBlock, command_status), 0x324);
        assert_eq!(offset_of!(ControllerRegisterBlock, port_status), 0x3FC);
        assert_eq!(offset_of!(ControllerRegisterBlock, mode), 0x400);
        assert_eq!(offset_of!(ControllerRegisterBlock, ports), 0x490);
    }

    #[test]
    fn port_block_offsets() {
        assert_eq!(offset_of!(PortRegisterBlock, enable), 0x000);
        assert_eq!(offset_of!(PortRegisterBlock, config_read), 0x004);
        assert_eq!(offset_of!(PortRegisterBlock, calibration), 0x024);
    }

    #[test]
    fn phy_block_offsets() {
        assert_eq!(offset_of!(PhyRegisterBlock, revision), 0x000);
        assert_eq!(offset_of!(PhyRegisterBlock, pir), 0x004);
        assert_eq!(offset_of!(PhyRegisterBlock, pgcr0), 0x008);
        assert_eq!(offset_of!(PhyRegisterBlock, pgcr1), 0x00C);
        assert_eq!(offset_of!(PhyRegisterBlock, pgsr0), 0x010);
        assert_eq!(offset_of!(PhyRegisterBlock, pgsr1), 0x014);
        assert_eq!(offset_of!(PhyRegisterBlock, pllcr), 0x018);
        assert_eq!(offset_of!(PhyRegisterBlock, ptr), 0x028);
        assert_eq!(offset_of!(PhyRegisterBlock, acbdlr), 0x034);
        assert_eq!(offset_of!(PhyRegisterBlock, dtpr), 0x048);
        assert_eq!(offset_of!(PhyRegisterBlock, mr), 0x054);
        assert_eq!(offset_of!(PhyRegisterBlock, odtcr), 0x064);
        assert_eq!(offset_of!(PhyRegisterBlock, pgcr2), 0x08C);
        assert_eq!(offset_of!(PhyRegisterBlock, zq), 0x184);
        assert_eq!(offset_of!(PhyRegisterBlock, dx), 0x1C0);
    }

    #[test]
    fn dx_block_offsets() {
        assert_eq!(offset_of!(DxRegisterBlock, gcr), 0x000);
        assert_eq!(offset_of!(DxRegisterBlock, bdlr), 0x00C);
        assert_eq!(offset_of!(DxRegisterBlock, lcdlr), 0x024);
    }
}
