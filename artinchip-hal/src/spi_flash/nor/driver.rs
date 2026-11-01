//! SPI NOR flash driver implementation.

use core::convert::Infallible;

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_storage::nor_flash::{ErrorType, NorFlash, ReadNorFlash};

use super::config::{
    NOR_PAGE_SIZE, NOR_READ_SIZE, NOR_SECTOR_SIZE, NorDeviceInfo, NorJedecId, SpiNorConfig,
};
use super::{opcode, status};
use crate::qspi::QuadSpiBus;
use crate::spi_flash::FlashError;

/// Maximum supported NOR page size for stack-allocated read-back verification.
const MAX_VERIFY_PAGE_SIZE: usize = 512;

#[repr(align(16))]
struct AlignedPage([u8; MAX_VERIFY_PAGE_SIZE]);

/// SPI NOR flash device driver.
pub struct SpiNorFlash<B, CS, D> {
    bus: B,
    cs: CS,
    delay: D,
    config: SpiNorConfig,
    device: NorDeviceInfo,
    id: NorJedecId,
}

impl<B, CS, D> SpiNorFlash<B, CS, D>
where
    B: QuadSpiBus,
    CS: OutputPin<Error = Infallible>,
    D: DelayNs,
{
    /// Initialize the SPI NOR flash in single-line mode.
    pub fn new(bus: B, cs: CS, delay: D) -> Result<Self, FlashError<B::Error>> {
        Self::with_config(bus, cs, delay, SpiNorConfig::single())
    }

    /// Initialize the SPI NOR flash with an explicit configuration.
    pub fn with_config(
        bus: B,
        mut cs: CS,
        delay: D,
        config: SpiNorConfig,
    ) -> Result<Self, FlashError<B::Error>> {
        cs.set_high().ok();
        let mut flash = Self {
            bus,
            cs,
            delay,
            config,
            device: config.device.unwrap_or(NorDeviceInfo::W25Q128JV),
            id: NorJedecId {
                manufacturer: 0,
                memory_type: 0,
                capacity: 0,
            },
        };
        let id = flash.jedec_id()?;
        flash.id = id;
        if config.device.is_none() {
            flash.device = NorDeviceInfo::from_jedec(id);
        }
        if flash.device.page_size > MAX_VERIFY_PAGE_SIZE {
            return Err(FlashError::NotSupported);
        }
        if config.quad.is_some() {
            flash.enable_quad()?;
        }
        Ok(flash)
    }

    /// Active driver configuration.
    pub const fn config(&self) -> SpiNorConfig {
        self.config
    }

    /// Resolved device geometry.
    pub const fn device(&self) -> NorDeviceInfo {
        self.device
    }

    /// JEDEC identification read during initialization.
    pub const fn id(&self) -> NorJedecId {
        self.id
    }

    /// Release the underlying bus, chip-select pin, and delay source.
    pub fn free(self) -> (B, CS, D) {
        (self.bus, self.cs, self.delay)
    }

    fn cs_low(&mut self) {
        self.cs.set_low().ok();
    }

    fn cs_high(&mut self) {
        self.cs.set_high().ok();
    }

    /// Frame a single-line write command with chip-select assertion.
    fn command(&mut self, bytes: &[u8]) -> Result<(), FlashError<B::Error>> {
        self.cs_low();
        let result = self.bus.write(bytes).map_err(FlashError::Spi);
        self.cs_high();
        result
    }

    /// Frame a single-line command followed by a single-line read phase.
    fn command_then_read(
        &mut self,
        command: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), FlashError<B::Error>> {
        self.cs_low();
        let mut result = self.bus.write(command).map_err(FlashError::Spi);
        if result.is_ok() {
            result = self.bus.read(buffer).map_err(FlashError::Spi);
        }
        self.cs_high();
        result
    }

    /// Read status register 1 (`05h`).
    pub fn read_status1(&mut self) -> Result<u8, FlashError<B::Error>> {
        let mut buffer = [0u8; 1];
        self.command_then_read(&[opcode::READ_STATUS1], &mut buffer)?;
        Ok(buffer[0])
    }

    /// Read status register 2 (`35h`).
    pub fn read_status2(&mut self) -> Result<u8, FlashError<B::Error>> {
        let mut buffer = [0u8; 1];
        self.command_then_read(&[opcode::READ_STATUS2], &mut buffer)?;
        Ok(buffer[0])
    }

    /// Return whether an erase, program, or status-write operation is active.
    pub fn is_busy(&mut self) -> Result<bool, FlashError<B::Error>> {
        Ok(self.read_status1()? & status::BUSY != 0)
    }

    fn wait_ready(&mut self, timeout_us: u32) -> Result<(), FlashError<B::Error>> {
        let mut waited = 0u32;
        loop {
            if self.read_status1()? & status::BUSY == 0 {
                return Ok(());
            }
            if waited >= timeout_us {
                return Err(FlashError::Timeout);
            }
            self.delay.delay_us(opcode::POLL_INTERVAL_US);
            waited = waited.saturating_add(opcode::POLL_INTERVAL_US);
        }
    }

    fn write_enable(&mut self) -> Result<(), FlashError<B::Error>> {
        self.command(&[opcode::WRITE_ENABLE])?;
        if self.read_status1()? & status::WRITE_ENABLE_LATCH == 0 {
            return Err(FlashError::WriteEnableFailed);
        }
        Ok(())
    }

    fn write_status1(&mut self, value: u8) -> Result<(), FlashError<B::Error>> {
        self.write_enable()?;
        self.command(&[opcode::WRITE_STATUS1, value])?;
        self.wait_ready(opcode::STATUS_WRITE_TIMEOUT_US)
    }

    fn write_status2(&mut self, value: u8) -> Result<(), FlashError<B::Error>> {
        self.write_enable()?;
        self.command(&[opcode::WRITE_STATUS2, value])?;
        self.wait_ready(opcode::STATUS_WRITE_TIMEOUT_US)
    }

    /// Read the 3-byte JEDEC identification (`9Fh`).
    pub fn jedec_id(&mut self) -> Result<NorJedecId, FlashError<B::Error>> {
        let mut buffer = [0u8; 3];
        self.command_then_read(&[opcode::READ_ID], &mut buffer)?;
        Ok(NorJedecId {
            manufacturer: buffer[0],
            memory_type: buffer[1],
            capacity: buffer[2],
        })
    }

    /// Read the 64-bit factory unique ID (`4Bh` + four dummy bytes).
    pub fn unique_id(&mut self) -> Result<u64, FlashError<B::Error>> {
        let mut buffer = [0u8; 8];
        self.command_then_read(
            &[opcode::READ_UNIQUE_ID, 0x00, 0x00, 0x00, 0x00],
            &mut buffer,
        )?;
        Ok(u64::from_be_bytes(buffer))
    }

    /// Check whether the quad-enable (`QE`) status bit is set.
    pub fn quad_enabled(&mut self) -> Result<bool, FlashError<B::Error>> {
        if self.read_status2()? & status::QE_SR2 != 0 {
            return Ok(true);
        }
        Ok(self.read_status1()? & status::QE_SR1 != 0)
    }

    /// Set the part's quad-enable (`QE`) bit.
    pub fn enable_quad(&mut self) -> Result<(), FlashError<B::Error>> {
        if self.quad_enabled()? {
            return Ok(());
        }

        let status2 = self.read_status2()?;
        self.write_status2(status2 | status::QE_SR2)?;
        if self.read_status2()? & status::QE_SR2 != 0 {
            return Ok(());
        }

        let status1 = self.read_status1()?;
        self.write_status1(status1 | status::QE_SR1)?;
        if self.read_status1()? & status::QE_SR1 != 0 {
            return Ok(());
        }

        Err(FlashError::NotSupported)
    }

    /// Clear the part's quad-enable (`QE`) bit.
    pub fn disable_quad(&mut self) -> Result<(), FlashError<B::Error>> {
        let status2 = self.read_status2()?;
        if status2 & status::QE_SR2 != 0 {
            self.write_status2(status2 & !status::QE_SR2)?;
            if self.read_status2()? & status::QE_SR2 != 0 {
                return Err(FlashError::NotSupported);
            }
            return Ok(());
        }

        let status1 = self.read_status1()?;
        if status1 & status::QE_SR1 == 0 {
            return Ok(());
        }
        self.write_status1(status1 & !status::QE_SR1)?;
        if self.read_status1()? & status::QE_SR1 != 0 {
            return Err(FlashError::NotSupported);
        }
        Ok(())
    }

    /// Read `bytes.len()` bytes starting at `offset` using the configured bus width.
    pub fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), FlashError<B::Error>> {
        if !range_in_bounds(self.device.capacity, offset, bytes.len()) {
            return Err(FlashError::OutOfBounds);
        }
        let mut done = 0usize;
        while done < bytes.len() {
            let length = (bytes.len() - done).min(opcode::MAX_BULK_LEN);
            let current = offset + done as u32;
            if let Some(quad) = self.config.quad {
                let address = address_bytes(current);
                let command = [quad.read_opcode, address[0], address[1], address[2]];
                self.cs_low();
                let result = self
                    .bus
                    .read_quad(
                        &command,
                        quad.tclqv_ns,
                        quad.dummy_margin,
                        &mut bytes[done..done + length],
                    )
                    .map_err(FlashError::Spi);
                self.cs_high();
                result?;
            } else {
                let command = read_command(current);
                self.command_then_read(&command, &mut bytes[done..done + length])?;
            }
            done += length;
        }
        Ok(())
    }

    /// Program page-aligned `bytes` starting at `offset` using the configured bus width.
    pub fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), FlashError<B::Error>> {
        if !range_in_bounds(self.device.capacity, offset, bytes.len()) {
            return Err(FlashError::OutOfBounds);
        }
        let page_size = self.device.page_size;
        if !(offset as usize).is_multiple_of(page_size) || !bytes.len().is_multiple_of(page_size) {
            return Err(FlashError::NotAligned);
        }
        for (index, chunk) in bytes.chunks(page_size).enumerate() {
            self.page_program(offset + (index * page_size) as u32, chunk)?;
        }
        Ok(())
    }

    fn page_program(&mut self, offset: u32, data: &[u8]) -> Result<(), FlashError<B::Error>> {
        self.write_enable()?;
        if let Some(quad) = self.config.quad {
            let address = address_bytes(offset);
            let command = [quad.page_program_opcode, address[0], address[1], address[2]];
            self.cs_low();
            let result = self.bus.write_quad(&command, data).map_err(FlashError::Spi);
            self.cs_high();
            result?;
        } else {
            let command = page_program_command(offset);
            self.cs_low();
            let mut result = self.bus.write(&command).map_err(FlashError::Spi);
            if result.is_ok() {
                result = self.bus.write(data).map_err(FlashError::Spi);
            }
            self.cs_high();
            result?;
        }
        self.wait_ready(opcode::PAGE_PROGRAM_TIMEOUT_US)?;

        let mut verify = AlignedPage([0u8; MAX_VERIFY_PAGE_SIZE]);
        self.read(offset, &mut verify.0[..data.len()])?;
        if &verify.0[..data.len()] != data {
            return Err(FlashError::VerifyFailed);
        }
        Ok(())
    }

    /// Clear status-register block-protection bits while preserving `QE`.
    pub fn unprotect(&mut self) -> Result<(u8, u8), FlashError<B::Error>> {
        let before = self.read_status1()?;
        let sr2 = self.read_status2()?;
        let wanted_sr1 = before & !status::SR1_PROTECT_MASK;
        let wanted_sr2 = sr2 & !status::SR2_PROTECT_MASK;
        if wanted_sr1 != before {
            self.write_status1(wanted_sr1)?;
        }
        if wanted_sr2 != sr2 {
            self.write_status2(wanted_sr2)?;
        }
        let after = self.read_status1()?;
        Ok((before, after))
    }

    /// Erase the sector-aligned byte range `from..to`.
    pub fn erase(&mut self, from: u32, to: u32) -> Result<(), FlashError<B::Error>> {
        let (from_usize, to_usize) = (from as usize, to as usize);
        let sector = self.device.sector_size;
        let block = self.device.block_size;
        if from_usize > to_usize || to_usize > self.device.capacity {
            return Err(FlashError::OutOfBounds);
        }
        if !from_usize.is_multiple_of(sector) || !to_usize.is_multiple_of(sector) {
            return Err(FlashError::NotAligned);
        }
        let mut address = from;
        while address < to {
            let (kind, next) = erase_step(address, to, sector as u32, block as u32);
            match kind {
                EraseKind::Block => self.erase_block(address)?,
                EraseKind::Sector => self.erase_sector(address)?,
            }
            address = next;
        }
        Ok(())
    }

    /// Erase one sector at sector-aligned `offset` (`20h`).
    pub fn erase_sector(&mut self, offset: u32) -> Result<(), FlashError<B::Error>> {
        let sector = self.device.sector_size;
        if !range_in_bounds(self.device.capacity, offset, sector) {
            return Err(FlashError::OutOfBounds);
        }
        if !(offset as usize).is_multiple_of(sector) {
            return Err(FlashError::NotAligned);
        }
        self.write_enable()?;
        let command = sector_erase_command(offset);
        self.command(&command)?;
        self.wait_ready(opcode::SECTOR_ERASE_TIMEOUT_US)
    }

    /// Erase one 64 KiB block at block-aligned `offset` (`D8h`).
    pub fn erase_block(&mut self, offset: u32) -> Result<(), FlashError<B::Error>> {
        let block = self.device.block_size;
        if !range_in_bounds(self.device.capacity, offset, block) {
            return Err(FlashError::OutOfBounds);
        }
        if !(offset as usize).is_multiple_of(block) {
            return Err(FlashError::NotAligned);
        }
        self.write_enable()?;
        let command = block_erase_command(offset);
        self.command(&command)?;
        self.wait_ready(opcode::BLOCK_ERASE_TIMEOUT_US)
    }
}

const fn address_bytes(offset: u32) -> [u8; 3] {
    [(offset >> 16) as u8, (offset >> 8) as u8, offset as u8]
}

const fn read_command(offset: u32) -> [u8; 4] {
    let address = address_bytes(offset);
    [opcode::READ, address[0], address[1], address[2]]
}

const fn page_program_command(offset: u32) -> [u8; 4] {
    let address = address_bytes(offset);
    [opcode::PAGE_PROGRAM, address[0], address[1], address[2]]
}

const fn sector_erase_command(offset: u32) -> [u8; 4] {
    let address = address_bytes(offset);
    [opcode::SECTOR_ERASE, address[0], address[1], address[2]]
}

const fn block_erase_command(offset: u32) -> [u8; 4] {
    let address = address_bytes(offset);
    [opcode::BLOCK_ERASE, address[0], address[1], address[2]]
}

const fn range_in_bounds(capacity: usize, offset: u32, length: usize) -> bool {
    let offset = offset as usize;
    offset <= capacity && length <= capacity - offset
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EraseKind {
    Sector,
    Block,
}

const fn erase_step(address: u32, end: u32, sector: u32, block: u32) -> (EraseKind, u32) {
    if address.is_multiple_of(block) && end - address >= block {
        (EraseKind::Block, address + block)
    } else {
        (EraseKind::Sector, address + sector)
    }
}

impl<B, CS, D> ErrorType for SpiNorFlash<B, CS, D>
where
    B: QuadSpiBus,
    CS: OutputPin<Error = Infallible>,
    D: DelayNs,
{
    type Error = FlashError<B::Error>;
}

impl<B, CS, D> ReadNorFlash for SpiNorFlash<B, CS, D>
where
    B: QuadSpiBus,
    CS: OutputPin<Error = Infallible>,
    D: DelayNs,
{
    const READ_SIZE: usize = NOR_READ_SIZE;

    fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        SpiNorFlash::read(self, offset, bytes)
    }

    fn capacity(&self) -> usize {
        self.device.capacity
    }
}

impl<B, CS, D> NorFlash for SpiNorFlash<B, CS, D>
where
    B: QuadSpiBus,
    CS: OutputPin<Error = Infallible>,
    D: DelayNs,
{
    const WRITE_SIZE: usize = NOR_PAGE_SIZE;
    const ERASE_SIZE: usize = NOR_SECTOR_SIZE;

    fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
        SpiNorFlash::erase(self, from, to)
    }

    fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        SpiNorFlash::write(self, offset, bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_command_encodes_opcode_and_address() {
        assert_eq!(read_command(0x00_0000), [0x03, 0x00, 0x00, 0x00]);
        assert_eq!(read_command(0x12_3456), [0x03, 0x12, 0x34, 0x56]);
        assert_eq!(read_command(0xFF_FFFF), [0x03, 0xFF, 0xFF, 0xFF]);
    }

    #[test]
    fn erase_plan_uses_blocks_only_for_aligned_block_ranges() {
        assert_eq!(
            erase_step(0x0000_0000, 0x0001_0000, 0x1000, 0x1_0000),
            (EraseKind::Block, 0x0001_0000)
        );
        assert_eq!(
            erase_step(0x0001_0000, 0x0003_0000, 0x1000, 0x1_0000),
            (EraseKind::Block, 0x0002_0000)
        );
        assert_eq!(
            erase_step(0x0000_1000, 0x0000_5000, 0x1000, 0x1_0000),
            (EraseKind::Sector, 0x0000_2000)
        );
        assert_eq!(
            erase_step(0x0001_0000, 0x0001_8000, 0x1000, 0x1_0000),
            (EraseKind::Sector, 0x0001_1000)
        );
    }

    #[test]
    fn jedec_table_resolves_w25q128jv_and_power_of_two_capacities() {
        let id = NorJedecId {
            manufacturer: 0xEF,
            memory_type: 0x40,
            capacity: 0x18,
        };
        assert_eq!(NorDeviceInfo::from_jedec(id), NorDeviceInfo::W25Q128JV);

        let generic_4m = NorJedecId {
            manufacturer: 0x1C,
            memory_type: 0x70,
            capacity: 0x16,
        };
        assert_eq!(
            NorDeviceInfo::from_jedec(generic_4m).capacity,
            4 * 1024 * 1024
        );
    }
}
