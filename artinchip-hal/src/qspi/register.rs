//! QSPI register blocks and registers.

use volatile_register::{RO, RW, WO};

/// Quad Serial Peripheral Interface Register Block.
#[repr(C)]
pub struct RegisterBlock {
    /// SPI version register (`SPI_VER`).
    #[doc(alias = "SPI_VER")]
    pub version: RO<Version>,
    /// SPI configuration register (`SPI_CFG`).
    #[doc(alias = "SPI_CFG")]
    pub config: RW<Config>,
    /// SPI transfer configuration register (`SPI_TCFG`).
    #[doc(alias = "SPI_TCFG")]
    pub trans_config: RW<TransferConfig>,
    _reserved0: [u8; 0x4],
    /// SPI interrupt control register (`SPI_ICR`).
    #[doc(alias = "SPI_ICR")]
    pub int_control: RW<IntControl>,
    /// SPI interrupt status register (`SPI_ISTS`).
    #[doc(alias = "SPI_ISTS")]
    pub int_status: RW<IntStatus>,
    /// SPI fifo control register (`SPI_FCTL`).
    #[doc(alias = "SPI_FCTL")]
    pub fifo_control: RW<FifoControl>,
    /// SPI fifo status register (`SPI_FSTS`).
    #[doc(alias = "SPI_FSTS")]
    pub fifo_status: RO<FifoStatus>,
    _reserved1: [u8; 0x4],
    /// SPI clock configuration register (`SPI_CCFG`).
    #[doc(alias = "SPI_CCFG")]
    pub clk_config: RW<ClkConfig>,
    /// TODO: unknown bitfield
    /// SPI miscellaneous configuration register (`SPI_MISC`).
    #[doc(alias = "SPI_MISC")]
    pub misc: RW<u32>,
    _reserved2: [u8; 0x4],
    /// SPI total bytes counter register (`SPI_TBC`).
    #[doc(alias = "SPI_TBC")]
    pub total_bytes_cnt: RW<TotalBytesCnt>,
    /// SPI transmit write counter register (`SPI_TWC`).
    #[doc(alias = "SPI_TWC")]
    pub trans_write_cnt: RW<TransCnt>,
    /// SPI transmit miscellaneous control register (`SPI_TMC`).
    #[doc(alias = "SPI_TMC")]
    pub trans_misc_control: RW<TransMiscControl>,
    _reserved3: [u8; 0x4],
    /// SPI bit-mode transfer configuration register (`SPI_BMTC`).
    #[doc(alias = "SPI_BMTC")]
    pub bit_mode_trans_config: RW<BitModeTransConfig>,
    /// SPI bit-mode clock configuration register (`SPI_BMCLK`).
    #[doc(alias = "SPI_BMCLK")]
    pub bit_mode_clk_config: RW<BitModeClkConfig>,
    /// SPI bit-mode tx data register (`SPI_BMTXD`).
    #[doc(alias = "SPI_BMTXD")]
    pub bit_mode_tx_data: RW<u32>,
    /// SPI bit-mode rx data register (`SPI_BMRXD`).
    #[doc(alias = "SPI_BMRXD")]
    pub bit_mode_rx_data: RW<u32>,
    /// SPI burst set register (`SPI_BTR`).
    #[doc(alias = "SPI_BTR")]
    pub burst_set: RW<BurstSet>,
    /// SPI read command mode register (`SPI_RCM`).
    #[doc(alias = "SPI_RCM")]
    pub read_cmd: RW<ReadCmdMode>,
    _reserved4: [u8; 0x8],
    /// SPI wrap length register (`SPI_WRAP_LEN`).
    #[doc(alias = "SPI_WRAP_LEN")]
    pub wrap_len: RW<WrapLen>,
    _reserved5: [u8; 0x28],
    /// SPI inner dma write transaction length register (`SPI_IDMA_TXLEN`).
    #[doc(alias = "SPI_IDMA_TXLEN")]
    pub idma_tx_len: RW<IdmaTxLen>,
    /// SPI inner dma read transaction length register (`SPI_IDMA_RXLEN`).
    #[doc(alias = "SPI_IDMA_RXLEN")]
    pub idma_rx_len: RW<IdmaRxLen>,
    /// SPI inner dma tx source address register (`SPI_IDMA_TXADDR`).
    #[doc(alias = "SPI_IDMA_TXADDR")]
    pub idma_tx_addr: RW<u32>,
    /// SPI inner dma rx source address register (`SPI_IDMA_RXADDR`).
    #[doc(alias = "SPI_IDMA_RXADDR")]
    pub idma_rx_addr: RW<u32>,
    /// SPI inner dma burst config register (`SPI_IDMA_BTCFG`).
    #[doc(alias = "SPI_IDMA_BTCFG")]
    pub idma_burst_cfg: RW<IdmaBurstConfig>,
    _reserved6: [u8; 0x160],
    /// SPI tx data register (`SPI_TXD`).
    #[doc(alias = "SPI_TXD")]
    pub tx_data: WO<u32>,
    _reserved7: [u8; 0xFC],
    /// SPI rx data register (`SPI_RXD`).
    #[doc(alias = "SPI_RXD")]
    pub rx_data: RO<u32>,
}

/// SPI version register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Version(u32);

impl Version {
    const VER: u32 = 0x0000_FFFF;

    /// Get the version number (`VER`).
    #[doc(alias = "VER")]
    #[inline]
    pub const fn version(self) -> u16 {
        (self.0 & Self::VER) as u16
    }
}

/// Access mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AccessMode {
    AHB,
    AXI,
}

/// Controller work mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WorkMode {
    Slave,
    Master,
}

/// SPI configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Config(u32);

impl Config {
    const CTRL_RST: u32 = 0x1 << 31;
    const RXFULL_STOP: u32 = 0x1 << 7;
    const TXIDMA_EN: u32 = 0x1 << 4;
    const RXIDMA_EN: u32 = 0x1 << 3;
    const AMOD: u32 = 0x1 << 2;
    const CTRL_MODE_SEL: u32 = 0x1 << 1;
    const CTRL_EN: u32 = 0x1;

    /// Set controller reset bit (`CTRL_RST`).
    #[doc(alias = "CTRL_RST")]
    #[inline]
    pub const fn set_ctrl_rst(self, reset: bool) -> Self {
        if reset {
            Self(self.0 | Self::CTRL_RST)
        } else {
            Self(self.0 & !Self::CTRL_RST)
        }
    }
    /// Get controller reset bit.
    #[inline]
    pub const fn ctrl_rst(self) -> bool {
        (self.0 & Self::CTRL_RST) != 0
    }
    /// Enable fifo full transfer stop (`RXFULL_STOP`).
    #[doc(alias = "RXFULL_STOP")]
    #[inline]
    pub const fn enable_rx_full_stop(self) -> Self {
        Self(self.0 | Self::RXFULL_STOP)
    }
    /// Disable fifo full transfer stop.
    #[inline]
    pub const fn disable_rx_full_stop(self) -> Self {
        Self(self.0 & !Self::RXFULL_STOP)
    }
    /// Check if fifo full transfer stop is enabled.
    #[inline]
    pub const fn is_rx_full_stop_enabled(self) -> bool {
        (self.0 & Self::RXFULL_STOP) != 0
    }
    /// Enable inner dma for tx channel (`TXIDMA_EN`).
    #[doc(alias = "TXIDMA_EN")]
    #[inline]
    pub const fn enable_tx_idma(self) -> Self {
        Self(self.0 | Self::TXIDMA_EN)
    }
    /// Disable inner dma for tx channel.
    #[inline]
    pub const fn disable_tx_idma(self) -> Self {
        Self(self.0 & !Self::TXIDMA_EN)
    }
    /// Check if inner dma for tx channel is enabled.
    #[inline]
    pub const fn is_tx_idma_enabled(self) -> bool {
        (self.0 & Self::TXIDMA_EN) != 0
    }
    /// Enable inner dma for rx channel (`RXIDMA_EN`).
    #[doc(alias = "RXIDMA_EN")]
    #[inline]
    pub const fn enable_rx_idma(self) -> Self {
        Self(self.0 | Self::RXIDMA_EN)
    }
    /// Disable inner dma for rx channel.
    #[inline]
    pub const fn disable_rx_idma(self) -> Self {
        Self(self.0 & !Self::RXIDMA_EN)
    }
    /// Check if inner dma for rx channel is enabled.
    #[inline]
    pub const fn is_rx_idma_enabled(self) -> bool {
        (self.0 & Self::RXIDMA_EN) != 0
    }
    /// Set access mode (`AMOD`).
    ///
    /// Access mode must be AXI when enabling XIP, and AHB when disabling it.
    #[doc(alias = "AMOD")]
    #[inline]
    pub const fn set_access_mode(self, mode: AccessMode) -> Self {
        Self((self.0 & !Self::AMOD) | (Self::AMOD & ((mode as u32) << 2)))
    }
    /// Get access mode.
    #[inline]
    pub const fn access_mode(self) -> AccessMode {
        match (self.0 & Self::AMOD) >> 2 {
            0 => AccessMode::AHB,
            1 => AccessMode::AXI,
            _ => unreachable!(),
        }
    }
    /// Set controller work mode (`CTRL_MODE_SEL`).
    ///
    /// Not writable when `START` = 1.
    #[doc(alias = "CTRL_MODE_SEL")]
    #[inline]
    pub const fn set_work_mode(self, mode: WorkMode) -> Self {
        Self((self.0 & !Self::CTRL_MODE_SEL) | (Self::CTRL_MODE_SEL & ((mode as u32) << 1)))
    }
    /// Get controller work mode.
    #[inline]
    pub const fn work_mode(self) -> WorkMode {
        match (self.0 & Self::CTRL_MODE_SEL) >> 1 {
            0 => WorkMode::Slave,
            1 => WorkMode::Master,
            _ => unreachable!(),
        }
    }
    /// Enable controller (`CTRL_EN`).
    ///
    /// Must enable once when changing transfer mode from bits to bytes.
    #[inline]
    pub const fn enable_ctrl(self) -> Self {
        Self(self.0 | Self::CTRL_EN)
    }
    /// Disable controller.
    #[inline]
    pub const fn disable_ctrl(self) -> Self {
        Self(self.0 & !Self::CTRL_EN)
    }
    /// Check if controller is enabled.
    #[inline]
    pub const fn is_ctrl_enabled(self) -> bool {
        (self.0 & Self::CTRL_EN) != 0
    }
}

/// CS level.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CsLevel {
    Low,
    High,
}

/// CS control mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CsCtrlMode {
    SpiController,
    Software,
}

/// External device cs pin number.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CsPin {
    /// CS0 is valid.
    Cs0,
    /// CS1 is valid.
    Cs1,
}

/// CS valid mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CsValidMode {
    /// CS stays active during the entire SPI transfer.
    Continuous,
    /// CS becomes inactive when the transfer is idle.
    InactiveWhenIdle,
}

/// CS polarity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CsPolarity {
    /// Valid when CS is low.
    Low,
    /// Valid when CS is high.
    High,
}

/// Clock polarity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClockPolarity {
    /// Clock is low when idle.
    Low,
    /// Clock is high when idle.
    High,
}

/// Clock phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClockPhase {
    /// Data sampled on the first edge of the clock.
    FirstEdge,
    /// Data sampled on the second edge of the clock.
    SecondEdge,
}

/// SPI transfer configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TransferConfig(u32);

impl TransferConfig {
    const START: u32 = 0x1 << 31;
    const SLV_OEN: u32 = 0x1 << 26;
    const THREE_WIRE_EN: u32 = 0x1 << 25;
    const TXDLY_EN: u32 = 0x1 << 14;
    const RXDLY_DIS: u32 = 0x1 << 13;
    const LSB_EN: u32 = 0x1 << 12;
    const RXINDLY_EN: u32 = 0x1 << 11;
    const HSWM: u32 = 0x1 << 10;
    const DMY_VAL: u32 = 0x1 << 9;
    const DINVD: u32 = 0x1 << 8;
    const CS_LEVEL: u32 = 0x1 << 7;
    const CS_CTL_SEL: u32 = 0x1 << 6;
    const CS_NUM: u32 = 0x3 << 4;
    const CS_VALID_CTL: u32 = 0x1 << 3;
    const CS_POL: u32 = 0x1 << 2;
    const CPOL: u32 = 0x1 << 1;
    const CPHA: u32 = 0x1 << 0;

    /// Set start bit (`START`).
    ///
    /// - 0: Idle state.
    /// - 1: Start data transmission.
    ///
    /// Set to 1 to begin SPI data transmission; it will automatically reset to 0 upon completion.
    /// Not writable when `START` = 1.
    #[doc(alias = "START")]
    #[inline]
    pub const fn set_start(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::START)
        } else {
            Self(self.0 & !Self::START)
        }
    }
    /// Get start bit.
    #[inline]
    pub const fn start(self) -> bool {
        (self.0 & Self::START) != 0
    }
    /// Enable slave mode output (`SLV_OEN`).
    ///
    /// In slave mode, Dual/Quad I/O mode output needs to be enabled,
    /// whereas single wire mode does not require configuration.
    #[doc(alias = "SLV_OEN")]
    #[inline]
    pub const fn enable_slave_output(self) -> Self {
        Self(self.0 | Self::SLV_OEN)
    }
    /// Disable slave mode output.
    #[inline]
    pub const fn disable_slave_output(self) -> Self {
        Self(self.0 & !Self::SLV_OEN)
    }
    /// Check if slave mode output is enabled.
    #[inline]
    pub const fn is_slave_output_enabled(self) -> bool {
        (self.0 & Self::SLV_OEN) != 0
    }
    /// Enable 3-wire byte mode (`THREE_WIRE_EN`).
    #[doc(alias = "THREE_WIRE_EN")]
    #[inline]
    pub const fn enable_three_wire(self) -> Self {
        Self(self.0 | Self::THREE_WIRE_EN)
    }
    /// Disable 3-wire byte mode.
    #[inline]
    pub const fn disable_three_wire(self) -> Self {
        Self(self.0 & !Self::THREE_WIRE_EN)
    }
    /// Check if 3-wire byte mode is enabled.
    #[inline]
    pub const fn is_three_wire_enabled(self) -> bool {
        (self.0 & Self::THREE_WIRE_EN) != 0
    }
    /// Enable tx data delay mode (`TXDLY_EN`).
    ///
    /// Must be enabled when using dual/quad mode;
    /// The transmission of data is delayed by half a clock cycle..
    #[doc(alias = "TXDLY_EN")]
    #[inline]
    pub const fn enable_tx_data_delay(self) -> Self {
        Self(self.0 | Self::TXDLY_EN)
    }
    /// Disable tx data delay mode.
    #[inline]
    pub const fn disable_tx_data_delay(self) -> Self {
        Self(self.0 & !Self::TXDLY_EN)
    }
    /// Check if tx data delay mode is enabled.
    #[inline]
    pub const fn is_tx_data_delay_enabled(self) -> bool {
        (self.0 & Self::TXDLY_EN) != 0
    }
    /// Enable rx delay sample mode (`RXDLY_DIS`).
    ///
    /// In normal sampling mode, SPI samples data on the edge;
    /// In delayed sampling mode, SPI delays by half a clock cycle before sampling on the next edge.
    #[doc(alias = "RXDLY_DIS")]
    #[inline]
    pub const fn enable_rx_data_delay(self) -> Self {
        Self(self.0 & !Self::RXDLY_DIS)
    }
    /// Disable rx delay sample mode.
    #[inline]
    pub const fn disable_rx_data_delay(self) -> Self {
        Self(self.0 | Self::RXDLY_DIS)
    }
    /// Check if rx delay sample mode is enabled.
    #[inline]
    pub const fn is_rx_data_delay_enabled(self) -> bool {
        (self.0 & Self::RXDLY_DIS) == 0
    }
    /// Enable low significant bit transmit (`LSB_EN`).
    ///
    /// Not writable when `START` = 1.
    #[doc(alias = "LSB_EN")]
    #[inline]
    pub const fn enable_lsb_transmit(self) -> Self {
        Self(self.0 | Self::LSB_EN)
    }
    /// Disable low significant bit transmit.
    #[inline]
    pub const fn disable_lsb_transmit(self) -> Self {
        Self(self.0 & !Self::LSB_EN)
    }
    /// Check if low significant bit transmit is enabled.
    #[inline]
    pub const fn is_lsb_transmit_enabled(self) -> bool {
        (self.0 & Self::LSB_EN) != 0
    }
    /// Enable rx data inner sample delay mode (`RXINDLY_EN`).
    #[doc(alias = "RXINDLY_EN")]
    #[inline]
    pub const fn enable_rx_inner_delay(self) -> Self {
        Self(self.0 | Self::RXINDLY_EN)
    }
    /// Disable rx data inner sample delay mode.
    #[inline]
    pub const fn disable_rx_inner_delay(self) -> Self {
        Self(self.0 & !Self::RXINDLY_EN)
    }
    /// Check if rx data inner sample delay mode is enabled.
    #[inline]
    pub const fn is_rx_inner_delay_enabled(self) -> bool {
        (self.0 & Self::RXINDLY_EN) != 0
    }
    /// Enable high speed write mode (`HSWM`).
    ///
    /// Not writable when `START` = 1.
    #[doc(alias = "HSWM")]
    #[inline]
    pub const fn enable_high_speed_write(self) -> Self {
        Self(self.0 | Self::HSWM)
    }
    /// Disable high speed write mode.
    #[inline]
    pub const fn disable_high_speed_write(self) -> Self {
        Self(self.0 & !Self::HSWM)
    }
    /// Check if high speed write mode is enabled.
    #[inline]
    pub const fn is_high_speed_write_enabled(self) -> bool {
        (self.0 & Self::HSWM) != 0
    }
    /// Enable dummy byte val (`DMY_VAL`).
    ///
    /// Not writable when `START` = 1.
    #[doc(alias = "DMY_VAL")]
    #[inline]
    pub const fn enable_dummy_byte_val(self) -> Self {
        Self(self.0 | Self::DMY_VAL)
    }
    /// Disable dummy byte val.
    #[inline]
    pub const fn disable_dummy_byte_val(self) -> Self {
        Self(self.0 & !Self::DMY_VAL)
    }
    /// Check if dummy byte val is enabled.
    #[inline]
    pub const fn is_dummy_byte_val_enabled(self) -> bool {
        (self.0 & Self::DMY_VAL) != 0
    }
    /// Enable discard invalid data (`DINVD`).
    ///
    /// Not writable when `START` = 1.
    #[doc(alias = "DINVD")]
    #[inline]
    pub const fn enable_discard_invalid_data(self) -> Self {
        Self(self.0 | Self::DINVD)
    }
    /// Disable discard invalid data.
    #[inline]
    pub const fn disable_discard_invalid_data(self) -> Self {
        Self(self.0 & !Self::DINVD)
    }
    /// Check if discard invalid data is enabled.
    #[inline]
    pub const fn is_discard_invalid_data_enabled(self) -> bool {
        (self.0 & Self::DINVD) != 0
    }
    /// Set cs level (`CS_LEVEL`).
    ///
    /// Not writable when `START` = 1.
    #[doc(alias = "CS_LEVEL")]
    #[inline]
    pub const fn set_cs_level(self, level: CsLevel) -> Self {
        Self((self.0 & !Self::CS_LEVEL) | (Self::CS_LEVEL & ((level as u32) << 7)))
    }
    /// Get cs level.
    #[inline]
    pub const fn cs_level(self) -> CsLevel {
        match (self.0 & Self::CS_LEVEL) >> 7 {
            0 => CsLevel::Low,
            1 => CsLevel::High,
            _ => unreachable!(),
        }
    }
    /// Set cs control mode (`CS_CTL_SEL`).
    ///
    /// Not writable when `START` = 1.
    #[doc(alias = "CS_CTL_SEL")]
    #[inline]
    pub const fn set_cs_ctrl_mode(self, mode: CsCtrlMode) -> Self {
        Self((self.0 & !Self::CS_CTL_SEL) | (Self::CS_CTL_SEL & ((mode as u32) << 6)))
    }
    /// Get cs control mode.
    #[inline]
    pub const fn cs_ctrl_mode(self) -> CsCtrlMode {
        match (self.0 & Self::CS_CTL_SEL) >> 6 {
            0 => CsCtrlMode::SpiController,
            1 => CsCtrlMode::Software,
            _ => unreachable!(),
        }
    }
    /// Set external device cs pin number (`CS_NUM`).
    ///
    /// Not writable when `START` = 1.
    #[doc(alias = "CS_NUM")]
    #[inline]
    pub const fn set_cs_pin_num(self, num: CsPin) -> Self {
        Self((self.0 & !Self::CS_NUM) | (Self::CS_NUM & ((num as u32) << 4)))
    }
    /// Get external device cs pin number.
    #[inline]
    pub const fn cs_pin_num(self) -> CsPin {
        match (self.0 & Self::CS_NUM) >> 4 {
            0 => CsPin::Cs0,
            1 => CsPin::Cs1,
            _ => unreachable!(),
        }
    }
    /// Set cs valid mode (`CS_VALID_CTL`).
    ///
    /// Not writable when `START` = 1;
    /// Valid only when cs control mode is `SpiController`.
    #[doc(alias = "CS_VALID_CTL")]
    #[inline]
    pub const fn set_cs_valid_mode(self, mode: CsValidMode) -> Self {
        Self((self.0 & !Self::CS_VALID_CTL) | (Self::CS_VALID_CTL & ((mode as u32) << 3)))
    }
    /// Get cs valid mode.
    #[inline]
    pub const fn cs_valid_mode(self) -> CsValidMode {
        match (self.0 & Self::CS_VALID_CTL) >> 3 {
            0 => CsValidMode::Continuous,
            1 => CsValidMode::InactiveWhenIdle,
            _ => unreachable!(),
        }
    }
    /// Set cs polarity (`CS_POL`).
    ///
    /// Not writable when `START` = 1.
    #[doc(alias = "CS_POL")]
    #[inline]
    pub const fn set_cs_pol(self, pol: CsPolarity) -> Self {
        Self((self.0 & !Self::CS_POL) | (Self::CS_POL & ((pol as u32) << 2)))
    }
    /// Get cs polarity.
    #[inline]
    pub const fn cs_pol(self) -> CsPolarity {
        match (self.0 & Self::CS_POL) >> 2 {
            0 => CsPolarity::Low,
            1 => CsPolarity::High,
            _ => unreachable!(),
        }
    }
    /// Set clock polarity (`CPOL`).
    ///
    /// Not writable when `START` = 1.
    #[doc(alias = "CPOL")]
    #[inline]
    pub const fn set_clk_pol(self, pol: ClockPolarity) -> Self {
        Self((self.0 & !Self::CPOL) | (Self::CPOL & ((pol as u32) << 1)))
    }
    /// Get clock polarity.
    #[inline]
    pub const fn clk_pol(self) -> ClockPolarity {
        match (self.0 & Self::CPOL) >> 1 {
            0 => ClockPolarity::Low,
            1 => ClockPolarity::High,
            _ => unreachable!(),
        }
    }
    /// Set clock phase (`CPHA`).
    ///
    /// Not writable when `START` = 1.
    #[doc(alias = "CPHA")]
    #[inline]
    pub const fn set_clk_phase(self, phase: ClockPhase) -> Self {
        Self((self.0 & !Self::CPHA) | (Self::CPHA & (phase as u32)))
    }
    /// Get clock phase.
    #[inline]
    pub const fn clk_phase(self) -> ClockPhase {
        match self.0 & Self::CPHA {
            0 => ClockPhase::FirstEdge,
            1 => ClockPhase::SecondEdge,
            _ => unreachable!(),
        }
    }
}

/// SPI interrupt control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IntControl(u32);

impl IntControl {
    const TXIDMA_DONE_INTE: u32 = 0x1 << 23;
    const RXIDMA_DONE_INTE: u32 = 0x1 << 22;
    const RXFIFO_UNALIGN_INTE: u32 = 0x1 << 21;
    const IDMA_ERROR_INTE: u32 = 0x1 << 20;
    const AXI_TRAN_DONE_INTE: u32 = 0x1 << 19;
    const AXI_CFG_ERROR_INTE: u32 = 0x1 << 18;
    const AXI_TRAN_ERROR_INTE: u32 = 0x1 << 17;
    const AHB_TRAN_ERROR_INTE: u32 = 0x1 << 16;
    const CS_INTE: u32 = 0x1 << 13;
    const TD_INTE: u32 = 0x1 << 12;
    const TF_UDR_INTE: u32 = 0x1 << 11;
    const TF_OVF_INTE: u32 = 0x1 << 10;
    const RF_UDR_INTE: u32 = 0x1 << 9;
    const RF_OVF_INTE: u32 = 0x1 << 8;
    const TF_FUL_INTE: u32 = 0x1 << 6;
    const TF_EMP_INTE: u32 = 0x1 << 5;
    const TF_RDY_INTE: u32 = 0x1 << 4;
    const RF_FUL_INTE: u32 = 0x1 << 2;
    const RF_EMP_INTE: u32 = 0x1 << 1;
    const RF_RDY_INTE: u32 = 0x1;

    /// Enable internal dma tx done interrupt (`TXIDMA_DONE_INTE`).
    #[doc(alias = "TXIDMA_DONE_INTE")]
    #[inline]
    pub const fn enable_tx_dma_done_int(self) -> Self {
        Self(self.0 | Self::TXIDMA_DONE_INTE)
    }
    /// Disable internal dma tx done interrupt.
    #[inline]
    pub const fn disable_tx_dma_done_int(self) -> Self {
        Self(self.0 & !Self::TXIDMA_DONE_INTE)
    }
    /// Check if internal dma tx done interrupt is enabled.
    #[inline]
    pub const fn is_tx_dma_done_int_enabled(self) -> bool {
        (self.0 & Self::TXIDMA_DONE_INTE) != 0
    }
    /// Enable internal dma rx done interrupt (`RXIDMA_DONE_INTE`).
    #[doc(alias = "RXIDMA_DONE_INTE")]
    #[inline]
    pub const fn enable_rx_dma_done_int(self) -> Self {
        Self(self.0 | Self::RXIDMA_DONE_INTE)
    }
    /// Disable internal dma rx done interrupt.
    #[inline]
    pub const fn disable_rx_dma_done_int(self) -> Self {
        Self(self.0 & !Self::RXIDMA_DONE_INTE)
    }
    /// Check if internal dma rx done interrupt is enabled.
    #[inline]
    pub const fn is_rx_dma_done_int_enabled(self) -> bool {
        (self.0 & Self::RXIDMA_DONE_INTE) != 0
    }
    /// Enable rx fifo unalign interrupt (`RXFIFO_UNALIGN_INTE`).
    #[doc(alias = "RXFIFO_UNALIGN_INTE")]
    #[inline]
    pub const fn enable_rx_fifo_unalign_int(self) -> Self {
        Self(self.0 | Self::RXFIFO_UNALIGN_INTE)
    }
    /// Disable rx fifo unalign interrupt.
    #[inline]
    pub const fn disable_rx_fifo_unalign_int(self) -> Self {
        Self(self.0 & !Self::RXFIFO_UNALIGN_INTE)
    }
    /// Check if rx fifo unalign interrupt is enabled.
    #[inline]
    pub const fn is_rx_fifo_unalign_int_enabled(self) -> bool {
        (self.0 & Self::RXFIFO_UNALIGN_INTE) != 0
    }
    /// Enable internal dma error interrupt (`IDMA_ERROR_INTE`).
    #[doc(alias = "IDMA_ERROR_INTE")]
    #[inline]
    pub const fn enable_idma_error_int(self) -> Self {
        Self(self.0 | Self::IDMA_ERROR_INTE)
    }
    /// Disable internal dma error interrupt.
    #[inline]
    pub const fn disable_idma_error_int(self) -> Self {
        Self(self.0 & !Self::IDMA_ERROR_INTE)
    }
    /// Check if internal dma error interrupt is enabled.
    #[inline]
    pub const fn is_idma_error_int_enabled(self) -> bool {
        (self.0 & Self::IDMA_ERROR_INTE) != 0
    }
    /// Enable axi transfer done interrupt (`AXI_TRAN_DONE_INTE`).
    #[doc(alias = "AXI_TRAN_DONE_INTE")]
    #[inline]
    pub const fn enable_axi_tran_done_int(self) -> Self {
        Self(self.0 | Self::AXI_TRAN_DONE_INTE)
    }
    /// Disable axi transfer done interrupt.
    #[inline]
    pub const fn disable_axi_tran_done_int(self) -> Self {
        Self(self.0 & !Self::AXI_TRAN_DONE_INTE)
    }
    /// Check if axi transfer done interrupt is enabled.
    #[inline]
    pub const fn is_axi_tran_done_int_enabled(self) -> bool {
        (self.0 & Self::AXI_TRAN_DONE_INTE) != 0
    }
    /// Enable axi config error interrupt (`AXI_CFG_ERROR_INTE`).
    #[doc(alias = "AXI_CFG_ERROR_INTE")]
    #[inline]
    pub const fn enable_axi_cfg_error_int(self) -> Self {
        Self(self.0 | Self::AXI_CFG_ERROR_INTE)
    }
    /// Disable axi config error interrupt.
    #[inline]
    pub const fn disable_axi_cfg_error_int(self) -> Self {
        Self(self.0 & !Self::AXI_CFG_ERROR_INTE)
    }
    /// Check if axi config error interrupt is enabled.
    #[inline]
    pub const fn is_axi_cfg_error_int_enabled(self) -> bool {
        (self.0 & Self::AXI_CFG_ERROR_INTE) != 0
    }
    /// Enable axi transfer error interrupt (`AXI_TRAN_ERROR_INTE`).
    #[doc(alias = "AXI_TRAN_ERROR_INTE")]
    #[inline]
    pub const fn enable_axi_tran_error_int(self) -> Self {
        Self(self.0 | Self::AXI_TRAN_ERROR_INTE)
    }
    /// Disable axi transfer error interrupt.
    #[inline]
    pub const fn disable_axi_tran_error_int(self) -> Self {
        Self(self.0 & !Self::AXI_TRAN_ERROR_INTE)
    }
    /// Check if axi transfer error interrupt is enabled.
    #[inline]
    pub const fn is_axi_tran_error_int_enabled(self) -> bool {
        (self.0 & Self::AXI_TRAN_ERROR_INTE) != 0
    }
    /// Enable ahb transfer error interrupt (`AHB_TRAN_ERROR_INTE`).
    #[doc(alias = "AHB_TRAN_ERROR_INTE")]
    #[inline]
    pub const fn enable_ahb_tran_error_int(self) -> Self {
        Self(self.0 | Self::AHB_TRAN_ERROR_INTE)
    }
    /// Disable ahb transfer error interrupt.
    #[inline]
    pub const fn disable_ahb_tran_error_int(self) -> Self {
        Self(self.0 & !Self::AHB_TRAN_ERROR_INTE)
    }
    /// Check if ahb transfer error interrupt is enabled.
    #[inline]
    pub const fn is_ahb_tran_error_int_enabled(self) -> bool {
        (self.0 & Self::AHB_TRAN_ERROR_INTE) != 0
    }
    /// Enable cs invalid interrupt (`CS_INTE`).
    #[doc(alias = "CS_INTE")]
    #[inline]
    pub const fn enable_cs_invalid_int(self) -> Self {
        Self(self.0 | Self::CS_INTE)
    }
    /// Disable cs invalid interrupt.
    #[inline]
    pub const fn disable_cs_invalid_int(self) -> Self {
        Self(self.0 & !Self::CS_INTE)
    }
    /// Check if cs invalid interrupt is enabled.
    #[inline]
    pub const fn is_cs_invalid_int_enabled(self) -> bool {
        (self.0 & Self::CS_INTE) != 0
    }
    /// Enable transfer done interrupt (`TD_INTE`).
    #[doc(alias = "TD_INTE")]
    #[inline]
    pub const fn enable_transfer_done_int(self) -> Self {
        Self(self.0 | Self::TD_INTE)
    }
    /// Disable transfer done interrupt.
    #[inline]
    pub const fn disable_transfer_done_int(self) -> Self {
        Self(self.0 & !Self::TD_INTE)
    }
    /// Check if transfer done interrupt is enabled.
    #[inline]
    pub const fn is_transfer_done_int_enabled(self) -> bool {
        (self.0 & Self::TD_INTE) != 0
    }
    /// Enable tx fifo underrun interrupt (`TF_UDR_INTE`).
    #[doc(alias = "TF_UDR_INTE")]
    #[inline]
    pub const fn enable_tx_fifo_underrun_int(self) -> Self {
        Self(self.0 | Self::TF_UDR_INTE)
    }
    /// Disable tx fifo underrun interrupt.
    #[inline]
    pub const fn disable_tx_fifo_underrun_int(self) -> Self {
        Self(self.0 & !Self::TF_UDR_INTE)
    }
    /// Check if tx fifo underrun interrupt is enabled.
    #[inline]
    pub const fn is_tx_fifo_underrun_int_enabled(self) -> bool {
        (self.0 & Self::TF_UDR_INTE) != 0
    }
    /// Enable tx fifo overflow interrupt (`TF_OVF_INTE`).
    #[doc(alias = "TF_OVF_INTE")]
    #[inline]
    pub const fn enable_tx_fifo_overflow_int(self) -> Self {
        Self(self.0 | Self::TF_OVF_INTE)
    }
    /// Disable tx fifo overflow interrupt.
    #[inline]
    pub const fn disable_tx_fifo_overflow_int(self) -> Self {
        Self(self.0 & !Self::TF_OVF_INTE)
    }
    /// Check if tx fifo overflow interrupt is enabled.
    #[inline]
    pub const fn is_tx_fifo_overflow_int_enabled(self) -> bool {
        (self.0 & Self::TF_OVF_INTE) != 0
    }
    /// Enable rx fifo underrun interrupt (`RF_UDR_INTE`).
    #[doc(alias = "RF_UDR_INTE")]
    #[inline]
    pub const fn enable_rx_fifo_underrun_int(self) -> Self {
        Self(self.0 | Self::RF_UDR_INTE)
    }
    /// Disable rx fifo underrun interrupt.
    #[inline]
    pub const fn disable_rx_fifo_underrun_int(self) -> Self {
        Self(self.0 & !Self::RF_UDR_INTE)
    }
    /// Check if rx fifo underrun interrupt is enabled.
    #[inline]
    pub const fn is_rx_fifo_underrun_int_enabled(self) -> bool {
        (self.0 & Self::RF_UDR_INTE) != 0
    }
    /// Enable rx fifo overflow interrupt (`RF_OVF_INTE`).
    #[doc(alias = "RF_OVF_INTE")]
    #[inline]
    pub const fn enable_rx_fifo_overflow_int(self) -> Self {
        Self(self.0 | Self::RF_OVF_INTE)
    }
    /// Disable rx fifo overflow interrupt.
    #[inline]
    pub const fn disable_rx_fifo_overflow_int(self) -> Self {
        Self(self.0 & !Self::RF_OVF_INTE)
    }
    /// Check if rx fifo overflow interrupt is enabled.
    #[inline]
    pub const fn is_rx_fifo_overflow_int_enabled(self) -> bool {
        (self.0 & Self::RF_OVF_INTE) != 0
    }
    /// Enable tx fifo full interrupt (`TF_FUL_INTE`).
    #[doc(alias = "TF_FUL_INTE")]
    #[inline]
    pub const fn enable_tx_fifo_full_int(self) -> Self {
        Self(self.0 | Self::TF_FUL_INTE)
    }
    /// Disable tx fifo full interrupt.
    #[inline]
    pub const fn disable_tx_fifo_full_int(self) -> Self {
        Self(self.0 & !Self::TF_FUL_INTE)
    }
    /// Check if tx fifo full interrupt is enabled.
    #[inline]
    pub const fn is_tx_fifo_full_int_enabled(self) -> bool {
        (self.0 & Self::TF_FUL_INTE) != 0
    }
    /// Enable tx fifo empty interrupt (`TF_EMP_INTE`).
    #[doc(alias = "TF_EMP_INTE")]
    #[inline]
    pub const fn enable_tx_fifo_empty_int(self) -> Self {
        Self(self.0 | Self::TF_EMP_INTE)
    }
    /// Disable tx fifo empty interrupt.
    #[inline]
    pub const fn disable_tx_fifo_empty_int(self) -> Self {
        Self(self.0 & !Self::TF_EMP_INTE)
    }
    /// Check if tx fifo empty interrupt is enabled.
    #[inline]
    pub const fn is_tx_fifo_empty_int_enabled(self) -> bool {
        (self.0 & Self::TF_EMP_INTE) != 0
    }
    /// Enable tx fifo ready interrupt (`TF_RDY_INTE`).
    #[doc(alias = "TF_RDY_INTE")]
    #[inline]
    pub const fn enable_tx_fifo_ready_int(self) -> Self {
        Self(self.0 | Self::TF_RDY_INTE)
    }
    /// Disable tx fifo ready interrupt.
    #[inline]
    pub const fn disable_tx_fifo_ready_int(self) -> Self {
        Self(self.0 & !Self::TF_RDY_INTE)
    }
    /// Check if tx fifo ready interrupt is enabled.
    #[inline]
    pub const fn is_tx_fifo_ready_int_enabled(self) -> bool {
        (self.0 & Self::TF_RDY_INTE) != 0
    }
    /// Enable rx fifo full interrupt (`RF_FUL_INTE`).
    #[doc(alias = "RF_FUL_INTE")]
    #[inline]
    pub const fn enable_rx_fifo_full_int(self) -> Self {
        Self(self.0 | Self::RF_FUL_INTE)
    }
    /// Disable rx fifo full interrupt.
    #[inline]
    pub const fn disable_rx_fifo_full_int(self) -> Self {
        Self(self.0 & !Self::RF_FUL_INTE)
    }
    /// Check if rx fifo full interrupt is enabled.
    #[inline]
    pub const fn is_rx_fifo_full_int_enabled(self) -> bool {
        (self.0 & Self::RF_FUL_INTE) != 0
    }
    /// Enable rx fifo empty interrupt (`RF_EMP_INTE`).
    #[doc(alias = "RF_EMP_INTE")]
    #[inline]
    pub const fn enable_rx_fifo_empty_int(self) -> Self {
        Self(self.0 | Self::RF_EMP_INTE)
    }
    /// Disable rx fifo empty interrupt.
    #[inline]
    pub const fn disable_rx_fifo_empty_int(self) -> Self {
        Self(self.0 & !Self::RF_EMP_INTE)
    }
    /// Check if rx fifo empty interrupt is enabled.
    #[inline]
    pub const fn is_rx_fifo_empty_int_enabled(self) -> bool {
        (self.0 & Self::RF_EMP_INTE) != 0
    }
    /// Enable rx fifo ready interrupt (`RF_RDY_INTE`).
    #[doc(alias = "RF_RDY_INTE")]
    #[inline]
    pub const fn enable_rx_fifo_ready_int(self) -> Self {
        Self(self.0 | Self::RF_RDY_INTE)
    }
    /// Disable rx fifo ready interrupt.
    #[inline]
    pub const fn disable_rx_fifo_ready_int(self) -> Self {
        Self(self.0 & !Self::RF_RDY_INTE)
    }
    /// Check if rx fifo ready interrupt is enabled.
    #[inline]
    pub const fn is_rx_fifo_ready_int_enabled(self) -> bool {
        (self.0 & Self::RF_RDY_INTE) != 0
    }
}

/// SPI interrupt status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IntStatus(u32);

impl IntStatus {
    const TXIDMA_DONE: u32 = 0x1 << 23;
    const RXIDMA_DONE: u32 = 0x1 << 22;
    const RXFIFO_UNALIGN: u32 = 0x1 << 21;
    const IDMA_ERROR: u32 = 0x1 << 20;
    const AXI_TRAN_DONE: u32 = 0x1 << 19;
    const AXI_CFG_ERROR: u32 = 0x1 << 18;
    const AXI_TRAN_ERROR: u32 = 0x1 << 17;
    const AHB_TRAN_ERROR: u32 = 0x1 << 16;
    const CS_INV: u32 = 0x1 << 13;
    const TD: u32 = 0x1 << 12;
    const TF_UDF: u32 = 0x1 << 11;
    const TF_OVF: u32 = 0x1 << 10;
    const RF_UDF: u32 = 0x1 << 9;
    const RF_OVF: u32 = 0x1 << 8;
    const TF_FULL: u32 = 0x1 << 6;
    const TF_EMP: u32 = 0x1 << 5;
    const TF_READY: u32 = 0x1 << 4;
    const RF_FULL: u32 = 0x1 << 2;
    const RF_EMP: u32 = 0x1 << 1;
    const RF_READY: u32 = 0x1;

    /// Check if internal dma tx done interrupt is pending (`TXIDMA_DONE`).
    #[doc(alias = "TXIDMA_DONE")]
    #[inline]
    pub const fn is_tx_dma_done_int_pending(self) -> bool {
        (self.0 & Self::TXIDMA_DONE) != 0
    }
    /// Clear internal dma tx done interrupt.
    #[inline]
    pub const fn clear_tx_dma_done_int(self) -> Self {
        Self(self.0 | Self::TXIDMA_DONE)
    }
    /// Check if internal dma rx done interrupt is pending (`RXIDMA_DONE`).
    #[doc(alias = "RXIDMA_DONE")]
    #[inline]
    pub const fn is_rx_dma_done_int_pending(self) -> bool {
        (self.0 & Self::RXIDMA_DONE) != 0
    }
    /// Clear internal dma rx done interrupt.
    #[inline]
    pub const fn clear_rx_dma_done_int(self) -> Self {
        Self(self.0 | Self::RXIDMA_DONE)
    }
    /// Check if rx fifo unalign interrupt is pending (`RXFIFO_UNALIGN`).
    #[doc(alias = "RXFIFO_UNALIGN")]
    #[inline]
    pub const fn is_rx_fifo_unalign_int_pending(self) -> bool {
        (self.0 & Self::RXFIFO_UNALIGN) != 0
    }
    /// Clear rx fifo unalign interrupt.
    #[inline]
    pub const fn clear_rx_fifo_unalign_int(self) -> Self {
        Self(self.0 | Self::RXFIFO_UNALIGN)
    }
    /// Check if internal dma error interrupt is pending (`IDMA_ERROR`).
    #[doc(alias = "IDMA_ERROR")]
    #[inline]
    pub const fn is_idma_error_int_pending(self) -> bool {
        (self.0 & Self::IDMA_ERROR) != 0
    }
    /// Clear internal dma error interrupt.
    #[inline]
    pub const fn clear_idma_error_int(self) -> Self {
        Self(self.0 | Self::IDMA_ERROR)
    }
    /// Check if axi transfer done interrupt is pending (`AXI_TRAN_DONE`).
    #[doc(alias = "AXI_TRAN_DONE")]
    #[inline]
    pub const fn is_axi_tran_done_int_pending(self) -> bool {
        (self.0 & Self::AXI_TRAN_DONE) != 0
    }
    /// Clear axi transfer done interrupt.
    #[inline]
    pub const fn clear_axi_tran_done_int(self) -> Self {
        Self(self.0 | Self::AXI_TRAN_DONE)
    }
    /// Check if axi config error interrupt is pending (`AXI_CFG_ERROR`).
    #[doc(alias = "AXI_CFG_ERROR")]
    #[inline]
    pub const fn is_axi_cfg_error_int_pending(self) -> bool {
        (self.0 & Self::AXI_CFG_ERROR) != 0
    }
    /// Clear axi config error interrupt.
    #[inline]
    pub const fn clear_axi_cfg_error_int(self) -> Self {
        Self(self.0 | Self::AXI_CFG_ERROR)
    }
    /// Check if axi transfer error interrupt is pending (`AXI_TRAN_ERROR`).
    #[doc(alias = "AXI_TRAN_ERROR")]
    #[inline]
    pub const fn is_axi_tran_error_int_pending(self) -> bool {
        (self.0 & Self::AXI_TRAN_ERROR) != 0
    }
    /// Clear axi transfer error interrupt.
    #[inline]
    pub const fn clear_axi_tran_error_int(self) -> Self {
        Self(self.0 | Self::AXI_TRAN_ERROR)
    }
    /// Check if ahb transfer error interrupt is pending (`AHB_TRAN_ERROR`).
    #[doc(alias = "AHB_TRAN_ERROR")]
    #[inline]
    pub const fn is_ahb_tran_error_int_pending(self) -> bool {
        (self.0 & Self::AHB_TRAN_ERROR) != 0
    }
    /// Clear ahb transfer error interrupt.
    #[inline]
    pub const fn clear_ahb_tran_error_int(self) -> Self {
        Self(self.0 | Self::AHB_TRAN_ERROR)
    }
    /// Check if cs invalid interrupt is pending (`CS_INV`).
    #[doc(alias = "CS_INV")]
    #[inline]
    pub const fn is_cs_invalid_int_pending(self) -> bool {
        (self.0 & Self::CS_INV) != 0
    }
    /// Clear cs invalid interrupt.
    #[inline]
    pub const fn clear_cs_invalid_int(self) -> Self {
        Self(self.0 | Self::CS_INV)
    }
    /// Check if transfer done interrupt is pending (`TD`).
    #[doc(alias = "TD")]
    #[inline]
    pub const fn is_transfer_done_int_pending(self) -> bool {
        (self.0 & Self::TD) != 0
    }
    /// Clear transfer done interrupt.
    #[inline]
    pub const fn clear_transfer_done_int(self) -> Self {
        Self(self.0 | Self::TD)
    }
    /// Check if tx fifo underrun interrupt is pending (`TF_UDF`).
    #[doc(alias = "TF_UDF")]
    #[inline]
    pub const fn is_tx_fifo_underrun_int_pending(self) -> bool {
        (self.0 & Self::TF_UDF) != 0
    }
    /// Clear tx fifo underrun interrupt.
    #[inline]
    pub const fn clear_tx_fifo_underrun_int(self) -> Self {
        Self(self.0 | Self::TF_UDF)
    }
    /// Check if tx fifo overflow interrupt is pending (`TF_OVF`).
    #[doc(alias = "TF_OVF")]
    #[inline]
    pub const fn is_tx_fifo_overflow_int_pending(self) -> bool {
        (self.0 & Self::TF_OVF) != 0
    }
    /// Clear tx fifo overflow interrupt.
    #[inline]
    pub const fn clear_tx_fifo_overflow_int(self) -> Self {
        Self(self.0 | Self::TF_OVF)
    }
    /// Check if rx fifo underrun interrupt is pending (`RF_UDF`).
    #[doc(alias = "RF_UDF")]
    #[inline]
    pub const fn is_rx_fifo_underrun_int_pending(self) -> bool {
        (self.0 & Self::RF_UDF) != 0
    }
    /// Clear rx fifo underrun interrupt.
    #[inline]
    pub const fn clear_rx_fifo_underrun_int(self) -> Self {
        Self(self.0 | Self::RF_UDF)
    }
    /// Check if rx fifo overflow interrupt is pending (`RF_OVF`).
    #[doc(alias = "RF_OVF")]
    #[inline]
    pub const fn is_rx_fifo_overflow_int_pending(self) -> bool {
        (self.0 & Self::RF_OVF) != 0
    }
    /// Clear rx fifo overflow interrupt.
    #[inline]
    pub const fn clear_rx_fifo_overflow_int(self) -> Self {
        Self(self.0 | Self::RF_OVF)
    }
    /// Check if tx fifo full interrupt is pending (`TF_FULL`).
    #[doc(alias = "TF_FULL")]
    #[inline]
    pub const fn is_tx_fifo_full_int_pending(self) -> bool {
        (self.0 & Self::TF_FULL) != 0
    }
    /// Clear tx fifo full interrupt.
    #[inline]
    pub const fn clear_tx_fifo_full_int(self) -> Self {
        Self(self.0 | Self::TF_FULL)
    }
    /// Check if tx fifo empty interrupt is pending (`TF_EMP`).
    #[doc(alias = "TF_EMP")]
    #[inline]
    pub const fn is_tx_fifo_empty_int_pending(self) -> bool {
        (self.0 & Self::TF_EMP) != 0
    }
    /// Clear tx fifo empty interrupt.
    #[inline]
    pub const fn clear_tx_fifo_empty_int(self) -> Self {
        Self(self.0 | Self::TF_EMP)
    }
    /// Check if tx fifo ready interrupt is pending (`TF_READY`).
    #[doc(alias = "TF_READY")]
    #[inline]
    pub const fn is_tx_fifo_ready_int_pending(self) -> bool {
        (self.0 & Self::TF_READY) != 0
    }
    /// Clear tx fifo ready interrupt.
    #[inline]
    pub const fn clear_tx_fifo_ready_int(self) -> Self {
        Self(self.0 | Self::TF_READY)
    }
    /// Check if rx fifo full interrupt is pending (`RF_FULL`).
    #[doc(alias = "RF_FULL")]
    #[inline]
    pub const fn is_rx_fifo_full_int_pending(self) -> bool {
        (self.0 & Self::RF_FULL) != 0
    }
    /// Clear rx fifo full interrupt.
    #[inline]
    pub const fn clear_rx_fifo_full_int(self) -> Self {
        Self(self.0 | Self::RF_FULL)
    }
    /// Check if rx fifo empty interrupt is pending (`RF_EMP`).
    #[doc(alias = "RF_EMP")]
    #[inline]
    pub const fn is_rx_fifo_empty_int_pending(self) -> bool {
        (self.0 & Self::RF_EMP) != 0
    }
    /// Clear rx fifo empty interrupt.
    #[inline]
    pub const fn clear_rx_fifo_empty_int(self) -> Self {
        Self(self.0 | Self::RF_EMP)
    }
    /// Check if rx fifo ready interrupt is pending (`RF_READY`).
    #[doc(alias = "RF_READY")]
    #[inline]
    pub const fn is_rx_fifo_ready_int_pending(self) -> bool {
        (self.0 & Self::RF_READY) != 0
    }
    /// Clear rx fifo ready interrupt.
    #[inline]
    pub const fn clear_rx_fifo_ready_int(self) -> Self {
        Self(self.0 | Self::RF_READY)
    }
}

/// SPI fifo control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FifoControl(u32);

impl FifoControl {
    const TF_RST: u32 = 0x1 << 31;
    const TF_DREQ_EN: u32 = 0x1 << 24;
    const TF_WATER_MARK: u32 = 0xFF << 16;
    const RF_RST: u32 = 0x1 << 15;
    const RF_DREQ_EN: u32 = 0x1 << 8;
    const RF_WATER_MARK: u32 = 0xFF;

    /// Get the tx fifo reset bit (`TF_RST`).
    #[doc(alias = "TF_RST")]
    #[inline]
    pub const fn tx_fifo_reset(self) -> bool {
        (self.0 & Self::TF_RST) != 0
    }
    /// Reset tx fifo.
    #[inline]
    pub const fn reset_tx_fifo(self) -> Self {
        Self(self.0 | Self::TF_RST)
    }
    /// Enable tx fifo DMA request (`TF_DREQ_EN`).
    #[doc(alias = "TF_DREQ_EN")]
    #[inline]
    pub const fn enable_tx_fifo_dma_req(self) -> Self {
        Self(self.0 | Self::TF_DREQ_EN)
    }
    /// Disable tx fifo DMA request.
    #[inline]
    pub const fn disable_tx_fifo_dma_req(self) -> Self {
        Self(self.0 & !Self::TF_DREQ_EN)
    }
    /// Check if tx fifo DMA request is enabled.
    #[inline]
    pub const fn is_tx_fifo_dma_req_enabled(self) -> bool {
        (self.0 & Self::TF_DREQ_EN) != 0
    }
    /// Set tx fifo water mark (`TF_WATER_MARK`).
    #[doc(alias = "TF_WATER_MARK")]
    #[inline]
    pub const fn set_tx_fifo_water_mark(self, mark: u8) -> Self {
        Self((self.0 & !Self::TF_WATER_MARK) | (Self::TF_WATER_MARK & ((mark as u32) << 16)))
    }
    /// Get tx fifo water mark.
    #[inline]
    pub const fn tx_fifo_water_mark(self) -> u8 {
        ((self.0 & Self::TF_WATER_MARK) >> 16) as u8
    }
    /// Get the rx fifo reset bit (`RF_RST`).
    #[doc(alias = "RF_RST")]
    #[inline]
    pub const fn rx_fifo_reset(self) -> bool {
        (self.0 & Self::RF_RST) != 0
    }
    /// Reset rx fifo.
    #[inline]
    pub const fn reset_rx_fifo(self) -> Self {
        Self(self.0 | Self::RF_RST)
    }
    /// Enable rx fifo DMA request (`RF_DREQ_EN`).
    #[doc(alias = "RF_DREQ_EN")]
    #[inline]
    pub const fn enable_rx_fifo_dma_req(self) -> Self {
        Self(self.0 | Self::RF_DREQ_EN)
    }
    /// Disable rx fifo DMA request.
    #[inline]
    pub const fn disable_rx_fifo_dma_req(self) -> Self {
        Self(self.0 & !Self::RF_DREQ_EN)
    }
    /// Check if rx fifo DMA request is enabled.
    #[inline]
    pub const fn is_rx_fifo_dma_req_enabled(self) -> bool {
        (self.0 & Self::RF_DREQ_EN) != 0
    }
    /// Set rx fifo water mark (`RF_WATER_MARK`).
    #[doc(alias = "RF_WATER_MARK")]
    #[inline]
    pub const fn set_rx_fifo_water_mark(self, mark: u8) -> Self {
        Self((self.0 & !Self::RF_WATER_MARK) | (Self::RF_WATER_MARK & (mark as u32)))
    }
    /// Get rx fifo water mark.
    #[inline]
    pub const fn rx_fifo_water_mark(self) -> u8 {
        (self.0 & Self::RF_WATER_MARK) as u8
    }
}

/// SPI fifo status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FifoStatus(u32);

impl FifoStatus {
    const TF_WBUF_STS: u32 = 0x1 << 31;
    const TF_WBUF_CNT: u32 = 0x7 << 28;
    const TF_CNT: u32 = 0xFF << 16;
    const RF_RBUF_STS: u32 = 0x1 << 15;
    const RF_RBUF_CNT: u32 = 0x7 << 12;
    const RF_CNT: u32 = 0xFF;

    /// Get tx fifo write buffer status (`TF_WBUF_STS`).
    #[doc(alias = "TF_WBUF_STS")]
    #[inline]
    pub const fn tx_fifo_write_buffer_status(self) -> bool {
        (self.0 & Self::TF_WBUF_STS) != 0
    }
    /// Get tx fifo write buffer count (`TF_WBUF_CNT`).
    #[doc(alias = "TF_WBUF_CNT")]
    #[inline]
    pub const fn tx_fifo_write_buffer_count(self) -> u8 {
        ((self.0 & Self::TF_WBUF_CNT) >> 28) as u8
    }
    /// Get tx fifo count (`TF_CNT`).
    #[doc(alias = "TF_CNT")]
    #[inline]
    pub const fn tx_fifo_count(self) -> u8 {
        ((self.0 & Self::TF_CNT) >> 16) as u8
    }
    /// Get rx fifo read buffer status (`RF_RBUF_STS`).
    #[doc(alias = "RF_RBUF_STS")]
    #[inline]
    pub const fn rx_fifo_read_buffer_status(self) -> bool {
        (self.0 & Self::RF_RBUF_STS) != 0
    }
    /// Get rx fifo read buffer count (`RF_RBUF_CNT`).
    #[doc(alias = "RF_RBUF_CNT")]
    #[inline]
    pub const fn rx_fifo_read_buffer_count(self) -> u8 {
        ((self.0 & Self::RF_RBUF_CNT) >> 12) as u8
    }
    /// Get rx fifo count (`RF_CNT`).
    #[doc(alias = "RF_CNT")]
    #[inline]
    pub const fn rx_fifo_count(self) -> u8 {
        (self.0 & Self::RF_CNT) as u8
    }
}

/// SPI clock divider selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClockDivSel {
    /// Use `CKDIV1` to configure the SPI clock.
    Div1,
    /// Use `CKDIV2` to configure the SPI clock.
    Div2,
}

/// SPI clock configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ClkConfig(u32);

impl ClkConfig {
    const CKDIV_SEL: u32 = 0x1 << 12;
    const CKDIV1: u32 = 0xF << 8;
    const CKDIV2: u32 = 0xFF;

    /// Set clock divider selection (`CKDIV_SEL`).
    #[doc(alias = "CKDIV_SEL")]
    #[inline]
    pub const fn set_clock_divider_selection(self, sel: ClockDivSel) -> Self {
        Self((self.0 & !Self::CKDIV_SEL) | (Self::CKDIV_SEL & ((sel as u32) << 12)))
    }
    /// Get clock divider selection.
    #[inline]
    pub const fn clock_divider_selection(self) -> ClockDivSel {
        match (self.0 & Self::CKDIV_SEL) >> 12 {
            0 => ClockDivSel::Div1,
            1 => ClockDivSel::Div2,
            _ => unreachable!(),
        }
    }
    /// Set clock divider 1 (`CKDIV1`).
    ///
    /// SPI_CLK = SRC_CLK/(2^CKDIV1).
    #[doc(alias = "CKDIV1")]
    #[inline]
    pub const fn set_clk_div_1(self, div: u8) -> Self {
        assert!(div <= 0xF, "CKDIV1 out of range (expected 0..=15)");
        Self((self.0 & !Self::CKDIV1) | (Self::CKDIV1 & ((div as u32) << 8)))
    }
    /// Get clock divider 1.
    #[inline]
    pub const fn clk_div_1(self) -> u8 {
        ((self.0 & Self::CKDIV1) >> 8) as u8
    }
    /// Set clock divider 2 (`CKDIV2`).
    ///
    /// SPI_CLK = SRC_CLK/(2*(CKDIV2 + 1)).
    #[doc(alias = "CKDIV2")]
    #[inline]
    pub const fn set_clk_div_2(self, div: u8) -> Self {
        Self((self.0 & !Self::CKDIV2) | (Self::CKDIV2 & (div as u32)))
    }
    /// Get clock divider 2.
    #[inline]
    pub const fn clk_div_2(self) -> u8 {
        (self.0 & Self::CKDIV2) as u8
    }
}

/// SPI total bytes counter register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TotalBytesCnt(u32);

impl TotalBytesCnt {
    const TOTAL_BYTES: u32 = 0xFFFFFF;

    /// Set total bytes to transfer (`TOTAL_BYTES`).
    #[doc(alias = "TOTAL_BYTES")]
    #[inline]
    pub const fn set_total_bytes(self, bytes: u32) -> Self {
        assert!(
            bytes <= 0xFFFFFF,
            "Total bytes out of range (expected 0..=0xFFFFFF)"
        );
        Self((self.0 & !Self::TOTAL_BYTES) | (Self::TOTAL_BYTES & bytes))
    }
    /// Get total bytes to transfer.
    #[inline]
    pub const fn total_bytes(self) -> u32 {
        self.0 & Self::TOTAL_BYTES
    }
}

/// SPI transmit write counter register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TransCnt(u32);

impl TransCnt {
    const TXD_CNT: u32 = 0xFFFFFF;

    /// Set transmit write counter (`TXD_CNT`).
    #[doc(alias = "TXD_CNT")]
    #[inline]
    pub const fn set_tx_cnt(self, cnt: u32) -> Self {
        assert!(
            cnt <= 0xFFFFFF,
            "Transmit write counter out of range (expected 0..=0xFFFFFF)"
        );
        Self((self.0 & !Self::TXD_CNT) | (Self::TXD_CNT & cnt))
    }
    /// Get transmit write counter.
    #[inline]
    pub const fn tx_cnt(self) -> u32 {
        self.0 & Self::TXD_CNT
    }
}

/// SPI transmit miscellaneous control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TransMiscControl(u32);

impl TransMiscControl {
    const QPI_EN: u32 = 0x1 << 31;
    const QADDR_EN: u32 = 0x1 << 30;
    const QUAD_EN: u32 = 0x1 << 29;
    const DUAL_EN: u32 = 0x1 << 28;
    const DMY_CNT: u32 = 0xF << 24;
    const STXD_CNT: u32 = 0xFFFFFF;

    /// Enable QPI transfer mode (`QPI_EN`).
    ///
    /// Valid only in quad transfer mode.
    #[doc(alias = "QPI_EN")]
    #[inline]
    pub const fn enable_qpi(self) -> Self {
        Self(self.0 | Self::QPI_EN)
    }
    /// Disable QPI transfer mode.
    #[inline]
    pub const fn disable_qpi(self) -> Self {
        Self(self.0 & !Self::QPI_EN)
    }
    /// Check if QPI transfer mode is enabled.
    #[inline]
    pub const fn is_qpi_enabled(self) -> bool {
        self.0 & Self::QPI_EN != 0
    }
    /// Enable QIO address transfer mode (`QADDR_EN`).
    ///
    /// Valid only in quad transfer mode.
    #[doc(alias = "QADDR_EN")]
    #[inline]
    pub const fn enable_qaddr(self) -> Self {
        Self(self.0 | Self::QADDR_EN)
    }
    /// Disable QIO address transfer mode.
    #[inline]
    pub const fn disable_qaddr(self) -> Self {
        Self(self.0 & !Self::QADDR_EN)
    }
    /// Check if QIO address transfer mode is enabled.
    #[inline]
    pub const fn is_qaddr_enabled(self) -> bool {
        self.0 & Self::QADDR_EN != 0
    }
    /// Enable quad data transfer mode (`QUAD_EN`).
    #[doc(alias = "QUAD_EN")]
    #[inline]
    pub const fn enable_quad(self) -> Self {
        Self(self.0 | Self::QUAD_EN)
    }
    /// Disable quad data transfer mode.
    #[inline]
    pub const fn disable_quad(self) -> Self {
        Self(self.0 & !Self::QUAD_EN)
    }
    /// Check if quad data transfer mode is enabled.
    #[inline]
    pub const fn is_quad_enabled(self) -> bool {
        self.0 & Self::QUAD_EN != 0
    }
    /// Enable dual data transfer mode (`DUAL_EN`).
    ///
    /// Valid only in single wire mode;
    /// Not writable when `START` = 1.
    #[doc(alias = "DUAL_EN")]
    #[inline]
    pub const fn enable_dual(self) -> Self {
        Self(self.0 | Self::DUAL_EN)
    }
    /// Disable dual data transfer mode.
    #[inline]
    pub const fn disable_dual(self) -> Self {
        Self(self.0 & !Self::DUAL_EN)
    }
    /// Check if dual data transfer mode is enabled.
    #[inline]
    pub const fn is_dual_enabled(self) -> bool {
        self.0 & Self::DUAL_EN != 0
    }
    /// Set dummy bytes count (`DMY_CNT`).
    ///
    /// Valid in dual and quad modes.
    /// Not writable when `START` = 1.
    #[doc(alias = "DMY_CNT")]
    #[inline]
    pub const fn set_dummy_count(self, count: u8) -> Self {
        Self((self.0 & !Self::DMY_CNT) | (Self::DMY_CNT & ((count as u32) << 24)))
    }
    /// Get dummy bytes count.
    #[inline]
    pub const fn dummy_count(self) -> u8 {
        ((self.0 & Self::DMY_CNT) >> 24) as u8
    }
    /// Set single wire tx data count (`STXD_CNT`).
    ///
    /// Not writable when `START` = 1.
    /// - Single wire mode: includes command, address, and data.
    /// - Dual and quad wire modes: includes command and address only.
    /// - Dual IO mode: includes command only.
    #[doc(alias = "STXD_CNT")]
    #[inline]
    pub const fn set_single_tx_count(self, count: u32) -> Self {
        assert!(
            count <= 0xFFFFFF,
            "Single wire tx data count out of range (expected 0..=0xFFFFFF)"
        );
        Self((self.0 & !Self::STXD_CNT) | (Self::STXD_CNT & count))
    }
    /// Get single wire tx data count.
    #[inline]
    pub const fn single_tx_count(self) -> u32 {
        self.0 & Self::STXD_CNT
    }
}

/// Bit-mode sample mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BMSampleMode {
    /// The SPI master delays by half a clock cycle and samples data on the rising edge of SCLK.
    Delay,
    /// The SPI master samples data on the rising edge of SCLK.
    Standard,
}

/// Bit-mode CS level.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BMCsLevel {
    Low,
    High,
}

/// Bit-mode CS control mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BMCsCtrlMode {
    SpiController,
    Software,
}

/// Bit-mode external device cs pin number.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BMCsPin {
    /// CS0 is valid.
    Cs0,
}

/// Bit-mode CS polarity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BMCsPolarity {
    /// Valid when CS is high.
    High,
    /// Valid when CS is low.
    Low,
}

/// Bus mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BusMode {
    /// Byte-aligned data stream for standard/dual/quad SPI.
    StandardBytes = 0,
    /// Bit-aligned data stream for 3-wire mode.
    QuadBits = 2,
    /// Bit-aligned data stream for standard SPI.
    StandardBits = 3,
}

/// SPI bit-mode transfer configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BitModeTransConfig(u32);

impl BitModeTransConfig {
    const BM_START: u32 = 0x1 << 31;
    const BM_SMP_SEL: u32 = 0x1 << 30;
    const BM_TD: u32 = 0x1 << 25;
    const BM_TD_INTE: u32 = 0x1 << 24;
    const BM_RXCNT: u32 = 0x3F << 16;
    const BM_TXCNT: u32 = 0x3F << 8;
    const BMCS_LEVEL: u32 = 0x1 << 7;
    const BMCS_CTRL_SEL: u32 = 0x1 << 6;
    const BMCS_POL: u32 = 0x1 << 5;
    const BMCS_NUM: u32 = 0x3 << 2;
    const BMOD_SEL: u32 = 0x3;

    /// Set bit-mode start bit (`BM_START`).
    #[doc(alias = "BM_START")]
    #[inline]
    pub const fn set_bit_mode_start(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::BM_START)
        } else {
            Self(self.0 & !Self::BM_START)
        }
    }
    /// Get bit-mode start bit.
    #[inline]
    pub const fn bit_mode_start(self) -> bool {
        self.0 & Self::BM_START != 0
    }
    /// Set bit-mode sample selection (`BM_SMP_SEL`).
    #[doc(alias = "BM_SMP_SEL")]
    #[inline]
    pub const fn set_bm_sample_mode(self, mode: BMSampleMode) -> Self {
        Self((self.0 & !Self::BM_SMP_SEL) | (Self::BM_SMP_SEL & ((mode as u32) << 30)))
    }
    /// Get bit-mode sample selection.
    #[inline]
    pub const fn bm_sample_mode(self) -> BMSampleMode {
        match (self.0 & Self::BM_SMP_SEL) >> 30 {
            0 => BMSampleMode::Delay,
            1 => BMSampleMode::Standard,
            _ => unreachable!(),
        }
    }
    /// Check if bit-mode transfer done (`BM_TD`).
    ///
    /// After the transmission is complete, this bit needs to be written with 1 again to clear it, and check whether this bit is 0;
    /// Only after it is cleared can the next transmission be initiated.
    #[doc(alias = "BM_TD")]
    #[inline]
    pub const fn bit_mode_transfer_done(self) -> bool {
        self.0 & Self::BM_TD != 0
    }
    /// Clear bit-mode transfer done flag.
    #[inline]
    pub const fn clear_bit_mode_transfer_done(self) -> Self {
        Self(self.0 | Self::BM_TD)
    }
    /// Enable bit-mode transfer done interrupt (`BM_TD_INTE`).
    ///
    /// Valid when bus mode is QuadBits or StandardBits.
    #[doc(alias = "BM_TD_INTE")]
    #[inline]
    pub const fn enable_bm_transfer_done_int(self) -> Self {
        Self(self.0 | Self::BM_TD_INTE)
    }
    /// Disable bit-mode transfer done interrupt.
    #[inline]
    pub const fn disable_bm_transfer_done_int(self) -> Self {
        Self(self.0 & !Self::BM_TD_INTE)
    }
    /// Check if bit-mode transfer done interrupt is enabled.
    #[inline]
    pub const fn is_bm_transfer_done_int_enabled(self) -> bool {
        self.0 & Self::BM_TD_INTE != 0
    }
    /// Set bit-mode rx data length (`BM_RXCNT`).
    ///
    /// Valid when bus mode is QuadBits or StandardBits;
    /// Not writable when `BM_START` = 1.
    #[doc(alias = "BM_RXCNT")]
    #[inline]
    pub const fn set_bm_rx_count(self, count: u8) -> Self {
        assert!(
            count <= 0x3F,
            "Bit-mode rx data length out of range (expected 0..=63)"
        );
        Self((self.0 & !Self::BM_RXCNT) | (Self::BM_RXCNT & ((count as u32) << 16)))
    }
    /// Get bit-mode rx data length.
    #[inline]
    pub const fn bm_rx_count(self) -> u8 {
        ((self.0 & Self::BM_RXCNT) >> 16) as u8
    }
    /// Set bit-mode tx data length (`BM_TXCNT`).
    ///
    /// Valid when bus mode is QuadBits or StandardBits;
    /// Not writable when `BM_START` = 1.
    #[doc(alias = "BM_TXCNT")]
    #[inline]
    pub const fn set_bm_tx_count(self, count: u8) -> Self {
        assert!(
            count <= 0x3F,
            "Bit-mode tx count out of range (expected 0..=63)"
        );
        Self((self.0 & !Self::BM_TXCNT) | (Self::BM_TXCNT & ((count as u32) << 8)))
    }
    /// Get bit-mode tx data length.
    #[inline]
    pub const fn bm_tx_count(self) -> u8 {
        ((self.0 & Self::BM_TXCNT) >> 8) as u8
    }
    /// Set bit-mode cs level (`BMCS_LEVEL`).
    ///
    /// Valid when bus mode is QuadBits or StandardBits;
    /// Not writable when `BM_START` = 1.
    #[doc(alias = "BMCS_LEVEL")]
    #[inline]
    pub const fn set_bm_cs_level(self, level: BMCsLevel) -> Self {
        Self((self.0 & !Self::BMCS_LEVEL) | (Self::BMCS_LEVEL & ((level as u32) << 7)))
    }
    /// Get bit-mode cs level.
    #[inline]
    pub const fn bm_cs_level(self) -> BMCsLevel {
        match (self.0 & Self::BMCS_LEVEL) >> 7 {
            0 => BMCsLevel::Low,
            1 => BMCsLevel::High,
            _ => unreachable!(),
        }
    }
    /// Set bit-mode cs control mode (`BMCS_CTRL_SEL`).
    ///
    /// Valid when bus mode is QuadBits or StandardBits;
    /// Not writable when `BM_START` = 1.
    #[doc(alias = "BMCS_CTRL_SEL")]
    #[inline]
    pub const fn set_bm_cs_ctrl_mode(self, mode: BMCsCtrlMode) -> Self {
        Self((self.0 & !Self::BMCS_CTRL_SEL) | (Self::BMCS_CTRL_SEL & ((mode as u32) << 6)))
    }
    /// Get bit-mode cs control mode.
    #[inline]
    pub const fn bm_cs_ctrl_mode(self) -> BMCsCtrlMode {
        match (self.0 & Self::BMCS_CTRL_SEL) >> 6 {
            0 => BMCsCtrlMode::SpiController,
            1 => BMCsCtrlMode::Software,
            _ => unreachable!(),
        }
    }
    /// Set bit-mode cs polarity (`BMCS_POL`).
    ///
    /// Valid when bus mode is QuadBits or StandardBits;
    /// Not writable when `BM_START` = 1.
    #[doc(alias = "BMCS_POL")]
    #[inline]
    pub const fn set_bm_cs_polarity(self, polarity: BMCsPolarity) -> Self {
        Self((self.0 & !Self::BMCS_POL) | (Self::BMCS_POL & ((polarity as u32) << 5)))
    }
    /// Get bit-mode cs polarity.
    #[inline]
    pub const fn bm_cs_polarity(self) -> BMCsPolarity {
        match (self.0 & Self::BMCS_POL) >> 5 {
            0 => BMCsPolarity::High,
            1 => BMCsPolarity::Low,
            _ => unreachable!(),
        }
    }
    /// Set bit-mode cs pin number (`BMCS_NUM`).
    ///
    /// Valid when bus mode is QuadBits or StandardBits;
    /// Not writable when `BM_START` = 1.
    #[doc(alias = "BMCS_NUM")]
    #[inline]
    pub const fn set_bm_cs_pin(self, pin: BMCsPin) -> Self {
        Self((self.0 & !Self::BMCS_NUM) | (Self::BMCS_NUM & ((pin as u32) << 2)))
    }
    /// Get bit-mode cs pin number.
    #[inline]
    pub const fn bm_cs_pin(self) -> BMCsPin {
        match (self.0 & Self::BMCS_NUM) >> 2 {
            0 => BMCsPin::Cs0,
            _ => unreachable!(),
        }
    }
    /// Set bus mode (`BMOD_SEL`).
    #[doc(alias = "BMOD_SEL")]
    #[inline]
    pub const fn set_bus_mode(self, mode: BusMode) -> Self {
        Self((self.0 & !Self::BMOD_SEL) | (Self::BMOD_SEL & (mode as u32)))
    }
    /// Get bus mode.
    #[inline]
    pub const fn bus_mode(self) -> BusMode {
        match self.0 & Self::BMOD_SEL {
            0 => BusMode::StandardBytes,
            2 => BusMode::QuadBits,
            3 => BusMode::StandardBits,
            _ => unreachable!(),
        }
    }
}

/// SPI bit-mode clock configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BitModeClkConfig(u32);

impl BitModeClkConfig {
    const BM_CKDIV: u32 = 0xFF;

    /// Set bit-mode clock divider (`BM_CKDIV`).
    ///
    /// SPI_CLK = SRC_CLK/(2*BM_CKDIV);
    /// If BM_CKDIV = 0, SPI_CLK = SRC_CLK/2.
    #[doc(alias = "BM_CKDIV")]
    #[inline]
    pub const fn set_bm_clkdiv(self, div: u8) -> Self {
        Self((self.0 & !Self::BM_CKDIV) | (Self::BM_CKDIV & (div as u32)))
    }
    /// Get bit-mode clock divider.
    #[inline]
    pub const fn bm_clkdiv(self) -> u8 {
        (self.0 & Self::BM_CKDIV) as u8
    }
}

/// Dummy width.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DummyWidth {
    /// 4-bit dummy.
    FourBit,
    /// 1-bit dummy.
    OneBit,
}

/// SPI burst set register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BurstSet(u32);

impl BurstSet {
    const CMD_INDEX: u32 = 0xFF << 24;
    const DUMMY_BYTE: u32 = 0xF << 20;
    const WIDTH: u32 = 0x1 << 19;
    const AUTO_WRAP_LEN: u32 = 0x1 << 18;
    const WRAP_EN: u32 = 0x1 << 16;
    const SPI_BURST_WRAPPED: u32 = 0xFF << 8;
    const SPI_BURST_LINEAR: u32 = 0xFF;

    /// Set command index (`CMD_INDEX`)
    #[doc(alias = "CMD_INDEX")]
    #[inline]
    pub fn set_cmd_index(self, index: u8) -> Self {
        Self((self.0 & !Self::CMD_INDEX) | (Self::CMD_INDEX & ((index as u32) << 24)))
    }
    /// Get command index.
    #[inline]
    pub fn cmd_index(self) -> u8 {
        ((self.0 & Self::CMD_INDEX) >> 24) as u8
    }
    /// Set dummy byte count (`DUMMY_BYTE`).
    #[doc(alias = "DUMMY_BYTE")]
    #[inline]
    pub fn set_dummy_byte_cnt(self, count: u8) -> Self {
        assert!(
            count <= 0xF,
            "Dummy byte count out of range (expected 0..=15)"
        );
        Self((self.0 & !Self::DUMMY_BYTE) | (Self::DUMMY_BYTE & ((count as u32) << 20)))
    }
    /// Get dummy byte count.
    #[inline]
    pub fn dummy_byte_cnt(self) -> u8 {
        ((self.0 & Self::DUMMY_BYTE) >> 20) as u8
    }
    /// Set dummy width (`WIDTH`).
    #[doc(alias = "WIDTH")]
    #[inline]
    pub fn set_dummy_width(self, width: DummyWidth) -> Self {
        Self((self.0 & !Self::WIDTH) | (Self::WIDTH & ((width as u32) << 19)))
    }
    /// Get dummy width.
    #[inline]
    pub fn dummy_width(self) -> DummyWidth {
        match (self.0 & Self::WIDTH) >> 19 {
            0 => DummyWidth::FourBit,
            1 => DummyWidth::OneBit,
            _ => unreachable!(),
        }
    }
    /// Enable auto wrap length (`AUTO_WRAP_LEN`).
    #[doc(alias = "AUTO_WRAP_LEN")]
    #[inline]
    pub fn enable_auto_wrap_len(self) -> Self {
        Self(self.0 | Self::AUTO_WRAP_LEN)
    }
    /// Disable auto wrap length.
    #[inline]
    pub fn disable_auto_wrap_len(self) -> Self {
        Self(self.0 & !Self::AUTO_WRAP_LEN)
    }
    /// Check if auto wrap length is enabled.
    #[inline]
    pub fn is_auto_wrap_len_enabled(self) -> bool {
        self.0 & Self::AUTO_WRAP_LEN != 0
    }
    /// Enable wrap mode (`WRAP_EN`).
    ///
    /// Must be enabled when the system's Cache function is activated.
    #[doc(alias = "WRAP_EN")]
    #[inline]
    pub fn enable_wrap(self) -> Self {
        Self(self.0 | Self::WRAP_EN)
    }
    /// Disable wrap mode.
    #[inline]
    pub fn disable_wrap(self) -> Self {
        Self(self.0 & !Self::WRAP_EN)
    }
    /// Check if wrap mode is enabled.
    #[inline]
    pub fn is_wrap_enabled(self) -> bool {
        self.0 & Self::WRAP_EN != 0
    }
    /// Set spi burst wrapped command (`SPI_BURST_WRAPPED`).
    #[doc(alias = "SPI_BURST_WRAPPED")]
    #[inline]
    pub fn set_spi_burst_wrapped(self, cmd: u8) -> Self {
        Self((self.0 & !Self::SPI_BURST_WRAPPED) | (Self::SPI_BURST_WRAPPED & ((cmd as u32) << 8)))
    }
    /// Get spi burst wrapped command.
    #[inline]
    pub fn spi_burst_wrapped(self) -> u8 {
        ((self.0 & Self::SPI_BURST_WRAPPED) >> 8) as u8
    }
    /// Set spi burst linear command (`SPI_BURST_LINEAR`).
    #[doc(alias = "SPI_BURST_LINEAR")]
    #[inline]
    pub fn set_spi_burst_linear(self, cmd: u8) -> Self {
        Self((self.0 & !Self::SPI_BURST_LINEAR) | (Self::SPI_BURST_LINEAR & (cmd as u32)))
    }
    /// Get spi burst linear command.
    #[inline]
    pub fn spi_burst_linear(self) -> u8 {
        (self.0 & Self::SPI_BURST_LINEAR) as u8
    }
}

/// SPI read command mode register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ReadCmdMode(u32);

impl ReadCmdMode {
    const CMD_INDEX: u32 = 0xFF << 24;
    const DUMMY_BYTE: u32 = 0xF << 20;
    const ADDR_4BYTE_EN: u32 = 0x1 << 18;
    const READ_MODE_BYTE_EN: u32 = 0x1 << 17;
    const RDCMD_BYPASS_EN: u32 = 0x1 << 16;
    const RDCMD_BYPASS_CODE: u32 = 0xFF << 8;
    const RDCMD_NORMAL_CODE: u32 = 0xFF;

    /// Set command index (`CMD_INDEX`).
    #[doc(alias = "CMD_INDEX")]
    #[inline]
    pub const fn set_cmd_index(self, index: u8) -> Self {
        Self((self.0 & !Self::CMD_INDEX) | (Self::CMD_INDEX & ((index as u32) << 24)))
    }
    /// Get command index.
    #[inline]
    pub const fn cmd_index(self) -> u8 {
        ((self.0 & Self::CMD_INDEX) >> 24) as u8
    }
    /// Set dummy byte count (`DUMMY_BYTE`).
    #[doc(alias = "DUMMY_BYTE")]
    #[inline]
    pub const fn set_dummy_byte_cnt(self, count: u8) -> Self {
        assert!(
            count <= 0xF,
            "Dummy byte count out of range (expected 0..=15)"
        );
        Self((self.0 & !Self::DUMMY_BYTE) | (Self::DUMMY_BYTE & ((count as u32) << 20)))
    }
    /// Get dummy byte count.
    #[inline]
    pub const fn dummy_byte_cnt(self) -> u8 {
        ((self.0 & Self::DUMMY_BYTE) >> 20) as u8
    }
    /// Enable 4-byte address mode (`ADDR_4BYTE_EN`).
    #[doc(alias = "ADDR_4BYTE_EN")]
    #[inline]
    pub const fn enable_4byte_addr(self) -> Self {
        Self(self.0 | Self::ADDR_4BYTE_EN)
    }
    /// Disable 4-byte address mode.
    #[inline]
    pub const fn disable_4byte_addr(self) -> Self {
        Self(self.0 & !Self::ADDR_4BYTE_EN)
    }
    /// Check if 4-byte address mode is enabled.
    #[inline]
    pub const fn is_4byte_addr_enabled(self) -> bool {
        self.0 & Self::ADDR_4BYTE_EN != 0
    }
    /// Enable read mode byte (`READ_MODE_BYTE_EN`).
    ///
    /// When the SPI operates in Dual I/O, Quad I/O, or QPI mode,
    /// the user can configure this bit to enable it;
    /// otherwise, it will be treated as a dummy.
    #[doc(alias = "READ_MODE_BYTE_EN")]
    #[inline]
    pub const fn enable_read_mode_byte(self) -> Self {
        Self(self.0 | Self::READ_MODE_BYTE_EN)
    }
    /// Disable read mode byte.
    #[inline]
    pub const fn disable_read_mode_byte(self) -> Self {
        Self(self.0 & !Self::READ_MODE_BYTE_EN)
    }
    /// Check if read mode byte is enabled.
    #[inline]
    pub const fn is_read_mode_byte_enabled(self) -> bool {
        self.0 & Self::READ_MODE_BYTE_EN != 0
    }
    /// Enable read command bypass mode (`RDCMD_BYPASS_EN`).
    #[doc(alias = "RDCMD_BYPASS_EN")]
    #[inline]
    pub const fn enable_read_cmd_bypass(self) -> Self {
        Self(self.0 | Self::RDCMD_BYPASS_EN)
    }
    /// Disable read command bypass mode.
    #[inline]
    pub const fn disable_read_cmd_bypass(self) -> Self {
        Self(self.0 & !Self::RDCMD_BYPASS_EN)
    }
    /// Check if read command bypass mode is enabled.
    #[inline]
    pub const fn is_read_cmd_bypass_enabled(self) -> bool {
        self.0 & Self::RDCMD_BYPASS_EN != 0
    }
    /// Set read command bypass code (`RDCMD_BYPASS_CODE`).
    #[doc(alias = "RDCMD_BYPASS_CODE")]
    #[inline]
    pub const fn set_read_cmd_bypass_code(self, code: u8) -> Self {
        Self((self.0 & !Self::RDCMD_BYPASS_CODE) | (Self::RDCMD_BYPASS_CODE & ((code as u32) << 8)))
    }
    /// Get read command bypass code.
    #[inline]
    pub const fn read_cmd_bypass_code(self) -> u8 {
        ((self.0 & Self::RDCMD_BYPASS_CODE) >> 8) as u8
    }
    /// Set read command normal code (`RDCMD_NORMAL_CODE`).
    #[doc(alias = "RDCMD_NORMAL_CODE")]
    #[inline]
    pub const fn set_read_cmd_normal_code(self, code: u8) -> Self {
        Self((self.0 & !Self::RDCMD_NORMAL_CODE) | (Self::RDCMD_NORMAL_CODE & (code as u32)))
    }
    /// Get read command normal code.
    #[inline]
    pub const fn read_cmd_normal_code(self) -> u8 {
        (self.0 & Self::RDCMD_NORMAL_CODE) as u8
    }
}

/// SPI wrap length register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct WrapLen(u32);

impl WrapLen {
    const WRAP64: u32 = 0xFF << 24;
    const WRAP32: u32 = 0xFF << 16;
    const WRAP16: u32 = 0xFF << 8;
    const WRAP08: u32 = 0xFF;

    /// Set command argument for 64-byte wrap length (`WRAP64`).
    #[doc(alias = "WRAP64")]
    #[inline]
    pub const fn set_wrap64(self, arg: u8) -> Self {
        Self((self.0 & !Self::WRAP64) | (Self::WRAP64 & ((arg as u32) << 24)))
    }
    /// Get command argument for 64-byte wrap length.
    #[inline]
    pub const fn wrap64(self) -> u8 {
        ((self.0 & Self::WRAP64) >> 24) as u8
    }
    /// Set command argument for 32-byte wrap length (`WRAP32`).
    #[doc(alias = "WRAP32")]
    #[inline]
    pub const fn set_wrap32(self, arg: u8) -> Self {
        Self((self.0 & !Self::WRAP32) | (Self::WRAP32 & ((arg as u32) << 16)))
    }
    /// Get command argument for 32-byte wrap length.
    #[inline]
    pub const fn wrap32(self) -> u8 {
        ((self.0 & Self::WRAP32) >> 16) as u8
    }
    /// Set command argument for 16-byte wrap length (`WRAP16`).
    #[doc(alias = "WRAP16")]
    #[inline]
    pub const fn set_wrap16(self, arg: u8) -> Self {
        Self((self.0 & !Self::WRAP16) | (Self::WRAP16 & ((arg as u32) << 8)))
    }
    /// Get command argument for 16-byte wrap length.
    #[inline]
    pub const fn wrap16(self) -> u8 {
        ((self.0 & Self::WRAP16) >> 8) as u8
    }
    /// Set command argument for 8-byte wrap length (`WRAP08`).
    #[doc(alias = "WRAP08")]
    #[inline]
    pub const fn set_wrap08(self, arg: u8) -> Self {
        Self((self.0 & !Self::WRAP08) | (Self::WRAP08 & (arg as u32)))
    }
    /// Get command argument for 8-byte wrap length.
    #[inline]
    pub const fn wrap08(self) -> u8 {
        (self.0 & Self::WRAP08) as u8
    }
}

/// SPI inner dma write transaction length register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IdmaTxLen(u32);

impl IdmaTxLen {
    const IDMA_TXLEN: u32 = 0xFFFFFF;

    /// Set idma tx length (`IDMA_TXLEN`)
    #[doc(alias = "IDMA_TXLEN")]
    #[inline]
    pub const fn set_idma_tx_len(self, len: u32) -> Self {
        assert!(
            len <= 0xFFFFFF,
            "IDMA tx length out of range (expected 0..=0xFFFFFF)"
        );
        Self((self.0 & !Self::IDMA_TXLEN) | (Self::IDMA_TXLEN & len))
    }
    /// Get idma tx length.
    #[inline]
    pub const fn idma_tx_len(self) -> u32 {
        self.0 & Self::IDMA_TXLEN
    }
}

/// SPI inner dma read transaction length register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IdmaRxLen(u32);

impl IdmaRxLen {
    const IDMA_RXLEN: u32 = 0xFFFFFF;

    /// Set idma rx length (`IDMA_RXLEN`)
    #[doc(alias = "IDMA_RXLEN")]
    #[inline]
    pub const fn set_idma_rx_len(self, len: u32) -> Self {
        assert!(
            len <= 0xFFFFFF,
            "IDMA rx length out of range (expected 0..=0xFFFFFF)"
        );
        Self((self.0 & !Self::IDMA_RXLEN) | (Self::IDMA_RXLEN & len))
    }
    /// Get idma rx length.
    #[inline]
    pub const fn idma_rx_len(self) -> u32 {
        self.0 & Self::IDMA_RXLEN
    }
}

/// Tx burst length.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdmaTxBurstLen {
    /// 4 words per burst.
    Burst4,
    /// 16 words per burst.
    Burst16,
    /// 32 words per burst.
    Burst32,
}

/// Rx burst length.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdmaRxBurstLen {
    /// 4 words per burst.
    Burst4,
    /// 16 words per burst.
    Burst16,
    /// 32 words per burst.
    Burst32,
    /// 64 words per burst.
    Burst64,
}

/// SPI inner dma burst config register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IdmaBurstConfig(u32);

impl IdmaBurstConfig {
    const RX_PRI_EN: u32 = 0x1 << 13;
    const AUTO_LEN: u32 = 0x1 << 12;
    const RX_BURST_LEN: u32 = 0x3 << 10;
    const TX_BURST_LEN: u32 = 0x3 << 8;

    /// Enable rx priority transmission (`RX_PRI_EN`).
    #[doc(alias = "RX_PRI_EN")]
    #[inline]
    pub const fn enable_rx_priority(self) -> Self {
        Self(self.0 | Self::RX_PRI_EN)
    }
    /// Disable rx priority transmission.
    #[inline]
    pub const fn disable_rx_priority(self) -> Self {
        Self(self.0 & !Self::RX_PRI_EN)
    }
    /// Check if rx priority transmission is enabled.
    #[inline]
    pub const fn is_rx_priority_enabled(self) -> bool {
        self.0 & Self::RX_PRI_EN != 0
    }
    /// Enable auto burst length (`AUTO_LEN`).
    #[doc(alias = "AUTO_LEN")]
    #[inline]
    pub const fn enable_auto_len(self) -> Self {
        Self(self.0 | Self::AUTO_LEN)
    }
    /// Disable auto burst length.
    #[inline]
    pub const fn disable_auto_len(self) -> Self {
        Self(self.0 & !Self::AUTO_LEN)
    }
    /// Check if auto burst length is enabled.
    #[inline]
    pub const fn is_auto_len_enabled(self) -> bool {
        self.0 & Self::AUTO_LEN != 0
    }
    /// Set rx burst length (`RX_BURST_LEN`).
    #[doc(alias = "RX_BURST_LEN")]
    #[inline]
    pub const fn set_rx_burst_len(self, len: IdmaRxBurstLen) -> Self {
        Self((self.0 & !Self::RX_BURST_LEN) | (Self::RX_BURST_LEN & ((len as u32) << 10)))
    }
    /// Get rx burst length.
    #[inline]
    pub const fn rx_burst_len(self) -> IdmaRxBurstLen {
        match (self.0 & Self::RX_BURST_LEN) >> 10 {
            0 => IdmaRxBurstLen::Burst4,
            1 => IdmaRxBurstLen::Burst16,
            2 => IdmaRxBurstLen::Burst32,
            3 => IdmaRxBurstLen::Burst64,
            _ => unreachable!(),
        }
    }
    /// Set tx burst length (`TX_BURST_LEN`).
    #[doc(alias = "TX_BURST_LEN")]
    #[inline]
    pub const fn set_tx_burst_len(self, len: IdmaTxBurstLen) -> Self {
        Self((self.0 & !Self::TX_BURST_LEN) | (Self::TX_BURST_LEN & ((len as u32) << 8)))
    }
    /// Get tx burst length.
    #[inline]
    pub const fn tx_burst_len(self) -> IdmaTxBurstLen {
        match (self.0 & Self::TX_BURST_LEN) >> 8 {
            0 => IdmaTxBurstLen::Burst4,
            1 => IdmaTxBurstLen::Burst16,
            2 => IdmaTxBurstLen::Burst32,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_should_panic;
    use core::mem::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, version), 0x0);
        assert_eq!(offset_of!(RegisterBlock, config), 0x4);
        assert_eq!(offset_of!(RegisterBlock, trans_config), 0x8);
        assert_eq!(offset_of!(RegisterBlock, int_control), 0x10);
        assert_eq!(offset_of!(RegisterBlock, int_status), 0x14);
        assert_eq!(offset_of!(RegisterBlock, fifo_control), 0x18);
        assert_eq!(offset_of!(RegisterBlock, fifo_status), 0x1C);
        assert_eq!(offset_of!(RegisterBlock, clk_config), 0x24);
        assert_eq!(offset_of!(RegisterBlock, misc), 0x28);
        assert_eq!(offset_of!(RegisterBlock, total_bytes_cnt), 0x30);
        assert_eq!(offset_of!(RegisterBlock, trans_write_cnt), 0x34);
        assert_eq!(offset_of!(RegisterBlock, trans_misc_control), 0x38);
        assert_eq!(offset_of!(RegisterBlock, bit_mode_trans_config), 0x40);
        assert_eq!(offset_of!(RegisterBlock, bit_mode_clk_config), 0x44);
        assert_eq!(offset_of!(RegisterBlock, bit_mode_tx_data), 0x48);
        assert_eq!(offset_of!(RegisterBlock, bit_mode_rx_data), 0x4C);
        assert_eq!(offset_of!(RegisterBlock, burst_set), 0x50);
        assert_eq!(offset_of!(RegisterBlock, read_cmd), 0x54);
        assert_eq!(offset_of!(RegisterBlock, wrap_len), 0x60);
        assert_eq!(offset_of!(RegisterBlock, idma_tx_len), 0x8C);
        assert_eq!(offset_of!(RegisterBlock, idma_rx_len), 0x90);
        assert_eq!(offset_of!(RegisterBlock, idma_tx_addr), 0x94);
        assert_eq!(offset_of!(RegisterBlock, idma_rx_addr), 0x98);
        assert_eq!(offset_of!(RegisterBlock, idma_burst_cfg), 0x9C);
        assert_eq!(offset_of!(RegisterBlock, tx_data), 0x200);
        assert_eq!(offset_of!(RegisterBlock, rx_data), 0x300);
    }

    #[test]
    fn struct_version_functions() {
        let val = Version(0xFFFF_FFFF);
        assert_eq!(val.version(), 0xFFFF);
        assert_eq!(val.0, 0xFFFF_FFFF);
    }

    #[test]
    fn struct_config_functions() {
        let mut val = Config(0x0).set_ctrl_rst(true);
        assert!(val.ctrl_rst());
        assert_eq!(val.0, 0x8000_0000);
        val = val.set_ctrl_rst(false);
        assert!(!val.ctrl_rst());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_rx_full_stop();
        assert!(val.is_rx_full_stop_enabled());
        assert_eq!(val.0, 0x0000_0080);
        val = val.disable_rx_full_stop();
        assert!(!val.is_rx_full_stop_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_tx_idma();
        assert!(val.is_tx_idma_enabled());
        assert_eq!(val.0, 0x0000_0010);
        val = val.disable_tx_idma();
        assert!(!val.is_tx_idma_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_rx_idma();
        assert!(val.is_rx_idma_enabled());
        assert_eq!(val.0, 0x0000_0008);
        val = val.disable_rx_idma();
        assert!(!val.is_rx_idma_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_access_mode(AccessMode::AXI);
        assert_eq!(val.access_mode(), AccessMode::AXI);
        assert_eq!(val.0, 0x0000_0004);
        val = val.set_access_mode(AccessMode::AHB);
        assert_eq!(val.access_mode(), AccessMode::AHB);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_work_mode(WorkMode::Master);
        assert_eq!(val.work_mode(), WorkMode::Master);
        assert_eq!(val.0, 0x0000_0002);
        val = val.set_work_mode(WorkMode::Slave);
        assert_eq!(val.work_mode(), WorkMode::Slave);
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_ctrl();
        assert!(val.is_ctrl_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_ctrl();
        assert!(!val.is_ctrl_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_transfer_config_functions() {
        let mut val = TransferConfig(0x0).set_start(true);
        assert!(val.start());
        assert_eq!(val.0, 0x8000_0000);
        val = val.set_start(false);
        assert!(!val.start());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_slave_output();
        assert!(val.is_slave_output_enabled());
        assert_eq!(val.0, 0x0400_0000);
        val = val.disable_slave_output();
        assert!(!val.is_slave_output_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_three_wire();
        assert!(val.is_three_wire_enabled());
        assert_eq!(val.0, 0x0200_0000);
        val = val.disable_three_wire();
        assert!(!val.is_three_wire_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_tx_data_delay();
        assert!(val.is_tx_data_delay_enabled());
        assert_eq!(val.0, 0x0000_4000);
        val = val.disable_tx_data_delay();
        assert!(!val.is_tx_data_delay_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.disable_rx_data_delay();
        assert!(!val.is_rx_data_delay_enabled());
        assert_eq!(val.0, 0x0000_2000);
        val = val.enable_rx_data_delay();
        assert!(val.is_rx_data_delay_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_lsb_transmit();
        assert!(val.is_lsb_transmit_enabled());
        assert_eq!(val.0, 0x0000_1000);
        val = val.disable_lsb_transmit();
        assert!(!val.is_lsb_transmit_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_rx_inner_delay();
        assert!(val.is_rx_inner_delay_enabled());
        assert_eq!(val.0, 0x0000_0800);
        val = val.disable_rx_inner_delay();
        assert!(!val.is_rx_inner_delay_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_high_speed_write();
        assert!(val.is_high_speed_write_enabled());
        assert_eq!(val.0, 0x0000_0400);
        val = val.disable_high_speed_write();
        assert!(!val.is_high_speed_write_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_dummy_byte_val();
        assert!(val.is_dummy_byte_val_enabled());
        assert_eq!(val.0, 0x0000_0200);
        val = val.disable_dummy_byte_val();
        assert!(!val.is_dummy_byte_val_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_discard_invalid_data();
        assert!(val.is_discard_invalid_data_enabled());
        assert_eq!(val.0, 0x0000_0100);
        val = val.disable_discard_invalid_data();
        assert!(!val.is_discard_invalid_data_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_cs_level(CsLevel::High);
        assert_eq!(val.cs_level(), CsLevel::High);
        assert_eq!(val.0, 0x0000_0080);
        val = val.set_cs_level(CsLevel::Low);
        assert_eq!(val.cs_level(), CsLevel::Low);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_cs_ctrl_mode(CsCtrlMode::Software);
        assert_eq!(val.cs_ctrl_mode(), CsCtrlMode::Software);
        assert_eq!(val.0, 0x0000_0040);
        val = val.set_cs_ctrl_mode(CsCtrlMode::SpiController);
        assert_eq!(val.cs_ctrl_mode(), CsCtrlMode::SpiController);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_cs_pin_num(CsPin::Cs1);
        assert_eq!(val.cs_pin_num(), CsPin::Cs1);
        assert_eq!(val.0, 0x0000_0010);
        val = val.set_cs_pin_num(CsPin::Cs0);
        assert_eq!(val.cs_pin_num(), CsPin::Cs0);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_cs_valid_mode(CsValidMode::InactiveWhenIdle);
        assert_eq!(val.cs_valid_mode(), CsValidMode::InactiveWhenIdle);
        assert_eq!(val.0, 0x0000_0008);
        val = val.set_cs_valid_mode(CsValidMode::Continuous);
        assert_eq!(val.cs_valid_mode(), CsValidMode::Continuous);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_cs_pol(CsPolarity::High);
        assert_eq!(val.cs_pol(), CsPolarity::High);
        assert_eq!(val.0, 0x0000_0004);
        val = val.set_cs_pol(CsPolarity::Low);
        assert_eq!(val.cs_pol(), CsPolarity::Low);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_clk_pol(ClockPolarity::High);
        assert_eq!(val.clk_pol(), ClockPolarity::High);
        assert_eq!(val.0, 0x0000_0002);
        val = val.set_clk_pol(ClockPolarity::Low);
        assert_eq!(val.clk_pol(), ClockPolarity::Low);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_clk_phase(ClockPhase::SecondEdge);
        assert_eq!(val.clk_phase(), ClockPhase::SecondEdge);
        assert_eq!(val.0, 0x0000_0001);
        val = val.set_clk_phase(ClockPhase::FirstEdge);
        assert_eq!(val.clk_phase(), ClockPhase::FirstEdge);
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_int_control_functions() {
        let mut val = IntControl(0x0).enable_tx_dma_done_int();
        assert!(val.is_tx_dma_done_int_enabled());
        assert_eq!(val.0, 0x0080_0000);
        val = val.disable_tx_dma_done_int();
        assert!(!val.is_tx_dma_done_int_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_rx_dma_done_int();
        assert!(val.is_rx_dma_done_int_enabled());
        assert_eq!(val.0, 0x0040_0000);
        val = val.disable_rx_dma_done_int();
        assert!(!val.is_rx_dma_done_int_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_rx_fifo_unalign_int();
        assert!(val.is_rx_fifo_unalign_int_enabled());
        assert_eq!(val.0, 0x0020_0000);
        val = val.disable_rx_fifo_unalign_int();
        assert!(!val.is_rx_fifo_unalign_int_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_idma_error_int();
        assert!(val.is_idma_error_int_enabled());
        assert_eq!(val.0, 0x0010_0000);
        val = val.disable_idma_error_int();
        assert!(!val.is_idma_error_int_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_axi_tran_done_int();
        assert!(val.is_axi_tran_done_int_enabled());
        assert_eq!(val.0, 0x0008_0000);
        val = val.disable_axi_tran_done_int();
        assert!(!val.is_axi_tran_done_int_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_axi_cfg_error_int();
        assert!(val.is_axi_cfg_error_int_enabled());
        assert_eq!(val.0, 0x0004_0000);
        val = val.disable_axi_cfg_error_int();
        assert!(!val.is_axi_cfg_error_int_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_axi_tran_error_int();
        assert!(val.is_axi_tran_error_int_enabled());
        assert_eq!(val.0, 0x0002_0000);
        val = val.disable_axi_tran_error_int();
        assert!(!val.is_axi_tran_error_int_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_ahb_tran_error_int();
        assert!(val.is_ahb_tran_error_int_enabled());
        assert_eq!(val.0, 0x0001_0000);
        val = val.disable_ahb_tran_error_int();
        assert!(!val.is_ahb_tran_error_int_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_cs_invalid_int();
        assert!(val.is_cs_invalid_int_enabled());
        assert_eq!(val.0, 0x0000_2000);
        val = val.disable_cs_invalid_int();
        assert!(!val.is_cs_invalid_int_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_transfer_done_int();
        assert!(val.is_transfer_done_int_enabled());
        assert_eq!(val.0, 0x0000_1000);
        val = val.disable_transfer_done_int();
        assert!(!val.is_transfer_done_int_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_tx_fifo_underrun_int();
        assert!(val.is_tx_fifo_underrun_int_enabled());
        assert_eq!(val.0, 0x0000_0800);
        val = val.disable_tx_fifo_underrun_int();
        assert!(!val.is_tx_fifo_underrun_int_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_tx_fifo_overflow_int();
        assert!(val.is_tx_fifo_overflow_int_enabled());
        assert_eq!(val.0, 0x0000_0400);
        val = val.disable_tx_fifo_overflow_int();
        assert!(!val.is_tx_fifo_overflow_int_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_rx_fifo_underrun_int();
        assert!(val.is_rx_fifo_underrun_int_enabled());
        assert_eq!(val.0, 0x0000_0200);
        val = val.disable_rx_fifo_underrun_int();
        assert!(!val.is_rx_fifo_underrun_int_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_rx_fifo_overflow_int();
        assert!(val.is_rx_fifo_overflow_int_enabled());
        assert_eq!(val.0, 0x0000_0100);
        val = val.disable_rx_fifo_overflow_int();
        assert!(!val.is_rx_fifo_overflow_int_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_tx_fifo_full_int();
        assert!(val.is_tx_fifo_full_int_enabled());
        assert_eq!(val.0, 0x0000_0040);
        val = val.disable_tx_fifo_full_int();
        assert!(!val.is_tx_fifo_full_int_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_tx_fifo_empty_int();
        assert!(val.is_tx_fifo_empty_int_enabled());
        assert_eq!(val.0, 0x0000_0020);
        val = val.disable_tx_fifo_empty_int();
        assert!(!val.is_tx_fifo_empty_int_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_tx_fifo_ready_int();
        assert!(val.is_tx_fifo_ready_int_enabled());
        assert_eq!(val.0, 0x0000_0010);
        val = val.disable_tx_fifo_ready_int();
        assert!(!val.is_tx_fifo_ready_int_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_rx_fifo_full_int();
        assert!(val.is_rx_fifo_full_int_enabled());
        assert_eq!(val.0, 0x0000_0004);
        val = val.disable_rx_fifo_full_int();
        assert!(!val.is_rx_fifo_full_int_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_rx_fifo_empty_int();
        assert!(val.is_rx_fifo_empty_int_enabled());
        assert_eq!(val.0, 0x0000_0002);
        val = val.disable_rx_fifo_empty_int();
        assert!(!val.is_rx_fifo_empty_int_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_rx_fifo_ready_int();
        assert!(val.is_rx_fifo_ready_int_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_rx_fifo_ready_int();
        assert!(!val.is_rx_fifo_ready_int_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_int_status_functions() {
        let mut val = IntStatus(0x0).clear_tx_dma_done_int();
        assert!(val.is_tx_dma_done_int_pending());
        assert_eq!(val.0, 0x0080_0000);

        val = IntStatus(0x0).clear_rx_dma_done_int();
        assert!(val.is_rx_dma_done_int_pending());
        assert_eq!(val.0, 0x0040_0000);

        val = IntStatus(0x0).clear_rx_fifo_unalign_int();
        assert!(val.is_rx_fifo_unalign_int_pending());
        assert_eq!(val.0, 0x0020_0000);

        val = IntStatus(0x0).clear_idma_error_int();
        assert!(val.is_idma_error_int_pending());
        assert_eq!(val.0, 0x0010_0000);

        val = IntStatus(0x0).clear_axi_tran_done_int();
        assert!(val.is_axi_tran_done_int_pending());
        assert_eq!(val.0, 0x0008_0000);

        val = IntStatus(0x0).clear_axi_cfg_error_int();
        assert!(val.is_axi_cfg_error_int_pending());
        assert_eq!(val.0, 0x0004_0000);

        val = IntStatus(0x0).clear_axi_tran_error_int();
        assert!(val.is_axi_tran_error_int_pending());
        assert_eq!(val.0, 0x0002_0000);

        val = IntStatus(0x0).clear_ahb_tran_error_int();
        assert!(val.is_ahb_tran_error_int_pending());
        assert_eq!(val.0, 0x0001_0000);

        val = IntStatus(0x0).clear_cs_invalid_int();
        assert!(val.is_cs_invalid_int_pending());
        assert_eq!(val.0, 0x0000_2000);

        val = IntStatus(0x0).clear_transfer_done_int();
        assert!(val.is_transfer_done_int_pending());
        assert_eq!(val.0, 0x0000_1000);

        val = IntStatus(0x0).clear_tx_fifo_underrun_int();
        assert!(val.is_tx_fifo_underrun_int_pending());
        assert_eq!(val.0, 0x0000_0800);

        val = IntStatus(0x0).clear_tx_fifo_overflow_int();
        assert!(val.is_tx_fifo_overflow_int_pending());
        assert_eq!(val.0, 0x0000_0400);

        val = IntStatus(0x0).clear_rx_fifo_underrun_int();
        assert!(val.is_rx_fifo_underrun_int_pending());
        assert_eq!(val.0, 0x0000_0200);

        val = IntStatus(0x0).clear_rx_fifo_overflow_int();
        assert!(val.is_rx_fifo_overflow_int_pending());
        assert_eq!(val.0, 0x0000_0100);

        val = IntStatus(0x0).clear_tx_fifo_full_int();
        assert!(val.is_tx_fifo_full_int_pending());
        assert_eq!(val.0, 0x0000_0040);

        val = IntStatus(0x0).clear_tx_fifo_empty_int();
        assert!(val.is_tx_fifo_empty_int_pending());
        assert_eq!(val.0, 0x0000_0020);

        val = IntStatus(0x0).clear_tx_fifo_ready_int();
        assert!(val.is_tx_fifo_ready_int_pending());
        assert_eq!(val.0, 0x0000_0010);

        val = IntStatus(0x0).clear_rx_fifo_full_int();
        assert!(val.is_rx_fifo_full_int_pending());
        assert_eq!(val.0, 0x0000_0004);

        val = IntStatus(0x0).clear_rx_fifo_empty_int();
        assert!(val.is_rx_fifo_empty_int_pending());
        assert_eq!(val.0, 0x0000_0002);

        val = IntStatus(0x0).clear_rx_fifo_ready_int();
        assert!(val.is_rx_fifo_ready_int_pending());
        assert_eq!(val.0, 0x0000_0001);
    }

    #[test]
    fn struct_fifo_control_functions() {
        let mut val = FifoControl(0x0).reset_tx_fifo();
        assert!(val.tx_fifo_reset());
        assert_eq!(val.0, 0x8000_0000);

        val = FifoControl(0x0).enable_tx_fifo_dma_req();
        assert!(val.is_tx_fifo_dma_req_enabled());
        assert_eq!(val.0, 0x0100_0000);
        val = val.disable_tx_fifo_dma_req();
        assert!(!val.is_tx_fifo_dma_req_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_tx_fifo_water_mark(0xFF);
        assert_eq!(val.tx_fifo_water_mark(), 0xFF);
        assert_eq!(val.0, 0x00FF_0000);

        val = FifoControl(0x0).reset_rx_fifo();
        assert!(val.rx_fifo_reset());
        assert_eq!(val.0, 0x0000_8000);

        val = FifoControl(0x0).enable_rx_fifo_dma_req();
        assert!(val.is_rx_fifo_dma_req_enabled());
        assert_eq!(val.0, 0x0000_0100);
        val = val.disable_rx_fifo_dma_req();
        assert!(!val.is_rx_fifo_dma_req_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_rx_fifo_water_mark(0xFF);
        assert_eq!(val.rx_fifo_water_mark(), 0xFF);
        assert_eq!(val.0, 0x0000_00FF);
    }

    #[test]
    fn struct_fifo_status_functions() {
        let mut val = FifoStatus(0x8000_0000);
        assert!(val.tx_fifo_write_buffer_status());

        val = FifoStatus(0x7000_0000);
        assert_eq!(val.tx_fifo_write_buffer_count(), 7);

        val = FifoStatus(0x00FF_0000);
        assert_eq!(val.tx_fifo_count(), 0xFF);

        val = FifoStatus(0x0000_8000);
        assert!(val.rx_fifo_read_buffer_status());

        val = FifoStatus(0x0000_7000);
        assert_eq!(val.rx_fifo_read_buffer_count(), 7);

        val = FifoStatus(0x0000_00FF);
        assert_eq!(val.rx_fifo_count(), 0xFF);
    }

    #[test]
    fn struct_clk_config_functions() {
        let mut val = ClkConfig(0x0).set_clock_divider_selection(ClockDivSel::Div2);
        assert_eq!(val.clock_divider_selection(), ClockDivSel::Div2);
        assert_eq!(val.0, 0x0000_1000);
        val = val.set_clock_divider_selection(ClockDivSel::Div1);
        assert_eq!(val.clock_divider_selection(), ClockDivSel::Div1);
        assert_eq!(val.0, 0x0000_0000);

        val = ClkConfig(0x0).set_clk_div_1(0xF);
        assert_eq!(val.clk_div_1(), 0xF);
        assert_eq!(val.0, 0x0000_0F00);

        val = ClkConfig(0x0).set_clk_div_2(0xFF);
        assert_eq!(val.clk_div_2(), 0xFF);
        assert_eq!(val.0, 0x0000_00FF);
    }

    test_should_panic!((
        test_clk_cfg_set_clk_div_1_panic,
        ClkConfig(0x0).set_clk_div_1(0x10),
        "CKDIV1 out of range (expected 0..=15)"
    ));

    #[test]
    fn struct_total_bytes_cnt_functions() {
        let val = TotalBytesCnt(0x0).set_total_bytes(0xFFFFFF);
        assert_eq!(val.total_bytes(), 0xFFFFFF);
        assert_eq!(val.0, 0x00FF_FFFF);
    }

    test_should_panic!((
        test_total_bytes_cnt_set_total_bytes_panic,
        TotalBytesCnt(0x0).set_total_bytes(0x1000000),
        "Total bytes out of range (expected 0..=0xFFFFFF)"
    ));

    #[test]
    fn struct_trans_cnt_functions() {
        let val = TransCnt(0x0).set_tx_cnt(0xFFFFFF);
        assert_eq!(val.tx_cnt(), 0xFFFFFF);
        assert_eq!(val.0, 0x00FF_FFFF);
    }

    test_should_panic!((
        test_trans_cnt_set_tx_cnt_panic,
        TransCnt(0x0).set_tx_cnt(0x1000000),
        "Transmit write counter out of range (expected 0..=0xFFFFFF)"
    ));

    #[test]
    fn struct_trans_misc_control_functions() {
        let mut val = TransMiscControl(0x0).enable_qpi();
        assert!(val.is_qpi_enabled());
        assert_eq!(val.0, 0x8000_0000);
        val = val.disable_qpi();
        assert!(!val.is_qpi_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_qaddr();
        assert!(val.is_qaddr_enabled());
        assert_eq!(val.0, 0x4000_0000);
        val = val.disable_qaddr();
        assert!(!val.is_qaddr_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_quad();
        assert!(val.is_quad_enabled());
        assert_eq!(val.0, 0x2000_0000);
        val = val.disable_quad();
        assert!(!val.is_quad_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_dual();
        assert!(val.is_dual_enabled());
        assert_eq!(val.0, 0x1000_0000);
        val = val.disable_dual();
        assert!(!val.is_dual_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_dummy_count(0xF);
        assert_eq!(val.dummy_count(), 0xF);
        assert_eq!(val.0, 0x0F00_0000);

        val = TransMiscControl(0x0).set_single_tx_count(0xFFFFFF);
        assert_eq!(val.single_tx_count(), 0xFFFFFF);
        assert_eq!(val.0, 0x00FF_FFFF);
    }

    test_should_panic!((
        test_trans_misc_ctrl_set_single_tx_count_panic,
        TransMiscControl(0x0).set_single_tx_count(0x1000000),
        "Single wire tx data count out of range (expected 0..=0xFFFFFF)"
    ));

    #[test]
    fn struct_bit_mode_trans_config_functions() {
        let mut val = BitModeTransConfig(0x0).set_bit_mode_start(true);
        assert!(val.bit_mode_start());
        assert_eq!(val.0, 0x8000_0000);
        val = val.set_bit_mode_start(false);
        assert!(!val.bit_mode_start());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_bm_sample_mode(BMSampleMode::Standard);
        assert_eq!(val.bm_sample_mode(), BMSampleMode::Standard);
        assert_eq!(val.0, 0x4000_0000);
        val = val.set_bm_sample_mode(BMSampleMode::Delay);
        assert_eq!(val.bm_sample_mode(), BMSampleMode::Delay);
        assert_eq!(val.0, 0x0000_0000);

        val = val.clear_bit_mode_transfer_done();
        assert!(val.bit_mode_transfer_done());
        assert_eq!(val.0, 0x0200_0000);

        val = BitModeTransConfig(0x0).enable_bm_transfer_done_int();
        assert!(val.is_bm_transfer_done_int_enabled());
        assert_eq!(val.0, 0x0100_0000);
        val = val.disable_bm_transfer_done_int();
        assert!(!val.is_bm_transfer_done_int_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_bm_rx_count(0x3F);
        assert_eq!(val.bm_rx_count(), 0x3F);
        assert_eq!(val.0, 0x003F_0000);

        val = BitModeTransConfig(0x0).set_bm_tx_count(0x3F);
        assert_eq!(val.bm_tx_count(), 0x3F);
        assert_eq!(val.0, 0x0000_3F00);

        val = BitModeTransConfig(0x0).set_bm_cs_level(BMCsLevel::High);
        assert_eq!(val.bm_cs_level(), BMCsLevel::High);
        assert_eq!(val.0, 0x0000_0080);
        val = val.set_bm_cs_level(BMCsLevel::Low);
        assert_eq!(val.bm_cs_level(), BMCsLevel::Low);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_bm_cs_ctrl_mode(BMCsCtrlMode::Software);
        assert_eq!(val.bm_cs_ctrl_mode(), BMCsCtrlMode::Software);
        assert_eq!(val.0, 0x0000_0040);
        val = val.set_bm_cs_ctrl_mode(BMCsCtrlMode::SpiController);
        assert_eq!(val.bm_cs_ctrl_mode(), BMCsCtrlMode::SpiController);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_bm_cs_polarity(BMCsPolarity::Low);
        assert_eq!(val.bm_cs_polarity(), BMCsPolarity::Low);
        assert_eq!(val.0, 0x0000_0020);
        val = val.set_bm_cs_polarity(BMCsPolarity::High);
        assert_eq!(val.bm_cs_polarity(), BMCsPolarity::High);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_bm_cs_pin(BMCsPin::Cs0);
        assert_eq!(val.bm_cs_pin(), BMCsPin::Cs0);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_bus_mode(BusMode::QuadBits);
        assert_eq!(val.bus_mode(), BusMode::QuadBits);
        assert_eq!(val.0, 0x0000_0002);
        val = val.set_bus_mode(BusMode::StandardBits);
        assert_eq!(val.bus_mode(), BusMode::StandardBits);
        assert_eq!(val.0, 0x0000_0003);
        val = val.set_bus_mode(BusMode::StandardBytes);
        assert_eq!(val.bus_mode(), BusMode::StandardBytes);
        assert_eq!(val.0, 0x0000_0000);
    }

    test_should_panic!((
        test_bm_trans_cfg_set_bm_rx_count_panic,
        BitModeTransConfig(0x0).set_bm_rx_count(0x40),
        "Bit-mode rx data length out of range (expected 0..=63)"
    ));

    test_should_panic!((
        test_bm_trans_cfg_set_bm_tx_count_panic,
        BitModeTransConfig(0x0).set_bm_tx_count(0x40),
        "Bit-mode tx count out of range (expected 0..=63)"
    ));

    #[test]
    fn struct_bit_mode_clk_config_functions() {
        let val = BitModeClkConfig(0x0).set_bm_clkdiv(0xFF);
        assert_eq!(val.bm_clkdiv(), 0xFF);
        assert_eq!(val.0, 0x0000_00FF);
    }

    #[test]
    fn struct_burst_set_functions() {
        let mut val = BurstSet(0x0).set_cmd_index(0xFF);
        assert_eq!(val.cmd_index(), 0xFF);
        assert_eq!(val.0, 0xFF00_0000);

        val = BurstSet(0x0).set_dummy_byte_cnt(0xF);
        assert_eq!(val.dummy_byte_cnt(), 0xF);
        assert_eq!(val.0, 0x00F0_0000);

        val = BurstSet(0x0).set_dummy_width(DummyWidth::OneBit);
        assert_eq!(val.dummy_width(), DummyWidth::OneBit);
        assert_eq!(val.0, 0x0008_0000);
        val = val.set_dummy_width(DummyWidth::FourBit);
        assert_eq!(val.dummy_width(), DummyWidth::FourBit);
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_auto_wrap_len();
        assert!(val.is_auto_wrap_len_enabled());
        assert_eq!(val.0, 0x0004_0000);
        val = val.disable_auto_wrap_len();
        assert!(!val.is_auto_wrap_len_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_wrap();
        assert!(val.is_wrap_enabled());
        assert_eq!(val.0, 0x0001_0000);
        val = val.disable_wrap();
        assert!(!val.is_wrap_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_spi_burst_wrapped(0xFF);
        assert_eq!(val.spi_burst_wrapped(), 0xFF);
        assert_eq!(val.0, 0x0000_FF00);

        val = BurstSet(0x0).set_spi_burst_linear(0xFF);
        assert_eq!(val.spi_burst_linear(), 0xFF);
        assert_eq!(val.0, 0x0000_00FF);
    }

    test_should_panic!((
        test_burst_set_set_dummy_byte_cnt_panic,
        BurstSet(0x0).set_dummy_byte_cnt(0x10),
        "Dummy byte count out of range (expected 0..=15)"
    ));

    #[test]
    fn struct_read_cmd_mode_functions() {
        let mut val = ReadCmdMode(0x0).set_cmd_index(0xFF);
        assert_eq!(val.cmd_index(), 0xFF);
        assert_eq!(val.0, 0xFF00_0000);

        val = ReadCmdMode(0x0).set_dummy_byte_cnt(0xF);
        assert_eq!(val.dummy_byte_cnt(), 0xF);
        assert_eq!(val.0, 0x00F0_0000);

        val = ReadCmdMode(0x0).enable_4byte_addr();
        assert!(val.is_4byte_addr_enabled());
        assert_eq!(val.0, 0x0004_0000);
        val = val.disable_4byte_addr();
        assert!(!val.is_4byte_addr_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_read_mode_byte();
        assert!(val.is_read_mode_byte_enabled());
        assert_eq!(val.0, 0x0002_0000);
        val = val.disable_read_mode_byte();
        assert!(!val.is_read_mode_byte_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_read_cmd_bypass();
        assert!(val.is_read_cmd_bypass_enabled());
        assert_eq!(val.0, 0x0001_0000);
        val = val.disable_read_cmd_bypass();
        assert!(!val.is_read_cmd_bypass_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_read_cmd_bypass_code(0xFF);
        assert_eq!(val.read_cmd_bypass_code(), 0xFF);
        assert_eq!(val.0, 0x0000_FF00);

        val = ReadCmdMode(0x0).set_read_cmd_normal_code(0xFF);
        assert_eq!(val.read_cmd_normal_code(), 0xFF);
        assert_eq!(val.0, 0x0000_00FF);
    }

    test_should_panic!((
        test_rd_cmd_mode_set_dummy_byte_cnt_panic,
        ReadCmdMode(0x0).set_dummy_byte_cnt(0x10),
        "Dummy byte count out of range (expected 0..=15)"
    ));

    #[test]
    fn struct_wrap_len_functions() {
        let mut val = WrapLen(0x0).set_wrap64(0xFF);
        assert_eq!(val.wrap64(), 0xFF);
        assert_eq!(val.0, 0xFF00_0000);

        val = WrapLen(0x0).set_wrap32(0xFF);
        assert_eq!(val.wrap32(), 0xFF);
        assert_eq!(val.0, 0x00FF_0000);

        val = WrapLen(0x0).set_wrap16(0xFF);
        assert_eq!(val.wrap16(), 0xFF);
        assert_eq!(val.0, 0x0000_FF00);

        val = WrapLen(0x0).set_wrap08(0xFF);
        assert_eq!(val.wrap08(), 0xFF);
        assert_eq!(val.0, 0x0000_00FF);
    }

    #[test]
    fn struct_idma_tx_len_functions() {
        let val = IdmaTxLen(0x0).set_idma_tx_len(0xFFFFFF);
        assert_eq!(val.idma_tx_len(), 0xFFFFFF);
        assert_eq!(val.0, 0x00FF_FFFF);
    }

    test_should_panic!((
        test_set_idma_tx_len_panic,
        IdmaTxLen(0x0).set_idma_tx_len(0x1000000),
        "IDMA tx length out of range (expected 0..=0xFFFFFF)"
    ));

    #[test]
    fn struct_idma_rx_len_functions() {
        let val = IdmaRxLen(0x0).set_idma_rx_len(0xFFFFFF);
        assert_eq!(val.idma_rx_len(), 0xFFFFFF);
        assert_eq!(val.0, 0x00FF_FFFF);
    }

    test_should_panic!((
        test_set_idma_rx_len_panic,
        IdmaRxLen(0x0).set_idma_rx_len(0x1000000),
        "IDMA rx length out of range (expected 0..=0xFFFFFF)"
    ));

    #[test]
    fn struct_idma_burst_config_functions() {
        let mut val = IdmaBurstConfig(0x0).enable_rx_priority();
        assert!(val.is_rx_priority_enabled());
        assert_eq!(val.0, 0x0000_2000);
        val = val.disable_rx_priority();
        assert!(!val.is_rx_priority_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_auto_len();
        assert!(val.is_auto_len_enabled());
        assert_eq!(val.0, 0x0000_1000);
        val = val.disable_auto_len();
        assert!(!val.is_auto_len_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_rx_burst_len(IdmaRxBurstLen::Burst64);
        assert_eq!(val.rx_burst_len(), IdmaRxBurstLen::Burst64);
        assert_eq!(val.0, 0x0000_0C00);
        val = val.set_rx_burst_len(IdmaRxBurstLen::Burst32);
        assert_eq!(val.rx_burst_len(), IdmaRxBurstLen::Burst32);
        assert_eq!(val.0, 0x0000_0800);
        val = val.set_rx_burst_len(IdmaRxBurstLen::Burst16);
        assert_eq!(val.rx_burst_len(), IdmaRxBurstLen::Burst16);
        assert_eq!(val.0, 0x0000_0400);
        val = val.set_rx_burst_len(IdmaRxBurstLen::Burst4);
        assert_eq!(val.rx_burst_len(), IdmaRxBurstLen::Burst4);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_tx_burst_len(IdmaTxBurstLen::Burst32);
        assert_eq!(val.tx_burst_len(), IdmaTxBurstLen::Burst32);
        assert_eq!(val.0, 0x0000_0200);
        val = val.set_tx_burst_len(IdmaTxBurstLen::Burst16);
        assert_eq!(val.tx_burst_len(), IdmaTxBurstLen::Burst16);
        assert_eq!(val.0, 0x0000_0100);
        val = val.set_tx_burst_len(IdmaTxBurstLen::Burst4);
        assert_eq!(val.tx_burst_len(), IdmaTxBurstLen::Burst4);
        assert_eq!(val.0, 0x0000_0000);
    }
}
