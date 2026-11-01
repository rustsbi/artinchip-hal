//! `"AIC "` boot image container.
//!
//! ```text
//! 0x000  256-byte AIC header
//! 0x100  optional loader payload (padded to 256)
//! ...    optional PBP resource   (offset in header)
//! ...    optional private data   (offset in header)
//! ...    optional public key / AES IV
//! +0x000 MD5 of image[8..sign_offset] at `sign_offset`
//! ```
//!
//! * `sign_offset = round_up(content, 256)`, `sign_length = 16` (MD5),
//! * 32-bit word sum of the whole image must equal `0xFFFF_FFFF`.

use crate::checksum::{checksum_for_words, verify_checksum};
use crate::preset::{EXT_ALIGNMENT, NAME_MAX};
use crate::util::round_up;

/// AIC header size in bytes.
pub const AIC_HEADER_SIZE: usize = 256;
/// Resource (`pbp`, private, key, IV) alignment.
pub const RESOURCE_ALIGNMENT: usize = 32;
/// Image/content alignment.
pub const IMAGE_ALIGNMENT: usize = 256;
/// MD5 digest size.
pub const MD5_SIZE: usize = 16;
/// Default `head_ver` value used by the vendor tool.
pub const DEFAULT_HEADER_VERSION: u32 = 0x0001_0001;

/// Offset of the optional application name (`0x54`, inside the header's reserved
/// area). The reader lives in `artinchip-hal/src/system/image.rs`.
pub const NAME_OFFSET: usize = 0x54;

pub use crate::preset::{EXT_ALIGNMENT as AIC_EXT_ALIGNMENT, NAME_MAX as AIC_NAME_MAX};

/// Build the 256-byte `"AIC "` header from explicit field values.
pub fn build_aic_header(header: &AicHeader) -> [u8; AIC_HEADER_SIZE] {
    let mut bytes = [0u8; AIC_HEADER_SIZE];
    bytes[0..4].copy_from_slice(b"AIC ");
    bytes[4..8].copy_from_slice(&header.checksum.to_le_bytes());
    bytes[8..12].copy_from_slice(&header.header_version.to_le_bytes());
    bytes[12..16].copy_from_slice(&header.image_length.to_le_bytes());
    bytes[16..20].copy_from_slice(&header.firmware_version.to_le_bytes());
    bytes[20..24].copy_from_slice(&header.loader_length.to_le_bytes());
    bytes[24..28].copy_from_slice(&header.loader_load_address.to_le_bytes());
    bytes[28..32].copy_from_slice(&header.loader_entry_point.to_le_bytes());
    bytes[32..36].copy_from_slice(&header.sign_algo.to_le_bytes());
    bytes[36..40].copy_from_slice(&header.enc_algo.to_le_bytes());
    bytes[40..44].copy_from_slice(&header.sign_offset.to_le_bytes());
    bytes[44..48].copy_from_slice(&header.sign_length.to_le_bytes());
    bytes[48..52].copy_from_slice(&header.sign_key_offset.to_le_bytes());
    bytes[52..56].copy_from_slice(&header.sign_key_length.to_le_bytes());
    bytes[56..60].copy_from_slice(&header.iv_data_offset.to_le_bytes());
    bytes[60..64].copy_from_slice(&header.iv_data_length.to_le_bytes());
    bytes[64..68].copy_from_slice(&header.priv_data_offset.to_le_bytes());
    bytes[68..72].copy_from_slice(&header.priv_data_length.to_le_bytes());
    bytes[72..76].copy_from_slice(&header.pbp_data_offset.to_le_bytes());
    bytes[76..80].copy_from_slice(&header.pbp_data_length.to_le_bytes());
    bytes[80..84].copy_from_slice(&header.loader_ext_offset.to_le_bytes());
    bytes[NAME_OFFSET..NAME_OFFSET + 4].copy_from_slice(&header.app_name_len.to_le_bytes());
    bytes[NAME_OFFSET + 4..NAME_OFFSET + 4 + NAME_MAX].copy_from_slice(&header.app_name);
    bytes
}

/// Fields of the 256-byte `"AIC "` header (little-endian on the wire).
#[derive(Clone, Copy, Debug, Default)]
pub struct AicHeader {
    /// Checksum over the whole image (word sum must reach `0xFFFF_FFFF`).
    pub checksum: u32,
    /// Header format version (`head_ver`).
    pub header_version: u32,
    /// Total image length, excluding the optional `with_ext` tail padding.
    pub image_length: u32,
    /// Anti-rollback firmware version.
    pub firmware_version: u32,
    /// Real loader payload size (0 when there is no loader).
    pub loader_length: u32,
    /// Loader load address.
    pub loader_load_address: u32,
    /// Loader entry point.
    pub loader_entry_point: u32,
    /// Signature algorithm: 0 = MD5, 1 = RSA-2048.
    pub sign_algo: u32,
    /// Encryption algorithm: 0 = none, 1 = AES-128-CBC.
    pub enc_algo: u32,
    /// Offset of the MD5/signature.
    pub sign_offset: u32,
    /// Length of the MD5/signature.
    pub sign_length: u32,
    /// Public key offset.
    pub sign_key_offset: u32,
    /// Public key length.
    pub sign_key_length: u32,
    /// AES IV offset.
    pub iv_data_offset: u32,
    /// AES IV length.
    pub iv_data_length: u32,
    /// Private resource offset.
    pub priv_data_offset: u32,
    /// Private resource real size.
    pub priv_data_length: u32,
    /// PBP resource offset.
    pub pbp_data_offset: u32,
    /// PBP resource real size.
    pub pbp_data_length: u32,
    /// Offset of the concatenated (`with_ext`) loader image.
    pub loader_ext_offset: u32,
    /// Length of [`Self::app_name`] in bytes (0 = unnamed).
    pub app_name_len: u32,
    /// Optional application name, NUL padded.
    pub app_name: [u8; NAME_MAX],
}

struct Loader<'a> {
    data: &'a [u8],
    load_address: u32,
    entry_point: u32,
}

/// Builder for an unsigned (MD5-only) `"AIC "` image.
#[derive(Default)]
pub struct AicImageBuilder<'a> {
    header_version: u32,
    firmware_version: u32,
    loader: Option<Loader<'a>>,
    pbp: Option<&'a [u8]>,
    private: Option<&'a [u8]>,
    app_name: Option<&'a str>,
    with_ext: bool,
    ext_reserve: usize,
}

impl<'a> AicImageBuilder<'a> {
    /// Create a builder with the default header version.
    pub fn new() -> Self {
        Self {
            header_version: DEFAULT_HEADER_VERSION,
            ..Self::default()
        }
    }

    /// Override the header (`head_ver`) field.
    pub fn header_version(mut self, version: u32) -> Self {
        self.header_version = version;
        self
    }

    /// Override the anti-rollback firmware version.
    pub fn firmware_version(mut self, version: u32) -> Self {
        self.firmware_version = version;
        self
    }

    /// Attach a loader payload with its load address and entry point.
    pub fn loader(mut self, data: &'a [u8], load_address: u32, entry_point: u32) -> Self {
        self.loader = Some(Loader {
            data,
            load_address,
            entry_point,
        });
        self
    }

    /// Attach a PBP resource.
    pub fn pbp(mut self, pbp: &'a [u8]) -> Self {
        self.pbp = Some(pbp);
        self
    }

    /// Attach the private (`pbp_cfg.bin`) resource.
    pub fn private(mut self, private: &'a [u8]) -> Self {
        self.private = Some(private);
        self
    }

    /// Record the application name in the reserved header area.
    ///
    /// The bootloader reads it back to decide whether the image in a partition is
    /// the application it was told to boot. Longer names are truncated to
    /// [`NAME_MAX`].
    pub fn app_name(mut self, name: &'a str) -> Self {
        self.app_name = Some(name);
        self
    }

    /// Mark the image as `with_ext`: `loader_length` is cleared, the loader is
    /// expected to be concatenated after the image, and the result is padded to
    /// [`EXT_ALIGNMENT`].
    pub fn with_ext(mut self, enabled: bool) -> Self {
        self.with_ext = enabled;
        self
    }

    /// Reserve `bytes` between the PBP image and the concatenated (`with_ext`)
    /// loader, moving [`AicHeader::loader_ext_offset`] up by that much.
    ///
    /// The nested loader is run in the RAM slot the BootROM copies the container
    /// to, and the PBP it runs is placed there too — but a PBP's *stack* is
    /// addressed relative to the code, so it lands in that same slot rather than in
    /// the SRAM the PBP was linked for. Without a reserve the stack sits on top of
    /// the loader that PBP is about to copy out of the container. See
    /// `artinchip-rt/src/pbp.rs` for the layout the reservation has to cover.
    pub fn ext_reserve(mut self, bytes: usize) -> Self {
        self.ext_reserve = bytes;
        self
    }

    /// Build the AIC container.
    pub fn build(&self) -> Vec<u8> {
        let loader_len = self.loader.as_ref().map_or(0, |l| l.data.len());
        let resource_start = AIC_HEADER_SIZE + round_up(loader_len, IMAGE_ALIGNMENT);

        let mut cursor = resource_start;
        let pbp_offset = cursor;
        if let Some(pbp) = self.pbp {
            cursor += round_up(pbp.len(), RESOURCE_ALIGNMENT);
        }
        let priv_offset = cursor;
        if let Some(private) = self.private {
            cursor += round_up(private.len(), RESOURCE_ALIGNMENT);
        }

        let signed_len = round_up(cursor, IMAGE_ALIGNMENT);
        let image_len = signed_len + MD5_SIZE;

        // The reserve is kept a whole number of [`EXT_ALIGNMENT`]s so the nested
        // loader stays aligned the same way it would without one.
        let loader_ext_offset = if self.with_ext {
            round_up(image_len, EXT_ALIGNMENT) + round_up(self.ext_reserve, EXT_ALIGNMENT)
        } else {
            0
        };

        let mut header = AicHeader {
            header_version: self.header_version,
            image_length: image_len as u32,
            firmware_version: self.firmware_version,
            loader_length: if self.with_ext { 0 } else { loader_len as u32 },
            loader_load_address: self.loader.as_ref().map_or(0, |l| l.load_address),
            loader_entry_point: self.loader.as_ref().map_or(0, |l| l.entry_point),
            sign_offset: signed_len as u32,
            sign_length: MD5_SIZE as u32,
            priv_data_offset: if self.private.is_some() {
                priv_offset as u32
            } else {
                0
            },
            priv_data_length: self.private.map_or(0, |p| p.len() as u32),
            pbp_data_offset: if self.pbp.is_some() {
                pbp_offset as u32
            } else {
                0
            },
            pbp_data_length: self.pbp.map_or(0, |p| p.len() as u32),
            loader_ext_offset: loader_ext_offset as u32,
            ..AicHeader::default()
        };

        if let Some(name) = self.app_name {
            let len = name.len().min(NAME_MAX);
            header.app_name_len = len as u32;
            header.app_name[..len].copy_from_slice(&name.as_bytes()[..len]);
        }

        let mut image = vec![0u8; image_len];

        if let Some(loader) = &self.loader {
            image[AIC_HEADER_SIZE..AIC_HEADER_SIZE + loader.data.len()]
                .copy_from_slice(loader.data);
        }
        if let Some(pbp) = self.pbp {
            image[pbp_offset..pbp_offset + pbp.len()].copy_from_slice(pbp);
        }
        if let Some(private) = self.private {
            image[priv_offset..priv_offset + private.len()].copy_from_slice(private);
        }

        header.checksum = 0;
        image[..AIC_HEADER_SIZE].copy_from_slice(&build_aic_header(&header));

        let digest = md5_digest(&image[8..signed_len]);
        image[signed_len..signed_len + MD5_SIZE].copy_from_slice(&digest);

        let checksum = checksum_for_words(&image, u32::MAX);
        image[4..8].copy_from_slice(&checksum.to_le_bytes());
        debug_assert!(verify_checksum(&image, u32::MAX));

        if self.with_ext {
            image.resize(loader_ext_offset, 0);
        }
        image
    }
}

fn md5_digest(data: &[u8]) -> [u8; MD5_SIZE] {
    use md5::{Digest, Md5};
    let digest = Md5::digest(data);
    let mut out = [0u8; MD5_SIZE];
    out.copy_from_slice(&digest);
    out
}

/// Convenience wrapper: wrap a single resource (typically a PBP) into an AIC
/// image, matching the current `pbp_ext.aic` use case.
pub fn wrap_resource(resource: &[u8]) -> Vec<u8> {
    AicImageBuilder::new().pbp(resource).build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_is_well_formed() {
        let image = wrap_resource(&[0xAAu8; 100]);
        assert_eq!(&image[..4], b"AIC ");
        assert_eq!(
            image.len(),
            round_up(256 + round_up(100, 32), 256) + MD5_SIZE
        );
        assert!(verify_checksum(&image, u32::MAX));
    }

    #[test]
    fn loader_pbp_private_layout() {
        let loader = [0x11u8; 700];
        let pbp = [0x22u8; 60];
        let private = [0x33u8; 40];
        let image = AicImageBuilder::new()
            .loader(&loader, 0x406c_0000, 0x406c_0100)
            .pbp(&pbp)
            .private(&private)
            .build();

        let read = |off: usize| u32::from_le_bytes(image[off..off + 4].try_into().unwrap());
        assert_eq!(read(0x14), loader.len() as u32);
        assert_eq!(read(0x18), 0x406c_0000);
        assert_eq!(read(0x1c), 0x406c_0100);
        assert_eq!(
            read(0x48),
            (AIC_HEADER_SIZE + round_up(loader.len(), 256)) as u32
        );
        assert_eq!(read(0x4c), pbp.len() as u32);
        assert_eq!(&image[0x100..0x100 + loader.len()], &loader);
    }

    /// Reproduce the vendor `pbp_ext.aic` layout
    /// (D13x `d13x.pbp` = 27728 B, `pbp_cfg.bin` = 736 B): the `with_ext`
    /// container carries no loader, and the loader is concatenated at
    /// `loader_ext_offset`. This is the image that must sit at flash offset 0.
    #[test]
    fn with_ext_matches_vendor_pbp_ext_layout() {
        let pbp = [0x5Au8; 27728];
        let private = [0xA5u8; 736];
        let image = AicImageBuilder::new()
            .pbp(&pbp)
            .private(&private)
            .with_ext(true)
            .build();

        let read = |off: usize| u32::from_le_bytes(image[off..off + 4].try_into().unwrap());
        assert_eq!(read(0x14), 0, "with_ext clears loader_length");
        assert_eq!(read(0x48), 0x100, "pbp follows the 256-byte header");
        assert_eq!(read(0x4c), 27728);
        assert_eq!(read(0x40), 0x6D60, "private follows the PBP");
        assert_eq!(read(0x44), 736);
        assert_eq!(read(0x28), 0x7100, "sign (MD5) offset");
        assert_eq!(read(0x0c), 0x7110, "image_length excludes the ext tail");
        assert_eq!(read(0x50), 0x7200, "loader is concatenated here");
        assert_eq!(image.len(), 0x7200);
        assert!(verify_checksum(&image, u32::MAX));
    }

    /// The name the bootloader matches an application image against is stored in
    /// the reserved header area, and a long name is truncated, not overflowed.
    #[test]
    fn app_name_round_trips() {
        let loader = [0x11u8; 128];
        let image = AicImageBuilder::new()
            .loader(&loader, 0x4070_0000, 0x4070_0100)
            .app_name("blinky")
            .build();

        let read = |off: usize| u32::from_le_bytes(image[off..off + 4].try_into().unwrap());
        assert_eq!(read(NAME_OFFSET), "blinky".len() as u32);
        assert_eq!(&image[NAME_OFFSET + 4..NAME_OFFSET + 4 + 6], b"blinky");

        let long = "x".repeat(NAME_MAX + 10);
        let image = AicImageBuilder::new()
            .loader(&loader, 0x4070_0000, 0x4070_0100)
            .app_name(&long)
            .build();
        let read = |off: usize| u32::from_le_bytes(image[off..off + 4].try_into().unwrap());
        assert_eq!(read(NAME_OFFSET), NAME_MAX as u32);
        assert!(verify_checksum(&image, u32::MAX));
    }

    /// `ext_reserve` pushes the concatenated loader up by the reserved bytes, so a
    /// PBP whose stack follows its own image does not land on the loader.
    #[test]
    fn ext_reserve_moves_the_concatenated_loader() {
        let pbp = [0x5Au8; 27728];
        let plain = AicImageBuilder::new().pbp(&pbp).with_ext(true).build();
        let reserved = AicImageBuilder::new()
            .pbp(&pbp)
            .with_ext(true)
            .ext_reserve(0x2000)
            .build();

        let read =
            |image: &[u8], off: usize| u32::from_le_bytes(image[off..off + 4].try_into().unwrap());
        let plain_ext = read(&plain, 0x50) as usize;
        let reserved_ext = read(&reserved, 0x50) as usize;
        assert_eq!(reserved_ext, plain_ext + 0x2000);
        assert_eq!(reserved.len(), reserved_ext);
        // The reserved bytes are a zero hole between the image and the loader.
        assert!(reserved[plain_ext..reserved_ext].iter().all(|&b| b == 0));
        // `image_length` is unaffected: the reserve is tail padding.
        assert_eq!(read(&plain, 0x0c), read(&reserved, 0x0c));
    }
}
