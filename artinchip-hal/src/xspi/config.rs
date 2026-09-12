//! XSPI PSRAM configuration.

use super::register::{CsSel, PinDriveStrength, PinPull};
use crate::sys_cfg::Ldo18Voltage;
use embedded_hal::spi::Phase;
use embedded_time::rate::Hertz;

/// SoC family a PSRAM configuration targets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chip {
    /// ArtInChip D13x.
    D13x,
}

/// PSRAM part a configuration targets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PsramModel {
    /// AP Memory APS3208K, 8 MiB, OPI DDR.
    Aps3208K,
}

/// Pull/drive strength of a single XSPI pin.
#[derive(Clone, Copy, Debug)]
pub struct PinConfig {
    /// Pull-up/pull-down state.
    pub pull: PinPull,
    /// Output drive strength.
    pub drv: PinDriveStrength,
}

/// Per-pin settings for one chip select's IO configuration.
#[derive(Clone, Copy, Debug)]
pub struct PsramIoConfig {
    /// Data lines `DQ0..DQ7`.
    pub dq: PinConfig,
    /// Chip select pin.
    pub cs: PinConfig,
    /// Data strobe pin.
    pub dqs: PinConfig,
    /// Clock pin.
    pub ck: PinConfig,
    /// Inverted clock pin.
    pub ckn: PinConfig,
    /// Data mask pin.
    pub dm: PinConfig,
}

/// Register-level parameters for one PSRAM part.
#[derive(Clone, Copy, Debug)]
pub struct PsramConfig {
    /// PSRAM interface clock; the DDR bus runs at twice this.
    pub clock_hz: Hertz,
    /// Base of the XIP window the controller decodes.
    pub xip_base: usize,
    /// Bytes of the XIP window used by the phase-training pattern test.
    pub training_len: usize,
    /// Chip selects to bring up.
    pub chip_selects: &'static [CsSel],
    /// `XSPI_TCR` chip-select write hold time, in cycles.
    pub cs_wr_hold: u8,
    /// `XSPI_TCR` chip-select read hold time, in cycles.
    pub cs_rd_hold: u8,
    /// `XSPI_TCR` SPI clock/data phase.
    pub clk_pha: Phase,
    /// `LDO18_CFG` voltage for the PSRAM I/O rail.
    pub ldo18_voltage: Ldo18Voltage,
    /// Per-chip-select IO configuration.
    pub io: PsramIoConfig,
    /// Address width in bytes.
    pub addr_width: u8,
    /// Mode-register writes as `(address, value)` using command `0xC0`.
    pub mode_registers: [(u8, u8); 2],
    /// XIP write prototype: `(command, dummy cycles, clock cycles)`.
    pub xip_write: (u8, u8, u32),
    /// XIP read prototype: `(command, dummy cycles, clock cycles)`.
    pub xip_read: (u8, u8, u32),
}

impl PsramConfig {
    /// OPI APS3208K 8 M at 198 MHz.
    pub const D13X_APS3208K_OPI: Self = Self {
        clock_hz: Hertz(198_000_000),
        xip_base: 0x4000_0000,
        training_len: 64 * 1024,
        chip_selects: &[CsSel::Cs0, CsSel::Cs1],
        cs_wr_hold: 8,
        cs_rd_hold: 2,
        clk_pha: Phase::CaptureOnSecondTransition,
        ldo18_voltage: Ldo18Voltage::V1_92,
        io: PsramIoConfig {
            dq: PinConfig {
                pull: PinPull::Disabled,
                drv: PinDriveStrength::Level2,
            },
            cs: PinConfig {
                pull: PinPull::PullUp,
                drv: PinDriveStrength::Level6,
            },
            dqs: PinConfig {
                pull: PinPull::Disabled,
                drv: PinDriveStrength::Level6,
            },
            ck: PinConfig {
                pull: PinPull::Disabled,
                drv: PinDriveStrength::Level5,
            },
            ckn: PinConfig {
                pull: PinPull::Disabled,
                drv: PinDriveStrength::Level3,
            },
            dm: PinConfig {
                pull: PinPull::PullDown,
                drv: PinDriveStrength::Level6,
            },
        },
        addr_width: 3,
        mode_registers: [(0x00, 0x19), (0x04, 0x80)],
        xip_write: (0x80, 2, 2),
        xip_read: (0x00, 6, 3),
    };

    /// Configuration for a `chip`/`model` pair.
    pub const fn chip_model(chip: Chip, model: PsramModel) -> Self {
        match (chip, model) {
            (Chip::D13x, PsramModel::Aps3208K) => Self::D13X_APS3208K_OPI,
        }
    }
}
