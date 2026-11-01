//! SPI NAND flash driver implementation and [`RawNandFlash`] trait.

use core::convert::Infallible;

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

use super::config::{NandGeometry, NandInit, NandJedecId, SpiNandConfig};
use super::opcode;
use super::status::{Configuration, EccVerdict, Protection, Status};
use crate::qspi::QuadSpiBus;
use crate::spi_flash::FlashError;

/// Maximum supported NAND main-area page size for stack-allocated verification.
const MAX_VERIFY_PAGE_SIZE: usize = 4096;

#[repr(align(16))]
struct AlignedPage([u8; MAX_VERIFY_PAGE_SIZE]);

/// SPI NAND flash device driver.
pub struct SpiNandFlash<B, CS, D> {
    bus: B,
    cs: CS,
    delay: D,
    config: SpiNandConfig,
    geometry: NandGeometry,
    init: NandInit,
}

impl<B, CS, D> SpiNandFlash<B, CS, D>
where
    B: QuadSpiBus,
    CS: OutputPin<Error = Infallible>,
    D: DelayNs,
{
    /// Initialize the SPI NAND flash in single-line mode and run bring-up.
    pub fn new(bus: B, cs: CS, delay: D) -> Result<Self, FlashError<B::Error>> {
        Self::with_config(bus, cs, delay, SpiNandConfig::single())
    }

    /// Initialize the SPI NAND flash with an explicit configuration and run bring-up.
    pub fn with_config(
        bus: B,
        mut cs: CS,
        delay: D,
        config: SpiNandConfig,
    ) -> Result<Self, FlashError<B::Error>> {
        cs.set_high().ok();
        let mut flash = Self {
            bus,
            cs,
            delay,
            config,
            geometry: config.geometry.unwrap_or(NandGeometry::W25N01GV),
            init: NandInit::UNKNOWN,
        };
        flash.init = flash.bring_up()?;
        if config.geometry.is_none() {
            flash.geometry = NandGeometry::from_jedec(flash.init.id);
        }
        if flash.geometry.page_size > MAX_VERIFY_PAGE_SIZE {
            return Err(FlashError::NotSupported);
        }
        Ok(flash)
    }

    /// Identification and feature-register snapshot recorded during bring-up.
    pub const fn info(&self) -> NandInit {
        self.init
    }

    /// Active driver configuration.
    pub const fn config(&self) -> SpiNandConfig {
        self.config
    }

    /// Resolved device geometry.
    pub const fn geometry(&self) -> NandGeometry {
        self.geometry
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

    fn command(&mut self, bytes: &[u8]) -> Result<(), FlashError<B::Error>> {
        self.cs_low();
        let result = self.bus.write(bytes).map_err(FlashError::Spi);
        self.cs_high();
        result
    }

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

    fn read_register(&mut self, register: u8) -> Result<u8, FlashError<B::Error>> {
        let mut buffer = [0u8; 1];
        self.command_then_read(&[opcode::GET_FEATURE, register], &mut buffer)?;
        Ok(buffer[0])
    }

    /// Clear the configuration register's `QE` bit.
    pub fn disable_quad(&mut self) -> Result<(), FlashError<B::Error>> {
        let configuration = self.read_configuration()?;
        if configuration.quad_enabled() {
            let wanted = configuration.with_bits(Configuration::QUAD_ENABLE, 0);
            self.write_register(opcode::REG_CONFIGURATION, wanted.byte())?;
            if self.read_configuration()?.quad_enabled() {
                return Err(FlashError::NotSupported);
            }
        }
        Ok(())
    }

    /// Write one feature register (`1Fh` + register address + value) after `WRITE ENABLE`.
    pub fn write_register(&mut self, register: u8, value: u8) -> Result<(), FlashError<B::Error>> {
        self.write_enable()?;
        self.command(&[opcode::SET_FEATURE, register, value])?;
        self.wait_ready(opcode::WRITE_REGISTER_TIMINGS)
    }

    /// Set the write-enable latch (`06h`) and verify `WEL`.
    pub fn write_enable(&mut self) -> Result<(), FlashError<B::Error>> {
        self.command(&[opcode::WRITE_ENABLE])?;
        if !self.read_status()?.write_enabled() {
            return Err(FlashError::WriteEnableFailed);
        }
        Ok(())
    }

    /// Read the status register (`C0h`).
    pub fn read_status(&mut self) -> Result<Status, FlashError<B::Error>> {
        Ok(Status::from_byte(self.read_register(opcode::REG_STATUS)?))
    }

    /// Read the configuration register (`B0h`).
    pub fn read_configuration(&mut self) -> Result<Configuration, FlashError<B::Error>> {
        Ok(Configuration::from_byte(
            self.read_register(opcode::REG_CONFIGURATION)?,
        ))
    }

    /// Read the block-protection register (`A0h`).
    pub fn read_protection(&mut self) -> Result<Protection, FlashError<B::Error>> {
        Ok(Protection::from_byte(
            self.read_register(opcode::REG_BLOCK_PROTECTION)?,
        ))
    }

    fn wait_ready(&mut self, timings: opcode::Timings) -> Result<(), FlashError<B::Error>> {
        self.delay.delay_us(timings.initial_us);
        let mut waited = timings.initial_us;
        loop {
            if !self.read_status()?.busy() {
                return Ok(());
            }
            if waited >= timings.timeout_us {
                return Err(FlashError::Timeout);
            }
            self.delay.delay_us(timings.poll_us);
            waited += timings.poll_us;
        }
    }

    /// Reset the part (`FFh`).
    pub fn reset(&mut self) -> Result<(), FlashError<B::Error>> {
        self.command(&[opcode::RESET])?;
        self.wait_ready(opcode::RESET_TIMINGS)
    }

    /// Read the JEDEC identification (`9Fh` + dummy byte).
    pub fn jedec_id(&mut self) -> Result<NandJedecId, FlashError<B::Error>> {
        let mut buffer = [0u8; 3];
        self.command_then_read(&[opcode::READ_ID, 0x00], &mut buffer)?;
        Ok(NandJedecId::from_bytes(buffer))
    }

    /// Reset the part and read its JEDEC identification.
    pub fn reset_and_identify(&mut self) -> Result<NandJedecId, FlashError<B::Error>> {
        self.reset()?;
        self.jedec_id()
    }

    /// Enable on-die ECC, buffer-read mode, and quad mode when configured.
    pub fn ensure_configuration(&mut self) -> Result<Configuration, FlashError<B::Error>> {
        let before = self.read_configuration()?;
        let mut wanted = before.with_bits(
            Configuration::ECC_ENABLE | Configuration::BUFFER_READ,
            Configuration::OPERATIONAL.byte(),
        );
        if self.config.quad.is_some() {
            wanted = wanted.with_bits(Configuration::QUAD_ENABLE, Configuration::QUAD_ENABLE);
        }
        if wanted != before {
            self.write_register(opcode::REG_CONFIGURATION, wanted.byte())?;
            if self.read_configuration()? != wanted {
                return Err(FlashError::NotSupported);
            }
        }
        Ok(before)
    }

    /// Unlock all blocks in the block-protection register (`A0h`).
    pub fn unprotect(&mut self) -> Result<Protection, FlashError<B::Error>> {
        let before = self.read_protection()?;
        if !before.is_unlocked() {
            self.write_register(opcode::REG_BLOCK_PROTECTION, 0x00)?;
        }
        Ok(before)
    }

    fn bring_up(&mut self) -> Result<NandInit, FlashError<B::Error>> {
        let id = self.reset_and_identify()?;
        let configuration_before = self.ensure_configuration()?;
        let protection_before = self.unprotect()?;
        let configuration_after = self.read_configuration()?;
        Ok(NandInit {
            id,
            configuration_before,
            configuration_after,
            protection_before,
        })
    }

    /// Load `page` from the NAND array into the cache (`13h` + 3-byte row address) and check ECC.
    pub fn page_read(&mut self, page: usize) -> Result<(), FlashError<B::Error>> {
        self.check_page(page)?;
        self.load_page(page)?;
        if self.read_status()?.ecc_verdict() == EccVerdict::Uncorrectable {
            return Err(FlashError::UncorrectableEcc);
        }
        Ok(())
    }

    fn load_page(&mut self, page: usize) -> Result<(), FlashError<B::Error>> {
        let address = page_address(page);
        self.command(&[opcode::PAGE_READ, address[0], address[1], address[2]])?;
        self.wait_ready(opcode::READ_TIMINGS)
    }

    /// Read from the cache starting at `column` using the configured bus width.
    pub fn read_cache(
        &mut self,
        column: usize,
        buffer: &mut [u8],
    ) -> Result<(), FlashError<B::Error>> {
        self.check_cache_range(column, buffer.len())?;
        let address = (column as u16).to_be_bytes();
        if let Some(quad) = self.config.quad {
            let command = [quad.read_cache_opcode, address[0], address[1]];
            self.cs_low();
            let result = self
                .bus
                .read_quad(&command, quad.tclqv_ns, quad.dummy_margin, buffer)
                .map_err(FlashError::Spi);
            self.cs_high();
            result
        } else {
            self.command_then_read(&[opcode::READ_CACHE, address[0], address[1], 0x00], buffer)
        }
    }

    /// Load program data into the cache starting at `column` using the configured bus width.
    pub fn load_program_data(
        &mut self,
        column: usize,
        data: &[u8],
    ) -> Result<(), FlashError<B::Error>> {
        self.check_cache_range(column, data.len())?;
        let address = (column as u16).to_be_bytes();
        if let Some(quad) = self.config.quad {
            let command = [quad.load_program_opcode, address[0], address[1]];
            self.cs_low();
            let result = self.bus.write_quad(&command, data).map_err(FlashError::Spi);
            self.cs_high();
            result
        } else {
            self.cs_low();
            let mut result = self
                .bus
                .write(&[opcode::LOAD_PROGRAM_DATA, address[0], address[1]])
                .map_err(FlashError::Spi);
            if result.is_ok() {
                result = self.bus.write(data).map_err(FlashError::Spi);
            }
            self.cs_high();
            result
        }
    }

    /// Execute cache program into `page` (`10h` + 3-byte row address).
    pub fn program_execute(&mut self, page: usize) -> Result<(), FlashError<B::Error>> {
        self.check_page(page)?;
        let address = page_address(page);
        self.command(&[opcode::PROGRAM_EXECUTE, address[0], address[1], address[2]])?;
        self.wait_ready(opcode::WRITE_TIMINGS)?;
        if self.read_status()?.program_failed() {
            return Err(FlashError::ProgramFailed);
        }
        Ok(())
    }

    /// Read one full main-area page.
    pub fn read_page(
        &mut self,
        page: usize,
        buffer: &mut [u8],
    ) -> Result<(), FlashError<B::Error>> {
        if buffer.len() != self.geometry.page_size {
            return Err(FlashError::InvalidLength);
        }
        self.page_read(page)?;
        self.read_cache(0, buffer)
    }

    /// Program one full main-area page and verify it by reading back.
    pub fn write_page(&mut self, page: usize, data: &[u8]) -> Result<(), FlashError<B::Error>> {
        let page_size = self.geometry.page_size;
        if data.len() != page_size {
            return Err(FlashError::InvalidLength);
        }
        self.check_page(page)?;
        self.write_enable()?;
        self.load_program_data(0, data)?;
        self.program_execute(page)?;

        let mut verify = AlignedPage([0u8; MAX_VERIFY_PAGE_SIZE]);
        self.read_page(page, &mut verify.0[..page_size])?;
        if &verify.0[..page_size] != data {
            return Err(FlashError::VerifyFailed);
        }
        Ok(())
    }

    /// Erase one block (`D8h` + block start row address).
    pub fn erase_block(&mut self, block: usize) -> Result<(), FlashError<B::Error>> {
        if block >= self.geometry.block_count {
            return Err(FlashError::OutOfBounds);
        }
        self.write_enable()?;
        let address = page_address(block * self.geometry.pages_per_block);
        self.command(&[opcode::BLOCK_ERASE, address[0], address[1], address[2]])?;
        self.wait_ready(opcode::ERASE_TIMINGS)?;
        if self.read_status()?.erase_failed() {
            return Err(FlashError::EraseFailed);
        }
        Ok(())
    }

    /// Read `buffer.len()` bytes starting at byte `offset` across page boundaries.
    pub fn read(&mut self, offset: usize, buffer: &mut [u8]) -> Result<(), FlashError<B::Error>> {
        self.check_range(offset, buffer.len())?;
        let page_size = self.geometry.page_size;
        let mut done = 0usize;
        while done < buffer.len() {
            let address = offset + done;
            let page = address / page_size;
            let column = address % page_size;
            let chunk = (page_size - column).min(buffer.len() - done);
            self.page_read(page)?;
            self.read_cache(column, &mut buffer[done..done + chunk])?;
            done += chunk;
        }
        Ok(())
    }

    /// Return whether `block` carries a non-`0xFF` bad-block marker in page 0 OOB[0].
    pub fn is_bad_block(&mut self, block: usize) -> Result<bool, FlashError<B::Error>> {
        if block >= self.geometry.block_count {
            return Err(FlashError::OutOfBounds);
        }
        self.load_page(block * self.geometry.pages_per_block)?;
        let mut marker = [0u8; 1];
        self.read_cache(self.geometry.page_size, &mut marker)?;
        Ok(marker[0] != 0xFF)
    }

    /// Mark `block` bad by writing `0x00` to the first spare byte of its first page.
    pub fn mark_bad_block(&mut self, block: usize) -> Result<(), FlashError<B::Error>> {
        if block >= self.geometry.block_count {
            return Err(FlashError::OutOfBounds);
        }
        if self.is_bad_block(block)? {
            return Ok(());
        }

        self.erase_block(block)?;
        self.write_enable()?;

        // Keep main area erased (`0xFF`) while programming the OOB bad-block marker.
        const FF_CHUNK: [u8; 256] = [0xFF; 256];
        let page_size = self.geometry.page_size;
        let mut column = 0;
        while column < page_size {
            let step = (page_size - column).min(FF_CHUNK.len());
            self.load_program_data(column, &FF_CHUNK[..step])?;
            column += step;
        }
        self.load_program_data(page_size, &[0x00])?;
        self.program_execute(block * self.geometry.pages_per_block)?;

        if !self.is_bad_block(block)? {
            return Err(FlashError::VerifyFailed);
        }
        Ok(())
    }

    fn check_page(&self, page: usize) -> Result<(), FlashError<B::Error>> {
        if page < self.geometry.page_count() {
            Ok(())
        } else {
            Err(FlashError::OutOfBounds)
        }
    }

    fn check_range(&self, offset: usize, length: usize) -> Result<(), FlashError<B::Error>> {
        match offset.checked_add(length) {
            Some(end) if end <= self.geometry.capacity() => Ok(()),
            _ => Err(FlashError::OutOfBounds),
        }
    }

    fn check_cache_range(&self, column: usize, length: usize) -> Result<(), FlashError<B::Error>> {
        match column.checked_add(length) {
            Some(end) if end <= self.geometry.page_size + self.geometry.oob_size => Ok(()),
            _ => Err(FlashError::OutOfBounds),
        }
    }
}

const fn page_address(page: usize) -> [u8; 3] {
    [(page >> 16) as u8, (page >> 8) as u8, page as u8]
}

/// Media-agnostic interface to a raw SPI NAND device.
pub trait RawNandFlash {
    /// Error type reported by NAND operations.
    type Error: core::fmt::Debug;

    /// Device geometry (page, OOB, and block counts).
    fn geometry(&self) -> NandGeometry;

    /// Read one full main-area page.
    fn read_page(&mut self, page: usize, buf: &mut [u8]) -> Result<(), Self::Error>;

    /// Program one full main-area page.
    fn write_page(&mut self, page: usize, data: &[u8]) -> Result<(), Self::Error>;

    /// Erase one block.
    fn erase_block(&mut self, block: usize) -> Result<(), Self::Error>;

    /// Return whether `block` is marked bad.
    fn is_bad_block(&mut self, block: usize) -> Result<bool, Self::Error>;

    /// Clear block protection so erase and program operations succeed.
    fn unprotect(&mut self) -> Result<(), Self::Error>;

    /// Read `buf.len()` bytes starting at byte `offset` across page boundaries.
    fn read(&mut self, offset: usize, buf: &mut [u8]) -> Result<(), Self::Error>;

    /// Main-area bytes per page.
    fn page_size(&self) -> usize {
        self.geometry().page_size
    }

    /// Pages per erase block.
    fn pages_per_block(&self) -> usize {
        self.geometry().pages_per_block
    }

    /// Number of erase blocks on the device.
    fn block_count(&self) -> usize {
        self.geometry().block_count
    }

    /// Spare-area (OOB) bytes per page.
    fn oob_size(&self) -> usize {
        self.geometry().oob_size
    }

    /// Bytes per erase block.
    fn block_size(&self) -> usize {
        self.geometry().block_size()
    }

    /// Total addressable main-area capacity in bytes.
    fn capacity(&self) -> usize {
        self.geometry().capacity()
    }
}

impl<B, CS, D> RawNandFlash for SpiNandFlash<B, CS, D>
where
    B: QuadSpiBus,
    CS: OutputPin<Error = Infallible>,
    D: DelayNs,
{
    type Error = FlashError<B::Error>;

    fn geometry(&self) -> NandGeometry {
        self.geometry
    }

    fn read_page(&mut self, page: usize, buf: &mut [u8]) -> Result<(), Self::Error> {
        SpiNandFlash::read_page(self, page, buf)
    }

    fn write_page(&mut self, page: usize, data: &[u8]) -> Result<(), Self::Error> {
        SpiNandFlash::write_page(self, page, data)
    }

    fn erase_block(&mut self, block: usize) -> Result<(), Self::Error> {
        SpiNandFlash::erase_block(self, block)
    }

    fn is_bad_block(&mut self, block: usize) -> Result<bool, Self::Error> {
        SpiNandFlash::is_bad_block(self, block)
    }

    fn unprotect(&mut self) -> Result<(), Self::Error> {
        SpiNandFlash::unprotect(self).map(|_| ())
    }

    fn read(&mut self, offset: usize, buf: &mut [u8]) -> Result<(), Self::Error> {
        SpiNandFlash::read(self, offset, buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_address_is_big_endian_and_three_bytes() {
        assert_eq!(page_address(0), [0x00, 0x00, 0x00]);
        assert_eq!(page_address(1), [0x00, 0x00, 0x01]);
        assert_eq!(page_address(0x0102), [0x00, 0x01, 0x02]);
        assert_eq!(page_address(0x12_3456), [0x12, 0x34, 0x56]);
        assert_eq!(
            page_address(NandGeometry::W25N01GV.page_count() - 1),
            [0x00, 0xFF, 0xFF]
        );
    }

    #[test]
    fn nand_geometry_resolves_jedec_parts() {
        let w25n01 = NandJedecId::from_bytes([0xEF, 0xAA, 0x21]);
        assert_eq!(NandGeometry::from_jedec(w25n01), NandGeometry::W25N01GV);

        let w25n02 = NandJedecId::from_bytes([0xEF, 0xAB, 0x21]);
        assert_eq!(NandGeometry::from_jedec(w25n02), NandGeometry::W25N02KV);
    }
}
