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
    let ld = &out.join("artinchip-rt.ld");

    // Cargo automatically sets CARGO_FEATURE_<name>=1 for each enabled feature
    // of this crate. When pbp-hello-world enables d21x feature (which propagates
    // via artinchip-rt/d21x), CARGO_FEATURE_D21X is set here. d21x takes priority
    // over d13x when both are enabled (e.g. default + --features d21x).
    let is_d21x = env::var("CARGO_FEATURE_D21X").is_ok();

    // With the `app` feature the crate is linked as an application image loaded
    // by the boot chain instead of a PBP: the 256-byte AIC header sits at the
    // load address, so the code is linked at `base + 0x100`. The base can be
    // overridden with `ARTINCHIP_APP_BASE` (hex or decimal).
    let script: String = if env::var("CARGO_FEATURE_APP").is_ok() {
        let default_base = if is_d21x { 0x0010_3100 } else { 0x406c_0000 };
        let base = env::var("ARTINCHIP_APP_BASE")
            .ok()
            .and_then(|value| parse_addr(&value))
            .unwrap_or(default_base);
        APP_SCRIPT_TEMPLATE.replace("{base}", &format!("{base:#x}"))
    } else if is_d21x {
        String::from_utf8(LINKER_SCRIPT_D21X.to_vec()).unwrap()
    } else {
        String::from_utf8(LINKER_SCRIPT_D13X.to_vec()).unwrap()
    };

    std::fs::write(ld, script).unwrap();

    println!("cargo:rustc-link-arg=-T{}", ld.display());
    println!("cargo:rustc-link-search={}", out.display());
}

/// Linker script for D13x / E907 (32-bit), (Compatible with D12x, G73x while not verified yet): PBP loaded at 0x3004_4000.
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

/// Application image linker script template. `{base}` is the fallback load
/// address; a `APP_BASE` symbol (from `--defsym=APP_BASE=...`, emitted by the
/// app crate's build script from its TOML manifest) overrides it. The 256-byte
/// AIC header lives at `base`, code starts at `base+0x100` and the loader entry
/// point is therefore `base + 0x100`.
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
