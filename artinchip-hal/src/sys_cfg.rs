//! System Configuration (SYSCFG).
//!
//! ArtInChip SoCs integrate two revisions of the SYSCFG peripheral:
//! - [`register_v1`]: SYSCFG v1 (D12x / D13x), providing `LDO25_CFG`, `LDO18_CFG`, and `LDO1x_CFG`.
//! - [`register_v2`]: SYSCFG v2 (D21x), providing `LDO30_CFG`, `LDO25_CFG`, `LDO1x_CFG` (`VCC_DRAM`),
//!   `DDR_VREF`, `DDR_REXT`, `USB1_REXT`, `PSEN_CFG`, `SRAM_CLK_CFG`, `USB0_CFG`,
//!   `GMAC0_CFG0`, `GMAC1_CFG`, and `SYSCFG_VER`.

mod common;
mod instance;
pub mod register_v1;
pub mod register_v2;

pub use common::*;
pub use instance::{SocSysCfg, SysCfg, V1, V2};
pub use register_v1::{Ldo18Config, Ldo18Voltage, Ldo25Config, Ldo25Voltage};
pub use register_v2::{DdrLdo1xConfig, DdrLdo1xVoltage, Ldo1xVoltage};
