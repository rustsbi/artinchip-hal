//! Efuse word access, on top of the SID block.

use crate::cmu::Cmu;

use super::{RegisterBlock, Sid};

/// Operation code for cell read command (`0xA1C`).
const OP_READ: u16 = 0xA1C;

/// Word offset to the redundant cell copy in the eFuse array.
const REDUNDANT_OFFSET: u32 = 64;

/// One efuse window over a brought-up SID block.
pub struct Efuse<'a> {
    sid: &'a RegisterBlock,
}

impl<'a> Efuse<'a> {
    /// Bring the SID block up (clock, reset, read timing) and return the window.
    pub fn new(cmu: &mut Cmu, sid: &'a mut Sid) -> Self {
        let clock = &cmu.register_block().clock_sid;
        unsafe {
            clock.modify(|val| val.enable_module_clk().enable_bus_clk());
        }

        let sid = sid.register_block();
        if sid.timing_low.read().rd_data_proc_time() == 0 {
            unsafe {
                sid.timing_low.modify(|val| {
                    val.set_rd_data_proc_time(4)
                        .set_rd_high_level_duration(2)
                        .set_wr_data_proc_time(0xFF)
                        .set_wr_high_level_duration(0xD8)
                });
            }
        }

        Self { sid }
    }

    /// Read `index` from the eFuse buffer.
    pub fn read(&self, index: usize) -> u32 {
        self.sid.buffer[index].read()
    }

    /// Read efuse cell word `index`, bypassing the eFuse buffer.
    pub fn read_cell(&mut self, index: u32) -> u32 {
        const POLL_LIMIT: u32 = 1_000_000;
        let mut value = 0u32;

        // Read the primary copy and its redundant copy 64 words higher, then OR them together.
        for half in 0..2 {
            let word = index + REDUNDANT_OFFSET * half;
            unsafe {
                self.sid
                    .addr
                    .modify(|val| val.set_efuse_addr((word << 2) as u16));
                self.sid
                    .ctrl
                    .modify(|val| val.set_efuse_op_code(OP_READ).set_efuse_read_start(true));
            }

            let mut polls = 0u32;
            while self.sid.ctrl.read().efuse_read_start() {
                polls += 1;
                if polls > POLL_LIMIT {
                    ::log::error!("Efuse: Cell read of word {word} never finished");
                    break;
                }
                core::hint::spin_loop();
            }

            value |= self.sid.rdata.read();
        }

        value
    }
}
