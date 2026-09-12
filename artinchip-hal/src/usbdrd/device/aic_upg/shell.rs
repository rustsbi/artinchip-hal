//! Official `aiburnfw`-style storage shell (`RUN_SHELL_STR`).
//!
//! When the device reports `boot_stage = 2` ("Storage R/W Mode", `GET_HWINFO`
//! offset 40) the host tool programs flash with **shell commands** sent as UPG
//! `RUN_SHELL_STR` (0x05) instead of using the FWC channel. This is the path the
//! vendor RAM helper `bin/aiburnfw_d13x.bin` implements, and the one observed
//! writing flash on the wire.
//!
//! Command set (recovered from `aiburnfw_d13x.bin` and the USB capture):
//!
//! ```text
//! get_bootdevice <addr> <len>          fill a device table at <addr>
//! bdopen  <id>                         open storage device
//! bdclose <id>                         close storage device
//! bdcfg   <id> ...                     (accepted, no-op)
//! bderase <id> <start> <size>          erase
//! bdwrite <id> <offset> <len> <buf>    program flash from a RAM buffer
//! bdread  <id> <offset> <len> <buf>    read flash into a RAM buffer
//! efuse ... / aicupg ...               (accepted, no-op)
//! ```
//!
//! `<buf>` is the RAM window handed to the host by `GET_MEM_BUF`; the host fills
//! it with `WRITE` (0x02) and we move it to/from flash here.

use core::str::Split;

use log::error;

use super::storage::Storage;

/// Whether a shell line programs flash, as opposed to only looking at it.
///
/// The storage path's `get_bootdevice` / `bdopen` / `bdcfg` / `bdread` are the
/// host inspecting the device; only `bderase` / `bdwrite` change it.
pub(super) fn is_write(line: &str) -> bool {
    line.starts_with("bderase") || line.starts_with("bdwrite")
}

/// Run one command line. Returns `true` on success (reported as `UPG_RESP_OK`).
pub(super) fn run(storage: &mut dyn Storage, line: &str) -> bool {
    let mut args = line.split(' ');
    let command = args.next().unwrap_or("");
    match command {
        // Device lifecycle: our media is always open, so these are no-ops.
        // `reset` is not a no-op: the engine sees it and hands the board over to
        // its application (see `Engine::reset_requested`); answering OK here is
        // what lets it reply before that happens.
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
            storage.flush();
            written == len
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
            // `aiburnfw` answers "Invalid command: %s" and reports failure.
            error!("shell: unsupported command {line:?}");
            false
        }
    }
}

/// Read exactly `N` following arguments as numbers.
fn take<const N: usize>(args: &mut Split<'_, char>) -> Option<[u64; N]> {
    let mut out = [0u64; N];
    for slot in out.iter_mut() {
        *slot = number(args.next()?)?;
    }
    Some(out)
}

/// Parse a decimal or `0x`-prefixed hexadecimal number.
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

/// Translate a host RAM address into an offset/length inside our staging buffer.
fn window(storage: &dyn Storage, addr: u64, len: u64) -> Option<(usize, usize)> {
    let base = storage.scratch_addr() as u64;
    let size = storage.scratch_len() as u64;
    let off = addr.checked_sub(base)?;
    if off >= size {
        return None;
    }
    Some((off as usize, len.min(size - off) as usize))
}

/// Best-effort `get_bootdevice` reply.
///
/// The vendor helper returns a small storage-device table (one entry per
/// backend) which the host reads back with `READ`. Only the scalar header layout
/// could be identified from the wire capture; the referenced strings are placed
/// inside the same buffer here so that every pointer stays inside the RAM window
/// the host already knows about.
///
/// Header fields identified from the capture (offsets from the start of the
/// reply):
///
/// ```text
/// 0x00 u32 device count (1)
/// 0x08 u32 table entries (2)
/// 0x18 u32 erase size
/// 0x1C u32 block size
/// 0x20 u32 total size
/// ```
///
/// This is the one reply whose full layout is not reversed yet; see the notes in
/// `docs/PBP-BOOTLOADER-REPORT.md`.
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
