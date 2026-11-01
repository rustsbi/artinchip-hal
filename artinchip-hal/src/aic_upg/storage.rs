//! Upgrade storage abstraction and NOR/NAND partition backends.
//!
//! * [`NorUpgStorage`] buffers writes per erase sector (read-modify-write) over any
//!   `embedded_storage::nor_flash::NorFlash` device.
//! * [`NandUpgStorage`] assembles pages over any [`RawNandFlash`] device, queries
//!   geometry dynamically from [`RawNandFlash::geometry`], and refuses factory-marked
//!   bad blocks rather than remapping absolute host image offsets.

use embedded_storage::nor_flash::NorFlash;

use crate::spi_flash::RawNandFlash;

/// Standard 4 KiB SPI NOR sector size used for sector assembly buffers.
pub const SECTOR_SIZE: usize = 4096;

/// Maximum supported SPI NAND main-area page size in [`NandUpgStorage`].
const MAX_NAND_PAGE_SIZE: usize = 4096;

/// One entry of the on-device flash partition table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Partition {
    /// Partition name matched against `fwc_meta.partition`.
    pub name: &'static str,
    /// Byte offset in flash.
    pub offset: u64,
    /// Partition size in bytes.
    pub size: u64,
}

/// Media-agnostic storage interface driven by [`Engine`](super::Engine).
pub trait StorageBackend {
    /// Media type string reported to the host (for example `"spi-nor"` or `"spi-nand"`).
    fn media_type(&self) -> &'static str;

    /// Media controller index.
    fn media_dev_id(&self) -> u32 {
        0
    }

    /// Erase/program block size in bytes reported to the host.
    fn block_size(&self) -> u32;

    /// Total addressable capacity in bytes.
    fn capacity(&self) -> u64;

    /// Erase `size` bytes starting at `start`.
    fn erase(&mut self, start: u64, size: u64) -> bool;

    /// Program `data` starting at media `offset`; returns bytes written.
    fn write(&mut self, offset: u64, data: &[u8]) -> usize;

    /// Read `out.len()` bytes starting at media `offset`; returns bytes read.
    fn read(&mut self, offset: u64, out: &mut [u8]) -> usize;

    /// RAM address of the host-visible staging buffer (`GET_MEM_BUF`).
    fn scratch_addr(&self) -> u32;

    /// Length of the staging buffer in bytes.
    fn scratch_len(&self) -> usize;

    /// Mutable view of the staging buffer.
    fn scratch(&mut self) -> &mut [u8];

    /// Program `len` bytes from `scratch[scratch_off..]` to media `offset`.
    fn write_from_scratch(&mut self, offset: u64, scratch_off: usize, len: usize) -> usize;

    /// Read `len` bytes from media `offset` into `scratch[scratch_off..]`.
    fn read_into_scratch(&mut self, offset: u64, scratch_off: usize, len: usize) -> usize;

    /// Resolve the flash offset of a named partition (first token of `;`-separated names).
    fn partition_offset(&self, name: &str) -> Option<u64>;

    /// Flush any buffered sector/page writes to flash.
    ///
    /// Returns `false` if the buffered data could not be written. On failure the
    /// backend keeps the buffer intact so the host can retry the operation.
    fn flush(&mut self) -> bool {
        true
    }

    /// Boot stage reported at `GET_HWINFO` offset 40 (`1` = FWC bootloader, `2` = Storage R/W).
    fn boot_stage(&self) -> u8 {
        2
    }
}

impl Partition {
    fn table_capacity(partitions: &[Partition]) -> u64 {
        partitions
            .iter()
            .map(|partition| partition.offset + partition.size)
            .max()
            .unwrap_or(0)
    }

    fn offset_of(partitions: &[Partition], name: &str) -> Option<u64> {
        let first = name.split(';').next().unwrap_or(name).trim();
        partitions
            .iter()
            .find(|partition| partition.name == first)
            .map(|partition| partition.offset)
    }
}

/// SPI NOR backed [`StorageBackend`].
pub struct NorUpgStorage<'a, F: NorFlash> {
    flash: F,
    partitions: &'a [Partition],
    buffer: &'a mut [u8],
    buffer_addr: u64,
    buffer_len: usize,
    scratch: &'a mut [u8],
    capacity: u64,
}

impl<'a, F: NorFlash> NorUpgStorage<'a, F> {
    /// Wrap `flash` with a sector read-modify-write `buffer` and host `scratch` window.
    pub fn new(
        flash: F,
        buffer: &'a mut [u8],
        scratch: &'a mut [u8],
        partitions: &'a [Partition],
    ) -> Self {
        assert!(
            buffer.len() >= F::ERASE_SIZE,
            "NorUpgStorage requires a sector buffer of at least F::ERASE_SIZE bytes"
        );
        let capacity = (flash.capacity() as u64).max(Partition::table_capacity(partitions));
        Self {
            flash,
            partitions,
            buffer,
            buffer_addr: 0,
            buffer_len: 0,
            scratch,
            capacity,
        }
    }

    /// Borrow the underlying NOR flash driver after flushing buffered writes.
    pub fn flash_mut(&mut self) -> Option<&mut F> {
        self.flush_buffer().then_some(&mut self.flash)
    }

    fn flush_buffer(&mut self) -> bool {
        if self.buffer_len == 0 {
            return true;
        }
        let addr = self.buffer_addr as u32;
        let sector = F::ERASE_SIZE;
        if let Err(err) = self.flash.erase(addr, addr + sector as u32) {
            log::error!("nor: Erase of sector {addr:#x} failed: {err:?}");
            return false;
        }
        if let Err(err) = self.flash.write(addr, &self.buffer[..sector]) {
            log::error!("nor: Program of sector {addr:#x} failed: {err:?}");
            return false;
        }
        self.buffer_len = 0;
        true
    }
}

impl<F: NorFlash> StorageBackend for NorUpgStorage<'_, F> {
    fn media_type(&self) -> &'static str {
        "spi-nor"
    }

    fn block_size(&self) -> u32 {
        F::ERASE_SIZE as u32
    }

    fn capacity(&self) -> u64 {
        self.capacity
    }

    fn erase(&mut self, start: u64, size: u64) -> bool {
        if !self.flush_buffer() {
            return false;
        }
        let end = start.saturating_add(size);
        if end > self.capacity {
            return false;
        }
        let sector = F::ERASE_SIZE as u64;
        let mut addr = start & !(sector - 1);
        while addr < end {
            if self
                .flash
                .erase(addr as u32, (addr + sector) as u32)
                .is_err()
            {
                return false;
            }
            addr += sector;
        }
        true
    }

    fn write(&mut self, offset: u64, data: &[u8]) -> usize {
        if data.is_empty() || offset + data.len() as u64 > self.capacity {
            return 0;
        }
        let sector_size = F::ERASE_SIZE as u64;
        let mut written = 0usize;
        while written < data.len() {
            let addr = offset + written as u64;
            let sector = addr & !(sector_size - 1);
            if self.buffer_len == 0 || self.buffer_addr != sector {
                if !self.flush_buffer() {
                    return written;
                }
                if self
                    .flash
                    .read(sector as u32, &mut self.buffer[..F::ERASE_SIZE])
                    .is_err()
                {
                    return written;
                }
                self.buffer_addr = sector;
                self.buffer_len = F::ERASE_SIZE;
            }
            let in_sector = (addr - sector) as usize;
            let len = (F::ERASE_SIZE - in_sector).min(data.len() - written);
            self.buffer[in_sector..in_sector + len].copy_from_slice(&data[written..written + len]);
            written += len;
            if in_sector + len == F::ERASE_SIZE && !self.flush_buffer() {
                return written;
            }
        }
        written
    }

    fn read(&mut self, offset: u64, out: &mut [u8]) -> usize {
        if !self.flush_buffer() {
            return 0;
        }
        let len = out.len().min(self.capacity.saturating_sub(offset) as usize);
        if len == 0 {
            return 0;
        }
        if self.flash.read(offset as u32, &mut out[..len]).is_err() {
            return 0;
        }
        len
    }

    fn scratch_addr(&self) -> u32 {
        self.scratch.as_ptr() as u32
    }

    fn scratch_len(&self) -> usize {
        self.scratch.len()
    }

    fn scratch(&mut self) -> &mut [u8] {
        self.scratch
    }

    fn write_from_scratch(&mut self, offset: u64, scratch_off: usize, len: usize) -> usize {
        if scratch_off >= self.scratch.len() {
            return 0;
        }
        let total = len.min(self.scratch.len() - scratch_off);
        let mut tmp = [0u8; 256];
        let mut done = 0;
        while done < total {
            let n = (total - done).min(tmp.len());
            tmp[..n].copy_from_slice(&self.scratch[scratch_off + done..scratch_off + done + n]);
            let written = self.write(offset + done as u64, &tmp[..n]);
            done += written;
            if written != n {
                break;
            }
        }
        done
    }

    fn read_into_scratch(&mut self, offset: u64, scratch_off: usize, len: usize) -> usize {
        if scratch_off >= self.scratch.len() {
            return 0;
        }
        let total = len.min(self.scratch.len() - scratch_off);
        let mut tmp = [0u8; 256];
        let mut done = 0;
        while done < total {
            let n = (total - done).min(tmp.len());
            let read = self.read(offset + done as u64, &mut tmp[..n]);
            if read == 0 {
                break;
            }
            self.scratch[scratch_off + done..scratch_off + done + read]
                .copy_from_slice(&tmp[..read]);
            done += read;
            if read != n {
                break;
            }
        }
        done
    }

    fn partition_offset(&self, name: &str) -> Option<u64> {
        Partition::offset_of(self.partitions, name)
    }

    fn flush(&mut self) -> bool {
        self.flush_buffer()
    }
}

/// SPI NAND backed [`StorageBackend`].
pub struct NandUpgStorage<'a, N: RawNandFlash> {
    flash: N,
    partitions: &'a [Partition],
    page: [u8; MAX_NAND_PAGE_SIZE],
    page_base: u64,
    page_len: usize,
    scratch: &'a mut [u8],
    capacity: u64,
    erased_block: Option<u32>,
}

impl<'a, N: RawNandFlash> NandUpgStorage<'a, N> {
    /// Wrap `flash` with host `scratch` staging memory and `partitions`.
    pub fn new(flash: N, scratch: &'a mut [u8], partitions: &'a [Partition]) -> Self {
        let geom = flash.geometry();
        assert!(
            geom.page_size <= MAX_NAND_PAGE_SIZE && geom.page_size.is_power_of_two(),
            "NandUpgStorage unsupported NAND page size"
        );
        let capacity = (geom.capacity() as u64).max(Partition::table_capacity(partitions));
        Self {
            flash,
            partitions,
            page: [0xff; MAX_NAND_PAGE_SIZE],
            page_base: 0,
            page_len: 0,
            scratch,
            capacity,
            erased_block: None,
        }
    }

    fn page_size(&self) -> usize {
        self.flash.geometry().page_size
    }

    fn block_size_bytes(&self) -> u64 {
        self.flash.geometry().block_size() as u64
    }

    fn erase_checked(&mut self, block: u32) -> bool {
        if block as usize >= self.flash.geometry().block_count {
            return false;
        }
        match self.flash.is_bad_block(block as usize) {
            Ok(false) => {}
            Ok(true) => {
                log::error!("nand: Block {block} is marked bad, refusing to program it");
                return false;
            }
            Err(err) => {
                log::error!("nand: Bad-block query for block {block} failed: {err:?}");
                return false;
            }
        }
        match self.flash.erase_block(block as usize) {
            Ok(()) => true,
            Err(err) => {
                log::error!("nand: Erase of block {block} failed: {err:?}");
                false
            }
        }
    }

    fn flush_page(&mut self) -> bool {
        if self.page_len == 0 {
            return true;
        }
        let block_size = self.block_size_bytes();
        let page_size = self.page_size();
        let block = (self.page_base / block_size) as u32;
        let page = (self.page_base / page_size as u64) as usize;
        let mut ok = false;
        for _ in 0..2 {
            if self.erased_block != Some(block) {
                if !self.erase_checked(block) {
                    self.erased_block = None;
                    continue;
                }
                self.erased_block = Some(block);
            }
            match self.flash.write_page(page, &self.page[..page_size]) {
                Ok(()) => {
                    ok = true;
                    break;
                }
                Err(_) => {
                    self.erased_block = None;
                }
            }
        }
        if ok {
            self.page_len = 0;
        }
        ok
    }

    fn write_stream(&mut self, offset: u64, data: &[u8]) -> usize {
        let page_size = self.page_size();
        let page_size_u64 = page_size as u64;
        let mut done = 0usize;
        while done < data.len() {
            let address = offset + done as u64;
            let base = address & !(page_size_u64 - 1);
            if self.page_len > 0 && base != self.page_base && !self.flush_page() {
                return done;
            }
            if self.page_len == 0 {
                self.page_base = base;
                self.page[..page_size].fill(0xff);
            }
            let in_page = (address - base) as usize;
            let len = (page_size - in_page).min(data.len() - done);
            self.page[in_page..in_page + len].copy_from_slice(&data[done..done + len]);
            self.page_len = self.page_len.max(in_page + len);
            done += len;
        }
        done
    }
}

impl<N: RawNandFlash> StorageBackend for NandUpgStorage<'_, N> {
    fn media_type(&self) -> &'static str {
        "spi-nand"
    }

    fn block_size(&self) -> u32 {
        self.page_size() as u32
    }

    fn capacity(&self) -> u64 {
        self.capacity
    }

    fn boot_stage(&self) -> u8 {
        1
    }

    fn erase(&mut self, start: u64, size: u64) -> bool {
        if !self.flush_page() {
            return false;
        }
        self.erased_block = None;
        let end = start.saturating_add(size);
        if end > self.capacity {
            return false;
        }
        let block_size = self.block_size_bytes();
        let mut address = start & !(block_size - 1);
        while address < end {
            let block = (address / block_size) as u32;
            if !self.erase_checked(block) {
                return false;
            }
            address += block_size;
        }
        true
    }

    fn write(&mut self, offset: u64, data: &[u8]) -> usize {
        if data.is_empty() || offset + data.len() as u64 > self.capacity {
            return 0;
        }
        self.write_stream(offset, data)
    }

    fn read(&mut self, offset: u64, out: &mut [u8]) -> usize {
        if !self.flush_page() {
            return 0;
        }
        let len = out.len().min(self.capacity.saturating_sub(offset) as usize);
        if len == 0 {
            return 0;
        }
        match self.flash.read(offset as usize, &mut out[..len]) {
            Ok(()) => len,
            Err(_) => 0,
        }
    }

    fn scratch_addr(&self) -> u32 {
        self.scratch.as_ptr() as u32
    }

    fn scratch(&mut self) -> &mut [u8] {
        self.scratch
    }

    fn scratch_len(&self) -> usize {
        self.scratch.len()
    }

    fn write_from_scratch(&mut self, offset: u64, scratch_off: usize, len: usize) -> usize {
        if scratch_off >= self.scratch.len() {
            return 0;
        }
        let total = len.min(self.scratch.len() - scratch_off);
        let mut tmp = [0u8; 256];
        let mut done = 0;
        while done < total {
            let n = (total - done).min(tmp.len());
            tmp[..n].copy_from_slice(&self.scratch[scratch_off + done..scratch_off + done + n]);
            let written = self.write(offset + done as u64, &tmp[..n]);
            done += written;
            if written != n {
                break;
            }
        }
        done
    }

    fn read_into_scratch(&mut self, offset: u64, scratch_off: usize, len: usize) -> usize {
        if scratch_off >= self.scratch.len() {
            return 0;
        }
        let total = len.min(self.scratch.len() - scratch_off);
        let mut tmp = [0u8; 256];
        let mut done = 0;
        while done < total {
            let n = (total - done).min(tmp.len());
            let read = self.read(offset + done as u64, &mut tmp[..n]);
            if read == 0 {
                break;
            }
            self.scratch[scratch_off + done..scratch_off + done + read]
                .copy_from_slice(&tmp[..read]);
            done += read;
            if read != n {
                break;
            }
        }
        done
    }

    fn partition_offset(&self, name: &str) -> Option<u64> {
        Partition::offset_of(self.partitions, name)
    }

    fn flush(&mut self) -> bool {
        let ok = self.flush_page();
        if ok {
            self.erased_block = None;
        }
        ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spi_flash::NandGeometry;
    use embedded_storage::nor_flash::{ErrorType, NorFlash, NorFlashErrorKind, ReadNorFlash};

    struct MockNor {
        data: [u8; 128],
        fail_erase: bool,
        fail_write: bool,
    }

    impl MockNor {
        fn new() -> Self {
            Self {
                data: [0xff; 128],
                fail_erase: false,
                fail_write: false,
            }
        }
    }

    impl ErrorType for MockNor {
        type Error = NorFlashErrorKind;
    }

    impl ReadNorFlash for MockNor {
        const READ_SIZE: usize = 1;

        fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
            let start = offset as usize;
            let end = start + bytes.len();
            if end > self.data.len() {
                return Err(NorFlashErrorKind::OutOfBounds);
            }
            bytes.copy_from_slice(&self.data[start..end]);
            Ok(())
        }

        fn capacity(&self) -> usize {
            self.data.len()
        }
    }

    impl NorFlash for MockNor {
        const WRITE_SIZE: usize = 1;
        const ERASE_SIZE: usize = 64;

        fn erase(&mut self, from: u32, to: u32) -> Result<(), Self::Error> {
            if self.fail_erase {
                return Err(NorFlashErrorKind::Other);
            }
            let (from, to) = (from as usize, to as usize);
            if to > self.data.len() || from > to {
                return Err(NorFlashErrorKind::OutOfBounds);
            }
            self.data[from..to].fill(0xff);
            Ok(())
        }

        fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
            if self.fail_write {
                return Err(NorFlashErrorKind::Other);
            }
            let start = offset as usize;
            let end = start + bytes.len();
            if end > self.data.len() {
                return Err(NorFlashErrorKind::OutOfBounds);
            }
            self.data[start..end].copy_from_slice(bytes);
            Ok(())
        }
    }

    #[test]
    fn nor_flush_keeps_buffer_after_erase_failure() {
        let mut buffer = [0u8; 64];
        let mut scratch = [0u8; 64];
        let mut storage = NorUpgStorage::new(MockNor::new(), &mut buffer, &mut scratch, &[]);

        assert_eq!(storage.write(0, &[0x5a; 8]), 8);
        storage.flash.fail_erase = true;
        assert!(!storage.flush());
        assert_eq!(storage.buffer_len, 64);

        storage.flash.fail_erase = false;
        assert!(storage.flush());
        assert_eq!(&storage.flash.data[..8], &[0x5a; 8]);
        assert_eq!(storage.buffer_len, 0);
    }

    #[test]
    fn nor_flush_keeps_buffer_after_write_failure() {
        let mut buffer = [0u8; 64];
        let mut scratch = [0u8; 64];
        let mut storage = NorUpgStorage::new(MockNor::new(), &mut buffer, &mut scratch, &[]);

        assert_eq!(storage.write(0, &[0xa5; 8]), 8);
        storage.flash.fail_write = true;
        assert!(!storage.flush());
        assert_eq!(storage.buffer_len, 64);

        storage.flash.fail_write = false;
        assert!(storage.flush());
        assert_eq!(&storage.flash.data[..8], &[0xa5; 8]);
    }

    struct MockNand {
        data: [u8; 512],
        fail_write: bool,
    }

    impl MockNand {
        fn new() -> Self {
            Self {
                data: [0xff; 512],
                fail_write: false,
            }
        }
    }

    impl RawNandFlash for MockNand {
        type Error = ();

        fn geometry(&self) -> NandGeometry {
            NandGeometry {
                page_size: 64,
                oob_size: 8,
                pages_per_block: 4,
                block_count: 2,
            }
        }

        fn read_page(&mut self, page: usize, buf: &mut [u8]) -> Result<(), Self::Error> {
            let start = page * 64;
            buf.copy_from_slice(&self.data[start..start + buf.len()]);
            Ok(())
        }

        fn write_page(&mut self, page: usize, data: &[u8]) -> Result<(), Self::Error> {
            if self.fail_write {
                return Err(());
            }
            let start = page * 64;
            self.data[start..start + data.len()].copy_from_slice(data);
            Ok(())
        }

        fn erase_block(&mut self, block: usize) -> Result<(), Self::Error> {
            let start = block * 256;
            self.data[start..start + 256].fill(0xff);
            Ok(())
        }

        fn is_bad_block(&mut self, _block: usize) -> Result<bool, Self::Error> {
            Ok(false)
        }

        fn unprotect(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }

        fn read(&mut self, offset: usize, buf: &mut [u8]) -> Result<(), Self::Error> {
            let end = offset + buf.len();
            if end > self.data.len() {
                return Err(());
            }
            buf.copy_from_slice(&self.data[offset..end]);
            Ok(())
        }
    }

    #[test]
    fn nand_flush_keeps_page_after_write_failure() {
        let mut scratch = [0u8; 64];
        let mut storage = NandUpgStorage::new(MockNand::new(), &mut scratch, &[]);

        assert_eq!(storage.write(0, &[0x3c; 8]), 8);
        assert_eq!(storage.page_len, 8);
        storage.flash.fail_write = true;
        assert!(!storage.flush());
        assert_eq!(storage.page_len, 8);

        storage.flash.fail_write = false;
        assert!(storage.flush());
        assert_eq!(&storage.flash.data[..8], &[0x3c; 8]);
        assert_eq!(storage.page_len, 0);
    }
}
