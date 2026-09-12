//! Packed multi-partition image (`AIC.FW`) container: the file `AiBurn`/`upgcmd`
//! consume for an image upgrade. Layout (verified against a vendor image):
//!
//! ```text
//! 0x000  0x800   firmware header (`struct image_header_upgrade`), zero padded
//! 0x800  meta area, 512-byte `"META"` entries
//! ...    file data area, each component 2048-byte aligned
//! ```
//!
//! Per the host, `0x800 + meta_size + file_size == actual size`.

use crate::util::round_up;

/// Firmware header magic (`"AIC.FW"`).
pub const FW_MAGIC: &[u8; 8] = b"AIC.FW\0\0";
/// Metadata entry magic.
pub const META_MAGIC: &[u8; 4] = b"META";
/// Header block size (header is zero padded to this).
pub const FW_HEADER_SIZE: usize = 0x800;
/// Metadata entry size.
pub const META_SIZE: usize = 512;
/// Component data alignment.
pub const DATA_ALIGNMENT: usize = 2048;
/// `ram` meta field value for components that are not loaded to RAM.
pub const RAM_NONE: u32 = 0xFFFF_FFFF;

/// A component stored in the image.
#[derive(Clone, Debug)]
pub struct FwComponent {
    /// Logical name, e.g. `spl`.
    pub name: String,
    pub partition: String,
    pub attr: String,
    pub filename: String,
    pub data: Vec<u8>,
    pub updater: bool,
    pub ram: Option<u32>,
}

impl FwComponent {
    /// Build a target (partition) component.
    pub fn target(
        name: impl Into<String>,
        partition: impl Into<String>,
        attr: impl Into<String>,
        filename: impl Into<String>,
        data: Vec<u8>,
    ) -> Self {
        Self {
            name: name.into(),
            partition: partition.into(),
            attr: attr.into(),
            filename: filename.into(),
            data,
            updater: false,
            ram: None,
        }
    }

    fn full_name(&self) -> String {
        if self.updater {
            format!("image.updater.{}", self.name)
        } else {
            format!("image.target.{}", self.name)
        }
    }
}

/// A complete `AIC.FW` image description.
#[derive(Clone, Debug, Default)]
pub struct FwImage {
    pub platform: String,
    pub product: String,
    pub version: String,
    pub media_type: String,
    pub media_dev_id: u32,
    pub media_id: String,
    /// Updaters first, then targets.
    pub components: Vec<FwComponent>,
}

impl FwImage {
    /// Build the packed image.
    pub fn build(&self) -> Vec<u8> {
        // One extra entry describes the 2048-byte header block itself.
        let entry_count = self.components.len() + 1;
        let meta_size = META_SIZE * entry_count;
        let file_offset = FW_HEADER_SIZE + meta_size;

        // Assign a 2048-byte aligned slot to every component.
        let mut offsets = Vec::with_capacity(self.components.len());
        let mut cursor = file_offset;
        for component in &self.components {
            offsets.push(cursor);
            cursor += round_up(component.data.len(), DATA_ALIGNMENT);
        }
        let file_size = cursor - file_offset;

        let mut image = vec![0u8; FW_HEADER_SIZE + meta_size + file_size];

        self.write_header(&mut image, meta_size, file_offset, file_size);

        // `image.info` covers the first 2048 bytes (the header block).
        let info_crc = crc32(&image[..FW_HEADER_SIZE]);
        write_meta(
            &mut image[FW_HEADER_SIZE..FW_HEADER_SIZE + META_SIZE],
            MetaRecord {
                name: "image.info",
                partition: "",
                offset: 0,
                size: FW_HEADER_SIZE as u32,
                crc: info_crc,
                ram: RAM_NONE,
                attr: "required",
                filename: "info.bin",
            },
        );

        for (index, component) in self.components.iter().enumerate() {
            let meta_off = FW_HEADER_SIZE + (index + 1) * META_SIZE;
            let record = MetaRecord {
                name: &component.full_name(),
                partition: &component.partition,
                offset: offsets[index] as u32,
                size: component.data.len() as u32,
                crc: crc32(&component.data),
                ram: component.ram.unwrap_or(RAM_NONE),
                attr: &component.attr,
                filename: &component.filename,
            };
            write_meta(&mut image[meta_off..meta_off + META_SIZE], record);

            let start = offsets[index];
            image[start..start + component.data.len()].copy_from_slice(&component.data);
        }

        image
    }

    fn write_header(
        &self,
        image: &mut [u8],
        meta_size: usize,
        file_offset: usize,
        file_size: usize,
    ) {
        let header = &mut image[..FW_HEADER_SIZE];
        header[0..8].copy_from_slice(FW_MAGIC);
        write_str(&mut header[0x08..0x48], &self.platform);
        write_str(&mut header[0x48..0x88], &self.product);
        write_str(&mut header[0x88..0xC8], &self.version);
        write_str(&mut header[0xC8..0x108], &self.media_type);
        header[0x108..0x10C].copy_from_slice(&self.media_dev_id.to_le_bytes());
        write_str(&mut header[0x10C..0x14C], &self.media_id);
        header[0x14C..0x150].copy_from_slice(&(FW_HEADER_SIZE as u32).to_le_bytes());
        header[0x150..0x154].copy_from_slice(&(meta_size as u32).to_le_bytes());
        header[0x154..0x158].copy_from_slice(&(file_offset as u32).to_le_bytes());
        header[0x158..0x15C].copy_from_slice(&(file_size as u32).to_le_bytes());
    }
}

struct MetaRecord<'a> {
    name: &'a str,
    partition: &'a str,
    offset: u32,
    size: u32,
    crc: u32,
    ram: u32,
    attr: &'a str,
    filename: &'a str,
}

fn write_meta(entry: &mut [u8], record: MetaRecord<'_>) {
    entry[0..4].copy_from_slice(META_MAGIC);
    write_str(&mut entry[0x08..0x48], record.name);
    write_str(&mut entry[0x48..0x88], record.partition);
    entry[0x88..0x8C].copy_from_slice(&record.offset.to_le_bytes());
    entry[0x8C..0x90].copy_from_slice(&record.size.to_le_bytes());
    entry[0x90..0x94].copy_from_slice(&record.crc.to_le_bytes());
    entry[0x94..0x98].copy_from_slice(&record.ram.to_le_bytes());
    write_str(&mut entry[0x98..0xD8], record.attr);
    write_str(&mut entry[0xD8..0x118], record.filename);
}

fn write_str(field: &mut [u8], value: &str) {
    let bytes = value.as_bytes();
    let len = bytes.len().min(field.len() - 1);
    field[..len].copy_from_slice(&bytes[..len]);
    field[len..].fill(0);
}

/// zlib/IEEE CRC-32.
pub fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc32_known_value() {
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
    }

    #[test]
    fn image_layout_and_size_relation() {
        let image = FwImage {
            platform: "d13x".into(),
            product: "test".into(),
            version: "0.1.0".into(),
            media_type: "spi-nor".into(),
            media_dev_id: 0,
            media_id: String::new(),
            components: vec![FwComponent::target(
                "bootloader",
                "bootloader",
                "mtd;required",
                "app-bootloader.bin",
                vec![0u8; 1000],
            )],
        };
        let bytes = image.build();
        assert_eq!(&bytes[..8], FW_MAGIC);
        let read = |off: usize| u32::from_le_bytes(bytes[off..off + 4].try_into().unwrap());
        let meta_size = read(0x150) as usize;
        let file_offset = read(0x154) as usize;
        let file_size = read(0x158) as usize;
        assert_eq!(FW_HEADER_SIZE + meta_size + file_size, bytes.len());
        assert_eq!(file_offset, FW_HEADER_SIZE + meta_size);
        assert_eq!(read(0x14C), FW_HEADER_SIZE as u32);
        // data slot aligned to 2048
        assert_eq!(file_size, round_up(1000, DATA_ALIGNMENT));
    }
}
