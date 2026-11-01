//! Links `app-blinky` at the `load_address` declared in the board preset (`boards/*.toml`)
//! and forwards `AIC_APP_NAME` to `#[app_entry]`.

use std::{env, path::Path};

use aicfwc::preset::{Chip, Preset};

fn main() {
    let chip = if env::var("CARGO_FEATURE_D21X").is_ok() {
        Chip::D21x
    } else {
        Chip::D13x
    };
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is always set");
    let boards_dir = aicfwc::preset::boards_dir_from(Path::new(&manifest_dir)).unwrap_or_else(|| {
        panic!(
            "no board preset directory (`boards/`) found above {manifest_dir}; the application's \
             link address comes from the board preset"
        )
    });
    let preset = Preset::for_chip(&boards_dir, chip).unwrap_or_else(|error| {
        panic!(
            "failed to load the board preset for chip {}: {error:#}",
            chip.name()
        )
    });
    println!("cargo:rerun-if-changed={}", preset.path.display());
    println!("cargo:rerun-if-changed={}", boards_dir.display());

    let app = preset.app("app-blinky").unwrap_or_else(|error| {
        panic!(
            "board preset {} cannot describe this application: {error:#}",
            preset.path.display()
        )
    });
    let base = app.load_address.unwrap_or_else(|| {
        panic!(
            "board preset {} declares no `[apps.app-blinky] load_address`",
            preset.path.display()
        )
    });

    println!("cargo:rustc-env=AIC_APP_NAME={}", app.name);
    println!("cargo:rustc-link-arg=-Tartinchip-app.ld");
    println!("cargo:rustc-link-arg=--defsym=APP_BASE={base:#x}");
}
