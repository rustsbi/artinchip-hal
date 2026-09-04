use std::{env, path::PathBuf};

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let ld = &out.join("artinchip-rt.ld");

    // Cargo automatically sets CARGO_FEATURE_<name>=1 for each enabled feature
    // of this crate. When pbp-hello-world enables d21x feature (which propagates
    // via artinchip-rt/d21x), CARGO_FEATURE_D21X is set here. d21x takes priority
    // over d13x when both are enabled (e.g. default + --features d21x).
    let script = if env::var("CARGO_FEATURE_D21X").is_ok() {
        LINKER_SCRIPT_D21X
    } else {
        LINKER_SCRIPT_D13X
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
