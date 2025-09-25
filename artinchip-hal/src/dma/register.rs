//! DMA register blocks and registers.

use volatile_register::{RO, RW};

/// Direct Memory Access register block.
#[repr(C)]
pub struct RegisterBlock {
    /// DMA interrupt request enable register (`DMA_IRQ_EN`).
    #[doc(alias = "DMA_IRQ_EN")]
    pub int_enable: RW<IntEnable>,
    _reserved0: [u8; 0xC],
    /// DMA interrupt request status register (`DMA_IRQ_STA`).
    #[doc(alias = "DMA_IRQ_STA")]
    pub int_status: RW<IntStatus>,
    _reserved1: [u8; 0xC],
    /// DMA memory configuration register (`DMA_MEM_CFG`).
    #[doc(alias = "DMA_MEM_CFG")]
    pub mem_cfg: RW<MemCfg>,
    _reserved2: [u8; 0x4],
    /// DMA gate register (`DMA_GATE`).
    #[doc(alias = "DMA_GATE")]
    pub gate: RW<Gate>,
    _reserved3: [u8; 0x4],
    /// DMA channel status register (`DMA_CH_STA`).
    #[doc(alias = "DMA_CH_STA")]
    pub ch_status: RO<ChStatus>,
    _reserved4: [u8; 0xCC],
    /// DMA channel groups.
    pub groups: [DmaCh; 8],
}

/// DMA channel register.
#[repr(C)]
pub struct DmaCh {
    /// DMA channel enable register (`DMA_CH_EN`).
    #[doc(alias = "DMA_CH_EN")]
    pub enable: RW<ChEnable>,
    /// DMA channel pause register (`DMA_CH_PAUSE`).
    #[doc(alias = "DMA_CH_PAUSE")]
    pub pause: RW<ChPause>,
    /// DMA channel task address register (`DMA_CH_TASK`).
    #[doc(alias = "DMA_CH_TASK")]
    pub task_addr: RW<u32>,
    /// DMA channel configuration register (`DMA_CH_CFG`).
    #[doc(alias = "DMA_CH_CFG")]
    pub config: RW<ChConfig>,
    /// DMA source address register (`DMA_SRC_ADDR`).
    #[doc(alias = "DMA_SRC_ADDR")]
    pub src_addr: RO<u32>,
    /// DMA destination address register (`DMA_SINK_ADDR`).
    #[doc(alias = "DMA_SINK_ADDR")]
    pub sink_addr: RO<u32>,
    /// DMA byte count left register (`DMA_BCNT_LEFT`).
    #[doc(alias = "DMA_BCNT_LEFT")]
    pub bcnt_left: RO<u32>,
    _reserved0: [u8; 0xC],
    /// DMA mode register (`DMA_MODE`).
    #[doc(alias = "DMA_MODE")]
    pub mode: RW<ChMode>,
    /// DMA former descriptor address register (`DMA_FDES_ADDR`).
    #[doc(alias = "DMA_FDES_ADDR")]
    pub fdes_addr: RO<u32>,
    /// DMA package number register (`DMA_PKG_NUM`).
    #[doc(alias = "DMA_PKG_NUM")]
    pub pkg_num: RO<u32>,
    /// DMA memory set register (`DMA_MEM_SET`).
    #[doc(alias = "DMA_MEM_SET")]
    pub mem_set: RW<u32>,
    _reserved1: [u8; 0x8],
}

/// Dma Channel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DmaChannel {
    Ch0,
    Ch1,
    Ch2,
    Ch3,
    Ch4,
    Ch5,
    Ch6,
    Ch7,
}

/// DMA interrupt request enable register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IntEnable(u32);

impl IntEnable {
    const CH7_ADDR_REQ_ERR_EN: u32 = 0x1 << 31;
    const CH7_ALL_FINISH_IRQ_EN: u32 = 0x1 << 30;
    const CH7_TASK_FINISH_IRQ_EN: u32 = 0x1 << 29;
    const CH7_TASK_HALF_IRQ_EN: u32 = 0x1 << 28;
    const CH6_ADDR_REQ_ERR_EN: u32 = 0x1 << 27;
    const CH6_ALL_FINISH_IRQ_EN: u32 = 0x1 << 26;
    const CH6_TASK_FINISH_IRQ_EN: u32 = 0x1 << 25;
    const CH6_TASK_HALF_IRQ_EN: u32 = 0x1 << 24;
    const CH5_ADDR_REQ_ERR_EN: u32 = 0x1 << 23;
    const CH5_ALL_FINISH_IRQ_EN: u32 = 0x1 << 22;
    const CH5_TASK_FINISH_IRQ_EN: u32 = 0x1 << 21;
    const CH5_TASK_HALF_IRQ_EN: u32 = 0x1 << 20;
    const CH4_ADDR_REQ_ERR_EN: u32 = 0x1 << 19;
    const CH4_ALL_FINISH_IRQ_EN: u32 = 0x1 << 18;
    const CH4_TASK_FINISH_IRQ_EN: u32 = 0x1 << 17;
    const CH4_TASK_HALF_IRQ_EN: u32 = 0x1 << 16;
    const CH3_ADDR_REQ_ERR_EN: u32 = 0x1 << 15;
    const CH3_ALL_FINISH_IRQ_EN: u32 = 0x1 << 14;
    const CH3_TASK_FINISH_IRQ_EN: u32 = 0x1 << 13;
    const CH3_TASK_HALF_IRQ_EN: u32 = 0x1 << 12;
    const CH2_ADDR_REQ_ERR_EN: u32 = 0x1 << 11;
    const CH2_ALL_FINISH_IRQ_EN: u32 = 0x1 << 10;
    const CH2_TASK_FINISH_IRQ_EN: u32 = 0x1 << 9;
    const CH2_TASK_HALF_IRQ_EN: u32 = 0x1 << 8;
    const CH1_ADDR_REQ_ERR_EN: u32 = 0x1 << 7;
    const CH1_ALL_FINISH_IRQ_EN: u32 = 0x1 << 6;
    const CH1_TASK_FINISH_IRQ_EN: u32 = 0x1 << 5;
    const CH1_TASK_HALF_IRQ_EN: u32 = 0x1 << 4;
    const CH0_ADDR_REQ_ERR_EN: u32 = 0x1 << 3;
    const CH0_ALL_FINISH_IRQ_EN: u32 = 0x1 << 2;
    const CH0_TASK_FINISH_IRQ_EN: u32 = 0x1 << 1;
    const CH0_TASK_HALF_IRQ_EN: u32 = 0x1;

    /// Enable channel address request error interrupt.
    #[inline]
    pub const fn enable_channel_addr_req_err(self, channel: DmaChannel) -> Self {
        let mask = match channel {
            DmaChannel::Ch0 => Self::CH0_ADDR_REQ_ERR_EN,
            DmaChannel::Ch1 => Self::CH1_ADDR_REQ_ERR_EN,
            DmaChannel::Ch2 => Self::CH2_ADDR_REQ_ERR_EN,
            DmaChannel::Ch3 => Self::CH3_ADDR_REQ_ERR_EN,
            DmaChannel::Ch4 => Self::CH4_ADDR_REQ_ERR_EN,
            DmaChannel::Ch5 => Self::CH5_ADDR_REQ_ERR_EN,
            DmaChannel::Ch6 => Self::CH6_ADDR_REQ_ERR_EN,
            DmaChannel::Ch7 => Self::CH7_ADDR_REQ_ERR_EN,
        };
        Self(self.0 | mask)
    }
    /// Disable channel address request error interrupt.
    #[inline]
    pub const fn disable_channel_addr_req_err(self, channel: DmaChannel) -> Self {
        let mask = match channel {
            DmaChannel::Ch0 => Self::CH0_ADDR_REQ_ERR_EN,
            DmaChannel::Ch1 => Self::CH1_ADDR_REQ_ERR_EN,
            DmaChannel::Ch2 => Self::CH2_ADDR_REQ_ERR_EN,
            DmaChannel::Ch3 => Self::CH3_ADDR_REQ_ERR_EN,
            DmaChannel::Ch4 => Self::CH4_ADDR_REQ_ERR_EN,
            DmaChannel::Ch5 => Self::CH5_ADDR_REQ_ERR_EN,
            DmaChannel::Ch6 => Self::CH6_ADDR_REQ_ERR_EN,
            DmaChannel::Ch7 => Self::CH7_ADDR_REQ_ERR_EN,
        };
        Self(self.0 & !mask)
    }
    /// Check if channel address request error interrupt is enabled.
    #[inline]
    pub const fn is_addr_req_err_enabled(self, channel: DmaChannel) -> bool {
        let mask = match channel {
            DmaChannel::Ch0 => Self::CH0_ADDR_REQ_ERR_EN,
            DmaChannel::Ch1 => Self::CH1_ADDR_REQ_ERR_EN,
            DmaChannel::Ch2 => Self::CH2_ADDR_REQ_ERR_EN,
            DmaChannel::Ch3 => Self::CH3_ADDR_REQ_ERR_EN,
            DmaChannel::Ch4 => Self::CH4_ADDR_REQ_ERR_EN,
            DmaChannel::Ch5 => Self::CH5_ADDR_REQ_ERR_EN,
            DmaChannel::Ch6 => Self::CH6_ADDR_REQ_ERR_EN,
            DmaChannel::Ch7 => Self::CH7_ADDR_REQ_ERR_EN,
        };
        (self.0 & mask) != 0
    }
    /// Enable channel all finish interrupt.
    #[inline]
    pub const fn enable_channel_all_finish(self, channel: DmaChannel) -> Self {
        let mask = match channel {
            DmaChannel::Ch0 => Self::CH0_ALL_FINISH_IRQ_EN,
            DmaChannel::Ch1 => Self::CH1_ALL_FINISH_IRQ_EN,
            DmaChannel::Ch2 => Self::CH2_ALL_FINISH_IRQ_EN,
            DmaChannel::Ch3 => Self::CH3_ALL_FINISH_IRQ_EN,
            DmaChannel::Ch4 => Self::CH4_ALL_FINISH_IRQ_EN,
            DmaChannel::Ch5 => Self::CH5_ALL_FINISH_IRQ_EN,
            DmaChannel::Ch6 => Self::CH6_ALL_FINISH_IRQ_EN,
            DmaChannel::Ch7 => Self::CH7_ALL_FINISH_IRQ_EN,
        };
        Self(self.0 | mask)
    }
    /// Disable channel all finish interrupt.
    #[inline]
    pub const fn disable_channel_all_finish(self, channel: DmaChannel) -> Self {
        let mask = match channel {
            DmaChannel::Ch0 => Self::CH0_ALL_FINISH_IRQ_EN,
            DmaChannel::Ch1 => Self::CH1_ALL_FINISH_IRQ_EN,
            DmaChannel::Ch2 => Self::CH2_ALL_FINISH_IRQ_EN,
            DmaChannel::Ch3 => Self::CH3_ALL_FINISH_IRQ_EN,
            DmaChannel::Ch4 => Self::CH4_ALL_FINISH_IRQ_EN,
            DmaChannel::Ch5 => Self::CH5_ALL_FINISH_IRQ_EN,
            DmaChannel::Ch6 => Self::CH6_ALL_FINISH_IRQ_EN,
            DmaChannel::Ch7 => Self::CH7_ALL_FINISH_IRQ_EN,
        };
        Self(self.0 & !mask)
    }
    /// Check if channel all finish interrupt is enabled.
    #[inline]
    pub const fn is_all_finish_enabled(self, channel: DmaChannel) -> bool {
        let mask = match channel {
            DmaChannel::Ch0 => Self::CH0_ALL_FINISH_IRQ_EN,
            DmaChannel::Ch1 => Self::CH1_ALL_FINISH_IRQ_EN,
            DmaChannel::Ch2 => Self::CH2_ALL_FINISH_IRQ_EN,
            DmaChannel::Ch3 => Self::CH3_ALL_FINISH_IRQ_EN,
            DmaChannel::Ch4 => Self::CH4_ALL_FINISH_IRQ_EN,
            DmaChannel::Ch5 => Self::CH5_ALL_FINISH_IRQ_EN,
            DmaChannel::Ch6 => Self::CH6_ALL_FINISH_IRQ_EN,
            DmaChannel::Ch7 => Self::CH7_ALL_FINISH_IRQ_EN,
        };
        (self.0 & mask) != 0
    }
    /// Enable channel task finish interrupt.
    #[inline]
    pub const fn enable_channel_task_finish(self, channel: DmaChannel) -> Self {
        let mask = match channel {
            DmaChannel::Ch0 => Self::CH0_TASK_FINISH_IRQ_EN,
            DmaChannel::Ch1 => Self::CH1_TASK_FINISH_IRQ_EN,
            DmaChannel::Ch2 => Self::CH2_TASK_FINISH_IRQ_EN,
            DmaChannel::Ch3 => Self::CH3_TASK_FINISH_IRQ_EN,
            DmaChannel::Ch4 => Self::CH4_TASK_FINISH_IRQ_EN,
            DmaChannel::Ch5 => Self::CH5_TASK_FINISH_IRQ_EN,
            DmaChannel::Ch6 => Self::CH6_TASK_FINISH_IRQ_EN,
            DmaChannel::Ch7 => Self::CH7_TASK_FINISH_IRQ_EN,
        };
        Self(self.0 | mask)
    }
    /// Disable channel task finish interrupt.
    #[inline]
    pub const fn disable_channel_task_finish(self, channel: DmaChannel) -> Self {
        let mask = match channel {
            DmaChannel::Ch0 => Self::CH0_TASK_FINISH_IRQ_EN,
            DmaChannel::Ch1 => Self::CH1_TASK_FINISH_IRQ_EN,
            DmaChannel::Ch2 => Self::CH2_TASK_FINISH_IRQ_EN,
            DmaChannel::Ch3 => Self::CH3_TASK_FINISH_IRQ_EN,
            DmaChannel::Ch4 => Self::CH4_TASK_FINISH_IRQ_EN,
            DmaChannel::Ch5 => Self::CH5_TASK_FINISH_IRQ_EN,
            DmaChannel::Ch6 => Self::CH6_TASK_FINISH_IRQ_EN,
            DmaChannel::Ch7 => Self::CH7_TASK_FINISH_IRQ_EN,
        };
        Self(self.0 & !mask)
    }
    /// Check if channel task finish interrupt is enabled.
    #[inline]
    pub const fn is_task_finish_enabled(self, channel: DmaChannel) -> bool {
        let mask = match channel {
            DmaChannel::Ch0 => Self::CH0_TASK_FINISH_IRQ_EN,
            DmaChannel::Ch1 => Self::CH1_TASK_FINISH_IRQ_EN,
            DmaChannel::Ch2 => Self::CH2_TASK_FINISH_IRQ_EN,
            DmaChannel::Ch3 => Self::CH3_TASK_FINISH_IRQ_EN,
            DmaChannel::Ch4 => Self::CH4_TASK_FINISH_IRQ_EN,
            DmaChannel::Ch5 => Self::CH5_TASK_FINISH_IRQ_EN,
            DmaChannel::Ch6 => Self::CH6_TASK_FINISH_IRQ_EN,
            DmaChannel::Ch7 => Self::CH7_TASK_FINISH_IRQ_EN,
        };
        (self.0 & mask) != 0
    }
    /// Enable channel task half interrupt.
    #[inline]
    pub const fn enable_channel_task_half(self, channel: DmaChannel) -> Self {
        let mask = match channel {
            DmaChannel::Ch0 => Self::CH0_TASK_HALF_IRQ_EN,
            DmaChannel::Ch1 => Self::CH1_TASK_HALF_IRQ_EN,
            DmaChannel::Ch2 => Self::CH2_TASK_HALF_IRQ_EN,
            DmaChannel::Ch3 => Self::CH3_TASK_HALF_IRQ_EN,
            DmaChannel::Ch4 => Self::CH4_TASK_HALF_IRQ_EN,
            DmaChannel::Ch5 => Self::CH5_TASK_HALF_IRQ_EN,
            DmaChannel::Ch6 => Self::CH6_TASK_HALF_IRQ_EN,
            DmaChannel::Ch7 => Self::CH7_TASK_HALF_IRQ_EN,
        };
        Self(self.0 | mask)
    }
    /// Disable channel task half interrupt.
    #[inline]
    pub const fn disable_channel_task_half(self, channel: DmaChannel) -> Self {
        let mask = match channel {
            DmaChannel::Ch0 => Self::CH0_TASK_HALF_IRQ_EN,
            DmaChannel::Ch1 => Self::CH1_TASK_HALF_IRQ_EN,
            DmaChannel::Ch2 => Self::CH2_TASK_HALF_IRQ_EN,
            DmaChannel::Ch3 => Self::CH3_TASK_HALF_IRQ_EN,
            DmaChannel::Ch4 => Self::CH4_TASK_HALF_IRQ_EN,
            DmaChannel::Ch5 => Self::CH5_TASK_HALF_IRQ_EN,
            DmaChannel::Ch6 => Self::CH6_TASK_HALF_IRQ_EN,
            DmaChannel::Ch7 => Self::CH7_TASK_HALF_IRQ_EN,
        };
        Self(self.0 & !mask)
    }
    /// Check if channel task half interrupt is enabled.
    #[inline]
    pub const fn is_task_half_enabled(self, channel: DmaChannel) -> bool {
        let mask = match channel {
            DmaChannel::Ch0 => Self::CH0_TASK_HALF_IRQ_EN,
            DmaChannel::Ch1 => Self::CH1_TASK_HALF_IRQ_EN,
            DmaChannel::Ch2 => Self::CH2_TASK_HALF_IRQ_EN,
            DmaChannel::Ch3 => Self::CH3_TASK_HALF_IRQ_EN,
            DmaChannel::Ch4 => Self::CH4_TASK_HALF_IRQ_EN,
            DmaChannel::Ch5 => Self::CH5_TASK_HALF_IRQ_EN,
            DmaChannel::Ch6 => Self::CH6_TASK_HALF_IRQ_EN,
            DmaChannel::Ch7 => Self::CH7_TASK_HALF_IRQ_EN,
        };
        (self.0 & mask) != 0
    }
}

/// DMA interrupt request status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IntStatus(u32);

impl IntStatus {
    const CH7_ADDR_REQ_ERR_STA: u32 = 0x1 << 31;
    const CH7_ALL_FINISH_IRQ_STA: u32 = 0x1 << 30;
    const CH7_TASK_FINISH_IRQ_STA: u32 = 0x1 << 29;
    const CH7_TASK_HALF_IRQ_STA: u32 = 0x1 << 28;
    const CH6_ADDR_REQ_ERR_STA: u32 = 0x1 << 27;
    const CH6_ALL_FINISH_IRQ_STA: u32 = 0x1 << 26;
    const CH6_TASK_FINISH_IRQ_STA: u32 = 0x1 << 25;
    const CH6_TASK_HALF_IRQ_STA: u32 = 0x1 << 24;
    const CH5_ADDR_REQ_ERR_STA: u32 = 0x1 << 23;
    const CH5_ALL_FINISH_IRQ_STA: u32 = 0x1 << 22;
    const CH5_TASK_FINISH_IRQ_STA: u32 = 0x1 << 21;
    const CH5_TASK_HALF_IRQ_STA: u32 = 0x1 << 20;
    const CH4_ADDR_REQ_ERR_STA: u32 = 0x1 << 19;
    const CH4_ALL_FINISH_IRQ_STA: u32 = 0x1 << 18;
    const CH4_TASK_FINISH_IRQ_STA: u32 = 0x1 << 17;
    const CH4_TASK_HALF_IRQ_STA: u32 = 0x1 << 16;
    const CH3_ADDR_REQ_ERR_STA: u32 = 0x1 << 15;
    const CH3_ALL_FINISH_IRQ_STA: u32 = 0x1 << 14;
    const CH3_TASK_FINISH_IRQ_STA: u32 = 0x1 << 13;
    const CH3_TASK_HALF_IRQ_STA: u32 = 0x1 << 12;
    const CH2_ADDR_REQ_ERR_STA: u32 = 0x1 << 11;
    const CH2_ALL_FINISH_IRQ_STA: u32 = 0x1 << 10;
    const CH2_TASK_FINISH_IRQ_STA: u32 = 0x1 << 9;
    const CH2_TASK_HALF_IRQ_STA: u32 = 0x1 << 8;
    const CH1_ADDR_REQ_ERR_STA: u32 = 0x1 << 7;
    const CH1_ALL_FINISH_IRQ_STA: u32 = 0x1 << 6;
    const CH1_TASK_FINISH_IRQ_STA: u32 = 0x1 << 5;
    const CH1_TASK_HALF_IRQ_STA: u32 = 0x1 << 4;
    const CH0_ADDR_REQ_ERR_STA: u32 = 0x1 << 3;
    const CH0_ALL_FINISH_IRQ_STA: u32 = 0x1 << 2;
    const CH0_TASK_FINISH_IRQ_STA: u32 = 0x1 << 1;
    const CH0_TASK_HALF_IRQ_STA: u32 = 0x1;

    /// Check if channel address request error interrupt is pending.
    #[inline]
    pub const fn is_addr_req_err_pending(self, channel: DmaChannel) -> bool {
        let mask = match channel {
            DmaChannel::Ch0 => Self::CH0_ADDR_REQ_ERR_STA,
            DmaChannel::Ch1 => Self::CH1_ADDR_REQ_ERR_STA,
            DmaChannel::Ch2 => Self::CH2_ADDR_REQ_ERR_STA,
            DmaChannel::Ch3 => Self::CH3_ADDR_REQ_ERR_STA,
            DmaChannel::Ch4 => Self::CH4_ADDR_REQ_ERR_STA,
            DmaChannel::Ch5 => Self::CH5_ADDR_REQ_ERR_STA,
            DmaChannel::Ch6 => Self::CH6_ADDR_REQ_ERR_STA,
            DmaChannel::Ch7 => Self::CH7_ADDR_REQ_ERR_STA,
        };
        (self.0 & mask) != 0
    }
    /// Clear channel address request error interrupt pending flag.
    #[inline]
    pub const fn clear_addr_req_err_pending(self, channel: DmaChannel) -> Self {
        let mask = match channel {
            DmaChannel::Ch0 => Self::CH0_ADDR_REQ_ERR_STA,
            DmaChannel::Ch1 => Self::CH1_ADDR_REQ_ERR_STA,
            DmaChannel::Ch2 => Self::CH2_ADDR_REQ_ERR_STA,
            DmaChannel::Ch3 => Self::CH3_ADDR_REQ_ERR_STA,
            DmaChannel::Ch4 => Self::CH4_ADDR_REQ_ERR_STA,
            DmaChannel::Ch5 => Self::CH5_ADDR_REQ_ERR_STA,
            DmaChannel::Ch6 => Self::CH6_ADDR_REQ_ERR_STA,
            DmaChannel::Ch7 => Self::CH7_ADDR_REQ_ERR_STA,
        };
        Self(self.0 | mask)
    }
    /// Check if channel all finish interrupt is pending.
    #[inline]
    pub const fn is_all_finish_pending(self, channel: DmaChannel) -> bool {
        let mask = match channel {
            DmaChannel::Ch0 => Self::CH0_ALL_FINISH_IRQ_STA,
            DmaChannel::Ch1 => Self::CH1_ALL_FINISH_IRQ_STA,
            DmaChannel::Ch2 => Self::CH2_ALL_FINISH_IRQ_STA,
            DmaChannel::Ch3 => Self::CH3_ALL_FINISH_IRQ_STA,
            DmaChannel::Ch4 => Self::CH4_ALL_FINISH_IRQ_STA,
            DmaChannel::Ch5 => Self::CH5_ALL_FINISH_IRQ_STA,
            DmaChannel::Ch6 => Self::CH6_ALL_FINISH_IRQ_STA,
            DmaChannel::Ch7 => Self::CH7_ALL_FINISH_IRQ_STA,
        };
        (self.0 & mask) != 0
    }
    /// Clear channel all finish interrupt pending flag.
    #[inline]
    pub const fn clear_all_finish_pending(self, channel: DmaChannel) -> Self {
        let mask = match channel {
            DmaChannel::Ch0 => Self::CH0_ALL_FINISH_IRQ_STA,
            DmaChannel::Ch1 => Self::CH1_ALL_FINISH_IRQ_STA,
            DmaChannel::Ch2 => Self::CH2_ALL_FINISH_IRQ_STA,
            DmaChannel::Ch3 => Self::CH3_ALL_FINISH_IRQ_STA,
            DmaChannel::Ch4 => Self::CH4_ALL_FINISH_IRQ_STA,
            DmaChannel::Ch5 => Self::CH5_ALL_FINISH_IRQ_STA,
            DmaChannel::Ch6 => Self::CH6_ALL_FINISH_IRQ_STA,
            DmaChannel::Ch7 => Self::CH7_ALL_FINISH_IRQ_STA,
        };
        Self(self.0 | mask)
    }
    /// Check if channel task finish interrupt is pending.
    #[inline]
    pub const fn is_task_finish_pending(self, channel: DmaChannel) -> bool {
        let mask = match channel {
            DmaChannel::Ch0 => Self::CH0_TASK_FINISH_IRQ_STA,
            DmaChannel::Ch1 => Self::CH1_TASK_FINISH_IRQ_STA,
            DmaChannel::Ch2 => Self::CH2_TASK_FINISH_IRQ_STA,
            DmaChannel::Ch3 => Self::CH3_TASK_FINISH_IRQ_STA,
            DmaChannel::Ch4 => Self::CH4_TASK_FINISH_IRQ_STA,
            DmaChannel::Ch5 => Self::CH5_TASK_FINISH_IRQ_STA,
            DmaChannel::Ch6 => Self::CH6_TASK_FINISH_IRQ_STA,
            DmaChannel::Ch7 => Self::CH7_TASK_FINISH_IRQ_STA,
        };
        (self.0 & mask) != 0
    }
    /// Clear channel task finish interrupt pending flag.
    #[inline]
    pub const fn clear_task_finish_pending(self, channel: DmaChannel) -> Self {
        let mask = match channel {
            DmaChannel::Ch0 => Self::CH0_TASK_FINISH_IRQ_STA,
            DmaChannel::Ch1 => Self::CH1_TASK_FINISH_IRQ_STA,
            DmaChannel::Ch2 => Self::CH2_TASK_FINISH_IRQ_STA,
            DmaChannel::Ch3 => Self::CH3_TASK_FINISH_IRQ_STA,
            DmaChannel::Ch4 => Self::CH4_TASK_FINISH_IRQ_STA,
            DmaChannel::Ch5 => Self::CH5_TASK_FINISH_IRQ_STA,
            DmaChannel::Ch6 => Self::CH6_TASK_FINISH_IRQ_STA,
            DmaChannel::Ch7 => Self::CH7_TASK_FINISH_IRQ_STA,
        };
        Self(self.0 | mask)
    }
    /// Check if channel task half interrupt is pending.
    #[inline]
    pub const fn is_task_half_pending(self, channel: DmaChannel) -> bool {
        let mask = match channel {
            DmaChannel::Ch0 => Self::CH0_TASK_HALF_IRQ_STA,
            DmaChannel::Ch1 => Self::CH1_TASK_HALF_IRQ_STA,
            DmaChannel::Ch2 => Self::CH2_TASK_HALF_IRQ_STA,
            DmaChannel::Ch3 => Self::CH3_TASK_HALF_IRQ_STA,
            DmaChannel::Ch4 => Self::CH4_TASK_HALF_IRQ_STA,
            DmaChannel::Ch5 => Self::CH5_TASK_HALF_IRQ_STA,
            DmaChannel::Ch6 => Self::CH6_TASK_HALF_IRQ_STA,
            DmaChannel::Ch7 => Self::CH7_TASK_HALF_IRQ_STA,
        };
        (self.0 & mask) != 0
    }
    /// Clear channel task half interrupt pending flag.
    #[inline]
    pub const fn clear_task_half_pending(self, channel: DmaChannel) -> Self {
        let mask = match channel {
            DmaChannel::Ch0 => Self::CH0_TASK_HALF_IRQ_STA,
            DmaChannel::Ch1 => Self::CH1_TASK_HALF_IRQ_STA,
            DmaChannel::Ch2 => Self::CH2_TASK_HALF_IRQ_STA,
            DmaChannel::Ch3 => Self::CH3_TASK_HALF_IRQ_STA,
            DmaChannel::Ch4 => Self::CH4_TASK_HALF_IRQ_STA,
            DmaChannel::Ch5 => Self::CH5_TASK_HALF_IRQ_STA,
            DmaChannel::Ch6 => Self::CH6_TASK_HALF_IRQ_STA,
            DmaChannel::Ch7 => Self::CH7_TASK_HALF_IRQ_STA,
        };
        Self(self.0 | mask)
    }
}

/// DMA memory burst length.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MemBurst {
    Burst8,
    Burst16,
}

/// DMA memory configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MemCfg(u32);

impl MemCfg {
    const MEM_BURST_SET: u32 = 0x1 << 31;

    /// Set memory burst length (`MEM_BURST_SET`).
    #[doc(alias = "MEM_BURST_SET")]
    #[inline]
    pub const fn set_burst_length(self, burst: MemBurst) -> Self {
        Self((self.0 & !Self::MEM_BURST_SET) | ((burst as u32) << 31))
    }
    /// Get memory burst length.
    #[inline]
    pub const fn burst_length(self) -> MemBurst {
        match (self.0 & Self::MEM_BURST_SET) >> 31 {
            0 => MemBurst::Burst8,
            1 => MemBurst::Burst16,
            _ => unreachable!(),
        }
    }
}

/// DMA gate register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Gate(u32);

impl Gate {
    const DMA_GATE_BYPASS: u32 = 0x1;

    /// Enable channel (`DMA_GATE_BYPASS`).
    #[doc(alias = "DMA_GATE_BYPASS")]
    #[inline]
    pub const fn enable_ch(self, channel: DmaChannel) -> Self {
        Self(self.0 | (Self::DMA_GATE_BYPASS << channel as u32))
    }
    /// Disable channel.
    #[inline]
    pub const fn disable_ch(self, channel: DmaChannel) -> Self {
        Self(self.0 & !(Self::DMA_GATE_BYPASS << channel as u32))
    }
    /// Check if channel is enabled.
    #[inline]
    pub const fn is_ch_enabled(self, channel: DmaChannel) -> bool {
        (self.0 & (Self::DMA_GATE_BYPASS << channel as u32)) != 0
    }
}

/// DMA channel status register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ChStatus(u32);

impl ChStatus {
    /// Check if channel is busy.
    #[inline]
    pub const fn is_busy(self, channel: DmaChannel) -> bool {
        (self.0 & (0x1 << (channel as u32))) != 0
    }
}

/// DMA channel enable register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ChEnable(u32);

impl ChEnable {
    const DMA_CH_ENDMA: u32 = 0x1;

    /// Enable channel (`DMA_CH_ENDMA`).
    ///
    /// - After the DMA transfer is completed, this bit will be automatically cleared to 0.
    /// - The current status of the DMA can be indicated by reading this bit.
    #[doc(alias = "DMA_CH_ENDMA")]
    #[inline]
    pub const fn enable_ch(self) -> Self {
        Self(self.0 | Self::DMA_CH_ENDMA)
    }
    /// Disable channel.
    #[inline]
    pub const fn disable_ch(self) -> Self {
        Self(self.0 & !Self::DMA_CH_ENDMA)
    }
    /// Check if channel is enabled.
    #[inline]
    pub const fn is_ch_enabled(self) -> bool {
        (self.0 & Self::DMA_CH_ENDMA) != 0
    }
}

/// DMA channel pause register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ChPause(u32);

impl ChPause {
    const MEM_BYTE_ENABLE: u32 = 0x1 << 5;
    const MEM_SET_ENABLE: u32 = 0x1 << 4;
    const DMA_CH_PAUSE: u32 = 0x1;

    /// Enable memory byte mode (`MEM_BYTE_ENABLE`).
    #[doc(alias = "MEM_BYTE_ENABLE")]
    #[inline]
    pub const fn enable_mem_byte(self) -> Self {
        Self(self.0 | Self::MEM_BYTE_ENABLE)
    }
    /// Disable memory byte mode.
    #[inline]
    pub const fn disable_mem_byte(self) -> Self {
        Self(self.0 & !Self::MEM_BYTE_ENABLE)
    }
    /// Check if memory byte mode is enabled.
    #[inline]
    pub const fn is_mem_byte_enabled(self) -> bool {
        (self.0 & Self::MEM_BYTE_ENABLE) != 0
    }
    /// Enable memory setting (`MEM_SET_ENABLE`).
    #[doc(alias = "MEM_SET_ENABLE")]
    #[inline]
    pub const fn enable_mem_set(self) -> Self {
        Self(self.0 | Self::MEM_SET_ENABLE)
    }
    /// Disable memory setting.
    #[inline]
    pub const fn disable_mem_set(self) -> Self {
        Self(self.0 & !Self::MEM_SET_ENABLE)
    }
    /// Check if memory setting is enabled.
    #[inline]
    pub const fn is_mem_set_enabled(self) -> bool {
        (self.0 & Self::MEM_SET_ENABLE) != 0
    }
    /// Pause channel (`DMA_CH_PAUSE`).
    #[doc(alias = "DMA_CH_PAUSE")]
    #[inline]
    pub const fn pause_ch(self) -> Self {
        Self(self.0 | Self::DMA_CH_PAUSE)
    }
    /// Resume channel.
    #[inline]
    pub const fn resume_ch(self) -> Self {
        Self(self.0 & !Self::DMA_CH_PAUSE)
    }
    /// Check if channel is paused.
    #[inline]
    pub const fn is_ch_paused(self) -> bool {
        (self.0 & Self::DMA_CH_PAUSE) != 0
    }
}

/// Dma channel data width.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DataWidth {
    Bits8,
    Bits16,
    Bits32,
    Bits64,
}

/// Dma channel burst size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BurstSize {
    Burst1,
    Burst4,
    Burst8,
    Burst16,
}

/// DMA device.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DmaDev {
    Sram,
    Dram,
    #[cfg(any(
        feature = "d12x",
        feature = "d13x",
        feature = "g73x",
        feature = "m6800"
    ))]
    Xip,
    #[cfg(any(
        feature = "d13x",
        feature = "d21x",
        feature = "g73x",
        feature = "m6800"
    ))]
    PsadcQ1,
    #[cfg(any(
        feature = "d13x",
        feature = "d21x",
        feature = "g73x",
        feature = "m6800"
    ))]
    PsadcQ2,
    #[cfg(any(
        feature = "d13x",
        feature = "d21x",
        feature = "g73x",
        feature = "m6800"
    ))]
    Spi2,
    #[cfg(any(
        feature = "d13x",
        feature = "d21x",
        feature = "g73x",
        feature = "m6800"
    ))]
    Spi3,
    Spi0,
    Spi1,
    #[cfg(any(
        feature = "d13x",
        feature = "d21x",
        feature = "g73x",
        feature = "m6800"
    ))]
    I2s0,
    #[cfg(any(
        feature = "d13x",
        feature = "d21x",
        feature = "g73x",
        feature = "m6800"
    ))]
    I2s1,
    #[cfg(any(
        feature = "d12x",
        feature = "d13x",
        feature = "g73x",
        feature = "m6800"
    ))]
    AudioDmic,
    #[cfg(feature = "d21x")]
    AudioAdc,
    Uart0,
    Uart1,
    Uart2,
    Uart3,
    #[cfg(any(
        feature = "d13x",
        feature = "d21x",
        feature = "g73x",
        feature = "m6800"
    ))]
    Uart4,
    #[cfg(any(
        feature = "d13x",
        feature = "d21x",
        feature = "g73x",
        feature = "m6800"
    ))]
    Uart5,
    #[cfg(any(
        feature = "d13x",
        feature = "d21x",
        feature = "g73x",
        feature = "m6800"
    ))]
    Uart6,
    #[cfg(any(
        feature = "d13x",
        feature = "d21x",
        feature = "g73x",
        feature = "m6800"
    ))]
    Uart7,
    #[cfg(any(
        feature = "d12x",
        feature = "d13x",
        feature = "g73x",
        feature = "m6800"
    ))]
    Xspi,
    #[cfg(feature = "m6800")]
    Sdfm0,
    #[cfg(feature = "m6800")]
    Sdfm1,
    #[cfg(feature = "m6800")]
    Sdfm2,
    #[cfg(feature = "m6800")]
    Sdfm3,
}

impl DmaDev {
    /// Get the DMA device ID value.
    #[inline]
    pub const fn id(self) -> u8 {
        match self {
            Self::Sram => 0,
            Self::Dram => {
                #[cfg(feature = "d21x")]
                {
                    1
                }
                #[cfg(not(feature = "d21x"))]
                {
                    0
                }
            }
            #[cfg(any(
                feature = "d12x",
                feature = "d13x",
                feature = "g73x",
                feature = "m6800"
            ))]
            Self::Xip => 0,
            #[cfg(any(
                feature = "d13x",
                feature = "d21x",
                feature = "g73x",
                feature = "m6800"
            ))]
            Self::PsadcQ1 => 4,
            #[cfg(any(
                feature = "d13x",
                feature = "d21x",
                feature = "g73x",
                feature = "m6800"
            ))]
            Self::PsadcQ2 => 5,
            #[cfg(any(
                feature = "d13x",
                feature = "d21x",
                feature = "g73x",
                feature = "m6800"
            ))]
            Self::Spi2 => 8,
            #[cfg(any(
                feature = "d13x",
                feature = "d21x",
                feature = "g73x",
                feature = "m6800"
            ))]
            Self::Spi3 => 9,
            Self::Spi0 => 10,
            Self::Spi1 => 11,
            #[cfg(any(
                feature = "d13x",
                feature = "d21x",
                feature = "g73x",
                feature = "m6800"
            ))]
            Self::I2s0 => 12,
            #[cfg(any(
                feature = "d13x",
                feature = "d21x",
                feature = "g73x",
                feature = "m6800"
            ))]
            Self::I2s1 => 13,
            #[cfg(any(
                feature = "d12x",
                feature = "d13x",
                feature = "g73x",
                feature = "m6800"
            ))]
            Self::AudioDmic => 14,
            #[cfg(feature = "d21x")]
            Self::AudioAdc => 15,
            Self::Uart0 => 16,
            Self::Uart1 => 17,
            Self::Uart2 => 18,
            Self::Uart3 => 19,
            #[cfg(any(
                feature = "d13x",
                feature = "d21x",
                feature = "g73x",
                feature = "m6800"
            ))]
            Self::Uart4 => 20,
            #[cfg(any(
                feature = "d13x",
                feature = "d21x",
                feature = "g73x",
                feature = "m6800"
            ))]
            Self::Uart5 => 21,
            #[cfg(any(
                feature = "d13x",
                feature = "d21x",
                feature = "g73x",
                feature = "m6800"
            ))]
            Self::Uart6 => 22,
            #[cfg(any(
                feature = "d13x",
                feature = "d21x",
                feature = "g73x",
                feature = "m6800"
            ))]
            Self::Uart7 => 23,
            #[cfg(any(
                feature = "d12x",
                feature = "d13x",
                feature = "g73x",
                feature = "m6800"
            ))]
            Self::Xspi => 24,
            #[cfg(feature = "m6800")]
            Self::Sdfm0 => 25,
            #[cfg(feature = "m6800")]
            Self::Sdfm1 => 26,
            #[cfg(feature = "m6800")]
            Self::Sdfm2 => 27,
            #[cfg(feature = "m6800")]
            Self::Sdfm3 => 28,
        }
    }
}

/// DMA channel configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ChConfig(u32);

impl ChConfig {
    const SNK_DATA_WIDTH: u32 = 0x3 << 25;
    const SNK_ADDR_MODE: u32 = 0x1 << 24;
    const SNK_BURST: u32 = 0x3 << 22;
    const SNK_DEV: u32 = 0x3F << 16;
    const SRC_DATA_WIDTH: u32 = 0x3 << 9;
    const SRC_ADDR_MODE: u32 = 0x1 << 8;
    const SRC_BURST: u32 = 0x3 << 6;
    const SRC_DEV: u32 = 0x3F;

    /// Set sink data width (`SNK_DATA_WIDTH`).
    #[doc(alias = "SNK_DATA_WIDTH")]
    #[inline]
    pub const fn set_snk_data_width(self, width: DataWidth) -> Self {
        Self((self.0 & !Self::SNK_DATA_WIDTH) | ((width as u32) << 25))
    }
    /// Get sink data width.
    #[inline]
    pub const fn snk_data_width(self) -> DataWidth {
        match (self.0 & Self::SNK_DATA_WIDTH) >> 25 {
            0 => DataWidth::Bits8,
            1 => DataWidth::Bits16,
            2 => DataWidth::Bits32,
            3 => DataWidth::Bits64,
            _ => unreachable!(),
        }
    }
    /// Enable sink address increment mode (`SNK_ADDR_MODE`).
    #[doc(alias = "SNK_ADDR_MODE")]
    #[inline]
    pub const fn enable_snk_addr_inc(self) -> Self {
        Self((self.0 & !Self::SNK_ADDR_MODE) | (1 << 24))
    }
    /// Disable sink address increment mode.
    #[inline]
    pub const fn disable_snk_addr_inc(self) -> Self {
        Self(self.0 & !Self::SNK_ADDR_MODE)
    }
    /// Check if sink address increment mode is enabled.
    #[inline]
    pub const fn is_snk_addr_inc_enabled(self) -> bool {
        (self.0 & Self::SNK_ADDR_MODE) != 0
    }
    /// Set sink burst size (`SNK_BURST`).
    #[doc(alias = "SNK_BURST")]
    #[inline]
    pub const fn set_snk_burst(self, burst: BurstSize) -> Self {
        Self((self.0 & !Self::SNK_BURST) | ((burst as u32) << 22))
    }
    /// Get sink burst size.
    #[inline]
    pub const fn snk_burst(self) -> BurstSize {
        match (self.0 & Self::SNK_BURST) >> 22 {
            0 => BurstSize::Burst1,
            1 => BurstSize::Burst4,
            2 => BurstSize::Burst8,
            3 => BurstSize::Burst16,
            _ => unreachable!(),
        }
    }
    /// Set sink device (`SNK_DEV`).
    #[doc(alias = "SNK_DEV")]
    #[inline]
    pub const fn set_snk_dev(self, dev: DmaDev) -> Self {
        Self((self.0 & !Self::SNK_DEV) | ((dev.id() as u32) << 16))
    }
    /// Get sink device id.
    #[inline]
    pub const fn snk_dev_id(self) -> u8 {
        ((self.0 & Self::SNK_DEV) >> 16) as u8
    }
    /// Set source data width (`SRC_DATA_WIDTH`).
    #[doc(alias = "SRC_DATA_WIDTH")]
    #[inline]
    pub const fn set_src_data_width(self, width: DataWidth) -> Self {
        Self((self.0 & !Self::SRC_DATA_WIDTH) | ((width as u32) << 9))
    }
    /// Get source data width.
    #[inline]
    pub const fn src_data_width(self) -> DataWidth {
        match (self.0 & Self::SRC_DATA_WIDTH) >> 9 {
            0 => DataWidth::Bits8,
            1 => DataWidth::Bits16,
            2 => DataWidth::Bits32,
            3 => DataWidth::Bits64,
            _ => unreachable!(),
        }
    }
    /// Enable source address increment mode (`SRC_ADDR_MODE`).
    #[inline]
    pub const fn enable_src_addr_inc(self) -> Self {
        Self((self.0 & !Self::SRC_ADDR_MODE) | (1 << 8))
    }
    /// Disable source address increment mode.
    #[inline]
    pub const fn disable_src_addr_inc(self) -> Self {
        Self(self.0 & !Self::SRC_ADDR_MODE)
    }
    /// Check if source address increment mode is enabled.
    #[inline]
    pub const fn is_src_addr_inc_enabled(self) -> bool {
        (self.0 & Self::SRC_ADDR_MODE) != 0
    }
    /// Set source burst size (`SRC_BURST`).
    #[doc(alias = "SRC_BURST")]
    #[inline]
    pub const fn set_src_burst(self, burst: BurstSize) -> Self {
        Self((self.0 & !Self::SRC_BURST) | ((burst as u32) << 6))
    }
    /// Get source burst size.
    #[inline]
    pub const fn src_burst(self) -> BurstSize {
        match (self.0 & Self::SRC_BURST) >> 6 {
            0 => BurstSize::Burst1,
            1 => BurstSize::Burst4,
            2 => BurstSize::Burst8,
            3 => BurstSize::Burst16,
            _ => unreachable!(),
        }
    }
    /// Set source device (`SRC_DEV`).
    #[doc(alias = "SRC_DEV")]
    #[inline]
    pub const fn set_src_dev(self, dev: DmaDev) -> Self {
        Self((self.0 & !Self::SRC_DEV) | (dev.id() as u32))
    }
    /// Get source device id.
    #[inline]
    pub const fn src_dev_id(self) -> u8 {
        (self.0 & Self::SRC_DEV) as u8
    }
}

/// Dma channel shake hand mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HandshakeMode {
    WaitForHandshake,
    Handshake,
}

/// DMA mode register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ChMode(u32);

impl ChMode {
    const SNK_HANDSHAKE_ENABLE: u32 = 0x1 << 6;
    const SRC_HANDSHAKE_ENABLE: u32 = 0x1 << 5;
    const DMA_SNK_MODE: u32 = 0x1 << 3;
    const DMA_SRC_MODE: u32 = 0x1 << 2;

    /// Enable sink handshake (`SNK_HANDSHAKE_ENABLE`).
    #[doc(alias = "SNK_HANDSHAKE_ENABLE")]
    #[inline]
    pub const fn enable_snk_handshake(self) -> Self {
        Self(self.0 | Self::SNK_HANDSHAKE_ENABLE)
    }
    /// Disable sink handshake.
    #[inline]
    pub const fn disable_snk_handshake(self) -> Self {
        Self(self.0 & !Self::SNK_HANDSHAKE_ENABLE)
    }
    /// Check if sink handshake is enabled.
    #[inline]
    pub const fn is_snk_handshake_enabled(self) -> bool {
        (self.0 & Self::SNK_HANDSHAKE_ENABLE) != 0
    }
    /// Enable source handshake (`SRC_HANDSHAKE_ENABLE`).
    #[doc(alias = "SRC_HANDSHAKE_ENABLE")]
    #[inline]
    pub const fn enable_src_handshake(self) -> Self {
        Self(self.0 | Self::SRC_HANDSHAKE_ENABLE)
    }
    /// Disable source handshake.
    #[inline]
    pub const fn disable_src_handshake(self) -> Self {
        Self(self.0 & !Self::SRC_HANDSHAKE_ENABLE)
    }
    /// Check if source handshake is enabled.
    #[inline]
    pub const fn is_src_handshake_enabled(self) -> bool {
        (self.0 & Self::SRC_HANDSHAKE_ENABLE) != 0
    }
    /// Set sink mode (`DMA_SNK_MODE`).
    #[doc(alias = "DMA_SNK_MODE")]
    #[inline]
    pub const fn set_snk_mode(self, mode: HandshakeMode) -> Self {
        Self((self.0 & !Self::DMA_SNK_MODE) | ((mode as u32) << 3))
    }
    /// Get sink mode.
    #[inline]
    pub const fn snk_mode(self) -> HandshakeMode {
        match (self.0 & Self::DMA_SNK_MODE) >> 3 {
            0 => HandshakeMode::WaitForHandshake,
            1 => HandshakeMode::Handshake,
            _ => unreachable!(),
        }
    }
    /// Set source mode (`DMA_SRC_MODE`).
    #[doc(alias = "DMA_SRC_MODE")]
    #[inline]
    pub const fn set_src_mode(self, mode: HandshakeMode) -> Self {
        Self((self.0 & !Self::DMA_SRC_MODE) | ((mode as u32) << 2))
    }
    /// Get source mode.
    #[inline]
    pub const fn src_mode(self) -> HandshakeMode {
        match (self.0 & Self::DMA_SRC_MODE) >> 2 {
            0 => HandshakeMode::WaitForHandshake,
            1 => HandshakeMode::Handshake,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BurstSize, ChConfig, ChEnable, ChMode, ChPause, ChStatus, DataWidth, DmaCh, DmaChannel,
        DmaDev, Gate, HandshakeMode, IntEnable, IntStatus, MemBurst, MemCfg, RegisterBlock,
    };
    use core::mem::{offset_of, size_of};

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, int_enable), 0x0);
        assert_eq!(offset_of!(RegisterBlock, int_status), 0x10);
        assert_eq!(offset_of!(RegisterBlock, mem_cfg), 0x20);
        assert_eq!(offset_of!(RegisterBlock, gate), 0x28);
        assert_eq!(offset_of!(RegisterBlock, ch_status), 0x30);
        assert_eq!(offset_of!(RegisterBlock, groups), 0x100);
    }

    #[test]
    fn struct_dmach_offset() {
        assert_eq!(offset_of!(DmaCh, enable), 0x0);
        assert_eq!(offset_of!(DmaCh, pause), 0x4);
        assert_eq!(offset_of!(DmaCh, task_addr), 0x8);
        assert_eq!(offset_of!(DmaCh, config), 0xC);
        assert_eq!(offset_of!(DmaCh, src_addr), 0x10);
        assert_eq!(offset_of!(DmaCh, sink_addr), 0x14);
        assert_eq!(offset_of!(DmaCh, bcnt_left), 0x18);
        assert_eq!(offset_of!(DmaCh, mode), 0x28);
        assert_eq!(offset_of!(DmaCh, fdes_addr), 0x2C);
        assert_eq!(offset_of!(DmaCh, pkg_num), 0x30);
        assert_eq!(offset_of!(DmaCh, mem_set), 0x34);
        assert_eq!(size_of::<DmaCh>(), 0x40);
    }

    #[test]
    fn struct_int_enable_functions() {
        for i in 0..8 {
            let (ch, err_en, all_finish_en, task_finish_en, task_half) = match i {
                0 => (
                    DmaChannel::Ch0,
                    0x0000_0008,
                    0x0000_0004,
                    0x0000_0002,
                    0x0000_0001,
                ),
                1 => (
                    DmaChannel::Ch1,
                    0x0000_0080,
                    0x0000_0040,
                    0x0000_0020,
                    0x0000_0010,
                ),
                2 => (
                    DmaChannel::Ch2,
                    0x0000_0800,
                    0x0000_0400,
                    0x0000_0200,
                    0x0000_0100,
                ),
                3 => (
                    DmaChannel::Ch3,
                    0x0000_8000,
                    0x0000_4000,
                    0x0000_2000,
                    0x0000_1000,
                ),
                4 => (
                    DmaChannel::Ch4,
                    0x0008_0000,
                    0x0004_0000,
                    0x0002_0000,
                    0x0001_0000,
                ),
                5 => (
                    DmaChannel::Ch5,
                    0x0080_0000,
                    0x0040_0000,
                    0x0020_0000,
                    0x0010_0000,
                ),
                6 => (
                    DmaChannel::Ch6,
                    0x0800_0000,
                    0x0400_0000,
                    0x0200_0000,
                    0x0100_0000,
                ),
                7 => (
                    DmaChannel::Ch7,
                    0x8000_0000,
                    0x4000_0000,
                    0x2000_0000,
                    0x1000_0000,
                ),
                _ => unreachable!(),
            };
            let mut val = IntEnable(0).enable_channel_addr_req_err(ch);
            assert!(val.is_addr_req_err_enabled(ch));
            assert_eq!(val.0, err_en);
            val = val.disable_channel_addr_req_err(ch);
            assert!(!val.is_addr_req_err_enabled(ch));
            assert_eq!(val.0, 0x0000_0000);

            val = val.enable_channel_all_finish(ch);
            assert!(val.is_all_finish_enabled(ch));
            assert_eq!(val.0, all_finish_en);
            val = val.disable_channel_all_finish(ch);
            assert!(!val.is_all_finish_enabled(ch));
            assert_eq!(val.0, 0x0000_0000);

            val = val.enable_channel_task_finish(ch);
            assert!(val.is_task_finish_enabled(ch));
            assert_eq!(val.0, task_finish_en);
            val = val.disable_channel_task_finish(ch);
            assert!(!val.is_task_finish_enabled(ch));
            assert_eq!(val.0, 0x0000_0000);

            val = val.enable_channel_task_half(ch);
            assert!(val.is_task_half_enabled(ch));
            assert_eq!(val.0, task_half);
            val = val.disable_channel_task_half(ch);
            assert!(!val.is_task_half_enabled(ch));
            assert_eq!(val.0, 0x0000_0000);
        }
    }

    #[test]
    fn struct_int_status_functions() {
        for i in 0..8 {
            let (ch, err_st, all_finish_st, task_finish_st, task_half_st) = match i {
                0 => (
                    DmaChannel::Ch0,
                    0x0000_0008,
                    0x0000_0004,
                    0x0000_0002,
                    0x0000_0001,
                ),
                1 => (
                    DmaChannel::Ch1,
                    0x0000_0080,
                    0x0000_0040,
                    0x0000_0020,
                    0x0000_0010,
                ),
                2 => (
                    DmaChannel::Ch2,
                    0x0000_0800,
                    0x0000_0400,
                    0x0000_0200,
                    0x0000_0100,
                ),
                3 => (
                    DmaChannel::Ch3,
                    0x0000_8000,
                    0x0000_4000,
                    0x0000_2000,
                    0x0000_1000,
                ),
                4 => (
                    DmaChannel::Ch4,
                    0x0008_0000,
                    0x0004_0000,
                    0x0002_0000,
                    0x0001_0000,
                ),
                5 => (
                    DmaChannel::Ch5,
                    0x0080_0000,
                    0x0040_0000,
                    0x0020_0000,
                    0x0010_0000,
                ),
                6 => (
                    DmaChannel::Ch6,
                    0x0800_0000,
                    0x0400_0000,
                    0x0200_0000,
                    0x0100_0000,
                ),
                7 => (
                    DmaChannel::Ch7,
                    0x8000_0000,
                    0x4000_0000,
                    0x2000_0000,
                    0x1000_0000,
                ),
                _ => unreachable!(),
            };
            let mut val = IntStatus(0).clear_addr_req_err_pending(ch);
            assert!(val.is_addr_req_err_pending(ch));
            assert_eq!(val.0, err_st);

            val = IntStatus(0).clear_all_finish_pending(ch);
            assert!(val.is_all_finish_pending(ch));
            assert_eq!(val.0, all_finish_st);

            val = IntStatus(0).clear_task_finish_pending(ch);
            assert!(val.is_task_finish_pending(ch));
            assert_eq!(val.0, task_finish_st);

            val = IntStatus(0).clear_task_half_pending(ch);
            assert!(val.is_task_half_pending(ch));
            assert_eq!(val.0, task_half_st);
        }
    }

    #[test]
    fn struct_mem_cfg_functions() {
        let mut cfg = MemCfg(0).set_burst_length(MemBurst::Burst16);
        assert_eq!(cfg.burst_length(), MemBurst::Burst16);
        assert_eq!(cfg.0, 0x8000_0000);

        cfg = cfg.set_burst_length(MemBurst::Burst8);
        assert_eq!(cfg.burst_length(), MemBurst::Burst8);
        assert_eq!(cfg.0, 0x0000_0000);
    }

    #[test]
    fn struct_gate_functions() {
        for i in 0..8 {
            let (ch, en_st) = match i {
                0 => (DmaChannel::Ch0, 0x0000_0001),
                1 => (DmaChannel::Ch1, 0x0000_0002),
                2 => (DmaChannel::Ch2, 0x0000_0004),
                3 => (DmaChannel::Ch3, 0x0000_0008),
                4 => (DmaChannel::Ch4, 0x0000_0010),
                5 => (DmaChannel::Ch5, 0x0000_0020),
                6 => (DmaChannel::Ch6, 0x0000_0040),
                7 => (DmaChannel::Ch7, 0x0000_0080),
                _ => unreachable!(),
            };
            let mut val = Gate(0).enable_ch(ch);
            assert!(val.is_ch_enabled(ch));
            assert_eq!(val.0, en_st);
            val = val.disable_ch(ch);
            assert!(!val.is_ch_enabled(ch));
            assert_eq!(val.0, 0x0000_0000);
        }
    }

    #[test]
    fn struct_ch_status_functions() {
        for i in 0..8 {
            let (ch, busy_st) = match i {
                0 => (DmaChannel::Ch0, 0x0000_0001),
                1 => (DmaChannel::Ch1, 0x0000_0002),
                2 => (DmaChannel::Ch2, 0x0000_0004),
                3 => (DmaChannel::Ch3, 0x0000_0008),
                4 => (DmaChannel::Ch4, 0x0000_0010),
                5 => (DmaChannel::Ch5, 0x0000_0020),
                6 => (DmaChannel::Ch6, 0x0000_0040),
                7 => (DmaChannel::Ch7, 0x0000_0080),
                _ => unreachable!(),
            };
            let val = ChStatus(busy_st);
            assert!(val.is_busy(ch));
        }
    }

    #[test]
    fn struct_ch_enable_functions() {
        let mut val = ChEnable(0).enable_ch();
        assert!(val.is_ch_enabled());
        assert_eq!(val.0, 0x0000_0001);
        val = val.disable_ch();
        assert!(!val.is_ch_enabled());
        assert_eq!(val.0, 0x0000_0000);
    }

    #[test]
    fn struct_ch_pause_functions() {
        let mut val = ChPause(0).enable_mem_byte();
        assert!(val.is_mem_byte_enabled());
        assert_eq!(val.0, 0x0000_0020);
        val = val.disable_mem_byte();
        assert!(!val.is_mem_byte_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_mem_set();
        assert!(val.is_mem_set_enabled());
        assert_eq!(val.0, 0x0000_0010);
        val = val.disable_mem_set();
        assert!(!val.is_mem_set_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.pause_ch();
        assert!(val.is_ch_paused());
        assert_eq!(val.0, 0x0000_0001);
    }

    #[test]
    fn struct_ch_config_functions() {
        for i in 0..4 {
            let (data_width, reg_val) = match i {
                0 => (DataWidth::Bits8, 0x0000_0000),
                1 => (DataWidth::Bits16, 0x0200_0000),
                2 => (DataWidth::Bits32, 0x0400_0000),
                3 => (DataWidth::Bits64, 0x0600_0000),
                _ => unreachable!(),
            };

            let val = ChConfig(0).set_snk_data_width(data_width);
            assert_eq!(val.snk_data_width(), data_width);
            assert_eq!(val.0, reg_val);
        }

        let mut val = ChConfig(0).enable_snk_addr_inc();
        assert!(val.is_snk_addr_inc_enabled());
        assert_eq!(val.0, 0x0100_0000);
        val = val.disable_snk_addr_inc();
        assert!(!val.is_snk_addr_inc_enabled());
        assert_eq!(val.0, 0x0000_0000);

        for i in 0..4 {
            let (burst_size, reg_val) = match i {
                0 => (BurstSize::Burst1, 0x0000_0000),
                1 => (BurstSize::Burst4, 0x0040_0000),
                2 => (BurstSize::Burst8, 0x0080_0000),
                3 => (BurstSize::Burst16, 0x00C0_0000),
                _ => unreachable!(),
            };
            let val = ChConfig(0).set_snk_burst(burst_size);
            assert_eq!(val.snk_burst(), burst_size);
            assert_eq!(val.0, reg_val);
        }

        let all_devices = [
            DmaDev::Sram,
            DmaDev::Dram,
            #[cfg(any(
                feature = "d12x",
                feature = "d13x",
                feature = "g73x",
                feature = "m6800"
            ))]
            DmaDev::Xip,
            #[cfg(any(
                feature = "d13x",
                feature = "d21x",
                feature = "g73x",
                feature = "m6800"
            ))]
            DmaDev::PsadcQ1,
            #[cfg(any(
                feature = "d13x",
                feature = "d21x",
                feature = "g73x",
                feature = "m6800"
            ))]
            DmaDev::PsadcQ2,
            #[cfg(any(
                feature = "d13x",
                feature = "d21x",
                feature = "g73x",
                feature = "m6800"
            ))]
            DmaDev::Spi2,
            #[cfg(any(
                feature = "d13x",
                feature = "d21x",
                feature = "g73x",
                feature = "m6800"
            ))]
            DmaDev::Spi3,
            DmaDev::Spi0,
            DmaDev::Spi1,
            #[cfg(any(
                feature = "d13x",
                feature = "d21x",
                feature = "g73x",
                feature = "m6800"
            ))]
            DmaDev::I2s0,
            #[cfg(any(
                feature = "d13x",
                feature = "d21x",
                feature = "g73x",
                feature = "m6800"
            ))]
            DmaDev::I2s1,
            #[cfg(any(
                feature = "d12x",
                feature = "d13x",
                feature = "g73x",
                feature = "m6800"
            ))]
            DmaDev::AudioDmic,
            #[cfg(feature = "d21x")]
            DmaDev::AudioAdc,
            DmaDev::Uart0,
            DmaDev::Uart1,
            DmaDev::Uart2,
            DmaDev::Uart3,
            #[cfg(any(
                feature = "d13x",
                feature = "d21x",
                feature = "g73x",
                feature = "m6800"
            ))]
            DmaDev::Uart4,
            #[cfg(any(
                feature = "d13x",
                feature = "d21x",
                feature = "g73x",
                feature = "m6800"
            ))]
            DmaDev::Uart5,
            #[cfg(any(
                feature = "d13x",
                feature = "d21x",
                feature = "g73x",
                feature = "m6800"
            ))]
            DmaDev::Uart6,
            #[cfg(any(
                feature = "d13x",
                feature = "d21x",
                feature = "g73x",
                feature = "m6800"
            ))]
            DmaDev::Uart7,
            #[cfg(any(
                feature = "d12x",
                feature = "d13x",
                feature = "g73x",
                feature = "m6800"
            ))]
            DmaDev::Xspi,
            #[cfg(feature = "m6800")]
            DmaDev::Sdfm0,
            #[cfg(feature = "m6800")]
            DmaDev::Sdfm1,
            #[cfg(feature = "m6800")]
            DmaDev::Sdfm2,
            #[cfg(feature = "m6800")]
            DmaDev::Sdfm3,
        ];

        for dev in all_devices.iter() {
            let val = ChConfig(0).set_snk_dev(*dev);
            assert_eq!(val.snk_dev_id(), dev.id());
            assert_eq!(val.0, (dev.id() as u32) << 16);
        }

        for i in 0..4 {
            let (data_width, reg_val) = match i {
                0 => (DataWidth::Bits8, 0x0000_0000),
                1 => (DataWidth::Bits16, 0x0000_0200),
                2 => (DataWidth::Bits32, 0x0000_0400),
                3 => (DataWidth::Bits64, 0x0000_0600),
                _ => unreachable!(),
            };

            let val = ChConfig(0).set_src_data_width(data_width);
            assert_eq!(val.src_data_width(), data_width);
            assert_eq!(val.0, reg_val);
        }

        let mut val = ChConfig(0).enable_src_addr_inc();
        assert!(val.is_src_addr_inc_enabled());
        assert_eq!(val.0, 0x0000_0100);
        val = val.disable_src_addr_inc();
        assert!(!val.is_src_addr_inc_enabled());
        assert_eq!(val.0, 0x0000_0000);

        for i in 0..4 {
            let (burst_size, reg_val) = match i {
                0 => (BurstSize::Burst1, 0x0000_0000),
                1 => (BurstSize::Burst4, 0x0000_0040),
                2 => (BurstSize::Burst8, 0x0000_0080),
                3 => (BurstSize::Burst16, 0x0000_00C0),
                _ => unreachable!(),
            };
            let val = ChConfig(0).set_src_burst(burst_size);
            assert_eq!(val.src_burst(), burst_size);
            assert_eq!(val.0, reg_val);
        }

        for dev in all_devices.iter() {
            let val = ChConfig(0).set_src_dev(*dev);
            assert_eq!(val.src_dev_id(), dev.id());
            assert_eq!(val.0, dev.id() as u32);
        }
    }

    #[test]
    fn struct_ch_mode_functions() {
        let mut val = ChMode(0).enable_snk_handshake();
        assert!(val.is_snk_handshake_enabled());
        assert_eq!(val.0, 0x0000_0040);
        val = val.disable_snk_handshake();
        assert!(!val.is_snk_handshake_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = ChMode(0).enable_src_handshake();
        assert!(val.is_src_handshake_enabled());
        assert_eq!(val.0, 0x0000_0020);
        val = val.disable_src_handshake();
        assert!(!val.is_src_handshake_enabled());
        assert_eq!(val.0, 0x0000_0000);

        for i in 0..2 {
            let (mode, reg_val) = match i {
                0 => (HandshakeMode::WaitForHandshake, 0x0000_0000),
                1 => (HandshakeMode::Handshake, 0x0000_0008),
                _ => unreachable!(),
            };
            let val = ChMode(0).set_snk_mode(mode);
            assert_eq!(val.snk_mode(), mode);
            assert_eq!(val.0, reg_val);
        }

        for i in 0..2 {
            let (mode, reg_val) = match i {
                0 => (HandshakeMode::WaitForHandshake, 0x0000_0000),
                1 => (HandshakeMode::Handshake, 0x0000_0004),
                _ => unreachable!(),
            };
            let val = ChMode(0).set_src_mode(mode);
            assert_eq!(val.src_mode(), mode);
            assert_eq!(val.0, reg_val);
        }
    }
}
