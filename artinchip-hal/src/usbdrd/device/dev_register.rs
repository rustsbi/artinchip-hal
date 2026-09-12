//! USBDRD device register blocks and registers.

use volatile_register::{RO, RW};

/// USB Device Controller registers.
#[repr(C)]
pub struct RegisterBlock {
    /// AHB Basic Configuration register (`AHB_BASIC`).
    #[doc(alias = "AHB_BASIC")]
    pub ahb_basic: RW<AhbBasic>,
    /// USB Device Init register (`USB_DEV_INIT`).
    #[doc(alias = "USB_DEV_INIT")]
    pub usb_dev_init: RW<UsbDevInit>,
    /// USB PHY Interface register (`USB_PHY_IF`).
    #[doc(alias = "USB_PHY_IF")]
    pub usb_phy_if: RW<UsbPhyIf>,
    /// USB ULPI PHY Configuration register (`USB_ULPI_PHY`).
    #[doc(alias = "USB_ULPI_PHY")]
    pub usb_ulpi_phy: RW<u32>,
    /// USB Interrupt Status register (`USB_INT_STS`).
    #[doc(alias = "USB_INT_STS")]
    pub usb_int_sts: RW<UsbIntSts>,
    /// USB Interrupt Mask register (`USB_INT_MSK`).
    #[doc(alias = "USB_INT_MSK")]
    pub usb_int_msk: RW<UsbIntMsk>,
    /// Receive FIFO Size register (`RXFIFO_SIZ`).
    #[doc(alias = "RXFIFO_SIZ")]
    pub rxfifo_siz: RW<RxFifoSiz>,
    /// Receive FIFO Status register (`RXFIFO_STS`).
    #[doc(alias = "RXFIFO_STS")]
    pub rxfifo_sts: RO<RxFifoSts>,
    /// Non-periodic TXFIFO Size register (`NPTXFIFO_SIZ`).
    #[doc(alias = "NPTXFIFO_SIZ")]
    pub nptxfifo_siz: RW<NpTxFifoSiz>,
    /// Non-periodic TXFIFO Status register (`NPTXFIFO_STS`).
    #[doc(alias = "NPTXFIFO_STS")]
    pub nptxfifo_sts: RO<NpTxFifoSts>,
    /// Periodic TXFIFO Size register.
    pub ptxfifo_siz: [RW<PTxFifoSiz>; 2],
    /// RXFIFO Status Pop register (`RXFIFO_STS_POP`).
    #[doc(alias = "RXFIFO_STS_POP")]
    pub rxfifo_sts_pop: RO<RxFifoSts>,
    _reserved0: [u8; 0xC],
    /// PHY Clock Control register (`PHY_CLK_CTL`).
    #[doc(alias = "PHY_CLK_CTL")]
    pub phy_clk_ctl: RW<PhyClkCtl>,
    _reserved1: [u8; 0x1BC],
    /// USB Device Configuration register (`USB_DEV_CONF`).
    #[doc(alias = "USB_DEV_CONF")]
    pub usb_dev_conf: RW<UsbDevConf>,
    /// USB Device Function register (`USB_DEV_FUNC`).
    #[doc(alias = "USB_DEV_FUNC")]
    pub usb_dev_func: RW<UsbDevFunc>,
    /// USB Line Status register (`USB_LINE_STS`).
    #[doc(alias = "USB_LINE_STS")]
    pub usb_line_sts: RO<UsbLineSts>,
    /// IN endpoint interrupt mask register (`INEP_INT_MSK`).
    #[doc(alias = "INEP_INT_MSK")]
    pub inep_int_msk: RW<InEpIntMsk>,
    /// OUT endpoint interrupt mask register (`OUTEP_INT_MSK`).
    #[doc(alias = "OUTEP_INT_MSK")]
    pub outep_int_msk: RW<OutEpIntMsk>,
    /// USB Endpoint Interrupt register (`USB_EP_INT`).
    #[doc(alias = "USB_EP_INT")]
    pub usb_ep_int: RO<UsbEpInt>,
    /// USB Endpoint Interrupt Mask register (`USB_EP_INT_MSK`).
    #[doc(alias = "USB_EP_INT_MSK")]
    pub usb_ep_int_msk: RW<UsbEpIntMsk>,
    _reserved2: [u8; 0x4],
    /// IN Endpoint Configuration registers.
    pub in_ep_cfg: [RW<InEpCfg>; 5],
    _reserved3: [u8; 0xC],
    /// OUT Endpoint Configuration registers.
    pub out_ep_cfg: [RW<OutEpCfg>; 5],
    _reserved4: [u8; 0xC],
    /// IN Endpoint Interrupt registers.
    pub in_ep_int: [RW<InEpInt>; 5],
    _reserved5: [u8; 0xC],
    /// OUT Endpoint Interrupt registers.
    pub out_ep_int: [RW<OutEpInt>; 5],
    _reserved6: [u8; 0xC],
    /// IN endpoint transfer size registers (EP0 through EP4).
    pub in_ep_tsf_siz: [RW<InEpTsfSiz>; 5],
    _reserved7: [u8; 0xC],
    /// OUT endpoint transfer size registers (EP0 through EP4).
    pub out_ep_tsf_siz: [RW<OutEpTsfSiz>; 5],
    _reserved8: [u8; 0x2C],
    /// IN Endpoint DMA Address registers.
    pub in_ep_dma: [RW<u32>; 5],
    _reserved9: [u8; 0xC],
    /// OUT Endpoint DMA Address registers.
    pub out_ep_dma: [RW<u32>; 5],
    _reserved10: [u8; 0xC],
    /// IN Endpoint TXFIFO Status registers.
    pub in_ep_txfifo_sta: [RO<InEpTxFifoSta>; 5],
    _reserved11: [u8; 0xC],
    /// Token Queue registers.
    pub tkn_queue: [RO<u32>; 4],
    _reserved12: [u8; 0xC8C],
    /// USB Device Version register (`USB_DEV_VERSION`).
    #[doc(alias = "USB_DEV_VERSION")]
    pub version: RO<u32>,
}

/// AhbBasic register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AhbBasic(u32);

impl AhbBasic {
    /// Zero value.
    pub const fn zero(self) -> Self {
        Self(0)
    }

    const NOTI_ALL_DMA_WRIT: u32 = 1 << 8;
    const REM_MEM_SUPP: u32 = 1 << 7;
    const INV_DESC_ENDIANNESS: u32 = 1 << 6;
    const AHB_SINGLE: u32 = 1 << 5;
    const TXENDDELAY: u32 = 1 << 3;
    const AHBIDLE: u32 = 1 << 2;
    const DMAREQ: u32 = 1 << 1;

    /// Set notification all DMA write bit (`NOTI_ALL_DMA_WRIT`).
    #[doc(alias = "NOTI_ALL_DMA_WRIT")]
    #[inline]
    pub const fn set_noti_all_dma_writ(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::NOTI_ALL_DMA_WRIT)
        } else {
            Self(self.0 & !Self::NOTI_ALL_DMA_WRIT)
        }
    }
    /// Get notification all DMA write bit.
    #[inline]
    pub const fn noti_all_dma_writ(self) -> bool {
        (self.0 & Self::NOTI_ALL_DMA_WRIT) != 0
    }
    /// Set remote memory support bit (`REM_MEM_SUPP`).
    #[doc(alias = "REM_MEM_SUPP")]
    #[inline]
    pub const fn set_rem_mem_supp(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::REM_MEM_SUPP)
        } else {
            Self(self.0 & !Self::REM_MEM_SUPP)
        }
    }
    /// Get remote memory support bit.
    #[inline]
    pub const fn rem_mem_supp(self) -> bool {
        (self.0 & Self::REM_MEM_SUPP) != 0
    }
    /// Set inverse descriptor endianness bit (`INV_DESC_ENDIANNESS`).
    #[doc(alias = "INV_DESC_ENDIANNESS")]
    #[inline]
    pub const fn set_inv_desc_endianness(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::INV_DESC_ENDIANNESS)
        } else {
            Self(self.0 & !Self::INV_DESC_ENDIANNESS)
        }
    }
    /// Get inverse descriptor endianness bit.
    #[inline]
    pub const fn inv_desc_endianness(self) -> bool {
        (self.0 & Self::INV_DESC_ENDIANNESS) != 0
    }
    /// Enable AHB single transfer (`AHB_SINGLE`).
    #[doc(alias = "AHB_SINGLE")]
    #[inline]
    pub const fn enable_ahb_single(self) -> Self {
        Self(self.0 | Self::AHB_SINGLE)
    }
    /// Disable AHB single transfer.
    #[inline]
    pub const fn disable_ahb_single(self) -> Self {
        Self(self.0 & !Self::AHB_SINGLE)
    }
    /// Check if AHB single transfer is enabled.
    #[inline]
    pub const fn is_ahb_single_enabled(self) -> bool {
        (self.0 & Self::AHB_SINGLE) != 0
    }
    /// Set TX end delay bit (`TXENDDELAY`).
    #[doc(alias = "TXENDDELAY")]
    #[inline]
    pub const fn set_tx_end_delay(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::TXENDDELAY)
        } else {
            Self(self.0 & !Self::TXENDDELAY)
        }
    }
    /// Get TX end delay bit.
    #[inline]
    pub const fn tx_end_delay(self) -> bool {
        (self.0 & Self::TXENDDELAY) != 0
    }
    /// Check if AHB is idle (`AHBIDLE`).
    #[doc(alias = "AHBIDLE")]
    #[inline]
    pub const fn is_ahb_idle(self) -> bool {
        (self.0 & Self::AHBIDLE) != 0
    }
    /// Check if DMA is requested (`DMAREQ`).
    #[doc(alias = "DMAREQ")]
    #[inline]
    pub const fn is_dma_req(self) -> bool {
        (self.0 & Self::DMAREQ) != 0
    }
}

/// DMA burst length.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DmaBurstLength {
    Single = 0,
    INCR = 1,
    INCR4 = 3,
    INCR8 = 5,
    INCR16 = 7,
}

/// Non-Periodic TXFIFO threshold.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NpTxFifoThres {
    HalfEmpty,
    FullEmpty,
}

/// TXFIFO selected for a flush operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TxFifoToClear {
    /// Non-periodic TXFIFO.
    NonPeriodic = 0,
    /// Periodic TXFIFO 1.
    Periodic1 = 1,
    /// Periodic TXFIFO 2.
    Periodic2 = 2,
    /// All TXFIFOs.
    All = 0x10,
}

/// UsbDevInit register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UsbDevInit(u32);

impl UsbDevInit {
    /// Zero value.
    pub const fn zero(self) -> Self {
        Self(0)
    }

    const DMA_BL: u32 = 0xF << 12;
    const EN_DMA: u32 = 1 << 11;
    const NP_TFTH: u32 = 1 << 10;
    const GIE: u32 = 1 << 9;
    const C_IN_TKN_Q: u32 = 1 << 8;
    const C_TX_NUM: u32 = 0x1F << 3;
    const C_TXFIFO: u32 = 1 << 2;
    const C_RXFIFO: u32 = 1 << 1;
    const C_SFTRST: u32 = 1;

    /// Set DMA burst length (`DMA_BL`).
    #[doc(alias = "DMA_BL")]
    #[inline]
    pub const fn set_dma_burst_length(self, burst_length: DmaBurstLength) -> Self {
        Self((self.0 & !Self::DMA_BL) | (Self::DMA_BL & ((burst_length as u32) << 12)))
    }
    /// Get DMA burst length.
    #[inline]
    pub const fn dma_burst_length(self) -> DmaBurstLength {
        match (self.0 & Self::DMA_BL) >> 12 {
            0 => DmaBurstLength::Single,
            1 => DmaBurstLength::INCR,
            3 => DmaBurstLength::INCR4,
            5 => DmaBurstLength::INCR8,
            _ => DmaBurstLength::INCR16,
        }
    }
    /// Enable DMA (`EN_DMA`).
    #[doc(alias = "EN_DMA")]
    pub const fn enable_dma(self) -> Self {
        Self(self.0 | Self::EN_DMA)
    }
    /// Disable DMA.
    #[inline]
    pub const fn disable_dma(self) -> Self {
        Self(self.0 & !Self::EN_DMA)
    }
    /// Check if DMA is enabled.
    #[inline]
    pub const fn is_dma_enabled(self) -> bool {
        (self.0 & Self::EN_DMA) != 0
    }
    /// Set non-periodic TXFIFO threshold (`NP_TFTH`).
    #[doc(alias = "NP_TFTH")]
    #[inline]
    pub const fn set_np_txfifo_threshold(self, threshold: NpTxFifoThres) -> Self {
        Self((self.0 & !Self::NP_TFTH) | ((threshold as u32) << 10))
    }
    /// Get non-periodic TXFIFO threshold.
    #[inline]
    pub const fn np_txfifo_threshold(self) -> NpTxFifoThres {
        if self.0 & Self::NP_TFTH != 0 {
            NpTxFifoThres::FullEmpty
        } else {
            NpTxFifoThres::HalfEmpty
        }
    }
    /// Enable global interrupt (`GIE`).
    #[doc(alias = "GIE")]
    #[inline]
    pub const fn enable_global_interrupt(self) -> Self {
        Self(self.0 | Self::GIE)
    }
    /// Disable global interrupt.
    #[inline]
    pub const fn disable_global_interrupt(self) -> Self {
        Self(self.0 & !Self::GIE)
    }
    /// Check whether global interrupt is enabled.
    #[inline]
    pub const fn is_global_interrupt_enabled(self) -> bool {
        self.0 & Self::GIE != 0
    }
    /// Clear the IN token queue (`C_IN_TKN_Q`).
    #[doc(alias = "C_IN_TKN_Q")]
    #[inline]
    pub const fn clear_in_token_queue(self) -> Self {
        Self(self.0 | Self::C_IN_TKN_Q)
    }
    /// Set TXFIFO number to clear (`C_TX_NUM`).
    #[doc(alias = "C_TX_NUM")]
    #[inline]
    pub const fn set_clear_txfifo_number(self, fifo: TxFifoToClear) -> Self {
        Self((self.0 & !Self::C_TX_NUM) | ((fifo as u32) << 3))
    }
    /// Get TXFIFO number selected for clearing.
    #[inline]
    pub const fn clear_txfifo_number(self) -> TxFifoToClear {
        match (self.0 & Self::C_TX_NUM) >> 3 {
            0 => TxFifoToClear::NonPeriodic,
            1 => TxFifoToClear::Periodic1,
            2 => TxFifoToClear::Periodic2,
            0x10 => TxFifoToClear::All,
            _ => panic!("Invalid TXFIFO selection"),
        }
    }
    /// Clear the selected TXFIFO (`C_TXFIFO`).
    #[doc(alias = "C_TXFIFO")]
    #[inline]
    pub const fn clear_txfifo(self) -> Self {
        Self(self.0 | Self::C_TXFIFO)
    }
    /// Check if the selected TXFIFO flush is still in progress.
    #[inline]
    pub const fn is_txfifo_flushing(self) -> bool {
        self.0 & Self::C_TXFIFO != 0
    }
    /// Clear RXFIFO (`C_RXFIFO`).
    #[doc(alias = "C_RXFIFO")]
    #[inline]
    pub const fn clear_rxfifo(self) -> Self {
        Self(self.0 | Self::C_RXFIFO)
    }
    /// Check if the RXFIFO flush is still in progress.
    #[inline]
    pub const fn is_rxfifo_flushing(self) -> bool {
        self.0 & Self::C_RXFIFO != 0
    }
    /// Trigger USB device soft reset (`C_SFTRST`).
    #[doc(alias = "C_SFTRST")]
    #[inline]
    pub const fn soft_reset(self) -> Self {
        Self(self.0 | Self::C_SFTRST)
    }
    /// Check if the device soft reset is in progress.
    #[inline]
    pub const fn is_soft_reset(self) -> bool {
        self.0 & Self::C_SFTRST != 0
    }
}

/// PHY low-power clock source.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PhyLpClk {
    /// 480 MHz internal PLL clock.
    InternalPll480M,
    /// 48 MHz external clock.
    ExternalClk48M,
}

/// ULPI interface data rate and bus width.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum UlpiDataRate {
    /// Single-edge data rate with an 8-bit data bus.
    SingleEdge8Bit,
    /// Double-edge data rate with a 4-bit data bus.
    DoubleEdge4Bit,
}

/// PHY interface type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PhyInterface {
    /// UTMI+ interface.
    UtmiPlus,
    /// ULPI interface.
    Ulpi,
}

/// UTMI+ PHY interface width.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum UtmiPhyInterfaceWidth {
    /// 8-bit interface.
    Bits8,
    /// 16-bit interface.
    Bits16,
}

/// USB PHY Interface register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UsbPhyIf(u32);

impl UsbPhyIf {
    /// Zero value.
    pub const fn zero(self) -> Self {
        Self(0)
    }

    const ULPI_CLK_SUS: u32 = 1 << 19;
    const ULPI_AUTO_RES: u32 = 1 << 18;
    const PHY_LP_CLK_SET: u32 = 1 << 15;
    const TA_TIME: u32 = 0xF << 10;
    const SEL_ULPI_DDR: u32 = 1 << 7;
    const SEL_PHY_IF: u32 = 1 << 4;
    const PHY_IF_WIDTH: u32 = 1 << 3;
    const TIMEOUT_CAL: u32 = 0x7;

    /// Set ULPI clock suspend bit (`ULPI_CLK_SUS`).
    #[doc(alias = "ULPI_CLK_SUS")]
    #[inline]
    pub const fn set_ulpi_clk_suspend(self, set: bool) -> Self {
        Self((self.0 & !Self::ULPI_CLK_SUS) | if set { Self::ULPI_CLK_SUS } else { 0 })
    }
    /// Get ULPI clock suspend bit.
    #[inline]
    pub const fn ulpi_clk_suspend(self) -> bool {
        self.0 & Self::ULPI_CLK_SUS != 0
    }
    /// Enable ULPI automatic resume (`ULPI_AUTO_RES`).
    #[doc(alias = "ULPI_AUTO_RES")]
    #[inline]
    pub const fn enable_ulpi_auto_resume(self) -> Self {
        Self(self.0 | Self::ULPI_AUTO_RES)
    }
    /// Disable ULPI automatic resume.
    #[inline]
    pub const fn disable_ulpi_auto_resume(self) -> Self {
        Self(self.0 & !Self::ULPI_AUTO_RES)
    }
    /// Check if ULPI automatic resume is enabled.
    #[inline]
    pub const fn is_ulpi_auto_resume_enabled(self) -> bool {
        self.0 & Self::ULPI_AUTO_RES != 0
    }
    /// Set PHY low-power clock source (`PHY_LP_CLK_SET`).
    #[doc(alias = "PHY_LP_CLK_SET")]
    #[inline]
    pub const fn set_phy_lp_clock(self, clock: PhyLpClk) -> Self {
        Self((self.0 & !Self::PHY_LP_CLK_SET) | ((clock as u32) << 15))
    }
    /// Get PHY low-power clock source.
    #[inline]
    pub const fn phy_lp_clock(self) -> PhyLpClk {
        match (self.0 & Self::PHY_LP_CLK_SET) >> 15 {
            0 => PhyLpClk::InternalPll480M,
            _ => PhyLpClk::ExternalClk48M,
        }
    }
    /// Set turnaround time (`TA_TIME`).
    #[doc(alias = "TA_TIME")]
    #[inline]
    pub const fn set_ta_time(self, val: u8) -> Self {
        assert!(val < 0x10, "Turnaround time out of range (expected 0..=15)");
        Self((self.0 & !Self::TA_TIME) | (((val as u32) << 10) & Self::TA_TIME))
    }
    /// Get turnaround time.
    #[inline]
    pub const fn ta_time(self) -> u8 {
        ((self.0 & Self::TA_TIME) >> 10) as u8
    }
    /// Set ULPI data rate and bus width (`SEL_ULPI_DDR`).
    #[doc(alias = "SEL_ULPI_DDR")]
    #[inline]
    pub const fn set_ulpi_data_rate(self, rate: UlpiDataRate) -> Self {
        Self((self.0 & !Self::SEL_ULPI_DDR) | ((rate as u32) << 7))
    }
    /// Get ULPI data rate and bus width.
    #[inline]
    pub const fn ulpi_data_rate(self) -> UlpiDataRate {
        match (self.0 & Self::SEL_ULPI_DDR) >> 7 {
            0 => UlpiDataRate::SingleEdge8Bit,
            _ => UlpiDataRate::DoubleEdge4Bit,
        }
    }
    /// Set PHY interface type (`SEL_PHY_IF`).
    #[doc(alias = "SEL_PHY_IF")]
    #[inline]
    pub const fn set_phy_interface(self, interface: PhyInterface) -> Self {
        Self((self.0 & !Self::SEL_PHY_IF) | ((interface as u32) << 4))
    }
    /// Get PHY interface type.
    #[inline]
    pub const fn phy_interface(self) -> PhyInterface {
        match (self.0 & Self::SEL_PHY_IF) >> 4 {
            0 => PhyInterface::UtmiPlus,
            _ => PhyInterface::Ulpi,
        }
    }
    /// Set UTMI+ PHY interface width (`PHY_IF_WIDTH`).
    #[doc(alias = "PHY_IF_WIDTH")]
    #[inline]
    pub const fn set_utmi_phy_interface_width(self, width: UtmiPhyInterfaceWidth) -> Self {
        Self((self.0 & !Self::PHY_IF_WIDTH) | ((width as u32) << 3))
    }
    /// Get UTMI+ PHY interface width.
    #[inline]
    pub const fn utmi_phy_interface_width(self) -> UtmiPhyInterfaceWidth {
        match (self.0 & Self::PHY_IF_WIDTH) >> 3 {
            0 => UtmiPhyInterfaceWidth::Bits8,
            _ => UtmiPhyInterfaceWidth::Bits16,
        }
    }
    /// Set timeout calibration (`TIMEOUT_CAL`).
    #[doc(alias = "TIMEOUT_CAL")]
    #[inline]
    pub const fn set_timeout_cal(self, value: u8) -> Self {
        assert!(
            value < 8,
            "Timeout calibration out of range (expected 0..=7)"
        );
        Self((self.0 & !Self::TIMEOUT_CAL) | value as u32)
    }
    /// Get timeout calibration.
    #[inline]
    pub const fn timeout_cal(self) -> u8 {
        (self.0 & Self::TIMEOUT_CAL) as u8
    }
}

/// USB interrupt status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UsbIntSts(u32);

impl UsbIntSts {
    /// Zero value.
    pub const fn zero(self) -> Self {
        Self(0)
    }
    /// Every defined write-1-to-clear interrupt bit (`0xbfff_ffff`; bit 30 is
    /// reserved and therefore left out).
    pub const fn clear_all(self) -> Self {
        Self(0xbfff_ffff)
    }

    const WAKEUP_INT: u32 = 1 << 31;
    const DATA_FET_STOP_INT: u32 = 1 << 22;
    const INCOMP_ISO_OUT_INT: u32 = 1 << 21;
    const INCOMP_ISO_IN_INT: u32 = 1 << 20;
    const OUT_EP_INT: u32 = 1 << 19;
    const IN_EP_INT: u32 = 1 << 18;
    const EP_MIS_INT: u32 = 1 << 17;
    const EOP_FR_INT: u32 = 1 << 15;
    const ISO_OUT_DROP_INT: u32 = 1 << 14;
    const ENUM_DONE: u32 = 1 << 13;
    const USB_RESET: u32 = 1 << 12;
    const USB_SUS: u32 = 1 << 11;
    const EARLY_SUS: u32 = 1 << 10;
    const OUT_NACK_EFF: u32 = 1 << 7;
    const IN_NACK_EFF: u32 = 1 << 6;
    const NP_TXFIFO_EMP: u32 = 1 << 5;
    const RXFIFO_NO_EMP: u32 = 1 << 4;
    const RX_SOF: u32 = 1 << 3;

    /// Check if wakeup interrupt is pending (`WAKEUP_INT`).
    #[doc(alias = "WAKEUP_INT")]
    #[inline]
    pub const fn is_wakeup_int(self) -> bool {
        (self.0 & Self::WAKEUP_INT) != 0
    }
    /// Clear wakeup interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_wakeup_int(self) -> Self {
        Self(self.0 | Self::WAKEUP_INT)
    }
    /// Check if data fetch stop interrupt is pending (`DATA_FET_STOP_INT`).
    #[doc(alias = "DATA_FET_STOP_INT")]
    #[inline]
    pub const fn is_data_fet_stop_int(self) -> bool {
        (self.0 & Self::DATA_FET_STOP_INT) != 0
    }
    /// Clear data fetch stop interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_data_fet_stop_int(self) -> Self {
        Self(self.0 | Self::DATA_FET_STOP_INT)
    }
    /// Check if incomplete isochronous OUT transfer interrupt is pending (`INCOMP_ISO_OUT_INT`).
    #[doc(alias = "INCOMP_ISO_OUT_INT")]
    #[inline]
    pub const fn is_incomp_iso_out_int(self) -> bool {
        (self.0 & Self::INCOMP_ISO_OUT_INT) != 0
    }
    /// Clear incomplete isochronous OUT transfer interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_incomp_iso_out_int(self) -> Self {
        Self(self.0 | Self::INCOMP_ISO_OUT_INT)
    }
    /// Check if incomplete isochronous IN transfer interrupt is pending (`INCOMP_ISO_IN_INT`).
    #[doc(alias = "INCOMP_ISO_IN_INT")]
    #[inline]
    pub const fn is_incomp_iso_in_int(self) -> bool {
        (self.0 & Self::INCOMP_ISO_IN_INT) != 0
    }
    /// Clear incomplete isochronous IN transfer interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_incomp_iso_in_int(self) -> Self {
        Self(self.0 | Self::INCOMP_ISO_IN_INT)
    }
    /// Check if OUT endpoint interrupt is pending (`OUT_EP_INT`).
    #[doc(alias = "OUT_EP_INT")]
    #[inline]
    pub const fn is_out_ep_int(self) -> bool {
        (self.0 & Self::OUT_EP_INT) != 0
    }
    /// Check if IN endpoint interrupt is pending (`IN_EP_INT`).
    #[doc(alias = "IN_EP_INT")]
    #[inline]
    pub const fn is_in_ep_int(self) -> bool {
        (self.0 & Self::IN_EP_INT) != 0
    }
    /// Check if endpoint mismatch interrupt is pending (`EP_MIS_INT`).
    #[doc(alias = "EP_MIS_INT")]
    #[inline]
    pub const fn is_ep_mis_int(self) -> bool {
        (self.0 & Self::EP_MIS_INT) != 0
    }
    /// Clear endpoint mismatch interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_ep_mis_int(self) -> Self {
        Self(self.0 | Self::EP_MIS_INT)
    }
    /// Check if end of periodic frame interrupt is pending (`EOP_FR_INT`).
    #[doc(alias = "EOP_FR_INT")]
    #[inline]
    pub const fn is_eop_fr_int(self) -> bool {
        (self.0 & Self::EOP_FR_INT) != 0
    }
    /// Clear end of periodic frame interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_eop_fr_int(self) -> Self {
        Self(self.0 | Self::EOP_FR_INT)
    }
    /// Check if isochronous OUT packet dropped interrupt is pending (`ISO_OUT_DROP_INT`).
    #[doc(alias = "ISO_OUT_DROP_INT")]
    #[inline]
    pub const fn is_iso_out_drop_int(self) -> bool {
        (self.0 & Self::ISO_OUT_DROP_INT) != 0
    }
    /// Clear isochronous OUT packet dropped interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_iso_out_drop_int(self) -> Self {
        Self(self.0 | Self::ISO_OUT_DROP_INT)
    }
    /// Check if enumeration done interrupt is pending (`ENUM_DONE`).
    #[doc(alias = "ENUM_DONE")]
    #[inline]
    pub const fn is_enum_done(self) -> bool {
        (self.0 & Self::ENUM_DONE) != 0
    }
    /// Clear enumeration done interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_enum_done(self) -> Self {
        Self(self.0 | Self::ENUM_DONE)
    }
    /// Check if USB reset interrupt is pending (`USB_RESET`).
    #[doc(alias = "USB_RESET")]
    #[inline]
    pub const fn is_usb_reset(self) -> bool {
        (self.0 & Self::USB_RESET) != 0
    }
    /// Clear USB reset interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_usb_reset(self) -> Self {
        Self(self.0 | Self::USB_RESET)
    }
    /// Check if USB suspend interrupt is pending (`USB_SUS`).
    #[doc(alias = "USB_SUS")]
    #[inline]
    pub const fn is_usb_sus(self) -> bool {
        (self.0 & Self::USB_SUS) != 0
    }
    /// Clear USB suspend interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_usb_sus(self) -> Self {
        Self(self.0 | Self::USB_SUS)
    }
    /// Check if early suspend interrupt is pending (`EARLY_SUS`).
    #[doc(alias = "EARLY_SUS")]
    #[inline]
    pub const fn is_early_sus(self) -> bool {
        (self.0 & Self::EARLY_SUS) != 0
    }
    /// Clear early suspend interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_early_sus(self) -> Self {
        Self(self.0 | Self::EARLY_SUS)
    }
    /// Check if OUT endpoint NACK effective status is pending (`OUT_NACK_EFF`).
    #[doc(alias = "OUT_NACK_EFF")]
    #[inline]
    pub const fn is_out_nack_eff(self) -> bool {
        (self.0 & Self::OUT_NACK_EFF) != 0
    }
    /// Check if IN endpoint NACK effective status is pending (`IN_NACK_EFF`).
    #[doc(alias = "IN_NACK_EFF")]
    #[inline]
    pub const fn is_in_nack_eff(self) -> bool {
        (self.0 & Self::IN_NACK_EFF) != 0
    }
    /// Check if non-periodic TXFIFO empty status is pending (`NP_TXFIFO_EMP`).
    #[doc(alias = "NP_TXFIFO_EMP")]
    #[inline]
    pub const fn is_np_txfifo_emp(self) -> bool {
        (self.0 & Self::NP_TXFIFO_EMP) != 0
    }
    /// Check if RXFIFO non-empty status is pending (`RXFIFO_NO_EMP`).
    #[doc(alias = "RXFIFO_NO_EMP")]
    #[inline]
    pub const fn is_rxfifo_no_emp(self) -> bool {
        (self.0 & Self::RXFIFO_NO_EMP) != 0
    }
    /// Check if received start of frame interrupt is pending (`RX_SOF`).
    #[doc(alias = "RX_SOF")]
    #[inline]
    pub const fn is_rx_sof(self) -> bool {
        (self.0 & Self::RX_SOF) != 0
    }
    /// Clear received start of frame interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_rx_sof(self) -> Self {
        Self(self.0 | Self::RX_SOF)
    }
}

/// USB Interrupt Mask register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UsbIntMsk(u32);

impl UsbIntMsk {
    /// Clear every bit.
    pub const fn clear_all(self) -> Self {
        Self(0)
    }

    const WAKEUP_INT_MSK: u32 = 1 << 31;
    const DATA_FET_STOP_INT_MSK: u32 = 1 << 22;
    const INCOMP_ISO_OUT_INT_MSK: u32 = 1 << 21;
    const INCOMP_ISO_IN_INT_MSK: u32 = 1 << 20;
    const OUT_EP_INT_MSK: u32 = 1 << 19;
    const IN_EP_INT_MSK: u32 = 1 << 18;
    const EP_MIS_INT_MSK: u32 = 1 << 17;
    const EOP_FR_INT_MSK: u32 = 1 << 15;
    const ISO_OUT_DROP_INT_MSK: u32 = 1 << 14;
    const ENUM_DONE_MSK: u32 = 1 << 13;
    const USB_RESET_MSK: u32 = 1 << 12;
    const USB_SUS_MSK: u32 = 1 << 11;
    const EARLY_SUS_MSK: u32 = 1 << 10;
    const OUT_NACK_EFF_MSK: u32 = 1 << 7;
    const IN_NACK_EFF_MSK: u32 = 1 << 6;
    const NP_TXFIFO_EMP_MSK: u32 = 1 << 5;
    const RXFIFO_NO_EMP_MSK: u32 = 1 << 4;
    const RX_SOF_MSK: u32 = 1 << 3;

    /// Enable wakeup interrupt mask (`WAKEUP_INT_MSK`).
    #[doc(alias = "WAKEUP_INT_MSK")]
    #[inline]
    pub const fn enable_wakeup_int(self) -> Self {
        Self(self.0 | Self::WAKEUP_INT_MSK)
    }
    /// Disable wakeup interrupt mask.
    #[inline]
    pub const fn disable_wakeup_int(self) -> Self {
        Self(self.0 & !Self::WAKEUP_INT_MSK)
    }
    /// Check if wakeup interrupt mask is enabled.
    #[inline]
    pub const fn is_wakeup_int_enabled(self) -> bool {
        (self.0 & Self::WAKEUP_INT_MSK) != 0
    }
    /// Enable data fetch stop interrupt mask (`DATA_FET_STOP_INT_MSK`).
    #[doc(alias = "DATA_FET_STOP_INT_MSK")]
    #[inline]
    pub const fn enable_data_fet_stop_int(self) -> Self {
        Self(self.0 | Self::DATA_FET_STOP_INT_MSK)
    }
    /// Disable data fetch stop interrupt mask.
    #[inline]
    pub const fn disable_data_fet_stop_int(self) -> Self {
        Self(self.0 & !Self::DATA_FET_STOP_INT_MSK)
    }
    /// Check if data fetch stop interrupt mask is enabled.
    #[inline]
    pub const fn is_data_fet_stop_int_enabled(self) -> bool {
        (self.0 & Self::DATA_FET_STOP_INT_MSK) != 0
    }
    /// Enable incomplete isochronous OUT transfer interrupt mask (`INCOMP_ISO_OUT_INT_MSK`).
    #[doc(alias = "INCOMP_ISO_OUT_INT_MSK")]
    #[inline]
    pub const fn enable_incomp_iso_out_int(self) -> Self {
        Self(self.0 | Self::INCOMP_ISO_OUT_INT_MSK)
    }
    /// Disable incomplete isochronous OUT transfer interrupt mask.
    #[inline]
    pub const fn disable_incomp_iso_out_int(self) -> Self {
        Self(self.0 & !Self::INCOMP_ISO_OUT_INT_MSK)
    }
    /// Check if incomplete isochronous OUT transfer interrupt mask is enabled.
    #[inline]
    pub const fn is_incomp_iso_out_int_enabled(self) -> bool {
        (self.0 & Self::INCOMP_ISO_OUT_INT_MSK) != 0
    }
    /// Enable incomplete isochronous IN transfer interrupt mask (`INCOMP_ISO_IN_INT_MSK`).
    #[doc(alias = "INCOMP_ISO_IN_INT_MSK")]
    #[inline]
    pub const fn enable_incomp_iso_in_int(self) -> Self {
        Self(self.0 | Self::INCOMP_ISO_IN_INT_MSK)
    }
    /// Disable incomplete isochronous IN transfer interrupt mask.
    #[inline]
    pub const fn disable_incomp_iso_in_int(self) -> Self {
        Self(self.0 & !Self::INCOMP_ISO_IN_INT_MSK)
    }
    /// Check if incomplete isochronous IN transfer interrupt mask is enabled.
    #[inline]
    pub const fn is_incomp_iso_in_int_enabled(self) -> bool {
        (self.0 & Self::INCOMP_ISO_IN_INT_MSK) != 0
    }
    /// Enable OUT endpoint interrupt mask (`OUT_EP_INT_MSK`).
    #[doc(alias = "OUT_EP_INT_MSK")]
    #[inline]
    pub const fn enable_out_ep_int(self) -> Self {
        Self(self.0 | Self::OUT_EP_INT_MSK)
    }
    /// Disable OUT endpoint interrupt mask.
    #[inline]
    pub const fn disable_out_ep_int(self) -> Self {
        Self(self.0 & !Self::OUT_EP_INT_MSK)
    }
    /// Check if OUT endpoint interrupt mask is enabled.
    #[inline]
    pub const fn is_out_ep_int_enabled(self) -> bool {
        (self.0 & Self::OUT_EP_INT_MSK) != 0
    }
    /// Enable IN endpoint interrupt mask (`IN_EP_INT_MSK`).
    #[doc(alias = "IN_EP_INT_MSK")]
    #[inline]
    pub const fn enable_in_ep_int(self) -> Self {
        Self(self.0 | Self::IN_EP_INT_MSK)
    }
    /// Disable IN endpoint interrupt mask.
    #[inline]
    pub const fn disable_in_ep_int(self) -> Self {
        Self(self.0 & !Self::IN_EP_INT_MSK)
    }
    /// Check if IN endpoint interrupt mask is enabled.
    #[inline]
    pub const fn is_in_ep_int_enabled(self) -> bool {
        (self.0 & Self::IN_EP_INT_MSK) != 0
    }
    /// Enable endpoint mismatch interrupt mask (`EP_MIS_INT_MSK`).
    #[doc(alias = "EP_MIS_INT_MSK")]
    #[inline]
    pub const fn enable_ep_mis_int(self) -> Self {
        Self(self.0 | Self::EP_MIS_INT_MSK)
    }
    /// Disable endpoint mismatch interrupt mask.
    #[inline]
    pub const fn disable_ep_mis_int(self) -> Self {
        Self(self.0 & !Self::EP_MIS_INT_MSK)
    }
    /// Check if endpoint mismatch interrupt mask is enabled.
    #[inline]
    pub const fn is_ep_mis_int_enabled(self) -> bool {
        (self.0 & Self::EP_MIS_INT_MSK) != 0
    }
    /// Enable end of periodic frame interrupt mask (`EOP_FR_INT_MSK`).
    #[doc(alias = "EOP_FR_INT_MSK")]
    #[inline]
    pub const fn enable_eop_fr_int(self) -> Self {
        Self(self.0 | Self::EOP_FR_INT_MSK)
    }
    /// Disable end of periodic frame interrupt mask.
    #[inline]
    pub const fn disable_eop_fr_int(self) -> Self {
        Self(self.0 & !Self::EOP_FR_INT_MSK)
    }
    /// Check if end of periodic frame interrupt mask is enabled.
    #[inline]
    pub const fn is_eop_fr_int_enabled(self) -> bool {
        (self.0 & Self::EOP_FR_INT_MSK) != 0
    }
    /// Enable isochronous OUT packet dropped interrupt mask (`ISO_OUT_DROP_INT_MSK`).
    #[doc(alias = "ISO_OUT_DROP_INT_MSK")]
    #[inline]
    pub const fn enable_iso_out_drop_int(self) -> Self {
        Self(self.0 | Self::ISO_OUT_DROP_INT_MSK)
    }
    /// Disable isochronous OUT packet dropped interrupt mask.
    #[inline]
    pub const fn disable_iso_out_drop_int(self) -> Self {
        Self(self.0 & !Self::ISO_OUT_DROP_INT_MSK)
    }
    /// Check if isochronous OUT packet dropped interrupt mask is enabled.
    #[inline]
    pub const fn is_iso_out_drop_int_enabled(self) -> bool {
        (self.0 & Self::ISO_OUT_DROP_INT_MSK) != 0
    }
    /// Enable enumeration done interrupt mask (`ENUM_DONE_MSK`).
    #[doc(alias = "ENUM_DONE_MSK")]
    #[inline]
    pub const fn enable_enum_done(self) -> Self {
        Self(self.0 | Self::ENUM_DONE_MSK)
    }
    /// Disable enumeration done interrupt mask.
    #[inline]
    pub const fn disable_enum_done(self) -> Self {
        Self(self.0 & !Self::ENUM_DONE_MSK)
    }
    /// Check if enumeration done interrupt mask is enabled.
    #[inline]
    pub const fn is_enum_done_enabled(self) -> bool {
        (self.0 & Self::ENUM_DONE_MSK) != 0
    }
    /// Enable USB reset interrupt mask (`USB_RESET_MSK`).
    #[doc(alias = "USB_RESET_MSK")]
    #[inline]
    pub const fn enable_usb_reset(self) -> Self {
        Self(self.0 | Self::USB_RESET_MSK)
    }
    /// Disable USB reset interrupt mask.
    #[inline]
    pub const fn disable_usb_reset(self) -> Self {
        Self(self.0 & !Self::USB_RESET_MSK)
    }
    /// Check if USB reset interrupt mask is enabled.
    #[inline]
    pub const fn is_usb_reset_enabled(self) -> bool {
        (self.0 & Self::USB_RESET_MSK) != 0
    }
    /// Enable USB suspend interrupt mask (`USB_SUS_MSK`).
    #[doc(alias = "USB_SUS_MSK")]
    #[inline]
    pub const fn enable_usb_sus(self) -> Self {
        Self(self.0 | Self::USB_SUS_MSK)
    }
    /// Disable USB suspend interrupt mask.
    #[inline]
    pub const fn disable_usb_sus(self) -> Self {
        Self(self.0 & !Self::USB_SUS_MSK)
    }
    /// Check if USB suspend interrupt mask is enabled.
    #[inline]
    pub const fn is_usb_sus_enabled(self) -> bool {
        (self.0 & Self::USB_SUS_MSK) != 0
    }
    /// Enable early suspend interrupt mask (`EARLY_SUS_MSK`).
    #[doc(alias = "EARLY_SUS_MSK")]
    #[inline]
    pub const fn enable_early_sus(self) -> Self {
        Self(self.0 | Self::EARLY_SUS_MSK)
    }
    /// Disable early suspend interrupt mask.
    #[inline]
    pub const fn disable_early_sus(self) -> Self {
        Self(self.0 & !Self::EARLY_SUS_MSK)
    }
    /// Check if early suspend interrupt mask is enabled.
    #[inline]
    pub const fn is_early_sus_enabled(self) -> bool {
        (self.0 & Self::EARLY_SUS_MSK) != 0
    }
    /// Enable OUT endpoint NACK effective status mask (`OUT_NACK_EFF_MSK`).
    #[doc(alias = "OUT_NACK_EFF_MSK")]
    #[inline]
    pub const fn enable_out_nack_eff(self) -> Self {
        Self(self.0 | Self::OUT_NACK_EFF_MSK)
    }
    /// Disable OUT endpoint NACK effective status mask.
    #[inline]
    pub const fn disable_out_nack_eff(self) -> Self {
        Self(self.0 & !Self::OUT_NACK_EFF_MSK)
    }
    /// Check if OUT endpoint NACK effective status mask is enabled.
    #[inline]
    pub const fn is_out_nack_eff_enabled(self) -> bool {
        (self.0 & Self::OUT_NACK_EFF_MSK) != 0
    }
    /// Enable IN endpoint NACK effective status mask (`IN_NACK_EFF_MSK`).
    #[doc(alias = "IN_NACK_EFF_MSK")]
    #[inline]
    pub const fn enable_in_nack_eff(self) -> Self {
        Self(self.0 | Self::IN_NACK_EFF_MSK)
    }
    /// Disable IN endpoint NACK effective status mask.
    #[inline]
    pub const fn disable_in_nack_eff(self) -> Self {
        Self(self.0 & !Self::IN_NACK_EFF_MSK)
    }
    /// Check if IN endpoint NACK effective status mask is enabled.
    #[inline]
    pub const fn is_in_nack_eff_enabled(self) -> bool {
        (self.0 & Self::IN_NACK_EFF_MSK) != 0
    }
    /// Enable non-periodic TXFIFO empty status mask (`NP_TXFIFO_EMP_MSK`).
    #[doc(alias = "NP_TXFIFO_EMP_MSK")]
    #[inline]
    pub const fn enable_np_txfifo_emp(self) -> Self {
        Self(self.0 | Self::NP_TXFIFO_EMP_MSK)
    }
    /// Disable non-periodic TXFIFO empty status mask.
    #[inline]
    pub const fn disable_np_txfifo_emp(self) -> Self {
        Self(self.0 & !Self::NP_TXFIFO_EMP_MSK)
    }
    /// Check if non-periodic TXFIFO empty status mask is enabled.
    #[inline]
    pub const fn is_np_txfifo_emp_enabled(self) -> bool {
        (self.0 & Self::NP_TXFIFO_EMP_MSK) != 0
    }
    /// Enable RXFIFO non-empty status mask (`RXFIFO_NO_EMP_MSK`).
    #[doc(alias = "RXFIFO_NO_EMP_MSK")]
    #[inline]
    pub const fn enable_rxfifo_no_emp(self) -> Self {
        Self(self.0 | Self::RXFIFO_NO_EMP_MSK)
    }
    /// Disable RXFIFO non-empty status mask.
    #[inline]
    pub const fn disable_rxfifo_no_emp(self) -> Self {
        Self(self.0 & !Self::RXFIFO_NO_EMP_MSK)
    }
    /// Check if RXFIFO non-empty status mask is enabled.
    #[inline]
    pub const fn is_rxfifo_no_emp_enabled(self) -> bool {
        (self.0 & Self::RXFIFO_NO_EMP_MSK) != 0
    }
    /// Enable received start of frame interrupt mask (`RX_SOF_MSK`).
    #[doc(alias = "RX_SOF_MSK")]
    #[inline]
    pub const fn enable_rx_sof(self) -> Self {
        Self(self.0 | Self::RX_SOF_MSK)
    }
    /// Disable received start of frame interrupt mask.
    #[inline]
    pub const fn disable_rx_sof(self) -> Self {
        Self(self.0 & !Self::RX_SOF_MSK)
    }
    /// Check if received start of frame interrupt mask is enabled.
    #[inline]
    pub const fn is_rx_sof_enabled(self) -> bool {
        (self.0 & Self::RX_SOF_MSK) != 0
    }
}

/// Receive FIFO Size register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct RxFifoSiz(u32);

impl RxFifoSiz {
    /// Zero value.
    pub const fn zero(self) -> Self {
        Self(0)
    }

    const RXFIFO_DEPTH: u32 = 0xFFFF;

    /// Set RX FIFO depth in 32-bit words (`RXFIFO_DEPTH`).
    #[doc(alias = "RXFIFO_DEPTH")]
    #[inline]
    pub const fn set_rxfifo_depth(self, val: u16) -> Self {
        Self((self.0 & !Self::RXFIFO_DEPTH) | ((val as u32) & Self::RXFIFO_DEPTH))
    }
    /// Get RX FIFO depth.
    #[inline]
    pub const fn rxfifo_depth(self) -> u16 {
        (self.0 & Self::RXFIFO_DEPTH) as u16
    }
}

/// Receive FIFO packet status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RxFifoPacketStatus {
    /// Global OUT NAK received.
    GlobalOutNak = 1,
    /// OUT data packet received.
    OutDataPacket = 2,
    /// OUT transfer completed.
    OutTransferComplete = 3,
    /// SETUP transaction completed.
    SetupTransactionComplete = 4,
    /// SETUP data packet received.
    SetupDataPacket = 6,
}

/// OUT data PID.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OutDataPid {
    /// DATA0 PID.
    Data0,
    /// DATA2 PID.
    Data2,
    /// DATA1 PID.
    Data1,
    /// MDATA PID.
    Mdata,
}

/// Receive FIFO Status register (read-only).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct RxFifoSts(u32);

impl RxFifoSts {
    /// Zero value.
    pub const fn zero(self) -> Self {
        Self(0)
    }

    const FR_NUM: u32 = 0xF << 21;
    const RX_PKT_STS: u32 = 0xF << 17;
    const OUT_DAT_PID: u32 = 0x3 << 15;
    const RX_BYTE_CNT: u32 = 0x7FF << 4;
    const OUTEP_NUM: u32 = 0xF;

    /// Get frame number (`FR_NUM`).
    #[doc(alias = "FR_NUM")]
    #[inline]
    pub const fn frame_number(self) -> u8 {
        ((self.0 & Self::FR_NUM) >> 21) as u8
    }
    /// Get received packet status (`RX_PKT_STS`).
    #[doc(alias = "RX_PKT_STS")]
    #[inline]
    pub const fn pkt_status(self) -> RxFifoPacketStatus {
        match (self.0 & Self::RX_PKT_STS) >> 17 {
            1 => RxFifoPacketStatus::GlobalOutNak,
            2 => RxFifoPacketStatus::OutDataPacket,
            3 => RxFifoPacketStatus::OutTransferComplete,
            4 => RxFifoPacketStatus::SetupTransactionComplete,
            6 => RxFifoPacketStatus::SetupDataPacket,
            _ => panic!("Invalid RXFIFO packet status"),
        }
    }
    /// Get OUT data PID (`OUT_DAT_PID`).
    #[doc(alias = "OUT_DAT_PID")]
    #[inline]
    pub const fn out_data_pid(self) -> OutDataPid {
        match (self.0 & Self::OUT_DAT_PID) >> 15 {
            0 => OutDataPid::Data0,
            1 => OutDataPid::Data2,
            2 => OutDataPid::Data1,
            _ => OutDataPid::Mdata,
        }
    }
    /// Get byte count (`RX_BYTE_CNT`).
    #[doc(alias = "RX_BYTE_CNT")]
    #[inline]
    pub const fn byte_count(self) -> u16 {
        ((self.0 & Self::RX_BYTE_CNT) >> 4) as u16
    }
    /// Get endpoint number (`OUTEP_NUM`).
    #[doc(alias = "OUTEP_NUM")]
    #[inline]
    pub const fn ep_num(self) -> u8 {
        (self.0 & Self::OUTEP_NUM) as u8
    }
}

/// Non-periodic TXFIFO Size register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NpTxFifoSiz(u32);

impl NpTxFifoSiz {
    /// Zero value.
    pub const fn zero(self) -> Self {
        Self(0)
    }

    const NPTXFIFO_SIZ: u32 = 0xFFFF << 16;
    const NPTXFIFO_START: u32 = 0xFFFF;

    /// Set FIFO size (`NPTXFIFO_SIZ`).
    #[doc(alias = "NPTXFIFO_SIZ")]
    #[inline]
    pub const fn set_size(self, val: u16) -> Self {
        Self((self.0 & !Self::NPTXFIFO_SIZ) | (((val as u32) << 16) & Self::NPTXFIFO_SIZ))
    }
    /// Get FIFO size.
    #[inline]
    pub const fn size(self) -> u16 {
        ((self.0 & Self::NPTXFIFO_SIZ) >> 16) as u16
    }
    /// Set start address of non-periodic TXFIFO (`NPTXFIFO_START`).
    #[doc(alias = "NPTXFIFO_START")]
    #[inline]
    pub const fn set_start_addr(self, val: u16) -> Self {
        Self((self.0 & !Self::NPTXFIFO_START) | ((val as u32) & Self::NPTXFIFO_START))
    }
    /// Get start address.
    #[inline]
    pub const fn start_addr(self) -> u16 {
        (self.0 & Self::NPTXFIFO_START) as u16
    }
}

/// Non-periodic TXFIFO Status register (read-only).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NpTxFifoSts(u32);

impl NpTxFifoSts {
    /// Zero value.
    pub const fn zero(self) -> Self {
        Self(0)
    }

    const NPTX_Q_TOP: u32 = 0xFF << 24;
    const NPTX_Q_SPACE_AVAIL: u32 = 0xF << 16;
    const NPTX_FIFO_SPACE_AVAIL: u32 = 0xFFFF;

    /// Get top entry in periodic request queue (`NPTX_Q_TOP`).
    #[doc(alias = "NPTX_Q_TOP")]
    #[inline]
    pub const fn queue_top(self) -> u8 {
        ((self.0 & Self::NPTX_Q_TOP) >> 24) as u8
    }
    /// Get space available in request queue (`NPTX_Q_SPACE_AVAIL`).
    #[doc(alias = "NPTX_Q_SPACE_AVAIL")]
    #[inline]
    pub const fn queue_space_avail(self) -> u8 {
        ((self.0 & Self::NPTX_Q_SPACE_AVAIL) >> 16) as u8
    }
    /// Get word count available in NPTX FIFO (`NPTX_FIFO_SPACE_AVAIL`).
    #[doc(alias = "NPTX_FIFO_SPACE_AVAIL")]
    #[inline]
    pub const fn fifo_space_avail(self) -> u16 {
        (self.0 & Self::NPTX_FIFO_SPACE_AVAIL) as u16
    }
}

/// Periodic TXFIFO Size register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PTxFifoSiz(u32);

impl PTxFifoSiz {
    /// Zero value.
    pub const fn zero(self) -> Self {
        Self(0)
    }

    const PTXFIFO_SIZ: u32 = 0xFFFF << 16;
    const PTXFIFO_START: u32 = 0xFFFF;

    /// Set FIFO size (`PTXFIFO_SIZ`).
    #[doc(alias = "PTXFIFO_SIZ")]
    #[inline]
    pub const fn set_size(self, val: u16) -> Self {
        Self((self.0 & !Self::PTXFIFO_SIZ) | (((val as u32) << 16) & Self::PTXFIFO_SIZ))
    }
    /// Get FIFO size.
    #[inline]
    pub const fn size(self) -> u16 {
        ((self.0 & Self::PTXFIFO_SIZ) >> 16) as u16
    }
    /// Set start address of periodic TXFIFO (`PTXFIFO_START`).
    #[doc(alias = "PTXFIFO_START")]
    #[inline]
    pub const fn set_start_addr(self, val: u16) -> Self {
        Self((self.0 & !Self::PTXFIFO_START) | ((val as u32) & Self::PTXFIFO_START))
    }
    /// Get start address.
    #[inline]
    pub const fn start_addr(self) -> u16 {
        (self.0 & Self::PTXFIFO_START) as u16
    }
}

/// PHY Clock Control register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PhyClkCtl(u32);

impl PhyClkCtl {
    /// Zero value.
    pub const fn zero(self) -> Self {
        Self(0)
    }

    const STP_PHY_CLK: u32 = 1;

    /// Set PHY clock stop (`STP_PHY_CLK`).
    #[doc(alias = "STP_PHY_CLK")]
    #[inline]
    pub const fn stop_phy_clk(self, stop: bool) -> Self {
        if stop {
            Self(self.0 | Self::STP_PHY_CLK)
        } else {
            Self(self.0 & !Self::STP_PHY_CLK)
        }
    }
    /// Check if PHY clock is stopped.
    #[inline]
    pub const fn is_phy_clk_stopped(self) -> bool {
        (self.0 & Self::STP_PHY_CLK) != 0
    }
}

/// USB device enumeration speed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum UsbDevSpeed {
    /// High speed.
    High,
    /// Full speed.
    Full,
}

/// Periodic-frame interrupt point.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PeriodicFrameInterval {
    /// Interrupt at 80% of the frame.
    Percent80,
    /// Interrupt at 85% of the frame.
    Percent85,
    /// Interrupt at 90% of the frame.
    Percent90,
    /// Interrupt at 95% of the frame.
    Percent95,
}

/// OUT transaction handling for a non-zero-length data packet during the status phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OutNZLDataHandling {
    /// Deliver the received packet to the application layer.
    DeliverToApplication,
    /// Respond with STALL and do not deliver the packet to the application layer.
    Stall,
}

/// USB Device Configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UsbDevConf(u32);

impl UsbDevConf {
    /// Zero value.
    pub const fn zero(self) -> Self {
        Self(0)
    }

    const IN_EP_MIS_THIN: u32 = 0x1F << 18;
    const MSK_EARLY_SUS_ERR_INT: u32 = 1 << 15;
    const EN_DLY_XCVR: u32 = 1 << 14;
    const SET_PER_FR_INT: u32 = 0x3 << 11;
    const USB_DEV_ADDR: u32 = 0x7F << 4;
    const OUT_RX_NZL_DATA_HDL: u32 = 1 << 2;
    const USB_DEV_SPD_SET: u32 = 0x3;

    /// Set IN endpoint mismatch threshold (`IN_EP_MIS_THIN`).
    #[doc(alias = "IN_EP_MIS_THIN")]
    #[inline]
    pub const fn set_in_ep_mis_thin(self, value: u8) -> Self {
        assert!(
            value < 0x20,
            "IN endpoint mismatch threshold out of range (expected 0..=31)"
        );
        Self((self.0 & !Self::IN_EP_MIS_THIN) | ((value as u32) << 18))
    }
    /// Get IN endpoint mismatch threshold.
    #[inline]
    pub const fn in_ep_mis_thin(self) -> u8 {
        ((self.0 & Self::IN_EP_MIS_THIN) >> 18) as u8
    }
    /// Enable erratic errors caused by an early-suspend interrupt (`MSK_EARLY_SUS_ERR_INT`).
    #[doc(alias = "MSK_EARLY_SUS_ERR_INT")]
    #[inline]
    pub const fn enable_early_suspend_erratic_error(self) -> Self {
        Self(self.0 & !Self::MSK_EARLY_SUS_ERR_INT)
    }
    /// Disable erratic errors caused by an early-suspend interrupt.
    #[inline]
    pub const fn disable_early_suspend_erratic_error(self) -> Self {
        Self(self.0 | Self::MSK_EARLY_SUS_ERR_INT)
    }
    /// Check if erratic errors caused by an early-suspend interrupt are enabled.
    #[inline]
    pub const fn is_early_suspend_erratic_error_enabled(self) -> bool {
        self.0 & Self::MSK_EARLY_SUS_ERR_INT == 0
    }
    /// Enable transceiver delay during chirp (`EN_DLY_XCVR`).
    #[doc(alias = "EN_DLY_XCVR")]
    #[inline]
    pub const fn enable_transceiver_delay(self) -> Self {
        Self(self.0 | Self::EN_DLY_XCVR)
    }
    /// Disable transceiver delay during chirp.
    #[inline]
    pub const fn disable_transceiver_delay(self) -> Self {
        Self(self.0 & !Self::EN_DLY_XCVR)
    }
    /// Check if transceiver delay during chirp is enabled.
    #[inline]
    pub const fn is_transceiver_delay_enabled(self) -> bool {
        self.0 & Self::EN_DLY_XCVR != 0
    }
    /// Set periodic-frame interrupt point (`SET_PER_FR_INT`).
    #[doc(alias = "SET_PER_FR_INT")]
    #[inline]
    pub const fn set_per_frame_int(self, interval: PeriodicFrameInterval) -> Self {
        Self((self.0 & !Self::SET_PER_FR_INT) | ((interval as u32) << 11))
    }
    /// Get periodic-frame interrupt point.
    #[inline]
    pub const fn per_frame_int(self) -> PeriodicFrameInterval {
        match (self.0 & Self::SET_PER_FR_INT) >> 11 {
            0 => PeriodicFrameInterval::Percent80,
            1 => PeriodicFrameInterval::Percent85,
            2 => PeriodicFrameInterval::Percent90,
            _ => PeriodicFrameInterval::Percent95,
        }
    }
    /// Set USB device address (`USB_DEV_ADDR`).
    #[doc(alias = "USB_DEV_ADDR")]
    #[inline]
    pub const fn set_dev_addr(self, value: u8) -> Self {
        assert!(
            value < 0x80,
            "USB device address out of range (expected 0..=127)"
        );
        Self((self.0 & !Self::USB_DEV_ADDR) | ((value as u32) << 4))
    }
    /// Get USB device address.
    #[inline]
    pub const fn dev_addr(self) -> u8 {
        ((self.0 & Self::USB_DEV_ADDR) >> 4) as u8
    }
    /// Set non-zero OUT data handling (`OUT_RX_NZL_DATA_HDL`).
    #[doc(alias = "OUT_RX_NZL_DATA_HDL")]
    #[inline]
    pub const fn set_out_nzl_data_handling(self, handling: OutNZLDataHandling) -> Self {
        Self((self.0 & !Self::OUT_RX_NZL_DATA_HDL) | ((handling as u32) << 2))
    }
    /// Get non-zero OUT data handling.
    #[inline]
    pub const fn out_nzl_data_handling(self) -> OutNZLDataHandling {
        match (self.0 & Self::OUT_RX_NZL_DATA_HDL) >> 2 {
            0 => OutNZLDataHandling::DeliverToApplication,
            _ => OutNZLDataHandling::Stall,
        }
    }
    /// Set USB device enumeration speed (`USB_DEV_SPD_SET`).
    #[doc(alias = "USB_DEV_SPD_SET")]
    #[inline]
    pub const fn set_dev_speed(self, speed: UsbDevSpeed) -> Self {
        Self((self.0 & !Self::USB_DEV_SPD_SET) | speed as u32)
    }
    /// Get USB device enumeration speed.
    #[inline]
    pub const fn dev_speed(self) -> UsbDevSpeed {
        match self.0 & Self::USB_DEV_SPD_SET {
            0 => UsbDevSpeed::High,
            1 => UsbDevSpeed::Full,
            _ => panic!("Invalid USB device speed"),
        }
    }
}

/// USB electrical test mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum UsbTestMode {
    /// Test mode disabled.
    Disabled,
    /// Test J mode.
    J,
    /// Test K mode.
    K,
    /// Test SE0 NAK mode.
    Se0Nak,
    /// Test packet mode.
    Packet,
    /// Test force-enable mode.
    ForceEnable,
}

/// USB device connection state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum UsbDeviceConnection {
    /// Device is connected and operates normally.
    Connected,
    /// Device performs a soft disconnect.
    Disconnected,
}

/// USB Device Function register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UsbDevFunc(u32);

impl UsbDevFunc {
    /// Zero value.
    pub const fn zero(self) -> Self {
        Self(0)
    }

    const SERVICE_INTERVAL_SUPPORTED: u32 = 1 << 19;
    const EN_NACK_BBL_ERR: u32 = 1 << 16;
    const IG_FR_NUM_ISO_EP: u32 = 1 << 15;
    const PWR_ON_DONE: u32 = 1 << 11;
    const CLR_OUT_NACK: u32 = 1 << 10;
    const SET_OUT_NACK: u32 = 1 << 9;
    const CLR_NP_IN_NACK: u32 = 1 << 8;
    const SET_NP_IN_NACK: u32 = 1 << 7;
    const TEST_MOD: u32 = 0x7 << 4;
    const GLB_OUT_NACK_STS: u32 = 1 << 3;
    const GLB_IN_NACK_STS: u32 = 1 << 2;
    const SET_DEV_DISCON: u32 = 1 << 1;
    const EN_REM_WAKUP: u32 = 1;

    /// Check if the service interval is supported (`SERVICE_INTERVAL_SUPPORTED`).
    ///
    /// Defined by the C SDK (`USBDEVFUNC_SERVICE_INTERVAL_SUPPORTED`) and not
    /// documented in the D13x user manual.
    #[doc(alias = "SERVICE_INTERVAL_SUPPORTED")]
    #[inline]
    pub const fn is_service_interval_supported(self) -> bool {
        self.0 & Self::SERVICE_INTERVAL_SUPPORTED != 0
    }
    /// Enable NACK babble-error handling (`EN_NACK_BBL_ERR`).
    #[doc(alias = "EN_NACK_BBL_ERR")]
    #[inline]
    pub const fn enable_nack_babble_error_handling(self) -> Self {
        Self(self.0 | Self::EN_NACK_BBL_ERR)
    }
    /// Disable NACK babble-error handling.
    #[inline]
    pub const fn disable_nack_babble_error_handling(self) -> Self {
        Self(self.0 & !Self::EN_NACK_BBL_ERR)
    }
    /// Check if NACK babble-error handling is enabled.
    #[inline]
    pub const fn is_nack_babble_error_handling_enabled(self) -> bool {
        self.0 & Self::EN_NACK_BBL_ERR != 0
    }
    /// Enable frame-number ignoring for isochronous endpoints (`IG_FR_NUM_ISO_EP`).
    #[doc(alias = "IG_FR_NUM_ISO_EP")]
    #[inline]
    pub const fn enable_isochronous_frame_number_ignore(self) -> Self {
        Self(self.0 | Self::IG_FR_NUM_ISO_EP)
    }
    /// Disable frame-number ignoring for isochronous endpoints.
    #[inline]
    pub const fn disable_isochronous_frame_number_ignore(self) -> Self {
        Self(self.0 & !Self::IG_FR_NUM_ISO_EP)
    }
    /// Check if frame-number ignoring for isochronous endpoints is enabled.
    #[inline]
    pub const fn is_isochronous_frame_number_ignore_enabled(self) -> bool {
        self.0 & Self::IG_FR_NUM_ISO_EP != 0
    }
    /// Set power-on programming done bit (`PWR_ON_DONE`).
    #[doc(alias = "PWR_ON_DONE")]
    #[inline]
    pub const fn set_pwr_on_done(self, set: bool) -> Self {
        Self((self.0 & !Self::PWR_ON_DONE) | if set { Self::PWR_ON_DONE } else { 0 })
    }
    /// Get power-on programming done bit.
    #[inline]
    pub const fn pwr_on_done(self) -> bool {
        self.0 & Self::PWR_ON_DONE != 0
    }
    /// Clear OUT NACK (`CLR_OUT_NACK`).
    #[doc(alias = "CLR_OUT_NACK")]
    #[inline]
    pub const fn clear_out_nack(self) -> Self {
        Self(self.0 | Self::CLR_OUT_NACK)
    }
    /// Set OUT NACK (`SET_OUT_NACK`).
    #[doc(alias = "SET_OUT_NACK")]
    #[inline]
    pub const fn set_out_nack(self) -> Self {
        Self(self.0 | Self::SET_OUT_NACK)
    }
    /// Clear non-periodic IN NACK (`CLR_NP_IN_NACK`).
    #[doc(alias = "CLR_NP_IN_NACK")]
    #[inline]
    pub const fn clear_np_in_nack(self) -> Self {
        Self(self.0 | Self::CLR_NP_IN_NACK)
    }
    /// Set non-periodic IN NACK (`SET_NP_IN_NACK`).
    #[doc(alias = "SET_NP_IN_NACK")]
    #[inline]
    pub const fn set_np_in_nack(self) -> Self {
        Self(self.0 | Self::SET_NP_IN_NACK)
    }
    /// Set USB test mode (`TEST_MOD`).
    #[doc(alias = "TEST_MOD")]
    #[inline]
    pub const fn set_test_mode(self, mode: UsbTestMode) -> Self {
        Self((self.0 & !Self::TEST_MOD) | ((mode as u32) << 4))
    }
    /// Get USB test mode.
    #[inline]
    pub const fn test_mode(self) -> UsbTestMode {
        match (self.0 & Self::TEST_MOD) >> 4 {
            0 => UsbTestMode::Disabled,
            1 => UsbTestMode::J,
            2 => UsbTestMode::K,
            3 => UsbTestMode::Se0Nak,
            4 => UsbTestMode::Packet,
            5 => UsbTestMode::ForceEnable,
            _ => panic!("Invalid USB test mode"),
        }
    }
    /// Check global OUT NACK status (`GLB_OUT_NACK_STS`).
    #[doc(alias = "GLB_OUT_NACK_STS")]
    #[inline]
    pub const fn is_glb_out_nack(self) -> bool {
        self.0 & Self::GLB_OUT_NACK_STS != 0
    }
    /// Check global IN NACK status (`GLB_IN_NACK_STS`).
    #[doc(alias = "GLB_IN_NACK_STS")]
    #[inline]
    pub const fn is_glb_in_nack(self) -> bool {
        self.0 & Self::GLB_IN_NACK_STS != 0
    }
    /// Set USB device connection state (`SET_DEV_DISCON`).
    #[doc(alias = "SET_DEV_DISCON")]
    #[inline]
    pub const fn set_connection(self, connection: UsbDeviceConnection) -> Self {
        Self((self.0 & !Self::SET_DEV_DISCON) | ((connection as u32) << 1))
    }
    /// Get USB device connection state.
    #[inline]
    pub const fn connection(self) -> UsbDeviceConnection {
        match (self.0 & Self::SET_DEV_DISCON) >> 1 {
            0 => UsbDeviceConnection::Connected,
            _ => UsbDeviceConnection::Disconnected,
        }
    }
    /// Enable remote wakeup (`EN_REM_WAKUP`).
    #[doc(alias = "EN_REM_WAKUP")]
    #[inline]
    pub const fn enable_remote_wakeup(self) -> Self {
        Self(self.0 | Self::EN_REM_WAKUP)
    }
    /// Disable remote wakeup.
    #[inline]
    pub const fn disable_remote_wakeup(self) -> Self {
        Self(self.0 & !Self::EN_REM_WAKUP)
    }
    /// Check whether remote wakeup is enabled.
    #[inline]
    pub const fn is_remote_wakeup_enabled(self) -> bool {
        self.0 & Self::EN_REM_WAKUP != 0
    }
}

/// USB Line Status register (read-only).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UsbLineSts(u32);

impl UsbLineSts {
    /// Zero value.
    pub const fn zero(self) -> Self {
        Self(0)
    }

    const DP_LEVEL: u32 = 1 << 23;
    const DM_LEVEL: u32 = 1 << 22;
    const RX_FR_NUM: u32 = 0x3FFF << 8;
    const ERRTIC_ERR: u32 = 1 << 3;
    const ENUM_SPEED: u32 = 0x3 << 1;
    const DET_SUS_STS: u32 = 1;

    /// Check D+ line level (`DAT_LINE_STS`).
    #[doc(alias = "DP_LEVEL")]
    #[inline]
    pub const fn dp_level(self) -> bool {
        (self.0 & Self::DP_LEVEL) != 0
    }
    /// Check D- line level (`DAT_LINE_STS`).
    #[doc(alias = "DM_LEVEL")]
    #[inline]
    pub const fn dm_level(self) -> bool {
        (self.0 & Self::DM_LEVEL) != 0
    }
    /// Get received SOF frame number (`RX_FR_NUM`).
    #[doc(alias = "RX_FR_NUM")]
    #[inline]
    pub const fn rx_fr_num(self) -> u16 {
        ((self.0 & Self::RX_FR_NUM) >> 8) as u16
    }
    /// Check erratic error status (`ERRTIC_ERR`).
    #[doc(alias = "ERRTIC_ERR")]
    #[inline]
    pub const fn is_errtic_error(self) -> bool {
        (self.0 & Self::ERRTIC_ERR) != 0
    }
    /// Get enumerated speed (`ENUM_SPEED`).
    #[doc(alias = "ENUM_SPEED")]
    #[inline]
    pub const fn enum_speed(self) -> UsbDevSpeed {
        match (self.0 & Self::ENUM_SPEED) >> 1 {
            0 => UsbDevSpeed::High,
            1 => UsbDevSpeed::Full,
            _ => panic!("Invalid USB device enumerated speed"),
        }
    }
    /// Check suspend detection status (`DET_SUS_STS`).
    #[doc(alias = "DET_SUS_STS")]
    #[inline]
    pub const fn is_suspend_detected(self) -> bool {
        (self.0 & Self::DET_SUS_STS) != 0
    }
}

/// IN endpoint interrupt mask register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct InEpIntMsk(u32);

impl InEpIntMsk {
    /// Clear every bit.
    pub const fn clear_all(self) -> Self {
        Self(0)
    }

    const NACK_INT_MSK: u32 = 1 << 13;
    const IN_NACK_EFF_INT_MSK: u32 = 1 << 6;
    const INTOKEN_MIS_INT_MSK: u32 = 1 << 5;
    const RX_INTOKEN_EMPTY_INT_MSK: u32 = 1 << 4;
    const TIME_OUT_INT_MSK: u32 = 1 << 3;
    const AHB_RW_ERR_INT_MSK: u32 = 1 << 2;
    const DIS_EP_INT_MSK: u32 = 1 << 1;
    const TX_COMP_INT_MSK: u32 = 1;

    /// Enable NACK interrupt (`NACK_INT_MSK`).
    #[doc(alias = "NACK_INT_MSK")]
    #[inline]
    pub const fn enable_nack_int(self) -> Self {
        Self(self.0 | Self::NACK_INT_MSK)
    }
    /// Disable NACK interrupt.
    #[inline]
    pub const fn disable_nack_int(self) -> Self {
        Self(self.0 & !Self::NACK_INT_MSK)
    }
    /// Check if NACK interrupt is enabled.
    #[inline]
    pub const fn is_nack_int_enabled(self) -> bool {
        (self.0 & Self::NACK_INT_MSK) != 0
    }
    /// Enable IN NACK effective interrupt (`IN_NACK_EFF_INT_MSK`).
    #[doc(alias = "IN_NACK_EFF_INT_MSK")]
    #[inline]
    pub const fn enable_in_nack_eff_int(self) -> Self {
        Self(self.0 | Self::IN_NACK_EFF_INT_MSK)
    }
    /// Disable IN NACK effective interrupt.
    #[inline]
    pub const fn disable_in_nack_eff_int(self) -> Self {
        Self(self.0 & !Self::IN_NACK_EFF_INT_MSK)
    }
    /// Check if IN NACK effective interrupt is enabled.
    #[inline]
    pub const fn is_in_nack_eff_int_enabled(self) -> bool {
        (self.0 & Self::IN_NACK_EFF_INT_MSK) != 0
    }
    /// Enable IN token endpoint mismatch interrupt (`INTOKEN_MIS_INT_MSK`).
    #[doc(alias = "INTOKEN_MIS_INT_MSK")]
    #[inline]
    pub const fn enable_intoken_mis_int(self) -> Self {
        Self(self.0 | Self::INTOKEN_MIS_INT_MSK)
    }
    /// Disable IN token endpoint mismatch interrupt.
    #[inline]
    pub const fn disable_intoken_mis_int(self) -> Self {
        Self(self.0 & !Self::INTOKEN_MIS_INT_MSK)
    }
    /// Check if IN token endpoint mismatch interrupt is enabled.
    #[inline]
    pub const fn is_intoken_mis_int_enabled(self) -> bool {
        (self.0 & Self::INTOKEN_MIS_INT_MSK) != 0
    }
    /// Enable IN token while TXFIFO empty interrupt (`RX_INTOKEN_EMPTY_INT_MSK`).
    #[doc(alias = "RX_INTOKEN_EMPTY_INT_MSK")]
    #[inline]
    pub const fn enable_rx_intoken_empty_int(self) -> Self {
        Self(self.0 | Self::RX_INTOKEN_EMPTY_INT_MSK)
    }
    /// Disable IN token while TXFIFO empty interrupt.
    #[inline]
    pub const fn disable_rx_intoken_empty_int(self) -> Self {
        Self(self.0 & !Self::RX_INTOKEN_EMPTY_INT_MSK)
    }
    /// Check if IN token while TXFIFO empty interrupt is enabled.
    #[inline]
    pub const fn is_rx_intoken_empty_int_enabled(self) -> bool {
        (self.0 & Self::RX_INTOKEN_EMPTY_INT_MSK) != 0
    }
    /// Enable timeout interrupt (`TIME_OUT_INT_MSK`).
    #[doc(alias = "TIME_OUT_INT_MSK")]
    #[inline]
    pub const fn enable_time_out_int(self) -> Self {
        Self(self.0 | Self::TIME_OUT_INT_MSK)
    }
    /// Disable timeout interrupt.
    #[inline]
    pub const fn disable_time_out_int(self) -> Self {
        Self(self.0 & !Self::TIME_OUT_INT_MSK)
    }
    /// Check if timeout interrupt is enabled.
    #[inline]
    pub const fn is_time_out_int_enabled(self) -> bool {
        (self.0 & Self::TIME_OUT_INT_MSK) != 0
    }
    /// Enable AHB read/write error interrupt (`AHB_RW_ERR_INT_MSK`).
    #[doc(alias = "AHB_RW_ERR_INT_MSK")]
    #[inline]
    pub const fn enable_ahb_rw_err_int(self) -> Self {
        Self(self.0 | Self::AHB_RW_ERR_INT_MSK)
    }
    /// Disable AHB read/write error interrupt.
    #[inline]
    pub const fn disable_ahb_rw_err_int(self) -> Self {
        Self(self.0 & !Self::AHB_RW_ERR_INT_MSK)
    }
    /// Check if AHB read/write error interrupt is enabled.
    #[inline]
    pub const fn is_ahb_rw_err_int_enabled(self) -> bool {
        (self.0 & Self::AHB_RW_ERR_INT_MSK) != 0
    }
    /// Enable endpoint disabled interrupt (`DIS_EP_INT_MSK`).
    #[doc(alias = "DIS_EP_INT_MSK")]
    #[inline]
    pub const fn enable_dis_ep_int(self) -> Self {
        Self(self.0 | Self::DIS_EP_INT_MSK)
    }
    /// Disable endpoint disabled interrupt.
    #[inline]
    pub const fn disable_dis_ep_int(self) -> Self {
        Self(self.0 & !Self::DIS_EP_INT_MSK)
    }
    /// Check if endpoint disabled interrupt is enabled.
    #[inline]
    pub const fn is_dis_ep_int_enabled(self) -> bool {
        (self.0 & Self::DIS_EP_INT_MSK) != 0
    }
    /// Enable transmit transfer complete interrupt (`TX_COMP_INT_MSK`).
    #[doc(alias = "TX_COMP_INT_MSK")]
    #[inline]
    pub const fn enable_tx_comp_int(self) -> Self {
        Self(self.0 | Self::TX_COMP_INT_MSK)
    }
    /// Disable transmit transfer complete interrupt.
    #[inline]
    pub const fn disable_tx_comp_int(self) -> Self {
        Self(self.0 & !Self::TX_COMP_INT_MSK)
    }
    /// Check if transmit transfer complete interrupt is enabled.
    #[inline]
    pub const fn is_tx_comp_int_enabled(self) -> bool {
        (self.0 & Self::TX_COMP_INT_MSK) != 0
    }
}

/// OUT endpoint interrupt mask register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OutEpIntMsk(u32);

impl OutEpIntMsk {
    /// Clear every bit.
    pub const fn clear_all(self) -> Self {
        Self(0)
    }

    const NYET_INT_MSK: u32 = 1 << 14;
    const NACK_INT_MSK: u32 = 1 << 13;
    const BABBLE_ERR_INT_MSK: u32 = 1 << 12;
    const B2B_SETUP_INT_MSK: u32 = 1 << 6;
    const STS_PHASE_RX_INT_MSK: u32 = 1 << 5;
    const OUT_TOKEN_EP_DIS_INT_MSK: u32 = 1 << 4;
    const SETUP_DONE_INT_MSK: u32 = 1 << 3;
    const AHB_RW_ERR_INT_MSK: u32 = 1 << 2;
    const DIS_EP_INT_MSK: u32 = 1 << 1;
    const RX_COMP_INT_MSK: u32 = 1;

    /// Enable NYET interrupt (`NYET_INT_MSK`).
    #[doc(alias = "NYET_INT_MSK")]
    #[inline]
    pub const fn enable_nyet_int(self) -> Self {
        Self(self.0 | Self::NYET_INT_MSK)
    }
    /// Disable NYET interrupt.
    #[inline]
    pub const fn disable_nyet_int(self) -> Self {
        Self(self.0 & !Self::NYET_INT_MSK)
    }
    /// Check if NYET interrupt is enabled.
    #[inline]
    pub const fn is_nyet_int_enabled(self) -> bool {
        (self.0 & Self::NYET_INT_MSK) != 0
    }
    /// Enable NACK interrupt (`NACK_INT_MSK`).
    #[doc(alias = "NACK_INT_MSK")]
    #[inline]
    pub const fn enable_nack_int(self) -> Self {
        Self(self.0 | Self::NACK_INT_MSK)
    }
    /// Disable NACK interrupt.
    #[inline]
    pub const fn disable_nack_int(self) -> Self {
        Self(self.0 & !Self::NACK_INT_MSK)
    }
    /// Check if NACK interrupt is enabled.
    #[inline]
    pub const fn is_nack_int_enabled(self) -> bool {
        (self.0 & Self::NACK_INT_MSK) != 0
    }
    /// Enable babble error interrupt (`BABBLE_ERR_INT_MSK`).
    #[doc(alias = "BABBLE_ERR_INT_MSK")]
    #[inline]
    pub const fn enable_babble_err_int(self) -> Self {
        Self(self.0 | Self::BABBLE_ERR_INT_MSK)
    }
    /// Disable babble error interrupt.
    #[inline]
    pub const fn disable_babble_err_int(self) -> Self {
        Self(self.0 & !Self::BABBLE_ERR_INT_MSK)
    }
    /// Check if babble error interrupt is enabled.
    #[inline]
    pub const fn is_babble_err_int_enabled(self) -> bool {
        (self.0 & Self::BABBLE_ERR_INT_MSK) != 0
    }
    /// Enable back-to-back setup interrupt (`B2B_SETUP_INT_MSK`).
    #[doc(alias = "B2B_SETUP_INT_MSK")]
    #[inline]
    pub const fn enable_b2b_setup_int(self) -> Self {
        Self(self.0 | Self::B2B_SETUP_INT_MSK)
    }
    /// Disable back-to-back setup interrupt.
    #[inline]
    pub const fn disable_b2b_setup_int(self) -> Self {
        Self(self.0 & !Self::B2B_SETUP_INT_MSK)
    }
    /// Check if back-to-back setup interrupt is enabled.
    #[inline]
    pub const fn is_b2b_setup_int_enabled(self) -> bool {
        (self.0 & Self::B2B_SETUP_INT_MSK) != 0
    }
    /// Enable status phase received interrupt (`STS_PHASE_RX_INT_MSK`).
    #[doc(alias = "STS_PHASE_RX_INT_MSK")]
    #[inline]
    pub const fn enable_sts_phase_rx_int(self) -> Self {
        Self(self.0 | Self::STS_PHASE_RX_INT_MSK)
    }
    /// Disable status phase received interrupt.
    #[inline]
    pub const fn disable_sts_phase_rx_int(self) -> Self {
        Self(self.0 & !Self::STS_PHASE_RX_INT_MSK)
    }
    /// Check if status phase received interrupt is enabled.
    #[inline]
    pub const fn is_sts_phase_rx_int_enabled(self) -> bool {
        (self.0 & Self::STS_PHASE_RX_INT_MSK) != 0
    }
    /// Enable OUT token while endpoint disabled interrupt (`OUT_TOKEN_EP_DIS_INT_MSK`).
    #[doc(alias = "OUT_TOKEN_EP_DIS_INT_MSK")]
    #[inline]
    pub const fn enable_out_token_ep_dis_int(self) -> Self {
        Self(self.0 | Self::OUT_TOKEN_EP_DIS_INT_MSK)
    }
    /// Disable OUT token while endpoint disabled interrupt.
    #[inline]
    pub const fn disable_out_token_ep_dis_int(self) -> Self {
        Self(self.0 & !Self::OUT_TOKEN_EP_DIS_INT_MSK)
    }
    /// Check if OUT token while endpoint disabled interrupt is enabled.
    #[inline]
    pub const fn is_out_token_ep_dis_int_enabled(self) -> bool {
        (self.0 & Self::OUT_TOKEN_EP_DIS_INT_MSK) != 0
    }
    /// Enable setup phase done interrupt (`SETUP_DONE_INT_MSK`).
    #[doc(alias = "SETUP_DONE_INT_MSK")]
    #[inline]
    pub const fn enable_setup_done_int(self) -> Self {
        Self(self.0 | Self::SETUP_DONE_INT_MSK)
    }
    /// Disable setup phase done interrupt.
    #[inline]
    pub const fn disable_setup_done_int(self) -> Self {
        Self(self.0 & !Self::SETUP_DONE_INT_MSK)
    }
    /// Check if setup phase done interrupt is enabled.
    #[inline]
    pub const fn is_setup_done_int_enabled(self) -> bool {
        (self.0 & Self::SETUP_DONE_INT_MSK) != 0
    }
    /// Enable AHB read/write error interrupt (`AHB_RW_ERR_INT_MSK`).
    #[doc(alias = "AHB_RW_ERR_INT_MSK")]
    #[inline]
    pub const fn enable_ahb_rw_err_int(self) -> Self {
        Self(self.0 | Self::AHB_RW_ERR_INT_MSK)
    }
    /// Disable AHB read/write error interrupt.
    #[inline]
    pub const fn disable_ahb_rw_err_int(self) -> Self {
        Self(self.0 & !Self::AHB_RW_ERR_INT_MSK)
    }
    /// Check if AHB read/write error interrupt is enabled.
    #[inline]
    pub const fn is_ahb_rw_err_int_enabled(self) -> bool {
        (self.0 & Self::AHB_RW_ERR_INT_MSK) != 0
    }
    /// Enable endpoint disabled interrupt (`DIS_EP_INT_MSK`).
    #[doc(alias = "DIS_EP_INT_MSK")]
    #[inline]
    pub const fn enable_dis_ep_int(self) -> Self {
        Self(self.0 | Self::DIS_EP_INT_MSK)
    }
    /// Disable endpoint disabled interrupt.
    #[inline]
    pub const fn disable_dis_ep_int(self) -> Self {
        Self(self.0 & !Self::DIS_EP_INT_MSK)
    }
    /// Check if endpoint disabled interrupt is enabled.
    #[inline]
    pub const fn is_dis_ep_int_enabled(self) -> bool {
        (self.0 & Self::DIS_EP_INT_MSK) != 0
    }
    /// Enable receive transfer complete interrupt (`RX_COMP_INT_MSK`).
    #[doc(alias = "RX_COMP_INT_MSK")]
    #[inline]
    pub const fn enable_rx_comp_int(self) -> Self {
        Self(self.0 | Self::RX_COMP_INT_MSK)
    }
    /// Disable receive transfer complete interrupt.
    #[inline]
    pub const fn disable_rx_comp_int(self) -> Self {
        Self(self.0 & !Self::RX_COMP_INT_MSK)
    }
    /// Check if receive transfer complete interrupt is enabled.
    #[inline]
    pub const fn is_rx_comp_int_enabled(self) -> bool {
        (self.0 & Self::RX_COMP_INT_MSK) != 0
    }
}

/// USB Endpoint Interrupt register (read-only).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UsbEpInt(u32);

impl UsbEpInt {
    /// Clear every bit.
    pub const fn clear_all(self) -> Self {
        Self(0)
    }

    const OUT_EP_INT: u32 = 0xFFFF << 16;
    const IN_EP_INT: u32 = 0xFFFF;

    /// Get OUT endpoint interrupt bits (`OUT_EP_INT`).
    #[doc(alias = "OUT_EP_INT")]
    #[inline]
    pub const fn out_ep_int(self) -> u16 {
        ((self.0 & Self::OUT_EP_INT) >> 16) as u16
    }
    /// Check if a specific OUT endpoint has pending interrupt.
    #[inline]
    pub const fn out_ep_pending(self, ep: u8) -> bool {
        assert!(ep <= 4, "Endpoint out of range (expected 0..=4)");
        (self.0 & (1 << (16 + ep as u32))) != 0
    }
    /// Get IN endpoint interrupt bits (`IN_EP_INT`).
    #[doc(alias = "IN_EP_INT")]
    #[inline]
    pub const fn in_ep_int(self) -> u16 {
        (self.0 & Self::IN_EP_INT) as u16
    }
    /// Check if a specific IN endpoint has pending interrupt.
    #[inline]
    pub const fn in_ep_pending(self, ep: u8) -> bool {
        assert!(ep <= 4, "Endpoint out of range (expected 0..=4)");
        (self.0 & (1 << (ep as u32))) != 0
    }
}

/// USB Endpoint Interrupt Mask register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UsbEpIntMsk(u32);

impl UsbEpIntMsk {
    /// Clear every bit.
    pub const fn clear_all(self) -> Self {
        Self(0)
    }

    const OUT_EP_INT: u32 = 0xFFFF << 16;
    const IN_EP_INT: u32 = 0xFFFF;

    /// Set OUT endpoint interrupt mask bits (`OUT_EP_INT`).
    #[doc(alias = "OUT_EP_INT")]
    #[inline]
    pub const fn set_out_ep_mask(self, val: u16) -> Self {
        Self((self.0 & !Self::OUT_EP_INT) | (((val as u32) << 16) & Self::OUT_EP_INT))
    }
    /// Get OUT endpoint interrupt mask bits.
    #[inline]
    pub const fn out_ep_mask(self) -> u16 {
        ((self.0 & Self::OUT_EP_INT) >> 16) as u16
    }
    /// Set IN endpoint interrupt mask bits (`IN_EP_INT`).
    #[doc(alias = "IN_EP_INT")]
    #[inline]
    pub const fn set_in_ep_mask(self, val: u16) -> Self {
        Self((self.0 & !Self::IN_EP_INT) | ((val as u32) & Self::IN_EP_INT))
    }
    /// Get IN endpoint interrupt mask bits.
    #[inline]
    pub const fn in_ep_mask(self) -> u16 {
        (self.0 & Self::IN_EP_INT) as u16
    }
}

/// USB endpoint transfer type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EndpointType {
    /// Control endpoint.
    Control,
    /// Isochronous endpoint.
    Isochronous,
    /// Bulk endpoint.
    Bulk,
    /// Interrupt endpoint.
    Interrupt,
}

/// TXFIFO selection for IN endpoint configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TxFifoSelect {
    /// Non-periodic TXFIFO.
    NonPeriodic,
    /// Periodic TXFIFO 1.
    Periodic1,
    /// Periodic TXFIFO 2.
    ///
    /// The D13x manual encodes periodic TXFIFO 2 as `0x3` (the vendor C SDK
    /// writes `0x2`, which is a reserved value and a bug in that older SDK).
    Periodic2,
}

impl TxFifoSelect {
    const fn encoding(self) -> u32 {
        match self {
            TxFifoSelect::NonPeriodic => 0,
            TxFifoSelect::Periodic1 => 1,
            TxFifoSelect::Periodic2 => 3,
        }
    }
}

/// IN Endpoint Configuration register.
///
/// The shared layout covers IN EP1 through EP4. IN EP0 lacks the `S_DATA1`,
/// `S_DATA0` and `DATA_PID_STS` fields, and its `MPS` field is limited to
/// bits 1:0 (64/32/16/8-byte encodings).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct InEpCfg(u32);

impl InEpCfg {
    /// Zero value.
    pub const fn zero(self) -> Self {
        Self(0)
    }

    const EN_EP: u32 = 1 << 31;
    const DIS_EP: u32 = 1 << 30;
    const S_DATA1: u32 = 1 << 29;
    const S_DATA0: u32 = 1 << 28;
    const S_NACK: u32 = 1 << 27;
    const C_NACK: u32 = 1 << 26;
    const TXFIFO_NUM: u32 = 0xF << 22;
    const EN_STALL: u32 = 1 << 21;
    const EP_TYPE_SEL: u32 = 0x3 << 18;
    const IN_EP_NACK_STS: u32 = 1 << 17;
    const DATA_PID_STS: u32 = 1 << 16;
    const ACT_EP: u32 = 1 << 15;
    const NXT_TX_EP: u32 = 0xF << 11;
    const MPS: u32 = 0x7FF;

    /// Enable endpoint (`EN_EP`).
    #[doc(alias = "EN_EP")]
    #[inline]
    pub const fn enable_ep(self) -> Self {
        Self(self.0 | Self::EN_EP)
    }
    /// Check if endpoint is enabled.
    #[inline]
    pub const fn is_ep_enabled(self) -> bool {
        (self.0 & Self::EN_EP) != 0
    }
    /// Disable endpoint (`DIS_EP`).
    #[doc(alias = "DIS_EP")]
    #[inline]
    pub const fn disable_ep(self) -> Self {
        Self(self.0 | Self::DIS_EP)
    }
    /// Clear the pending-disable bit (`DIS_EP`).
    ///
    /// `EPENA` and `EPDIS` must never be set at the same time: writing
    /// `EPENA` on an endpoint that still has a pending disable is undefined, and
    /// on this controller an IN endpoint then never transmits.
    #[doc(alias = "DIS_EP")]
    #[inline]
    pub const fn clear_ep_disable(self) -> Self {
        Self(self.0 & !Self::DIS_EP)
    }
    /// Check whether an endpoint disable is pending.
    #[inline]
    pub const fn is_ep_disable_pending(self) -> bool {
        (self.0 & Self::DIS_EP) != 0
    }
    /// Set DATA1 PID or select an odd isochronous frame (`S_DATA1`).
    #[doc(alias = "S_DATA1")]
    #[inline]
    pub const fn set_data1_pid(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::S_DATA1)
        } else {
            Self(self.0 & !Self::S_DATA1)
        }
    }
    /// Set DATA0 PID or select an even isochronous frame (`S_DATA0`).
    #[doc(alias = "S_DATA0")]
    #[inline]
    pub const fn set_data0_pid(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::S_DATA0)
        } else {
            Self(self.0 & !Self::S_DATA0)
        }
    }
    /// Set endpoint NACK (`S_NACK`).
    #[doc(alias = "S_NACK")]
    #[inline]
    pub const fn set_nack(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::S_NACK)
        } else {
            Self(self.0 & !Self::S_NACK)
        }
    }
    /// Clear endpoint NACK (`C_NACK`).
    #[doc(alias = "C_NACK")]
    #[inline]
    pub const fn clear_nack(self, clear: bool) -> Self {
        if clear {
            Self(self.0 | Self::C_NACK)
        } else {
            Self(self.0 & !Self::C_NACK)
        }
    }
    /// Set TXFIFO number (`TXFIFO_NUM`).
    #[doc(alias = "TXFIFO_NUM")]
    #[inline]
    pub const fn set_tx_fifo_num(self, val: TxFifoSelect) -> Self {
        Self((self.0 & !Self::TXFIFO_NUM) | (val.encoding() << 22))
    }
    /// Get TXFIFO number.
    #[inline]
    pub const fn tx_fifo_num(self) -> TxFifoSelect {
        match (self.0 & Self::TXFIFO_NUM) >> 22 {
            0 => TxFifoSelect::NonPeriodic,
            1 => TxFifoSelect::Periodic1,
            3 => TxFifoSelect::Periodic2,
            _ => panic!("Invalid TXFIFO selection"),
        }
    }
    /// Enable STALL handshake (`EN_STALL`).
    #[doc(alias = "EN_STALL")]
    #[inline]
    pub const fn enable_stall(self) -> Self {
        Self(self.0 | Self::EN_STALL)
    }
    /// Disable STALL handshake.
    #[inline]
    pub const fn disable_stall(self) -> Self {
        Self(self.0 & !Self::EN_STALL)
    }
    /// Check if STALL handshake is enabled.
    #[inline]
    pub const fn is_stall_enabled(self) -> bool {
        (self.0 & Self::EN_STALL) != 0
    }
    /// Set endpoint type (`EP_TYPE_SEL`).
    #[doc(alias = "EP_TYPE_SEL")]
    #[inline]
    pub const fn set_ep_type(self, ep_type: EndpointType) -> Self {
        Self((self.0 & !Self::EP_TYPE_SEL) | ((ep_type as u32) << 18))
    }
    /// Get endpoint type.
    #[inline]
    pub const fn ep_type(self) -> EndpointType {
        match (self.0 & Self::EP_TYPE_SEL) >> 18 {
            0 => EndpointType::Control,
            1 => EndpointType::Isochronous,
            2 => EndpointType::Bulk,
            _ => EndpointType::Interrupt,
        }
    }
    /// Get IN endpoint NACK status (`IN_EP_NACK_STS`).
    #[doc(alias = "IN_EP_NACK_STS")]
    #[inline]
    pub const fn is_nack_active(self) -> bool {
        (self.0 & Self::IN_EP_NACK_STS) != 0
    }
    /// Get data PID or isochronous frame parity status (`DATA_PID_STS`).
    #[doc(alias = "DATA_PID_STS")]
    #[inline]
    pub const fn data_pid_status(self) -> bool {
        (self.0 & Self::DATA_PID_STS) != 0
    }
    /// Activate endpoint (`ACT_EP`).
    #[doc(alias = "ACT_EP")]
    #[inline]
    pub const fn activate_ep(self) -> Self {
        Self(self.0 | Self::ACT_EP)
    }
    /// Check if endpoint is active.
    #[inline]
    pub const fn is_ep_active(self) -> bool {
        (self.0 & Self::ACT_EP) != 0
    }
    /// Set next transmit endpoint (`NXT_TX_EP`).
    #[doc(alias = "NXT_TX_EP")]
    #[inline]
    pub const fn set_next_tx_ep(self, ep: u8) -> Self {
        assert!(
            ep <= 4,
            "Next transmit endpoint out of range (expected 0..=4)"
        );
        Self((self.0 & !Self::NXT_TX_EP) | ((ep as u32) << 11))
    }
    /// Get next transmit endpoint.
    #[inline]
    pub const fn next_tx_ep(self) -> u8 {
        ((self.0 & Self::NXT_TX_EP) >> 11) as u8
    }
    /// Set maximum packet size (`MPS`).
    #[doc(alias = "MPS")]
    #[inline]
    pub const fn set_mps(self, val: u16) -> Self {
        assert!(val <= 0x7FF, "MPS out of range (expected 0..=2047)");
        Self((self.0 & !Self::MPS) | ((val as u32) & Self::MPS))
    }
    /// Get maximum packet size.
    #[inline]
    pub const fn mps(self) -> u16 {
        (self.0 & Self::MPS) as u16
    }
}

/// OUT Endpoint Configuration register.
///
/// The shared layout covers OUT EP1 through EP4. OUT EP0 lacks the `S_DATA1`,
/// `S_DATA0` and `DATA_PID_STS` fields, and its `MPS` field is limited to
/// bits 1:0 (64/32/16/8-byte encodings).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OutEpCfg(u32);

impl OutEpCfg {
    /// Zero value.
    pub const fn zero(self) -> Self {
        Self(0)
    }

    const EN_EP: u32 = 1 << 31;
    const DIS_EP: u32 = 1 << 30;
    const S_DATA1: u32 = 1 << 29;
    const S_DATA0: u32 = 1 << 28;
    const S_NACK: u32 = 1 << 27;
    const C_NACK: u32 = 1 << 26;
    const EN_STALL: u32 = 1 << 21;
    const EN_SNOOP: u32 = 1 << 20;
    const EP_TYPE_SEL: u32 = 0x3 << 18;
    const OUT_EP_NACK_STS: u32 = 1 << 17;
    const DATA_PID_STS: u32 = 1 << 16;
    const ACT_EP: u32 = 1 << 15;
    const MPS: u32 = 0x7FF;

    /// Enable endpoint (`EN_EP`).
    #[doc(alias = "EN_EP")]
    #[inline]
    pub const fn enable_ep(self) -> Self {
        Self(self.0 | Self::EN_EP)
    }
    /// Check if endpoint is enabled.
    #[inline]
    pub const fn is_ep_enabled(self) -> bool {
        (self.0 & Self::EN_EP) != 0
    }
    /// Disable endpoint (`DIS_EP`).
    #[doc(alias = "DIS_EP")]
    #[inline]
    pub const fn disable_ep(self) -> Self {
        Self(self.0 | Self::DIS_EP)
    }
    /// Clear the pending-disable bit (`DIS_EP`).
    ///
    /// `EPENA` and `EPDIS` must never be set at the same time: writing
    /// `EPENA` on an endpoint that still has a pending disable is undefined, and
    /// on this controller an IN endpoint then never transmits.
    #[doc(alias = "DIS_EP")]
    #[inline]
    pub const fn clear_ep_disable(self) -> Self {
        Self(self.0 & !Self::DIS_EP)
    }
    /// Check whether an endpoint disable is pending.
    #[inline]
    pub const fn is_ep_disable_pending(self) -> bool {
        (self.0 & Self::DIS_EP) != 0
    }
    /// Set DATA1 PID or select an odd isochronous frame (`S_DATA1`).
    #[doc(alias = "S_DATA1")]
    #[inline]
    pub const fn set_data1_pid(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::S_DATA1)
        } else {
            Self(self.0 & !Self::S_DATA1)
        }
    }
    /// Set DATA0 PID or select an even isochronous frame (`S_DATA0`).
    #[doc(alias = "S_DATA0")]
    #[inline]
    pub const fn set_data0_pid(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::S_DATA0)
        } else {
            Self(self.0 & !Self::S_DATA0)
        }
    }
    /// Set endpoint NACK (`S_NACK`).
    #[doc(alias = "S_NACK")]
    #[inline]
    pub const fn set_nack(self, set: bool) -> Self {
        if set {
            Self(self.0 | Self::S_NACK)
        } else {
            Self(self.0 & !Self::S_NACK)
        }
    }
    /// Clear endpoint NACK (`C_NACK`).
    #[doc(alias = "C_NACK")]
    #[inline]
    pub const fn clear_nack(self, clear: bool) -> Self {
        if clear {
            Self(self.0 | Self::C_NACK)
        } else {
            Self(self.0 & !Self::C_NACK)
        }
    }
    /// Enable STALL handshake (`EN_STALL`).
    #[doc(alias = "EN_STALL")]
    #[inline]
    pub const fn enable_stall(self) -> Self {
        Self(self.0 | Self::EN_STALL)
    }
    /// Disable STALL handshake.
    #[inline]
    pub const fn disable_stall(self) -> Self {
        Self(self.0 & !Self::EN_STALL)
    }
    /// Check if STALL handshake is enabled.
    #[inline]
    pub const fn is_stall_enabled(self) -> bool {
        (self.0 & Self::EN_STALL) != 0
    }
    /// Enable snoop mode (`EN_SNOOP`).
    #[doc(alias = "EN_SNOOP")]
    #[inline]
    pub const fn enable_snoop(self) -> Self {
        Self(self.0 | Self::EN_SNOOP)
    }
    /// Disable snoop mode.
    #[inline]
    pub const fn disable_snoop(self) -> Self {
        Self(self.0 & !Self::EN_SNOOP)
    }
    /// Check if snoop mode is enabled.
    #[inline]
    pub const fn is_snoop_enabled(self) -> bool {
        (self.0 & Self::EN_SNOOP) != 0
    }
    /// Set endpoint type (`EP_TYPE_SEL`).
    #[doc(alias = "EP_TYPE_SEL")]
    #[inline]
    pub const fn set_ep_type(self, ep_type: EndpointType) -> Self {
        Self((self.0 & !Self::EP_TYPE_SEL) | ((ep_type as u32) << 18))
    }
    /// Get endpoint type.
    #[inline]
    pub const fn ep_type(self) -> EndpointType {
        match (self.0 & Self::EP_TYPE_SEL) >> 18 {
            0 => EndpointType::Control,
            1 => EndpointType::Isochronous,
            2 => EndpointType::Bulk,
            _ => EndpointType::Interrupt,
        }
    }
    /// Get OUT endpoint NACK status (`OUT_EP_NACK_STS`).
    #[doc(alias = "OUT_EP_NACK_STS")]
    #[inline]
    pub const fn is_nack_active(self) -> bool {
        (self.0 & Self::OUT_EP_NACK_STS) != 0
    }
    /// Get data PID or isochronous frame parity status (`DATA_PID_STS`).
    #[doc(alias = "DATA_PID_STS")]
    #[inline]
    pub const fn data_pid_status(self) -> bool {
        (self.0 & Self::DATA_PID_STS) != 0
    }
    /// Activate endpoint (`ACT_EP`).
    #[doc(alias = "ACT_EP")]
    #[inline]
    pub const fn activate_ep(self) -> Self {
        Self(self.0 | Self::ACT_EP)
    }
    /// Check if endpoint is active.
    #[inline]
    pub const fn is_ep_active(self) -> bool {
        (self.0 & Self::ACT_EP) != 0
    }
    /// Set maximum packet size (`MPS`).
    #[doc(alias = "MPS")]
    #[inline]
    pub const fn set_mps(self, val: u16) -> Self {
        assert!(val <= 0x7FF, "MPS out of range (expected 0..=2047)");
        Self((self.0 & !Self::MPS) | ((val as u32) & Self::MPS))
    }
    /// Get maximum packet size.
    #[inline]
    pub const fn mps(self) -> u16 {
        (self.0 & Self::MPS) as u16
    }
}

/// IN Endpoint Interrupt register (Write-1-to-Clear).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct InEpInt(u32);

impl InEpInt {
    /// Every write-1-to-clear endpoint interrupt bit.
    pub const fn clear_all(self) -> Self {
        Self(u32::MAX)
    }

    const NACK_INT: u32 = 1 << 13;
    const BABBLE_ERR_INT: u32 = 1 << 12;
    const TXFIFO_EMP_INT: u32 = 1 << 7;
    const IN_NACK_EFF_INT: u32 = 1 << 6;
    const INTOKEN_MIS_INT: u32 = 1 << 5;
    const RX_INTOKEN_EMPTY_INT: u32 = 1 << 4;
    const TIME_OUT_INT: u32 = 1 << 3;
    const AHB_RW_ERR_INT: u32 = 1 << 2;
    const DIS_EP_INT: u32 = 1 << 1;
    const TX_COMP_INT: u32 = 1;

    /// Check if NACK interrupt is pending (`NACK_INT`).
    #[doc(alias = "NACK_INT")]
    #[inline]
    pub const fn is_nack_int(self) -> bool {
        (self.0 & Self::NACK_INT) != 0
    }
    /// Clear NACK interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_nack_int(self) -> Self {
        Self(self.0 | Self::NACK_INT)
    }
    /// Check if babble error interrupt is pending (`BABBLE_ERR_INT`).
    #[doc(alias = "BABBLE_ERR_INT")]
    #[inline]
    pub const fn is_babble_err_int(self) -> bool {
        (self.0 & Self::BABBLE_ERR_INT) != 0
    }
    /// Clear babble error interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_babble_err_int(self) -> Self {
        Self(self.0 | Self::BABBLE_ERR_INT)
    }
    /// Check if TXFIFO empty interrupt is pending (`TXFIFO_EMP_INT`).
    #[doc(alias = "TXFIFO_EMP_INT")]
    #[inline]
    pub const fn is_txfifo_emp_int(self) -> bool {
        (self.0 & Self::TXFIFO_EMP_INT) != 0
    }
    /// Clear TXFIFO empty interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_txfifo_emp_int(self) -> Self {
        Self(self.0 | Self::TXFIFO_EMP_INT)
    }
    /// Check if IN NACK effective interrupt is pending (`IN_NACK_EFF_INT`).
    #[doc(alias = "IN_NACK_EFF_INT")]
    #[inline]
    pub const fn is_in_nack_eff_int(self) -> bool {
        (self.0 & Self::IN_NACK_EFF_INT) != 0
    }
    /// Clear IN NACK effective interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_in_nack_eff_int(self) -> Self {
        Self(self.0 | Self::IN_NACK_EFF_INT)
    }
    /// Check if IN token endpoint mismatch interrupt is pending (`INTOKEN_MIS_INT`).
    #[doc(alias = "INTOKEN_MIS_INT")]
    #[inline]
    pub const fn is_intoken_mis_int(self) -> bool {
        (self.0 & Self::INTOKEN_MIS_INT) != 0
    }
    /// Clear IN token endpoint mismatch interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_intoken_mis_int(self) -> Self {
        Self(self.0 | Self::INTOKEN_MIS_INT)
    }
    /// Check if IN token while TXFIFO empty interrupt is pending (`RX_INTOKEN_EMPTY_INT`).
    #[doc(alias = "RX_INTOKEN_EMPTY_INT")]
    #[inline]
    pub const fn is_rx_intoken_empty_int(self) -> bool {
        (self.0 & Self::RX_INTOKEN_EMPTY_INT) != 0
    }
    /// Clear IN token while TXFIFO empty interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_rx_intoken_empty_int(self) -> Self {
        Self(self.0 | Self::RX_INTOKEN_EMPTY_INT)
    }
    /// Check if timeout interrupt is pending (`TIME_OUT_INT`).
    #[doc(alias = "TIME_OUT_INT")]
    #[inline]
    pub const fn is_time_out_int(self) -> bool {
        (self.0 & Self::TIME_OUT_INT) != 0
    }
    /// Clear timeout interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_time_out_int(self) -> Self {
        Self(self.0 | Self::TIME_OUT_INT)
    }
    /// Check if AHB read/write error interrupt is pending (`AHB_RW_ERR_INT`).
    #[doc(alias = "AHB_RW_ERR_INT")]
    #[inline]
    pub const fn is_ahb_rw_err_int(self) -> bool {
        (self.0 & Self::AHB_RW_ERR_INT) != 0
    }
    /// Clear AHB read/write error interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_ahb_rw_err_int(self) -> Self {
        Self(self.0 | Self::AHB_RW_ERR_INT)
    }
    /// Check if endpoint disabled interrupt is pending (`DIS_EP_INT`).
    #[doc(alias = "DIS_EP_INT")]
    #[inline]
    pub const fn is_dis_ep_int(self) -> bool {
        (self.0 & Self::DIS_EP_INT) != 0
    }
    /// Clear endpoint disabled interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_dis_ep_int(self) -> Self {
        Self(self.0 | Self::DIS_EP_INT)
    }
    /// Check if transmit transfer complete interrupt is pending (`TX_COMP_INT`).
    #[doc(alias = "TX_COMP_INT")]
    #[inline]
    pub const fn is_tx_comp_int(self) -> bool {
        (self.0 & Self::TX_COMP_INT) != 0
    }
    /// Clear transmit transfer complete interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_tx_comp_int(self) -> Self {
        Self(self.0 | Self::TX_COMP_INT)
    }
}

/// OUT Endpoint Interrupt register (Write-1-to-Clear).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OutEpInt(u32);

impl OutEpInt {
    /// Every write-1-to-clear endpoint interrupt bit.
    pub const fn clear_all(self) -> Self {
        Self(u32::MAX)
    }

    const SETUP_PKT_RX_INT: u32 = 1 << 15;
    const NYET_INT: u32 = 1 << 14;
    const NACK_INT: u32 = 1 << 13;
    const BABBLE_ERR_INT: u32 = 1 << 12;
    const PKT_DROP_STS_INT: u32 = 1 << 11;
    const B2B_SETUP_INT: u32 = 1 << 6;
    const STS_PHASE_RX_INT: u32 = 1 << 5;
    const OUT_TOKEN_EP_DIS_INT: u32 = 1 << 4;
    const SETUP_DONE_INT: u32 = 1 << 3;
    const AHB_RW_ERR_INT: u32 = 1 << 2;
    const DIS_EP_INT: u32 = 1 << 1;
    const RX_COMP_INT: u32 = 1;

    /// Check if setup packet received interrupt is pending (`SETUP_PKT_RX_INT`).
    #[doc(alias = "SETUP_PKT_RX_INT")]
    #[inline]
    pub const fn is_setup_pkt_rx_int(self) -> bool {
        (self.0 & Self::SETUP_PKT_RX_INT) != 0
    }
    /// Clear setup packet received interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_setup_pkt_rx_int(self) -> Self {
        Self(self.0 | Self::SETUP_PKT_RX_INT)
    }
    /// Check if NYET interrupt is pending (`NYET_INT`).
    #[doc(alias = "NYET_INT")]
    #[inline]
    pub const fn is_nyet_int(self) -> bool {
        (self.0 & Self::NYET_INT) != 0
    }
    /// Clear NYET interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_nyet_int(self) -> Self {
        Self(self.0 | Self::NYET_INT)
    }
    /// Check if NACK interrupt is pending (`NACK_INT`).
    #[doc(alias = "NACK_INT")]
    #[inline]
    pub const fn is_nack_int(self) -> bool {
        (self.0 & Self::NACK_INT) != 0
    }
    /// Clear NACK interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_nack_int(self) -> Self {
        Self(self.0 | Self::NACK_INT)
    }
    /// Check if babble error interrupt is pending (`BABBLE_ERR_INT`).
    #[doc(alias = "BABBLE_ERR_INT")]
    #[inline]
    pub const fn is_babble_err_int(self) -> bool {
        (self.0 & Self::BABBLE_ERR_INT) != 0
    }
    /// Clear babble error interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_babble_err_int(self) -> Self {
        Self(self.0 | Self::BABBLE_ERR_INT)
    }
    /// Check if packet dropped status interrupt is pending (`PKT_DROP_STS_INT`).
    #[doc(alias = "PKT_DROP_STS_INT")]
    #[inline]
    pub const fn is_pkt_drop_sts_int(self) -> bool {
        (self.0 & Self::PKT_DROP_STS_INT) != 0
    }
    /// Clear packet dropped status interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_pkt_drop_sts_int(self) -> Self {
        Self(self.0 | Self::PKT_DROP_STS_INT)
    }
    /// Check if back-to-back setup interrupt is pending (`B2B_SETUP_INT`).
    #[doc(alias = "B2B_SETUP_INT")]
    #[inline]
    pub const fn is_b2b_setup_int(self) -> bool {
        (self.0 & Self::B2B_SETUP_INT) != 0
    }
    /// Clear back-to-back setup interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_b2b_setup_int(self) -> Self {
        Self(self.0 | Self::B2B_SETUP_INT)
    }
    /// Check if status phase received interrupt is pending (`STS_PHASE_RX_INT`).
    #[doc(alias = "STS_PHASE_RX_INT")]
    #[inline]
    pub const fn is_sts_phase_rx_int(self) -> bool {
        (self.0 & Self::STS_PHASE_RX_INT) != 0
    }
    /// Clear status phase received interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_sts_phase_rx_int(self) -> Self {
        Self(self.0 | Self::STS_PHASE_RX_INT)
    }
    /// Check if OUT token while endpoint disabled interrupt is pending (`OUT_TOKEN_EP_DIS_INT`).
    #[doc(alias = "OUT_TOKEN_EP_DIS_INT")]
    #[inline]
    pub const fn is_out_token_ep_dis_int(self) -> bool {
        (self.0 & Self::OUT_TOKEN_EP_DIS_INT) != 0
    }
    /// Clear OUT token while endpoint disabled interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_out_token_ep_dis_int(self) -> Self {
        Self(self.0 | Self::OUT_TOKEN_EP_DIS_INT)
    }
    /// Check if setup phase done interrupt is pending (`SETUP_DONE_INT`).
    #[doc(alias = "SETUP_DONE_INT")]
    #[inline]
    pub const fn is_setup_done_int(self) -> bool {
        (self.0 & Self::SETUP_DONE_INT) != 0
    }
    /// Clear setup phase done interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_setup_done_int(self) -> Self {
        Self(self.0 | Self::SETUP_DONE_INT)
    }
    /// Check if AHB read/write error interrupt is pending (`AHB_RW_ERR_INT`).
    #[doc(alias = "AHB_RW_ERR_INT")]
    #[inline]
    pub const fn is_ahb_rw_err_int(self) -> bool {
        (self.0 & Self::AHB_RW_ERR_INT) != 0
    }
    /// Clear AHB read/write error interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_ahb_rw_err_int(self) -> Self {
        Self(self.0 | Self::AHB_RW_ERR_INT)
    }
    /// Check if endpoint disabled interrupt is pending (`DIS_EP_INT`).
    #[doc(alias = "DIS_EP_INT")]
    #[inline]
    pub const fn is_dis_ep_int(self) -> bool {
        (self.0 & Self::DIS_EP_INT) != 0
    }
    /// Clear endpoint disabled interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_dis_ep_int(self) -> Self {
        Self(self.0 | Self::DIS_EP_INT)
    }
    /// Check if receive transfer complete interrupt is pending (`RX_COMP_INT`).
    #[doc(alias = "RX_COMP_INT")]
    #[inline]
    pub const fn is_rx_comp_int(self) -> bool {
        (self.0 & Self::RX_COMP_INT) != 0
    }
    /// Clear receive transfer complete interrupt (Write-1-to-Clear).
    #[inline]
    pub const fn clear_rx_comp_int(self) -> Self {
        Self(self.0 | Self::RX_COMP_INT)
    }
}

/// IN EP0 and EP1 Transfer Size register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct InEp01TsfSiz(u32);

impl InEp01TsfSiz {
    /// Zero value.
    pub const fn zero(self) -> Self {
        Self(0)
    }

    const TX_PKT_CNT: u32 = 0x3 << 19;
    const XFR_SIZ: u32 = 0x7F;

    /// Set transmit packet count (`TX_PKT_CNT`).
    #[doc(alias = "TX_PKT_CNT")]
    #[inline]
    pub const fn set_tx_pkt_cnt(self, count: u8) -> Self {
        assert!(
            count < 4,
            "Transmit packet count out of range (expected 0..=3)"
        );
        Self((self.0 & !Self::TX_PKT_CNT) | ((count as u32) << 19))
    }
    /// Get transmit packet count.
    #[inline]
    pub const fn tx_pkt_cnt(self) -> u8 {
        ((self.0 & Self::TX_PKT_CNT) >> 19) as u8
    }
    /// Set transfer size in bytes (`XFR_SIZ`).
    #[doc(alias = "XFR_SIZ")]
    #[inline]
    pub const fn set_xfr_siz(self, size: u8) -> Self {
        assert!(size < 0x80, "Transfer size out of range (expected 0..=127)");
        Self((self.0 & !Self::XFR_SIZ) | size as u32)
    }
    /// Get transfer size in bytes.
    #[inline]
    pub const fn xfr_siz(self) -> u8 {
        (self.0 & Self::XFR_SIZ) as u8
    }
}

/// IN Endpoint Transfer Size register for endpoints 2 through 4.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct InEpTsfSiz(u32);

impl InEpTsfSiz {
    /// Zero value.
    pub const fn zero(self) -> Self {
        Self(0)
    }

    const TX_MULTI_PKT: u32 = 0x3 << 29;
    const PKT_CNT: u32 = 0x3FF << 19;
    const XFR_SIZ: u32 = 0x7FFFF;

    /// Set transmit packets per microframe (`TX_MULTI_PKT`).
    #[doc(alias = "TX_MULTI_PKT")]
    #[inline]
    pub const fn set_tx_multi_pkt(self, count: u8) -> Self {
        assert!(
            count >= 1 && count <= 3,
            "Transmit packet count out of range (expected 1..=3)"
        );
        Self((self.0 & !Self::TX_MULTI_PKT) | ((count as u32) << 29))
    }
    /// Get transmit packets per microframe.
    #[inline]
    pub const fn tx_multi_pkt(self) -> u8 {
        ((self.0 & Self::TX_MULTI_PKT) >> 29) as u8
    }
    /// Set packet count (`PKT_CNT`).
    #[doc(alias = "PKT_CNT")]
    #[inline]
    pub const fn set_pkt_cnt(self, count: u16) -> Self {
        assert!(
            count < 0x400,
            "Packet count out of range (expected 0..=1023)"
        );
        Self((self.0 & !Self::PKT_CNT) | ((count as u32) << 19))
    }
    /// Get packet count.
    #[inline]
    pub const fn pkt_cnt(self) -> u16 {
        ((self.0 & Self::PKT_CNT) >> 19) as u16
    }
    /// Set transfer size in bytes (`XFR_SIZ`).
    #[doc(alias = "XFR_SIZ")]
    #[inline]
    pub const fn set_xfr_siz(self, size: u32) -> Self {
        assert!(
            size < 0x8_0000,
            "Transfer size out of range (expected 0..=524287)"
        );
        Self((self.0 & !Self::XFR_SIZ) | size)
    }
    /// Get transfer size in bytes.
    #[inline]
    pub const fn xfr_siz(self) -> u32 {
        self.0 & Self::XFR_SIZ
    }
}

/// OUT EP0 Transfer Size register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OutEp0TsfSiz(u32);

impl OutEp0TsfSiz {
    /// Zero value.
    pub const fn zero(self) -> Self {
        Self(0)
    }

    const SUP_PKT_CNT: u32 = 0x3 << 29;
    const PKT_CNT: u32 = 1 << 19;
    const XFR_SIZ: u32 = 0x7F;

    /// Set setup packet count (`SUP_PKT_CNT`).
    #[doc(alias = "SUP_PKT_CNT")]
    #[inline]
    pub const fn set_sup_pkt_cnt(self, count: u8) -> Self {
        assert!(
            count <= 3,
            "Setup packet count out of range (expected 0..=3)"
        );
        Self((self.0 & !Self::SUP_PKT_CNT) | ((count as u32) << 29))
    }
    /// Get setup packet count.
    #[inline]
    pub const fn sup_pkt_cnt(self) -> u8 {
        ((self.0 & Self::SUP_PKT_CNT) >> 29) as u8
    }
    /// Set packet count (`PKT_CNT`).
    #[doc(alias = "PKT_CNT")]
    #[inline]
    pub const fn set_pkt_cnt(self, count: bool) -> Self {
        if count {
            Self(self.0 | Self::PKT_CNT)
        } else {
            Self(self.0 & !Self::PKT_CNT)
        }
    }
    /// Get packet count.
    #[inline]
    pub const fn pkt_cnt(self) -> bool {
        (self.0 & Self::PKT_CNT) != 0
    }
    /// Set transfer size in bytes (`XFR_SIZ`).
    #[doc(alias = "XFR_SIZ")]
    #[inline]
    pub const fn set_xfr_siz(self, size: u8) -> Self {
        assert!(size < 0x80, "Transfer size out of range (expected 0..=127)");
        Self((self.0 & !Self::XFR_SIZ) | size as u32)
    }
    /// Get transfer size in bytes.
    #[inline]
    pub const fn xfr_siz(self) -> u8 {
        (self.0 & Self::XFR_SIZ) as u8
    }
}

/// OUT Endpoint Transfer Size register for endpoints 1 through 4.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OutEpTsfSiz(u32);

impl OutEpTsfSiz {
    /// Zero value.
    pub const fn zero(self) -> Self {
        Self(0)
    }

    const RX_DTA_PID: u32 = 0x3 << 29;
    const PKT_CNT: u32 = 0x3FF << 19;
    const XFR_SIZ: u32 = 0x7FFFF;

    /// Set EP0 setup packet count (`SUP_PKT_CNT`). The same bits are the
    /// received-data PID for non-control OUT endpoints.
    #[inline]
    pub const fn set_sup_pkt_cnt(self, count: u8) -> Self {
        assert!(
            count <= 3,
            "Setup packet count out of range (expected 0..=3)"
        );
        Self((self.0 & !Self::RX_DTA_PID) | ((count as u32) << 29))
    }

    /// Get received data PID (`RX_DTA_PID`).
    #[doc(alias = "RX_DTA_PID")]
    #[inline]
    pub const fn rx_data_pid(self) -> OutDataPid {
        match (self.0 & Self::RX_DTA_PID) >> 29 {
            0 => OutDataPid::Data0,
            1 => OutDataPid::Data2,
            2 => OutDataPid::Data1,
            _ => OutDataPid::Mdata,
        }
    }
    /// Set packet count (`PKT_CNT`).
    #[doc(alias = "PKT_CNT")]
    #[inline]
    pub const fn set_pkt_cnt(self, count: u16) -> Self {
        assert!(
            count < 0x400,
            "Packet count out of range (expected 0..=1023)"
        );
        Self((self.0 & !Self::PKT_CNT) | ((count as u32) << 19))
    }
    /// Get packet count.
    #[inline]
    pub const fn pkt_cnt(self) -> u16 {
        ((self.0 & Self::PKT_CNT) >> 19) as u16
    }
    /// Set transfer size in bytes (`XFR_SIZ`).
    #[doc(alias = "XFR_SIZ")]
    #[inline]
    pub const fn set_xfr_siz(self, size: u32) -> Self {
        assert!(
            size < 0x8_0000,
            "Transfer size out of range (expected 0..=524287)"
        );
        Self((self.0 & !Self::XFR_SIZ) | size)
    }
    /// Get transfer size in bytes.
    #[inline]
    pub const fn xfr_siz(self) -> u32 {
        self.0 & Self::XFR_SIZ
    }
}

/// IN Endpoint TXFIFO Status register (read-only).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct InEpTxFifoSta(u32);

impl InEpTxFifoSta {
    /// Zero value.
    pub const fn zero(self) -> Self {
        Self(0)
    }

    const IN_EP_TXFIFO_STS: u32 = 0xFFFF;

    /// Get IN endpoint TXFIFO status (`IN_EP_TXFIFO_STS`).
    #[doc(alias = "IN_EP_TXFIFO_STS")]
    #[inline]
    pub const fn in_ep_txfifo_sts(self) -> u16 {
        (self.0 & Self::IN_EP_TXFIFO_STS) as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::offset_of;

    #[test]
    fn test_register_block_offsets() {
        assert_eq!(offset_of!(RegisterBlock, ahb_basic), 0x000);
        assert_eq!(offset_of!(RegisterBlock, usb_dev_init), 0x004);
        assert_eq!(offset_of!(RegisterBlock, usb_phy_if), 0x008);
        assert_eq!(offset_of!(RegisterBlock, usb_ulpi_phy), 0x00C);
        assert_eq!(offset_of!(RegisterBlock, usb_int_sts), 0x010);
        assert_eq!(offset_of!(RegisterBlock, usb_int_msk), 0x014);
        assert_eq!(offset_of!(RegisterBlock, rxfifo_siz), 0x018);
        assert_eq!(offset_of!(RegisterBlock, rxfifo_sts), 0x01C);
        assert_eq!(offset_of!(RegisterBlock, nptxfifo_siz), 0x020);
        assert_eq!(offset_of!(RegisterBlock, nptxfifo_sts), 0x024);
        assert_eq!(offset_of!(RegisterBlock, ptxfifo_siz), 0x028);
        assert_eq!(offset_of!(RegisterBlock, rxfifo_sts_pop), 0x030);
        assert_eq!(offset_of!(RegisterBlock, phy_clk_ctl), 0x040);
        assert_eq!(offset_of!(RegisterBlock, usb_dev_conf), 0x200);
        assert_eq!(offset_of!(RegisterBlock, usb_dev_func), 0x204);
        assert_eq!(offset_of!(RegisterBlock, usb_line_sts), 0x208);
        assert_eq!(offset_of!(RegisterBlock, inep_int_msk), 0x20C);
        assert_eq!(offset_of!(RegisterBlock, outep_int_msk), 0x210);
        assert_eq!(offset_of!(RegisterBlock, usb_ep_int), 0x214);
        assert_eq!(offset_of!(RegisterBlock, usb_ep_int_msk), 0x218);
        assert_eq!(offset_of!(RegisterBlock, in_ep_cfg), 0x220);
        assert_eq!(offset_of!(RegisterBlock, out_ep_cfg), 0x240);
        assert_eq!(offset_of!(RegisterBlock, in_ep_int), 0x260);
        assert_eq!(offset_of!(RegisterBlock, out_ep_int), 0x280);
        assert_eq!(offset_of!(RegisterBlock, in_ep_tsf_siz), 0x2A0);
        assert_eq!(offset_of!(RegisterBlock, out_ep_tsf_siz), 0x2C0);
        assert_eq!(offset_of!(RegisterBlock, in_ep_dma), 0x300);
        assert_eq!(offset_of!(RegisterBlock, out_ep_dma), 0x320);
        assert_eq!(offset_of!(RegisterBlock, in_ep_txfifo_sta), 0x340);
        assert_eq!(offset_of!(RegisterBlock, tkn_queue), 0x360);
        assert_eq!(offset_of!(RegisterBlock, version), 0xFFC);
    }
}
