//! `aiburnfw`-compatible storage shell (`RUN_SHELL_STR`, opcode `0x05`).
//!
//! When `GET_HWINFO` reports `boot_stage = 2` ("Storage R/W Mode"), the host tool
//! drives flash operations via ASCII `bd*` shell commands instead of the FWC channel:
//!
//! ```text
//! get_bootdevice <addr> <len>          fill a device table at <addr>
//! bdopen  <id>                         open storage device
//! bdclose <id>                         close storage device
//! bdcfg   <id> ...                     accepted, no-op
//! bderase <id> <start> <size>          erase byte range
//! bdwrite <id> <offset> <len> <buf>    program flash from staging buffer
//! bdread  <id> <offset> <len> <buf>    read flash into staging buffer
//! efuse ... / aicupg ... / reset       accepted lifecycle commands
//! ```

use core::str::Split;

use log::error;

use super::storage::StorageBackend as Storage;

/// Return whether a shell line mutates flash (`bderase` / `bdwrite`).
pub(super) fn is_write(line: &str) -> bool {
    line.starts_with("bderase") || line.starts_with("bdwrite")
}

/// Execute one shell command line against `storage`.
pub(super) fn run(storage: &mut dyn Storage, line: &str) -> bool {
    let mut args = line.split(' ');
    let command = args.next().unwrap_or("");
    match command {
        "bdopen" | "bdclose" | "bdcfg" | "efuse" | "aicupg" | "reset" => true,

        "bderase" => {
            let (_id, [start, size]) = match (args.next(), take::<2>(&mut args)) {
                (Some(id), Some(nums)) => (id, nums),
                _ => return false,
            };
            storage.erase(start, size)
        }

        "bdwrite" => {
            let (_id, [offset, len, addr]) = match (args.next(), take::<3>(&mut args)) {
                (Some(id), Some(nums)) => (id, nums),
                _ => return false,
            };
            let Some((scratch_off, len)) = window(storage, addr, len) else {
                error!("shell: bdwrite bad buffer {addr:#x}");
                return false;
            };
            let written = storage.write_from_scratch(offset, scratch_off, len);
            written == len && storage.flush()
        }

        "bdread" => {
            let (_id, [offset, len, addr]) = match (args.next(), take::<3>(&mut args)) {
                (Some(id), Some(nums)) => (id, nums),
                _ => return false,
            };
            let Some((scratch_off, len)) = window(storage, addr, len) else {
                error!("shell: bdread bad buffer {addr:#x}");
                return false;
            };
            let read = storage.read_into_scratch(offset, scratch_off, len);
            read == len
        }

        "get_bootdevice" => {
            let Some([addr, len]) = take::<2>(&mut args) else {
                return false;
            };
            let Some((scratch_off, len)) = window(storage, addr, len) else {
                error!("shell: get_bootdevice bad buffer {addr:#x}");
                return false;
            };
            fill_bootdevice(storage, scratch_off, len);
            true
        }

        _ => {
            error!("shell: unsupported command {line:?}");
            false
        }
    }
}

fn take<const N: usize>(args: &mut Split<'_, char>) -> Option<[u64; N]> {
    let mut out = [0u64; N];
    for slot in out.iter_mut() {
        *slot = number(args.next()?)?;
    }
    Some(out)
}

fn number(token: &str) -> Option<u64> {
    let token = token.trim();
    match token
        .strip_prefix("0x")
        .or_else(|| token.strip_prefix("0X"))
    {
        Some(hex) => u64::from_str_radix(hex, 16).ok(),
        None => token.parse().ok(),
    }
}

fn window(storage: &dyn Storage, addr: u64, len: u64) -> Option<(usize, usize)> {
    let base = storage.scratch_addr() as u64;
    let size = storage.scratch_len() as u64;
    let off = addr.checked_sub(base)?;
    if off >= size {
        return None;
    }
    Some((off as usize, len.min(size - off) as usize))
}

/// Populate the `get_bootdevice` storage-descriptor reply inside the staging buffer.
fn fill_bootdevice(storage: &mut dyn Storage, scratch_off: usize, len: usize) {
    let base = storage.scratch_addr();
    let block = storage.block_size();
    let capacity = storage.capacity();

    let scratch = storage.scratch();
    let Some(buf) = scratch.get_mut(scratch_off..scratch_off + len) else {
        return;
    };
    buf.fill(0);

    put_u32(buf, 0x00, 1);
    put_u32(buf, 0x08, 2);
    put_u32(buf, 0x18, block);
    put_u32(buf, 0x1C, block);
    put_u32(buf, 0x20, capacity as u32);

    const NAME_OFF: usize = 0x40;
    const NAME: &[u8] = b"artinchip spi-nor\0";
    if buf.len() >= NAME_OFF + NAME.len() {
        buf[NAME_OFF..NAME_OFF + NAME.len()].copy_from_slice(NAME);
        let ptr = base
            .wrapping_add(scratch_off as u32)
            .wrapping_add(NAME_OFF as u32);
        put_u32(buf, 0x24, ptr);
    }
}

fn put_u32(buf: &mut [u8], offset: usize, value: u32) {
    if let Some(slot) = buf.get_mut(offset..offset + 4) {
        slot.copy_from_slice(&value.to_le_bytes());
    }
}
