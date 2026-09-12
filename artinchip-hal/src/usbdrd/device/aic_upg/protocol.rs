//! AIC USB-UPG device-side protocol engine.
//!
//! Two layers, both reversed from the vendor firmware and the USB capture:
//!
//! * transport: 31-byte `"USBC"` CBW (only `WRITE=1` / `READ=2`) + 13-byte
//!   `"USBS"` CSW; the host `WRITE`s `UPGC` frames and `READ`s `UPGR` replies,
//! * command: 16-byte `UPGC`/`UPGR` header + payload.
//!
//! The engine is transport agnostic: the USB loop feeds it the data phase of
//! each transport `WRITE` and drains pending response bytes for each transport
//! `READ`.
//!
//! The command set is exposed in full even where the bootloader does not
//! exercise it yet.
#![allow(dead_code)]

use heapless::Vec;
use log::{debug, error};

use super::storage::Storage;

/// `"UPGC"` request magic.
pub const UPG_CMD_MAGIC: u32 = 0x4347_5055;
/// `"UPGR"` response magic.
pub const UPG_RESP_MAGIC: u32 = 0x5247_5055;
/// `"USBC"` CBW signature.
pub const CBW_MAGIC: u32 = 0x4342_5355;
/// `"USBS"` CSW signature.
pub const CSW_MAGIC: u32 = 0x5342_5355;
/// CBW size.
pub const CBW_SIZE: usize = 31;
/// CSW size.
pub const CSW_SIZE: usize = 13;
/// UPG header size.
pub const UPG_HEADER_SIZE: usize = 16;

/// Transport command: host → device data.
pub const TRANSPORT_WRITE: u8 = 0x01;
/// Transport command: device → host data.
pub const TRANSPORT_READ: u8 = 0x02;

/// Response status: success.
pub const RESP_OK: u8 = 0;
/// Response status: failure.
pub const RESP_FAIL: u8 = 1;

/// Maximum buffered `UPGC` frame (header + payload).
pub const REQ_MAX: usize = 16 + 8192;
/// Maximum buffered `UPGR` response (header + payload).
pub const RESP_MAX: usize = 16 + 4096;

/// Size of the `struct hwinfo` payload (`basic_cmd.c`).
pub const HWINFO_SIZE: usize = 108;

/// Build stamp the device reports in `hwinfo[8..]`.
///
/// The vendor source leaves this zeroed, but the real device fills it in and
/// AiBurn branches on it: a `"2023"` / `"-50-"` / `"05"` pattern selects an
/// alternate command set, and the captured burn-mode device reports this value.
const HWINFO_BUILD_STAMP: &[u8] = b"24-06-24 10:07";

/// Protocol version reported in `hwinfo[72..76]`.
///
/// AiBurn reads `hwinfo[72..76] & 0xFFFF_0000` and only continues for a version
/// it knows (`0x1602`, `0x1603`, `0x1605`, `0x1609`); anything else is reported
/// as `Core id invalid`. The captured vendor device reports `0x1603_0003`.
pub const UPG_PROTO_VERSION: u32 = 0x1603_0003;

/// `GET_HWINFO`.
pub const CMD_GET_HWINFO: u8 = 0x00;
/// `GET_TRACEINFO`.
pub const CMD_GET_TRACEINFO: u8 = 0x01;
/// `WRITE` to memory.
pub const CMD_WRITE: u8 = 0x02;
/// `READ` from memory.
pub const CMD_READ: u8 = 0x03;
/// `EXEC`.
pub const CMD_EXEC: u8 = 0x04;
/// `RUN_SHELL_STR`.
pub const CMD_RUN_SHELL_STR: u8 = 0x05;
/// `GET_MEM_BUF`.
pub const CMD_GET_MEM_BUF: u8 = 0x08;
/// `FREE_MEM_BUF`.
pub const CMD_FREE_MEM_BUF: u8 = 0x09;
/// `SET_UPG_CFG`.
pub const CMD_SET_UPG_CFG: u8 = 0x0A;
/// `SET_UPG_END`.
pub const CMD_SET_UPG_END: u8 = 0x0B;
/// `GET_LOG_SIZE`.
pub const CMD_GET_LOG_SIZE: u8 = 0x0C;
/// `GET_LOG_DATA`.
pub const CMD_GET_LOG_DATA: u8 = 0x0D;
/// `SET_FWC_META`.
pub const CMD_SET_FWC_META: u8 = 0x10;
/// `GET_BLOCK_SIZE`.
pub const CMD_GET_BLOCK_SIZE: u8 = 0x11;
/// `SEND_FWC_DATA`.
pub const CMD_SEND_FWC_DATA: u8 = 0x12;
/// `GET_FWC_CRC`.
pub const CMD_GET_FWC_CRC: u8 = 0x13;
/// `GET_FWC_BURN_RESULT`.
pub const CMD_GET_FWC_BURN_RESULT: u8 = 0x14;
/// `GET_FWC_RUN_RESULT`.
pub const CMD_GET_FWC_RUN_RESULT: u8 = 0x15;
/// `GET_STORAGE_MEDIA`.
pub const CMD_GET_STORAGE_MEDIA: u8 = 0x16;
/// `GET_PARTITION_TABLE`.
pub const CMD_GET_PARTITION_TABLE: u8 = 0x17;
/// `READ_FWC_DATA`.
pub const CMD_READ_FWC_DATA: u8 = 0x18;
/// `SET_UART_ARGS`.
pub const CMD_SET_UART_ARGS: u8 = 0x19;
/// `GET_STORAGE_GEOME`.
pub const CMD_GET_STORAGE_GEOME: u8 = 0x1A;
/// `ERASE_STORAGE`.
pub const CMD_ERASE_STORAGE: u8 = 0x1B;

/// Parsed transport command block wrapper.
#[derive(Clone, Copy, Debug, Default)]
pub struct Cbw {
    /// `"USBC"`.
    pub signature: u32,
    /// Echoed in the CSW.
    pub tag: u32,
    /// Data phase length.
    pub data_len: u32,
    /// `0x80` device → host, `0x00` host → device.
    pub flags: u8,
    /// CB length.
    pub cb_length: u8,
    /// Transport command (`TRANSPORT_WRITE`/`TRANSPORT_READ`).
    pub command: u8,
}

impl Cbw {
    /// Parse a 31-byte CBW.
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < CBW_SIZE {
            return None;
        }
        let signature = u32::from_le_bytes(bytes[0..4].try_into().ok()?);
        let cbw = Self {
            signature,
            tag: u32::from_le_bytes(bytes[4..8].try_into().ok()?),
            data_len: u32::from_le_bytes(bytes[8..12].try_into().ok()?),
            flags: bytes[12],
            cb_length: bytes[14],
            command: bytes[15],
        };
        (cbw.signature == CBW_MAGIC).then_some(cbw)
    }
}

/// Command status wrapper.
#[derive(Clone, Copy, Debug, Default)]
pub struct Csw {
    /// `"USBS"`.
    pub signature: u32,
    /// CBW tag.
    pub tag: u32,
    /// Untransferred bytes.
    pub residue: u32,
    /// `0` = passed, non-zero = failed.
    pub status: u8,
}

impl Csw {
    /// Serialize to the 13-byte wire format.
    pub fn to_bytes(self) -> [u8; CSW_SIZE] {
        let mut bytes = [0u8; CSW_SIZE];
        bytes[0..4].copy_from_slice(&self.signature.to_le_bytes());
        bytes[4..8].copy_from_slice(&self.tag.to_le_bytes());
        bytes[8..12].copy_from_slice(&self.residue.to_le_bytes());
        bytes[12] = self.status;
        bytes
    }

    /// Build a status reply for `tag`.
    pub fn reply(tag: u32, status: u8, residue: u32) -> Self {
        Self {
            signature: CSW_MAGIC,
            tag,
            residue,
            status,
        }
    }
}

/// Per component firmware state.
#[derive(Clone, Copy, Debug)]
struct FwcState {
    meta: [u8; 512],
    meta_valid: bool,
    /// Flash offset the component is programmed at (`None` for updater/run
    /// components that are not stored in flash).
    target: Option<u64>,
    written: u64,
    crc: u32,
    burn_result: i32,
    run_result: i32,
}

impl Default for FwcState {
    fn default() -> Self {
        Self {
            meta: [0u8; 512],
            meta_valid: false,
            target: None,
            written: 0,
            crc: 0,
            burn_result: 0,
            run_result: 0,
        }
    }
}

/// Protocol engine over a [`Storage`] sink.
pub struct Engine<'a> {
    storage: &'a mut dyn Storage,
    request: Vec<u8, REQ_MAX>,
    request_total: usize,
    response: Vec<u8, RESP_MAX>,
    response_pos: usize,
    fwc: FwcState,
    cfg_mode: u8,
    partition_table: &'static str,
    last_status: u8,
    /// Set once the host does something that changes state, as opposed to only
    /// looking at the device.
    ///
    /// The async loop uses this to decide whether it may still time out: AiBurn
    /// polls `GET_HWINFO` / `GET_PARTITION_TABLE` as soon as it is connected, and
    /// treating that as "a host is driving an upgrade" would pin the loader in
    /// upgrade mode for as long as the tool is open, so the application could
    /// never boot.
    host_upgrading: bool,
    /// Set when the host asked the device to hand over to its application
    /// (`reset`), which is how AiBurn ends a successful burn.
    reset_requested: bool,
    /// Bytes still awaited for the in-flight streamed `SEND_FWC_DATA`.
    ///
    /// That command's 16-byte header announces the *whole* component length but
    /// the payload follows as a series of header-less transport writes, and the
    /// host reads exactly one response afterwards. Buffering it under the normal
    /// "header + `data_length`" rule would need a ~64 KiB buffer and would never
    /// match the wire framing, so it is consumed incrementally instead.
    stream_remaining: u64,
}

impl<'a> Engine<'a> {
    /// Create an engine on top of `storage`.
    pub fn new(storage: &'a mut dyn Storage, partition_table: &'static str) -> Self {
        Self {
            storage,
            request: Vec::new(),
            request_total: 0,
            response: Vec::new(),
            response_pos: 0,
            fwc: FwcState::default(),
            cfg_mode: 0,
            partition_table,
            last_status: RESP_OK,
            host_upgrading: false,
            reset_requested: false,
            stream_remaining: 0,
        }
    }

    /// Whether the host has started an upgrade, as opposed to merely querying the
    /// device. See [`Self::host_upgrading`].
    pub fn host_upgrading(&self) -> bool {
        self.host_upgrading
    }

    /// Whether the host asked us to hand the board over to its application.
    pub fn reset_requested(&self) -> bool {
        self.reset_requested
    }

    /// Feed the data phase of a transport `WRITE`.
    ///
    /// Returns the CSW status (`RESP_OK`/`RESP_FAIL`).
    pub fn transport_write(&mut self, data: &[u8]) -> u8 {
        self.last_status = RESP_OK;
        let mut offset = 0;
        while offset < data.len() {
            // A streamed `SEND_FWC_DATA` owns every following byte until the
            // announced length has arrived.
            if self.stream_remaining != 0 {
                let take = (self.stream_remaining as usize).min(data.len() - offset);
                let status = self.write_fwc_payload(&data[offset..offset + take]);
                offset += take;
                if status != RESP_OK {
                    self.stream_remaining = 0;
                    self.last_status = status;
                    return status;
                }
                continue;
            }

            if self.request_total == 0 {
                // Collect the 16-byte UPGC header.
                let want = UPG_HEADER_SIZE - self.request.len();
                let take = want.min(data.len() - offset);
                append(&mut self.request, &data[offset..offset + take]);
                offset += take;
                if self.request.len() < UPG_HEADER_SIZE {
                    break;
                }
                match parse_header(&self.request) {
                    Some((command, data_length, _)) => {
                        if command == CMD_SEND_FWC_DATA {
                            // Not a self-contained frame: switch to streaming and
                            // let the remainder of this write be consumed as
                            // payload.
                            self.reset_request();
                            let status = self.begin_fwc_stream(data_length);
                            if status != RESP_OK {
                                self.last_status = status;
                                return status;
                            }
                            continue;
                        }
                        self.request_total = UPG_HEADER_SIZE + data_length as usize;
                        if self.request_total > REQ_MAX {
                            error!(
                                "aic_upg: UPGC frame too long ({} > {REQ_MAX})",
                                self.request_total
                            );
                            self.fail();
                            self.reset_request();
                            return self.last_status;
                        }
                    }
                    None => {
                        // A packet that does not carry a command header is the
                        // continuation of a streaming command, but no streamed
                        // command is in flight (the payload case is handled
                        // above), so the host and device are out of sync.
                        error!(
                            "aic_upg: payload continuation +{} without known frame",
                            data.len()
                        );
                        self.fail();
                        self.reset_request();
                        return self.last_status;
                    }
                }
            } else if self.request.len() < self.request_total {
                let want = self.request_total - self.request.len();
                let take = want.min(data.len() - offset);
                append(&mut self.request, &data[offset..offset + take]);
                offset += take;
            }

            // The frame may already be complete here: most commands
            // (`GET_HWINFO`, `GET_BLOCK_SIZE`, `GET_FWC_CRC`, `SET_UPG_END`...)
            // are a bare 16-byte header with no payload, and executing them only
            // inside the payload branch would never happen for those.
            if self.request.len() == self.request_total {
                let frame = core::mem::take(&mut self.request);
                let status = self.execute(&frame);
                self.reset_request();
                if status != RESP_OK {
                    self.last_status = status;
                    return status;
                }
            }
        }
        self.last_status
    }

    /// Drain pending response bytes for a transport `READ`.
    pub fn transport_read(&mut self, out: &mut [u8]) -> usize {
        let remaining = self.response.len().saturating_sub(self.response_pos);
        let len = remaining.min(out.len());
        out[..len].copy_from_slice(&self.response[self.response_pos..self.response_pos + len]);
        self.response_pos += len;
        len
    }

    /// Whether unread response bytes are pending.
    pub fn has_response(&self) -> bool {
        self.response_pos < self.response.len()
    }

    /// Status reported by the last [`Self::transport_write`].
    pub fn last_status(&self) -> u8 {
        self.last_status
    }

    /// Read access to the underlying storage.
    pub fn storage(&self) -> &dyn Storage {
        self.storage
    }

    /// Drop any in-flight request/response and firmware state.
    ///
    /// Called after a bus reset: the controller aborts queued transfers, so a
    /// partially received frame must not be completed with stale bytes.
    pub fn reset(&mut self) {
        self.reset_request();
        self.response.clear();
        self.response_pos = 0;
        self.fwc = FwcState::default();
        self.cfg_mode = 0;
        self.last_status = RESP_OK;
        self.stream_remaining = 0;
    }

    fn reset_request(&mut self) {
        self.request.clear();
        self.request_total = 0;
    }

    fn fail(&mut self) {
        self.last_status = RESP_FAIL;
    }

    fn begin_response(&mut self, command: u8) {
        self.response.clear();
        self.response_pos = 0;
        push_header(&mut self.response, UPG_RESP_MAGIC, command, RESP_OK, 0);
    }

    fn finish_response(&mut self, data: &[u8]) -> u8 {
        if self.response.len() + data.len() > RESP_MAX {
            self.response.clear();
            self.response_pos = 0;
            push_header(&mut self.response, 0, RESP_FAIL, 0, 0);
            return RESP_FAIL;
        }
        let data_len = data.len() as u32;
        append(&mut self.response, data);
        // Patch the data length + checksum into the response header.
        let command = self.response[6];
        let status = self.response[7];
        let header = build_header(UPG_RESP_MAGIC, command, status, data_len);
        self.response[..UPG_HEADER_SIZE].copy_from_slice(&header);
        RESP_OK
    }

    fn fail_response(&mut self, command: u8) -> u8 {
        self.response.clear();
        self.response_pos = 0;
        push_header(&mut self.response, UPG_RESP_MAGIC, command, RESP_FAIL, 0);
        RESP_FAIL
    }

    /// Begin a streamed `SEND_FWC_DATA`.
    ///
    /// The header carries the *whole* component length and nothing is answered
    /// until the last byte arrives, matching `fwc_cmd.c`:
    ///
    /// ```text
    /// -> [CMD HEADER] -> [DATA PKT] -> [DATA PKT] ... <- [RESP HEADER]
    /// ```
    ///
    /// Answering per packet would desynchronise the host (it only reads one
    /// response, from `aicupg_cmd_send_fwc_data_final`), and buffering the whole
    /// component as a frame would need a ~64 KiB buffer for no benefit.
    fn begin_fwc_stream(&mut self, total: u32) -> u8 {
        if !self.fwc.meta_valid {
            return self.fail_response(CMD_SEND_FWC_DATA);
        }
        self.fwc.written = 0;
        self.fwc.crc = 0;
        self.fwc.burn_result = 0;
        self.fwc.run_result = 0;
        self.stream_remaining = total as u64;
        if self.stream_remaining == 0 {
            self.finish_fwc_stream();
        }
        RESP_OK
    }

    /// Consume one payload packet of the in-flight `SEND_FWC_DATA`.
    ///
    /// `written` tracks the *stream*, so a media error still advances it and
    /// keeps consuming: aborting mid-stream would leave the host sending packets
    /// the engine no longer recognises as payload. The failure is reported once,
    /// in the single response at the end.
    fn write_fwc_payload(&mut self, data: &[u8]) -> u8 {
        if let Some(base) = self.fwc.target {
            let written = self.storage.write(base + self.fwc.written, data);
            if written != data.len() && self.fwc.burn_result == 0 {
                error!(
                    "aic_upg: SEND_FWC_DATA short write {}/{} at {:#x}",
                    written,
                    data.len(),
                    base + self.fwc.written
                );
                self.fwc.burn_result = 1;
            }
        }
        self.fwc.crc = crc32_update(self.fwc.crc, data);
        self.fwc.written += data.len() as u64;
        self.stream_remaining = self.stream_remaining.saturating_sub(data.len() as u64);
        if self.stream_remaining == 0 {
            self.finish_fwc_stream();
        }
        RESP_OK
    }

    /// Queue the single response that ends a `SEND_FWC_DATA`.
    fn finish_fwc_stream(&mut self) {
        if self.fwc.burn_result != 0 {
            self.fail_response(CMD_SEND_FWC_DATA);
        } else {
            self.begin_response(CMD_SEND_FWC_DATA);
            let _ = self.finish_response(&[]);
        }
    }

    /// Build the 108-byte `struct hwinfo` payload.
    ///
    /// Field offsets follow `basic_cmd.c` (`magic[8]`, `reserved0[30]`,
    /// `init_mode`, `curr_mode`, `boot_stage`, `reserved1[3]`, `chipid[4]`,
    /// `reserved2[48]`), with the two fields the vendor source leaves zeroed but
    /// the real device fills in: the build stamp at offset 8 and the protocol
    /// version word at offset 72.
    ///
    /// `init_mode`/`curr_mode` stay 0 and `chipid` stays zeroed (the bootloader
    /// has no efuse accessor); the host accepts that, but it does *not* accept a
    /// zero protocol version.
    fn hwinfo(&self) -> [u8; HWINFO_SIZE] {
        let mut hw = [0u8; HWINFO_SIZE];
        hw[..6].copy_from_slice(b"HWINFO");
        hw[8..8 + HWINFO_BUILD_STAMP.len()].copy_from_slice(HWINFO_BUILD_STAMP);
        // Offset 40 decides which personality the host talks to: 0 = Boot ROM,
        // 1 = bootloader (FWC channel), 2 = storage R/W.
        hw[40] = self.storage.boot_stage();
        hw[72..76].copy_from_slice(&UPG_PROTO_VERSION.to_le_bytes());
        hw
    }

    /// Reply for a command this bootloader does not implement.
    ///
    /// The vendor (`unsupported_cmd_read_output_data`) answers `UPG_RESP_FAIL`
    /// with `data_length = sizeof(struct hwinfo)`, not `0`. Answering `OK` with an
    /// empty payload tells the host the command succeeded and leaves it driving
    /// stale state; the transport itself still passed, so the CSW stays `OK`.
    fn unsupported_response(&mut self, command: u8) -> u8 {
        self.response.clear();
        self.response_pos = 0;
        push_header(&mut self.response, UPG_RESP_MAGIC, command, RESP_FAIL, 0);
        let _ = self.finish_response(&[0u8; HWINFO_SIZE]);
        RESP_OK
    }

    /// Dispatch one complete `UPGC` frame.
    fn execute(&mut self, frame: &[u8]) -> u8 {
        self.execute_inner(frame)
    }

    fn execute_inner(&mut self, frame: &[u8]) -> u8 {
        let Some((command, data_length, _)) = parse_header(frame) else {
            return RESP_FAIL;
        };
        let data = &frame[UPG_HEADER_SIZE..];
        if data.len() < data_length as usize {
            return RESP_FAIL;
        }
        let data = &data[..data_length as usize];

        if !is_status_query(command) {
            self.host_upgrading = true;
        }

        match command {
            CMD_GET_HWINFO => {
                self.begin_response(command);
                self.finish_response(&self.hwinfo())
            }
            CMD_GET_TRACEINFO => self.unsupported_response(command),
            CMD_WRITE => {
                let Some((addr, len)) = read_u32_pair(data) else {
                    return self.fail_response(command);
                };
                let payload = &data[8..];
                if payload.len() < len as usize {
                    return self.fail_response(command);
                }
                let scratch = self.storage.scratch();
                let base = scratch.as_ptr() as usize;
                if (addr as usize) < base || (addr as usize) + len as usize > base + scratch.len() {
                    return self.fail_response(command);
                }
                let offset = addr as usize - base;
                scratch[offset..offset + len as usize].copy_from_slice(&payload[..len as usize]);
                self.begin_response(command);
                self.finish_response(&[])
            }
            CMD_READ => {
                let Some((addr, len)) = read_u32_pair(data) else {
                    return self.fail_response(command);
                };
                let scratch = self.storage.scratch();
                let base = scratch.as_ptr() as usize;
                if (addr as usize) < base || (addr as usize) + len as usize > base + scratch.len() {
                    return self.fail_response(command);
                }
                let offset = addr as usize - base;
                let len = len as usize;
                let mut payload: Vec<u8, RESP_MAX> = Vec::new();
                let _ = payload.extend_from_slice(&scratch[offset..offset + len]);
                self.begin_response(command);
                self.finish_response(&payload)
            }
            CMD_GET_MEM_BUF => {
                let addr = self.storage.scratch_addr();
                self.begin_response(command);
                self.finish_response(&addr.to_le_bytes())
            }
            CMD_FREE_MEM_BUF => {
                self.begin_response(command);
                self.finish_response(&[])
            }
            CMD_SET_UPG_CFG => {
                if data.len() >= 32 {
                    self.cfg_mode = data[0];
                }
                self.begin_response(command);
                self.finish_response(&[])
            }
            CMD_SET_UPG_END => {
                self.storage.flush();
                self.begin_response(command);
                self.finish_response(&[])
            }
            CMD_GET_LOG_SIZE => {
                self.begin_response(command);
                self.finish_response(&0u32.to_le_bytes())
            }
            CMD_GET_LOG_DATA => self.unsupported_response(command),
            CMD_SET_FWC_META => {
                if data.len() < 512 {
                    return self.fail_response(command);
                }
                let meta = &data[data.len() - 512..];
                if &meta[..4] != b"META" {
                    return self.fail_response(command);
                }
                self.storage.flush();
                self.fwc.meta.copy_from_slice(meta);
                self.fwc.meta_valid = true;
                // Target components name a partition; updater/`run` components
                // leave it empty and are not programmed into flash.
                let partition = meta_str(&self.fwc.meta, 0x48, 64);
                let size = u32::from_le_bytes(self.fwc.meta[0x8C..0x90].try_into().unwrap());
                self.fwc.target = if partition.is_empty() {
                    None
                } else {
                    match self.storage.partition_offset(partition) {
                        Some(offset) => Some(offset),
                        None => {
                            error!(
                                "aic_upg: FWC meta name=\"{}\" partition=\"{}\" -> UNKNOWN partition",
                                meta_str(&self.fwc.meta, 0x08, 64),
                                partition
                            );
                            return self.fail_response(command);
                        }
                    }
                };
                debug!(
                    "aic_upg: FWC meta name=\"{}\" partition=\"{}\" size={} -> flash offset {:?}",
                    meta_str(&self.fwc.meta, 0x08, 64),
                    partition,
                    size,
                    self.fwc.target
                );
                self.fwc.written = 0;
                self.fwc.crc = 0;
                self.fwc.burn_result = 0;
                self.fwc.run_result = 0;
                self.begin_response(command);
                self.finish_response(&[])
            }
            CMD_GET_BLOCK_SIZE => {
                let size = self.storage.block_size();
                self.begin_response(command);
                self.finish_response(&size.to_le_bytes())
            }
            // `SEND_FWC_DATA` normally never reaches here: its header only
            // announces the component length, so `transport_write` streams the
            // payload (see `begin_fwc_stream`). Answering a bare header with an
            // empty success keeps a header-only variant from wedging the host.
            CMD_SEND_FWC_DATA => {
                if !self.fwc.meta_valid {
                    return self.fail_response(command);
                }
                self.begin_response(command);
                self.finish_response(&[])
            }
            CMD_GET_FWC_CRC => {
                self.storage.flush();
                // Read back what is actually in flash: this is the decisive
                // evidence that the write reached the media.
                let mut probe = [0u8; 16];
                let base = self.fwc.target.unwrap_or(0);
                let n = self.storage.read(base, &mut probe);
                debug!(
                    "aic_upg: flushed, wrote {} bytes, flash[{:#x}..] = {}",
                    self.fwc.written,
                    base,
                    Hex(&probe[..n])
                );
                self.begin_response(command);
                self.finish_response(&self.fwc.crc.to_le_bytes())
            }
            CMD_GET_FWC_BURN_RESULT => {
                let result = self.fwc.burn_result as u32;
                self.begin_response(command);
                self.finish_response(&result.to_le_bytes())
            }
            CMD_GET_FWC_RUN_RESULT => {
                let result = self.fwc.run_result as u32;
                self.begin_response(command);
                self.finish_response(&result.to_le_bytes())
            }
            CMD_GET_STORAGE_MEDIA => {
                let mut media = [0u8; 68];
                let name = self.storage.media_type().as_bytes();
                let len = name.len().min(63);
                media[..len].copy_from_slice(&name[..len]);
                media[64..68].copy_from_slice(&self.storage.media_dev_id().to_le_bytes());
                self.begin_response(command);
                self.finish_response(&media)
            }
            CMD_GET_PARTITION_TABLE => {
                if self.partition_table.is_empty() {
                    return self.fail_response(command);
                }
                let table = self.partition_table;
                self.begin_response(command);
                self.finish_response(table.as_bytes())
            }
            CMD_READ_FWC_DATA => {
                let Some((offset, len)) = read_u32_pair(data) else {
                    return self.fail_response(command);
                };
                let mut buf = [0u8; 512];
                let len = (len as usize).min(buf.len());
                let read = self.storage.read(offset as u64, &mut buf[..len]);
                let mut payload: Vec<u8, RESP_MAX> = Vec::new();
                let _ = payload.extend_from_slice(&buf[..read]);
                self.begin_response(command);
                self.finish_response(&payload)
            }
            CMD_SET_UART_ARGS => {
                self.begin_response(command);
                self.finish_response(&[])
            }
            CMD_GET_STORAGE_GEOME => {
                let mut geo = [0u8; 24];
                geo[0..8].copy_from_slice(&self.storage.capacity().to_le_bytes());
                geo[8..12].copy_from_slice(&self.storage.block_size().to_le_bytes());
                geo[12..16].copy_from_slice(&self.storage.block_size().to_le_bytes());
                self.begin_response(command);
                self.finish_response(&geo)
            }
            CMD_ERASE_STORAGE => {
                if data.len() < 24 {
                    return self.fail_response(command);
                }
                let start = u64::from_le_bytes(data[0..8].try_into().unwrap());
                let size = u64::from_le_bytes(data[8..16].try_into().unwrap());
                self.storage.flush();
                if !self.storage.erase(start, size) {
                    return self.fail_response(command);
                }
                self.begin_response(command);
                self.finish_response(&[])
            }
            CMD_RUN_SHELL_STR => {
                // The official "Storage R/W" personality: the host drives flash
                // with `bd*` shell commands. Only the whitelisted command set is
                // accepted (see `super::shell`); anything else fails, so the host
                // can never run arbitrary code on the device.
                let Some(text) = shell_line(data) else {
                    return self.fail_response(command);
                };
                // `get_bootdevice` / `bdopen` / ... are how the host looks at the
                // device; only the commands that actually program flash count as
                // driving an upgrade (see `host_upgrading`). `reset` ends the
                // session instead, so the caller can boot the application.
                if super::shell::is_write(text) {
                    self.host_upgrading = true;
                }
                if text.split(' ').next() == Some("reset") {
                    self.reset_requested = true;
                }
                if super::shell::run(&mut *self.storage, text) {
                    self.begin_response(command);
                    self.finish_response(&[])
                } else {
                    self.fail_response(command)
                }
            }
            // EXEC is intentionally not implemented: a wrong jump from the host
            // must never run arbitrary code.
            _ => self.unsupported_response(command),
        }
    }
}

fn append<const N: usize>(buffer: &mut Vec<u8, N>, data: &[u8]) {
    let _ = buffer.extend_from_slice(data);
}

/// Commands the host sends just to look at the device.
///
/// AiBurn issues these while it is merely connected (`Get device hd info`,
/// `Refresh the partition tree`), so they must not count as an upgrade - see
/// [`Engine::host_upgrading`]. Everything else, including any command we do not
/// know, means the host is doing something.
fn is_status_query(command: u8) -> bool {
    matches!(
        command,
        CMD_GET_HWINFO
            | CMD_GET_TRACEINFO
            | CMD_READ
            | CMD_GET_MEM_BUF
            | CMD_FREE_MEM_BUF
            | CMD_GET_LOG_SIZE
            | CMD_GET_LOG_DATA
            | CMD_GET_BLOCK_SIZE
            | CMD_GET_FWC_CRC
            | CMD_GET_FWC_BURN_RESULT
            | CMD_GET_FWC_RUN_RESULT
            | CMD_GET_STORAGE_MEDIA
            | CMD_GET_PARTITION_TABLE
            | CMD_READ_FWC_DATA
            | CMD_GET_STORAGE_GEOME
            | CMD_RUN_SHELL_STR
    )
}

fn parse_header(frame: &[u8]) -> Option<(u8, u32, u32)> {
    if frame.len() < UPG_HEADER_SIZE || &frame[..4] != b"UPGC" {
        return None;
    }
    let command = frame[6];
    let data_length = u32::from_le_bytes(frame[8..12].try_into().ok()?);
    let checksum = u32::from_le_bytes(frame[12..16].try_into().ok()?);
    let expected = header_checksum(UPG_CMD_MAGIC, command, frame[7], data_length);
    (checksum == expected).then_some((command, data_length, checksum))
}

fn header_checksum(magic: u32, command: u8, status: u8, data_length: u32) -> u32 {
    let field = ((status as u32) << 24) | ((command as u32) << 16) | ((1u32) << 8) | 1u32;
    magic.wrapping_add(field).wrapping_add(data_length)
}

fn build_header(magic: u32, command: u8, status: u8, data_length: u32) -> [u8; UPG_HEADER_SIZE] {
    let mut header = [0u8; UPG_HEADER_SIZE];
    header[..4].copy_from_slice(&magic.to_le_bytes());
    header[4] = 1; // protocol
    header[5] = 1; // version
    header[6] = command;
    header[7] = status;
    header[8..12].copy_from_slice(&data_length.to_le_bytes());
    header[12..16]
        .copy_from_slice(&header_checksum(magic, command, status, data_length).to_le_bytes());
    header
}

fn push_header<const N: usize>(
    buffer: &mut Vec<u8, N>,
    magic: u32,
    command: u8,
    status: u8,
    data_length: u32,
) {
    let header = build_header(magic, command, status, data_length);
    append(buffer, &header);
}

fn read_u32_pair(data: &[u8]) -> Option<(u32, u32)> {
    if data.len() < 8 {
        return None;
    }
    Some((
        u32::from_le_bytes(data[0..4].try_into().ok()?),
        u32::from_le_bytes(data[4..8].try_into().ok()?),
    ))
}

/// Decode a `RUN_SHELL_STR` payload: `u32 length` followed by the ASCII command
/// line (no NUL terminator), as sent by the host tool.
fn shell_line(data: &[u8]) -> Option<&str> {
    let len = u32::from_le_bytes(data.get(..4)?.try_into().ok()?) as usize;
    let text = data.get(4..4 + len)?;
    core::str::from_utf8(text).ok()
}

/// Read a NUL-terminated string field out of the 512-byte `fwc_meta`.
fn meta_str(meta: &[u8; 512], offset: usize, len: usize) -> &str {
    let field = &meta[offset..offset + len];
    let end = field.iter().position(|&b| b == 0).unwrap_or(field.len());
    core::str::from_utf8(&field[..end]).unwrap_or("")
}

/// Lower-case hex formatter for log output.
struct Hex<'a>(&'a [u8]);

impl core::fmt::Display for Hex<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

/// zlib/IEEE CRC-32 (as used by the vendor `fwc_meta.crc`).
pub fn crc32_update(crc: u32, data: &[u8]) -> u32 {
    let mut crc = !crc;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}
