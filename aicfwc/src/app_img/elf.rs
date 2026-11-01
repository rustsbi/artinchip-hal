//! Minimal ELF parser: turns an application ELF directly into an AIC `loader`
//! payload, or flattens a PBP ELF. Only what the image packer needs is
//! implemented.
//!
//! Both classes are supported because the chips differ: the D13x's E907 images
//! are ELF32, the D21x's C906 images ELF64. Addresses are narrowed to `u32` — every
//! load address in these images is well below 4 GiB — and a value that does not
//! fit is an error rather than a truncation.

use anyhow::{Context, Result, bail};

const SHF_ALLOC: u64 = 0x2;
const SHT_PROGBITS: u32 = 1;
const PT_LOAD: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Class {
    Elf32,
    Elf64,
}

/// The header fields both flattens need, in one shape for either class.
struct Header {
    class: Class,
    entry: u32,
    phoff: usize,
    phentsize: usize,
    phnum: usize,
    shoff: usize,
    shentsize: usize,
    shnum: usize,
}

fn u16_at(data: &[u8], offset: usize) -> Result<u16> {
    let bytes = data.get(offset..offset + 2).context("truncated ELF")?;
    Ok(u16::from_le_bytes(bytes.try_into().unwrap()))
}

fn u32_at(data: &[u8], offset: usize) -> Result<u32> {
    let bytes = data.get(offset..offset + 4).context("truncated ELF")?;
    Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
}

fn u64_at(data: &[u8], offset: usize) -> Result<u64> {
    let bytes = data.get(offset..offset + 8).context("truncated ELF")?;
    Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
}

/// Narrow an ELF64 address: an AIC header has no field for anything wider.
fn narrow(value: u64, what: &str) -> Result<u32> {
    u32::try_from(value).with_context(|| format!("{what} {value:#x} does not fit in 32 bits"))
}

fn header(data: &[u8]) -> Result<Header> {
    if data.len() < 52 || &data[..4] != b"\x7fELF" {
        bail!("input is not an ELF file");
    }
    if data[5] != 1 {
        bail!("only little-endian ELF is supported");
    }
    if u16_at(data, 18)? != 0xF3 {
        bail!("ELF is not a RISC-V executable");
    }

    let class = match data[4] {
        1 => Class::Elf32,
        2 => Class::Elf64,
        other => bail!("unsupported ELF class {other}"),
    };

    Ok(match class {
        Class::Elf32 => Header {
            class,
            entry: u32_at(data, 0x18)?,
            phoff: u32_at(data, 0x1C)? as usize,
            phentsize: u16_at(data, 0x2A)? as usize,
            phnum: u16_at(data, 0x2C)? as usize,
            shoff: u32_at(data, 0x20)? as usize,
            shentsize: u16_at(data, 0x2E)? as usize,
            shnum: u16_at(data, 0x30)? as usize,
        },
        Class::Elf64 => Header {
            class,
            entry: narrow(u64_at(data, 0x18)?, "entry point")?,
            phoff: narrow(u64_at(data, 0x20)?, "program header offset")? as usize,
            phentsize: u16_at(data, 0x36)? as usize,
            phnum: u16_at(data, 0x38)? as usize,
            shoff: narrow(u64_at(data, 0x28)?, "section header offset")? as usize,
            shentsize: u16_at(data, 0x3A)? as usize,
            shnum: u16_at(data, 0x3C)? as usize,
        },
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

/// The loadable segments of an ELF, as `(file offset, virtual address, physical
/// address, file size)`.
///
/// The physical address is the load address: a linker's `AT()` sets it apart
/// from the virtual one, and a raw binary is what the load addresses describe.
fn load_segments(data: &[u8], header: &Header) -> Result<Vec<(usize, u32, u32, usize)>> {
    let mut segments = Vec::new();
    for index in 0..header.phnum {
        let offset = header.phoff + index * header.phentsize;
        if u32_at(data, offset)? != PT_LOAD {
            continue;
        }
        let (p_offset, p_vaddr, p_paddr, p_filesz) = match header.class {
            Class::Elf32 => (
                u32_at(data, offset + 4)? as usize,
                u32_at(data, offset + 8)?,
                u32_at(data, offset + 12)?,
                u32_at(data, offset + 16)? as usize,
            ),
            Class::Elf64 => (
                narrow(u64_at(data, offset + 8)?, "segment offset")? as usize,
                narrow(u64_at(data, offset + 16)?, "segment address")?,
                narrow(u64_at(data, offset + 24)?, "segment load address")?,
                narrow(u64_at(data, offset + 32)?, "segment size")? as usize,
            ),
        };
        let file_end = p_offset
            .checked_add(p_filesz)
            .filter(|end| *end <= data.len())
            .with_context(|| format!("PT_LOAD segment {index} is out of range"))?;
        segments.push((p_offset, p_vaddr, p_paddr, file_end - p_offset));
    }
    Ok(segments)
}

/// Where a section is loaded: its containing segment's physical address plus the
/// section's offset inside that segment.
///
/// A section no `PT_LOAD` covers is not loaded by any segment, and there the
/// virtual address is all there is — true of every section in an ELF with no
/// program headers, where the two addresses coincide anyway.
fn section_load_address(segments: &[(usize, u32, u32, usize)], address: u32) -> u32 {
    for (_, p_vaddr, p_paddr, p_filesz) in segments {
        let span = *p_filesz as u32;
        if *p_vaddr <= address && address < p_vaddr.wrapping_add(span) {
            return p_paddr.wrapping_add(address - *p_vaddr);
        }
    }
    address
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

    // The application image keeps the segments in *virtual* address order: the
    // AIC header's `load_address`/`entry` are virtual addresses, and the app
    // script has no `AT()`, so the two addresses coincide. A segment's file
    // content is what lands in the image.
    let mut loads = Vec::new();
    for (p_offset, p_vaddr, _, p_filesz) in load_segments(data, &header)? {
        loads.push((p_vaddr, &data[p_offset..p_offset + p_filesz]));
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
    let segments = load_segments(data, &header)?;

    let mut sections = Vec::new();
    for index in 0..header.shnum {
        let offset = header.shoff + index * header.shentsize;
        let (sh_type, sh_flags, sh_addr, sh_offset, sh_size) = match header.class {
            Class::Elf32 => (
                u32_at(data, offset + 4)?,
                u32_at(data, offset + 8)? as u64,
                u32_at(data, offset + 12)? as u64,
                u32_at(data, offset + 16)? as u64,
                u32_at(data, offset + 20)? as u64,
            ),
            Class::Elf64 => (
                u32_at(data, offset + 4)?,
                u64_at(data, offset + 8)?,
                u64_at(data, offset + 16)?,
                u64_at(data, offset + 24)?,
                u64_at(data, offset + 32)?,
            ),
        };
        if sh_type != SHT_PROGBITS || sh_flags & SHF_ALLOC == 0 || sh_size == 0 {
            continue;
        }
        let start = narrow(sh_offset, "section offset")? as usize;
        let end = start
            .checked_add(narrow(sh_size, "section size")? as usize)
            .filter(|end| *end <= data.len())
            .with_context(|| format!("section {index} content is out of range"))?;
        let address = section_load_address(&segments, narrow(sh_addr, "section address")?);
        sections.push((address, &data[start..end]));
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

    /// Assemble a minimal ELF64 with one 8-byte allocated `PROGBITS` section and
    /// one `PT_LOAD` segment covering it — the shape a D21x application ELF has.
    fn synth64() -> Vec<u8> {
        const CONTENT: usize = 0x100;
        const PHOFF: usize = 0x40;
        const SHOFF: usize = 0x200;
        const ADDR: u64 = 0x4200_0100;

        let mut data = vec![0u8; 0x280];
        data[..4].copy_from_slice(b"\x7fELF");
        data[4] = 2; // ELF64
        data[5] = 1; // little endian
        data[18..20].copy_from_slice(&0xF3u16.to_le_bytes()); // EM_RISCV
        data[0x18..0x20].copy_from_slice(&(ADDR + 8).to_le_bytes()); // e_entry
        data[0x20..0x28].copy_from_slice(&(PHOFF as u64).to_le_bytes());
        data[0x28..0x30].copy_from_slice(&(SHOFF as u64).to_le_bytes());
        data[0x36..0x38].copy_from_slice(&56u16.to_le_bytes()); // e_phentsize
        data[0x38..0x3A].copy_from_slice(&1u16.to_le_bytes()); // e_phnum
        data[0x3A..0x3C].copy_from_slice(&64u16.to_le_bytes()); // e_shentsize
        data[0x3C..0x3E].copy_from_slice(&2u16.to_le_bytes()); // e_shnum

        data[CONTENT..CONTENT + 8].copy_from_slice(b"ABCDEFGH");

        // One PT_LOAD covering the section.
        let mut ph = [0u8; 56];
        ph[0..4].copy_from_slice(&PT_LOAD.to_le_bytes());
        ph[4..8].copy_from_slice(&5u32.to_le_bytes()); // R+X
        ph[8..16].copy_from_slice(&(CONTENT as u64).to_le_bytes());
        ph[16..24].copy_from_slice(&ADDR.to_le_bytes());
        ph[24..32].copy_from_slice(&ADDR.to_le_bytes()); // p_paddr == p_vaddr
        ph[32..40].copy_from_slice(&8u64.to_le_bytes()); // p_filesz
        ph[40..48].copy_from_slice(&8u64.to_le_bytes()); // p_memsz
        data[PHOFF..PHOFF + 56].copy_from_slice(&ph);

        // Section 0 (null, already zero) and section 1: allocated PROGBITS.
        let mut sh = [0u8; 64];
        sh[4..8].copy_from_slice(&SHT_PROGBITS.to_le_bytes());
        sh[8..16].copy_from_slice(&SHF_ALLOC.to_le_bytes());
        sh[16..24].copy_from_slice(&ADDR.to_le_bytes());
        sh[24..32].copy_from_slice(&(CONTENT as u64).to_le_bytes());
        sh[32..40].copy_from_slice(&8u64.to_le_bytes());
        data[SHOFF + 64..SHOFF + 128].copy_from_slice(&sh);
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
            (SHT_PROGBITS, SHF_ALLOC as u32, 0x3004_4000, base, 8),
            // NOBITS (`.bss`): occupies an address but has no file content.
            (8, SHF_ALLOC as u32, 0x3004_4008, 0, 8),
            // Not allocated: `rust-objcopy -O binary` drops it.
            (SHT_PROGBITS, 0, 0x3004_4008, base + 8, 8),
            (SHT_PROGBITS, SHF_ALLOC as u32, 0x3004_4010, base + 16, 8),
        ];
        let image = to_binary(&synth(&sections, &content)).unwrap();

        let mut expected = Vec::new();
        expected.extend_from_slice(b"AAAAAAAA");
        expected.extend_from_slice(&[0u8; 8]); // the NOBITS hole
        expected.extend_from_slice(b"DDDDDDDD");
        assert_eq!(image, expected);
    }

    #[test]
    fn elf64_segments_and_sections() {
        let data = synth64();

        // Sections: the PBP path.
        assert_eq!(to_binary(&data).unwrap(), b"ABCDEFGH");

        // Segments: the application path, with the AIC header stripped off.
        let image = parse(&data).unwrap();
        assert_eq!(image.entry, 0x4200_0108);
        assert_eq!(image.min_vaddr, 0x4200_0100);
        assert_eq!(image.load_address, 0x4200_0000);
        assert_eq!(image.payload, b"ABCDEFGH");
    }
}
