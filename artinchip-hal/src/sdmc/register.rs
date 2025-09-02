//! SDMC register blocks and registers.

use volatile_register::{RO, RW, WO};

/// Secure Digital Host Controller registers.
#[repr(C)]
pub struct RegisterBlock {
    /// SDMC block count register (`SDMC_BLKCNT`).
    #[doc(alias = "SDMC_BLKCNT")]
    pub block_count: RW<BlockCount>,
    /// SDMC block size register (`SDMC_BLKSIZ`).
    #[doc(alias = "SDMC_BLKSIZ")]
    pub block_size: RW<BlockSize>,
    /// SDMC command argument register (`SDMC_CMDARG`).
    #[doc(alias = "SDMC_CMDARG")]
    pub cmd_arg: RW<u32>,
    /// SDMC command configuration register (`SDMC_CMD`).
    #[doc(alias = "SDMC_CMD")]
    pub cmd_config: RW<CommandConfig>,
    /// SDMC response register (`SDMC_RESP`).
    #[doc(alias = "SDMC_RESP")]
    pub resp: RO<u128>,
    /// SDMC transfer timeout control register (`SDMC_TTMC`).
    #[doc(alias = "SDMC_TTMC")]
    pub timeout: RW<TimeoutCtrl>,
    /// SDMC transferred card byte count register (`SDMC_TCBC`).
    #[doc(alias = "SDMC_TCBC")]
    pub transfer_card_byte_count: RO<u32>,
    /// SDMC transferred fifo byte count register (`SDMC_TFBC`).
    #[doc(alias = "SDMC_TFBC")]
    pub transfer_fifo_byte_count: RO<u32>,
    /// SDMC controller status register (`SDMC_CTRST`).
    #[doc(alias = "SDMC_CTRST")]
    pub ctrl_status: RO<CtrlStatus>,
    /// SDMC host control 1 register (`SDMC_HCTRL1`).
    #[doc(alias = "SDMC_HCTRL1")]
    pub host_ctrl1: RW<HostCtrl1>,
    /// SDMC clock control register (`SDMC_CLKCTRL`).
    #[doc(alias = "SDMC_CLKCTRL")]
    pub clk_ctrl: RW<ClockCtrl>,
    /// SDMC host control 2 register (`SDMC_HCTRL2`).
    #[doc(alias = "SDMC_HCTRL2")]
    pub host_ctrl2: RW<HostCtrl2>,
    /// SDMC interrupt enable register (`SDMC_INTEN`).
    #[doc(alias = "SDMC_INTEN")]
    pub int_enable: RW<IntEnable>,
    /// SDMC enabled interrupt status register (`SDMC_INTST`).
    #[doc(alias = "SDMC_INTST")]
    pub int_status: RO<IntStatus>,
    /// SDMC original interrupt status register (`SDMC_OINTST`).
    #[doc(alias = "SDMC_OINTST")]
    pub original_int_status: RW<OriginalIntStatus>,
    /// SDMC fifo config register (`SDMC_FIFOCFG`).
    #[doc(alias = "SDMC_FIFOCFG")]
    pub fifo_config: RW<FifoConfig>,
    _reserved0: [u8; 0x8],
    /// SDMC card detect register (`SDMC_CDET`).
    #[doc(alias = "SDMC_CDET")]
    pub card_detect: RW<CardDetect>,
    _reserved1: [u8; 0x28],
    /// SDMC peripheral bus configuration register (`SDMC_PBUSCFG`).
    #[doc(alias = "SDMC_PBUSCFG")]
    pub periph_bus_config: RW<PeripheralBusConfig>,
    /// SDMC inside dma resume capture descriptor register (`SDMC_IDMARCAP`).
    #[doc(alias = "SDMC_IDMARCAP")]
    pub idma_resume_capture_desc: WO<u32>,
    /// SDMC descriptor list start address (`SDMC_IDMASADDR`).
    #[doc(alias = "SDMC_IDMASADDR")]
    pub desc_list_addr: RW<u32>,
    /// SDMC inside dma controller status register (`SDMC_IDMAST`).
    #[doc(alias = "SDMC_IDMAST")]
    pub idma_status: RW<IdmaStatus>,
    /// SDMC inside dma controller interrupt enable register (`SDMC_IDMAINTEN`).
    #[doc(alias = "SDMC_IDMAINTEN")]
    pub idma_int_enable: RW<IdmaInterruptEnable>,
    /// SDMC current descriptor address register (`SDMC_IDMACDA`).
    #[doc(alias = "SDMC_IDMACDA")]
    pub current_desc_addr: RO<u32>,
    /// SDMC current buffer address register (`SDMC_IDMACBA`).
    #[doc(alias = "SDMC_IDMACBA")]
    pub current_buf_addr: RO<u32>,
    _reserved2: [u8; 0x64],
    /// SDMC card threshold control register (`SDMC_CTC`).
    #[doc(alias = "SDMC_CTC")]
    pub card_threshold_ctrl: RW<CardThresCtrl>,
    /// SDMC card delay chain and phase control register (`SDMC_DLYCTRL`).
    #[doc(alias = "SDMC_DLYCTRL")]
    pub card_delay_chain_phase_ctrl: RW<CardDelayCtrl>,
    /// SDMC emmc configuration register (`SDMC_EMCR`).
    #[doc(alias = "SDMC_EMCR")]
    pub emmc_config: RW<EmmcConfig>,
    _reserved3: [u8; 0xC],
    /// SDMC version id register (`SDMC_VERID`).
    #[doc(alias = "SDMC_VERID")]
    pub version_id: RO<u32>,
    _reserved4: [u8; 0xE4],
    /// SDMC data fifo register (`SDMC_FIFO`).
    #[doc(alias = "SDMC_FIFO")]
    pub fifo: RW<u32>,
}

/// SDMC block count register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BlockCount(u32);

impl BlockCount {
    const BLK_CNT: u32 = 0xFF;

    /// Set block count (`BLK_CNT`).
    #[doc(alias = "BLK_CNT")]
    #[inline]
    pub const fn set_block_count(self, value: u16) -> Self {
        Self((self.0 & !Self::BLK_CNT) | (Self::BLK_CNT & (value as u32)))
    }
    /// Get block count.
    #[inline]
    pub const fn block_count(self) -> u16 {
        (self.0 & Self::BLK_CNT) as u16
    }
}

/// SDMC block size register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct BlockSize(u32);

impl BlockSize {
    const BLK_SIZE: u32 = 0xFFFF;

    /// Set block size (`BLK_SIZE`).
    #[doc(alias = "BLK_SIZE")]
    #[inline]
    pub const fn set_block_size(self, value: u16) -> Self {
        Self((self.0 & !Self::BLK_SIZE) | (Self::BLK_SIZE & (value as u32)))
    }
    /// Get block size.
    #[inline]
    pub const fn block_size(self) -> u16 {
        (self.0 & Self::BLK_SIZE) as u16
    }
}

/// Command boot mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CmdBootMode {
    /// Execute the default boot operation.
    Default,
    /// Execute the alternative boot operation.
    Custom,
}

/// Command transfer mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CmdTransferMode {
    /// Data transfer command based on blocks.
    Block,
    /// Data transfer command based on bytes.
    Byte,
}

/// Command transfer direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CmdTransferDir {
    /// Data transfer direction is card to the host.
    Card2Host,
    /// Data transfer direction is host to the card.
    Host2Card,
}

/// Command response length.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CmdRespLen {
    /// Command response length is 48 bits.
    ShortResp,
    /// Command response length is 136 bits.
    LongResp,
}

/// SDMC command configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CommandConfig(u32);

impl CommandConfig {
    const START_CMD: u32 = 0x1 << 31;
    const USE_HOLD_REG: u32 = 0x1 << 29;
    const VOLT_SWITCH: u32 = 0x1 << 28;
    const BOOT_MODE: u32 = 0x1 << 27;
    const BOOT_DIS: u32 = 0x1 << 26;
    const BOOT_ACK_EXP: u32 = 0x1 << 25;
    const BOOT_EN: u32 = 0x1 << 24;
    const UPD_CLK_REG: u32 = 0x1 << 21;
    const SEND_INIT: u32 = 0x1 << 15;
    const STOP_ABORT_CMD: u32 = 0x1 << 14;
    const WAIT_PRVDATA_DONE: u32 = 0x1 << 13;
    const AUTO_STOP: u32 = 0x1 << 12;
    const TRANS_MODE: u32 = 0x1 << 11;
    const TRANS_DIR: u32 = 0x1 << 10;
    const DATA_EXP: u32 = 0x1 << 9;
    const CHECK_RESP_CRC: u32 = 0x1 << 8;
    const RESP_LEN: u32 = 0x1 << 7;
    const RESP_EXP: u32 = 0x1 << 6;
    const CMD_INDEX: u32 = 0x3F;

    /// Set start send command bit (`START_CMD`).
    ///
    /// - Once the command is acquired by the CIF, this bit is automatically reset to zero.
    /// - When this bit is set to 1, the Host cannot write any more commands to the register.
    /// - If an attempt is made to write a new command at this time, a hardware lock error interrupt is set to 1.
    /// - Once the command is issued and a response is received, the command completion interrupt is set to 1.
    #[doc(alias = "START_CMD")]
    #[inline]
    pub const fn set_start_cmd(self, start: bool) -> Self {
        if start {
            Self(self.0 | Self::START_CMD)
        } else {
            Self(self.0 & !Self::START_CMD)
        }
    }
    /// Get start send command bit.
    #[inline]
    pub const fn start_cmd(self) -> bool {
        (self.0 & Self::START_CMD) != 0
    }
    /// Enable hold register (`USE_HOLD_REG`).
    ///
    /// When enabled, commands and data are sent to the card through the hold register, with a hold time increase of 0.8 ns delay.
    #[doc(alias = "USE_HOLD_REG")]
    #[inline]
    pub const fn enable_hold_reg(self) -> Self {
        Self(self.0 | Self::USE_HOLD_REG)
    }
    /// Disable hold register.
    #[inline]
    pub const fn disable_hold_reg(self) -> Self {
        Self(self.0 & !Self::USE_HOLD_REG)
    }
    /// Check if hold register is enabled.
    #[inline]
    pub const fn is_hold_reg_enabled(self) -> bool {
        (self.0 & Self::USE_HOLD_REG) != 0
    }
    /// Enable voltage switch (`VOLT_SWITCH`).
    ///
    /// Must be enabled when using the command for switching the card voltage (CMD11).
    #[doc(alias = "VOLT_SWITCH")]
    #[inline]
    pub const fn enable_voltage_switch(self) -> Self {
        Self(self.0 | Self::VOLT_SWITCH)
    }
    /// Disable voltage switch.
    #[inline]
    pub const fn disable_voltage_switch(self) -> Self {
        Self(self.0 & !Self::VOLT_SWITCH)
    }
    /// Check if voltage switch is enabled.
    #[inline]
    pub const fn is_voltage_switch_enabled(self) -> bool {
        (self.0 & Self::VOLT_SWITCH) != 0
    }
    /// Set boot mode (`BOOT_MODE`).
    #[doc(alias = "BOOT_MODE")]
    #[inline]
    pub const fn set_boot_mode(self, mode: CmdBootMode) -> Self {
        Self((self.0 & !Self::BOOT_MODE) | (Self::BOOT_MODE & ((mode as u32) << 27)))
    }
    /// Get boot mode.
    #[inline]
    pub const fn boot_mode(self) -> CmdBootMode {
        match (self.0 & Self::BOOT_MODE) >> 27 {
            0 => CmdBootMode::Default,
            1 => CmdBootMode::Custom,
            _ => unreachable!(),
        }
    }
    /// Set boot mode disable bit (`BOOT_DIS`).
    ///
    /// - It cannot be set to 1 simultaneously with `BOOT_EN`.
    /// - When set to 1 at the same time as `START_CMD`, CIF will terminate the boot operation.
    #[doc(alias = "BOOT_DIS")]
    #[inline]
    pub const fn set_boot_mode_dis(self, disable: bool) -> Self {
        if disable {
            Self(self.0 | Self::BOOT_DIS)
        } else {
            Self(self.0 & !Self::BOOT_DIS)
        }
    }
    /// Get boot mode disable bit.
    #[inline]
    pub const fn boot_mode_dis(self) -> bool {
        (self.0 & Self::BOOT_DIS) != 0
    }
    /// Set boot acknowledge expected bit (`BOOT_ACK_EXP`).
    ///
    /// When the BOOT_EN bit is set to 1 simultaneously, the CIF should receive a boot operation response signal 0-1-0 from the card.
    #[doc(alias = "BOOT_ACK_EXP")]
    #[inline]
    pub const fn set_boot_ack_exp(self, expect: bool) -> Self {
        if expect {
            Self(self.0 | Self::BOOT_ACK_EXP)
        } else {
            Self(self.0 & !Self::BOOT_ACK_EXP)
        }
    }
    /// Get boot acknowledge expected bit.
    #[inline]
    pub const fn boot_ack_exp(self) -> bool {
        (self.0 & Self::BOOT_ACK_EXP) != 0
    }
    /// Set boot enable bit (`BOOT_EN`).
    ///
    /// - `BOOT_MODE` = 0 means that when executing the default boot mode, it must be set to 1.
    /// - It cannot be set to 1 simultaneously with the `BOOT_DIS` bit.
    /// - When set to 1 simultaneously with the `START_CMD` bit, the CIF begins to send the boot sequence, and the command line corresponding to the card is at a low level.
    #[doc(alias = "BOOT_EN")]
    #[inline]
    pub const fn set_boot_en(self, enable: bool) -> Self {
        if enable {
            Self(self.0 | Self::BOOT_EN)
        } else {
            Self(self.0 & !Self::BOOT_EN)
        }
    }
    /// Get boot enable bit.
    #[inline]
    pub const fn boot_en(self) -> bool {
        (self.0 & Self::BOOT_EN) != 0
    }
    /// Enable update clock only (`UPD_CLK_REG`).
    ///
    /// When enabled, no commands will be sent, only the value of the clock register is updated to the relevant clock fields.
    #[doc(alias = "UPD_CLK_REG")]
    #[inline]
    pub const fn enable_update_clk_reg(self) -> Self {
        Self(self.0 | Self::UPD_CLK_REG)
    }
    /// Disable update clock only.
    #[inline]
    pub const fn disable_update_clk_reg(self) -> Self {
        Self(self.0 & !Self::UPD_CLK_REG)
    }
    /// Check if update clock only is enabled.
    #[inline]
    pub const fn is_update_clk_reg_enabled(self) -> bool {
        (self.0 & Self::UPD_CLK_REG) != 0
    }
    /// Enable send initialization sequence (`SEND_INIT`).
    ///
    /// After power-on, 80 clock cycles must be sent before sending the first command to allow for the completion of the internal initialization of the Card. In Boot mode, this bit does not need to be set to 1.
    #[doc(alias = "SEND_INIT")]
    #[inline]
    pub const fn enable_send_init(self) -> Self {
        Self(self.0 | Self::SEND_INIT)
    }
    /// Disable send initialization sequence.
    #[inline]
    pub const fn disable_send_init(self) -> Self {
        Self(self.0 & !Self::SEND_INIT)
    }
    /// Check if send initialization sequence is enabled.
    #[inline]
    pub const fn is_send_init_enabled(self) -> bool {
        (self.0 & Self::SEND_INIT) != 0
    }
    /// Enable send stop or abort command (`STOP_ABORT_CMD`).
    ///
    /// When operating in open or predefined data transfer modes, the Host needs to send a stop or discard command to terminate the data transmission.
    /// This bit should be set to 1 to ensure that the command and data state machine of the CIF correctly return to an idle state.
    #[doc(alias = "STOP_ABORT_CMD")]
    #[inline]
    pub const fn enable_send_stop(self) -> Self {
        Self(self.0 | Self::STOP_ABORT_CMD)
    }
    /// Disable send stop or abort command.
    #[inline]
    pub const fn disable_send_stop(self) -> Self {
        Self(self.0 & !Self::STOP_ABORT_CMD)
    }
    /// Check if send stop or abort command is enabled.
    #[inline]
    pub const fn is_send_stop_enabled(self) -> bool {
        (self.0 & Self::STOP_ABORT_CMD) != 0
    }
    /// Enable wait previous data transfer done (`WAIT_PRVDATA_DONE`).
    ///
    /// Typically, during data transmission or when terminating the current transmission,
    /// if one wishes to inquire about the current status of the external device, this bit can be set to zero.
    #[doc(alias = "WAIT_PRVDATA_DONE")]
    #[inline]
    pub const fn enable_wait_prvdata_done(self) -> Self {
        Self(self.0 | Self::WAIT_PRVDATA_DONE)
    }
    /// Disable wait previous data transfer done.
    #[inline]
    pub const fn disable_wait_prvdata_done(self) -> Self {
        Self(self.0 & !Self::WAIT_PRVDATA_DONE)
    }
    /// Check if wait previous data transfer done is enabled.
    #[inline]
    pub const fn is_wait_prvdata_done_enabled(self) -> bool {
        (self.0 & Self::WAIT_PRVDATA_DONE) != 0
    }
    /// Enable auto send stop command (`AUTO_STOP`).
    #[doc(alias = "AUTO_STOP")]
    #[inline]
    pub const fn enable_auto_stop(self) -> Self {
        Self(self.0 | Self::AUTO_STOP)
    }
    /// Disable auto send stop command.
    #[inline]
    pub const fn disable_auto_stop(self) -> Self {
        Self(self.0 & !Self::AUTO_STOP)
    }
    /// Check if auto send stop command is enabled.
    #[inline]
    pub const fn is_auto_stop_enabled(self) -> bool {
        (self.0 & Self::AUTO_STOP) != 0
    }
    /// Set command transfer mode (`TRANS_MODE`).
    #[doc(alias = "TRANS_MODE")]
    #[inline]
    pub const fn set_transfer_mode(self, mode: CmdTransferMode) -> Self {
        Self(self.0 & !Self::TRANS_MODE | (Self::TRANS_MODE & ((mode as u32) << 11)))
    }
    /// Get command transfer mode.
    #[inline]
    pub const fn transfer_mode(self) -> CmdTransferMode {
        match (self.0 & Self::TRANS_MODE) >> 11 {
            0 => CmdTransferMode::Block,
            1 => CmdTransferMode::Byte,
            _ => unreachable!(),
        }
    }
    /// Set command transfer direction (`TRANS_DIR`).
    #[doc(alias = "TRANS_DIR")]
    #[inline]
    pub const fn set_transfer_dir(self, dir: CmdTransferDir) -> Self {
        Self(self.0 & !Self::TRANS_DIR | (Self::TRANS_DIR & ((dir as u32) << 10)))
    }
    /// Get command transfer direction.
    #[inline]
    pub const fn transfer_dir(self) -> CmdTransferDir {
        match (self.0 & Self::TRANS_DIR) >> 10 {
            0 => CmdTransferDir::Card2Host,
            1 => CmdTransferDir::Host2Card,
            _ => unreachable!(),
        }
    }
    /// Set data expected bit (`DATA_EXP`).
    #[doc(alias = "DATA_EXP")]
    #[inline]
    pub const fn set_data_expected(self, expected: bool) -> Self {
        if expected {
            Self(self.0 | Self::DATA_EXP)
        } else {
            Self(self.0 & !Self::DATA_EXP)
        }
    }
    /// Get data expected bit.
    #[inline]
    pub const fn data_expected(self) -> bool {
        (self.0 & Self::DATA_EXP) != 0
    }
    /// Enable check response crc (`CHECK_RESP_CRC`).
    #[doc(alias = "CHECK_RESP_CRC")]
    #[inline]
    pub const fn enable_check_resp_crc(self) -> Self {
        Self(self.0 | Self::CHECK_RESP_CRC)
    }
    /// Disable check response crc.
    #[inline]
    pub const fn disable_check_resp_crc(self) -> Self {
        Self(self.0 & !Self::CHECK_RESP_CRC)
    }
    /// Check if check response crc is enabled.
    #[inline]
    pub const fn is_check_resp_crc_enabled(self) -> bool {
        (self.0 & Self::CHECK_RESP_CRC) != 0
    }
    /// Set response length.
    #[inline]
    pub const fn set_resp_len(self, len: CmdRespLen) -> Self {
        Self(self.0 & !Self::RESP_LEN | (Self::RESP_LEN & ((len as u32) << 7)))
    }
    /// Get response length.
    #[inline]
    pub const fn resp_len(self) -> CmdRespLen {
        match (self.0 & Self::RESP_LEN) >> 7 {
            0 => CmdRespLen::ShortResp,
            1 => CmdRespLen::LongResp,
            _ => unreachable!(),
        }
    }
    /// Set response expected (`RESP_EXP`).
    #[doc(alias = "RESP_EXP")]
    #[inline]
    pub const fn set_resp_expected(self, expected: bool) -> Self {
        if expected {
            Self(self.0 | Self::RESP_EXP)
        } else {
            Self(self.0 & !Self::RESP_EXP)
        }
    }
    /// Get response expected.
    #[inline]
    pub const fn resp_expected(self) -> bool {
        (self.0 & Self::RESP_EXP) != 0
    }
    /// Set command index (`CMD_INDEX`).
    #[doc(alias = "CMD_INDEX")]
    #[inline]
    pub const fn set_cmd_index(self, index: u8) -> Self {
        assert!(index < 64, "Command index out of range (expected 0..=63)");
        Self(self.0 & !Self::CMD_INDEX | (Self::CMD_INDEX & (index as u32)))
    }
    /// Get command index.
    #[inline]
    pub const fn cmd_index(self) -> u8 {
        (self.0 & Self::CMD_INDEX) as u8
    }
}

/// SDMC transfer timeout control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TimeoutCtrl(u32);

impl TimeoutCtrl {
    const DATA_TMO: u32 = 0xFF_FFFF << 8;
    const RESP_TMO: u32 = 0xFF;

    /// Set data timeout period (`DATA_TMO`).
    ///
    /// - The timeout counter is activated only after the Card clock has halted.
    /// - Unit: clock cycles of the output clock.
    /// - If the timeout value approaches 100 ms, it is necessary to use software to configure the GTC timer for timeout detection.
    ///   In this case, the read data timeout interrupt should be masked (bit 9 of the `SDMC_INTEN` register should be cleared to 0).
    #[doc(alias = "DATA_TMO")]
    #[inline]
    pub const fn set_data_timeout(self, value: u32) -> Self {
        assert!(
            value < 0x1000000,
            "Data timeout out of range (expected 0..=0xFF_FFFF)"
        );
        Self((self.0 & !Self::DATA_TMO) | (Self::DATA_TMO & (value << 8)))
    }
    /// Get data timeout period.
    #[inline]
    pub const fn data_timeout(self) -> u32 {
        (self.0 & Self::DATA_TMO) >> 8
    }
    /// Set response timeout period (`RESP_TMO`).
    ///
    /// Unit: clock cycles of the output clock.
    #[doc(alias = "RESP_TMO")]
    #[inline]
    pub const fn set_resp_timeout(self, value: u8) -> Self {
        Self((self.0 & !Self::RESP_TMO) | (Self::RESP_TMO & (value as u32)))
    }
    /// Get response timeout period.
    #[inline]
    pub const fn resp_timeout(self) -> u8 {
        (self.0 & Self::RESP_TMO) as u8
    }
}

/// Command FSM state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CmdFsmState {
    /// Idle.
    Idle,
    /// Send initialization sequence.
    SendInitSeq,
    /// Tx command start.
    TransmitStart,
    /// Tx command data.
    TransmitTx,
    /// Tx command index and argument.
    TransmitCmdIdxAndArg,
    /// Tx crc7.
    TransmitCrc7,
    /// Tx end.
    TransmitEnd,
    /// Rx response start.
    ReceiveRespStart,
    /// Rx interrupt response.
    ReceiveIntResp,
    /// Rx response tx.
    ReceiveRespTx,
    /// Rx response command index.
    ReceiveRespCmdIdx,
    /// Rx response data.
    ReceiveRespData,
    /// Rx response crc7.
    ReceiveRespCrc7,
    /// Rx response end.
    ReceiveRespEnd,
    /// Command path wait ncc.
    CmdPathWaitNcc,
    /// Wait state.
    WaitState,
}

/// SDMC controller status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CtrlStatus(u32);

impl CtrlStatus {
    const DMA_REQ: u32 = 1 << 31;
    const DMA_ACK: u32 = 1 << 30;
    const FIFO_CNT: u32 = 0x1FFF << 17;
    const RESP_INDEX: u32 = 0x3F << 11;
    const DATA_MC_BUSY: u32 = 1 << 10;
    const DATA0_BUSY: u32 = 1 << 9;
    const DATA3_STATUS: u32 = 1 << 8;
    const CMD_STAT: u32 = 0xF << 4;
    const FIFO_FULL: u32 = 1 << 3;
    const FIFO_EMPTY: u32 = 1 << 2;
    const FIFO_TX_WMARK: u32 = 1 << 1;
    const FIFO_RX_WMARK: u32 = 1;

    /// Get dma request status (`DMA_REQ`).
    #[doc(alias = "DMA_REQ")]
    #[inline]
    pub const fn dma_request(self) -> bool {
        (self.0 & Self::DMA_REQ) != 0
    }
    /// Get dma acknowledge status (`DMA_ACK`).
    #[doc(alias = "DMA_ACK")]
    #[inline]
    pub const fn dma_acknowledge(self) -> bool {
        (self.0 & Self::DMA_ACK) != 0
    }
    /// Get fifo count (`FIFO_CNT`).
    #[doc(alias = "FIFO_CNT")]
    #[inline]
    pub const fn fifo_count(self) -> u16 {
        ((self.0 & Self::FIFO_CNT) >> 17) as u16
    }
    /// Get previous index (`RESP_INDEX`).
    #[doc(alias = "RESP_INDEX")]
    #[inline]
    pub const fn resp_index(self) -> u8 {
        ((self.0 & Self::RESP_INDEX) >> 11) as u8
    }
    /// Get data state machine busy status (`DATA_MC_BUSY`).
    #[doc(alias = "DATA_MC_BUSY")]
    #[inline]
    pub const fn data_mc_busy(self) -> bool {
        (self.0 & Self::DATA_MC_BUSY) != 0
    }
    /// Get data0 busy status (`DATA0_BUSY`).
    #[doc(alias = "DATA0_BUSY")]
    #[inline]
    pub const fn data0_busy(self) -> bool {
        (self.0 & Self::DATA0_BUSY) != 0
    }
    /// Get data3 card inserted status (`DATA3_STATUS`).
    #[doc(alias = "DATA3_STATUS")]
    #[inline]
    pub const fn data3_status(self) -> bool {
        (self.0 & Self::DATA3_STATUS) != 0
    }
    /// Get command fsm status (`CMD_STAT`).
    #[doc(alias = "CMD_STAT")]
    #[inline]
    pub const fn cmd_fsm_status(self) -> CmdFsmState {
        match (self.0 & Self::CMD_STAT) >> 4 {
            0 => CmdFsmState::Idle,
            1 => CmdFsmState::SendInitSeq,
            2 => CmdFsmState::TransmitStart,
            3 => CmdFsmState::TransmitTx,
            4 => CmdFsmState::TransmitCmdIdxAndArg,
            5 => CmdFsmState::TransmitCrc7,
            6 => CmdFsmState::TransmitEnd,
            7 => CmdFsmState::ReceiveRespStart,
            8 => CmdFsmState::ReceiveIntResp,
            9 => CmdFsmState::ReceiveRespTx,
            10 => CmdFsmState::ReceiveRespCmdIdx,
            11 => CmdFsmState::ReceiveRespData,
            12 => CmdFsmState::ReceiveRespCrc7,
            13 => CmdFsmState::ReceiveRespEnd,
            14 => CmdFsmState::CmdPathWaitNcc,
            15 => CmdFsmState::WaitState,
            _ => unreachable!(),
        }
    }
    /// Get fifo full status (`FIFO_FULL`).
    #[doc(alias = "FIFO_FULL")]
    #[inline]
    pub const fn fifo_full(self) -> bool {
        (self.0 & Self::FIFO_FULL) != 0
    }
    /// Get fifo empty status (`FIFO_EMPTY`).
    #[doc(alias = "FIFO_EMPTY")]
    #[inline]
    pub const fn fifo_empty(self) -> bool {
        (self.0 & Self::FIFO_EMPTY) != 0
    }
    /// Get fifo tx watermark (`FIFO_TX_WMARK`).
    #[doc(alias = "FIFO_TX_WMARK")]
    #[inline]
    pub const fn fifo_tx_wmark(self) -> bool {
        (self.0 & Self::FIFO_TX_WMARK) != 0
    }
    /// Get fifo rx watermark (`FIFO_RX_WMARK`).
    #[doc(alias = "FIFO_RX_WMARK")]
    #[inline]
    pub const fn fifo_rx_wmark(self) -> bool {
        (self.0 & Self::FIFO_RX_WMARK) != 0
    }
}

/// SDMC host control 1 register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct HostCtrl1(u32);

impl HostCtrl1 {
    const USE_IDMAC: u32 = 0x1 << 25;
    const OD_PUP_EN: u32 = 0x1 << 24;
    const ABORT_RDATA: u32 = 0x1 << 8;
    const SEND_IRQ_RESP: u32 = 0x1 << 7;
    const READ_WAIT: u32 = 0x1 << 6;
    const INT_EN: u32 = 0x1 << 4;
    const HW_RSTN: u32 = 0x1 << 3;
    const DMA_RESET: u32 = 0x1 << 2;
    const FIFO_RESET: u32 = 0x1 << 1;
    const CTRL_RESET: u32 = 0x1;

    /// Enable inside dma controller (`USE_IDMAC`).
    ///
    /// - When enabled, data is transferred using only internal DMA.
    /// - When disabled, data transmission is carried out through the AHB slave interface by the host.
    #[doc(alias = "USE_IDMAC")]
    #[inline]
    pub const fn enable_idmac(self) -> Self {
        Self(self.0 | Self::USE_IDMAC)
    }
    /// Disable inside dma controller.
    #[inline]
    pub const fn disable_idmac(self) -> Self {
        Self(self.0 & !Self::USE_IDMAC)
    }
    /// Check if inside dma controller is enabled.
    #[inline]
    pub const fn is_idmac_enabled(self) -> bool {
        (self.0 & Self::USE_IDMAC) != 0
    }
    /// Enable open drain pull up (`OD_PUP_EN`).
    ///
    /// Set the output of the command signal to open-drain mode, in which the command signal may be driven to a low level or a high impedance state, but must not be driven to a high level.
    #[doc(alias = "OD_PUP_EN")]
    #[inline]
    pub const fn enable_opendrain_pullup(self) -> Self {
        Self(self.0 | Self::OD_PUP_EN)
    }
    /// Disable open drain pull up.
    #[inline]
    pub const fn disable_opendrain_pullup(self) -> Self {
        Self(self.0 & !Self::OD_PUP_EN)
    }
    /// Check if open drain pull up is enabled.
    #[inline]
    pub const fn is_opendrain_pullup_enabled(self) -> bool {
        (self.0 & Self::OD_PUP_EN) != 0
    }
    /// Set abort read data bit (`ABORT_RDATA`).
    ///
    /// When reading data, after sending the pause command, the software continuously polls the SDIO device to determine whether it has entered the pause state.
    /// When the pause operation occurs, the software will set this bit 1 to reset the data state machine and wait for the next data transmission.
    /// Once the data state machine is reset to the idle state, this bit will be automatically cleared.
    #[doc(alias = "ABORT_RDATA")]
    #[inline]
    pub const fn set_abort_rdata(self, abort: bool) -> Self {
        if abort {
            Self(self.0 | Self::ABORT_RDATA)
        } else {
            Self(self.0 & !Self::ABORT_RDATA)
        }
    }
    /// Get abort read data bit.
    #[inline]
    pub const fn abort_rdata(self) -> bool {
        (self.0 & Self::ABORT_RDATA) != 0
    }
    /// Set send irq response bit (`SEND_IRQ_RESP`).
    ///
    /// - Once the response is sent, this bit is automatically reset to zero.
    /// - In order to wait for the MMC card interrupt, the Host sends the CMD40 command, and the controller waits for an interrupt response from the MMC card.
    ///   If the user wishes for the controller to exit the interrupt status waiting, they can set that bit to 1. At this point, the controller's command state machine will automatically send CMD40,
    ///   and after obtaining a response from the bus, it will return to the idle state.
    #[doc(alias = "SEND_IRQ_RESP")]
    #[inline]
    pub const fn set_send_irq_resp(self, send: bool) -> Self {
        if send {
            Self(self.0 | Self::SEND_IRQ_RESP)
        } else {
            Self(self.0 & !Self::SEND_IRQ_RESP)
        }
    }
    /// Get send irq response bit.
    #[inline]
    pub const fn send_irq_resp(self) -> bool {
        (self.0 & Self::SEND_IRQ_RESP) != 0
    }
    /// Set read wait bit (`READ_WAIT`).
    ///
    /// - Used for the SDIO card device read wait functionality.
    /// - If set this bit to 1, the controller temporarily pulls the Data[2] pin low to suspend data reading, allowing the sending of CMD52 and other I/O commands.
    #[doc(alias = "READ_WAIT")]
    #[inline]
    pub const fn set_read_wait(self, enable: bool) -> Self {
        if enable {
            Self(self.0 | Self::READ_WAIT)
        } else {
            Self(self.0 & !Self::READ_WAIT)
        }
    }
    /// Get read wait bit.
    #[inline]
    pub const fn read_wait(self) -> bool {
        (self.0 & Self::READ_WAIT) != 0
    }
    /// Enable global interrupt (`INT_EN`).
    #[doc(alias = "INT_EN")]
    #[inline]
    pub const fn enable_int(self) -> Self {
        Self(self.0 | Self::INT_EN)
    }
    /// Disable global interrupt.
    #[inline]
    pub const fn disable_int(self) -> Self {
        Self(self.0 & !Self::INT_EN)
    }
    /// Check if global interrupt is enabled.
    #[inline]
    pub const fn is_int_enabled(self) -> bool {
        (self.0 & Self::INT_EN) != 0
    }
    /// Set hardware reset n bit.
    ///
    /// When this bit is set to 0, the reset signal is valid.
    /// The device enters the pre-idle state and requires reinitialization.
    #[doc(alias = "HW_RSTN")]
    #[inline]
    pub const fn set_hw_rstn(self, rstn: bool) -> Self {
        if rstn {
            Self(self.0 | Self::HW_RSTN)
        } else {
            Self(self.0 & !Self::HW_RSTN)
        }
    }
    /// Get hardware reset n bit.
    #[inline]
    pub const fn hw_rstn(self) -> bool {
        (self.0 & Self::HW_RSTN) != 0
    }
    /// Set dma reset bit (`DMA_RESET`).
    #[doc(alias = "DMA_RESET")]
    #[inline]
    pub const fn set_dma_rst(self, rst: bool) -> Self {
        if rst {
            Self(self.0 | Self::DMA_RESET)
        } else {
            Self(self.0 & !Self::DMA_RESET)
        }
    }
    /// Get dma reset bit.
    #[inline]
    pub const fn dma_rst(self) -> bool {
        (self.0 & Self::DMA_RESET) != 0
    }

    /// Set fifo reset bit (`FIFO_RESET`).
    ///
    /// - Reset the FIFO data and FIFO pointer; they will be automatically cleared to zero after the reset is completed.
    /// - After this bit is cleared, the FIFO pointer is reset after 2 cycles of the system clock and 2 cycles of the card clock.
    #[doc(alias = "FIFO_RESET")]
    #[inline]
    pub const fn set_fifo_reset(self, rst: bool) -> Self {
        if rst {
            Self(self.0 | Self::FIFO_RESET)
        } else {
            Self(self.0 & !Self::FIFO_RESET)
        }
    }
    /// Get fifo reset bit.
    #[inline]
    pub const fn fifo_reset(self) -> bool {
        (self.0 & Self::FIFO_RESET) != 0
    }
    /// Set controller reset bit (`CTRL_RESET`).
    ///
    /// - The bit will be automatically cleared after two AHB clock cycles.
    /// - The reset module includes, but is not limited to:
    ///   - HIF/CIF interface
    ///   - CIF and state machine
    ///   - The `ABORT_RDATA`/`SEND_IRQ_RESP`/`READ_WAIT` bit fields in the control register
    ///   - The `START_CMD` bit field in the command register
    /// - This reset does not affect other registers, DMA interfaces, FIFOs, and host interrupts.
    #[doc(alias = "CTRL_RESET")]
    #[inline]
    pub const fn set_ctrl_reset(self, rst: bool) -> Self {
        if rst {
            Self(self.0 | Self::CTRL_RESET)
        } else {
            Self(self.0 & !Self::CTRL_RESET)
        }
    }
    /// Get controller reset bit.
    #[inline]
    pub const fn ctrl_reset(self) -> bool {
        (self.0 & Self::CTRL_RESET) != 0
    }
}

/// SDMC clock control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ClockCtrl(u32);

impl ClockCtrl {
    const CLK_LP_EN: u32 = 0x1 << 16;
    const CLK_DIV: u32 = 0xFF << 8;
    const CLK_EN: u32 = 0x1;

    /// Enable clock low power mode (`CLK_LP_EN`).
    #[doc(alias = "CLK_LP_EN")]
    #[inline]
    pub const fn enable_clk_lp(self) -> Self {
        Self(self.0 | Self::CLK_LP_EN)
    }
    /// Disable clock low power mode.
    #[inline]
    pub const fn disable_clk_lp(self) -> Self {
        Self(self.0 & !Self::CLK_LP_EN)
    }
    /// Check if clock low power mode is enabled.
    #[inline]
    pub const fn is_clk_lp_enabled(self) -> bool {
        (self.0 & Self::CLK_LP_EN) != 0
    }
    /// Set clock divider (`CLK_DIV`).
    #[doc(alias = "CLK_DIV")]
    #[inline]
    pub const fn set_clk_div(self, div: u8) -> Self {
        Self((self.0 & !Self::CLK_DIV) | (Self::CLK_DIV & ((div as u32) << 8)))
    }
    /// Get clock divider.
    #[inline]
    pub const fn clk_div(self) -> u8 {
        ((self.0 & Self::CLK_DIV) >> 8) as u8
    }
    /// Enable clock (`CLK_EN`).
    #[doc(alias = "CLK_EN")]
    #[inline]
    pub const fn enable_clk(self) -> Self {
        Self(self.0 | Self::CLK_EN)
    }
    /// Disable clock.
    #[inline]
    pub const fn disable_clk(self) -> Self {
        Self(self.0 & !Self::CLK_EN)
    }
    /// Check if clock is enabled.
    #[inline]
    pub const fn is_clk_enabled(self) -> bool {
        (self.0 & Self::CLK_EN) != 0
    }
}

/// Bus width.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BusWidth {
    /// 1-wire mode.
    OneWire,
    /// 4-wire mode.
    FourWire,
}

/// Timeout unit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TimeoutUnit {
    /// 1 times cclk_out.
    CclkOutX1,
    /// 256 times cclk_out.
    CclkOutX256,
}

/// Transfer mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TransferMode {
    /// Sdr mode.
    Sdr,
    /// Ddr mode.
    Ddr,
}

/// SDMC host control 2 register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct HostCtrl2(u32);

impl HostCtrl2 {
    const BUS_WIDTH: u32 = 0x3 << 28;
    const DTO_UNIT: u32 = 0x1 << 25;
    const RTO_UNIT: u32 = 0x1 << 24;
    const DDR_MODE: u32 = 0x1 << 16;

    /// Set bus width (`BUS_WIDTH`).
    #[doc(alias = "BUS_WIDTH")]
    #[inline]
    pub const fn set_bus_width(self, width: BusWidth) -> Self {
        Self((self.0 & !Self::BUS_WIDTH) | (Self::BUS_WIDTH & ((width as u32) << 28)))
    }
    /// Get bus width.
    #[inline]
    pub const fn bus_width(self) -> BusWidth {
        match (self.0 & Self::BUS_WIDTH) >> 28 {
            0 => BusWidth::OneWire,
            1 => BusWidth::FourWire,
            _ => unreachable!(),
        }
    }
    /// Set data timeout unit (`DTO_UNIT`).
    #[doc(alias = "DTO_UNIT")]
    #[inline]
    pub const fn set_dto_unit(self, unit: TimeoutUnit) -> Self {
        Self((self.0 & !Self::DTO_UNIT) | (Self::DTO_UNIT & ((unit as u32) << 25)))
    }
    /// Get data timeout unit.
    #[inline]
    pub const fn dto_unit(self) -> TimeoutUnit {
        match (self.0 & Self::DTO_UNIT) >> 25 {
            0 => TimeoutUnit::CclkOutX1,
            1 => TimeoutUnit::CclkOutX256,
            _ => unreachable!(),
        }
    }
    /// Set response timeout unit (`RTO_UNIT`).
    #[doc(alias = "RTO_UNIT")]
    #[inline]
    pub const fn set_rto_unit(self, unit: TimeoutUnit) -> Self {
        Self((self.0 & !Self::RTO_UNIT) | (Self::RTO_UNIT & ((unit as u32) << 24)))
    }
    /// Get response timeout unit.
    #[inline]
    pub const fn rto_unit(self) -> TimeoutUnit {
        match (self.0 & Self::RTO_UNIT) >> 24 {
            0 => TimeoutUnit::CclkOutX1,
            1 => TimeoutUnit::CclkOutX256,
            _ => unreachable!(),
        }
    }
    /// Set transfer mode (`DDR_MODE`).
    #[doc(alias = "DDR_MODE")]
    #[inline]
    pub const fn set_transfer_mode(self, mode: TransferMode) -> Self {
        Self((self.0 & !Self::DDR_MODE) | (Self::DDR_MODE & ((mode as u32) << 16)))
    }
    /// Get transfer mode.
    #[inline]
    pub const fn transfer_mode(self) -> TransferMode {
        match (self.0 & Self::DDR_MODE) >> 16 {
            0 => TransferMode::Sdr,
            1 => TransferMode::Ddr,
            _ => unreachable!(),
        }
    }
}

/// SDMC interrupt.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SdmcInterrupt {
    /// Card detect.
    CardDetect,
    /// Response error.
    RespError,
    /// Command send complete.
    CmdComplete,
    /// Data transfer done.
    DataDone,
    /// Transmit data fifo request.
    TxFifoReq,
    /// Receive data fifo request.
    RxFifoReq,
    /// Response CRC error.
    RespCrcError,
    /// Data CRC error.
    DataCrcError,
    /// Response timeout.
    RespTimeout,
    /// Read data timeout.
    ReadTimeout,
    /// Host data timeout or voltage switch interrupt.
    HostTimeoutOrVSwitch,
    /// FIFO underrun or overrun.
    FifoUndOverrun,
    /// Hardware lock error.
    HwLockError,
    /// Start bit error or busy clear.
    StartBitOrBusyClear,
    /// Auto command done.
    AutoCmdDone,
    /// End bit error or writing operation lacks CRC.
    EndBitOrWriteNoCrc,
}

/// SDMC interrupt enable register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IntEnable(u32);

impl IntEnable {
    const SDIO_INTEN: u32 = 0x1 << 16;
    const INT_EN: u32 = 0xFFFF;

    /// Enable sdio interrupt (`SDIO_INTEN`).
    #[doc(alias = "SDIO_INTEN")]
    #[inline]
    pub const fn enable_sdmc_interrupt(self) -> Self {
        Self(self.0 | Self::SDIO_INTEN)
    }
    /// Disable sdio interrupt.
    #[inline]
    pub const fn disable_sdmc_interrupt(self) -> Self {
        Self(self.0 & !Self::SDIO_INTEN)
    }
    /// Check if sdio interrupt is enabled.
    #[inline]
    pub const fn is_sdmc_interrupt_enabled(self) -> bool {
        (self.0 & Self::SDIO_INTEN) != 0
    }
    /// Enable selected interrupt (`INT_EN`).
    #[doc(alias = "INT_EN")]
    #[inline]
    pub const fn enable_interrupt(self, interrupt: SdmcInterrupt) -> Self {
        Self((self.0 & !Self::INT_EN) | (Self::INT_EN & (1 << (interrupt as u32))))
    }
    /// Disable selected interrupt.
    #[inline]
    pub const fn disable_interrupt(self, interrupt: SdmcInterrupt) -> Self {
        Self(self.0 & !(1 << (interrupt as u32)))
    }
    /// Check if selected interrupt is enabled.
    #[inline]
    pub const fn is_interrupt_enabled(self, interrupt: SdmcInterrupt) -> bool {
        (self.0 & (1 << (interrupt as u32))) != 0
    }
}

/// SDMC interrupt status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IntStatus(u32);

impl IntStatus {
    const SDIO_INT_STAT: u32 = 0x1 << 16;
    const _INT_STAT: u32 = 0xFFFF;

    /// Check if sdio interrupt is pending (`SDIO_INT_STAT`).
    #[doc(alias = "SDIO_INT_STAT")]
    #[inline]
    pub const fn is_sdio_interrupt_pending(self) -> bool {
        (self.0 & Self::SDIO_INT_STAT) != 0
    }
    /// Check if selected interrupt is pending.
    #[doc(alias = "INT_STAT")]
    #[inline]
    pub const fn is_interrupt_pending(self, interrupt: SdmcInterrupt) -> bool {
        (self.0 & (1 << (interrupt as u32))) != 0
    }
}

/// SDMC original interrupt status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OriginalIntStatus(u32);

impl OriginalIntStatus {
    const SDIO_INT_STAT: u32 = 0x1 << 16;
    const INT_STAT: u32 = 0xFFFF;

    /// Check if sdio interrupt is pending (`SDIO_INT_STAT`).
    #[doc(alias = "SDIO_INT_STAT")]
    #[inline]
    pub const fn is_sdio_interrupt_pending(self) -> bool {
        (self.0 & Self::SDIO_INT_STAT) != 0
    }
    /// Clear sdio interrupt.
    #[inline]
    pub const fn clear_sdio_interrupt(self) -> Self {
        Self(self.0 | Self::SDIO_INT_STAT)
    }
    /// Check if selected interrupt is pending.
    #[doc(alias = "INT_STAT")]
    #[inline]
    pub const fn is_interrupt_pending(self, interrupt: SdmcInterrupt) -> bool {
        (self.0 & (1 << (interrupt as u32))) != 0
    }
    /// Clear selected interrupt.
    #[inline]
    pub const fn clear_interrupt(self, interrupt: SdmcInterrupt) -> Self {
        Self((self.0 & !Self::INT_STAT) | (Self::INT_STAT & (1 << (interrupt as u32))))
    }
}

/// SDMC burst length.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SdmcBurstLength {
    /// 1 word.
    WordX1,
    /// 4 words.
    WordX4,
    /// 8 words.
    WordX8,
    /// 16 words.
    WordX16,
    /// 32 words.
    WordX32,
    /// 64 words.
    WordX64,
    /// 128 words.
    WordX128,
    /// 256 words.
    WordX256,
}

/// SDMC fifo config register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FifoConfig(u32);

impl FifoConfig {
    const DMA_BL: u32 = 0x7 << 28;
    const RX_WMARK: u32 = 0x7FF << 16;
    const TX_WMARK: u32 = 0xFFF;

    /// Set dma burst length (`DMA_BL`).
    ///
    /// The burst transmission length should be set to a common divisor of (`RX_WMARK` + 1) and (`FIFO_DEPTH` - `TX_WMARK`).
    /// For example, with `FIFO_DEPTH` = 16:
    /// - The allowed combinations of `DMA_BL` and `TX_WMARK` are as follows:
    ///     - `DMA_BL` = 1, `TX_WMARK` = 1 - 15
    ///     - `DMA_BL` = 4, `TX_WMARK` = 8
    ///     - `DMA_BL` = 4, `TX_WMARK` = 4
    ///     - `DMA_BL` = 4, `TX_WMARK` = 12
    ///     - `DMA_BL` = 8, `TX_WMARK` = 8
    ///     - `DMA_BL` = 8, `TX_WMARK` = 4
    /// - The allowed combinations of `DMA_BL` and `RX_WMARK` are as follows:
    ///     - `DMA_BL` = 1, `RX_WMARK` = 0 - 14
    ///     - `DMA_BL` = 4, `RX_WMARK` = 3
    ///     - `DMA_BL` = 4, `RX_WMARK` = 7
    ///     - `DMA_BL` = 4, `RX_WMARK` = 11
    ///     - `DMA_BL` = 8, `RX_WMARK` = 7
    ///
    /// Recommended values: `DMA_BL` = 8, `TX_WMARK` = 8, `RX_WMARK` = 7.
    #[doc(alias = "DMA_BL")]
    #[inline]
    pub const fn set_dma_burst_length(self, length: SdmcBurstLength) -> Self {
        Self((self.0 & !Self::DMA_BL) | (Self::DMA_BL & ((length as u32) << 28)))
    }
    /// Get dma burst length.
    #[inline]
    pub const fn dma_burst_length(self) -> SdmcBurstLength {
        match (self.0 & Self::DMA_BL) >> 28 {
            0 => SdmcBurstLength::WordX1,
            1 => SdmcBurstLength::WordX4,
            2 => SdmcBurstLength::WordX8,
            3 => SdmcBurstLength::WordX16,
            4 => SdmcBurstLength::WordX32,
            5 => SdmcBurstLength::WordX64,
            6 => SdmcBurstLength::WordX128,
            7 => SdmcBurstLength::WordX256,
            _ => unreachable!(),
        }
    }
    /// Set fifo rx watermark (`RX_WMARK`).
    ///
    /// When the amount of data in the FIFO exceeds the RX watermark, the DMA/FIFO request signal is asserted.
    /// If interrupts are enabled, an interrupt will be generated.
    /// - In non-DMA mode, when the `RxFifoReq` interrupt is enabled, an interrupt is generated instead of a DMA request.
    ///   If the remaining bytes at the end of a packet are fewer than the watermark, no interrupt will be generated.
    ///   When a data transfer complete interrupt is generated, the host must read the remaining bytes.
    /// - In DMA mode, upon receipt of the last packet, the DMA will issue a final request to read out any remaining FIFO data
    ///   even if the remaining data is below the watermark, and then generate a data transfer complete interrupt.
    ///
    /// Constraint: `RX_WMARK` ≤ `FIFO_DEPTH` - 2.
    ///
    /// Recommended value: (`FIFO_DEPTH`)/2 - 1.
    #[doc(alias = "RX_WMARK")]
    #[inline]
    pub const fn set_fifo_rx_watermark(self, watermark: u16) -> Self {
        assert!(
            watermark < 0x800,
            "FIFO RX watermark out of range (expected 0..=0x7FF)"
        );
        Self((self.0 & !Self::RX_WMARK) | (Self::RX_WMARK & ((watermark as u32) << 16)))
    }
    /// Get fifo rx watermark.
    #[inline]
    pub const fn fifo_rx_watermark(self) -> u16 {
        ((self.0 & Self::RX_WMARK) >> 16) as u16
    }
    /// Set fifo tx watermark (`TX_WMARK`).
    ///
    /// When the amount of data in the FIFO is less than or equal to the TX watermark, the DMA/FIFO request signal is asserted.
    /// If interrupts are enabled, an interrupt will be generated.
    /// - In non-DMA mode, when the `TxFifoReq` interrupt is enabled, an interrupt is generated instead of a DMA request.
    ///   After the last packet is sent, the host should fill the FIFO with the remaining data.
    /// - In DMA mode, after the last packet is completed, if the final transfer is smaller than the burst length, the DMA controller will perform an additional transfer until all data is transmitted.
    ///
    /// Constraint: `TX_WMARK` ≥ 1.
    ///
    /// Recommended value: (`FIFO_DEPTH`)/2.
    #[doc(alias = "TX_WMARK")]
    #[inline]
    pub const fn set_fifo_tx_watermark(self, watermark: u16) -> Self {
        assert!(
            watermark < 0x1000,
            "FIFO TX watermark out of range (expected 0..=0xFFF)"
        );
        Self((self.0 & !Self::TX_WMARK) | (Self::TX_WMARK & (watermark as u32)))
    }
    /// Get fifo tx watermark.
    #[inline]
    pub const fn fifo_tx_watermark(self) -> u16 {
        (self.0 & Self::TX_WMARK) as u16
    }
}

/// SDMC card detect register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CardDetect(u32);

impl CardDetect {
    const CDET_STAT: u32 = 0x1 << 24;
    const DEBC_CNT: u32 = 0xFF_FFFF;

    /// Get card detect status (`CDET_STAT`).
    #[doc(alias = "CDET_STAT")]
    #[inline]
    pub const fn card_detect_status(self) -> bool {
        (self.0 & Self::CDET_STAT) != 0
    }
    /// Set debounce clock count (`DEBC_CNT`).
    ///
    /// The typical debounce time is between 5 to 25 milliseconds.
    #[doc(alias = "DEBC_CNT")]
    #[inline]
    pub const fn set_debounce_count(self, count: u32) -> Self {
        assert!(
            count < 0x100_0000,
            "Debounce count out of range (expected 0..=0xFF_FFFF)"
        );
        Self((self.0 & !Self::DEBC_CNT) | (Self::DEBC_CNT & count))
    }
}

/// SDMC peripheral bus configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PeripheralBusConfig(u32);

impl PeripheralBusConfig {
    const PBL: u32 = 0x7 << 8;
    const DE: u32 = 0x1 << 7;
    const DSL: u32 = 0x1F << 2;
    const FB: u32 = 0x1 << 1;
    const SWR: u32 = 0x1;

    /// Get programed burst length (`PBL`).
    #[doc(alias = "PBL")]
    #[inline]
    pub const fn programmed_burst_length(self) -> SdmcBurstLength {
        match (self.0 & Self::PBL) >> 8 {
            0 => SdmcBurstLength::WordX1,
            1 => SdmcBurstLength::WordX4,
            2 => SdmcBurstLength::WordX8,
            3 => SdmcBurstLength::WordX16,
            4 => SdmcBurstLength::WordX32,
            5 => SdmcBurstLength::WordX64,
            6 => SdmcBurstLength::WordX128,
            7 => SdmcBurstLength::WordX256,
            _ => unreachable!(),
        }
    }
    /// Enable inside dma (`DE`).
    #[doc(alias = "DE")]
    #[inline]
    pub const fn enable_idma(self) -> Self {
        Self(self.0 | Self::DE)
    }
    /// Disable inside dma.
    #[inline]
    pub const fn disable_idma(self) -> Self {
        Self(self.0 & !Self::DE)
    }
    /// Check if inside dma is enabled.
    #[inline]
    pub const fn is_idma_enabled(self) -> bool {
        (self.0 & Self::DE) != 0
    }
    /// Set descriptor skip length (`DSL`).
    ///
    /// - Defines the length of data skipped between two non-sequential descriptions, measured in 32-bit units.
    /// - This is applicable only to the description of dual buffering structures.
    #[doc(alias = "DSL")]
    #[inline]
    pub const fn set_desc_skip_length(self, length: u8) -> Self {
        assert!(
            length < 32,
            "Descriptor skip length out of range (expected 0..=31)"
        );
        Self((self.0 & !Self::DSL) | (Self::DSL & ((length as u32) << 2)))
    }
    /// Get descriptor skip length.
    #[inline]
    pub const fn desc_skip_length(self) -> u32 {
        (self.0 & Self::DSL) >> 2
    }
    /// Enable fixed burst length (`FB`).
    ///
    /// - Enabled: AHB consistently uses either SINGLE, INCR4, INCR8, or INCR16 mode to execute burst transfers.
    /// - Disabled: AHB uses SINGLE and INCR modes to perform burst transfers.
    #[doc(alias = "FB")]
    #[inline]
    pub const fn enable_fixed_burst_length(self) -> Self {
        Self(self.0 | Self::FB)
    }
    /// Disable fixed burst length.
    #[inline]
    pub const fn disable_fixed_burst_length(self) -> Self {
        Self(self.0 & !Self::FB)
    }
    /// Check if fixed burst length is enabled.
    #[inline]
    pub const fn is_fixed_burst_length_enabled(self) -> bool {
        (self.0 & Self::FB) != 0
    }
    /// Set software reset bit (`SWR`).
    #[doc(alias = "SWR")]
    #[inline]
    pub const fn set_software_reset(self, rst: bool) -> Self {
        if rst {
            Self(self.0 | Self::SWR)
        } else {
            Self(self.0 & !Self::SWR)
        }
    }
    /// Get software reset bit.
    #[inline]
    pub const fn software_reset(self) -> bool {
        (self.0 & Self::SWR) != 0
    }
}

/// Inside dma FSM state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IdmaFsmState {
    /// Dma idle.
    DmaIdle,
    /// Dma suspend.
    DmaSuspend,
    /// Dmac read.
    DmacRead,
    /// Dmac check.
    DmacCheck,
    /// Dma read request wait.
    DmaReadReqWait,
    /// Dma write request wait.
    DmaWriteReqWait,
    /// Dma read.
    DmaRead,
    /// Dma write.
    DmaWrite,
    /// Descriptor close.
    DescClose,
}

/// Inside dma FSM state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IdmaFatalBusError {
    /// Host aborted during transmit.
    HostAbortTransmit,
    /// Host aborted during receive.
    HostAbortReceive,
}

/// SDMC inside dma controller status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IdmaStatus(u32);

impl IdmaStatus {
    const IDMA_FSM: u32 = 0xF << 13;
    const FBE_CODE: u32 = 0x7 << 10;
    const ABNORM_INT: u32 = 0x1 << 9;
    const NORM_INT: u32 = 0x1 << 8;
    const CARD_ERR_INT: u32 = 0x1 << 5;
    const DES_UNU_INT: u32 = 0x1 << 4;
    const FBE_INT: u32 = 0x1 << 2;
    const RX_DONE_INT: u32 = 0x1 << 1;
    const TX_DONE_INT: u32 = 0x1;

    /// Get IDMA FSM state (`IDMA_FSM`).
    #[doc(alias = "IDMA_FSM")]
    #[inline]
    pub const fn idma_fsm_state(self) -> IdmaFsmState {
        match (self.0 & Self::IDMA_FSM) >> 13 {
            0 => IdmaFsmState::DmaIdle,
            1 => IdmaFsmState::DmaSuspend,
            2 => IdmaFsmState::DmacRead,
            3 => IdmaFsmState::DmacCheck,
            4 => IdmaFsmState::DmaReadReqWait,
            5 => IdmaFsmState::DmaWriteReqWait,
            6 => IdmaFsmState::DmaRead,
            7 => IdmaFsmState::DmaWrite,
            8 => IdmaFsmState::DescClose,
            _ => unreachable!(),
        }
    }
    /// Get fatal bus error type (`FBE_CODE`).
    ///
    /// - Valid only when `IDMAST`[2] is set to 1.
    /// - This bit field does not generate an interrupt.
    #[doc(alias = "FBE_CODE")]
    #[inline]
    pub const fn fatal_bus_error(self) -> IdmaFatalBusError {
        match (self.0 & Self::FBE_CODE) >> 10 {
            0 => IdmaFatalBusError::HostAbortTransmit,
            1 => IdmaFatalBusError::HostAbortReceive,
            _ => unreachable!(),
        }
    }
    /// Check if abnormal interrupt is pending (`ABNORM_INT`).
    ///
    /// An abnormal interrupt is triggered when any of the following interrupts occurs:
    /// - `IDMAST`[2]: Fatal bus interrupt
    /// - `IDMAST`[4]: Descriptor unavailable interrupt
    #[doc(alias = "ABNORM_INT")]
    #[inline]
    pub const fn is_abnormal_interrupt_pending(self) -> bool {
        (self.0 & Self::ABNORM_INT) != 0
    }
    /// Clear abnormal interrupt status.
    #[inline]
    pub const fn clear_abnormal_interrupt(self) -> Self {
        Self(self.0 | Self::ABNORM_INT)
    }
    /// Check if normal interrupt is pending (`NORM_INT`).
    ///
    /// A normal interrupt is triggered upon the occurrence of any of the following interrupts:
    /// - `IDMAST`[0]: Transmission interrupt
    /// - `IDMAST`[1]: Reception interrupt
    #[doc(alias = "NORM_INT")]
    #[inline]
    pub const fn is_normal_interrupt_pending(self) -> bool {
        (self.0 & Self::NORM_INT) != 0
    }
    /// Clear normal interrupt status.
    #[inline]
    pub const fn clear_normal_interrupt(self) -> Self {
        Self(self.0 | Self::NORM_INT)
    }
    /// Check if card error interrupt is pending (`CARD_ERR_INT`).
    ///
    /// A card error interrupt is triggered when any of the following interrupts occurs:
    /// - `SdmcInterrupt::EndBitOrWriteNoCrc`.
    /// - `SdmcInterrupt::RespTimeout`.
    /// - `SdmcInterrupt::RespCrcError`.
    /// - `SdmcInterrupt::StartBitOrBusyClear`.
    /// - `SdmcInterrupt::ReadTimeout`.
    /// - `SdmcInterrupt::DataCrcError`.
    /// - `SdmcInterrupt::RespError`.
    #[doc(alias = "CARD_ERR_INT")]
    #[inline]
    pub const fn is_card_error_interrupt_pending(self) -> bool {
        (self.0 & Self::CARD_ERR_INT) != 0
    }
    /// Clear card error interrupt status.
    #[inline]
    pub const fn clear_card_error_interrupt(self) -> Self {
        Self(self.0 | Self::CARD_ERR_INT)
    }
    /// Check if descriptor unusable interrupt is pending (`DES_UNU_INT`).
    #[doc(alias = "DES_UNU_INT")]
    #[inline]
    pub const fn is_desc_unusable_interrupt_pending(self) -> bool {
        (self.0 & Self::DES_UNU_INT) != 0
    }
    /// Clear descriptor unusable interrupt status.
    #[inline]
    pub const fn clear_desc_unusable_interrupt(self) -> Self {
        Self(self.0 | Self::DES_UNU_INT)
    }
    /// Check if fatal bus error interrupt is pending (`FBE_INT`).
    #[doc(alias = "FBE_INT")]
    #[inline]
    pub const fn is_fatal_bus_error_interrupt_pending(self) -> bool {
        (self.0 & Self::FBE_INT) != 0
    }
    /// Clear fatal bus error interrupt status.
    #[inline]
    pub const fn clear_fatal_bus_error_interrupt(self) -> Self {
        Self(self.0 | Self::FBE_INT)
    }
    /// Check if receive done interrupt is pending (`RX_DONE_INT`).
    #[doc(alias = "RX_DONE_INT")]
    #[inline]
    pub const fn is_rx_done_interrupt_pending(self) -> bool {
        (self.0 & Self::RX_DONE_INT) != 0
    }
    /// Clear receive done interrupt status.
    #[inline]
    pub const fn clear_rx_done_interrupt(self) -> Self {
        Self(self.0 | Self::RX_DONE_INT)
    }
    /// Check if transmit done interrupt is pending (`TX_DONE_INT`).
    #[doc(alias = "TX_DONE_INT")]
    #[inline]
    pub const fn is_tx_done_interrupt_pending(self) -> bool {
        (self.0 & Self::TX_DONE_INT) != 0
    }
    /// Clear transmit done interrupt status.
    #[inline]
    pub const fn clear_tx_done_interrupt(self) -> Self {
        Self(self.0 | Self::TX_DONE_INT)
    }
}

/// SDMC inside dma controller interrupt enable register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IdmaInterruptEnable(u32);

impl IdmaInterruptEnable {
    const ABNORM_INTEN: u32 = 0x1 << 9;
    const NORM_INTEN: u32 = 0x1 << 8;
    const CARD_ERR_INTEN: u32 = 0x1 << 5;
    const DES_UNU_INTEN: u32 = 0x1 << 4;
    const FBE_INTEN: u32 = 0x1 << 2;
    const RX_DONE_INTEN: u32 = 0x1 << 1;
    const TX_DONE_INTEN: u32 = 0x1;

    /// Enable abnormal interrupt (`ABNORM_INTEN`).
    ///
    /// An abnormal interrupt is triggered when any of the following interrupts occurs:
    /// - `IDMAST`[2]: Fatal bus interrupt
    /// - `IDMAST`[4]: Descriptor unavailable interrupt
    #[doc(alias = "ABNORM_INTEN")]
    #[inline]
    pub const fn enable_abnormal_interrupt(self) -> Self {
        Self(self.0 | Self::ABNORM_INTEN)
    }
    /// Disable abnormal interrupt.
    #[inline]
    pub const fn disable_abnormal_interrupt(self) -> Self {
        Self(self.0 & !Self::ABNORM_INTEN)
    }
    /// Check if abnormal interrupt is enabled.
    #[inline]
    pub const fn is_abnormal_interrupt_enabled(self) -> bool {
        (self.0 & Self::ABNORM_INTEN) != 0
    }
    /// Enable normal interrupt (`NORM_INTEN`).
    ///
    /// A normal interrupt is triggered upon the occurrence of any of the following interrupts:
    /// - `IDMAST`[0]: Transmission interrupt
    /// - `IDMAST`[1]: Reception interrupt
    #[doc(alias = "NORM_INTEN")]
    #[inline]
    pub const fn enable_normal_interrupt(self) -> Self {
        Self(self.0 | Self::NORM_INTEN)
    }
    /// Disable normal interrupt.
    #[inline]
    pub const fn disable_normal_interrupt(self) -> Self {
        Self(self.0 & !Self::NORM_INTEN)
    }
    /// Check if normal interrupt is enabled.
    #[inline]
    pub const fn is_normal_interrupt_enabled(self) -> bool {
        (self.0 & Self::NORM_INTEN) != 0
    }
    /// Enable card error interrupt (`CARD_ERR_INTEN`).
    ///
    /// A card error interrupt is triggered when any of the following interrupts occurs:
    /// - `SdmcInterrupt::EndBitOrWriteNoCrc`.
    /// - `SdmcInterrupt::RespTimeout`.
    /// - `SdmcInterrupt::RespCrcError`.
    /// - `SdmcInterrupt::StartBitOrBusyClear`.
    /// - `SdmcInterrupt::ReadTimeout`.
    /// - `SdmcInterrupt::DataCrcError`.
    /// - `SdmcInterrupt::RespError`.
    #[doc(alias = "CARD_ERR_INTEN")]
    #[inline]
    pub const fn enable_card_error_interrupt(self) -> Self {
        Self(self.0 | Self::CARD_ERR_INTEN)
    }
    /// Disable card error interrupt.
    #[inline]
    pub const fn disable_card_error_interrupt(self) -> Self {
        Self(self.0 & !Self::CARD_ERR_INTEN)
    }
    /// Check if card error interrupt is enabled.
    #[inline]
    pub const fn is_card_error_interrupt_enabled(self) -> bool {
        (self.0 & Self::CARD_ERR_INTEN) != 0
    }
    /// Enable descriptor unusable interrupt (`DES_UNU_INTEN`).
    #[doc(alias = "DES_UNU_INTEN")]
    #[inline]
    pub const fn enable_desc_unusable_interrupt(self) -> Self {
        Self(self.0 | Self::DES_UNU_INTEN)
    }
    /// Disable descriptor unusable interrupt.
    #[inline]
    pub const fn disable_desc_unusable_interrupt(self) -> Self {
        Self(self.0 & !Self::DES_UNU_INTEN)
    }
    /// Check if descriptor unusable interrupt is enabled.
    #[inline]
    pub const fn is_desc_unusable_interrupt_enabled(self) -> bool {
        (self.0 & Self::DES_UNU_INTEN) != 0
    }
    /// Enable fatal bus error interrupt (`FBE_INTEN`).
    #[doc(alias = "FBE_INTEN")]
    #[inline]
    pub const fn enable_fatal_bus_error_interrupt(self) -> Self {
        Self(self.0 | Self::FBE_INTEN)
    }
    /// Disable fatal bus error interrupt.
    #[inline]
    pub const fn disable_fatal_bus_error_interrupt(self) -> Self {
        Self(self.0 & !Self::FBE_INTEN)
    }
    /// Check if fatal bus error interrupt is enabled.
    #[inline]
    pub const fn is_fatal_bus_error_interrupt_enabled(self) -> bool {
        (self.0 & Self::FBE_INTEN) != 0
    }
    /// Enable receive done interrupt (`RX_DONE_INTEN`).
    #[doc(alias = "RX_DONE_INTEN")]
    #[inline]
    pub const fn enable_rx_done_interrupt(self) -> Self {
        Self(self.0 | Self::RX_DONE_INTEN)
    }
    /// Disable receive done interrupt.
    #[inline]
    pub const fn disable_rx_done_interrupt(self) -> Self {
        Self(self.0 & !Self::RX_DONE_INTEN)
    }
    /// Check if receive done interrupt is enabled.
    #[inline]
    pub const fn is_rx_done_interrupt_enabled(self) -> bool {
        (self.0 & Self::RX_DONE_INTEN) != 0
    }
    /// Enable transmit done interrupt (`TX_DONE_INTEN`).
    #[doc(alias = "TX_DONE_INTEN")]
    #[inline]
    pub const fn enable_tx_done_interrupt(self) -> Self {
        Self(self.0 | Self::TX_DONE_INTEN)
    }
    /// Disable transmit done interrupt.
    #[inline]
    pub const fn disable_tx_done_interrupt(self) -> Self {
        Self(self.0 & !Self::TX_DONE_INTEN)
    }
    /// Check if transmit done interrupt is enabled.
    #[inline]
    pub const fn is_tx_done_interrupt_enabled(self) -> bool {
        (self.0 & Self::TX_DONE_INTEN) != 0
    }
}

/// SDMC card threshold control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CardThresCtrl(u32);

impl CardThresCtrl {
    const CTH_SIZE: u32 = 0x3FF << 16;
    const CDWTH_EN: u32 = 0x1 << 2;
    const BC_INTEN: u32 = 0x1 << 1;
    const CDRTH_EN: u32 = 0x1;

    /// Set card threshold size (CTH_SIZE).
    #[doc(alias = "CTH_SIZE")]
    #[inline]
    pub const fn set_card_threshold_size(self, size: u16) -> Self {
        assert!(
            size < 1024,
            "Card threshold size out of range (expected 0..=1023)"
        );
        Self((self.0 & !Self::CTH_SIZE) | (Self::CTH_SIZE & ((size as u32) << 16)))
    }
    /// Get card threshold size.
    #[inline]
    pub const fn card_threshold_size(self) -> u16 {
        ((self.0 & Self::CTH_SIZE) >> 16) as u16
    }
    /// Enable card write threshold (`CDWTH_EN`).
    #[doc(alias = "CDWTH_EN")]
    #[inline]
    pub const fn enable_card_write_threshold(self) -> Self {
        Self(self.0 | Self::CDWTH_EN)
    }
    /// Disable card write threshold.
    #[inline]
    pub const fn disable_card_write_threshold(self) -> Self {
        Self(self.0 & !Self::CDWTH_EN)
    }
    /// Check if card write threshold is enabled.
    #[inline]
    pub const fn is_card_write_threshold_enabled(self) -> bool {
        (self.0 & Self::CDWTH_EN) != 0
    }
    /// Enable card busy clear interrupt (`BC_INTEN`).
    #[doc(alias = "BC_INTEN")]
    #[inline]
    pub const fn enable_card_busy_clear_interrupt(self) -> Self {
        Self(self.0 | Self::BC_INTEN)
    }
    /// Disable card busy clear interrupt.
    #[inline]
    pub const fn disable_card_busy_clear_interrupt(self) -> Self {
        Self(self.0 & !Self::BC_INTEN)
    }
    /// Check if card busy clear interrupt is enabled.
    #[inline]
    pub const fn is_card_busy_clear_interrupt_enabled(self) -> bool {
        (self.0 & Self::BC_INTEN) != 0
    }
    /// Enable card read threshold (`CDRTH_EN`).
    #[doc(alias = "CDRTH_EN")]
    #[inline]
    pub const fn enable_card_read_threshold(self) -> Self {
        Self(self.0 | Self::CDRTH_EN)
    }
    /// Disable card read threshold.
    #[inline]
    pub const fn disable_card_read_threshold(self) -> Self {
        Self(self.0 & !Self::CDRTH_EN)
    }
    /// Check if card read threshold is enabled.
    #[inline]
    pub const fn is_card_read_threshold_enabled(self) -> bool {
        (self.0 & Self::CDRTH_EN) != 0
    }
}

/// SDMC external clock multiplex.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ExtClkMux {
    /// Input clock divided by 4 (DS, HS-SDR, HS-DDR).
    ClkInDiv4,
    /// Input clock divided by 2 (HS, HS-SDR, HS-DDR).
    ClkInDiv2,
    /// Input clock not divided (HS-DDR).
    ClkInDiv1,
}

/// SDMC clock phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ClockPhase {
    /// 0 degree.
    Deg0,
    /// 90 degrees.
    Deg90,
    /// 180 degrees.
    Deg180,
    /// 270 degrees.
    Deg270,
}

/// SDMC card delay chain and phase control register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CardDelayCtrl(u32);

impl CardDelayCtrl {
    const EXT_CLK_MUX: u32 = 0x3 << 30;
    const CLK_DRV_PHASE: u32 = 0x3 << 28;
    const CLK_DRV_DELAY: u32 = 0x1F << 23;
    const CLK_SMPL_PHASE: u32 = 0x3 << 21;
    const CLK_SMPL_DELAY: u32 = 0x1F << 16;
    const SAMP_SWAP: u32 = 0x1 << 8;
    const SAMP_SEQ_EN: u32 = 0x1 << 6;
    const EVEN_SHIFT: u32 = 0x3 << 4;
    const SHIFT_EN: u32 = 0x3;

    /// Set external clock multiplex (`EXT_CLK_MUX`).
    #[doc(alias = "EXT_CLK_MUX")]
    #[inline]
    pub const fn set_ext_clk_mux(self, mux: ExtClkMux) -> Self {
        Self((self.0 & !Self::EXT_CLK_MUX) | (Self::EXT_CLK_MUX & ((mux as u32) << 30)))
    }
    /// Get external clock multiplex.
    #[inline]
    pub const fn ext_clk_mux(self) -> ExtClkMux {
        match (self.0 & Self::EXT_CLK_MUX) >> 30 {
            0 => ExtClkMux::ClkInDiv4,
            1 => ExtClkMux::ClkInDiv2,
            2 => ExtClkMux::ClkInDiv1,
            _ => unreachable!(),
        }
    }
    /// Set driver clock phase (`CLK_DRV_PHASE`).
    #[doc(alias = "CLK_DRV_PHASE")]
    #[inline]
    pub const fn set_drv_clk_phase(self, phase: ClockPhase) -> Self {
        Self((self.0 & !Self::CLK_DRV_PHASE) | (Self::CLK_DRV_PHASE & ((phase as u32) << 28)))
    }
    /// Get driver clock phase.
    #[inline]
    pub const fn drv_clk_phase(self) -> ClockPhase {
        match (self.0 & Self::CLK_DRV_PHASE) >> 28 {
            0 => ClockPhase::Deg0,
            1 => ClockPhase::Deg90,
            2 => ClockPhase::Deg180,
            3 => ClockPhase::Deg270,
            _ => unreachable!(),
        }
    }
    /// Set driver clock delay (`CLK_DRV_DELAY`).
    #[doc(alias = "CLK_DRV_DELAY")]
    #[inline]
    pub const fn set_drv_clk_delay(self, delay: u16) -> Self {
        assert!(
            delay < 32,
            "Driver clock delay out of range (expected 0..=31)"
        );
        Self((self.0 & !Self::CLK_DRV_DELAY) | (Self::CLK_DRV_DELAY & ((delay as u32) << 23)))
    }
    /// Get driver clock delay.
    #[inline]
    pub const fn drv_clk_delay(self) -> u16 {
        ((self.0 & Self::CLK_DRV_DELAY) >> 23) as u16
    }
    /// Set sample clock phase (`CLK_SMPL_PHASE`).
    #[doc(alias = "CLK_SMPL_PHASE")]
    #[inline]
    pub const fn set_samp_clk_phase(self, phase: ClockPhase) -> Self {
        Self((self.0 & !Self::CLK_SMPL_PHASE) | (Self::CLK_SMPL_PHASE & ((phase as u32) << 21)))
    }
    /// Get sample clock phase.
    #[inline]
    pub const fn samp_clk_phase(self) -> ClockPhase {
        match (self.0 & Self::CLK_SMPL_PHASE) >> 21 {
            0 => ClockPhase::Deg0,
            1 => ClockPhase::Deg90,
            2 => ClockPhase::Deg180,
            3 => ClockPhase::Deg270,
            _ => unreachable!(),
        }
    }
    /// Set sample clock delay (`CLK_SMPL_DELAY`).
    #[doc(alias = "CLK_SMPL_DELAY")]
    #[inline]
    pub const fn set_samp_clk_delay(self, delay: u16) -> Self {
        assert!(
            delay < 32,
            "Sample clock delay out of range (expected 0..=31)"
        );
        Self((self.0 & !Self::CLK_SMPL_DELAY) | (Self::CLK_SMPL_DELAY & ((delay as u32) << 16)))
    }
    /// Get sample clock delay.
    #[inline]
    pub const fn samp_clk_delay(self) -> u16 {
        ((self.0 & Self::CLK_SMPL_DELAY) >> 16) as u16
    }
    /// Set ddr sample clock edge swap bit (`SAMP_SWAP`).
    ///
    /// By default, odd edge data is sampled first.
    #[doc(alias = "SAMP_SWAP")]
    #[inline]
    pub const fn set_ddr_samp_swap(self, swap: bool) -> Self {
        if swap {
            Self(self.0 | Self::SAMP_SWAP)
        } else {
            Self(self.0 & !Self::SAMP_SWAP)
        }
    }
    /// Get ddr sample clock edge swap bit.
    #[inline]
    pub const fn ddr_samp_swap(self) -> bool {
        (self.0 & Self::SAMP_SWAP) != 0
    }
    /// Enable ddr sample clock edge swap (`SAMP_SEQ_EN`).
    #[inline]
    pub const fn enable_ddr_samp_swap(self) -> Self {
        Self(self.0 | Self::SAMP_SEQ_EN)
    }
    /// Disable ddr sample clock edge swap.
    #[inline]
    pub const fn disable_ddr_samp_swap(self) -> Self {
        Self(self.0 & !Self::SAMP_SEQ_EN)
    }
    /// Check if ddr sample clock edge swap is enabled.
    #[inline]
    pub const fn is_ddr_samp_swap_enabled(self) -> bool {
        (self.0 & Self::SAMP_SEQ_EN) != 0
    }
    /// Set even data clock phase (`EVEN_SHIFT`).
    #[doc(alias = "EVEN_SHIFT")]
    #[inline]
    pub const fn set_ddr_samp_even_phase(self, phase: ClockPhase) -> Self {
        Self((self.0 & !Self::EVEN_SHIFT) | (Self::EVEN_SHIFT & ((phase as u32) << 4)))
    }
    /// Get even data clock phase.
    #[inline]
    pub const fn ddr_samp_even_phase(self) -> ClockPhase {
        match (self.0 & Self::EVEN_SHIFT) >> 4 {
            0 => ClockPhase::Deg0,
            1 => ClockPhase::Deg90,
            2 => ClockPhase::Deg180,
            3 => ClockPhase::Deg270,
            _ => unreachable!(),
        }
    }
    /// Set clock phase (`SHIFT_EN`).
    #[doc(alias = "SHIFT_EN")]
    #[inline]
    pub const fn set_clk_phase(self, phase: ClockPhase) -> Self {
        Self((self.0 & !Self::SHIFT_EN) | (Self::SHIFT_EN & (phase as u32)))
    }
    /// Get clock phase.
    #[inline]
    pub const fn clk_phase(self) -> ClockPhase {
        match self.0 & Self::SHIFT_EN {
            0 => ClockPhase::Deg0,
            1 => ClockPhase::Deg90,
            2 => ClockPhase::Deg180,
            3 => ClockPhase::Deg270,
            _ => unreachable!(),
        }
    }
}

/// SDMC emmc configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct EmmcConfig(u32);

impl EmmcConfig {
    const HALF_START_BIT: u32 = 0x1;

    /// Set half start bit (`HALF_START`).
    ///
    /// - 0: A complete clock cycle
    /// - 1: Less than a complete clock cycle
    /// - For eMMC version 4.5 and above, this bit is set to 1.
    /// - For SD card applications, this bit is cleared to zero.
    /// - In DDR mode, this bit field will be ignored.
    #[doc(alias = "HALF_START")]
    #[inline]
    pub const fn set_half_start(self, enable: bool) -> Self {
        if enable {
            Self(self.0 | Self::HALF_START_BIT)
        } else {
            Self(self.0 & !Self::HALF_START_BIT)
        }
    }
    /// Get half start bit.
    #[inline]
    pub const fn half_start(self) -> bool {
        (self.0 & Self::HALF_START_BIT) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BlockCount, BlockSize, BusWidth, CardDelayCtrl, CardDetect, CardThresCtrl, ClockCtrl,
        ClockPhase, CmdBootMode, CmdFsmState, CmdRespLen, CmdTransferDir, CmdTransferMode,
        CommandConfig, CtrlStatus, EmmcConfig, ExtClkMux, FifoConfig, HostCtrl1, HostCtrl2,
        IdmaFatalBusError, IdmaFsmState, IdmaInterruptEnable, IdmaStatus, IntEnable, IntStatus,
        OriginalIntStatus, PeripheralBusConfig, RegisterBlock, SdmcBurstLength, SdmcInterrupt,
        TimeoutCtrl, TimeoutUnit, TransferMode,
    };
    use crate::test_should_panic;
    use core::mem::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, block_count), 0x0);
        assert_eq!(offset_of!(RegisterBlock, block_size), 0x4);
        assert_eq!(offset_of!(RegisterBlock, cmd_arg), 0x8);
        assert_eq!(offset_of!(RegisterBlock, cmd_config), 0xC);
        assert_eq!(offset_of!(RegisterBlock, resp), 0x10);
        assert_eq!(offset_of!(RegisterBlock, timeout), 0x20);
        assert_eq!(offset_of!(RegisterBlock, transfer_card_byte_count), 0x24);
        assert_eq!(offset_of!(RegisterBlock, transfer_fifo_byte_count), 0x28);
        assert_eq!(offset_of!(RegisterBlock, ctrl_status), 0x2C);
        assert_eq!(offset_of!(RegisterBlock, host_ctrl1), 0x30);
        assert_eq!(offset_of!(RegisterBlock, clk_ctrl), 0x34);
        assert_eq!(offset_of!(RegisterBlock, host_ctrl2), 0x38);
        assert_eq!(offset_of!(RegisterBlock, int_enable), 0x3C);
        assert_eq!(offset_of!(RegisterBlock, int_status), 0x40);
        assert_eq!(offset_of!(RegisterBlock, original_int_status), 0x44);
        assert_eq!(offset_of!(RegisterBlock, fifo_config), 0x48);
        assert_eq!(offset_of!(RegisterBlock, card_detect), 0x54);
        assert_eq!(offset_of!(RegisterBlock, periph_bus_config), 0x80);
        assert_eq!(offset_of!(RegisterBlock, idma_resume_capture_desc), 0x84);
        assert_eq!(offset_of!(RegisterBlock, desc_list_addr), 0x88);
        assert_eq!(offset_of!(RegisterBlock, idma_status), 0x8C);
        assert_eq!(offset_of!(RegisterBlock, idma_int_enable), 0x90);
        assert_eq!(offset_of!(RegisterBlock, current_desc_addr), 0x94);
        assert_eq!(offset_of!(RegisterBlock, current_buf_addr), 0x98);
        assert_eq!(offset_of!(RegisterBlock, card_threshold_ctrl), 0x100);
        assert_eq!(
            offset_of!(RegisterBlock, card_delay_chain_phase_ctrl),
            0x104
        );
        assert_eq!(offset_of!(RegisterBlock, emmc_config), 0x108);
        assert_eq!(offset_of!(RegisterBlock, version_id), 0x118);
        assert_eq!(offset_of!(RegisterBlock, fifo), 0x200);
    }

    #[test]
    fn struct_block_count_functions() {
        let val = BlockCount(0).set_block_count(0xFF);
        assert_eq!(val.block_count(), 0xFF);
        assert_eq!(val.0, 0x0000_00FF);
    }

    #[test]
    fn struct_block_size_functions() {
        let val = BlockSize(0).set_block_size(0xFFFF);
        assert_eq!(val.block_size(), 0xFFFF);
        assert_eq!(val.0, 0x0000_FFFF);
    }

    #[test]
    fn struct_command_config_functions() {
        let mut config = CommandConfig(0).set_start_cmd(true);
        assert!(config.start_cmd());
        assert_eq!(config.0, 0x8000_0000);

        config = config.set_start_cmd(false);
        assert!(!config.start_cmd());
        assert_eq!(config.0, 0x0000_0000);

        config = config.enable_hold_reg();
        assert!(config.is_hold_reg_enabled());
        assert_eq!(config.0, 0x2000_0000);

        config = config.disable_hold_reg();
        assert!(!config.is_hold_reg_enabled());
        assert_eq!(config.0, 0x0000_0000);

        config = config.enable_voltage_switch();
        assert!(config.is_voltage_switch_enabled());
        assert_eq!(config.0, 0x1000_0000);

        config = config.disable_voltage_switch();
        assert!(!config.is_voltage_switch_enabled());
        assert_eq!(config.0, 0x0000_0000);

        config = config.set_boot_mode(CmdBootMode::Custom);
        assert_eq!(config.boot_mode(), CmdBootMode::Custom);
        assert_eq!(config.0, 0x0800_0000);

        config = config.set_boot_mode(CmdBootMode::Default);
        assert_eq!(config.boot_mode(), CmdBootMode::Default);
        assert_eq!(config.0, 0x0000_0000);

        config = config.set_boot_mode_dis(true);
        assert!(config.boot_mode_dis());
        assert_eq!(config.0, 0x0400_0000);

        config = config.set_boot_mode_dis(false);
        assert!(!config.boot_mode_dis());
        assert_eq!(config.0, 0x0000_0000);

        config = config.set_boot_ack_exp(true);
        assert!(config.boot_ack_exp());
        assert_eq!(config.0, 0x0200_0000);

        config = config.set_boot_ack_exp(false);
        assert!(!config.boot_ack_exp());
        assert_eq!(config.0, 0x0000_0000);

        config = config.set_boot_en(true);
        assert!(config.boot_en());
        assert_eq!(config.0, 0x0100_0000);

        config = config.set_boot_en(false);
        assert!(!config.boot_en());
        assert_eq!(config.0, 0x0000_0000);

        config = config.enable_update_clk_reg();
        assert!(config.is_update_clk_reg_enabled());
        assert_eq!(config.0, 0x0020_0000);

        config = config.disable_update_clk_reg();
        assert!(!config.is_update_clk_reg_enabled());
        assert_eq!(config.0, 0x0000_0000);

        config = config.enable_send_init();
        assert!(config.is_send_init_enabled());
        assert_eq!(config.0, 0x0000_8000);

        config = config.disable_send_init();
        assert!(!config.is_send_init_enabled());
        assert_eq!(config.0, 0x0000_0000);

        config = config.enable_send_stop();
        assert!(config.is_send_stop_enabled());
        assert_eq!(config.0, 0x0000_4000);

        config = config.disable_send_stop();
        assert!(!config.is_send_stop_enabled());
        assert_eq!(config.0, 0x0000_0000);

        config = config.enable_wait_prvdata_done();
        assert!(config.is_wait_prvdata_done_enabled());
        assert_eq!(config.0, 0x0000_2000);

        config = config.disable_wait_prvdata_done();
        assert!(!config.is_wait_prvdata_done_enabled());
        assert_eq!(config.0, 0x0000_0000);

        config = config.enable_auto_stop();
        assert!(config.is_auto_stop_enabled());
        assert_eq!(config.0, 0x0000_1000);

        config = config.disable_auto_stop();
        assert!(!config.is_auto_stop_enabled());
        assert_eq!(config.0, 0x0000_0000);

        config = config.set_transfer_mode(CmdTransferMode::Byte);
        assert_eq!(config.transfer_mode(), CmdTransferMode::Byte);
        assert_eq!(config.0, 0x0000_0800);

        config = config.set_transfer_mode(CmdTransferMode::Block);
        assert_eq!(config.transfer_mode(), CmdTransferMode::Block);
        assert_eq!(config.0, 0x0000_0000);

        config = config.set_transfer_dir(CmdTransferDir::Host2Card);
        assert_eq!(config.transfer_dir(), CmdTransferDir::Host2Card);
        assert_eq!(config.0, 0x0000_0400);

        config = config.set_transfer_dir(CmdTransferDir::Card2Host);
        assert_eq!(config.transfer_dir(), CmdTransferDir::Card2Host);
        assert_eq!(config.0, 0x0000_0000);

        config = config.set_data_expected(true);
        assert!(config.data_expected());
        assert_eq!(config.0, 0x0000_0200);

        config = config.set_data_expected(false);
        assert!(!config.data_expected());
        assert_eq!(config.0, 0x0000_0000);

        config = config.enable_check_resp_crc();
        assert!(config.is_check_resp_crc_enabled());
        assert_eq!(config.0, 0x0000_0100);

        config = config.disable_check_resp_crc();
        assert!(!config.is_check_resp_crc_enabled());
        assert_eq!(config.0, 0x0000_0000);

        config = config.set_resp_len(CmdRespLen::LongResp);
        assert_eq!(config.resp_len(), CmdRespLen::LongResp);
        assert_eq!(config.0, 0x0000_0080);

        config = config.set_resp_len(CmdRespLen::ShortResp);
        assert_eq!(config.resp_len(), CmdRespLen::ShortResp);
        assert_eq!(config.0, 0x0000_0000);

        config = config.set_resp_expected(true);
        assert!(config.resp_expected());
        assert_eq!(config.0, 0x0000_0040);

        config = config.set_resp_expected(false);
        assert!(!config.resp_expected());
        assert_eq!(config.0, 0x0000_0000);

        config = config.set_cmd_index(0x3F);
        assert_eq!(config.cmd_index(), 0x3F);
        assert_eq!(config.0, 0x0000_003F);
    }

    test_should_panic!((
        test_command_config_set_cmd_index_panic,
        CommandConfig(0).set_cmd_index(0x40),
        "Command index out of range (expected 0..=63)"
    ),);

    #[test]
    fn struct_timeout_ctrl_functions() {
        let mut ctrl = TimeoutCtrl(0).set_data_timeout(0xFF_FFFF);
        assert_eq!(ctrl.data_timeout(), 0xFF_FFFF);
        assert_eq!(ctrl.0, 0xFFFF_FF00);

        ctrl = TimeoutCtrl(0).set_resp_timeout(0xFF);
        assert_eq!(ctrl.resp_timeout(), 0xFF);
        assert_eq!(ctrl.0, 0x0000_00FF);
    }

    test_should_panic!((
        test_timeout_ctrl_set_data_timeout_panic,
        TimeoutCtrl(0).set_data_timeout(0x100_0000),
        "Data timeout out of range (expected 0..=0xFF_FFFF)"
    ),);

    #[test]
    fn struct_ctrl_status_functions() {
        let mut status = CtrlStatus(0x8000_0000);
        assert!(status.dma_request());

        status = CtrlStatus(0x4000_0000);
        assert!(status.dma_acknowledge());

        status = CtrlStatus(0x3FFE_0000);
        assert_eq!(status.fifo_count(), 0x1FFF);

        status = CtrlStatus(0x0000_F800);
        assert_eq!(status.resp_index(), 0x1F);

        status = CtrlStatus(0x0000_0400);
        assert!(status.data_mc_busy());

        status = CtrlStatus(0x0000_0200);
        assert!(status.data0_busy());

        status = CtrlStatus(0x0000_0100);
        assert!(status.data3_status());

        for i in 0..16 {
            let (reg_val, expected_status) = match i {
                0 => (0x0000_0000, CmdFsmState::Idle),
                1 => (0x0000_0010, CmdFsmState::SendInitSeq),
                2 => (0x0000_0020, CmdFsmState::TransmitStart),
                3 => (0x0000_0030, CmdFsmState::TransmitTx),
                4 => (0x0000_0040, CmdFsmState::TransmitCmdIdxAndArg),
                5 => (0x0000_0050, CmdFsmState::TransmitCrc7),
                6 => (0x0000_0060, CmdFsmState::TransmitEnd),
                7 => (0x0000_0070, CmdFsmState::ReceiveRespStart),
                8 => (0x0000_0080, CmdFsmState::ReceiveIntResp),
                9 => (0x0000_0090, CmdFsmState::ReceiveRespTx),
                10 => (0x0000_00A0, CmdFsmState::ReceiveRespCmdIdx),
                11 => (0x0000_00B0, CmdFsmState::ReceiveRespData),
                12 => (0x0000_00C0, CmdFsmState::ReceiveRespCrc7),
                13 => (0x0000_00D0, CmdFsmState::ReceiveRespEnd),
                14 => (0x0000_00E0, CmdFsmState::CmdPathWaitNcc),
                15 => (0x0000_00F0, CmdFsmState::WaitState),
                _ => unreachable!(),
            };
            let status = CtrlStatus(reg_val);
            assert_eq!(status.cmd_fsm_status(), expected_status);
        }

        status = CtrlStatus(0x0000_0008);
        assert!(status.fifo_full());

        status = CtrlStatus(0x0000_0004);
        assert!(status.fifo_empty());

        status = CtrlStatus(0x0000_0002);
        assert!(status.fifo_tx_wmark());

        status = CtrlStatus(0x0000_0001);
        assert!(status.fifo_rx_wmark());
    }

    #[test]
    fn struct_host_ctrl1_functions() {
        let mut ctrl = HostCtrl1(0).enable_idmac();
        assert!(ctrl.is_idmac_enabled());
        assert_eq!(ctrl.0, 0x0200_0000);

        ctrl = ctrl.disable_idmac();
        assert!(!ctrl.is_idmac_enabled());
        assert_eq!(ctrl.0, 0x0000_0000);

        ctrl = ctrl.enable_opendrain_pullup();
        assert!(ctrl.is_opendrain_pullup_enabled());
        assert_eq!(ctrl.0, 0x0100_0000);

        ctrl = ctrl.disable_opendrain_pullup();
        assert!(!ctrl.is_opendrain_pullup_enabled());
        assert_eq!(ctrl.0, 0x0000_0000);

        ctrl = ctrl.set_abort_rdata(true);
        assert!(ctrl.abort_rdata());
        assert_eq!(ctrl.0, 0x0000_0100);

        ctrl = ctrl.set_abort_rdata(false);
        assert!(!ctrl.abort_rdata());
        assert_eq!(ctrl.0, 0x0000_0000);

        ctrl = ctrl.set_send_irq_resp(true);
        assert!(ctrl.send_irq_resp());
        assert_eq!(ctrl.0, 0x0000_0080);

        ctrl = ctrl.set_send_irq_resp(false);
        assert!(!ctrl.send_irq_resp());
        assert_eq!(ctrl.0, 0x0000_0000);

        ctrl = ctrl.set_read_wait(true);
        assert!(ctrl.read_wait());
        assert_eq!(ctrl.0, 0x0000_0040);

        ctrl = ctrl.set_read_wait(false);
        assert!(!ctrl.read_wait());
        assert_eq!(ctrl.0, 0x0000_0000);

        ctrl = ctrl.enable_int();
        assert!(ctrl.is_int_enabled());
        assert_eq!(ctrl.0, 0x0000_0010);

        ctrl = ctrl.disable_int();
        assert!(!ctrl.is_int_enabled());
        assert_eq!(ctrl.0, 0x0000_0000);

        ctrl = ctrl.set_hw_rstn(true);
        assert!(ctrl.hw_rstn());
        assert_eq!(ctrl.0, 0x0000_0008);

        ctrl = ctrl.set_hw_rstn(false);
        assert!(!ctrl.hw_rstn());
        assert_eq!(ctrl.0, 0x0000_0000);

        ctrl = ctrl.set_dma_rst(true);
        assert!(ctrl.dma_rst());
        assert_eq!(ctrl.0, 0x0000_0004);

        ctrl = ctrl.set_dma_rst(false);
        assert!(!ctrl.dma_rst());
        assert_eq!(ctrl.0, 0x0000_0000);

        ctrl = ctrl.set_fifo_reset(true);
        assert!(ctrl.fifo_reset());
        assert_eq!(ctrl.0, 0x0000_0002);

        ctrl = ctrl.set_fifo_reset(false);
        assert!(!ctrl.fifo_reset());
        assert_eq!(ctrl.0, 0x0000_0000);

        ctrl = ctrl.set_ctrl_reset(true);
        assert!(ctrl.ctrl_reset());
        assert_eq!(ctrl.0, 0x0000_0001);

        ctrl = ctrl.set_ctrl_reset(false);
        assert!(!ctrl.ctrl_reset());
        assert_eq!(ctrl.0, 0x0000_0000);
    }

    #[test]
    fn struct_clock_ctrl_functions() {
        let mut ctrl = ClockCtrl(0).enable_clk_lp();
        assert!(ctrl.is_clk_lp_enabled());
        assert_eq!(ctrl.0, 0x0001_0000);

        ctrl = ctrl.disable_clk_lp();
        assert!(!ctrl.is_clk_lp_enabled());
        assert_eq!(ctrl.0, 0x0000_0000);

        ctrl = ctrl.set_clk_div(0xFF);
        assert_eq!(ctrl.clk_div(), 0xFF);
        assert_eq!(ctrl.0, 0x0000_FF00);

        ctrl = ClockCtrl(0).enable_clk();
        assert!(ctrl.is_clk_enabled());
        assert_eq!(ctrl.0, 0x0000_0001);

        ctrl = ctrl.disable_clk();
        assert!(!ctrl.is_clk_enabled());
        assert_eq!(ctrl.0, 0x0000_0000);
    }

    #[test]
    fn struct_host_ctrl2_functions() {
        let mut ctrl = HostCtrl2(0).set_bus_width(BusWidth::FourWire);
        assert_eq!(ctrl.bus_width(), BusWidth::FourWire);
        assert_eq!(ctrl.0, 0x1000_0000);

        ctrl = ctrl.set_bus_width(BusWidth::OneWire);
        assert_eq!(ctrl.bus_width(), BusWidth::OneWire);
        assert_eq!(ctrl.0, 0x0000_0000);

        ctrl = ctrl.set_dto_unit(TimeoutUnit::CclkOutX256);
        assert_eq!(ctrl.dto_unit(), TimeoutUnit::CclkOutX256);
        assert_eq!(ctrl.0, 0x0200_0000);

        ctrl = ctrl.set_dto_unit(TimeoutUnit::CclkOutX1);
        assert_eq!(ctrl.dto_unit(), TimeoutUnit::CclkOutX1);
        assert_eq!(ctrl.0, 0x0000_0000);

        ctrl = ctrl.set_rto_unit(TimeoutUnit::CclkOutX256);
        assert_eq!(ctrl.rto_unit(), TimeoutUnit::CclkOutX256);
        assert_eq!(ctrl.0, 0x0100_0000);

        ctrl = ctrl.set_rto_unit(TimeoutUnit::CclkOutX1);
        assert_eq!(ctrl.rto_unit(), TimeoutUnit::CclkOutX1);
        assert_eq!(ctrl.0, 0x0000_0000);

        ctrl = ctrl.set_transfer_mode(TransferMode::Ddr);
        assert_eq!(ctrl.transfer_mode(), TransferMode::Ddr);
        assert_eq!(ctrl.0, 0x0001_0000);

        ctrl = ctrl.set_transfer_mode(TransferMode::Sdr);
        assert_eq!(ctrl.transfer_mode(), TransferMode::Sdr);
        assert_eq!(ctrl.0, 0x0000_0000);
    }

    #[test]
    fn struct_int_enable_functions() {
        let mut inten = IntEnable(0).enable_sdmc_interrupt();
        assert!(inten.is_sdmc_interrupt_enabled());
        assert_eq!(inten.0, 0x0001_0000);

        inten = inten.disable_sdmc_interrupt();
        assert!(!inten.is_sdmc_interrupt_enabled());
        assert_eq!(inten.0, 0x0000_0000);

        for i in 0..16 {
            let (reg_val, expected_interrupt) = match i {
                0 => (0x0000_0001, SdmcInterrupt::CardDetect),
                1 => (0x0000_0002, SdmcInterrupt::RespError),
                2 => (0x0000_0004, SdmcInterrupt::CmdComplete),
                3 => (0x0000_0008, SdmcInterrupt::DataDone),
                4 => (0x0000_0010, SdmcInterrupt::TxFifoReq),
                5 => (0x0000_0020, SdmcInterrupt::RxFifoReq),
                6 => (0x0000_0040, SdmcInterrupt::RespCrcError),
                7 => (0x0000_0080, SdmcInterrupt::DataCrcError),
                8 => (0x0000_0100, SdmcInterrupt::RespTimeout),
                9 => (0x0000_0200, SdmcInterrupt::ReadTimeout),
                10 => (0x0000_0400, SdmcInterrupt::HostTimeoutOrVSwitch),
                11 => (0x0000_0800, SdmcInterrupt::FifoUndOverrun),
                12 => (0x0000_1000, SdmcInterrupt::HwLockError),
                13 => (0x0000_2000, SdmcInterrupt::StartBitOrBusyClear),
                14 => (0x0000_4000, SdmcInterrupt::AutoCmdDone),
                15 => (0x0000_8000, SdmcInterrupt::EndBitOrWriteNoCrc),
                _ => unreachable!(),
            };

            inten = inten.enable_interrupt(expected_interrupt);
            assert!(inten.is_interrupt_enabled(expected_interrupt));
            assert_eq!(inten.0, reg_val);

            inten = inten.disable_interrupt(expected_interrupt);
            assert!(!inten.is_interrupt_enabled(expected_interrupt));
            assert_eq!(inten.0, 0x0000_0000);
        }
    }

    #[test]
    fn struct_int_status_functions() {
        let mut status: IntStatus = IntStatus(0x10000);
        assert!(status.is_sdio_interrupt_pending());

        for i in 0..16 {
            let (reg_val, expected_interrupt) = match i {
                0 => (0x0000_0001, SdmcInterrupt::CardDetect),
                1 => (0x0000_0002, SdmcInterrupt::RespError),
                2 => (0x0000_0004, SdmcInterrupt::CmdComplete),
                3 => (0x0000_0008, SdmcInterrupt::DataDone),
                4 => (0x0000_0010, SdmcInterrupt::TxFifoReq),
                5 => (0x0000_0020, SdmcInterrupt::RxFifoReq),
                6 => (0x0000_0040, SdmcInterrupt::RespCrcError),
                7 => (0x0000_0080, SdmcInterrupt::DataCrcError),
                8 => (0x0000_0100, SdmcInterrupt::RespTimeout),
                9 => (0x0000_0200, SdmcInterrupt::ReadTimeout),
                10 => (0x0000_0400, SdmcInterrupt::HostTimeoutOrVSwitch),
                11 => (0x0000_0800, SdmcInterrupt::FifoUndOverrun),
                12 => (0x0000_1000, SdmcInterrupt::HwLockError),
                13 => (0x0000_2000, SdmcInterrupt::StartBitOrBusyClear),
                14 => (0x0000_4000, SdmcInterrupt::AutoCmdDone),
                15 => (0x0000_8000, SdmcInterrupt::EndBitOrWriteNoCrc),
                _ => unreachable!(),
            };

            status = IntStatus(reg_val);
            assert!(status.is_interrupt_pending(expected_interrupt));
        }
    }

    #[test]
    fn struct_origin_int_status_functions() {
        let mut status = OriginalIntStatus(0x10000);
        assert!(status.is_sdio_interrupt_pending());

        status = OriginalIntStatus(0).clear_sdio_interrupt();
        assert_eq!(status.0, 0x0001_0000);

        for i in 0..16 {
            let (reg_val, expected_interrupt) = match i {
                0 => (0x0000_0001, SdmcInterrupt::CardDetect),
                1 => (0x0000_0002, SdmcInterrupt::RespError),
                2 => (0x0000_0004, SdmcInterrupt::CmdComplete),
                3 => (0x0000_0008, SdmcInterrupt::DataDone),
                4 => (0x0000_0010, SdmcInterrupt::TxFifoReq),
                5 => (0x0000_0020, SdmcInterrupt::RxFifoReq),
                6 => (0x0000_0040, SdmcInterrupt::RespCrcError),
                7 => (0x0000_0080, SdmcInterrupt::DataCrcError),
                8 => (0x0000_0100, SdmcInterrupt::RespTimeout),
                9 => (0x0000_0200, SdmcInterrupt::ReadTimeout),
                10 => (0x0000_0400, SdmcInterrupt::HostTimeoutOrVSwitch),
                11 => (0x0000_0800, SdmcInterrupt::FifoUndOverrun),
                12 => (0x0000_1000, SdmcInterrupt::HwLockError),
                13 => (0x0000_2000, SdmcInterrupt::StartBitOrBusyClear),
                14 => (0x0000_4000, SdmcInterrupt::AutoCmdDone),
                15 => (0x0000_8000, SdmcInterrupt::EndBitOrWriteNoCrc),
                _ => unreachable!(),
            };

            status = OriginalIntStatus(reg_val);
            assert!(status.is_interrupt_pending(expected_interrupt));

            status = OriginalIntStatus(0).clear_interrupt(expected_interrupt);
            assert_eq!(status.0, reg_val);
        }
    }

    #[test]
    fn struct_fifo_config_functions() {
        for i in 0..8 {
            let (reg_val, expected_length) = match i {
                0 => (0x0000_0000, SdmcBurstLength::WordX1),
                1 => (0x1000_0000, SdmcBurstLength::WordX4),
                2 => (0x2000_0000, SdmcBurstLength::WordX8),
                3 => (0x3000_0000, SdmcBurstLength::WordX16),
                4 => (0x4000_0000, SdmcBurstLength::WordX32),
                5 => (0x5000_0000, SdmcBurstLength::WordX64),
                6 => (0x6000_0000, SdmcBurstLength::WordX128),
                7 => (0x7000_0000, SdmcBurstLength::WordX256),
                _ => unreachable!(),
            };

            let config = FifoConfig(0).set_dma_burst_length(expected_length);
            assert_eq!(config.dma_burst_length(), expected_length);
            assert_eq!(config.0, reg_val);
        }

        let mut config = FifoConfig(0).set_fifo_rx_watermark(0x7FF);
        assert_eq!(config.fifo_rx_watermark(), 0x7FF);
        assert_eq!(config.0, 0x07FF_0000);

        config = FifoConfig(0).set_fifo_tx_watermark(0xFFF);
        assert_eq!(config.fifo_tx_watermark(), 0xFFF);
        assert_eq!(config.0, 0x0000_0FFF);
    }

    test_should_panic!(
        (
            test_fifo_config_set_fifo_rx_watermark_panic,
            FifoConfig(0).set_fifo_rx_watermark(0x800),
            "FIFO RX watermark out of range (expected 0..=0x7FF)"
        ),
        (
            test_fifo_config_set_fifo_tx_watermark_panic,
            FifoConfig(0).set_fifo_tx_watermark(0x1000),
            "FIFO TX watermark out of range (expected 0..=0xFFF)"
        ),
    );

    #[test]
    fn struct_card_detect_functions() {
        let mut val = CardDetect(0x0100_0000);
        assert!(val.card_detect_status());

        val = CardDetect(0).set_debounce_count(0xFF_FFFF);
        assert_eq!(val.0, 0x00FF_FFFF);
    }

    test_should_panic!((
        test_card_detect_set_debounce_count_panic,
        CardDetect(0).set_debounce_count(0x100_0000),
        "Debounce count out of range (expected 0..=0xFF_FFFF)"
    ),);

    #[test]
    fn struct_peripheral_bus_config_functions() {
        for i in 0..8 {
            let (reg_val, expected_length) = match i {
                0 => (0x0000_0000, SdmcBurstLength::WordX1),
                1 => (0x0000_0100, SdmcBurstLength::WordX4),
                2 => (0x0000_0200, SdmcBurstLength::WordX8),
                3 => (0x0000_0300, SdmcBurstLength::WordX16),
                4 => (0x0000_0400, SdmcBurstLength::WordX32),
                5 => (0x0000_0500, SdmcBurstLength::WordX64),
                6 => (0x0000_0600, SdmcBurstLength::WordX128),
                7 => (0x0000_0700, SdmcBurstLength::WordX256),
                _ => unreachable!(),
            };

            let config = PeripheralBusConfig(reg_val);
            assert_eq!(config.programmed_burst_length(), expected_length);
        }

        let mut config = PeripheralBusConfig(0).enable_idma();
        assert!(config.is_idma_enabled());
        assert_eq!(config.0, 0x0000_0080);

        config = config.disable_idma();
        assert!(!config.is_idma_enabled());
        assert_eq!(config.0, 0x0000_0000);

        config = config.set_desc_skip_length(0x1F);
        assert_eq!(config.desc_skip_length(), 0x1F);
        assert_eq!(config.0, 0x0000_007C);

        config = PeripheralBusConfig(0).enable_fixed_burst_length();
        assert!(config.is_fixed_burst_length_enabled());
        assert_eq!(config.0, 0x0000_0002);

        config = config.disable_fixed_burst_length();
        assert!(!config.is_fixed_burst_length_enabled());
        assert_eq!(config.0, 0x0000_0000);

        config = config.set_software_reset(true);
        assert!(config.software_reset());
        assert_eq!(config.0, 0x0000_0001);

        config = config.set_software_reset(false);
        assert!(!config.software_reset());
        assert_eq!(config.0, 0x0000_0000);
    }

    test_should_panic!((
        test_peripheral_bus_config_set_desc_skip_length_panic,
        PeripheralBusConfig(0).set_desc_skip_length(0x20),
        "Descriptor skip length out of range (expected 0..=31)"
    ),);

    #[test]
    fn struct_idma_status_functions() {
        for i in 0..9 {
            let (reg_val, expected_state) = match i {
                0 => (0x0000_0000, IdmaFsmState::DmaIdle),
                1 => (0x0000_2000, IdmaFsmState::DmaSuspend),
                2 => (0x0000_4000, IdmaFsmState::DmacRead),
                3 => (0x0000_6000, IdmaFsmState::DmacCheck),
                4 => (0x0000_8000, IdmaFsmState::DmaReadReqWait),
                5 => (0x0000_A000, IdmaFsmState::DmaWriteReqWait),
                6 => (0x0000_C000, IdmaFsmState::DmaRead),
                7 => (0x0000_E000, IdmaFsmState::DmaWrite),
                8 => (0x0001_0000, IdmaFsmState::DescClose),
                _ => unreachable!(),
            };
            let status = IdmaStatus(reg_val);
            assert_eq!(status.idma_fsm_state(), expected_state);
        }

        let mut status = IdmaStatus(0x0000_0400);
        assert_eq!(
            status.fatal_bus_error(),
            IdmaFatalBusError::HostAbortReceive
        );

        status = IdmaStatus(0x0000_0000);
        assert_eq!(
            status.fatal_bus_error(),
            IdmaFatalBusError::HostAbortTransmit
        );

        status = IdmaStatus(0x0000_0200);
        assert!(status.is_abnormal_interrupt_pending());
        assert_eq!(IdmaStatus(0).clear_abnormal_interrupt().0, 0x0000_0200);

        status = IdmaStatus(0x0000_0100);
        assert!(status.is_normal_interrupt_pending());
        assert_eq!(IdmaStatus(0).clear_normal_interrupt().0, 0x0000_0100);

        status = IdmaStatus(0x0000_0020);
        assert!(status.is_card_error_interrupt_pending());
        assert_eq!(IdmaStatus(0).clear_card_error_interrupt().0, 0x0000_0020);

        status = IdmaStatus(0x0000_0010);
        assert!(status.is_desc_unusable_interrupt_pending());
        assert_eq!(IdmaStatus(0).clear_desc_unusable_interrupt().0, 0x0000_0010);

        status = IdmaStatus(0x0000_0004);
        assert!(status.is_fatal_bus_error_interrupt_pending());
        assert_eq!(
            IdmaStatus(0).clear_fatal_bus_error_interrupt().0,
            0x0000_0004
        );

        status = IdmaStatus(0x0000_0002);
        assert!(status.is_rx_done_interrupt_pending());
        assert_eq!(IdmaStatus(0).clear_rx_done_interrupt().0, 0x0000_0002);

        status = IdmaStatus(0x0000_0001);
        assert!(status.is_tx_done_interrupt_pending());
        assert_eq!(IdmaStatus(0).clear_tx_done_interrupt().0, 0x0000_0001);
    }

    #[test]
    fn struct_idma_interrupt_enable_functions() {
        let mut inten = IdmaInterruptEnable(0);

        inten = inten.enable_abnormal_interrupt();
        assert!(inten.is_abnormal_interrupt_enabled());
        assert_eq!(inten.0, 0x0000_0200);

        inten = inten.disable_abnormal_interrupt();
        assert!(!inten.is_abnormal_interrupt_enabled());
        assert_eq!(inten.0, 0x0000_0000);

        inten = inten.enable_normal_interrupt();
        assert!(inten.is_normal_interrupt_enabled());
        assert_eq!(inten.0, 0x0000_0100);

        inten = inten.disable_normal_interrupt();
        assert!(!inten.is_normal_interrupt_enabled());
        assert_eq!(inten.0, 0x0000_0000);

        inten = inten.enable_card_error_interrupt();
        assert!(inten.is_card_error_interrupt_enabled());
        assert_eq!(inten.0, 0x0000_0020);

        inten = inten.disable_card_error_interrupt();
        assert!(!inten.is_card_error_interrupt_enabled());
        assert_eq!(inten.0, 0x0000_0000);

        inten = inten.enable_desc_unusable_interrupt();
        assert!(inten.is_desc_unusable_interrupt_enabled());
        assert_eq!(inten.0, 0x0000_0010);

        inten = inten.disable_desc_unusable_interrupt();
        assert!(!inten.is_desc_unusable_interrupt_enabled());
        assert_eq!(inten.0, 0x0000_0000);

        inten = inten.enable_fatal_bus_error_interrupt();
        assert!(inten.is_fatal_bus_error_interrupt_enabled());
        assert_eq!(inten.0, 0x0000_0004);

        inten = inten.disable_fatal_bus_error_interrupt();
        assert!(!inten.is_fatal_bus_error_interrupt_enabled());
        assert_eq!(inten.0, 0x0000_0000);

        inten = inten.enable_rx_done_interrupt();
        assert!(inten.is_rx_done_interrupt_enabled());
        assert_eq!(inten.0, 0x0000_0002);

        inten = inten.disable_rx_done_interrupt();
        assert!(!inten.is_rx_done_interrupt_enabled());
        assert_eq!(inten.0, 0x0000_0000);

        inten = inten.enable_tx_done_interrupt();
        assert!(inten.is_tx_done_interrupt_enabled());
        assert_eq!(inten.0, 0x0000_0001);

        inten = inten.disable_tx_done_interrupt();
        assert!(!inten.is_tx_done_interrupt_enabled());
        assert_eq!(inten.0, 0x0000_0000);
    }

    #[test]
    fn struct_card_thres_ctrl_functions() {
        let mut ctrl = CardThresCtrl(0);

        ctrl = ctrl.set_card_threshold_size(0x3FF);
        assert_eq!(ctrl.card_threshold_size(), 0x3FF);
        assert_eq!(ctrl.0, 0x03FF_0000);

        ctrl = CardThresCtrl(0).enable_card_write_threshold();
        assert!(ctrl.is_card_write_threshold_enabled());
        assert_eq!(ctrl.0, 0x0000_0004);

        ctrl = ctrl.disable_card_write_threshold();
        assert!(!ctrl.is_card_write_threshold_enabled());
        assert_eq!(ctrl.0, 0x0000_0000);

        ctrl = ctrl.enable_card_busy_clear_interrupt();
        assert!(ctrl.is_card_busy_clear_interrupt_enabled());
        assert_eq!(ctrl.0, 0x0000_0002);

        ctrl = ctrl.disable_card_busy_clear_interrupt();
        assert!(!ctrl.is_card_busy_clear_interrupt_enabled());
        assert_eq!(ctrl.0, 0x0000_0000);

        ctrl = ctrl.enable_card_read_threshold();
        assert!(ctrl.is_card_read_threshold_enabled());
        assert_eq!(ctrl.0, 0x0000_0001);

        ctrl = ctrl.disable_card_read_threshold();
        assert!(!ctrl.is_card_read_threshold_enabled());
        assert_eq!(ctrl.0, 0x0000_0000);
    }

    test_should_panic!((
        test_card_thres_ctrl_set_card_threshold_size_panic,
        CardThresCtrl(0).set_card_threshold_size(0x400),
        "Card threshold size out of range (expected 0..=1023)"
    ),);

    #[test]
    fn struct_card_delay_ctrl_functions() {
        let mut ctrl = CardDelayCtrl(0);

        ctrl = ctrl.set_ext_clk_mux(ExtClkMux::ClkInDiv1);
        assert_eq!(ctrl.ext_clk_mux(), ExtClkMux::ClkInDiv1);
        assert_eq!(ctrl.0, 0x8000_0000);

        ctrl = ctrl.set_ext_clk_mux(ExtClkMux::ClkInDiv2);
        assert_eq!(ctrl.ext_clk_mux(), ExtClkMux::ClkInDiv2);
        assert_eq!(ctrl.0, 0x4000_0000);

        ctrl = ctrl.set_ext_clk_mux(ExtClkMux::ClkInDiv4);
        assert_eq!(ctrl.ext_clk_mux(), ExtClkMux::ClkInDiv4);
        assert_eq!(ctrl.0, 0x0000_0000);

        ctrl = ctrl.set_drv_clk_phase(ClockPhase::Deg270);
        assert_eq!(ctrl.drv_clk_phase(), ClockPhase::Deg270);
        assert_eq!(ctrl.0, 0x3000_0000);

        ctrl = ctrl.set_drv_clk_phase(ClockPhase::Deg180);
        assert_eq!(ctrl.drv_clk_phase(), ClockPhase::Deg180);
        assert_eq!(ctrl.0, 0x2000_0000);

        ctrl = ctrl.set_drv_clk_phase(ClockPhase::Deg90);
        assert_eq!(ctrl.drv_clk_phase(), ClockPhase::Deg90);
        assert_eq!(ctrl.0, 0x1000_0000);

        ctrl = ctrl.set_drv_clk_phase(ClockPhase::Deg0);
        assert_eq!(ctrl.drv_clk_phase(), ClockPhase::Deg0);
        assert_eq!(ctrl.0, 0x0000_0000);

        ctrl = ctrl.set_drv_clk_delay(0x1F);
        assert_eq!(ctrl.drv_clk_delay(), 0x1F);
        assert_eq!(ctrl.0, 0x0F80_0000);

        ctrl = CardDelayCtrl(0).set_samp_clk_phase(ClockPhase::Deg270);
        assert_eq!(ctrl.samp_clk_phase(), ClockPhase::Deg270);
        assert_eq!(ctrl.0, 0x0060_0000);

        ctrl = ctrl.set_samp_clk_phase(ClockPhase::Deg180);
        assert_eq!(ctrl.samp_clk_phase(), ClockPhase::Deg180);
        assert_eq!(ctrl.0, 0x0040_0000);

        ctrl = ctrl.set_samp_clk_phase(ClockPhase::Deg90);
        assert_eq!(ctrl.samp_clk_phase(), ClockPhase::Deg90);
        assert_eq!(ctrl.0, 0x0020_0000);

        ctrl = ctrl.set_samp_clk_phase(ClockPhase::Deg0);
        assert_eq!(ctrl.samp_clk_phase(), ClockPhase::Deg0);
        assert_eq!(ctrl.0, 0x0000_0000);

        ctrl = ctrl.set_samp_clk_delay(0x1F);
        assert_eq!(ctrl.samp_clk_delay(), 0x1F);
        assert_eq!(ctrl.0, 0x001F_0000);

        ctrl = CardDelayCtrl(0).set_ddr_samp_swap(true);
        assert!(ctrl.ddr_samp_swap());
        assert_eq!(ctrl.0, 0x0000_0100);

        ctrl = ctrl.set_ddr_samp_swap(false);
        assert!(!ctrl.ddr_samp_swap());
        assert_eq!(ctrl.0, 0x0000_0000);

        ctrl = ctrl.enable_ddr_samp_swap();
        assert!(ctrl.is_ddr_samp_swap_enabled());
        assert_eq!(ctrl.0, 0x0000_0040);

        ctrl = ctrl.disable_ddr_samp_swap();
        assert!(!ctrl.is_ddr_samp_swap_enabled());
        assert_eq!(ctrl.0, 0x0000_0000);

        ctrl = ctrl.set_ddr_samp_even_phase(ClockPhase::Deg270);
        assert_eq!(ctrl.ddr_samp_even_phase(), ClockPhase::Deg270);
        assert_eq!(ctrl.0, 0x0000_0030);

        ctrl = ctrl.set_ddr_samp_even_phase(ClockPhase::Deg180);
        assert_eq!(ctrl.ddr_samp_even_phase(), ClockPhase::Deg180);
        assert_eq!(ctrl.0, 0x0000_0020);

        ctrl = ctrl.set_ddr_samp_even_phase(ClockPhase::Deg90);
        assert_eq!(ctrl.ddr_samp_even_phase(), ClockPhase::Deg90);
        assert_eq!(ctrl.0, 0x0000_0010);

        ctrl = ctrl.set_ddr_samp_even_phase(ClockPhase::Deg0);
        assert_eq!(ctrl.ddr_samp_even_phase(), ClockPhase::Deg0);
        assert_eq!(ctrl.0, 0x0000_0000);

        ctrl = ctrl.set_clk_phase(ClockPhase::Deg270);
        assert_eq!(ctrl.clk_phase(), ClockPhase::Deg270);
        assert_eq!(ctrl.0, 0x0000_0003);

        ctrl = ctrl.set_clk_phase(ClockPhase::Deg180);
        assert_eq!(ctrl.clk_phase(), ClockPhase::Deg180);
        assert_eq!(ctrl.0, 0x0000_0002);

        ctrl = ctrl.set_clk_phase(ClockPhase::Deg90);
        assert_eq!(ctrl.clk_phase(), ClockPhase::Deg90);
        assert_eq!(ctrl.0, 0x0000_0001);

        ctrl = ctrl.set_clk_phase(ClockPhase::Deg0);
        assert_eq!(ctrl.clk_phase(), ClockPhase::Deg0);
        assert_eq!(ctrl.0, 0x0000_0000);
    }

    test_should_panic!(
        (
            test_card_delay_ctrl_set_drv_clk_delay_panic,
            CardDelayCtrl(0).set_drv_clk_delay(0x20),
            "Driver clock delay out of range (expected 0..=31)"
        ),
        (
            test_card_delay_ctrl_set_samp_clk_delay_panic,
            CardDelayCtrl(0).set_samp_clk_delay(0x20),
            "Sample clock delay out of range (expected 0..=31)"
        ),
    );

    #[test]
    fn struct_emmc_config_functions() {
        let mut config = EmmcConfig(0).set_half_start(true);
        assert!(config.half_start());
        assert_eq!(config.0, 0x0000_0001);

        config = config.set_half_start(false);
        assert!(!config.half_start());
        assert_eq!(config.0, 0x0000_0000);
    }
}
