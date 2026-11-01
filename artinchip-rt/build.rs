use std::{env, path::PathBuf};

fn parse_addr(value: &str) -> Option<u32> {
    let value = value.trim();
    if let Some(hex) = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
    {
        u32::from_str_radix(hex, 16).ok()
    } else {
        value.parse().ok()
    }
}

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let is_d21x = env::var("CARGO_FEATURE_D21X").is_ok();

    let pbp_script = if is_d21x {
        String::from_utf8(LINKER_SCRIPT_D21X.to_vec()).unwrap()
    } else {
        String::from_utf8(LINKER_SCRIPT_D13X.to_vec()).unwrap()
    };

    let default_base = if is_d21x { 0x0010_3100 } else { 0x406c_0000 };
    let base = env::var("ARTINCHIP_APP_BASE")
        .ok()
        .and_then(|value| parse_addr(&value))
        .unwrap_or(default_base);
    let app_script = APP_SCRIPT_TEMPLATE.replace("{base}", &format!("{base:#x}"));

    std::fs::write(out.join("artinchip-pbp.ld"), &pbp_script).unwrap();
    std::fs::write(out.join("artinchip-app.ld"), &app_script).unwrap();

    // Compatibility alias for existing crates passing `-Tartinchip-rt.ld`.
    let compat_script = if env::var("CARGO_FEATURE_APP").is_ok() {
        &app_script
    } else {
        &pbp_script
    };
    let ld = out.join("artinchip-rt.ld");
    std::fs::write(&ld, compat_script).unwrap();

    println!("cargo:rustc-link-arg=-T{}", ld.display());
    println!("cargo:rustc-link-search={}", out.display());
}

/// Linker script for D13x / E907 (32-bit): PBP loaded at 0x3004_4000.
const LINKER_SCRIPT_D13X: &[u8] = b"OUTPUT_ARCH(riscv)
ENTRY(_start)
SECTIONS {
    . = 0x30044000 - 0x8;
    .head : ALIGN(4) {
        KEEP(*(.head.pbp))
    }
    . = 0x30044000;
    .text : ALIGN(4) {
        *(.text.entry)
        *(.text .text.*)
    }
    .rodata : ALIGN(4) {
        srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        . = ALIGN(4);
        erodata = .;
    }
    .clic.vector_table (NOLOAD) : ALIGN(64) {
        KEEP(*(.clic.vector_table))
        . = ALIGN(64);
    }
    .data : ALIGN(4) {
        sdata = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        . = ALIGN(4);
        edata = .;
    }
    sidata = LOADADDR(.data);
    .bss (NOLOAD) : ALIGN(4) {
        *(.bss.uninit)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;
    }
    /DISCARD/ : {
        *(.eh_frame)
    }
}";

/// Linker script for D21x / C906 (RV64): PBP loaded at SRAM 0x0010_3000.
const LINKER_SCRIPT_D21X: &[u8] = b"OUTPUT_ARCH(riscv)
ENTRY(_start)
SECTIONS {
    . = 0x00103000 - 0x20;
    .head : {
        KEEP(*(.head.pbp))
        KEEP(*(.head.pbp.version))
    }
    .text 0x00103120 : AT(0x00103000) {
        *(.text.entry)
        *(.text .text.*)
        . = ALIGN(8);
    }
    .rodata : AT(LOADADDR(.text) + SIZEOF(.text)) {
        srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        . = ALIGN(8);
        erodata = .;
    }
    .data : AT(LOADADDR(.rodata) + SIZEOF(.rodata)) {
        sdata = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        . = ALIGN(8);
        edata = .;
    }
    sidata = LOADADDR(.data);
    .bss (NOLOAD) : {
        *(.bss.uninit)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        . = ALIGN(8);
        ebss = .;
    }
    /DISCARD/ : {
        *(.eh_frame)
    }
}";

/// Application image linker script template (`APP_BASE` from `--defsym=APP_BASE=...`
/// overrides `{base}`; the 256-byte AIC header precedes code at `base + 0x100`).
const APP_SCRIPT_TEMPLATE: &str = r#"OUTPUT_ARCH(riscv)
ENTRY(_start)
SECTIONS {
    . = DEFINED(APP_BASE) ? (APP_BASE + 0x100) : ({base} + 0x100);
    .text : ALIGN(4) {
        *(.text.entry)
        *(.text .text.*)
    }
    .rodata : ALIGN(4) {
        srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        KEEP(*(.app.name))
        . = ALIGN(4);
        erodata = .;
    }
    .clic.vector_table (NOLOAD) : ALIGN(64) {
        KEEP(*(.clic.vector_table))
        . = ALIGN(64);
    }
    .data : ALIGN(4) {
        sdata = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        . = ALIGN(4);
        edata = .;
    }
    sidata = LOADADDR(.data);
    .bss (NOLOAD) : ALIGN(4) {
        *(.bss.uninit)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        ebss = .;
    }
    /DISCARD/ : {
        *(.eh_frame)
    }
}"#;
