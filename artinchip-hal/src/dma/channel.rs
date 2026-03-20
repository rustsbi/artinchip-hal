//! DMA channel implementation.

use super::register::*;
use super::task::DmaTask;
use crate::cmu::Cmu;

/// DMA channels.
pub struct DmaChannels {
    pub ch0: DmaChannel<'static, 0>,
    pub ch1: DmaChannel<'static, 1>,
    pub ch2: DmaChannel<'static, 2>,
    pub ch3: DmaChannel<'static, 3>,
    pub ch4: DmaChannel<'static, 4>,
    pub ch5: DmaChannel<'static, 5>,
    pub ch6: DmaChannel<'static, 6>,
    pub ch7: DmaChannel<'static, 7>,
}

/// DMA channel with statically known channel number.
pub struct DmaChannel<'a, const I: u8> {
    reg: &'a RegisterBlock,
}

impl<'a, const I: u8> DmaChannel<'a, I> {
    /// Create a new DMA channel.
    pub fn __new(reg: &'a RegisterBlock, cmu: &mut Cmu) -> Self {
        let clk = &cmu.register_block().clock_dma;
        if !clk.read().is_bus_clk_enabled() {
            unsafe {
                // Initialize module clock.
                // Reference: https://aicdoc.artinchip.com/topics/ic/cmu/cmu-function2-d13x.html#topic_yvp_f24_4bc__table_qb3_bn5_ydc
                clk.modify(|v| v.enable_bus_clk());
                clk.modify(|v| v.enable_module_reset());
                riscv::asm::delay(500);
                clk.modify(|v| v.disable_module_reset());
            }
        }
        Self { reg }
    }

    /// Get a reference to the DMA channel's register.
    fn channel(&self) -> &DmaCh {
        &self.reg.groups[I as usize]
    }

    /// Check if channel is busy.
    pub fn is_busy(&self) -> bool {
        self.reg.ch_status.read().is_busy(I)
    }

    /// Start DMA transfer with the given task.
    pub fn start(&mut self, task: &DmaTask) {
        let task_addr = task as *const _ as u32;

        self.disable_all_irq();
        self.clear_all_pending();
        self.enable_all_irq();

        unsafe {
            // Write mode register.
            self.channel().mode.write(task.mode);

            // Write task address.
            self.channel().task_addr.write(task_addr);

            // Normal mode - clear pause.
            self.channel().pause.write(ChPause::zeroed());

            // Enable channel.
            self.channel().enable.modify(|v| v.enable_ch());
        }
    }

    /// Stop DMA transfer.
    pub fn stop(&mut self) {
        unsafe {
            // Disable all interrupts for this channel.
            self.disable_all_irq();

            // Pause channel.
            self.channel().pause.modify(|v| v.pause_ch());

            // Disable channel.
            self.channel().enable.modify(|v| v.disable_ch());

            // Resume (clear pause bit).
            self.channel().pause.modify(|v| v.resume_ch());
        }
    }

    /// Pause the DMA channel.
    pub fn pause(&mut self) {
        unsafe {
            self.channel().pause.modify(|v| v.pause_ch());
        }
    }

    /// Resume the DMA channel.
    pub fn resume(&mut self) {
        unsafe {
            self.channel().pause.modify(|v| v.resume_ch());
        }
    }

    /// Check if channel is paused.
    pub fn is_paused(&self) -> bool {
        self.channel().pause.read().is_ch_paused()
    }

    /// Check if channel is enabled.
    pub fn is_enabled(&self) -> bool {
        self.channel().enable.read().is_ch_enabled()
    }

    /// Enable address request error interrupt.
    pub fn enable_addr_req_error_irq(&mut self) {
        unsafe {
            self.reg
                .int_enable
                .modify(|v| v.enable_channel_addr_req_err(I));
        }
    }

    /// Disable address request error interrupt.
    pub fn disable_addr_req_error_irq(&mut self) {
        unsafe {
            self.reg
                .int_enable
                .modify(|v| v.disable_channel_addr_req_err(I));
        }
    }

    /// Enable all finish interrupt.
    pub fn enable_all_finish_irq(&mut self) {
        unsafe {
            self.reg
                .int_enable
                .modify(|v| v.enable_channel_all_finish(I));
        }
    }

    /// Disable all finish interrupt.
    pub fn disable_all_finish_irq(&mut self) {
        unsafe {
            self.reg
                .int_enable
                .modify(|v| v.disable_channel_all_finish(I));
        }
    }

    /// Enable task finish interrupt.
    pub fn enable_task_finish_irq(&mut self) {
        unsafe {
            self.reg
                .int_enable
                .modify(|v| v.enable_channel_task_finish(I));
        }
    }

    /// Disable task finish interrupt.
    pub fn disable_task_finish_irq(&mut self) {
        unsafe {
            self.reg
                .int_enable
                .modify(|v| v.disable_channel_task_finish(I));
        }
    }

    /// Enable task half interrupt.
    pub fn enable_task_half_irq(&mut self) {
        unsafe {
            self.reg
                .int_enable
                .modify(|v| v.enable_channel_task_half(I));
        }
    }

    /// Disable task half interrupt.
    pub fn disable_task_half_irq(&mut self) {
        unsafe {
            self.reg
                .int_enable
                .modify(|v| v.disable_channel_task_half(I));
        }
    }

    /// Enable all interrupts for this channel.
    pub fn enable_all_irq(&mut self) {
        unsafe {
            self.reg.int_enable.modify(|v| {
                v.enable_channel_addr_req_err(I)
                    .enable_channel_all_finish(I)
                    .enable_channel_task_finish(I)
                    .enable_channel_task_half(I)
            });
        }
    }

    /// Disable all interrupts for this channel.
    pub fn disable_all_irq(&mut self) {
        unsafe {
            self.reg.int_enable.modify(|v| {
                v.disable_channel_addr_req_err(I)
                    .disable_channel_all_finish(I)
                    .disable_channel_task_finish(I)
                    .disable_channel_task_half(I)
            });
        }
    }

    /// Check if address request error interrupt is pending.
    pub fn is_addr_req_error_pending(&self) -> bool {
        self.reg.int_status.read().is_addr_req_err_pending(I)
    }

    /// Clear address request error interrupt pending flag.
    pub fn clear_addr_req_error_pending(&mut self) {
        unsafe {
            self.reg
                .int_status
                .modify(|v| v.clear_addr_req_err_pending(I));
        }
    }

    /// Check if all finish interrupt is pending.
    pub fn is_all_finish_pending(&self) -> bool {
        self.reg.int_status.read().is_all_finish_pending(I)
    }

    /// Clear all finish interrupt pending flag.
    pub fn clear_all_finish_pending(&mut self) {
        unsafe {
            self.reg
                .int_status
                .modify(|v| v.clear_all_finish_pending(I));
        }
    }

    /// Check if task finish interrupt is pending.
    pub fn is_task_finish_pending(&self) -> bool {
        self.reg.int_status.read().is_task_finish_pending(I)
    }

    /// Clear task finish interrupt pending flag.
    pub fn clear_task_finish_pending(&mut self) {
        unsafe {
            self.reg
                .int_status
                .modify(|v| v.clear_task_finish_pending(I));
        }
    }

    /// Check if task half interrupt is pending.
    pub fn is_task_half_pending(&self) -> bool {
        self.reg.int_status.read().is_task_half_pending(I)
    }

    /// Clear task half interrupt pending flag.
    pub fn clear_task_half_pending(&mut self) {
        unsafe {
            self.reg.int_status.modify(|v| v.clear_task_half_pending(I));
        }
    }

    /// Clear all interrupt pending flags for this channel.
    pub fn clear_all_pending(&mut self) {
        unsafe {
            self.reg.int_status.modify(|v| {
                v.clear_addr_req_err_pending(I)
                    .clear_all_finish_pending(I)
                    .clear_task_finish_pending(I)
                    .clear_task_half_pending(I)
            });
        }
    }

    /// Get source address.
    pub fn src_addr(&self) -> u32 {
        self.channel().src_addr.read()
    }

    /// Get sink (destination) address.
    pub fn sink_addr(&self) -> u32 {
        self.channel().sink_addr.read()
    }

    /// Get byte count left.
    pub fn bcnt_left(&self) -> u32 {
        self.channel().bcnt_left.read()
    }

    /// Get former descriptor address.
    pub fn fdes_addr(&self) -> u32 {
        self.channel().fdes_addr.read()
    }

    /// Get package number.
    pub fn pkg_num(&self) -> u32 {
        self.channel().pkg_num.read()
    }

    /// Get task address.
    pub fn task_addr(&self) -> u32 {
        self.channel().task_addr.read()
    }

    /// Set memory set value (for memory fill operations).
    pub fn set_mem_set(&mut self, value: u32) {
        unsafe {
            self.channel().mem_set.write(value);
        }
    }

    /// Get memory set value.
    pub fn mem_set(&self) -> u32 {
        self.channel().mem_set.read()
    }
}
