use std::{env, path::PathBuf};

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let ld = &out.join("artinchip-rt.ld");

    std::fs::write(ld, LINKER_SCRIPT).unwrap();

    println!("cargo:rustc-link-arg=-T{}", ld.display());
    println!("cargo:rustc-link-search={}", out.display());
}

const LINKER_SCRIPT: &[u8] = b"OUTPUT_ARCH(riscv)
ENTRY(_start) 
SECTIONS {
    . = 0x30040000;
	.head : ALIGN(4) {		
        KEEP(*(.head.pbp))
	}
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
