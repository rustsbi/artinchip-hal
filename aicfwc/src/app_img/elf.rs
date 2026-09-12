//! Minimal ELF32 parser: turns an application ELF directly into an AIC `loader`
//! payload, or flattens a PBP ELF. Only what the image packer needs is
//! implemented.

use anyhow::{Result, bail};

/// `SHF_ALLOC`.
const SHF_ALLOC: u32 = 0x2;
/// `SHT_PROGBITS`.
const SHT_PROGBITS: u32 = 1;

/// ELF32 little-endian header fields both flattens need.
struct Header {
    entry: u32,
    phoff: usize,
    phentsize: usize,
    phnum: usize,
    shoff: usize,
    shentsize: usize,
    shnum: usize,
}

fn header(data: &[u8]) -> Result<Header> {
    if data.len() < 52 || &data[..4] != b"\x7fELF" {
        bail!("input is not an ELF file");
    }
    if data[4] != 1 {
        bail!("only ELF32 is supported");
    }
    if data[5] != 1 {
        bail!("only little-endian ELF is supported");
    }
    if u16::from_le_bytes(data[18..20].try_into().unwrap()) != 0xF3 {
        bail!("ELF is not a RISC-V executable");
    }
    Ok(Header {
        entry: u32::from_le_bytes(data[0x18..0x1C].try_into().unwrap()),
        phoff: u32::from_le_bytes(data[0x1C..0x20].try_into().unwrap()) as usize,
        phentsize: u16::from_le_bytes(data[0x2A..0x2C].try_into().unwrap()) as usize,
        phnum: u16::from_le_bytes(data[0x2C..0x2E].try_into().unwrap()) as usize,
        shoff: u32::from_le_bytes(data[0x20..0x24].try_into().unwrap()) as usize,
        shentsize: u16::from_le_bytes(data[0x2E..0x30].try_into().unwrap()) as usize,
        shnum: u16::from_le_bytes(data[0x30..0x32].try_into().unwrap()) as usize,
    })
}

/// Lay `(address, bytes)` pieces out from the lowest address, zero filling gaps.
fn flatten(pieces: Vec<(u32, &[u8])>) -> Result<Vec<u8>> {
    let Some(min) = pieces.iter().map(|(address, _)| *address).min() else {
        bail!("nothing to flatten");
    };
    let Some(max) = pieces
        .iter()
        .map(|(address, bytes)| *address as usize + bytes.len())
        .max()
    else {
        bail!("nothing to flatten");
    };
    let mut image = vec![0u8; max - min as usize];
    for (address, bytes) in pieces {
        let start = address as usize - min as usize;
        image[start..start + bytes.len()].copy_from_slice(bytes);
    }
    Ok(image)
}

/// A flattened ELF image.
#[derive(Clone, Debug)]
pub struct ElfImage {
    pub entry: u32,
    pub min_vaddr: u32,
    /// `min_vaddr - 0x100` (the AIC header size).
    pub load_address: u32,
    pub payload: Vec<u8>,
}

pub fn parse(data: &[u8]) -> Result<ElfImage> {
    let header = header(data)?;
    let entry = header.entry;

    let mut loads = Vec::new();
    for index in 0..header.phnum {
        let offset = header.phoff + index * header.phentsize;
        if offset + 32 > data.len() {
            bail!("program header {index} is out of range");
        }
        let p_type = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        if p_type != 1 {
            continue; // only PT_LOAD
        }
        let p_offset =
            u32::from_le_bytes(data[offset + 4..offset + 8].try_into().unwrap()) as usize;
        let p_vaddr = u32::from_le_bytes(data[offset + 8..offset + 12].try_into().unwrap());
        let p_filesz =
            u32::from_le_bytes(data[offset + 16..offset + 20].try_into().unwrap()) as usize;
        let file_end = p_offset
            .checked_add(p_filesz)
            .filter(|end| *end <= data.len())
            .ok_or_else(|| anyhow::anyhow!("PT_LOAD segment {index} is out of range"))?;
        loads.push((p_vaddr, &data[p_offset..file_end]));
    }

    if loads.is_empty() {
        bail!("ELF has no PT_LOAD segments");
    }

    let Some(min_vaddr) = loads.iter().map(|(vaddr, _)| *vaddr).min() else {
        bail!("ELF has no PT_LOAD segments");
    };
    if min_vaddr < 0x100 {
        bail!("lowest load address {min_vaddr:#x} is too low for an AIC header");
    }

    Ok(ElfImage {
        entry,
        min_vaddr,
        load_address: min_vaddr - 0x100,
        payload: flatten(loads)?,
    })
}

/// Flatten an ELF the way `rust-objcopy -O binary` does: allocated `PROGBITS`
/// sections laid out by address, gaps zero filled.
///
/// This is what a **PBP** needs, and it is not the same as [`parse`]: a PBP's
/// linker script maps the ELF headers into its first segment, so the segment-based
/// image starts with `\x7fELF` where the PBP must start with `"PBP "`.
pub fn to_binary(data: &[u8]) -> Result<Vec<u8>> {
    let header = header(data)?;

    let mut sections = Vec::new();
    for index in 0..header.shnum {
        let offset = header.shoff + index * header.shentsize;
        if offset + 40 > data.len() {
            bail!("section header {index} is out of range");
        }
        let sh_type = u32::from_le_bytes(data[offset + 4..offset + 8].try_into().unwrap());
        let sh_flags = u32::from_le_bytes(data[offset + 8..offset + 12].try_into().unwrap());
        let sh_addr = u32::from_le_bytes(data[offset + 12..offset + 16].try_into().unwrap());
        let sh_offset = u32::from_le_bytes(data[offset + 16..offset + 20].try_into().unwrap());
        let sh_size = u32::from_le_bytes(data[offset + 20..offset + 24].try_into().unwrap());
        if sh_type != SHT_PROGBITS || sh_flags & SHF_ALLOC == 0 || sh_size == 0 {
            continue;
        }
        let start = sh_offset as usize;
        let end = start
            .checked_add(sh_size as usize)
            .filter(|end| *end <= data.len())
            .ok_or_else(|| anyhow::anyhow!("section {index} content is out of range"))?;
        sections.push((sh_addr, &data[start..end]));
    }

    if sections.is_empty() {
        bail!("ELF has no allocated PROGBITS sections");
    }
    flatten(sections)
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT_BASE: u32 = 0x100;

    /// Assemble a minimal ELF32 whose section table is `sections`, with content
    /// placed at `CONTENT_BASE + relative offset`.
    ///
    /// `sections` is `(sh_type, sh_flags, sh_addr, content offset, size)`.
    fn synth(sections: &[(u32, u32, u32, u32, u32)], content: &[u8]) -> Vec<u8> {
        const SHOFF: usize = 0x200;
        let mut data = vec![0u8; CONTENT_BASE as usize];
        data[..4].copy_from_slice(b"\x7fELF");
        data[4] = 1; // ELF32
        data[5] = 1; // little endian
        data[18..20].copy_from_slice(&0xF3u16.to_le_bytes()); // EM_RISCV
        data[0x20..0x24].copy_from_slice(&(SHOFF as u32).to_le_bytes());
        data[0x2E..0x30].copy_from_slice(&40u16.to_le_bytes());
        data[0x30..0x32].copy_from_slice(&(sections.len() as u16).to_le_bytes());
        data.extend_from_slice(content);
        data.resize(SHOFF, 0);

        for (sh_type, sh_flags, sh_addr, sh_offset, sh_size) in sections {
            let mut header = [0u8; 40];
            header[4..8].copy_from_slice(&sh_type.to_le_bytes());
            header[8..12].copy_from_slice(&sh_flags.to_le_bytes());
            header[12..16].copy_from_slice(&sh_addr.to_le_bytes());
            header[16..20].copy_from_slice(&sh_offset.to_le_bytes());
            header[20..24].copy_from_slice(&sh_size.to_le_bytes());
            data.extend_from_slice(&header);
        }
        data
    }

    #[test]
    fn to_binary_keeps_allocated_progbits_only() {
        let mut content = Vec::new();
        content.extend_from_slice(b"AAAAAAAA"); // relative 0
        content.extend_from_slice(b"CCCCCCCC"); // relative 8
        content.extend_from_slice(b"DDDDDDDD"); // relative 16

        let base = CONTENT_BASE;
        let sections = [
            (0, 0, 0, 0, 0), // null entry
            (SHT_PROGBITS, SHF_ALLOC, 0x3004_4000, base, 8),
            // NOBITS (`.bss`): occupies an address but has no file content.
            (8, SHF_ALLOC, 0x3004_4008, 0, 8),
            // Not allocated: `rust-objcopy -O binary` drops it.
            (SHT_PROGBITS, 0, 0x3004_4008, base + 8, 8),
            (SHT_PROGBITS, SHF_ALLOC, 0x3004_4010, base + 16, 8),
        ];
        let image = to_binary(&synth(&sections, &content)).unwrap();

        let mut expected = Vec::new();
        expected.extend_from_slice(b"AAAAAAAA");
        expected.extend_from_slice(&[0u8; 8]); // the NOBITS hole
        expected.extend_from_slice(b"DDDDDDDD");
        assert_eq!(image, expected);
    }

    #[test]
    fn to_binary_rejects_non_elf() {
        assert!(to_binary(b"not an elf at all").is_err());
    }
}
