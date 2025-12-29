//! I2C register blocks and registers.

use volatile_register::{RO, RW, WO};

/// Inter-Integrated Circuit Register Block.
#[repr(C)]
pub struct RegisterBlock {
    /// I2C control register (`I2C_CTL`).
    #[doc(alias = "I2C_CTL")]
    pub ctrl: RW<Control>,
    /// I2C target register (`I2C_TAR`).
    #[doc(alias = "I2C_TAR")]
    pub target: RW<Target>,
    /// I2C slave address register (`I2C_SAR`).
    #[doc(alias = "I2C_SAR")]
    pub slave_addr: RW<Slave>,
    /// I2C acknowledge general call register (`I2C_ACK_GEN_CALL`).
    #[doc(alias = "I2C_ACK_GEN_CALL")]
    pub ack_gen_call: RW<AckGenCall>,
    /// I2C data command register (`I2C_DATA_CMD`).
    #[doc(alias = "I2C_DATA_CMD")]
    pub data_cmd: RW<DataCommand>,
    _reserved0: [u8; 0xC],
    /// I2C standard SCL high count register (`I2C_SS_SCL_HCNT`).
    #[doc(alias = "I2C_SS_SCL_HCNT")]
    pub ss_scl_hcnt: RW<StandardSclHighCnt>,
    /// I2C standard SCL low count register (`I2C_SS_SCL_LCNT`).
    #[doc(alias = "I2C_SS_SCL_LCNT")]
    pub ss_scl_lcnt: RW<StandardSclLowCnt>,
    /// I2C fast SCL high count register (`I2C_FS_SCL_HCNT`).
    #[doc(alias = "I2C_FS_SCL_HCNT")]
    pub fs_scl_hcnt: RW<FastSclHighCnt>,
    /// I2C fast SCL low count register (`I2C_FS_SCL_LCNT`).
    #[doc(alias = "I2C_FS_SCL_LCNT")]
    pub fs_scl_lcnt: RW<FastSclLowCnt>,
    /// I2C SDA hold time register (`I2C_SDA_HOLD`).
    #[doc(alias = "I2C_SDA_HOLD")]
    pub sda_hold: RW<SdaHold>,
    /// I2C SDA setup register (`I2C_SDA_SETUP`).
    #[doc(alias = "I2C_SDA_SETUP")]
    pub sda_setup: RW<SdaSetup>,
    /// I2C interrupt mask register (`I2C_INTR_MASK`).
    #[doc(alias = "I2C_INTR_MASK")]
    pub intr_mask: RW<InterruptMask>,
    /// I2C interrupt clear register (`I2C_INTR_CLR`).
    #[doc(alias = "I2C_INTR_CLR")]
    pub intr_clear: WO<InterruptClear>,
    /// I2C raw interrupt status register (`I2C_RAW_INTR_STAT`).
    #[doc(alias = "I2C_RAW_INTR_STAT")]
    pub raw_intr_stat: RO<RawInterruptStatus>,
    _reserved1: [u8; 0x4],
    /// I2C enable register (`I2C_ENABLE`).
    #[doc(alias = "I2C_ENABLE")]
    pub enable: RW<Enable>,
    /// I2C enable status register (`I2C_ENABLE_STATUS`).
    #[doc(alias = "I2C_ENABLE_STATUS")]
    pub enable_status: RO<EnableStatus>,
    /// I2C status register (`I2C_STATUS`).
    #[doc(alias = "I2C_STATUS")]
    pub status: RO<Status>,
    /// I2C transport abort source register (`I2C_TX_ABRT_SOURCE`).
    #[doc(alias = "I2C_TX_ABRT_SOURCE")]
    pub tx_abrt_source: RO<TxAbortSource>,
    _reserved2: [u8; 0x38],
    /// I2C receive threshold register (`I2C_RX_TL`).
    #[doc(alias = "I2C_RX_TL")]
    pub rx_tl: RW<RxThreshold>,
    /// I2C transport threshold register (`I2C_TX_TL`).
    #[doc(alias = "I2C_TX_TL")]
    pub tx_tl: RW<TxThreshold>,
    /// I2C transport FIFO level register (`I2C_TXFLR`).
    #[doc(alias = "I2C_TXFLR")]
    pub tx_flr: RO<TxFifoLevel>,
    /// I2C receive FIFO level register (`I2C_RXFLR`).
    #[doc(alias = "I2C_RXFLR")]
    pub rx_flr: RO<RxFifoLevel>,
    /// I2C SCL stuck timeout register (`I2C_SCL_STUCK_TIMEOUT`).
    #[doc(alias = "I2C_SCL_STUCK_TIMEOUT")]
    pub scl_stuck_timeout: RW<u32>,
    /// I2C SDA stuck timeout register (`I2C_SDA_STUCK_TIMEOUT`).
    #[doc(alias = "I2C_SDA_STUCK_TIMEOUT")]
    pub sda_stuck_timeout: RW<u32>,
    _reserved3: [u8; 0x8],
    /// I2C fast speed spike suppression length register (`I2C_FS_SPKLEN`).
    #[doc(alias = "I2C_FS_SPKLEN")]
    pub fs_spklen: RW<u32>,
    _reserved4: [u8; 0x48],
    /// I2C version register (`VERSION`).
    #[doc(alias = "VERSION")]
    pub version: RO<u32>,
}

/// Speed mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SpeedMode {
    /// Standard mode (up to 100 kbps).
    Standard = 1,
    /// Fast mode (up to 400 kbps).
    Fast = 2,
}

/// Addressing mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    /// 7-bit addressing mode.
    Bit7,
    /// 10-bit addressing mode.
    Bit10,
}

/// I2C control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Control(u32);

impl Control {
    const BUS_CLEAR_FEATURE_CTL: u32 = 0x1 << 10;
    const RX_FIFO_FULL_HLD_CTL: u32 = 0x1 << 9;
    const TX_EMPTY_CTL: u32 = 0x1 << 8;
    const STOP_DET_IFADDRESSED: u32 = 0x1 << 7;
    const RESTART_ENABLE: u32 = 0x1 << 6;
    const SPEED_MODE_SELECT: u32 = 0x3 << 4;
    const TENBITADDR_SELECT_SLAVE: u32 = 0x1 << 3;
    const TENBITADDR_SELECT_MASTER: u32 = 0x1 << 2;
    const SLAVE_MODE_DISABLE: u32 = 0x1 << 1;
    const MASTER_MODE_ENABLE: u32 = 0x1 << 0;

    /// Enable bus clear feature (`BUS_CLEAR_FEATURE_CTL`).
    ///
    /// The SDA deadlock mechanism is enabled, and the `SCL_STUCK_AT_LOW` interrupt can be triggered.
    #[doc(alias = "BUS_CLEAR_FEATURE_CTL")]
    #[inline]
    pub const fn enable_bus_clear_feature(self) -> Self {
        Self(self.0 | Self::BUS_CLEAR_FEATURE_CTL)
    }
    /// Disable bus clear feature.
    #[inline]
    pub const fn disable_bus_clear_feature(self) -> Self {
        Self(self.0 & !Self::BUS_CLEAR_FEATURE_CTL)
    }
    /// Check if bus clear feature is enabled.
    #[inline]
    pub const fn is_bus_clear_feature_enabled(self) -> bool {
        (self.0 & Self::BUS_CLEAR_FEATURE_CTL) != 0
    }
    /// Enable rx fifo full hold control (`RX_FIFO_FULL_HLD_CTL`).
    #[doc(alias = "RX_FIFO_FULL_HLD_CTL")]
    #[inline]
    pub const fn enable_rx_fifo_full_hold(self) -> Self {
        Self(self.0 | Self::RX_FIFO_FULL_HLD_CTL)
    }
    /// Disable rx fifo full hold control.
    #[inline]
    pub const fn disable_rx_fifo_full_hold(self) -> Self {
        Self(self.0 & !Self::RX_FIFO_FULL_HLD_CTL)
    }
    /// Check if rx fifo full hold control is enabled.
    #[inline]
    pub const fn is_rx_fifo_full_hold_enabled(self) -> bool {
        (self.0 & Self::RX_FIFO_FULL_HLD_CTL) != 0
    }
    /// Set tx empty control bit (`TX_EMPTY_CTL`).
    #[doc(alias = "TX_EMPTY_CTL")]
    #[inline]
    pub const fn set_tx_empty_control(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::TX_EMPTY_CTL)
        } else {
            Self(self.0 & !Self::TX_EMPTY_CTL)
        }
    }
    /// Get tx empty control bit.
    #[inline]
    pub const fn tx_empty_control(self) -> bool {
        (self.0 & Self::TX_EMPTY_CTL) != 0
    }
    /// Set stop detect interrupt if addressed bit (`STOP_DET_IFADDRESSED`).
    ///
    /// `STOP_DET` interrupt control, in slave mode:
    /// - 0: Issue `STOP_DET` interrupt regardless of whether addressed or not.
    /// - 1: Issue `STOP_DET` interrupt only when addressed.
    #[doc(alias = "STOP_DET_IFADDRESSED")]
    #[inline]
    pub const fn set_stop_detect_if_addressed(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::STOP_DET_IFADDRESSED)
        } else {
            Self(self.0 & !Self::STOP_DET_IFADDRESSED)
        }
    }
    /// Get stop detect interrupt if addressed bit.
    #[inline]
    pub const fn stop_detect_if_addressed(self) -> bool {
        (self.0 & Self::STOP_DET_IFADDRESSED) != 0
    }
    /// Enable restart in master mode (`RESTART_ENABLE`).
    #[doc(alias = "RESTART_ENABLE")]
    #[inline]
    pub const fn enable_restart(self) -> Self {
        Self(self.0 | Self::RESTART_ENABLE)
    }
    /// Disable restart in master mode.
    #[inline]
    pub const fn disable_restart(self) -> Self {
        Self(self.0 & !Self::RESTART_ENABLE)
    }
    /// Check if restart in master mode is enabled.
    #[inline]
    pub const fn is_restart_enabled(self) -> bool {
        (self.0 & Self::RESTART_ENABLE) != 0
    }
    /// Set speed mode (`SPEED_MODE_SELECT`).
    #[doc(alias = "SPEED_MODE_SELECT")]
    #[inline]
    pub const fn set_speed_mode(self, mode: SpeedMode) -> Self {
        Self((self.0 & !Self::SPEED_MODE_SELECT) | (Self::SPEED_MODE_SELECT & ((mode as u32) << 4)))
    }
    /// Get speed mode.
    #[inline]
    pub const fn speed_mode(self) -> SpeedMode {
        match (self.0 & Self::SPEED_MODE_SELECT) >> 4 {
            1 => SpeedMode::Standard,
            2 => SpeedMode::Fast,
            _ => unreachable!(),
        }
    }
    /// Set addressing mode for slave (`TENBITADDR_SELECT_SLAVE`).
    #[doc(alias = "TENBITADDR_SELECT_SLAVE")]
    #[inline]
    pub const fn set_addressing_mode_slave(self, mode: AddressingMode) -> Self {
        Self(
            (self.0 & !Self::TENBITADDR_SELECT_SLAVE)
                | (Self::TENBITADDR_SELECT_SLAVE & ((mode as u32) << 3)),
        )
    }
    /// Get addressing mode for slave.
    #[inline]
    pub const fn addressing_mode_slave(self) -> AddressingMode {
        match (self.0 & Self::TENBITADDR_SELECT_SLAVE) >> 3 {
            0 => AddressingMode::Bit7,
            1 => AddressingMode::Bit10,
            _ => unreachable!(),
        }
    }
    /// Set addressing mode for master (`TENBITADDR_SELECT_MASTER`).
    #[doc(alias = "TENBITADDR_SELECT_MASTER")]
    #[inline]
    pub const fn set_addressing_mode_master(self, mode: AddressingMode) -> Self {
        Self(
            (self.0 & !Self::TENBITADDR_SELECT_MASTER)
                | (Self::TENBITADDR_SELECT_MASTER & ((mode as u32) << 2)),
        )
    }
    /// Get addressing mode for master.
    #[inline]
    pub const fn addressing_mode_master(self) -> AddressingMode {
        match (self.0 & Self::TENBITADDR_SELECT_MASTER) >> 2 {
            0 => AddressingMode::Bit7,
            1 => AddressingMode::Bit10,
            _ => unreachable!(),
        }
    }
    /// Enable slave mode (`SLAVE_MODE_DISABLE`).
    #[doc(alias = "SLAVE_MODE_DISABLE")]
    #[inline]
    pub const fn enable_slave_mode(self) -> Self {
        Self(self.0 & !Self::SLAVE_MODE_DISABLE)
    }
    /// Disable slave mode.
    #[inline]
    pub const fn disable_slave_mode(self) -> Self {
        Self(self.0 | Self::SLAVE_MODE_DISABLE)
    }
    /// Check if slave mode is enabled.
    #[inline]
    pub const fn is_slave_mode_enabled(self) -> bool {
        (self.0 & Self::SLAVE_MODE_DISABLE) == 0
    }
    /// Enable master mode (`MASTER_MODE_ENABLE`).
    #[doc(alias = "MASTER_MODE_ENABLE")]
    #[inline]
    pub const fn enable_master_mode(self) -> Self {
        Self(self.0 | Self::MASTER_MODE_ENABLE)
    }
    /// Disable master mode.
    #[inline]
    pub const fn disable_master_mode(self) -> Self {
        Self(self.0 & !Self::MASTER_MODE_ENABLE)
    }
    /// Check if master mode is enabled.
    #[inline]
    pub const fn is_master_mode_enabled(self) -> bool {
        (self.0 & Self::MASTER_MODE_ENABLE) != 0
    }
}

/// I2C target register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Target(u32);

impl Target {
    const GEN_CALL_CTL: u32 = 0x1 << 11;
    const START_BYTE_CTL: u32 = 0x1 << 10;
    const I2C_TAR: u32 = 0x3FF;

    /// Enable general call mode (`GEN_CALL_CTL`).
    #[doc(alias = "GEN_CALL_CTL")]
    #[inline]
    pub const fn enable_general_call(self) -> Self {
        Self(self.0 | Self::GEN_CALL_CTL)
    }
    /// Disable general call mode.
    #[inline]
    pub const fn disable_general_call(self) -> Self {
        Self(self.0 & !Self::GEN_CALL_CTL)
    }
    /// Check if general call mode is enabled.
    #[inline]
    pub const fn is_general_call_enabled(self) -> bool {
        (self.0 & Self::GEN_CALL_CTL) != 0
    }
    /// Enable start byte (`START_BYTE_CTL`).
    #[doc(alias = "START_BYTE_CTL")]
    #[inline]
    pub const fn enable_start_byte(self) -> Self {
        Self(self.0 | Self::START_BYTE_CTL)
    }
    /// Disable start byte.
    #[inline]
    pub const fn disable_start_byte(self) -> Self {
        Self(self.0 & !Self::START_BYTE_CTL)
    }
    /// Check if start byte is enabled.
    #[inline]
    pub const fn is_start_byte_enabled(self) -> bool {
        (self.0 & Self::START_BYTE_CTL) != 0
    }
    /// Set target address (`I2C_TAR`).
    #[doc(alias = "I2C_TAR")]
    #[inline]
    pub const fn set_target_address(self, address: u16) -> Self {
        assert!(address < 0x400, "Address out of range (expected 0..=0x3FF)");
        Self((self.0 & !Self::I2C_TAR) | (Self::I2C_TAR & (address as u32)))
    }
    /// Get target address.
    #[inline]
    pub const fn target_address(self) -> u16 {
        (self.0 & Self::I2C_TAR) as u16
    }
}

/// I2C slave address register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Slave(u32);

impl Slave {
    const I2C_SAR: u32 = 0x3FF;

    /// Set slave address (`I2C_SAR`).
    #[doc(alias = "I2C_SAR")]
    #[inline]
    pub const fn set_slave_address(self, address: u16) -> Self {
        assert!(address < 0x400, "Address out of range (expected 0..=0x3FF)");
        Self((self.0 & !Self::I2C_SAR) | (Self::I2C_SAR & (address as u32)))
    }
    /// Get slave address.
    #[inline]
    pub const fn slave_address(self) -> u16 {
        (self.0 & Self::I2C_SAR) as u16
    }
}

/// I2C acknowledge general call register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AckGenCall(u32);

impl AckGenCall {
    const ACK_GEN_CALL: u32 = 0x1;

    /// Enable acknowledge general call (`ACK_GEN_CALL`).
    #[doc(alias = "ACK_GEN_CALL")]
    #[inline]
    pub const fn enable_ack_general_call(self) -> Self {
        Self(self.0 | Self::ACK_GEN_CALL)
    }
    /// Disable acknowledge general call.
    #[inline]
    pub const fn disable_ack_general_call(self) -> Self {
        Self(self.0 & !Self::ACK_GEN_CALL)
    }
    /// Check if acknowledge general call is enabled.
    #[inline]
    pub const fn is_ack_general_call_enabled(self) -> bool {
        (self.0 & Self::ACK_GEN_CALL) != 0
    }
}

/// Transfer mode for DataCommand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferMode {
    Write,
    Read,
}

/// I2C data command register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DataCommand(u32);

impl DataCommand {
    const RESTART: u32 = 0x1 << 10;
    const STOP: u32 = 0x1 << 9;
    const CMD: u32 = 0x1 << 8;
    const DAT: u32 = 0xFF;

    /// Set restart bit (`RESTART`).
    ///
    /// - 0: When the transfer direction differs from the previous transfer command's direction.
    ///     - If RESTART_ENABLE(I2C_CTL[6]) is 1, issue a RESTART signal.
    ///     - If RESTART_ENABLE(I2C_CTL[6]) is 0, send a STOP signal followed by a START signal.
    /// - 1: Regardless of whether the current data transfer direction is the same as the previous transfer command's direction.
    ///     - If RESTART_ENABLE(I2C_CTL[6]) is 1, issue a RESTART signal.
    ///     - If RESTART_ENABLE(I2C_CTL[6]) is 0, send a STOP signal followed by a START signal.
    #[doc(alias = "RESTART")]
    #[inline]
    pub const fn set_restart(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::RESTART)
        } else {
            Self(self.0 & !Self::RESTART)
        }
    }
    /// Set stop bit (`STOP`).
    ///
    /// - 0: Do not follow with a STOP signal after one byte transfer completes.
    /// - 1: Follow with a STOP signal after one byte transfer completes.
    #[doc(alias = "STOP")]
    #[inline]
    pub const fn set_stop(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::STOP)
        } else {
            Self(self.0 & !Self::STOP)
        }
    }
    /// Set transfer mode (`CMD`).
    #[doc(alias = "CMD")]
    #[inline]
    pub const fn set_transfer_mode(self, mode: TransferMode) -> Self {
        Self((self.0 & !Self::CMD) | (Self::CMD & ((mode as u32) << 8)))
    }
    /// Set data byte (`DAT`).
    #[doc(alias = "DAT")]
    #[inline]
    pub const fn set_data_byte(self, data: u8) -> Self {
        Self((self.0 & !Self::DAT) | (Self::DAT & (data as u32)))
    }
    /// Get data byte.
    #[inline]
    pub const fn data_byte(self) -> u8 {
        (self.0 & Self::DAT) as u8
    }
}

/// I2C standard SCL high count register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct StandardSclHighCnt(u32);

impl StandardSclHighCnt {
    const I2C_SS_SCL_HCNT: u32 = 0xFFFF;

    /// Set standard SCL high count (`I2C_SS_SCL_HCNT`).
    ///
    /// - This bit can only be written when I2C is disabled (I2C_ENABLE[0] written to 0); writes at other times are invalid.
    /// - The minimum value is 0x6; writing a value less than 0x6 is invalid.
    ///
    /// Calculation method:
    ///
    /// - Standard SCL high-level minimum time: 4000 ns.
    /// - Standard SCL low-level minimum time: 4700 ns.
    ///
    /// Using standard mode as an example:
    ///
    /// - SCL high-level time = (`I2C_SS_SCL_HCNT` + `I2C_FS_SPIKELEN` + 8) * I2C_clk_period.
    #[doc(alias = "I2C_SS_SCL_HCNT")]
    #[inline]
    pub const fn set_scl_high_count(self, count: u16) -> Self {
        assert!(count >= 6, "Standard SCL high count must be at least 6");
        Self((self.0 & !Self::I2C_SS_SCL_HCNT) | (Self::I2C_SS_SCL_HCNT & (count as u32)))
    }
    /// Get standard SCL high count.
    #[inline]
    pub const fn scl_high_count(self) -> u16 {
        (self.0 & Self::I2C_SS_SCL_HCNT) as u16
    }
}

/// I2C standard SCL low count register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct StandardSclLowCnt(u32);

impl StandardSclLowCnt {
    const I2C_SS_SCL_LCNT: u32 = 0xFFFF;

    /// Set standard SCL low count (`I2C_SS_SCL_LCNT`).
    ///
    /// - This bit can only be written when I2C is disabled (I2C_ENABLE[0] written to 0); writes at other times are invalid.
    /// - The minimum value is 0x8; writing a value less than 0x8 is invalid.
    ///
    /// Calculation method:
    ///
    /// - Standard SCL high-level minimum time: 4000 ns.
    /// - Standard SCL low-level minimum time: 4700 ns.
    ///
    /// Using standard mode as an example:
    ///
    /// - SCL low-level time = (`I2C_SS_SCL_LCNT` + 1) * I2C_clk_period.
    #[doc(alias = "I2C_SS_SCL_LCNT")]
    #[inline]
    pub const fn set_scl_low_count(self, count: u16) -> Self {
        assert!(count >= 8, "Standard SCL low count must be at least 8");
        Self((self.0 & !Self::I2C_SS_SCL_LCNT) | (Self::I2C_SS_SCL_LCNT & (count as u32)))
    }
    /// Get standard SCL low count.
    #[inline]
    pub const fn scl_low_count(self) -> u16 {
        (self.0 & Self::I2C_SS_SCL_LCNT) as u16
    }
}

/// I2C fast SCL high count register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FastSclHighCnt(u32);

impl FastSclHighCnt {
    const I2C_FS_SCL_HCNT: u32 = 0xFFFF;

    /// Set fast SCL high count (`I2C_FS_SCL_HCNT`).
    ///
    /// - This bit can only be written when I2C is disabled (I2C_ENABLE[0] written to 0); writes at other times are invalid.
    /// - The minimum value is 0x6; writing a value less than 0x6 is invalid.
    ///
    /// Calculation method:
    ///
    /// - Fast SCL high-level minimum time: 600 ns.
    /// - Fast SCL low-level minimum time: 1300 ns.
    ///
    /// Using fast mode as an example:
    ///
    /// - SCL high-level time = (`I2C_FS_SCL_HCNT` + `I2C_FS_SPIKELEN` + 8) * I2C_clk_period.
    #[doc(alias = "I2C_FS_SCL_HCNT")]
    #[inline]
    pub const fn set_scl_high_count(self, count: u16) -> Self {
        assert!(count >= 6, "Fast SCL high count must be at least 6");
        Self((self.0 & !Self::I2C_FS_SCL_HCNT) | (Self::I2C_FS_SCL_HCNT & (count as u32)))
    }
    /// Get fast SCL high count.
    #[inline]
    pub const fn scl_high_count(self) -> u16 {
        (self.0 & Self::I2C_FS_SCL_HCNT) as u16
    }
}

/// I2C fast SCL low count register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FastSclLowCnt(u32);

impl FastSclLowCnt {
    const I2C_FS_SCL_LCNT: u32 = 0xFFFF;

    /// Set fast SCL low count (`I2C_FS_SCL_LCNT`).
    ///
    /// - This bit can only be written when I2C is disabled (I2C_ENABLE[0] written to 0); writes at other times are invalid.
    /// - The minimum value is 0x8; writing a value less than 0x8 is invalid.
    ///
    /// Calculation method:
    ///
    /// - Fast SCL high-level minimum time: 600 ns.
    /// - Fast SCL low-level minimum time: 1300 ns.
    ///
    /// Using fast mode as an example:
    ///
    /// - SCL low-level time = (`I2C_FS_SCL_LCNT` + 1) * I2C_clk_period.
    #[doc(alias = "I2C_FS_SCL_LCNT")]
    #[inline]
    pub const fn set_scl_low_count(self, count: u16) -> Self {
        assert!(count >= 8, "Fast SCL low count must be at least 8");
        Self((self.0 & !Self::I2C_FS_SCL_LCNT) | (Self::I2C_FS_SCL_LCNT & (count as u32)))
    }
    /// Get fast SCL low count.
    #[inline]
    pub const fn scl_low_count(self) -> u16 {
        (self.0 & Self::I2C_FS_SCL_LCNT) as u16
    }
}

/// I2C SDA hold time register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SdaHold(u32);

impl SdaHold {
    const I2C_SDA_RX_HOLD: u32 = 0xFFFF << 16;
    const I2C_SDA_TX_HOLD: u32 = 0xFFFF;

    /// Set SDA rx hold time (`I2C_SDA_RX_HOLD`).
    ///
    /// Unit: I2C clock cycles.
    #[doc(alias = "I2C_SDA_RX_HOLD")]
    #[inline]
    pub const fn set_sda_rx_hold(self, hold: u16) -> Self {
        Self((self.0 & !Self::I2C_SDA_RX_HOLD) | (Self::I2C_SDA_RX_HOLD & ((hold as u32) << 16)))
    }
    /// Get SDA rx hold time.
    #[inline]
    pub const fn sda_rx_hold(self) -> u16 {
        ((self.0 & Self::I2C_SDA_RX_HOLD) >> 16) as u16
    }
    /// Set SDA tx hold time (`I2C_SDA_TX_HOLD`).
    ///
    /// Unit: I2C clock cycles.
    #[doc(alias = "I2C_SDA_TX_HOLD")]
    #[inline]
    pub const fn set_sda_tx_hold(self, hold: u16) -> Self {
        Self((self.0 & !Self::I2C_SDA_TX_HOLD) | (Self::I2C_SDA_TX_HOLD & (hold as u32)))
    }
    /// Get SDA tx hold time.
    #[inline]
    pub const fn sda_tx_hold(self) -> u16 {
        (self.0 & Self::I2C_SDA_TX_HOLD) as u16
    }
}

/// I2C SDA setup register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SdaSetup(u32);

impl SdaSetup {
    const I2C_SDA_SETUP: u32 = 0xFF;

    /// Set SDA setup time (`I2C_SDA_SETUP`).
    ///
    /// SDA setup time value = this value - 1;
    /// Unit: I2C clock cycles.
    #[doc(alias = "I2C_SDA_SETUP")]
    #[inline]
    pub const fn set_sda_setup(self, setup: u8) -> Self {
        assert!(setup >= 2, "SDA setup time must be at least 2 clock cycles");
        Self((self.0 & !Self::I2C_SDA_SETUP) | (Self::I2C_SDA_SETUP & (setup as u32)))
    }
    /// Get SDA setup time.
    #[inline]
    pub const fn sda_setup(self) -> u8 {
        (self.0 & Self::I2C_SDA_SETUP) as u8
    }
}

/// I2C interrupt mask register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct InterruptMask(u32);

impl InterruptMask {
    const M_SCL_STUCK_AT_LOW: u32 = 0x1 << 14;
    const M_MASTER_ON_HOLD: u32 = 0x1 << 13;
    const M_GEN_CALL: u32 = 0x1 << 11;
    const M_START_DET: u32 = 0x1 << 10;
    const M_STOP_DET: u32 = 0x1 << 9;
    const M_ACTIVITY: u32 = 0x1 << 8;
    const M_RX_DONE: u32 = 0x1 << 7;
    const M_TX_ABRT: u32 = 0x1 << 6;
    const M_RD_REQ: u32 = 0x1 << 5;
    const M_TX_EMPTY: u32 = 0x1 << 4;
    const M_RX_FULL: u32 = 0x1 << 2;
    const M_RX_UNDER: u32 = 0x1;

    /// Enable scl stuck at low interrupt (`M_SCL_STUCK_AT_LOW`).
    #[doc(alias = "M_SCL_STUCK_AT_LOW")]
    #[inline]
    pub const fn enable_scl_stuck_at_low(self) -> Self {
        Self(self.0 | Self::M_SCL_STUCK_AT_LOW)
    }
    /// Disable scl stuck at low interrupt.
    #[inline]
    pub const fn disable_scl_stuck_at_low(self) -> Self {
        Self(self.0 & !Self::M_SCL_STUCK_AT_LOW)
    }
    /// Check if scl stuck at low interrupt is enabled.
    #[inline]
    pub const fn is_scl_stuck_at_low_enabled(self) -> bool {
        (self.0 & Self::M_SCL_STUCK_AT_LOW) != 0
    }
    /// Enable master on hold interrupt (`M_MASTER_ON_HOLD`).
    #[doc(alias = "M_MASTER_ON_HOLD")]
    #[inline]
    pub const fn enable_master_on_hold(self) -> Self {
        Self(self.0 | Self::M_MASTER_ON_HOLD)
    }
    /// Disable master on hold interrupt.
    #[inline]
    pub const fn disable_master_on_hold(self) -> Self {
        Self(self.0 & !Self::M_MASTER_ON_HOLD)
    }
    /// Check if master on hold interrupt is enabled.
    #[inline]
    pub const fn is_master_on_hold_enabled(self) -> bool {
        (self.0 & Self::M_MASTER_ON_HOLD) != 0
    }
    /// Enable general call interrupt (`M_GEN_CALL`).
    #[doc(alias = "M_GEN_CALL")]
    #[inline]
    pub const fn enable_general_call(self) -> Self {
        Self(self.0 | Self::M_GEN_CALL)
    }
    /// Disable general call interrupt.
    #[inline]
    pub const fn disable_general_call(self) -> Self {
        Self(self.0 & !Self::M_GEN_CALL)
    }
    /// Check if general call interrupt is enabled.
    #[inline]
    pub const fn is_general_call_enabled(self) -> bool {
        (self.0 & Self::M_GEN_CALL) != 0
    }
    /// Enable start detect interrupt (`M_START_DET`).
    #[doc(alias = "M_START_DET")]
    #[inline]
    pub const fn enable_start_detect(self) -> Self {
        Self(self.0 | Self::M_START_DET)
    }
    /// Disable start detect interrupt.
    #[inline]
    pub const fn disable_start_detect(self) -> Self {
        Self(self.0 & !Self::M_START_DET)
    }
    /// Check if start detect interrupt is enabled.
    #[inline]
    pub const fn is_start_detect_enabled(self) -> bool {
        (self.0 & Self::M_START_DET) != 0
    }
    /// Enable stop detect interrupt (`M_STOP_DET`).
    #[doc(alias = "M_STOP_DET")]
    #[inline]
    pub const fn enable_stop_detect(self) -> Self {
        Self(self.0 | Self::M_STOP_DET)
    }
    /// Disable stop detect interrupt.
    #[inline]
    pub const fn disable_stop_detect(self) -> Self {
        Self(self.0 & !Self::M_STOP_DET)
    }
    /// Check if stop detect interrupt is enabled.
    #[inline]
    pub const fn is_stop_detect_enabled(self) -> bool {
        (self.0 & Self::M_STOP_DET) != 0
    }
    /// Enable activity interrupt (`M_ACTIVITY`).
    #[doc(alias = "M_ACTIVITY")]
    #[inline]
    pub const fn enable_activity(self) -> Self {
        Self(self.0 | Self::M_ACTIVITY)
    }
    /// Disable activity interrupt.
    #[inline]
    pub const fn disable_activity(self) -> Self {
        Self(self.0 & !Self::M_ACTIVITY)
    }
    /// Check if activity interrupt is enabled.
    #[inline]
    pub const fn is_activity_enabled(self) -> bool {
        (self.0 & Self::M_ACTIVITY) != 0
    }
    /// Enable rx done interrupt (`M_RX_DONE`).
    #[doc(alias = "M_RX_DONE")]
    #[inline]
    pub const fn enable_rx_done(self) -> Self {
        Self(self.0 | Self::M_RX_DONE)
    }
    /// Disable rx done interrupt.
    #[inline]
    pub const fn disable_rx_done(self) -> Self {
        Self(self.0 & !Self::M_RX_DONE)
    }
    /// Check if rx done interrupt is enabled.
    #[inline]
    pub const fn is_rx_done_enabled(self) -> bool {
        (self.0 & Self::M_RX_DONE) != 0
    }
    /// Enable tx abort interrupt (`M_TX_ABRT`).
    #[doc(alias = "M_TX_ABRT")]
    #[inline]
    pub const fn enable_tx_abort(self) -> Self {
        Self(self.0 | Self::M_TX_ABRT)
    }
    /// Disable tx abort interrupt.
    #[inline]
    pub const fn disable_tx_abort(self) -> Self {
        Self(self.0 & !Self::M_TX_ABRT)
    }
    /// Check if tx abort interrupt is enabled.
    #[inline]
    pub const fn is_tx_abort_enabled(self) -> bool {
        (self.0 & Self::M_TX_ABRT) != 0
    }
    /// Enable read request interrupt (`M_RD_REQ`).
    #[doc(alias = "M_RD_REQ")]
    #[inline]
    pub const fn enable_read_request(self) -> Self {
        Self(self.0 | Self::M_RD_REQ)
    }
    /// Disable read request interrupt.
    #[inline]
    pub const fn disable_read_request(self) -> Self {
        Self(self.0 & !Self::M_RD_REQ)
    }
    /// Check if read request interrupt is enabled.
    #[inline]
    pub const fn is_read_request_enabled(self) -> bool {
        (self.0 & Self::M_RD_REQ) != 0
    }
    /// Enable tx empty interrupt (`M_TX_EMPTY`).
    #[doc(alias = "M_TX_EMPTY")]
    #[inline]
    pub const fn enable_tx_empty(self) -> Self {
        Self(self.0 | Self::M_TX_EMPTY)
    }
    /// Disable tx empty interrupt.
    #[inline]
    pub const fn disable_tx_empty(self) -> Self {
        Self(self.0 & !Self::M_TX_EMPTY)
    }
    /// Check if tx empty interrupt is enabled.
    #[inline]
    pub const fn is_tx_empty_enabled(self) -> bool {
        (self.0 & Self::M_TX_EMPTY) != 0
    }
    /// Enable rx full interrupt (`M_RX_FULL`).
    #[doc(alias = "M_RX_FULL")]
    #[inline]
    pub const fn enable_rx_full(self) -> Self {
        Self(self.0 | Self::M_RX_FULL)
    }
    /// Disable rx full interrupt.
    #[inline]
    pub const fn disable_rx_full(self) -> Self {
        Self(self.0 & !Self::M_RX_FULL)
    }
    /// Check if rx full interrupt is enabled.
    #[inline]
    pub const fn is_rx_full_enabled(self) -> bool {
        (self.0 & Self::M_RX_FULL) != 0
    }
    /// Enable rx under interrupt (`M_RX_UNDER`).
    #[doc(alias = "M_RX_UNDER")]
    #[inline]
    pub const fn enable_rx_under(self) -> Self {
        Self(self.0 | Self::M_RX_UNDER)
    }
    /// Disable rx under interrupt.
    #[inline]
    pub const fn disable_rx_under(self) -> Self {
        Self(self.0 & !Self::M_RX_UNDER)
    }
    /// Check if rx under interrupt is enabled.
    #[inline]
    pub const fn is_rx_under_enabled(self) -> bool {
        (self.0 & Self::M_RX_UNDER) != 0
    }
}

/// I2C interrupt clear register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct InterruptClear(u32);

impl InterruptClear {
    const CLR_SCL_STUCK_AT_LOW: u32 = 0x1 << 14;
    const CLR_MASTER_ON_HOLD: u32 = 0x1 << 13;
    const CLR_GEN_CALL: u32 = 0x1 << 11;
    const CLR_START_DET: u32 = 0x1 << 10;
    const CLR_STOP_DET: u32 = 0x1 << 9;
    const CLR_ACTIVITY: u32 = 0x1 << 8;
    const CLR_RX_DONE: u32 = 0x1 << 7;
    const CLR_TX_ABRT: u32 = 0x1 << 6;
    const CLR_RD_REQ: u32 = 0x1 << 5;
    const CLR_TX_EMPTY: u32 = 0x1 << 4;
    const CLR_RX_FULL: u32 = 0x1 << 2;
    const CLR_RX_UNDER: u32 = 0x1;

    /// Clear scl stuck at low interrupt (`CLR_SCL_STUCK_AT_LOW`).
    #[doc(alias = "CLR_SCL_STUCK_AT_LOW")]
    #[inline]
    pub const fn clear_scl_stuck_at_low(self) -> Self {
        Self(self.0 | Self::CLR_SCL_STUCK_AT_LOW)
    }
    /// Clear master on hold interrupt (`CLR_MASTER_ON_HOLD`).
    #[doc(alias = "CLR_MASTER_ON_HOLD")]
    #[inline]
    pub const fn clear_master_on_hold(self) -> Self {
        Self(self.0 | Self::CLR_MASTER_ON_HOLD)
    }
    /// Clear general call interrupt (`CLR_GEN_CALL`).
    #[doc(alias = "CLR_GEN_CALL")]
    #[inline]
    pub const fn clear_general_call(self) -> Self {
        Self(self.0 | Self::CLR_GEN_CALL)
    }
    /// Clear start detect interrupt (`CLR_START_DET`).
    #[doc(alias = "CLR_START_DET")]
    #[inline]
    pub const fn clear_start_detect(self) -> Self {
        Self(self.0 | Self::CLR_START_DET)
    }
    /// Clear stop detect interrupt (`CLR_STOP_DET`).
    #[doc(alias = "CLR_STOP_DET")]
    #[inline]
    pub const fn clear_stop_detect(self) -> Self {
        Self(self.0 | Self::CLR_STOP_DET)
    }
    /// Clear activity interrupt (`CLR_ACTIVITY`).
    #[doc(alias = "CLR_ACTIVITY")]
    #[inline]
    pub const fn clear_activity(self) -> Self {
        Self(self.0 | Self::CLR_ACTIVITY)
    }
    /// Clear rx done interrupt (`CLR_RX_DONE`).
    #[doc(alias = "CLR_RX_DONE")]
    #[inline]
    pub const fn clear_rx_done(self) -> Self {
        Self(self.0 | Self::CLR_RX_DONE)
    }
    /// Clear tx abort interrupt (`CLR_TX_ABRT`).
    #[doc(alias = "CLR_TX_ABRT")]
    #[inline]
    pub const fn clear_tx_abort(self) -> Self {
        Self(self.0 | Self::CLR_TX_ABRT)
    }
    /// Clear read request interrupt (`CLR_RD_REQ`).
    #[doc(alias = "CLR_RD_REQ")]
    #[inline]
    pub const fn clear_read_request(self) -> Self {
        Self(self.0 | Self::CLR_RD_REQ)
    }
    /// Clear tx empty interrupt (`CLR_TX_EMPTY`).
    #[doc(alias = "CLR_TX_EMPTY")]
    #[inline]
    pub const fn clear_tx_empty(self) -> Self {
        Self(self.0 | Self::CLR_TX_EMPTY)
    }
    /// Clear rx full interrupt (`CLR_RX_FULL`).
    #[doc(alias = "CLR_RX_FULL")]
    #[inline]
    pub const fn clear_rx_full(self) -> Self {
        Self(self.0 | Self::CLR_RX_FULL)
    }
    /// Clear rx under interrupt (`CLR_RX_UNDER`).
    #[doc(alias = "CLR_RX_UNDER")]
    #[inline]
    pub const fn clear_rx_under(self) -> Self {
        Self(self.0 | Self::CLR_RX_UNDER)
    }
}

/// I2C raw interrupt status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct RawInterruptStatus(u32);

impl RawInterruptStatus {
    const SCL_STUCK_AT_LOW: u32 = 0x1 << 14;
    const MASTER_ON_HOLD: u32 = 0x1 << 13;
    const GEN_CALL: u32 = 0x1 << 11;
    const START_DET: u32 = 0x1 << 10;
    const STOP_DET: u32 = 0x1 << 9;
    const ACTIVITY: u32 = 0x1 << 8;
    const RX_DONE: u32 = 0x1 << 7;
    const TX_ABRT: u32 = 0x1 << 6;
    const RD_REQ: u32 = 0x1 << 5;
    const TX_EMPTY: u32 = 0x1 << 4;
    const RX_FULL: u32 = 0x1 << 2;
    const RX_UNDER: u32 = 0x1;

    /// Check if scl stuck at low interrupt is pending (`SCL_STUCK_AT_LOW`).
    #[doc(alias = "SCL_STUCK_AT_LOW")]
    #[inline]
    pub const fn is_scl_stuck_at_low_pending(self) -> bool {
        (self.0 & Self::SCL_STUCK_AT_LOW) != 0
    }
    /// Check if master on hold interrupt is pending (`MASTER_ON_HOLD`).
    #[doc(alias = "MASTER_ON_HOLD")]
    #[inline]
    pub const fn is_master_on_hold_pending(self) -> bool {
        (self.0 & Self::MASTER_ON_HOLD) != 0
    }
    /// Check if general call interrupt is pending (`GEN_CALL`).
    #[doc(alias = "GEN_CALL")]
    #[inline]
    pub const fn is_general_call_pending(self) -> bool {
        (self.0 & Self::GEN_CALL) != 0
    }
    /// Check if start detect interrupt is pending (`START_DET`).
    #[doc(alias = "START_DET")]
    #[inline]
    pub const fn is_start_detect_pending(self) -> bool {
        (self.0 & Self::START_DET) != 0
    }
    /// Check if stop detect interrupt is pending (`STOP_DET`).
    #[doc(alias = "STOP_DET")]
    #[inline]
    pub const fn is_stop_detect_pending(self) -> bool {
        (self.0 & Self::STOP_DET) != 0
    }
    /// Check if activity interrupt is pending (`ACTIVITY`).
    #[doc(alias = "ACTIVITY")]
    #[inline]
    pub const fn is_activity_pending(self) -> bool {
        (self.0 & Self::ACTIVITY) != 0
    }
    /// Check if rx done interrupt is pending (`RX_DONE`).
    #[doc(alias = "RX_DONE")]
    #[inline]
    pub const fn is_rx_done_pending(self) -> bool {
        (self.0 & Self::RX_DONE) != 0
    }
    /// Check if tx abort interrupt is pending (`TX_ABRT`).
    #[doc(alias = "TX_ABRT")]
    #[inline]
    pub const fn is_tx_abort_pending(self) -> bool {
        (self.0 & Self::TX_ABRT) != 0
    }
    /// Check if read request interrupt is pending (`RD_REQ`).
    #[doc(alias = "RD_REQ")]
    #[inline]
    pub const fn is_read_request_pending(self) -> bool {
        (self.0 & Self::RD_REQ) != 0
    }
    /// Check if tx empty interrupt is pending (`TX_EMPTY`).
    #[doc(alias = "TX_EMPTY")]
    #[inline]
    pub const fn is_tx_empty_pending(self) -> bool {
        (self.0 & Self::TX_EMPTY) != 0
    }
    /// Check if rx full interrupt is pending (`RX_FULL`).
    #[doc(alias = "RX_FULL")]
    #[inline]
    pub const fn is_rx_full_pending(self) -> bool {
        (self.0 & Self::RX_FULL) != 0
    }
    /// Check if rx under interrupt is pending (`RX_UNDER`).
    #[doc(alias = "RX_UNDER")]
    #[inline]
    pub const fn is_rx_under_pending(self) -> bool {
        (self.0 & Self::RX_UNDER) != 0
    }
}

/// I2C enable register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Enable(u32);

impl Enable {
    const SDA_STUCK_RECOVERY_ENABLE: u32 = 0x1 << 3;
    const ABORT: u32 = 0x1 << 1;
    const ENABLE: u32 = 0x1;

    /// Enable sda stuck recovery (`SDA_STUCK_RECOVERY_ENABLE`).
    #[doc(alias = "SDA_STUCK_RECOVERY_ENABLE")]
    #[inline]
    pub const fn enable_sda_stuck_recovery(self) -> Self {
        Self(self.0 | Self::SDA_STUCK_RECOVERY_ENABLE)
    }
    /// Disable sda stuck recovery.
    #[inline]
    pub const fn disable_sda_stuck_recovery(self) -> Self {
        Self(self.0 & !Self::SDA_STUCK_RECOVERY_ENABLE)
    }
    /// Check if sda stuck recovery is enabled.
    #[inline]
    pub const fn is_sda_stuck_recovery_enabled(self) -> bool {
        (self.0 & Self::SDA_STUCK_RECOVERY_ENABLE) != 0
    }
    /// Set transmit abort (`ABORT`).
    #[inline]
    pub const fn set_abort(self, abort: bool) -> Self {
        if abort {
            Self(self.0 | Self::ABORT)
        } else {
            Self(self.0 & !Self::ABORT)
        }
    }
    /// Get transmit abort bit.
    #[inline]
    pub const fn abort(self) -> bool {
        (self.0 & Self::ABORT) != 0
    }
    /// Enable I2C (`ENABLE`).
    #[doc(alias = "ENABLE")]
    #[inline]
    pub const fn enable_i2c(self) -> Self {
        Self(self.0 | Self::ENABLE)
    }
    /// Disable I2C (`ENABLE`).
    #[inline]
    pub const fn disable_i2c(self) -> Self {
        Self(self.0 & !Self::ENABLE)
    }
    /// Check if I2C is enabled.
    #[inline]
    pub const fn is_i2c_enabled(self) -> bool {
        (self.0 & Self::ENABLE) != 0
    }
}

/// I2C enable status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct EnableStatus(u32);

impl EnableStatus {
    const SLV_RX_DATA_LOST: u32 = 0x1 << 2;
    const SLV_DISABLED_WHILE_BUSY: u32 = 0x1 << 1;
    const I2C_EN_STATUS: u32 = 0x1;

    /// Check if rx data is lost in slave mode (`SLV_RX_DATA_LOST`).
    #[doc(alias = "SLV_RX_DATA_LOST")]
    #[inline]
    pub const fn is_slave_rx_data_lost(self) -> bool {
        (self.0 & Self::SLV_RX_DATA_LOST) != 0
    }
    /// Check if slave mode was disabled while busy (`SLV_DISABLED_WHILE_BUSY`).
    #[doc(alias = "SLV_DISABLED_WHILE_BUSY")]
    #[inline]
    pub const fn is_slave_disabled_while_busy(self) -> bool {
        (self.0 & Self::SLV_DISABLED_WHILE_BUSY) == 0
    }
    /// Check if I2C is enabled (`I2C_EN_STATUS`).
    #[doc(alias = "I2C_EN_STATUS")]
    #[inline]
    pub const fn is_i2c_enabled(self) -> bool {
        (self.0 & Self::I2C_EN_STATUS) != 0
    }
}

/// I2C status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Status(u32);

impl Status {
    const SDA_STUCK_NOT_RECOVERED: u32 = 0x1 << 11;
    const SLV_HOLD_RX_FIFO_FULL: u32 = 0x1 << 10;
    const SLV_HOLD_TX_FIFO_EMPTY: u32 = 0x1 << 9;
    const MST_HOLD_RX_FIFO_FULL: u32 = 0x1 << 8;
    const MST_HOLD_TX_FIFO_EMPTY: u32 = 0x1 << 7;
    const SLV_ACTIVITY: u32 = 0x1 << 6;
    const MST_ACTIVITY: u32 = 0x1 << 5;
    const RFF: u32 = 0x1 << 4;
    const RFNE: u32 = 0x1 << 3;
    const TFE: u32 = 0x1 << 2;
    const TFNF: u32 = 0x1 << 1;
    const ACTIVITY: u32 = 0x1;

    /// Check if sda stuck is not recovered (`SDA_STUCK_NOT_RECOVERED`).
    ///
    /// Only valid in master mode.
    #[doc(alias = "SDA_STUCK_NOT_RECOVERED")]
    #[inline]
    pub const fn is_sda_stuck_not_recovered(self) -> bool {
        (self.0 & Self::SDA_STUCK_NOT_RECOVERED) != 0
    }
    /// Check if slave holds the bus because rx fifo is full (`SLV_HOLD_RX_FIFO_FULL`).
    #[doc(alias = "SLV_HOLD_RX_FIFO_FULL")]
    #[inline]
    pub const fn is_slave_hold_rx_fifo_full(self) -> bool {
        (self.0 & Self::SLV_HOLD_RX_FIFO_FULL) != 0
    }
    /// Check if slave holds the bus because tx fifo is empty (`SLV_HOLD_TX_FIFO_EMPTY`).
    #[doc(alias = "SLV_HOLD_TX_FIFO_EMPTY")]
    #[inline]
    pub const fn is_slave_hold_tx_fifo_empty(self) -> bool {
        (self.0 & Self::SLV_HOLD_TX_FIFO_EMPTY) != 0
    }
    /// Check if master holds the bus because rx fifo is full (`MST_HOLD_RX_FIFO_FULL`).
    #[doc(alias = "MST_HOLD_RX_FIFO_FULL")]
    #[inline]
    pub const fn is_master_hold_rx_fifo_full(self) -> bool {
        (self.0 & Self::MST_HOLD_RX_FIFO_FULL) != 0
    }
    /// Check if master holds the bus because tx fifo is empty (`MST_HOLD_TX_FIFO_EMPTY`).
    #[doc(alias = "MST_HOLD_TX_FIFO_EMPTY")]
    #[inline]
    pub const fn is_master_hold_tx_fifo_empty(self) -> bool {
        (self.0 & Self::MST_HOLD_TX_FIFO_EMPTY) != 0
    }
    /// Check if slave mode is active (`SLV_ACTIVITY`).
    #[doc(alias = "SLV_ACTIVITY")]
    #[inline]
    pub const fn is_slave_active(self) -> bool {
        (self.0 & Self::SLV_ACTIVITY) != 0
    }
    /// Check if master mode is active (`MST_ACTIVITY`).
    #[doc(alias = "MST_ACTIVITY")]
    #[inline]
    pub const fn is_master_active(self) -> bool {
        (self.0 & Self::MST_ACTIVITY) != 0
    }
    /// Check if rx fifo is full (`RFF`).
    #[doc(alias = "RFF")]
    #[inline]
    pub const fn is_rx_fifo_full(self) -> bool {
        (self.0 & Self::RFF) != 0
    }
    /// Check if rx fifo is not empty (`RFNE`).
    #[doc(alias = "RFNE")]
    #[inline]
    pub const fn is_rx_fifo_not_empty(self) -> bool {
        (self.0 & Self::RFNE) != 0
    }
    /// Check if tx fifo is empty (`TFE`).
    #[doc(alias = "TFE")]
    #[inline]
    pub const fn is_tx_fifo_empty(self) -> bool {
        (self.0 & Self::TFE) != 0
    }
    /// Check if tx fifo is not full (`TFNF`).
    #[doc(alias = "TFNF")]
    #[inline]
    pub const fn is_tx_fifo_not_full(self) -> bool {
        (self.0 & Self::TFNF) != 0
    }
    /// Check if I2C is active (`ACTIVITY`).
    #[doc(alias = "ACTIVITY")]
    #[inline]
    pub const fn is_i2c_active(self) -> bool {
        (self.0 & Self::ACTIVITY) != 0
    }
}

/// I2C transport abort source register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TxAbortSource(u32);

impl TxAbortSource {
    const TX_FLUSH_CNT: u32 = 0x1FF << 23;
    const ABRT_SDA_STUCK_AT_LOW: u32 = 0x1 << 17;
    const ABRT_USER_ABRT: u32 = 0x1 << 16;
    const ABRT_SLVRD_INTX: u32 = 0x1 << 15;
    const ABRT_SLV_ARBLOST: u32 = 0x1 << 14;
    const ABRT_SLVFLUSH_TXFIFO: u32 = 0x1 << 13;
    const ABRT_LOST: u32 = 0x1 << 12;
    const ABRT_MASTER_DIS: u32 = 0x1 << 11;
    const ABRT_10B_RD_NORSTRT: u32 = 0x1 << 10;
    const ABRT_SBYTE_NORSTRT: u32 = 0x1 << 9;
    const ABRT_SBYTE_ACKDET: u32 = 0x1 << 7;
    const ABRT_GCALL_READ: u32 = 0x1 << 5;
    const ABRT_GCALL_NOACK: u32 = 0x1 << 4;
    const ABRT_TXDATA_NOACK: u32 = 0x1 << 3;
    const ABRT_10ADDR2_NOACK: u32 = 0x1 << 2;
    const ABRT_10ADDR1_NOACK: u32 = 0x1 << 1;
    const ABRT_7B_ADDR_NOACK: u32 = 0x1;

    /// Get tx flush count due to tx abort(`TX_FLUSH_CNT`).
    pub const fn tx_flush_cnt(self) -> u32 {
        (self.0 & Self::TX_FLUSH_CNT) >> 23
    }
    /// Check if abort is due to SDA stuck at low (`ABRT_SDA_STUCK_AT_LOW`).
    ///
    /// Valid only in master mode.
    #[doc(alias = "ABRT_SDA_STUCK_AT_LOW")]
    #[inline]
    pub const fn is_abrt_sda_stuck_at_low(self) -> bool {
        (self.0 & Self::ABRT_SDA_STUCK_AT_LOW) != 0
    }
    /// Check if abort is due to user abort (`ABRT_USER_ABRT`).
    ///
    /// Valid only in master mode.
    #[doc(alias = "ABRT_USER_ABRT")]
    #[inline]
    pub const fn is_abrt_user_abrt(self) -> bool {
        (self.0 & Self::ABRT_USER_ABRT) != 0
    }
    /// Check if abort is due to slave read interrupt (`ABRT_SLVRD_INTX`).
    #[doc(alias = "ABRT_SLVRD_INTX")]
    #[inline]
    pub const fn is_abrt_slvrd_intx(self) -> bool {
        (self.0 & Self::ABRT_SLVRD_INTX) != 0
    }
    /// Check if abort is due to slave arbitration lost (`ABRT_SLV_ARBLOST`).
    #[doc(alias = "ABRT_SLV_ARBLOST")]
    #[inline]
    pub const fn is_abrt_slv_arb_lost(self) -> bool {
        (self.0 & Self::ABRT_SLV_ARBLOST) != 0
    }
    /// Check if abort is due to slave flush tx fifo (`ABRT_SLVFLUSH_TXFIFO`).
    #[doc(alias = "ABRT_SLVFLUSH_TXFIFO")]
    #[inline]
    pub const fn is_abrt_slvflush_txfifo(self) -> bool {
        (self.0 & Self::ABRT_SLVFLUSH_TXFIFO) != 0
    }
    /// Check if abort is due to lost arbitration in master mode (`ABRT_LOST`).
    #[doc(alias = "ABRT_LOST")]
    #[inline]
    pub const fn is_abrt_lost(self) -> bool {
        (self.0 & Self::ABRT_LOST) != 0
    }
    /// Check if abort is due to master disabled (`ABRT_MASTER_DIS`).
    #[doc(alias = "ABRT_MASTER_DIS")]
    #[inline]
    pub const fn is_abrt_master_dis(self) -> bool {
        (self.0 & Self::ABRT_MASTER_DIS) != 0
    }
    /// Check if abort is due to 10-bit read without restart (`ABRT_10B_RD_NORSTRT`).
    #[doc(alias = "ABRT_10B_RD_NORSTRT")]
    #[inline]
    pub const fn is_abrt_10b_rd_no_rstrt(self) -> bool {
        (self.0 & Self::ABRT_10B_RD_NORSTRT) != 0
    }
    /// Check if abort is due to single byte no restart (`ABRT_SBYTE_NORSTRT`).
    #[doc(alias = "ABRT_SBYTE_NORSTRT")]
    #[inline]
    pub const fn is_abrt_sbyte_no_rstrt(self) -> bool {
        (self.0 & Self::ABRT_SBYTE_NORSTRT) != 0
    }
    /// Check if abort is due to single byte ACK detected (`ABRT_SBYTE_ACKDET`).
    #[doc(alias = "ABRT_SBYTE_ACKDET")]
    #[inline]
    pub const fn is_abrt_sbyte_ackdet(self) -> bool {
        (self.0 & Self::ABRT_SBYTE_ACKDET) != 0
    }
    /// Check if abort is due to general call read (`ABRT_GCALL_READ`).
    #[doc(alias = "ABRT_GCALL_READ")]
    #[inline]
    pub const fn is_abrt_gcall_read(self) -> bool {
        (self.0 & Self::ABRT_GCALL_READ) != 0
    }
    /// Check if abort is due to general no acknowledge (`ABRT_GCALL_NOACK`).
    #[doc(alias = "ABRT_GCALL_NOACK")]
    #[inline]
    pub const fn is_abrt_gcall_noack(self) -> bool {
        (self.0 & Self::ABRT_GCALL_NOACK) != 0
    }
    /// Check if abort is due to transmit data no acknowledge (`ABRT_TXDATA_NOACK`).
    #[doc(alias = "ABRT_TXDATA_NOACK")]
    #[inline]
    pub const fn is_abrt_txdata_noack(self) -> bool {
        (self.0 & Self::ABRT_TXDATA_NOACK) != 0
    }
    /// Check if abort is due to 10-bit address 2 no acknowledge (`ABRT_10ADDR2_NOACK`).
    #[doc(alias = "ABRT_10ADDR2_NOACK")]
    #[inline]
    pub const fn is_abrt_10addr2_noack(self) -> bool {
        (self.0 & Self::ABRT_10ADDR2_NOACK) != 0
    }
    /// Check if abort is due to 10-bit address 1 no acknowledge (`ABRT_10ADDR1_NOACK`).
    #[doc(alias = "ABRT_10ADDR1_NOACK")]
    #[inline]
    pub const fn is_abrt_10addr1_noack(self) -> bool {
        (self.0 & Self::ABRT_10ADDR1_NOACK) != 0
    }
    /// Check if abort is due to 7-bit address no acknowledge (`ABRT_7B_ADDR_NOACK`).
    #[doc(alias = "ABRT_7B_ADDR_NOACK")]
    #[inline]
    pub const fn is_abrt_7b_addr_noack(self) -> bool {
        (self.0 & Self::ABRT_7B_ADDR_NOACK) != 0
    }
}

/// I2C receive threshold register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct RxThreshold(u32);

impl RxThreshold {
    const RX_TL: u32 = 0x7;

    /// Set rx threshold (`RX_TL`).
    #[doc(alias = "RX_TL")]
    #[inline]
    pub const fn set_rx_threshold(self, thres: u8) -> Self {
        assert!(thres < 8, "Threshold out of range (expected 0..=8)");
        Self((self.0 & !Self::RX_TL) | ((thres as u32) & Self::RX_TL))
    }
    /// Get rx threshold.
    #[inline]
    pub const fn rx_threshold(self) -> u8 {
        (self.0 & Self::RX_TL) as u8
    }
}

/// I2C transport threshold register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TxThreshold(u32);

impl TxThreshold {
    const TX_TL: u32 = 0x7;

    /// Set tx threshold (`TX_TL`).
    #[doc(alias = "TX_TL")]
    #[inline]
    pub const fn set_tx_threshold(self, thres: u8) -> Self {
        assert!(thres < 8, "Threshold out of range (expected 0..=8)");
        Self((self.0 & !Self::TX_TL) | ((thres as u32) & Self::TX_TL))
    }
    /// Get tx threshold.
    #[inline]
    pub const fn tx_threshold(self) -> u8 {
        (self.0 & Self::TX_TL) as u8
    }
}

/// I2C transport FIFO level register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TxFifoLevel(u32);

impl TxFifoLevel {
    const TXFLR: u32 = 0xF;

    /// Get tx fifo count (`TXFLR`).
    #[inline]
    pub const fn tx_fifo_count(self) -> u8 {
        (self.0 & Self::TXFLR) as u8
    }
}

/// I2C receive FIFO level register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct RxFifoLevel(u32);

impl RxFifoLevel {
    const RXFLR: u32 = 0xF;

    /// Get rx fifo count (`RXFLR`).
    #[inline]
    pub const fn rx_fifo_count(self) -> u8 {
        (self.0 & Self::RXFLR) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AckGenCall, AddressingMode, Control, DataCommand, Enable, EnableStatus, FastSclHighCnt,
        FastSclLowCnt, InterruptClear, InterruptMask, RawInterruptStatus, RegisterBlock,
        RxFifoLevel, RxThreshold, SdaHold, SdaSetup, Slave, SpeedMode, StandardSclHighCnt,
        StandardSclLowCnt, Status, Target, TransferMode, TxAbortSource, TxFifoLevel, TxThreshold,
    };
    use crate::test_should_panic;
    use core::mem::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, ctrl), 0x0);
        assert_eq!(offset_of!(RegisterBlock, target), 0x4);
        assert_eq!(offset_of!(RegisterBlock, slave_addr), 0x8);
        assert_eq!(offset_of!(RegisterBlock, ack_gen_call), 0xC);
        assert_eq!(offset_of!(RegisterBlock, data_cmd), 0x10);
        assert_eq!(offset_of!(RegisterBlock, ss_scl_hcnt), 0x20);
        assert_eq!(offset_of!(RegisterBlock, ss_scl_lcnt), 0x24);
        assert_eq!(offset_of!(RegisterBlock, fs_scl_hcnt), 0x28);
        assert_eq!(offset_of!(RegisterBlock, fs_scl_lcnt), 0x2C);
        assert_eq!(offset_of!(RegisterBlock, sda_hold), 0x30);
        assert_eq!(offset_of!(RegisterBlock, sda_setup), 0x34);
        assert_eq!(offset_of!(RegisterBlock, intr_mask), 0x38);
        assert_eq!(offset_of!(RegisterBlock, intr_clear), 0x3C);
        assert_eq!(offset_of!(RegisterBlock, raw_intr_stat), 0x40);
        assert_eq!(offset_of!(RegisterBlock, enable), 0x48);
        assert_eq!(offset_of!(RegisterBlock, enable_status), 0x4C);
        assert_eq!(offset_of!(RegisterBlock, status), 0x50);
        assert_eq!(offset_of!(RegisterBlock, tx_abrt_source), 0x54);
        assert_eq!(offset_of!(RegisterBlock, rx_tl), 0x90);
        assert_eq!(offset_of!(RegisterBlock, tx_tl), 0x94);
        assert_eq!(offset_of!(RegisterBlock, tx_flr), 0x98);
        assert_eq!(offset_of!(RegisterBlock, rx_flr), 0x9C);
        assert_eq!(offset_of!(RegisterBlock, scl_stuck_timeout), 0xA0);
        assert_eq!(offset_of!(RegisterBlock, sda_stuck_timeout), 0xA4);
        assert_eq!(offset_of!(RegisterBlock, fs_spklen), 0xB0);
        assert_eq!(offset_of!(RegisterBlock, version), 0xFC);
    }

    #[test]
    fn struct_control_functions() {
        let mut val = Control(0x0);

        val = val.enable_bus_clear_feature();
        assert!(val.is_bus_clear_feature_enabled());
        assert_eq!(val.0, 0x0000_0400);
        val = val.disable_bus_clear_feature();
        assert!(!val.is_bus_clear_feature_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_rx_fifo_full_hold();
        assert!(val.is_rx_fifo_full_hold_enabled());
        assert_eq!(val.0, 0x0000_0200);
        val = val.disable_rx_fifo_full_hold();
        assert!(!val.is_rx_fifo_full_hold_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_tx_empty_control(true);
        assert!(val.tx_empty_control());
        assert_eq!(val.0, 0x0000_0100);
        val = val.set_tx_empty_control(false);
        assert!(!val.tx_empty_control());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_stop_detect_if_addressed(true);
        assert!(val.stop_detect_if_addressed());
        assert_eq!(val.0, 0x0000_0080);
        val = val.set_stop_detect_if_addressed(false);
        assert!(!val.stop_detect_if_addressed());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_restart();
        assert!(val.is_restart_enabled());
        assert_eq!(val.0, 0x0000_0040);
        val = val.disable_restart();
        assert!(!val.is_restart_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_speed_mode(SpeedMode::Fast);
        assert_eq!(val.speed_mode(), SpeedMode::Fast);
        assert_eq!(val.0, 0x0000_0020);
        val = val.set_speed_mode(SpeedMode::Standard);
        assert_eq!(val.speed_mode(), SpeedMode::Standard);
        assert_eq!(val.0, 0x0000_0010);

        val = Control(0x0).set_addressing_mode_slave(AddressingMode::Bit10);
        assert_eq!(val.addressing_mode_slave(), AddressingMode::Bit10);
        assert_eq!(val.0, 0x0000_0008);
        val = val.set_addressing_mode_slave(AddressingMode::Bit7);
        assert_eq!(val.addressing_mode_slave(), AddressingMode::Bit7);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_addressing_mode_master(AddressingMode::Bit10);
        assert_eq!(val.addressing_mode_master(), AddressingMode::Bit10);
        assert_eq!(val.0, 0x0000_0004);
        val = val.set_addressing_mode_master(AddressingMode::Bit7);
        assert_eq!(val.addressing_mode_master(), AddressingMode::Bit7);
        assert_eq!(val.0, 0x0000_0000);

        val = val.disable_slave_mode();
        assert!(!val.is_slave_mode_enabled());
        assert_eq!(val.0, 0x0000_0002);
        val = val.enable_slave_mode();
        assert!(val.is_slave_mode_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_master_mode();
        assert!(val.is_master_mode_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_master_mode();
        assert!(!val.is_master_mode_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_target_functions() {
        let mut val = Target(0x0);

        val = val.enable_general_call();
        assert!(val.is_general_call_enabled());
        assert_eq!(val.0, 0x0000_0800);
        val = val.disable_general_call();
        assert!(!val.is_general_call_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_start_byte();
        assert!(val.is_start_byte_enabled());
        assert_eq!(val.0, 0x0000_0400);
        val = val.disable_start_byte();
        assert!(!val.is_start_byte_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_target_address(0x3F);
        assert_eq!(val.target_address(), 0x3F);
        assert_eq!(val.0, 0x0000_003F);
    }

    test_should_panic!((
        test_set_target_address_panic,
        Target(0x0).set_target_address(0x400),
        "Address out of range (expected 0..=0x3FF)"
    ),);

    #[test]
    fn struct_slave_functions() {
        let val = Slave(0x0).set_slave_address(0x3FF);
        assert_eq!(val.slave_address(), 0x3FF);
        assert_eq!(val.0, 0x0000_03FF);
    }

    test_should_panic!((
        test_set_slave_address_panic,
        Slave(0x0).set_slave_address(0x400),
        "Address out of range (expected 0..=0x3FF)"
    ),);

    #[test]
    fn struct_ack_gen_call_functions() {
        let mut val = AckGenCall(0x0);

        val = val.enable_ack_general_call();
        assert!(val.is_ack_general_call_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_ack_general_call();
        assert!(!val.is_ack_general_call_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_data_command_functions() {
        let mut val = DataCommand(0x0);

        val = val.set_restart(true);
        assert_eq!(val.0, 0x0000_0400);
        val = val.set_restart(false);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_stop(true);
        assert_eq!(val.0, 0x0000_0200);
        val = val.set_stop(false);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_transfer_mode(TransferMode::Read);
        assert_eq!(val.0, 0x0000_0100);
        val = val.set_transfer_mode(TransferMode::Write);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_data_byte(0xFF);
        assert_eq!(val.data_byte(), 0xFF);
        assert_eq!(val.0, 0x0000_00FF);
    }

    #[test]
    fn struct_standard_scl_high_cnt_functions() {
        let val = StandardSclHighCnt(0x0).set_scl_high_count(0xFFFF);
        assert_eq!(val.scl_high_count(), 0xFFFF);
        assert_eq!(val.0, 0x0000_FFFF);
    }

    test_should_panic!((
        test_set_scl_high_count_panic,
        StandardSclHighCnt(0x0).set_scl_high_count(0x1),
        "Standard SCL high count must be at least 6"
    ));

    #[test]
    fn struct_standard_scl_low_cnt_functions() {
        let val = StandardSclLowCnt(0x0).set_scl_low_count(0xFFFF);
        assert_eq!(val.scl_low_count(), 0xFFFF);
        assert_eq!(val.0, 0x0000_FFFF);
    }

    test_should_panic!((
        test_set_scl_low_count_panic,
        StandardSclLowCnt(0x0).set_scl_low_count(0x1),
        "Standard SCL low count must be at least 8"
    ));

    #[test]
    fn struct_fast_scl_high_cnt_functions() {
        let val = FastSclHighCnt(0x0).set_scl_high_count(0xFFFF);
        assert_eq!(val.scl_high_count(), 0xFFFF);
        assert_eq!(val.0, 0x0000_FFFF);
    }

    test_should_panic!((
        test_set_fast_scl_high_count_panic,
        FastSclHighCnt(0x0).set_scl_high_count(0x1),
        "Fast SCL high count must be at least 6"
    ));

    #[test]
    fn struct_fast_scl_low_cnt_functions() {
        let val = FastSclLowCnt(0x0).set_scl_low_count(0xFFFF);
        assert_eq!(val.scl_low_count(), 0xFFFF);
        assert_eq!(val.0, 0x0000_FFFF);
    }

    test_should_panic!((
        test_set_fast_scl_low_count_panic,
        FastSclLowCnt(0x0).set_scl_low_count(0x1),
        "Fast SCL low count must be at least 8"
    ));

    #[test]
    fn struct_sda_hold_functions() {
        let mut val = SdaHold(0x0);

        val = val.set_sda_rx_hold(0xFFFF);
        assert_eq!(val.sda_rx_hold(), 0xFFFF);
        assert_eq!(val.0, 0xFFFF_0000);

        val = SdaHold(0x0).set_sda_tx_hold(0xFFFF);
        assert_eq!(val.sda_tx_hold(), 0xFFFF);
        assert_eq!(val.0, 0x0000_FFFF);
    }

    #[test]
    fn struct_sda_setup_functions() {
        let mut val = SdaSetup(0x0);

        val = val.set_sda_setup(0xFF);
        assert_eq!(val.sda_setup(), 0xFF);
        assert_eq!(val.0, 0x0000_00FF);
    }

    test_should_panic!((
        test_set_sda_setup_panic,
        SdaSetup(0x0).set_sda_setup(0x1),
        "SDA setup time must be at least 2 clock cycles"
    ),);

    #[test]
    fn struct_interrupt_mask_functions() {
        let mut val = InterruptMask(0x0);

        val = val.enable_scl_stuck_at_low();
        assert!(val.is_scl_stuck_at_low_enabled());
        assert_eq!(val.0, 0x0000_4000);
        val = val.disable_scl_stuck_at_low();
        assert!(!val.is_scl_stuck_at_low_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_master_on_hold();
        assert!(val.is_master_on_hold_enabled());
        assert_eq!(val.0, 0x0000_2000);
        val = val.disable_master_on_hold();
        assert!(!val.is_master_on_hold_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_general_call();
        assert!(val.is_general_call_enabled());
        assert_eq!(val.0, 0x0000_0800);
        val = val.disable_general_call();
        assert!(!val.is_general_call_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_start_detect();
        assert!(val.is_start_detect_enabled());
        assert_eq!(val.0, 0x0000_0400);
        val = val.disable_start_detect();
        assert!(!val.is_start_detect_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_stop_detect();
        assert!(val.is_stop_detect_enabled());
        assert_eq!(val.0, 0x0000_0200);
        val = val.disable_stop_detect();
        assert!(!val.is_stop_detect_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_activity();
        assert!(val.is_activity_enabled());
        assert_eq!(val.0, 0x0000_0100);
        val = val.disable_activity();
        assert!(!val.is_activity_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_rx_done();
        assert!(val.is_rx_done_enabled());
        assert_eq!(val.0, 0x0000_0080);
        val = val.disable_rx_done();
        assert!(!val.is_rx_done_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_tx_abort();
        assert!(val.is_tx_abort_enabled());
        assert_eq!(val.0, 0x0000_0040);
        val = val.disable_tx_abort();
        assert!(!val.is_tx_abort_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_read_request();
        assert!(val.is_read_request_enabled());
        assert_eq!(val.0, 0x0000_0020);
        val = val.disable_read_request();
        assert!(!val.is_read_request_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_tx_empty();
        assert!(val.is_tx_empty_enabled());
        assert_eq!(val.0, 0x0000_0010);
        val = val.disable_tx_empty();
        assert!(!val.is_tx_empty_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_rx_full();
        assert!(val.is_rx_full_enabled());
        assert_eq!(val.0, 0x0000_0004);
        val = val.disable_rx_full();
        assert!(!val.is_rx_full_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_rx_under();
        assert!(val.is_rx_under_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_rx_under();
        assert!(!val.is_rx_under_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_interrupt_clear_functions() {
        let mut val = InterruptClear(0x0);

        val = val.clear_scl_stuck_at_low();
        assert_eq!(val.0, 0x0000_4000);

        val = InterruptClear(0x0).clear_master_on_hold();
        assert_eq!(val.0, 0x0000_2000);

        val = InterruptClear(0x0).clear_general_call();
        assert_eq!(val.0, 0x0000_0800);

        val = InterruptClear(0x0).clear_start_detect();
        assert_eq!(val.0, 0x0000_0400);

        val = InterruptClear(0x0).clear_stop_detect();
        assert_eq!(val.0, 0x0000_0200);

        val = InterruptClear(0x0).clear_activity();
        assert_eq!(val.0, 0x0000_0100);

        val = InterruptClear(0x0).clear_rx_done();
        assert_eq!(val.0, 0x0000_0080);

        val = InterruptClear(0x0).clear_tx_abort();
        assert_eq!(val.0, 0x0000_0040);

        val = InterruptClear(0x0).clear_read_request();
        assert_eq!(val.0, 0x0000_0020);

        val = InterruptClear(0x0).clear_tx_empty();
        assert_eq!(val.0, 0x0000_0010);

        val = InterruptClear(0x0).clear_rx_full();
        assert_eq!(val.0, 0x0000_0004);

        val = InterruptClear(0x0).clear_rx_under();
        assert_eq!(val.0, 0x0000_0001);
    }

    #[test]
    fn struct_raw_interrupt_status_functions() {
        let mut val = RawInterruptStatus(0x0000_4000);
        assert!(val.is_scl_stuck_at_low_pending());

        val = RawInterruptStatus(0x0000_2000);
        assert!(val.is_master_on_hold_pending());

        val = RawInterruptStatus(0x0000_0800);
        assert!(val.is_general_call_pending());

        val = RawInterruptStatus(0x0000_0400);
        assert!(val.is_start_detect_pending());

        val = RawInterruptStatus(0x0000_0200);
        assert!(val.is_stop_detect_pending());

        val = RawInterruptStatus(0x0000_0100);
        assert!(val.is_activity_pending());

        val = RawInterruptStatus(0x0000_0080);
        assert!(val.is_rx_done_pending());

        val = RawInterruptStatus(0x0000_0040);
        assert!(val.is_tx_abort_pending());

        val = RawInterruptStatus(0x0000_0020);
        assert!(val.is_read_request_pending());

        val = RawInterruptStatus(0x0000_0010);
        assert!(val.is_tx_empty_pending());

        val = RawInterruptStatus(0x0000_0004);
        assert!(val.is_rx_full_pending());

        val = RawInterruptStatus(0x0000_0001);
        assert!(val.is_rx_under_pending());
    }

    #[test]
    fn struct_enable_functions() {
        let mut val = Enable(0x0);

        val = val.enable_sda_stuck_recovery();
        assert!(val.is_sda_stuck_recovery_enabled());
        assert_eq!(val.0, 0x0000_0008);
        val = val.disable_sda_stuck_recovery();
        assert!(!val.is_sda_stuck_recovery_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_abort(true);
        assert!(val.abort());
        assert_eq!(val.0, 0x0000_0002);
        val = val.set_abort(false);
        assert!(!val.abort());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_i2c();
        assert!(val.is_i2c_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_i2c();
        assert!(!val.is_i2c_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_enable_status_functions() {
        let mut val = EnableStatus(0x0000_0004);
        assert!(val.is_slave_rx_data_lost());

        val = EnableStatus(0x0000_0002);
        assert!(!val.is_slave_disabled_while_busy());

        val = EnableStatus(0x0000_0001);
        assert!(val.is_i2c_enabled());
    }

    #[test]
    fn struct_status_functions() {
        let mut val = Status(0x0000_0800);
        assert!(val.is_sda_stuck_not_recovered());

        val = Status(0x0000_0400);
        assert!(val.is_slave_hold_rx_fifo_full());

        val = Status(0x0000_0200);
        assert!(val.is_slave_hold_tx_fifo_empty());

        val = Status(0x0000_0100);
        assert!(val.is_master_hold_rx_fifo_full());

        val = Status(0x0000_0080);
        assert!(val.is_master_hold_tx_fifo_empty());

        val = Status(0x0000_0040);
        assert!(val.is_slave_active());

        val = Status(0x0000_0020);
        assert!(val.is_master_active());

        val = Status(0x0000_0010);
        assert!(val.is_rx_fifo_full());

        val = Status(0x0000_0008);
        assert!(val.is_rx_fifo_not_empty());

        val = Status(0x0000_0004);
        assert!(val.is_tx_fifo_empty());

        val = Status(0x0000_0002);
        assert!(val.is_tx_fifo_not_full());

        val = Status(0x0000_0001);
        assert!(val.is_i2c_active());
    }

    #[test]
    fn struct_tx_abort_source_functions() {
        let mut val = TxAbortSource(0xFF80_0000);
        assert_eq!(val.tx_flush_cnt(), 0x1FF);

        val = TxAbortSource(0x0002_0000);
        assert!(val.is_abrt_sda_stuck_at_low());

        val = TxAbortSource(0x0001_0000);
        assert!(val.is_abrt_user_abrt());

        val = TxAbortSource(0x0000_8000);
        assert!(val.is_abrt_slvrd_intx());

        val = TxAbortSource(0x0000_4000);
        assert!(val.is_abrt_slv_arb_lost());

        val = TxAbortSource(0x0000_2000);
        assert!(val.is_abrt_slvflush_txfifo());

        val = TxAbortSource(0x0000_1000);
        assert!(val.is_abrt_lost());

        val = TxAbortSource(0x0000_0800);
        assert!(val.is_abrt_master_dis());

        val = TxAbortSource(0x0000_0400);
        assert!(val.is_abrt_10b_rd_no_rstrt());

        val = TxAbortSource(0x0000_0200);
        assert!(val.is_abrt_sbyte_no_rstrt());

        val = TxAbortSource(0x0000_0080);
        assert!(val.is_abrt_sbyte_ackdet());

        val = TxAbortSource(0x0000_0020);
        assert!(val.is_abrt_gcall_read());

        val = TxAbortSource(0x0000_0010);
        assert!(val.is_abrt_gcall_noack());

        val = TxAbortSource(0x0000_0008);
        assert!(val.is_abrt_txdata_noack());

        val = TxAbortSource(0x0000_0004);
        assert!(val.is_abrt_10addr2_noack());

        val = TxAbortSource(0x0000_0002);
        assert!(val.is_abrt_10addr1_noack());

        val = TxAbortSource(0x0000_0001);
        assert!(val.is_abrt_7b_addr_noack());
    }

    #[test]
    fn struct_rx_threshold_functions() {
        let mut val = RxThreshold(0x0);
        val = val.set_rx_threshold(0x7);
        assert_eq!(val.rx_threshold(), 0x7);
        assert_eq!(val.0, 0x0000_0007);
    }

    test_should_panic!((
        test_set_rx_threshold_panic,
        RxThreshold(0x0).set_rx_threshold(0x8),
        "Threshold out of range (expected 0..=8)"
    ),);

    #[test]
    fn struct_tx_threshold_functions() {
        let mut val = TxThreshold(0x0);
        val = val.set_tx_threshold(0x7);
        assert_eq!(val.tx_threshold(), 0x7);
        assert_eq!(val.0, 0x0000_0007);
    }

    test_should_panic!((
        test_set_tx_threshold_panic,
        TxThreshold(0x0).set_tx_threshold(0x8),
        "Threshold out of range (expected 0..=8)"
    ),);

    #[test]
    fn struct_tx_fifo_level() {
        let val = TxFifoLevel(0x0000_000F);
        assert_eq!(val.tx_fifo_count(), 0xF);
    }

    #[test]
    fn struct_rx_fifo_level() {
        let val = RxFifoLevel(0x0000_000F);
        assert_eq!(val.rx_fifo_count(), 0xF);
    }
}
