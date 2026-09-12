//! AIC USB upgrade (USBUPG) device support.
//!
//! Laid out like the other peripherals: this file only wires up the submodules
//! and re-exports their public items.
//!
//! * `non_blocking` - the interrupt-driven upgrade device (enumeration core and
//!   the async front-end that drives it).
//! * `protocol` - the vendor `USBUPG` command engine.
//! * `storage` - the media the engine programs.
//! * `shell` - the `RUN_SHELL_STR` (`bd*`) storage commands.

#[cfg(feature = "clic-interrupts")]
mod non_blocking;
mod protocol;
mod shell;
mod storage;

#[cfg(feature = "clic-interrupts")]
pub use non_blocking::*;
pub use protocol::*;
pub use storage::*;
