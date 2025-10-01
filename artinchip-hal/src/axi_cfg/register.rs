//! AXI configuration register blocks and registers.

use volatile_register::{RO, RW};

/// AXI configuration register block.
#[repr(C)]
pub struct RegisterBlock {
    /// Axi configuration groups.
    pub groups: [AxiCfg; 14],
    _reserved0: [u8; 0xF1C],
    /// Axi config version register (`AXICFG_VER`).
    #[doc(alias = "AXICFG_VER")]
    pub version: RO<u32>,
}

/// Axi operation mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    Read,
    Write,
}

/// Axi port.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port {
    /// Central Process Unit.
    Cpu,
    /// Advanced High Performance Bus.
    Ahb,
    /// Display Engine.
    De,
    /// Graphics Engine.
    Ge,
    /// Video Engine.
    Ve,
    /// Digital Video Port.
    Dvp,
    /// Crypto Engine.
    Ce,
}

impl RegisterBlock {
    /// Get the AXI configuration for a specific port and mode.
    #[inline]
    pub const fn get_cfg(&self, port: Port, mode: Mode) -> &AxiCfg {
        let index = match port {
            Port::Cpu => 0,
            Port::Ahb => 1,
            Port::De => 2,
            Port::Ge => 3,
            Port::Ve => 4,
            Port::Dvp => 5,
            Port::Ce => 6,
        } + match mode {
            Mode::Read => 0,
            Mode::Write => 7,
        };
        &self.groups[index]
    }
}

/// Axi configuration register.
#[repr(C)]
pub struct AxiCfg {
    /// Axi configuration 0 register (`AXI_CFG0`).
    #[doc(alias = "AXI_CFG0")]
    pub config: RW<AxiConfig>,
    _reserved0: [u8; 0xC],
}

/// Qos selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum QosSel {
    /// Quality of service from external master.
    External,
    /// Quality of service from internal register.
    Internal,
}

/// Axi configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AxiConfig(u32);

impl AxiConfig {
    const QOS_VAL: u32 = 0xF << 24;
    const QOS_SEL: u32 = 0x1 << 16;
    const SLV_READY: u32 = 0x1 << 1;

    /// Set the quality of service (`QOS_VAL`).
    #[doc(alias = "QOS_VAL")]
    #[inline]
    pub const fn set_qos(self, qos: u8) -> Self {
        Self((self.0 & !Self::QOS_VAL) | (Self::QOS_VAL & ((qos as u32 & 0xF) << 24)))
    }
    /// Get the quality of service.
    #[inline]
    pub const fn qos(&self) -> u8 {
        ((self.0 & Self::QOS_VAL) >> 24) as u8
    }
    /// Set quality of service selection (`QOS_SEL`).
    #[doc(alias = "QOS_SEL")]
    #[inline]
    pub const fn set_qos_sel(self, sel: QosSel) -> Self {
        Self((self.0 & !Self::QOS_SEL) | (Self::QOS_SEL & ((sel as u32) << 16)))
    }
    /// Get quality of service selection.
    #[inline]
    pub const fn qos_sel(&self) -> QosSel {
        if (self.0 & Self::QOS_SEL) != 0 {
            QosSel::Internal
        } else {
            QosSel::External
        }
    }
    /// Enable slave ready (`SLV_READY`).
    #[doc(alias = "SLV_READY")]
    #[inline]
    pub const fn enable_slave_ready(self) -> Self {
        Self(self.0 | Self::SLV_READY)
    }
    /// Disable slave ready.
    #[inline]
    pub const fn disable_slave_ready(self) -> Self {
        Self(self.0 & !Self::SLV_READY)
    }
    /// Check if slave ready is enabled.
    #[inline]
    pub const fn is_slave_ready_enabled(&self) -> bool {
        (self.0 & Self::SLV_READY) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::{AxiCfg, AxiConfig, QosSel, RegisterBlock};
    use core::mem::{offset_of, size_of};

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, groups), 0x00);
        assert_eq!(size_of::<AxiCfg>(), 0x10);
        assert_eq!(offset_of!(RegisterBlock, version), 0xFFC);
    }

    #[test]
    fn struct_register_block_functions() {
        let block = RegisterBlock {
            groups: core::array::from_fn(|_| AxiCfg {
                config: unsafe { core::mem::zeroed() },
                _reserved0: [0; 0xC],
            }),
            _reserved0: [0; 0xF1C],
            version: unsafe { core::mem::zeroed() },
        };

        fn usize_to_port(i: usize) -> super::Port {
            match i {
                0 => super::Port::Cpu,
                1 => super::Port::Ahb,
                2 => super::Port::De,
                3 => super::Port::Ge,
                4 => super::Port::Ve,
                5 => super::Port::Dvp,
                6 => super::Port::Ce,
                _ => panic!("Invalid port index"),
            }
        }

        for i in 0..7 {
            let port = usize_to_port(i);
            assert!(core::ptr::eq(
                block.get_cfg(port, super::Mode::Read),
                &block.groups[i]
            ));
            assert!(core::ptr::eq(
                block.get_cfg(port, super::Mode::Write),
                &block.groups[i + 7]
            ));
        }
    }

    #[test]
    fn struct_axiconfig_functions() {
        let mut config = AxiConfig(0).set_qos(0xF);
        assert_eq!(config.qos(), 0xF);
        assert_eq!(config.0, 0x0F00_0000);

        config = AxiConfig(0).set_qos_sel(QosSel::Internal);
        assert_eq!(config.qos_sel(), QosSel::Internal);
        assert_eq!(config.0, 0x0001_0000);

        config = config.set_qos_sel(QosSel::External);
        assert_eq!(config.qos_sel(), QosSel::External);
        assert_eq!(config.0, 0x0000_0000);

        config = AxiConfig(0).enable_slave_ready();
        assert!(config.is_slave_ready_enabled());
        assert_eq!(config.0, 0x0000_0002);

        config = config.disable_slave_ready();
        assert!(!config.is_slave_ready_enabled());
        assert_eq!(config.0, 0x0000_0000);
    }
}
