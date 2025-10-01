//! CE register blocks and registers.

use volatile_register::{RO, RW};

/// Cryptographic Engine register block.
#[repr(C)]
pub struct RegisterBlock {
    /// Interrupt control register (`CE_IRQ_CTL`).
    #[doc(alias = "CE_IRQ_CTL")]
    pub int_ctrl: RW<IntCtrl>,
    /// Interrupt status register (`CE_IRQ_STA`).
    #[doc(alias = "CE_IRQ_STA")]
    pub int_status: RW<IntStatus>,
    /// Task descriptor address register (`CE_TSK_ADDR`).
    #[doc(alias = "CE_TSK_ADDR")]
    pub task_addr: RW<u32>,
    /// Task control register (`CE_TSK_CTL`).
    #[doc(alias = "CE_TSK_CTL")]
    pub task_ctrl: RW<TaskCtrl>,
    /// Task status register (`CE_TSK_STA`).
    #[doc(alias = "CE_TSK_STA")]
    pub task_status: RO<TaskStatus>,
    /// Task error register (`CE_TSK_ERR`).
    #[doc(alias = "CE_TSK_ERR")]
    pub task_err: RW<u32>,
    _reserved0: [u8; 0xFE4],
    /// Version register (`CE_VER`).
    #[doc(alias = "CE_VER")]
    pub version: RO<u32>,
}

/// Interrupt control register.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct IntCtrl(u32);

impl IntCtrl {
    const CRYPTO_ASYM_INT_EN: u32 = 0x1 << 2;
    const CRYPTO_HASH_INT_EN: u32 = 0x1 << 1;
    const CRYPTO_SYM_INT_EN: u32 = 0x1;

    /// Enable asymmetric unit interrupt.
    #[inline]
    pub const fn enable_asym_int(self) -> Self {
        Self(self.0 | Self::CRYPTO_ASYM_INT_EN)
    }
    /// Disable asymmetric unit interrupt.
    #[inline]
    pub const fn disable_asym_int(self) -> Self {
        Self(self.0 & !Self::CRYPTO_ASYM_INT_EN)
    }
    /// Check if asymmetric unit interrupt is enabled.
    #[inline]
    pub const fn is_asym_int_enabled(self) -> bool {
        self.0 & Self::CRYPTO_ASYM_INT_EN != 0
    }
    /// Enable hash unit interrupt.
    #[inline]
    pub const fn enable_hash_int(self) -> Self {
        Self(self.0 | Self::CRYPTO_HASH_INT_EN)
    }
    /// Disable hash unit interrupt.
    #[inline]
    pub const fn disable_hash_int(self) -> Self {
        Self(self.0 & !Self::CRYPTO_HASH_INT_EN)
    }
    /// Check if hash unit interrupt is enabled.
    #[inline]
    pub const fn is_hash_int_enabled(self) -> bool {
        self.0 & Self::CRYPTO_HASH_INT_EN != 0
    }
    /// Enable symmetric unit interrupt.
    #[inline]
    pub const fn enable_sym_int(self) -> Self {
        Self(self.0 | Self::CRYPTO_SYM_INT_EN)
    }
    /// Disable symmetric unit interrupt.
    #[inline]
    pub const fn disable_sym_int(self) -> Self {
        Self(self.0 & !Self::CRYPTO_SYM_INT_EN)
    }
    /// Check if symmetric unit interrupt is enabled.
    #[inline]
    pub const fn is_sym_int_enabled(self) -> bool {
        self.0 & Self::CRYPTO_SYM_INT_EN != 0
    }
}

/// Interrupt status register.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct IntStatus(u32);

impl IntStatus {
    const CRYPTO_ASYM_INT_STATUS: u32 = 0x1 << 2;
    const CRYPTO_HASH_INT_STATUS: u32 = 0x1 << 1;
    const CRYPTO_SYM_INT_STATUS: u32 = 0x1;

    /// Check if asymmetric unit interrupt is pending.
    #[inline]
    pub const fn is_asym_int_pending(self) -> bool {
        self.0 & Self::CRYPTO_ASYM_INT_STATUS != 0
    }
    /// Clear asymmetric unit interrupt status.
    #[inline]
    pub const fn clear_asym_int_status(self) -> Self {
        Self(self.0 | Self::CRYPTO_ASYM_INT_STATUS)
    }
    /// Check if hash unit interrupt is pending.
    #[inline]
    pub const fn is_hash_int_pending(self) -> bool {
        self.0 & Self::CRYPTO_HASH_INT_STATUS != 0
    }
    /// Clear hash unit interrupt status.
    #[inline]
    pub const fn clear_hash_int_status(self) -> Self {
        Self(self.0 | Self::CRYPTO_HASH_INT_STATUS)
    }
    /// Check if symmetric unit interrupt is pending.
    #[inline]
    pub const fn is_sym_int_pending(self) -> bool {
        self.0 & Self::CRYPTO_SYM_INT_STATUS != 0
    }
    /// Clear symmetric unit interrupt status.
    #[inline]
    pub const fn clear_sym_int_status(self) -> Self {
        Self(self.0 | Self::CRYPTO_SYM_INT_STATUS)
    }
}

/// Crypto algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CryptoAlgorithm {
    /// AES-ECB algorithm.
    AesEcb = 0x0,
    /// AES-CBC algorithm.
    AesCbc = 0x1,
    /// AES-CTR algorithm.
    AesCtr = 0x2,
    /// DES-ECB algorithm.
    DesEcb = 0x10,
    /// DES-CBC algorithm.
    DesCbc = 0x11,
    /// TDES-ECB algorithm.
    TdesEcb = 0x20,
    /// TDES-CBC algorithm.
    TdesCbc = 0x21,
    /// RSA algorithm.
    Rsa = 0x30,
    /// SHA-1 algorithm.
    Sha1 = 0x40,
    /// SHA-224 algorithm.
    Sha224 = 0x41,
    /// SHA-256 algorithm.
    Sha256 = 0x42,
    /// SHA-384 algorithm.
    Sha384 = 0x43,
    /// SHA-512 algorithm.
    Sha512 = 0x44,
    /// MD5 algorithm.
    Md5 = 0x45,
    /// HMAC-SHA-1 algorithm.
    HmacSha1 = 0x46,
    /// HMAC-SHA-256 algorithm.
    HmacSha256 = 0x47,
    /// TRNG algorithm.
    Trng = 0x50,
}

/// Task control register.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TaskCtrl(u32);

impl TaskCtrl {
    const TASK_LOAD: u32 = 0x1 << 31;
    const CURRENT_ALGORITHM: u32 = 0xFF;

    /// Start a new task by loading the task descriptor from the address in `CE_TSK_ADDR`.
    #[inline]
    pub const fn start_task(self) -> Self {
        Self(self.0 | Self::TASK_LOAD)
    }
    /// Set load task bit.
    #[inline]
    pub const fn set_load_task_bit(self, set: bool) -> Self {
        Self(if set {
            self.0 | Self::TASK_LOAD
        } else {
            self.0 & !Self::TASK_LOAD
        })
    }
    /// Get load task bit.
    #[inline]
    pub const fn load_task_bit(self) -> bool {
        self.0 & Self::TASK_LOAD != 0
    }
    /// Set current algorithm.
    #[inline]
    pub const fn set_current_algorithm(self, alg: CryptoAlgorithm) -> Self {
        Self((self.0 & !Self::CURRENT_ALGORITHM) | (Self::CURRENT_ALGORITHM & (alg as u32)))
    }
    /// Get current algorithm.
    #[inline]
    pub const fn current_algorithm(self) -> CryptoAlgorithm {
        match self.0 & Self::CURRENT_ALGORITHM {
            0x0 => CryptoAlgorithm::AesEcb,
            0x1 => CryptoAlgorithm::AesCbc,
            0x2 => CryptoAlgorithm::AesCtr,
            0x10 => CryptoAlgorithm::DesEcb,
            0x11 => CryptoAlgorithm::DesCbc,
            0x20 => CryptoAlgorithm::TdesEcb,
            0x21 => CryptoAlgorithm::TdesCbc,
            0x30 => CryptoAlgorithm::Rsa,
            0x40 => CryptoAlgorithm::Sha1,
            0x41 => CryptoAlgorithm::Sha224,
            0x42 => CryptoAlgorithm::Sha256,
            0x43 => CryptoAlgorithm::Sha384,
            0x44 => CryptoAlgorithm::Sha512,
            0x45 => CryptoAlgorithm::Md5,
            0x46 => CryptoAlgorithm::HmacSha1,
            0x47 => CryptoAlgorithm::HmacSha256,
            0x50 => CryptoAlgorithm::Trng,
            _ => unreachable!(),
        }
    }
}

/// Task status register.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TaskStatus(u32);

impl TaskStatus {
    const ASYM_TASK: u32 = 0xFF << 16;
    const HASH_TASK: u32 = 0xFF << 8;
    const SYM_TASK: u32 = 0xFF;

    /// Get asymmetric unit running algorithm.
    #[inline]
    pub const fn asym_task(self) -> CryptoAlgorithm {
        match (self.0 & Self::ASYM_TASK) >> 16 {
            0x0 => CryptoAlgorithm::AesEcb,
            0x1 => CryptoAlgorithm::AesCbc,
            0x2 => CryptoAlgorithm::AesCtr,
            0x10 => CryptoAlgorithm::DesEcb,
            0x11 => CryptoAlgorithm::DesCbc,
            0x20 => CryptoAlgorithm::TdesEcb,
            0x21 => CryptoAlgorithm::TdesCbc,
            0x30 => CryptoAlgorithm::Rsa,
            0x40 => CryptoAlgorithm::Sha1,
            0x41 => CryptoAlgorithm::Sha224,
            0x42 => CryptoAlgorithm::Sha256,
            0x43 => CryptoAlgorithm::Sha384,
            0x44 => CryptoAlgorithm::Sha512,
            0x45 => CryptoAlgorithm::Md5,
            0x46 => CryptoAlgorithm::HmacSha1,
            0x47 => CryptoAlgorithm::HmacSha256,
            0x50 => CryptoAlgorithm::Trng,
            _ => unreachable!(),
        }
    }
    /// Get hash unit running algorithm.
    #[inline]
    pub const fn hash_task(self) -> CryptoAlgorithm {
        match (self.0 & Self::HASH_TASK) >> 8 {
            0x0 => CryptoAlgorithm::AesEcb,
            0x1 => CryptoAlgorithm::AesCbc,
            0x2 => CryptoAlgorithm::AesCtr,
            0x10 => CryptoAlgorithm::DesEcb,
            0x11 => CryptoAlgorithm::DesCbc,
            0x20 => CryptoAlgorithm::TdesEcb,
            0x21 => CryptoAlgorithm::TdesCbc,
            0x30 => CryptoAlgorithm::Rsa,
            0x40 => CryptoAlgorithm::Sha1,
            0x41 => CryptoAlgorithm::Sha224,
            0x42 => CryptoAlgorithm::Sha256,
            0x43 => CryptoAlgorithm::Sha384,
            0x44 => CryptoAlgorithm::Sha512,
            0x45 => CryptoAlgorithm::Md5,
            0x46 => CryptoAlgorithm::HmacSha1,
            0x47 => CryptoAlgorithm::HmacSha256,
            0x50 => CryptoAlgorithm::Trng,
            _ => unreachable!(),
        }
    }
    /// Get symmetric unit running algorithm.
    #[inline]
    pub const fn sym_task(self) -> CryptoAlgorithm {
        match self.0 & Self::SYM_TASK {
            0x0 => CryptoAlgorithm::AesEcb,
            0x1 => CryptoAlgorithm::AesCbc,
            0x2 => CryptoAlgorithm::AesCtr,
            0x10 => CryptoAlgorithm::DesEcb,
            0x11 => CryptoAlgorithm::DesCbc,
            0x20 => CryptoAlgorithm::TdesEcb,
            0x21 => CryptoAlgorithm::TdesCbc,
            0x30 => CryptoAlgorithm::Rsa,
            0x40 => CryptoAlgorithm::Sha1,
            0x41 => CryptoAlgorithm::Sha224,
            0x42 => CryptoAlgorithm::Sha256,
            0x43 => CryptoAlgorithm::Sha384,
            0x44 => CryptoAlgorithm::Sha512,
            0x45 => CryptoAlgorithm::Md5,
            0x46 => CryptoAlgorithm::HmacSha1,
            0x47 => CryptoAlgorithm::HmacSha256,
            0x50 => CryptoAlgorithm::Trng,
            _ => unreachable!(),
        }
    }
}

/// Crypto unit error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CryptoUnitError {
    /// Algorithm not supported.
    UnsupportedAlgorithm = 0x1,
    /// Data length error.
    DataLengthError = 0x2,
    /// Secure SRAM access error.
    SecureSramAccessError = 0x4,
    /// Address not 8-byte aligned.
    AddressNotAligned = 0x8,
    /// Key length incorrect.
    KeyLengthIncorrect = 0x10,
}

/// Task error register.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TaskError(u32);

impl TaskError {
    const ASYM_ERR: u32 = 0xFF << 16;
    const HASH_ERR: u32 = 0xFF << 8;
    const SYM_ERR: u32 = 0xFF;

    /// Get asymmetric unit error.
    #[inline]
    pub const fn asym_err(self) -> CryptoUnitError {
        match (self.0 & Self::ASYM_ERR) >> 16 {
            0x1 => CryptoUnitError::UnsupportedAlgorithm,
            0x2 => CryptoUnitError::DataLengthError,
            0x4 => CryptoUnitError::SecureSramAccessError,
            0x8 => CryptoUnitError::AddressNotAligned,
            0x10 => CryptoUnitError::KeyLengthIncorrect,
            _ => unreachable!(),
        }
    }
    /// Clear asymmetric unit error.
    #[inline]
    pub const fn clear_asym_err(self) -> Self {
        Self(self.0 | Self::ASYM_ERR)
    }
    /// Get hash unit error.
    #[inline]
    pub const fn hash_err(self) -> CryptoUnitError {
        match (self.0 & Self::HASH_ERR) >> 8 {
            0x1 => CryptoUnitError::UnsupportedAlgorithm,
            0x2 => CryptoUnitError::DataLengthError,
            0x4 => CryptoUnitError::SecureSramAccessError,
            0x8 => CryptoUnitError::AddressNotAligned,
            0x10 => CryptoUnitError::KeyLengthIncorrect,
            _ => unreachable!(),
        }
    }
    /// Clear hash unit error.
    #[inline]
    pub const fn clear_hash_err(self) -> Self {
        Self(self.0 | Self::HASH_ERR)
    }
    /// Get symmetric unit error.
    #[inline]
    pub const fn sym_err(self) -> CryptoUnitError {
        match self.0 & Self::SYM_ERR {
            0x1 => CryptoUnitError::UnsupportedAlgorithm,
            0x2 => CryptoUnitError::DataLengthError,
            0x4 => CryptoUnitError::SecureSramAccessError,
            0x8 => CryptoUnitError::AddressNotAligned,
            0x10 => CryptoUnitError::KeyLengthIncorrect,
            _ => unreachable!(),
        }
    }
    /// Clear symmetric unit error.
    #[inline]
    pub const fn clear_sym_err(self) -> Self {
        Self(self.0 | Self::SYM_ERR)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CryptoAlgorithm, CryptoUnitError, IntCtrl, IntStatus, RegisterBlock, TaskCtrl, TaskError,
        TaskStatus,
    };
    use core::mem::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, int_ctrl), 0x0);
        assert_eq!(offset_of!(RegisterBlock, int_status), 0x4);
        assert_eq!(offset_of!(RegisterBlock, task_addr), 0x8);
        assert_eq!(offset_of!(RegisterBlock, task_ctrl), 0xC);
        assert_eq!(offset_of!(RegisterBlock, task_status), 0x10);
        assert_eq!(offset_of!(RegisterBlock, task_err), 0x14);
        assert_eq!(offset_of!(RegisterBlock, version), 0xFFC);
    }

    #[test]
    fn struct_int_ctrl_functions() {
        let mut ctrl = IntCtrl(0).enable_asym_int();
        assert!(ctrl.is_asym_int_enabled());
        assert_eq!(ctrl.0, 0x0000_0004);

        ctrl = ctrl.disable_asym_int();
        assert!(!ctrl.is_asym_int_enabled());
        assert_eq!(ctrl.0, 0x0000_0000);

        ctrl = IntCtrl(0).enable_hash_int();
        assert!(ctrl.is_hash_int_enabled());
        assert_eq!(ctrl.0, 0x0000_0002);

        ctrl = ctrl.disable_hash_int();
        assert!(!ctrl.is_hash_int_enabled());
        assert_eq!(ctrl.0, 0x0000_0000);

        ctrl = IntCtrl(0).enable_sym_int();
        assert!(ctrl.is_sym_int_enabled());
        assert_eq!(ctrl.0, 0x0000_0001);

        ctrl = ctrl.disable_sym_int();
        assert!(!ctrl.is_sym_int_enabled());
        assert_eq!(ctrl.0, 0x0000_0000);
    }

    #[test]
    fn struct_int_status_functions() {
        let mut status = IntStatus(0x0).clear_asym_int_status();
        assert!(status.is_asym_int_pending());
        assert_eq!(status.0, 0x0000_0004);

        status = IntStatus(0x0).clear_hash_int_status();
        assert!(status.is_hash_int_pending());
        assert_eq!(status.0, 0x0000_0002);

        status = IntStatus(0x0).clear_sym_int_status();
        assert!(status.is_sym_int_pending());
        assert_eq!(status.0, 0x0000_0001);
    }

    #[test]
    fn struct_task_ctrl_functions() {
        let mut ctrl = TaskCtrl(0).start_task();
        assert!(ctrl.load_task_bit());
        assert_eq!(ctrl.0, 0x8000_0000);

        ctrl = ctrl.set_load_task_bit(false);
        assert!(!ctrl.load_task_bit());
        assert_eq!(ctrl.0, 0x0000_0000);

        ctrl = ctrl.set_load_task_bit(true);
        assert!(ctrl.load_task_bit());
        assert_eq!(ctrl.0, 0x8000_0000);

        for i in 0..17 {
            let (alg, reg_val) = match i {
                0 => (CryptoAlgorithm::AesEcb, 0x0000_0000),
                1 => (CryptoAlgorithm::AesCbc, 0x0000_0001),
                2 => (CryptoAlgorithm::AesCtr, 0x0000_0002),
                3 => (CryptoAlgorithm::DesEcb, 0x0000_0010),
                4 => (CryptoAlgorithm::DesCbc, 0x0000_0011),
                5 => (CryptoAlgorithm::TdesEcb, 0x0000_0020),
                6 => (CryptoAlgorithm::TdesCbc, 0x0000_0021),
                7 => (CryptoAlgorithm::Rsa, 0x0000_0030),
                8 => (CryptoAlgorithm::Sha1, 0x0000_0040),
                9 => (CryptoAlgorithm::Sha224, 0x0000_0041),
                10 => (CryptoAlgorithm::Sha256, 0x0000_0042),
                11 => (CryptoAlgorithm::Sha384, 0x0000_0043),
                12 => (CryptoAlgorithm::Sha512, 0x0000_0044),
                13 => (CryptoAlgorithm::Md5, 0x0000_0045),
                14 => (CryptoAlgorithm::HmacSha1, 0x0000_0046),
                15 => (CryptoAlgorithm::HmacSha256, 0x0000_0047),
                16 => (CryptoAlgorithm::Trng, 0x0000_0050),
                _ => unreachable!(),
            };

            ctrl = TaskCtrl(0).set_current_algorithm(alg);
            assert_eq!(ctrl.current_algorithm(), alg);
            assert_eq!(ctrl.0, reg_val);
        }
    }

    #[test]
    fn struct_task_status_functions() {
        for i in 0..17 {
            let (alg, reg_val) = match i {
                0 => (CryptoAlgorithm::AesEcb, 0x0000_0000),
                1 => (CryptoAlgorithm::AesCbc, 0x0000_0001),
                2 => (CryptoAlgorithm::AesCtr, 0x0000_0002),
                3 => (CryptoAlgorithm::DesEcb, 0x0000_0010),
                4 => (CryptoAlgorithm::DesCbc, 0x0000_0011),
                5 => (CryptoAlgorithm::TdesEcb, 0x0000_0020),
                6 => (CryptoAlgorithm::TdesCbc, 0x0000_0021),
                7 => (CryptoAlgorithm::Rsa, 0x0000_0030),
                8 => (CryptoAlgorithm::Sha1, 0x0000_0040),
                9 => (CryptoAlgorithm::Sha224, 0x0000_0041),
                10 => (CryptoAlgorithm::Sha256, 0x0000_0042),
                11 => (CryptoAlgorithm::Sha384, 0x0000_0043),
                12 => (CryptoAlgorithm::Sha512, 0x0000_0044),
                13 => (CryptoAlgorithm::Md5, 0x0000_0045),
                14 => (CryptoAlgorithm::HmacSha1, 0x0000_0046),
                15 => (CryptoAlgorithm::HmacSha256, 0x0000_0047),
                16 => (CryptoAlgorithm::Trng, 0x0000_0050),
                _ => unreachable!(),
            };

            let mut status = TaskStatus(reg_val);
            assert_eq!(status.sym_task(), alg);
            status = TaskStatus(reg_val << 8);
            assert_eq!(status.hash_task(), alg);
            status = TaskStatus(reg_val << 16);
            assert_eq!(status.asym_task(), alg);
        }
    }

    #[test]
    fn struct_task_error_functions() {
        for i in 0..5 {
            let (err, reg_val) = match i {
                0 => (CryptoUnitError::UnsupportedAlgorithm, 0x0000_0001),
                1 => (CryptoUnitError::DataLengthError, 0x0000_0002),
                2 => (CryptoUnitError::SecureSramAccessError, 0x0000_0004),
                3 => (CryptoUnitError::AddressNotAligned, 0x0000_0008),
                4 => (CryptoUnitError::KeyLengthIncorrect, 0x0000_0010),
                _ => unreachable!(),
            };

            let mut task_err = TaskError(reg_val);
            assert_eq!(task_err.sym_err(), err);
            task_err = task_err.clear_sym_err();
            assert_eq!(task_err.0, 0x0000_00FF);

            task_err = TaskError(reg_val << 8);
            assert_eq!(task_err.hash_err(), err);
            task_err = task_err.clear_hash_err();
            assert_eq!(task_err.0, 0x0000_FF00);

            task_err = TaskError(reg_val << 16);
            assert_eq!(task_err.asym_err(), err);
            task_err = task_err.clear_asym_err();
            assert_eq!(task_err.0, 0x00FF_0000);
        }
    }
}
