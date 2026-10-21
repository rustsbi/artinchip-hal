//! Blocking QSPI interface.

// Reference:
// - https://github.com/artinchip/luban-lite/blob/main/bsp/artinchip/hal/qspi/hal_qspi.c
// - https://aicdoc.artinchip.com/topics/ic/qspi/qspi-programming-guide-d13x.html

use super::config::*;
use super::error::QspiError;
use super::instance::Qspi;
use super::pad::*;
use super::register::*;
use crate::cmu::{ClockTree, Cmu};
use embedded_hal::spi::{MODE_1, MODE_3, Operation};

pub struct BlockingQspi<'a, const I: u8, PAD>
where
    PAD: QspiPads<I>,
{
    reg: &'a RegisterBlock,
    _config: QspiConfig,
    pad: PAD,
}

/// QSPI internal clock divider selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DividerSelect {
    /// CDR1 mode: sclk >> div.
    Cdr1(u8),
    /// CDR2 mode: sclk / (2 * (div + 1)).
    Cdr2(u8),
}

impl<'a, const I: u8, PAD> BlockingQspi<'a, I, PAD>
where
    PAD: QspiPads<I>,
{
    const MAX_HZ: u32 = 100_000_000;
    const MIN_HZ: u32 = 100_000;
    const DEFAULT_TX_WATERMARK: u8 = 32;
    const DEFAULT_RX_WATERMARK: u8 = 32;
    const FIFO_DEPTH: u8 = 64;

    /// Create a new blocking QSPI interface.
    pub fn new(
        reg: &'a RegisterBlock,
        pad: PAD,
        config: QspiConfig,
        clock_tree: &ClockTree,
        cmu: &mut Cmu,
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
        let pll_fra0_freq = clock_tree.pll_fra0.0;

        // Search the module divider and internal divider together so the
        // selected SCK is as close as possible to the requested frequency.
        let (mod_div, divider) = Self::select_clock(pll_fra0_freq, target_freq);

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
            let cs_ctrl_mode = if config.cs_config.is_some() {
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
            pad,
        }
    }

    /// Reset the TX and RX FIFOs.
    fn reset_fifos(&mut self) {
        unsafe {
            self.reg
                .fifo_control
                .modify(|v| v.reset_tx_fifo().reset_rx_fifo());
            riscv::asm::delay(10);
        }
    }

    /// Start a new transfer.
    fn start_transfer(&mut self, total: usize, tx: usize) {
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
    fn wait_transfer_done(&mut self) -> Result<(), QspiError> {
        let mut timeout = 100_000;
        while self.reg.trans_config.read().start() {
            timeout -= 1;
            if timeout == 0 {
                return Err(QspiError::Timeout);
            }
            core::hint::spin_loop();
        }
        Ok(())
    }

    /// Write bytes to the TX FIFO.
    fn write_bytes(&mut self, buf: &[u8]) -> Result<(), QspiError> {
        let mut idx = 0usize;

        while idx < buf.len() {
            let mut timeout = 100_000;
            while self.reg.fifo_status.read().tx_fifo_count() >= Self::FIFO_DEPTH - 4 {
                timeout -= 1;
                if timeout == 0 {
                    return Err(QspiError::Timeout);
                }
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
        Ok(())
    }

    /// Read bytes from the RX FIFO.
    fn read_bytes(&self, buf: &mut [u8]) -> Result<(), QspiError> {
        if buf.is_empty() {
            return Ok(());
        }

        let mut idx = 0usize;

        while idx < buf.len() {
            // Calculate the number of bytes to read this time (maximum 4 bytes).
            let chunk = core::cmp::min(4, buf.len() - idx);

            let mut timeout = 100_000;
            while (self.reg.fifo_status.read().rx_fifo_count() as usize) < chunk {
                timeout -= 1;
                if timeout == 0 {
                    return Err(QspiError::Timeout);
                }
                core::hint::spin_loop();
            }

            let word = self.reg.rx_data.read();

            for i in 0..chunk {
                buf[idx + i] = ((word >> (i * 8)) & 0xFF) as u8;
            }

            idx += chunk;
        }
        Ok(())
    }

    /// Select the module and internal dividers with the smallest frequency
    /// error. Ties prefer a clock at or below the requested frequency.
    fn select_clock(pll_fra0: u32, target: u32) -> (u8, DividerSelect) {
        let mut best_error = u32::MAX;
        let mut best_over = true;
        let mut best_mod_div = 0;
        let mut best_divider = DividerSelect::Cdr1(0);

        // CDR1: SCK = module / 2^cdr1.
        // CDR2: SCK = module / (2 * (cdr2 + 1)).
        for mod_div in 0..=0x1F {
            let module = pll_fra0 / (mod_div as u32 + 1);
            if module == 0 || module > Self::MAX_HZ {
                continue;
            }

            for cdr1 in 0..=0xF {
                let actual = module >> cdr1;
                let error = actual.abs_diff(target);
                let over = actual > target;
                if error < best_error || (error == best_error && !over && best_over) {
                    best_error = error;
                    best_over = over;
                    best_mod_div = mod_div;
                    best_divider = DividerSelect::Cdr1(cdr1);
                }
            }

            for cdr2 in 0..=0xFF {
                let actual = module / (2 * (cdr2 as u32 + 1));
                let error = actual.abs_diff(target);
                let over = actual > target;
                if error < best_error || (error == best_error && !over && best_over) {
                    best_error = error;
                    best_over = over;
                    best_mod_div = mod_div;
                    best_divider = DividerSelect::Cdr2(cdr2);
                }
            }
        }

        (best_mod_div, best_divider)
    }

    /// Free the blocking QSPI and return QSPI instance and all pads.
    pub fn free(self, cmu: &Cmu) -> (Qspi<I>, PAD) {
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

        (Qspi::<I>::__new(self.reg as *const RegisterBlock), self.pad)
    }
}

impl<'a, const I: u8, PAD> embedded_hal::spi::ErrorType for BlockingQspi<'a, I, PAD>
where
    PAD: QspiPads<I>,
{
    type Error = QspiError;
}

impl<'a, const I: u8, PAD> embedded_hal::spi::SpiBus for BlockingQspi<'a, I, PAD>
where
    PAD: QspiPads<I>,
{
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        if words.is_empty() {
            return Ok(());
        }
        self.reset_fifos();
        self.start_transfer(words.len(), 0);
        self.read_bytes(words)?;
        self.wait_transfer_done()
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        if words.is_empty() {
            return Ok(());
        }
        self.reset_fifos();
        self.start_transfer(words.len(), words.len());
        self.write_bytes(words)?;
        let mut timeout = 100_000;
        while self.reg.fifo_status.read().tx_fifo_count() != 0 {
            timeout -= 1;
            if timeout == 0 {
                return Err(QspiError::Timeout);
            }
            core::hint::spin_loop();
        }
        self.wait_transfer_done()
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
        self.wait_transfer_done()
    }
}

impl<'a, const I: u8, PAD> embedded_hal::spi::SpiDevice for BlockingQspi<'a, I, PAD>
where
    PAD: QspiPads<I>,
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
                    self.write_bytes(buf)?;

                    let mut timeout = 100_000;
                    while self.reg.fifo_status.read().tx_fifo_count() != 0 {
                        timeout -= 1;
                        if timeout == 0 {
                            return Err(QspiError::Timeout);
                        }
                        core::hint::spin_loop();
                    }
                    self.wait_transfer_done()?;
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
                        let mut timeout = 100_000;
                        while self.reg.fifo_status.read().tx_fifo_count()
                            >= Self::DEFAULT_TX_WATERMARK
                        {
                            timeout -= 1;
                            if timeout == 0 {
                                return Err(QspiError::Timeout);
                            }
                            core::hint::spin_loop();
                        }

                        let chunk = core::cmp::min(4, buf.len() - idx);
                        unsafe {
                            self.reg.tx_data.write(0xFFFF_FFFF);
                        }
                        idx += chunk;
                    }

                    let mut timeout = 100_000;
                    while self.reg.fifo_status.read().tx_fifo_count() != 0 {
                        timeout -= 1;
                        if timeout == 0 {
                            return Err(QspiError::Timeout);
                        }
                        core::hint::spin_loop();
                    }

                    self.read_bytes(buf)?;

                    self.wait_transfer_done()?;

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
