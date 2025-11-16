//! UART register blocks and registers.

use volatile_register::{RO, RW, WO};

use uart16550::Uart16550;

/// Universal Asynchronous Receiver-Transmitter registers.
#[repr(C)]
pub struct RegisterBlock {
    /// Standard UART 16550 registers.
    ///
    /// Includes the transmit and receive buffers, line control, and modem control registers.
    pub uart16550: Uart16550<u32>,
    /// UART scratch register (`UART_SCH`).
    #[doc(alias = "UART_SCH")]
    pub sch: RW<UartScratch>,
    _reserved0: [u8; 0x5C],
    /// UART status register (`UART_USR`).
    #[doc(alias = "UART_USR")]
    pub usr: RO<UartStatus>,
    /// UART transmit FIFO level register (`UART_TFL`).
    #[doc(alias = "UART_TFL")]
    pub tfl: RO<TransmitFifoLevel>,
    /// UART receive FIFO level register (`UART_RFL`).
    #[doc(alias = "UART_RFL")]
    pub rfl: RO<ReceiveFifoLevel>,
    /// UART DMA handshake configuration register (`UART_HSK`).
    #[doc(alias = "UART_HSK")]
    pub hsk: RW<DmaHandshakeConfig>,
    _reserved1: [u8; 0x18],
    /// UART halt transmit register (`UART_HALT`).
    #[doc(alias = "UART_HALT")]
    pub halt: RW<HaltTx>,
    _reserved2: [u8; 0x8],
    /// UART debug dll register (`UART_DBG_DLL`).
    #[doc(alias = "UART_DBG_DLL")]
    pub dbg_dll: RW<DebugDll>,
    /// UART debug dlh register (`UART_DBG_DLH`).
    #[doc(alias = "UART_DBG_DLH")]
    pub dbg_dlh: RW<DebugDlh>,
    /// UART RS485 DE time register (`UART_485_DE`).
    #[doc(alias = "UART_485_DE")]
    pub rs485_de: RW<Rs485DeTime>,
    _reserved3: [u8; 0x4],
    /// UART RS485 control and status register (`UART_485_CTL`).
    #[doc(alias = "UART_485_CTL")]
    pub rs485_ctl: RW<Rs485Control>,
    /// UART RS485 address match register (`RS485_ADDR_MATCH`).
    #[doc(alias = "RS485_ADDR_MATCH")]
    pub rs485_addr_match: RW<Rs485AddressMatch>,
    /// UART RS485 bus idle check register (`BUS_IDLE_CHK`).
    #[doc(alias = "BUS_IDLE_CHK")]
    pub rs485_bus_idle_check: RW<Rs485BusIdleCheck>,
    /// UART transmit delay register (`TX_DLY`).
    #[doc(alias = "TX_DLY")]
    pub tx_delay: RW<TransmitDelay>,
    /// UART debug register (`UART_DBR`).
    #[doc(alias = "UART_DBR")]
    pub debug: WO<DebugRegister>,
    _reserved4: [u8; 0x28],
    /// UART version register (`UART_VERSION`).
    #[doc(alias = "UART_VERSION")]
    pub version: RO<Version>,
}

/// UART scratch register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UartScratch(u32);

impl UartScratch {
    const SCRATCH_REG: u32 = 0xFF;

    /// Set the scratch value (`SCRATCH_REG`).
    #[doc(alias = "SCRATCH_REG")]
    #[inline]
    pub const fn set_scratch_val(self, val: u8) -> Self {
        Self((self.0 & !Self::SCRATCH_REG) | (Self::SCRATCH_REG & (val as u32)))
    }
    /// Get the scratch value.
    #[inline]
    pub const fn scratch_val(self) -> u8 {
        (self.0 & Self::SCRATCH_REG) as u8
    }
}

/// UART status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UartStatus(u32);

impl UartStatus {
    const RFF: u32 = 1 << 4;
    const RFNE: u32 = 1 << 3;
    const TFE: u32 = 1 << 2;
    const TFNF: u32 = 1 << 1;
    const BUSY: u32 = 1;

    /// Check if the receive fifo is full (`RFF`).
    #[doc(alias = "RFF")]
    #[inline]
    pub const fn is_receive_fifo_full(self) -> bool {
        (self.0 & Self::RFF) != 0
    }
    /// Check if the receive fifo is not empty (`RFNE`).
    #[doc(alias = "RFNE")]
    #[inline]
    pub const fn is_receive_fifo_not_empty(self) -> bool {
        (self.0 & Self::RFNE) != 0
    }
    /// Check if the transmit fifo is empty (`TFE`).
    #[doc(alias = "TFE")]
    #[inline]
    pub const fn is_transmit_fifo_empty(self) -> bool {
        (self.0 & Self::TFE) != 0
    }
    /// Check if the transmit fifo is not full (`TFNF`).
    #[doc(alias = "TFNF")]
    #[inline]
    pub const fn is_transmit_fifo_not_full(self) -> bool {
        (self.0 & Self::TFNF) != 0
    }
    /// Check if the peripheral is busy (`BUSY`).
    #[doc(alias = "BUSY")]
    #[inline]
    pub const fn is_busy(self) -> bool {
        (self.0 & Self::BUSY) != 0
    }
}

/// UART transmit FIFO level register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TransmitFifoLevel(u32);

impl TransmitFifoLevel {
    const TFL: u32 = 0x1FF;

    /// Get the transmit fifo level (`TFL`).
    #[doc(alias = "TFL")]
    #[inline]
    pub const fn tx_level(self) -> u16 {
        (self.0 & Self::TFL) as u16
    }
}

/// UART receive FIFO level register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ReceiveFifoLevel(u32);

impl ReceiveFifoLevel {
    const RFL: u32 = 0x1FF;

    /// Get the receive fifo level (`RFL`).
    #[doc(alias = "RFL")]
    #[inline]
    pub const fn rx_level(self) -> u16 {
        (self.0 & Self::RFL) as u16
    }
}

/// UART dma handshake configuration mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DmaHandshakeMode {
    /// DMA wait cycle mode.
    WaitCycle = 0xA5,
    /// DMA handshake mode.
    Handshake = 0xE5,
}

/// UART DMA handshake configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DmaHandshakeConfig(u32);

impl DmaHandshakeConfig {
    const HSK: u32 = 0xFF;

    /// Set the dma handshake mode (`HSK`).
    #[doc(alias = "HSK")]
    #[inline]
    pub const fn set_handshake_mode(self, mode: DmaHandshakeMode) -> Self {
        Self((self.0 & !Self::HSK) | (Self::HSK & (mode as u32)))
    }
    /// Get the dma handshake mode.
    #[inline]
    pub const fn handshake_mode(self) -> DmaHandshakeMode {
        match self.0 & Self::HSK {
            0xA5 => DmaHandshakeMode::WaitCycle,
            0xE5 => DmaHandshakeMode::Handshake,
            _ => unreachable!(),
        }
    }
}

/// UART halt transmit register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct HaltTx(u32);

impl HaltTx {
    const DMA_PTE_RX: u32 = 0x1 << 7;
    const DMA_PTE_TX: u32 = 0x1 << 6;
    const HALT_CHANGE_UPDATE: u32 = 0x1 << 2;
    const HALT_CHCFG_AT_BUSY: u32 = 0x1 << 1;
    const HALT_TX: u32 = 0x1;

    /// Set the DMA_PTE_RX bit (`DMA_PTE_RX`).
    ///
    /// Controls RX_DRQ sending behavior:
    /// - In DMA1 mode: sends DRQ when RFL >= trigger depth or on receive timeout
    /// - In DMA0 mode: if DMA_PTE_RX=1 and FIFO enabled, sends DRQ when RFL > trigger depth
    /// - Otherwise: sends DRQ once receive data is valid
    #[doc(alias = "DMA_PTE_RX")]
    #[inline]
    pub const fn set_dma_pte_rx(self, enable: bool) -> Self {
        Self(if enable {
            self.0 | Self::DMA_PTE_RX
        } else {
            self.0 & !Self::DMA_PTE_RX
        })
    }
    /// Check if the DMA_PTE_RX bit is set.
    #[inline]
    pub const fn is_dma_pte_rx_set(self) -> bool {
        (self.0 & Self::DMA_PTE_RX) != 0
    }
    /// Set the DMA_PTE_TX bit (`DMA_PTE_TX`).
    ///
    /// Controls TX_REQ sending behavior:
    /// - In DMA1 mode (FIFO enabled):
    ///     - If DMA_PTE_TX is set to 1, sends DMA request when TFL < trigger depth.
    ///     - If DMA_PTE_TX is set to 0, sends DMA request when FIFO is empty. DMA request stops when FIFO is full.
    /// - In DMA0 mode:
    ///     - If DMA_PTE_TX is set to 1 and FIFO is enabled, sends DMA request when TFL < trigger depth.
    ///     - If DMA_PTE_TX is set to 1 and FIFO is not enabled, sends DMA request when THRE is empty.
    ///     - If DMA_PTE_TX is set to 0, sends DMA request when FIFO is empty.
    #[doc(alias = "DMA_PTE_TX")]
    #[inline]
    pub const fn set_dma_pte_tx(self, enable: bool) -> Self {
        Self(if enable {
            self.0 | Self::DMA_PTE_TX
        } else {
            self.0 & !Self::DMA_PTE_TX
        })
    }
    /// Check if the DMA_PTE_TX bit is set.
    #[inline]
    pub const fn is_dma_pte_tx_set(self) -> bool {
        (self.0 & Self::DMA_PTE_TX) != 0
    }
    /// Set halt change update (`HALT_CHANGE_UPDATE`).
    #[inline]
    pub const fn set_halt_change_update(self, enable: bool) -> Self {
        if enable {
            Self(self.0 | Self::HALT_CHANGE_UPDATE)
        } else {
            Self(self.0 & !Self::HALT_CHANGE_UPDATE)
        }
    }
    /// Set halt change config at busy (`HALT_CHCFG_AT_BUSY`).
    #[inline]
    pub const fn set_halt_change_config_at_busy(self, enable: bool) -> Self {
        if enable {
            Self(self.0 | Self::HALT_CHCFG_AT_BUSY)
        } else {
            Self(self.0 & !Self::HALT_CHCFG_AT_BUSY)
        }
    }
    /// Halt transmission (`HALT_TX`).
    ///
    /// If FIFO is disabled, setting this bit has no effect.
    #[doc(alias = "HALT_TX")]
    #[inline]
    pub const fn halt_tx(self, enable: bool) -> Self {
        Self(if enable {
            self.0 | Self::HALT_TX
        } else {
            self.0 & !Self::HALT_TX
        })
    }
    /// Check if transmission is halted.
    #[inline]
    pub const fn is_tx_halted(self) -> bool {
        (self.0 & Self::HALT_TX) != 0
    }
}

/// UART debug dll register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DebugDll(u32);

impl DebugDll {
    const DEBUG_DLL: u32 = 0xFF;

    /// Set the debug dll value (`DEBUG_DLL`).
    #[doc(alias = "DEBUG_DLL")]
    #[inline]
    pub const fn set_debug_dll(self, val: u8) -> Self {
        Self((self.0 & !Self::DEBUG_DLL) | (Self::DEBUG_DLL & (val as u32)))
    }
    /// Get the debug dll value.
    #[inline]
    pub const fn debug_dll(self) -> u8 {
        (self.0 & Self::DEBUG_DLL) as u8
    }
}

/// UART debug dlh register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DebugDlh(u32);

impl DebugDlh {
    const DEBUG_DLH: u32 = 0xFF;

    /// Set the debug dlh value (`DEBUG_DLH`).
    #[doc(alias = "DEBUG_DLH")]
    #[inline]
    pub const fn set_debug_dlh(self, val: u8) -> Self {
        Self((self.0 & !Self::DEBUG_DLH) | (Self::DEBUG_DLH & (val as u32)))
    }
    /// Get the debug dlh value.
    #[inline]
    pub const fn debug_dlh(self) -> u8 {
        (self.0 & Self::DEBUG_DLH) as u8
    }
}

/// UART rs485 de time register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Rs485DeTime(u32);

impl Rs485DeTime {
    const DE_DAT: u32 = 0xF << 4;
    const DE_AT: u32 = 0xF;

    /// Set the driver enable de-assert time (`DE_DAT`).
    ///
    /// The time interval between the serial data stop bit and the falling edge of the DE signal, measured in serial clock cycles.
    #[doc(alias = "DE_DAT")]
    #[inline]
    pub const fn set_de_deassert_time(self, time: u8) -> Self {
        assert!(
            time < 16,
            "Driver enable de-assert time out of range (expected 0..=15)"
        );
        Self((self.0 & !Self::DE_DAT) | (Self::DE_DAT & ((time as u32) << 4)))
    }
    /// Get the driver enable de-assert time.
    #[inline]
    pub const fn de_deassert_time(self) -> u8 {
        ((self.0 & Self::DE_DAT) >> 4) as u8
    }
    /// Set the driver enable assert time (`DE_AT`).
    ///
    /// The time interval between the rising edge of the DE signal and the start bit of the serial data, measured in serial clock cycles.
    #[doc(alias = "DE_AT")]
    #[inline]
    pub const fn set_de_assert_time(self, time: u8) -> Self {
        assert!(
            time < 16,
            "Driver enable assert time out of range (expected 0..=15)"
        );
        Self((self.0 & !Self::DE_AT) | (Self::DE_AT & (time as u32)))
    }
    /// Get the driver enable assert time.
    #[inline]
    pub const fn de_assert_time(self) -> u8 {
        (self.0 & Self::DE_AT) as u8
    }
}

/// UART rs485 control mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Rs485ControlMode {
    Hardware = 0,
    Software = 1,
}

/// UART rs485 slave mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Rs485SlaveMode {
    /// Normal Multi-drop Mode (NMM).
    NMM = 0,
    /// Auto Address Detect Mode (AAD).
    AAD = 1,
}

/// UART rs485 control and status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Rs485Control(u32);

impl Rs485Control {
    const RS485_CTL_MODE: u32 = 0x1 << 7;
    const AAD_ADDR_F: u32 = 0x1 << 6;
    const RS485_ADDR_DET_F: u32 = 0x1 << 5;
    const RX_BF_ADDR: u32 = 0x1 << 3;
    const RX_AF_ADDR: u32 = 0x1 << 2;
    const RS485_SLAVE_MODE_SEL: u32 = 0x3;

    /// Set rs485 control mode (`RS485_CTL_MODE`).
    #[doc(alias = "RS485_CTL_MODE")]
    #[inline]
    pub const fn set_rs485_control_mode(self, mode: Rs485ControlMode) -> Self {
        Self((self.0 & !Self::RS485_CTL_MODE) | ((mode as u32) << 7))
    }
    /// Get rs485 control mode.
    #[inline]
    pub const fn rs485_control_mode(self) -> Rs485ControlMode {
        match (self.0 & Self::RS485_CTL_MODE) >> 7 {
            0 => Rs485ControlMode::Hardware,
            1 => Rs485ControlMode::Software,
            _ => unreachable!(),
        }
    }
    /// Check if address is matched (`AAD_ADDR_F`).
    #[doc(alias = "AAD_ADDR_F")]
    #[inline]
    pub const fn is_rs485_address_matched(self) -> bool {
        (self.0 & Self::AAD_ADDR_F) != 0
    }
    /// Clear rs485 address match flag.
    #[inline]
    pub const fn clear_rs485_address_matched(self) -> Self {
        Self(self.0 | Self::AAD_ADDR_F)
    }
    /// Check if address is detected (`RS485_ADDR_DET_F`).
    #[doc(alias = "RS485_ADDR_DET_F")]
    #[inline]
    pub const fn is_rs485_address_detected(self) -> bool {
        (self.0 & Self::RS485_ADDR_DET_F) != 0
    }
    /// Clear rs485 address detected flag.
    #[inline]
    pub const fn clear_rs485_address_detected(self) -> Self {
        Self(self.0 | Self::RS485_ADDR_DET_F)
    }
    /// Enable receive all data before receiving a address (`RX_BF_ADDR`).
    #[doc(alias = "RX_BF_ADDR")]
    #[inline]
    pub const fn enable_receive_all_before_addr(self) -> Self {
        Self(self.0 | Self::RX_BF_ADDR)
    }
    /// Disable receive all data before receiving a address.
    #[inline]
    pub const fn disable_receive_all_before_addr(self) -> Self {
        Self(self.0 & !Self::RX_BF_ADDR)
    }
    /// Check if receive all data before receiving a address is enabled.
    #[inline]
    pub const fn is_receive_all_before_addr_enabled(self) -> bool {
        (self.0 & Self::RX_BF_ADDR) != 0
    }
    /// Enable receive all data after receiving a address (`RX_AF_ADDR`).
    #[doc(alias = "RX_AF_ADDR")]
    #[inline]
    pub const fn enable_receive_all_after_addr(self) -> Self {
        Self(self.0 | Self::RX_AF_ADDR)
    }
    /// Disable receive all data after receiving a address.
    #[inline]
    pub const fn disable_receive_all_after_addr(self) -> Self {
        Self(self.0 & !Self::RX_AF_ADDR)
    }
    /// Check if receive all data after receiving a address is enabled.
    #[inline]
    pub const fn is_receive_all_after_addr_enabled(self) -> bool {
        (self.0 & Self::RX_AF_ADDR) != 0
    }
    /// Set rs485 slave mode (`RS485_SLAVE_MODE_SEL`).
    #[doc(alias = "RS485_SLAVE_MODE_SEL")]
    #[inline]
    pub const fn set_rs485_slave_mode(self, mode: Rs485SlaveMode) -> Self {
        Self((self.0 & !Self::RS485_SLAVE_MODE_SEL) | ((mode as u32) & Self::RS485_SLAVE_MODE_SEL))
    }
    /// Get rs485 slave mode.
    #[inline]
    pub const fn rs485_slave_mode(self) -> Rs485SlaveMode {
        match self.0 & Self::RS485_SLAVE_MODE_SEL {
            0 => Rs485SlaveMode::NMM,
            1 => Rs485SlaveMode::AAD,
            _ => unreachable!(),
        }
    }
}

/// UART RS485 address match register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Rs485AddressMatch(u32);

impl Rs485AddressMatch {
    const ADDR_MATCH: u32 = 0xFF;

    /// Set match address (`ADDR_MATCH`).
    ///
    /// Valid only in AAD mode.
    #[doc(alias = "ADDR_MATCH")]
    #[inline]
    pub const fn set_match_address(self, addr: u32) -> Self {
        Self((self.0 & !Self::ADDR_MATCH) | (addr & Self::ADDR_MATCH))
    }
    /// Get match address.
    #[inline]
    pub const fn match_address(self) -> u32 {
        self.0 & Self::ADDR_MATCH
    }
}

/// UART RS485 bus status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Rs485BusStatus {
    /// Bus is idle.
    Idle = 0,
    /// Bus is busy.
    Busy = 1,
}

/// UART RS485 bus idle check register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Rs485BusIdleCheck(u32);

impl Rs485BusIdleCheck {
    const BUS_IDLE_CHK_EN: u32 = 0x1 << 7;
    const BUS_STATUS: u32 = 0x1 << 6;
    const ADJ_TIME: u32 = 0x3F;

    /// Enable bus idle check (`BUS_IDLE_CHK_EN`).
    #[doc(alias = "BUS_IDLE_CHK_EN")]
    #[inline]
    pub const fn enable_bus_idle_check(self) -> Self {
        Self(self.0 | Self::BUS_IDLE_CHK_EN)
    }
    /// Disable bus idle check.
    #[inline]
    pub const fn disable_bus_idle_check(self) -> Self {
        Self(self.0 & !Self::BUS_IDLE_CHK_EN)
    }
    /// Check if bus idle check is enabled.
    #[inline]
    pub const fn is_bus_idle_check_enabled(self) -> bool {
        (self.0 & Self::BUS_IDLE_CHK_EN) != 0
    }
    /// Get bus status (`BUS_STATUS`).
    #[doc(alias = "BUS_STATUS")]
    #[inline]
    pub const fn bus_status(self) -> Rs485BusStatus {
        match (self.0 & Self::BUS_STATUS) >> 6 {
            0 => Rs485BusStatus::Idle,
            1 => Rs485BusStatus::Busy,
            _ => unreachable!(),
        }
    }
    /// Get the bus idle time.
    ///
    /// The bus idle time, where each unit represents 8 × 16 × Tclk.
    #[doc(alias = "BUS_IDLE_TIME")]
    #[inline]
    pub const fn bus_idle_time(self) -> u32 {
        self.0 & Self::ADJ_TIME
    }
}

/// UART transmit delay register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TransmitDelay(u32);

impl TransmitDelay {
    const TX_DLY: u32 = 0xFF;

    /// Set the transmit delay value (`TX_DLY`).
    ///
    /// The delay time between the last stop bit and the next start bit. The unit is 16 × Tclk.
    /// It is used to control the interval between two bytes during data transmission.
    #[doc(alias = "TX_DLY")]
    #[inline]
    pub const fn set_transmit_delay(self, val: u8) -> Self {
        Self((self.0 & !Self::TX_DLY) | (Self::TX_DLY & (val as u32)))
    }
    /// Get the transmit delay value.
    #[inline]
    pub const fn transmit_delay(self) -> u8 {
        (self.0 & Self::TX_DLY) as u8
    }
}

/// UART debug register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct DebugRegister(u32);

impl DebugRegister {
    const BYPASS_DEBUG: u32 = 0x1 << 3;

    /// Set the bypass prefetch debugging.
    #[inline]
    pub const fn set_bypass_debug(self, enable: bool) -> Self {
        if enable {
            Self(self.0 | Self::BYPASS_DEBUG)
        } else {
            Self(self.0 & !Self::BYPASS_DEBUG)
        }
    }
}

/// UART version register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Version(u32);

impl Version {
    const VERSION: u32 = 0xFF;

    /// Get the version value (`VERSION`).
    #[doc(alias = "VERSION")]
    #[inline]
    pub const fn version(self) -> u8 {
        (self.0 & Self::VERSION) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DebugDlh, DebugDll, DebugRegister, DmaHandshakeConfig, DmaHandshakeMode, HaltTx,
        ReceiveFifoLevel, RegisterBlock, Rs485AddressMatch, Rs485BusIdleCheck, Rs485BusStatus,
        Rs485Control, Rs485ControlMode, Rs485DeTime, Rs485SlaveMode, TransmitDelay,
        TransmitFifoLevel, UartScratch, UartStatus, Version,
    };
    use crate::test_should_panic;
    use core::mem::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, uart16550), 0x0);
        assert_eq!(offset_of!(RegisterBlock, sch), 0x1C);
        assert_eq!(offset_of!(RegisterBlock, usr), 0x7C);
        assert_eq!(offset_of!(RegisterBlock, tfl), 0x80);
        assert_eq!(offset_of!(RegisterBlock, rfl), 0x84);
        assert_eq!(offset_of!(RegisterBlock, hsk), 0x88);
        assert_eq!(offset_of!(RegisterBlock, halt), 0xA4);
        assert_eq!(offset_of!(RegisterBlock, dbg_dll), 0xB0);
        assert_eq!(offset_of!(RegisterBlock, dbg_dlh), 0xB4);
        assert_eq!(offset_of!(RegisterBlock, rs485_de), 0xB8);
        assert_eq!(offset_of!(RegisterBlock, rs485_ctl), 0xC0);
        assert_eq!(offset_of!(RegisterBlock, rs485_addr_match), 0xC4);
        assert_eq!(offset_of!(RegisterBlock, rs485_bus_idle_check), 0xC8);
        assert_eq!(offset_of!(RegisterBlock, tx_delay), 0xCC);
        assert_eq!(offset_of!(RegisterBlock, debug), 0xD0);
        assert_eq!(offset_of!(RegisterBlock, version), 0xFC);
    }

    #[test]
    fn struct_uart_scratch_functions() {
        let val = UartScratch(0).set_scratch_val(0xFF);
        assert_eq!(val.scratch_val(), 0xFF);
        assert_eq!(val.0, 0x0000_00FF);
    }

    #[test]
    fn struct_uart_status_functions() {
        let val = UartStatus(0x1F);
        assert!(val.is_receive_fifo_full());
        assert!(val.is_receive_fifo_not_empty());
        assert!(val.is_transmit_fifo_empty());
        assert!(val.is_transmit_fifo_not_full());
        assert!(val.is_busy());
    }

    #[test]
    fn struct_transmit_fifo_level_functions() {
        let val = TransmitFifoLevel(0x1FF);
        assert_eq!(val.tx_level(), 0x1FF);
    }

    #[test]
    fn struct_receive_fifo_level_functions() {
        let val = ReceiveFifoLevel(0x1FF);
        assert_eq!(val.rx_level(), 0x1FF);
    }

    #[test]
    fn struct_dma_handshake_config_functions() {
        let mut val = DmaHandshakeConfig(0);
        val = val.set_handshake_mode(DmaHandshakeMode::WaitCycle);
        assert_eq!(val.handshake_mode(), DmaHandshakeMode::WaitCycle);
        assert_eq!(val.0, 0x0000_00A5);

        val = val.set_handshake_mode(DmaHandshakeMode::Handshake);
        assert_eq!(val.handshake_mode(), DmaHandshakeMode::Handshake);
        assert_eq!(val.0, 0x0000_00E5);
    }

    #[test]
    fn struct_halt_tx_functions() {
        let mut val = HaltTx(0);
        val = val.set_dma_pte_rx(true);
        assert!(val.is_dma_pte_rx_set());
        assert_eq!(val.0, 0x0000_0080);

        val = val.set_dma_pte_rx(false);
        assert!(!val.is_dma_pte_rx_set());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_dma_pte_tx(true);
        assert!(val.is_dma_pte_tx_set());
        assert_eq!(val.0, 0x0000_0040);

        val = val.set_dma_pte_tx(false);
        assert!(!val.is_dma_pte_tx_set());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_halt_change_update(true);
        assert_eq!(val.0, 0x0000_0004);
        val = val.set_halt_change_update(false);
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_halt_change_config_at_busy(true);
        assert_eq!(val.0, 0x0000_0002);

        val = val.set_halt_change_config_at_busy(false);
        assert_eq!(val.0, 0x0000_0000);

        val = val.halt_tx(true);
        assert!(val.is_tx_halted());
        assert_eq!(val.0, 0x0000_0001);

        val = val.halt_tx(false);
        assert!(!val.is_tx_halted());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_debug_dll_functions() {
        let val = DebugDll(0).set_debug_dll(0xFF);
        assert_eq!(val.debug_dll(), 0xFF);
        assert_eq!(val.0, 0x0000_00FF);
    }

    #[test]
    fn struct_debug_dlh_functions() {
        let val = DebugDlh(0).set_debug_dlh(0xFF);
        assert_eq!(val.debug_dlh(), 0xFF);
        assert_eq!(val.0, 0x0000_00FF);
    }

    #[test]
    fn struct_rs485_de_time_functions() {
        let mut val = Rs485DeTime(0);
        val = val.set_de_deassert_time(0xF);
        assert_eq!(val.de_deassert_time(), 0xF);
        assert_eq!(val.0, 0x0000_00F0);

        val = Rs485DeTime(0).set_de_assert_time(0xF);
        assert_eq!(val.de_assert_time(), 0xF);
        assert_eq!(val.0, 0x0000_000F);
    }

    test_should_panic!(
        (
            test_rs485_set_de_deassert_time_panic,
            Rs485DeTime(0).set_de_deassert_time(16),
            "Driver enable de-assert time out of range (expected 0..=15)"
        ),
        (
            test_rs485_set_de_assert_time_panic,
            Rs485DeTime(0).set_de_assert_time(16),
            "Driver enable assert time out of range (expected 0..=15)"
        ),
    );

    #[test]
    fn struct_rs485_control_functions() {
        let mut val = Rs485Control(0);
        val = val.set_rs485_control_mode(Rs485ControlMode::Software);
        assert_eq!(val.rs485_control_mode(), Rs485ControlMode::Software);
        assert_eq!(val.0, 0x0000_0080);

        val = val.set_rs485_control_mode(Rs485ControlMode::Hardware);
        assert_eq!(val.rs485_control_mode(), Rs485ControlMode::Hardware);
        assert_eq!(val.0, 0x0000_0000);

        val = Rs485Control(0x0000_0040);
        assert!(val.is_rs485_address_matched());
        val = Rs485Control(0x0).clear_rs485_address_matched();
        assert_eq!(val.0, 0x0000_0040);

        val = Rs485Control(0x0000_0020);
        assert!(val.is_rs485_address_detected());
        val = Rs485Control(0x0).clear_rs485_address_detected();
        assert_eq!(val.0, 0x0000_0020);

        val = Rs485Control(0x0).enable_receive_all_before_addr();
        assert!(val.is_receive_all_before_addr_enabled());
        assert_eq!(val.0, 0x0000_0008);

        val = val.disable_receive_all_before_addr();
        assert!(!val.is_receive_all_before_addr_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = Rs485Control(0x0).enable_receive_all_after_addr();
        assert!(val.is_receive_all_after_addr_enabled());
        assert_eq!(val.0, 0x0000_0004);

        val = val.disable_receive_all_after_addr();
        assert!(!val.is_receive_all_after_addr_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.set_rs485_slave_mode(Rs485SlaveMode::AAD);
        assert_eq!(val.rs485_slave_mode(), Rs485SlaveMode::AAD);
        assert_eq!(val.0, 0x0000_0001);

        val = val.set_rs485_slave_mode(Rs485SlaveMode::NMM);
        assert_eq!(val.rs485_slave_mode(), Rs485SlaveMode::NMM);
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_rs485_address_match_functions() {
        let val = Rs485AddressMatch(0).set_match_address(0xFF);
        assert_eq!(val.match_address(), 0xFF);
        assert_eq!(val.0, 0x0000_00FF);
    }

    #[test]
    fn struct_rs485_bus_idle_check_functions() {
        let mut val = Rs485BusIdleCheck(0);
        val = val.enable_bus_idle_check();
        assert!(val.is_bus_idle_check_enabled());
        assert_eq!(val.0, 0x0000_0080);

        val = val.disable_bus_idle_check();
        assert!(!val.is_bus_idle_check_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = Rs485BusIdleCheck(0x0000_0040);
        assert_eq!(val.bus_status(), Rs485BusStatus::Busy);
        val = Rs485BusIdleCheck(0x0);
        assert_eq!(val.bus_status(), Rs485BusStatus::Idle);

        val = Rs485BusIdleCheck(0x0000_003F);
        assert_eq!(val.bus_idle_time(), 0x3F);
    }

    #[test]
    fn struct_transmit_delay_functions() {
        let val = TransmitDelay(0).set_transmit_delay(0xFF);
        assert_eq!(val.transmit_delay(), 0xFF);
        assert_eq!(val.0, 0x0000_00FF);
    }

    #[test]
    fn struct_debug_register_functions() {
        let mut val = DebugRegister(0).set_bypass_debug(true);
        assert_eq!(val.0, 0x0000_0008);
        val = val.set_bypass_debug(false);
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_version_functions() {
        let val = Version(0xFF);
        assert_eq!(val.version(), 0xFF);
    }
}
