//! Blocking I2C interface.

use embedded_hal::i2c::{Operation, SevenBitAddress, TenBitAddress};

use super::config::{I2cConfig, Role};
use super::instance::I2c;
use super::pad::{I2cPad, SerialClock, SerialData};
use super::register::{AddressMode, RegisterBlock, SpeedMode, TransferMode};
use crate::cmu;

/// Blocking I2C interface.
pub struct BlockingI2c<'a, const I: u8, SCL, SDA>
where
    SCL: I2cPad<I> + SerialClock<I>,
    SDA: I2cPad<I> + SerialData<I>,
{
    reg: &'a RegisterBlock,
    scl: SCL,
    sda: SDA,
}

impl<'a, const I: u8, SCL, SDA> BlockingI2c<'a, I, SCL, SDA>
where
    SCL: I2cPad<I> + SerialClock<I>,
    SDA: I2cPad<I> + SerialData<I>,
{
    // I2C fixed clock is 24Mhz.
    const I2C_DEFAULT_CLOCK: u32 = 24_000_000;

    /// Create a new blocking serial.
    pub fn new(
        reg: &'a RegisterBlock,
        scl: SCL,
        sda: SDA,
        config: I2cConfig,
        clk: &cmu::RegisterBlock,
    ) -> Self {
        // Reference: https://aicdoc.artinchip.com/topics/ic/i2c/i2c-programming-guide-d13x.html
        let i2c_clk = match I {
            0 => &clk.clock_i2c0,
            1 => &clk.clock_i2c1,
            2 => &clk.clock_i2c2,
            3 => &clk.clock_i2c3,
            _ => panic!("Invalid I2C index"),
        };
        unsafe {
            // Initialize module clock.
            // Reference: https://aicdoc.artinchip.com/topics/ic/cmu/cmu-function2-d13x.html#topic_yvp_f24_4bc__table_qb3_bn5_ydc
            i2c_clk.modify(|v| v.enable_bus_clk());
            i2c_clk.modify(|v| v.enable_module_reset());
            riscv::asm::delay(500);
            i2c_clk.modify(|v| v.disable_module_reset());

            // Disable I2C module before configuration.
            reg.enable.modify(|v| v.disable_i2c());

            // Disable interrupts.
            reg.intr_mask.modify(|v| v.disable_all());

            // Configure SCL high and low counts.
            let (scl_hcnt, scl_lcnt) = Self::calc_scl_cnt(config.speed_mode);
            match config.speed_mode {
                SpeedMode::Standard => {
                    reg.ss_scl_hcnt.modify(|v| v.set_scl_high_count(scl_hcnt));
                    reg.ss_scl_lcnt.modify(|v| v.set_scl_low_count(scl_lcnt));
                }
                SpeedMode::Fast => {
                    reg.fs_scl_hcnt.modify(|v| v.set_scl_high_count(scl_hcnt));
                    reg.fs_scl_lcnt.modify(|v| v.set_scl_low_count(scl_lcnt));
                }
            }

            // Configure SDA hold time.
            reg.sda_hold
                .modify(|v| v.set_sda_tx_hold(10).set_sda_rx_hold(0));

            // Configure I2C role.
            match config.role {
                Role::Master => reg.ctrl.modify(|v| {
                    v.enable_master_mode()
                        .disable_slave_mode()
                        .set_speed_mode(config.speed_mode)
                        .enable_restart()
                }),
                Role::Slave => reg.ctrl.modify(|v| {
                    v.enable_slave_mode()
                        .disable_master_mode()
                        .set_speed_mode(config.speed_mode)
                        .set_stop_detect_if_addressed(true)
                }),
            }

            // Enable I2C.
            reg.enable.modify(|v| v.enable_i2c());
        }

        Self { reg, scl, sda }
    }

    /// Set address.
    pub fn set_address(&self, address: u16) {
        // Disable I2C.
        unsafe {
            self.reg.enable.modify(|v| v.disable_i2c());
        }

        let addr_mode = if address > 0x7F {
            AddressMode::Bit10
        } else {
            AddressMode::Bit7
        };
        unsafe {
            if self.reg.ctrl.read().is_master_mode_enabled() {
                self.reg
                    .ctrl
                    .modify(|v| v.set_address_mode_master(addr_mode));
                self.reg.target.modify(|v| v.set_target_address(address));
            } else {
                self.reg
                    .ctrl
                    .modify(|v| v.set_address_mode_slave(addr_mode));
                self.reg.slave_addr.modify(|v| v.set_slave_address(address));
            }
        }

        // Enable I2C.
        unsafe {
            self.reg.enable.modify(|v| v.enable_i2c());
        }
    }

    /// Calculate SCL high and low counts.
    ///
    /// Standard mode: tHIGH_min = 4000ns, tLOW_min = 4700ns.
    /// Fast mode: tHIGH_min = 600ns, tLOW_min = 1300ns.
    #[inline]
    fn calc_scl_cnt(speed: SpeedMode) -> (u16, u16) {
        // Reference: https://github.com/artinchip/luban-lite/blob/main/bsp/artinchip/hal/i2c/hal_i2c.c#L83
        const SS_MIN_SCL_HIGH_NS: u32 = 4000; // Standard mode tHIGH min.
        const SS_MIN_SCL_LOW_NS: u32 = 4700; // Standard mode tLOW min.
        const FS_MIN_SCL_HIGH_NS: u32 = 600; // Fast mode tHIGH min.
        const FS_MIN_SCL_LOW_NS: u32 = 1300; // Fast mode tLOW min.
        const MARGIN: u16 = 2; // Safety margin.

        let (high_ns, low_ns) = match speed {
            SpeedMode::Standard => (SS_MIN_SCL_HIGH_NS, SS_MIN_SCL_LOW_NS),
            SpeedMode::Fast => (FS_MIN_SCL_HIGH_NS, FS_MIN_SCL_LOW_NS),
        };

        // Calculate counts:  count = (time_ns * clk_freq_khz) / 1_000_000.
        // Divide clk_freq by 1000 first to prevent overflow.
        let clk_khz = Self::I2C_DEFAULT_CLOCK / 1000;
        let hcnt = ((high_ns * clk_khz) / 1_000_000) as u16 + MARGIN;
        let lcnt = ((low_ns * clk_khz) / 1_000_000) as u16 + MARGIN;

        (hcnt, lcnt)
    }

    /// Free the blocking I2C and return I2C instance, SCL and SDA pads.
    pub fn free(self, clk: &cmu::RegisterBlock) -> (I2c<I>, SCL, SDA) {
        unsafe {
            let i2c_clk = match I {
                0 => &clk.clock_i2c0,
                1 => &clk.clock_i2c1,
                2 => &clk.clock_i2c2,
                _ => &clk.clock_i2c3,
            };
            i2c_clk.modify(|v| v.disable_bus_clk().enable_module_reset());
        }
        (I2c::__new(self.reg), self.scl, self.sda)
    }
}

impl<'a, const I: u8, SCL, SDA> embedded_hal::i2c::ErrorType for BlockingI2c<'a, I, SCL, SDA>
where
    SCL: I2cPad<I> + SerialClock<I>,
    SDA: I2cPad<I> + SerialData<I>,
{
    type Error = core::convert::Infallible;
}

impl<'a, const I: u8, SCL, SDA> embedded_hal::i2c::I2c<SevenBitAddress>
    for BlockingI2c<'a, I, SCL, SDA>
where
    SCL: I2cPad<I> + SerialClock<I>,
    SDA: I2cPad<I> + SerialData<I>,
{
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.set_address(address as u16);

        let total_ops = operations.len();

        for (idx, operation) in operations.iter_mut().enumerate() {
            let is_last_op = idx == total_ops - 1;

            match operation {
                Operation::Write(bytes) => {
                    for (byte_idx, &byte) in bytes.iter().enumerate() {
                        while !self.reg.status.read().is_tx_fifo_not_full() {
                            core::hint::spin_loop();
                        }

                        let is_last_byte = byte_idx == bytes.len() - 1;
                        // Only send STOP on last byte of last operation
                        let send_stop = is_last_op && is_last_byte;

                        unsafe {
                            // I2C_DATA_CMD[8]=0 (write), [7:0]=data, [9]=STOP
                            self.reg.data_cmd.modify(|v| {
                                v.set_data_byte(byte)
                                    .set_transfer_mode(TransferMode::Write)
                                    .set_stop(send_stop)
                            });
                        }
                    }
                }
                Operation::Read(buffer) => {
                    let len = buffer.len();

                    // Issue read commands
                    for byte_idx in 0..len {
                        while !self.reg.status.read().is_tx_fifo_not_full() {
                            core::hint::spin_loop();
                        }

                        let is_last_byte = byte_idx == len - 1;
                        let send_stop = is_last_op && is_last_byte;

                        unsafe {
                            // I2C_DATA_CMD[8]=1 (read), [9]=STOP
                            self.reg.data_cmd.modify(|v| {
                                v.set_transfer_mode(TransferMode::Read).set_stop(send_stop)
                            });
                        }
                    }

                    // Read data from RX FIFO
                    for byte in buffer.iter_mut() {
                        while self.reg.rx_flr.read().rx_fifo_count() == 0 {
                            core::hint::spin_loop();
                        }
                        *byte = self.reg.data_cmd.read().data_byte();
                    }
                }
            }
        }

        Ok(())
    }
}

impl<'a, const I: u8, SCL, SDA> embedded_hal::i2c::I2c<TenBitAddress>
    for BlockingI2c<'a, I, SCL, SDA>
where
    SCL: I2cPad<I> + SerialClock<I>,
    SDA: I2cPad<I> + SerialData<I>,
{
    fn transaction(
        &mut self,
        address: u16,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.set_address(address);

        let total_ops = operations.len();

        for (idx, operation) in operations.iter_mut().enumerate() {
            let is_last_op = idx == total_ops - 1;

            match operation {
                Operation::Write(bytes) => {
                    for (byte_idx, &byte) in bytes.iter().enumerate() {
                        while !self.reg.status.read().is_tx_fifo_not_full() {
                            core::hint::spin_loop();
                        }

                        let is_last_byte = byte_idx == bytes.len() - 1;
                        // Only send STOP on last byte of last operation
                        let send_stop = is_last_op && is_last_byte;

                        unsafe {
                            // I2C_DATA_CMD[8]=0 (write), [7:0]=data, [9]=STOP
                            self.reg.data_cmd.modify(|v| {
                                v.set_data_byte(byte)
                                    .set_transfer_mode(TransferMode::Write)
                                    .set_stop(send_stop)
                            });
                        }
                    }
                }
                Operation::Read(buffer) => {
                    let len = buffer.len();

                    // Issue read commands
                    for byte_idx in 0..len {
                        while !self.reg.status.read().is_tx_fifo_not_full() {
                            core::hint::spin_loop();
                        }

                        let is_last_byte = byte_idx == len - 1;
                        let send_stop = is_last_op && is_last_byte;

                        unsafe {
                            // I2C_DATA_CMD[8]=1 (read), [9]=STOP
                            self.reg.data_cmd.modify(|v| {
                                v.set_transfer_mode(TransferMode::Read).set_stop(send_stop)
                            });
                        }
                    }

                    // Read data from RX FIFO
                    for byte in buffer.iter_mut() {
                        while self.reg.rx_flr.read().rx_fifo_count() == 0 {
                            core::hint::spin_loop();
                        }
                        *byte = self.reg.data_cmd.read().data_byte();
                    }
                }
            }
        }

        Ok(())
    }
}
