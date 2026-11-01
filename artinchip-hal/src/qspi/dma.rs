//! QSPI bulk transfers through the SoC's general-purpose DMA controller.
//!
//! The QSPI controller's FIFO DMA request lines are wired to the GPDMA; the
//! controller itself still frames each half-duplex phase, while the DMA channel
//! moves the payload between memory and `SPI_TXD`/`SPI_RXD`.

use super::blocking::BlockingQspi;
use super::bus::QuadSpiBus;
use super::error::QspiError;
use super::pad::QspiPads;
use crate::cache;
use crate::dma::{
    BurstSize, ChConfig, ChMode, DataWidth, DmaChannel, DmaId, DmaTask, HandshakeMode,
};
use embedded_hal::spi::{ErrorType, SpiBus};

/// Smallest payload worth a DMA setup.
const MIN_DMA_LEN: usize = 64;
/// Memory alignment the DMA controller requires for source/destination.
const DMA_ALIGN: usize = 8;

const fn dma_request(qspi_index: u8) -> u8 {
    match qspi_index {
        0 => DmaId::Spi0.raw(),
        1 => DmaId::Spi1.raw(),
        2 => DmaId::Spi2.raw(),
        _ => DmaId::Spi3.raw(),
    }
}

/// Blocking QSPI with bulk phases moved by a GPDMA channel.
pub struct DmaQspi<'a, const I: u8, PAD, const CH: u8>
where
    PAD: QspiPads<I>,
{
    bus: BlockingQspi<'a, I, PAD>,
    dma: DmaChannel<'a, CH>,
    task: DmaTask,
}

impl<'a, const I: u8, PAD, const CH: u8> DmaQspi<'a, I, PAD, CH>
where
    PAD: QspiPads<I>,
{
    /// Wrap a blocking QSPI bus and the DMA channel used for bulk phases.
    pub fn new(bus: BlockingQspi<'a, I, PAD>, dma: DmaChannel<'a, CH>) -> Self {
        Self {
            bus,
            dma,
            task: DmaTask {
                cfg: ChConfig::zeroed(),
                src: 0,
                dst: 0,
                len: 0,
                delay: DmaTask::DEFAULT_DELAY,
                p_next: DmaTask::TASK_END,
                mode: ChMode::zeroed(),
                v_next: None,
            },
        }
    }

    /// Return the underlying bus and DMA channel.
    pub fn free(self) -> (BlockingQspi<'a, I, PAD>, DmaChannel<'a, CH>) {
        (self.bus, self.dma)
    }

    fn dma_capable(len: usize, ptr: usize) -> bool {
        len >= MIN_DMA_LEN && len.is_multiple_of(DMA_ALIGN) && ptr.is_multiple_of(DMA_ALIGN)
    }

    fn tx_config(&self) -> (ChConfig, ChMode) {
        let cfg = ChConfig::zeroed()
            .set_src_dev(DmaId::Sram.raw())
            .set_src_data_width(DataWidth::Bits32)
            .set_src_burst(BurstSize::Burst16)
            .enable_src_addr_inc()
            .set_snk_dev(dma_request(I))
            .set_snk_data_width(DataWidth::Bits32)
            .set_snk_burst(BurstSize::Burst8)
            .disable_snk_addr_inc();
        let mode = ChMode::zeroed()
            .enable_handshake()
            .enable_snk_handshake()
            .set_snk_mode(HandshakeMode::Handshake)
            .set_src_mode(HandshakeMode::Wait);
        (cfg, mode)
    }

    fn rx_config(&self) -> (ChConfig, ChMode) {
        let cfg = ChConfig::zeroed()
            .set_src_dev(dma_request(I))
            .set_src_data_width(DataWidth::Bits32)
            .set_src_burst(BurstSize::Burst8)
            .disable_src_addr_inc()
            .set_snk_dev(DmaId::Sram.raw())
            .set_snk_data_width(DataWidth::Bits32)
            .set_snk_burst(BurstSize::Burst16)
            .enable_snk_addr_inc();
        let mode = ChMode::zeroed()
            .enable_handshake()
            .enable_src_handshake()
            .set_src_mode(HandshakeMode::Handshake)
            .set_snk_mode(HandshakeMode::Wait);
        (cfg, mode)
    }

    fn write_dma(&mut self, data: &[u8], single_line: bool) -> Result<(), QspiError> {
        let (cfg, mode) = self.tx_config();
        let src = data.as_ptr() as usize;
        self.task = DmaTask {
            cfg,
            src: src as u32,
            dst: self.bus.tx_data_addr(),
            len: data.len() as u32,
            delay: DmaTask::DEFAULT_DELAY,
            p_next: DmaTask::TASK_END,
            mode,
            v_next: None,
        };
        unsafe {
            let task = core::ptr::addr_of!(self.task) as usize;
            cache::dcache_clean_invalidate_range(task, core::mem::size_of::<DmaTask>());
            cache::dcache_clean_invalidate_range(src, data.len());
        }

        self.bus.prepare_tx_dma(data.len(), single_line);
        self.dma.clear_all_pending();
        self.dma.start(&self.task);
        self.bus.start_transfer();
        let dma_result = self.wait_dma_done();
        let bus_result = self.bus.wait_transfer_done();
        self.bus.finish_dma();
        self.dma.stop();
        dma_result?;
        bus_result
    }

    fn read_dma(&mut self, data: &mut [u8]) -> Result<(), QspiError> {
        let (cfg, mode) = self.rx_config();
        let dst = data.as_ptr() as usize;
        self.task = DmaTask {
            cfg,
            src: self.bus.rx_data_addr(),
            dst: dst as u32,
            len: data.len() as u32,
            delay: DmaTask::DEFAULT_DELAY,
            p_next: DmaTask::TASK_END,
            mode,
            v_next: None,
        };
        unsafe {
            let task = core::ptr::addr_of!(self.task) as usize;
            cache::dcache_clean_invalidate_range(task, core::mem::size_of::<DmaTask>());
            cache::dcache_clean_invalidate_range(dst, data.len());
        }

        self.bus.prepare_rx_dma(data.len());
        self.dma.clear_all_pending();
        self.dma.start(&self.task);
        self.bus.start_transfer();
        let bus_result = self.bus.wait_transfer_done();
        let dma_result = self.wait_dma_done();
        self.bus.finish_dma();
        self.dma.stop();
        unsafe { cache::dcache_invalidate_range(dst, data.len()) };
        bus_result?;
        dma_result
    }

    fn wait_dma_done(&self) -> Result<(), QspiError> {
        let mut timeout = 200_000;
        while !self.dma.is_all_finish_pending() {
            timeout -= 1;
            if timeout == 0 {
                return Err(QspiError::Timeout);
            }
            core::hint::spin_loop();
        }
        Ok(())
    }
}

impl<const I: u8, PAD, const CH: u8> ErrorType for DmaQspi<'_, I, PAD, CH>
where
    PAD: QspiPads<I>,
{
    type Error = QspiError;
}

impl<const I: u8, PAD, const CH: u8> SpiBus<u8> for DmaQspi<'_, I, PAD, CH>
where
    PAD: QspiPads<I>,
{
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        if Self::dma_capable(words.len(), words.as_ptr() as usize) {
            self.read_dma(words)
        } else {
            self.bus.read(words)
        }
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        if Self::dma_capable(words.len(), words.as_ptr() as usize) {
            self.write_dma(words, true)
        } else {
            self.bus.write(words)
        }
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
        self.bus.transfer_in_place(words)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.bus.flush()
    }
}

impl<const I: u8, PAD, const CH: u8> QuadSpiBus for DmaQspi<'_, I, PAD, CH>
where
    PAD: QspiPads<I>,
{
    fn read_quad(
        &mut self,
        command: &[u8],
        tclqv_ns: u32,
        margin: u8,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        if Self::dma_capable(data.len(), data.as_ptr() as usize) {
            self.bus.quad_read_start(command, tclqv_ns, margin)?;
            let result = self.read_dma(data);
            self.bus.quad_read_end()?;
            result
        } else {
            self.bus.read_quad(command, tclqv_ns, margin, data)
        }
    }

    fn write_quad(&mut self, command: &[u8], data: &[u8]) -> Result<(), Self::Error> {
        if Self::dma_capable(data.len(), data.as_ptr() as usize) {
            self.bus.quad_write_start(command)?;
            let result = self.write_dma(data, false);
            self.bus.quad_write_end()?;
            result
        } else {
            self.bus.write_quad(command, data)
        }
    }
}
