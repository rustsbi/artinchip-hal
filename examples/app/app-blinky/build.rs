//! Links the app at the PSRAM address the bootloader copies it to.
//!
//! `app-bootloader.toml` declares this app as an image component; because it
//! leaves `load_address`/`entry_point` unset, `aicfwc` reads both back out of the
//! ELF, so the linker base below is the single source of truth for where the app
//! is loaded and entered.

/// PSRAM address the loader copies the app payload to.
///
/// Chosen clear of everything the boot chain occupies:
/// `0x4000_0000` is the PSRAM pattern test, `0x4010_0000` the updater RAM slot
/// and `0x406c_0000..0x406e_1000` the bootloader itself (code + 64 KiB stack).
const APP_BASE: u32 = 0x4070_0000;

fn main() {
    println!("cargo:rustc-link-arg=-Tartinchip-rt.ld");
    println!("cargo:rustc-link-arg=--defsym=APP_BASE={APP_BASE:#x}");
}
