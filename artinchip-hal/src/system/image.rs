//! `"AIC "` container parsing and loader relocation.
//!
//! An AIC image is a 256-byte `"AIC "` header followed by a loader payload and
//! optional `pbp`/`private` resources, in one of two layouts:
//!
//! * **plain** — the payload sits directly after the header at `0x100`,
//! * **`with_ext`** — a PBP is inline and the loader is appended as a nested AIC
//!   image at `loader_ext_offset` (vendor `bootloader.aic`).
//!
//! `loader_load_address` is the **container base**: the payload sits one header
//! above it, hence the packer computes `min_vaddr - 0x100`.

use crate::spi_flash::RawNandFlash;

/// `"AIC "` header size.
pub const AIC_HEADER_SIZE: usize = 256;

const AIC_PAYLOAD_OFFSET: u32 = 0x100;

const FIELD_LOADER_LENGTH: usize = 0x14;
const FIELD_LOADER_LOAD_ADDRESS: usize = 0x18;
const FIELD_LOADER_ENTRY_POINT: usize = 0x1C;
const FIELD_LOADER_EXT_OFFSET: usize = 0x50;
/// Application name length header field (LE `u32`, `0` = unnamed); kept in sync
/// with `aicfwc`'s `aic.rs`.
const FIELD_APP_NAME_LEN: usize = 0x54;
const FIELD_APP_NAME: usize = 0x58;

/// Maximum application name length read back from a container header.
pub const APP_NAME_MAX: usize = 32;

const LOAD_CHUNK: usize = 4096;

/// AIC image error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// The backing source could not be read.
    Read,
    /// No `"AIC "` header at the requested offset.
    NotAic,
    /// The header carries no usable loader.
    NoLoader,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Read => write!(f, "image source read failed"),
            Self::NotAic => write!(f, "no AIC header at the requested offset"),
            Self::NoLoader => write!(f, "AIC header has no usable loader"),
        }
    }
}

impl core::error::Error for Error {}

/// Random-access byte source an image can be read from (SPI-NOR, SPI-NAND, RAM, ...).
pub trait ReadAt {
    /// Fill `buf` with `buf.len()` bytes starting at `offset`.
    fn read_at(&mut self, offset: u32, buf: &mut [u8]) -> Result<(), Error>;
}

impl<F> ReadAt for F
where
    F: embedded_storage::nor_flash::ReadNorFlash,
{
    fn read_at(&mut self, offset: u32, buf: &mut [u8]) -> Result<(), Error> {
        embedded_storage::nor_flash::ReadNorFlash::read(self, offset, buf).map_err(|_| Error::Read)
    }
}

/// [`ReadAt`] adapter over any [`RawNandFlash`] device.
pub struct NandRead<F>(pub F);

impl<F> ReadAt for NandRead<F>
where
    F: RawNandFlash,
{
    fn read_at(&mut self, offset: u32, buf: &mut [u8]) -> Result<(), Error> {
        self.0.read(offset as usize, buf).map_err(|_| Error::Read)
    }
}

/// [`ReadAt`] adapter over an initialized memory-mapped RAM/PSRAM/DRAM window.
pub struct Ram {
    _private: (),
}

impl Ram {
    /// Create a memory-mapped `ReadAt` reader.
    ///
    /// # Safety
    /// Every `[offset, offset + len)` range subsequently queried through [`ReadAt`]
    /// must reside in initialized, readable memory and not alias active `&mut` borrows.
    pub const unsafe fn new() -> Self {
        Self { _private: () }
    }
}

impl ReadAt for Ram {
    fn read_at(&mut self, offset: u32, buf: &mut [u8]) -> Result<(), Error> {
        unsafe {
            core::ptr::copy_nonoverlapping(offset as *const u8, buf.as_mut_ptr(), buf.len());
        }
        Ok(())
    }
}

/// A loader resolved from an AIC container.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Loader {
    pub src: u32,
    pub dst: u32,
    pub entry: u32,
    pub length: u32,
    /// Application name recorded in the header, NUL padded.
    pub name: [u8; APP_NAME_MAX],
    /// Length of [`Self::name`] in bytes (`0` = unnamed).
    pub name_len: u8,
}

impl Loader {
    /// Application name recorded in the container header, if it has one.
    pub fn name(&self) -> Option<&str> {
        let bytes = self.name.get(..self.name_len as usize)?;
        core::str::from_utf8(bytes)
            .ok()
            .filter(|name| !name.is_empty())
    }

    /// Return `true` when both `[dst, dst + length)` and `entry` fall inside `[base, base + size)`.
    pub const fn fits_window(&self, base: u32, size: u32) -> bool {
        let start = base as u64;
        let end = start + size as u64;
        let dst = self.dst as u64;
        let dst_end = dst + self.length as u64;
        let entry = self.entry as u64;
        dst >= start && dst_end <= end && entry >= start && entry < end
    }
}

/// Return `true` when `loader`'s destination range and entry lie inside `[base, base + size)`.
pub const fn loader_fits_window(loader: &Loader, base: u32, size: u32) -> bool {
    loader.fits_window(base, size)
}

fn read_u32(header: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(header[offset..offset + 4].try_into().unwrap())
}

fn read_header(src: &mut impl ReadAt, addr: u32) -> Result<[u8; AIC_HEADER_SIZE], Error> {
    let mut header = [0u8; AIC_HEADER_SIZE];
    src.read_at(addr, &mut header)?;
    if &header[..4] != b"AIC " {
        return Err(Error::NotAic);
    }
    Ok(header)
}

/// Resolve the AIC loader stored at `base` in `src`.
pub fn resolve(src: &mut impl ReadAt, base: u32) -> Result<Loader, Error> {
    let outer = read_header(src, base)?;
    let ext = read_u32(&outer, FIELD_LOADER_EXT_OFFSET);

    // A `with_ext` container nests a dedicated AIC image holding the loader; a
    // plain one embeds the payload directly after its own header.
    let (header, src) = if ext != 0 {
        let nested = base.checked_add(ext).ok_or(Error::NotAic)?;
        (read_header(src, nested)?, nested + AIC_PAYLOAD_OFFSET)
    } else {
        (outer, base + AIC_PAYLOAD_OFFSET)
    };

    let load_address = read_u32(&header, FIELD_LOADER_LOAD_ADDRESS);
    let length = read_u32(&header, FIELD_LOADER_LENGTH);
    let entry = read_u32(&header, FIELD_LOADER_ENTRY_POINT);
    let name_len = read_u32(&header, FIELD_APP_NAME_LEN).min(APP_NAME_MAX as u32) as usize;
    let mut name = [0u8; APP_NAME_MAX];
    name[..name_len].copy_from_slice(&header[FIELD_APP_NAME..FIELD_APP_NAME + name_len]);

    // Reject a header that cannot describe a runnable payload, including one
    // whose copy would wrap the address space.
    let dst = load_address.checked_add(AIC_HEADER_SIZE as u32);
    let end = dst.and_then(|dst| dst.checked_add(length));
    let (Some(dst), Some(_)) = (dst, end) else {
        return Err(Error::NoLoader);
    };
    if length == 0 || load_address == 0 || entry == 0 {
        return Err(Error::NoLoader);
    }

    Ok(Loader {
        src,
        dst,
        entry,
        length,
        name,
        name_len: name_len as u8,
    })
}

/// Copy a loader payload to [`Loader::dst`].
///
/// The destination must be writable RAM. The D-cache is **not** flushed here;
/// callers that run the payload must flush it first.
pub fn load(src: &mut impl ReadAt, loader: &Loader) -> Result<(), Error> {
    let total = loader.length as usize;
    let mut done = 0usize;
    while done < total {
        let len = (total - done).min(LOAD_CHUNK);
        // Safety: `dst` was validated against overflow in `resolve`; the caller
        // is responsible for it being writable RAM.
        let buf = unsafe {
            core::slice::from_raw_parts_mut((loader.dst as usize + done) as *mut u8, len)
        };
        src.read_at(loader.src + done as u32, buf)?;
        done += len;
    }
    Ok(())
}
