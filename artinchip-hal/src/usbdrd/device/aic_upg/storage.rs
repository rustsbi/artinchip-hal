//! Upgrade storage abstraction.
//!
//! The AIC UPG protocol never talks to flash directly: image components are
//! addressed by partition name (`fwc_meta.partition`) and erased/programmed by
//! the device firmware. This trait is the seam between the protocol engine and
//! the real media driver.
//!
//! * [`NorStorage`] programs a real SPI-NOR through a [`NorFlash`] adapter.
//! * [`RamStorage`] is a RAM sink used to validate the host handshake without
//!   touching flash.
#![allow(dead_code)]

/// SPI flash sector size (erase granularity).
pub const SECTOR_SIZE: usize = 4096;

/// One entry of the on-device partition table.
#[derive(Clone, Copy, Debug)]
pub struct Partition {
    /// Partition name as used in `fwc_meta.partition`.
    pub name: &'static str,
    /// Byte offset in the flash.
    pub offset: u64,
    /// Partition size in bytes.
    pub size: u64,
}

/// Failure of one [`NorFlash`] media operation.
///
/// The protocol only distinguishes "worked" from "did not", so there is nothing
/// to report beyond the failure itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NorError;

/// Minimal SPI-NOR operations the storage layer needs.
pub trait NorFlash {
    /// Read `buf.len()` bytes at `addr`.
    fn read(&mut self, addr: u32, buf: &mut [u8]) -> Result<(), NorError>;
    /// Erase the 4 KiB sector containing `addr`.
    fn erase_sector(&mut self, addr: u32) -> Result<(), NorError>;
    /// Erase + program a sector-aligned, full-sector buffer.
    fn program_sector(&mut self, addr: u32, data: &[u8]) -> Result<(), NorError>;
}

/// Media behind the upgrade protocol.
pub trait Storage {
    /// Media type string reported to the host, e.g. `"spi-nor"`.
    fn media_type(&self) -> &'static str;

    /// Media device id (controller id).
    fn media_dev_id(&self) -> u32 {
        0
    }

    /// Erase block size in bytes.
    fn block_size(&self) -> u32;

    /// Total capacity in bytes.
    fn capacity(&self) -> u64;

    /// Erase `size` bytes starting at `start` (media offset).
    fn erase(&mut self, start: u64, size: u64) -> bool;

    /// Program `data` at `offset` (media offset); returns bytes written.
    fn write(&mut self, offset: u64, data: &[u8]) -> usize;

    /// Read `out.len()` bytes from `offset` (media offset).
    fn read(&mut self, offset: u64, out: &mut [u8]) -> usize;

    /// Address of a host-visible staging buffer (used by `GET_MEM_BUF`).
    fn scratch_addr(&self) -> u32;

    /// Length of the staging buffer.
    fn scratch_len(&self) -> usize;

    /// Staging buffer used by host `WRITE` commands.
    fn scratch(&mut self) -> &mut [u8];

    /// Program `len` bytes from the staging buffer (at `scratch_off`) to the
    /// media at `offset`; returns bytes written.
    ///
    /// Exists because the staging buffer is borrowed from `self`, so the caller
    /// cannot hold it and `&mut self` at the same time.
    fn write_from_scratch(&mut self, offset: u64, scratch_off: usize, len: usize) -> usize;

    /// Read `len` bytes from the media into the staging buffer at `scratch_off`.
    fn read_into_scratch(&mut self, offset: u64, scratch_off: usize, len: usize) -> usize;

    /// Flash offset of a named partition (`fwc_meta.partition`); the first name
    /// of a `;`-separated list is used.
    fn partition_offset(&self, name: &str) -> Option<u64>;

    /// Flush any buffered writes to the media.
    fn flush(&mut self) {}

    /// Boot stage reported in `GET_HWINFO` offset 40.
    ///
    /// The host tool decodes this byte as: `0` = Boot ROM, `1` = Bootloader
    /// (FWC channel), `2` = Storage R/W (shell `bd*` channel). The storage path
    /// is the one the vendor RAM helper `aiburnfw_*.bin` reports and the one
    /// proven to program flash on this board.
    fn boot_stage(&self) -> u8 {
        2
    }
}

/// SPI-NOR backed [`Storage`].
///
/// Writes are buffered per 4 KiB sector (read-modify-write) because the NOR
/// driver erases whole sectors before programming; the caller must call
/// [`Storage::flush`] at component boundaries.
pub struct NorStorage<F: NorFlash> {
    flash: F,
    partitions: &'static [Partition],
    buffer: &'static mut [u8],
    buffer_addr: u64,
    buffer_len: usize,
    scratch: &'static mut [u8],
    capacity: u64,
}

impl<F: NorFlash> NorStorage<F> {
    /// Create a storage layer over `flash`.
    ///
    /// `buffer` must be at least [`SECTOR_SIZE`] bytes (sector assembly);
    /// `scratch` is the host-visible staging buffer.
    pub fn new(
        flash: F,
        buffer: &'static mut [u8],
        scratch: &'static mut [u8],
        partitions: &'static [Partition],
    ) -> Self {
        let capacity = partitions
            .iter()
            .map(|p| p.offset + p.size)
            .max()
            .unwrap_or(0);
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

    fn flush_buffer(&mut self) -> bool {
        if self.buffer_len == 0 {
            return true;
        }
        let result = self
            .flash
            .program_sector(self.buffer_addr as u32, &self.buffer[..SECTOR_SIZE]);
        self.buffer_len = 0;
        result.is_ok()
    }
}

impl<F: NorFlash> Storage for NorStorage<F> {
    fn media_type(&self) -> &'static str {
        "spi-nor"
    }

    fn block_size(&self) -> u32 {
        SECTOR_SIZE as u32
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
        let mut addr = start & !(SECTOR_SIZE as u64 - 1);
        while addr < end {
            if self.flash.erase_sector(addr as u32).is_err() {
                return false;
            }
            addr += SECTOR_SIZE as u64;
        }
        true
    }

    /// Read-modify-write per sector so sequential small writes are safe even
    /// though the NOR driver erases whole sectors.
    fn write(&mut self, offset: u64, data: &[u8]) -> usize {
        if data.is_empty() {
            return 0;
        }
        if offset + data.len() as u64 > self.capacity {
            return 0;
        }
        let mut written = 0usize;
        while written < data.len() {
            let addr = offset + written as u64;
            let sector = addr & !(SECTOR_SIZE as u64 - 1);
            if self.buffer_len == 0 || self.buffer_addr != sector {
                if !self.flush_buffer() {
                    return written;
                }
                if self
                    .flash
                    .read(sector as u32, &mut self.buffer[..SECTOR_SIZE])
                    .is_err()
                {
                    return written;
                }
                self.buffer_addr = sector;
                self.buffer_len = SECTOR_SIZE;
            }
            let in_sector = (addr - sector) as usize;
            let len = (SECTOR_SIZE - in_sector).min(data.len() - written);
            self.buffer[in_sector..in_sector + len].copy_from_slice(&data[written..written + len]);
            written += len;
            if in_sector + len == SECTOR_SIZE && !self.flush_buffer() {
                return written;
            }
        }
        written
    }

    fn read(&mut self, offset: u64, out: &mut [u8]) -> usize {
        if !self.flush_buffer() {
            return 0;
        }
        let len = out
            .len()
            .min((self.capacity.saturating_sub(offset)) as usize);
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
        }
        done
    }

    fn partition_offset(&self, name: &str) -> Option<u64> {
        let first = name.split(';').next().unwrap_or(name).trim();
        self.partitions
            .iter()
            .find(|partition| partition.name == first)
            .map(|partition| partition.offset)
    }

    fn flush(&mut self) {
        let _ = self.flush_buffer();
    }
}

/// A RAM-backed [`Storage`] used to validate host communication only (the data
/// is not retained across a reboot).
pub struct RamStorage {
    data: &'static mut [u8],
    scratch: &'static mut [u8],
    partitions: &'static [Partition],
    block_size: u32,
}

impl RamStorage {
    /// # Safety
    /// `data`/`scratch` must be uniquely owned for the program lifetime.
    pub unsafe fn new(
        data: &'static mut [u8],
        scratch: &'static mut [u8],
        partitions: &'static [Partition],
    ) -> Self {
        Self {
            data,
            scratch,
            partitions,
            block_size: SECTOR_SIZE as u32,
        }
    }
}

impl Storage for RamStorage {
    fn media_type(&self) -> &'static str {
        "spi-nor"
    }

    fn block_size(&self) -> u32 {
        self.block_size
    }

    fn capacity(&self) -> u64 {
        self.data.len() as u64
    }

    fn erase(&mut self, start: u64, size: u64) -> bool {
        let start = start as usize;
        let end = start.saturating_add(size as usize);
        if end > self.data.len() {
            return false;
        }
        self.data[start..end].fill(0xFF);
        true
    }

    fn write(&mut self, offset: u64, data: &[u8]) -> usize {
        let offset = offset as usize;
        if offset >= self.data.len() {
            return 0;
        }
        let len = data.len().min(self.data.len() - offset);
        self.data[offset..offset + len].copy_from_slice(&data[..len]);
        len
    }

    fn read(&mut self, offset: u64, out: &mut [u8]) -> usize {
        let offset = offset as usize;
        if offset >= self.data.len() {
            return 0;
        }
        let len = out.len().min(self.data.len() - offset);
        out[..len].copy_from_slice(&self.data[offset..offset + len]);
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
        }
        done
    }

    fn partition_offset(&self, name: &str) -> Option<u64> {
        let first = name.split(';').next().unwrap_or(name).trim();
        self.partitions
            .iter()
            .find(|partition| partition.name == first)
            .map(|partition| partition.offset)
    }
}
