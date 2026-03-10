//! Blocking QSPI interface.

// Reference:
// - https://github.com/artinchip/luban-lite/blob/main/bsp/artinchip/hal/qspi/hal_qspi.c
// - https://aicdoc.artinchip.com/topics/ic/qspi/qspi-programming-guide-d13x.html

use super::config::*;
use super::instance::Qspi;
use super::pad::*;
use super::register::*;
use crate::cmu::Cmu;
use embedded_hal::spi::{MODE_1, MODE_3, Operation};

pub struct BlockingQspi<'a, const I: u8, SCK, MOSI, MISO, CS, WP, HOLD>
where
    SCK: QspiPad<I> + SerialClock<I>,
    MOSI: QspiPad<I> + MasterOutSlaveIn<I>,
    MISO: QspiPad<I> + MasterInSlaveOut<I>,
    CS: QspiPad<I> + ChipSelect<I>,
    WP: QspiPad<I> + WriteProtect<I>,
    HOLD: QspiPad<I> + Hold<I>,
{
    reg: &'a RegisterBlock,
    _config: QspiConfig,
    sck: SCK,
    mosi: Option<MOSI>,
    miso: Option<MISO>,
    cs: Option<CS>,
    wp: Option<WP>,
    hold: Option<HOLD>,
}

/// QSPI internal clock divider selection.
enum DividerSelect {
    /// CDR1 mode: sclk >> div.
    Cdr1(u8),
    /// CDR2 mode: sclk / (2 * (div + 1)).
    Cdr2(u8),
}

#[allow(clippy::too_many_arguments)]
impl<'a, const I: u8, SCK, MOSI, MISO, CS, WP, HOLD>
    BlockingQspi<'a, I, SCK, MOSI, MISO, CS, WP, HOLD>
where
    SCK: QspiPad<I> + SerialClock<I>,
    MOSI: QspiPad<I> + MasterOutSlaveIn<I>,
    MISO: QspiPad<I> + MasterInSlaveOut<I>,
    CS: QspiPad<I> + ChipSelect<I>,
    WP: QspiPad<I> + WriteProtect<I>,
    HOLD: QspiPad<I> + Hold<I>,
{
    const MAX_HZ: u32 = 133_000_000;
    const MIN_HZ: u32 = 3_000;
    const DEFAULT_TX_WATERMARK: u8 = 32;
    const DEFAULT_RX_WATERMARK: u8 = 32;
    const PLL_FRA0_FREQ: u32 = 768_000_000;
    const FIFO_DEPTH: u8 = 64;

    /// Create a new blocking QSPI interface.
    pub fn new(
        reg: &'a RegisterBlock,
        sck: SCK,
        mosi: Option<MOSI>,
        miso: Option<MISO>,
        cs: Option<CS>,
        wp: Option<WP>,
        hold: Option<HOLD>,
        config: QspiConfig,
        cmu: &Cmu,
    ) -> Self {
        if config.mode == MODE_1 || config.mode == MODE_3 {
            panic!("QSPI only supports SPI modes 0 and 2");
        }

        let clk = cmu.register_block();
        let qspi_clk = match I {
            0 => &clk.clock_qspi0,
            1 => &clk.clock_qspi1,
            2 => &clk.clock_qspi2,
            3 => &clk.clock_qspi3,
            _ => panic!("Invalid QSPI index"),
        };

        let target_freq = config.freq.0.clamp(Self::MIN_HZ, Self::MAX_HZ);
        let pll_fra0_freq = Self::PLL_FRA0_FREQ;

        // Calculate module clock divider from PLL_FRA0.
        // Formula: module_clk = PLL_FRA0 / (mod_div + 1).
        // Strategy: Use appropriate module clock for different frequency ranges.
        let mod_div = if target_freq < 1_000_000 {
            15 // 768MHz / 16 = 48MHz.
        } else if target_freq < 10_000_000 {
            7 // 768MHz / 8 = 96MHz.
        } else if target_freq < 50_000_000 {
            3 // 768MHz / 4 = 192MHz.
        } else {
            0 // 768MHz / 1 = 768MHz.
        };

        let actual_module_clk = pll_fra0_freq / (mod_div as u32 + 1);

        // Calculate internal QSPI divider to reach target frequency.
        let divider = Self::calculate_best_divider(actual_module_clk, target_freq);

        unsafe {
            // Configure and enable module clock.
            // Reference: https://aicdoc.artinchip.com/topics/ic/cmu/cmu-function2-d13x.html
            qspi_clk.modify(|v| {
                v.set_module_clk_div(mod_div)
                    .enable_module_clk()
                    .enable_bus_clk()
            });
            qspi_clk.modify(|v| v.enable_module_reset());
            riscv::asm::delay(500);
            qspi_clk.modify(|v| v.disable_module_reset());

            // Software reset QSPI controller.
            reg.config.modify(|v| v.set_ctrl_rst(true));
            riscv::asm::delay(500);

            // Configure internal clock divider BEFORE enabling controller.
            reg.clk_config.modify(|v| match divider {
                DividerSelect::Cdr1(div) => v
                    .set_clock_divider_selection(ClockDivSel::Div1)
                    .set_clk_div_1(div),
                DividerSelect::Cdr2(div) => v
                    .set_clock_divider_selection(ClockDivSel::Div2)
                    .set_clk_div_2(div),
            });

            // Small delay to ensure clock config is latched.
            riscv::asm::delay(100);

            // Clear reset and enable controller.
            reg.config.modify(|v| {
                v.set_ctrl_mode(config.ctrl_mode)
                    .set_ctrl_rst(false) // Clear reset flag.
                    .enable_ctrl()
                    .enable_rx_full_stop()
            });

            // Disable all interrupts.
            reg.int_control.modify(|v| v.disable_all_int());

            // Configure transfer settings.
            let cs_ctrl_mode = if cs.is_some() {
                CsCtrlMode::SpiController
            } else {
                CsCtrlMode::Software
            };

            reg.trans_config.modify(|v| {
                let v = v
                    .enable_discard_invalid_data()
                    .set_clk_pha(config.mode.phase)
                    .set_clk_pol(config.mode.polarity)
                    .set_cs_ctrl_mode(cs_ctrl_mode)
                    .enable_tx_data_delay();
                if config.lsb_first {
                    v.enable_lsb_transmit()
                } else {
                    v.disable_lsb_transmit()
                }
            });

            if let Some(cs_config) = config.cs_config {
                reg.trans_config.modify(|v| {
                    v.set_cs_pol(cs_config.polarity)
                        .set_cs_level(cs_config.level)
                        .set_cs_pin_num(cs_config.pin)
                });
            }

            // Configure RX sample delay based on frequency.
            if target_freq <= 24_000_000 {
                // No delay: bit13=1, bit11=0.
                reg.trans_config
                    .modify(|v| v.disable_rx_data_delay().disable_rx_inner_delay());
            } else if target_freq <= 60_000_000 {
                // Half clock delay: bit13=0, bit11=0.
                reg.trans_config
                    .modify(|v| v.enable_rx_data_delay().disable_rx_inner_delay());
            } else {
                // One clock delay: bit13=0, bit11=1.
                reg.trans_config
                    .modify(|v| v.enable_rx_data_delay().enable_rx_inner_delay());
            }

            // Configure work mode.
            match config.work_mode {
                WorkMode::Standard => {
                    if config.bit_mode_en {
                        // The transmission unit: bits.
                        reg.bit_mode_trans_config
                            .modify(|v| v.set_bus_mode(BusMode::StandardBits));
                    } else {
                        // The transmission unit: bytes.
                        reg.bit_mode_trans_config
                            .modify(|v| v.set_bus_mode(BusMode::StandardBytes));
                    }
                }
                WorkMode::ThreeWire => {
                    if config.bit_mode_en {
                        // The transmission unit: bits.
                        reg.bit_mode_trans_config
                            .modify(|v| v.set_bus_mode(BusMode::StandardBits));
                    } else {
                        // The transmission unit: bytes.
                        reg.bit_mode_trans_config
                            .modify(|v| v.set_bus_mode(BusMode::StandardBytes));
                        reg.trans_config.modify(|v| v.enable_three_wire());
                    }
                }
                WorkMode::Dual | WorkMode::DualIO => {
                    reg.trans_misc_control.modify(|v| v.enable_dual());
                }
                WorkMode::Quad | WorkMode::QuadIO => {
                    reg.trans_misc_control.modify(|v| v.enable_quad());
                }
                WorkMode::Qpi => {
                    reg.trans_misc_control.modify(|v| v.enable_qpi());
                }
            }

            // Initialize FIFO.
            reg.fifo_control.modify(|v| {
                v.reset_tx_fifo()
                    .reset_rx_fifo()
                    .set_tx_fifo_water_mark(Self::DEFAULT_TX_WATERMARK)
                    .set_rx_fifo_water_mark(Self::DEFAULT_RX_WATERMARK)
            });

            // Wait for FIFO reset to complete.
            riscv::asm::delay(100);

            // Clear all interrupt status.
            reg.int_status.modify(|v| v.clear_all_int());
        }

        Self {
            reg,
            _config: config,
            sck,
            mosi,
            miso,
            cs,
            wp,
            hold,
        }
    }

    /// Reset the TX and RX FIFOs.
    fn reset_fifos(&self) {
        unsafe {
            self.reg
                .fifo_control
                .modify(|v| v.reset_tx_fifo().reset_rx_fifo());
            riscv::asm::delay(10);
        }
    }

    /// Start a new transfer.
    fn start_transfer(&self, total: usize, tx: usize) {
        unsafe {
            self.reg
                .total_bytes_cnt
                .modify(|v| v.set_total_bytes(total as u32));
            self.reg.trans_write_cnt.modify(|v| v.set_tx_cnt(tx as u32));

            // Standard single-line mode: single_tx_count should equal tx_cnt.
            self.reg
                .trans_misc_control
                .modify(|v| v.set_single_tx_count(tx as u32));

            self.reg.trans_config.modify(|v| v.set_start(true));
        }
    }

    /// Wait for the transfer to complete.
    fn wait_transfer_done(&self) {
        while self.reg.trans_config.read().start() {
            core::hint::spin_loop();
        }
    }

    /// Write bytes to the TX FIFO.
    fn write_bytes(&self, buf: &[u8]) {
        let mut idx = 0usize;

        while idx < buf.len() {
            // Wait for FIFO to have enough space (at least one u32).
            while self.reg.fifo_status.read().tx_fifo_count() >= Self::FIFO_DEPTH - 4 {
                core::hint::spin_loop();
            }

            // Pack into u32 (little-endian).
            let chunk = core::cmp::min(4, buf.len() - idx);
            let mut word = 0u32;
            for i in 0..chunk {
                word |= (buf[idx + i] as u32) << (i * 8);
            }

            unsafe {
                self.reg.tx_data.write(word);
            }

            idx += chunk;
        }
    }

    /// Read bytes from the RX FIFO.
    fn read_bytes(&self, buf: &mut [u8]) {
        if buf.is_empty() {
            return;
        }

        let mut idx = 0usize;

        while idx < buf.len() {
            // Calculate the number of bytes to read this time (maximum 4 bytes).
            let chunk = core::cmp::min(4, buf.len() - idx);

            // Wait for at least chunk bytes in FIFO.
            while (self.reg.fifo_status.read().rx_fifo_count() as usize) < chunk {
                core::hint::spin_loop();
            }

            let word = self.reg.rx_data.read();

            for i in 0..chunk {
                buf[idx + i] = ((word >> (i * 8)) & 0xFF) as u8;
            }

            idx += chunk;
        }
    }

    /// Calculate the best internal clock divider to achieve target frequency.
    /// - CDR1: SPI_CLK = sclk / (2^cdr1).
    /// - CDR2: SPI_CLK = sclk / (2 * (cdr2 + 1)).
    fn calculate_best_divider(sclk: u32, bus_hz: u32) -> DividerSelect {
        // Calculate best CDR1 divider.
        // Formula: SPI_CLK = sclk / (2^cdr1).
        let mut cdr1 = 0u8;
        while (sclk >> cdr1) > bus_hz && cdr1 < 0xF {
            cdr1 += 1;
        }
        cdr1 = cdr1.min(0xF);

        // Refine cdr1: check if (cdr1-1) produces closer frequency.
        if cdr1 > 0 && (sclk >> cdr1) < bus_hz {
            let cdr1_clk = sclk >> cdr1;
            let cdr1_clkt = sclk >> (cdr1 - 1);
            if (cdr1_clkt - bus_hz) < (bus_hz - cdr1_clk) {
                cdr1 -= 1;
            }
        }

        // Calculate best CDR2 divider.
        // Formula: SPI_CLK = sclk / (2 * (cdr2 + 1)).
        let mut cdr2 = if bus_hz > 0 {
            let divisor = 2u32.saturating_mul(bus_hz);
            sclk.checked_div(divisor)
                .unwrap_or(0xFF)
                .saturating_sub(1)
                .min(0xFF) as u8
        } else {
            0xFF
        };

        // Refine cdr2: check if (cdr2+1) produces closer frequency.
        let cdr2_clk = sclk / (2 * cdr2 as u32 + 1);
        if cdr2 < 0xFF && cdr2_clk > bus_hz {
            let cdr2_clkt = sclk / (2 * (cdr2 as u32 + 1) + 1);
            if (bus_hz - cdr2_clkt) < (cdr2_clk - bus_hz) {
                cdr2 += 1;
            }
        }

        // Recalculate final frequencies.
        let cdr1_clk = sclk >> cdr1;
        let cdr2_clk = sclk / (2 * cdr2 as u32 + 1);

        // Priority: exact match > closest below target > closest above target.
        if cdr1_clk == bus_hz {
            return DividerSelect::Cdr1(cdr1);
        } else if cdr2_clk == bus_hz {
            return DividerSelect::Cdr2(cdr2);
        }

        // Both frequencies are below target: use the higher one (closer to target).
        if cdr2_clk < bus_hz && cdr1_clk < bus_hz {
            if cdr2_clk > cdr1_clk {
                return DividerSelect::Cdr2(cdr2);
            }
            return DividerSelect::Cdr1(cdr1);
        }

        // Mixed or both above target: use the lower frequency (safer for stability).
        if cdr2_clk < cdr1_clk {
            DividerSelect::Cdr2(cdr2)
        } else {
            DividerSelect::Cdr1(cdr1)
        }
    }

    /// Free the blocking QSPI and return QSPI instance and all pads.
    #[allow(clippy::type_complexity)]
    pub fn free(
        self,
        cmu: &Cmu,
    ) -> (
        Qspi<I>,
        SCK,
        Option<MOSI>,
        Option<MISO>,
        Option<CS>,
        Option<WP>,
        Option<HOLD>,
    ) {
        let clk = cmu.register_block();
        let qspi_clk = match I {
            0 => &clk.clock_qspi0,
            1 => &clk.clock_qspi1,
            2 => &clk.clock_qspi2,
            _ => &clk.clock_qspi3,
        };

        // Disable clock and reset module.
        unsafe {
            qspi_clk.modify(|v| {
                v.disable_module_clk()
                    .disable_bus_clk()
                    .enable_module_reset()
            });
        }

        (
            Qspi::<I>::__new(self.reg as *const RegisterBlock),
            self.sck,
            self.mosi,
            self.miso,
            self.cs,
            self.wp,
            self.hold,
        )
    }
}

impl<'a, const I: u8, SCK, MOSI, MISO, CS, WP, HOLD> embedded_hal::spi::ErrorType
    for BlockingQspi<'a, I, SCK, MOSI, MISO, CS, WP, HOLD>
where
    SCK: QspiPad<I> + SerialClock<I>,
    MOSI: QspiPad<I> + MasterOutSlaveIn<I>,
    MISO: QspiPad<I> + MasterInSlaveOut<I>,
    CS: QspiPad<I> + ChipSelect<I>,
    WP: QspiPad<I> + WriteProtect<I>,
    HOLD: QspiPad<I> + Hold<I>,
{
    type Error = core::convert::Infallible;
}

impl<'a, const I: u8, SCK, MOSI, MISO, CS, WP, HOLD> embedded_hal::spi::SpiBus
    for BlockingQspi<'a, I, SCK, MOSI, MISO, CS, WP, HOLD>
where
    SCK: QspiPad<I> + SerialClock<I>,
    MOSI: QspiPad<I> + MasterOutSlaveIn<I>,
    MISO: QspiPad<I> + MasterInSlaveOut<I>,
    CS: QspiPad<I> + ChipSelect<I>,
    WP: QspiPad<I> + WriteProtect<I>,
    HOLD: QspiPad<I> + Hold<I>,
{
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        if words.is_empty() {
            return Ok(());
        }
        self.reset_fifos();
        self.start_transfer(words.len(), 0);
        self.read_bytes(words);
        self.wait_transfer_done();
        Ok(())
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        if words.is_empty() {
            return Ok(());
        }
        self.reset_fifos();
        self.start_transfer(words.len(), words.len());
        self.write_bytes(words);
        while self.reg.fifo_status.read().tx_fifo_count() != 0 {
            core::hint::spin_loop();
        }
        self.wait_transfer_done();
        Ok(())
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        if !write.is_empty() {
            self.write(write)?;
        }
        if !read.is_empty() {
            self.read(read)?;
        }
        Ok(())
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        let mut temp = [0u8; 256];
        for chunk in words.chunks_mut(256) {
            let len = chunk.len();
            temp[..len].copy_from_slice(chunk);
            self.transfer(chunk, &temp[..len])?;
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.wait_transfer_done();
        Ok(())
    }
}

impl<'a, const I: u8, SCK, MOSI, MISO, CS, WP, HOLD> embedded_hal::spi::SpiDevice
    for BlockingQspi<'a, I, SCK, MOSI, MISO, CS, WP, HOLD>
where
    SCK: QspiPad<I> + SerialClock<I>,
    MOSI: QspiPad<I> + MasterOutSlaveIn<I>,
    MISO: QspiPad<I> + MasterInSlaveOut<I>,
    CS: QspiPad<I> + ChipSelect<I>,
    WP: QspiPad<I> + WriteProtect<I>,
    HOLD: QspiPad<I> + Hold<I>,
{
    fn transaction(&mut self, operations: &mut [Operation<'_, u8>]) -> Result<(), Self::Error> {
        for op in operations.iter_mut() {
            match op {
                Operation::Write(buf) => {
                    if buf.is_empty() {
                        continue;
                    }
                    self.reset_fifos();
                    self.start_transfer(buf.len(), buf.len());
                    self.write_bytes(buf);

                    // Wait for TX FIFO to empty.
                    while self.reg.fifo_status.read().tx_fifo_count() != 0 {
                        core::hint::spin_loop();
                    }
                    self.wait_transfer_done();
                }
                Operation::Read(buf) => {
                    if buf.is_empty() {
                        continue;
                    }

                    self.reset_fifos();

                    // Disable discard_invalid_data during Read.
                    unsafe {
                        self.reg
                            .trans_config
                            .modify(|v| v.disable_discard_invalid_data());
                    }

                    // Set transfer: txlen = rxlen (to generate clock).
                    self.start_transfer(buf.len(), buf.len());

                    // Send dummy bytes (0xFF) to generate SCK clock.
                    let mut idx = 0;
                    while idx < buf.len() {
                        // Wait for FIFO to have space.
                        while self.reg.fifo_status.read().tx_fifo_count()
                            >= Self::DEFAULT_TX_WATERMARK
                        {
                            core::hint::spin_loop();
                        }

                        let chunk = core::cmp::min(4, buf.len() - idx);
                        unsafe {
                            self.reg.tx_data.write(0xFFFF_FFFF);
                        }
                        idx += chunk;
                    }

                    // Wait for dummy data to finish sending.
                    while self.reg.fifo_status.read().tx_fifo_count() != 0 {
                        core::hint::spin_loop();
                    }

                    // Read data from RX FIFO.
                    self.read_bytes(buf);

                    self.wait_transfer_done();

                    // Restore settings.
                    unsafe {
                        self.reg
                            .trans_config
                            .modify(|v| v.enable_discard_invalid_data());
                    }
                }
                Operation::Transfer(read, write) => {
                    self.transfer(read, write)?;
                }
                Operation::TransferInPlace(buf) => {
                    self.transfer_in_place(buf)?;
                }
                Operation::DelayNs(ns) => {
                    riscv::asm::delay(*ns / 2);
                }
            }
        }
        Ok(())
    }
}
