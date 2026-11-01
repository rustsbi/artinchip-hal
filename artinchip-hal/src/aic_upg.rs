//! ArtInChip USB upgrade (`USBUPG`) protocol, storage backends, and USB class.
//!
//! * `class` - `embassy-usb` vendor-bulk device front-end (`async-usb` feature).
//! * `protocol` - transport-agnostic `USBUPG` command and streaming engine.
//! * `shell` - `RUN_SHELL_STR` (`bd*`) storage command interpreter.
//! * `storage` - partition-mapped flash backends ([`NorUpgStorage`], [`NandUpgStorage`]).

#[cfg(all(not(feature = "d12x"), feature = "async-usb"))]
mod class;
mod protocol;
mod shell;
mod storage;

#[cfg(all(not(feature = "d12x"), feature = "async-usb"))]
pub use class::{AicUpgBuffers, AicUpgUsb};
pub use protocol::*;
pub use storage::{NandUpgStorage, NorUpgStorage, Partition, SECTOR_SIZE, StorageBackend};
