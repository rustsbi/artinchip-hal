//! GPIO register blocks and registers.

use volatile_register::{RO, RW, WO};

/// General Purpose Input Output register block.
#[repr(C)]
pub struct RegisterBlock {
    /// GPIO group PA, PB until PU.
    pub groups: [GpioGroup; 15],
    _reserved0: [u8; 252],
    /// GPIO version register (`VERSION`).
    #[doc(alias = "VERSION")]
    pub version: RO<u32>,
}

/// GPIO group register block.
#[repr(C)]
pub struct GpioGroup {
    /// Input state register (`GEN_IN_STA`).
    ///
    /// This register is valid only under the following conditions:
    /// - When `PIN_FUN` = 1 and `GEN_IN_EN` = 1.
    /// - When `PIN_FUN` = 2~15 and `SPE_IE_FORCE` = 1.
    #[doc(alias = "GEN_IN_STA")]
    pub input_state: RO<InputState>,
    /// Output configuration register (`GEN_OUT_CFG`).
    ///
    /// This register is valid only when:
    /// - `PIN_FUN` = 1 and `GEN_OUT_EN` = 1.
    #[doc(alias = "GEN_OUT_CFG")]
    pub output_config: RW<OutputConfig>,
    /// Interrupt enable register (`GEN_IRQ_EN`).
    ///
    /// This register is valid only when:
    /// - `PIN_FUN` = 1 and `GEN_IN_EN` = 1.
    #[doc(alias = "GEN_IRQ_EN")]
    pub interrupt_enable: RW<InterruptEnable>,
    /// Interrupt state register (`GEN_IRQ_STA`).
    ///
    /// This register is valid only when all the following conditions are met:
    /// - `PIN_FUN` = 1
    /// - `GEN_IN_EN` = 1
    /// - `IRQ_EN` = 1
    #[doc(alias = "GEN_IRQ_STA")]
    pub interrupt_state: RW<InterruptState>,
    /// Clear output register (`GEN_OUT_CLR`).
    #[doc(alias = "GEN_OUT_CLR")]
    pub output_clear: WO<OutputClear>,
    /// Set output register (`GEN_OUT_SET`).
    #[doc(alias = "GEN_OUT_SET")]
    pub output_set: WO<OutputSet>,
    /// Toggle output register (`GEN_OUT_TOG`).
    #[doc(alias = "GEN_OUT_TOG")]
    pub output_toggle: WO<OutputToggle>,
    _reserved0: [u8; 100],
    /// Pin configuration register (`PIN_CFG`).
    #[doc(alias = "PIN_CFG")]
    pub pin_config: [RW<PinConfig>; 32],
}

/// Input state register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct InputState(u32);

impl InputState {
    /// Check if the pin is high.
    #[inline]
    pub const fn is_high(self, pin: usize) -> bool {
        assert!(pin < 32, "Pin index out of range (expected 0..=31)");
        (self.0 & (1 << pin)) != 0
    }
    /// Check if the pin is low.
    #[inline]
    pub const fn is_low(self, pin: usize) -> bool {
        assert!(pin < 32, "Pin index out of range (expected 0..=31)");
        (self.0 & (1 << pin)) == 0
    }
}

/// Output configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OutputConfig(u32);

impl OutputConfig {
    /// Check if the pin is high.
    #[inline]
    pub const fn is_high(self, pin: usize) -> bool {
        assert!(pin < 32, "Pin index out of range (expected 0..=31)");
        (self.0 & (1 << pin)) != 0
    }
    /// Check if the pin is low.
    #[inline]
    pub const fn is_low(self, pin: usize) -> bool {
        assert!(pin < 32, "Pin index out of range (expected 0..=31)");
        (self.0 & (1 << pin)) == 0
    }
    /// Set the pin to high.
    #[inline]
    pub const fn set_high(self, pin: usize) -> Self {
        assert!(pin < 32, "Pin index out of range (expected 0..=31)");
        Self(self.0 | (1 << pin))
    }
    /// Set the pin to low.
    #[inline]
    pub const fn set_low(self, pin: usize) -> Self {
        assert!(pin < 32, "Pin index out of range (expected 0..=31)");
        Self(self.0 & !(1 << pin))
    }
}

/// Interrupt enable register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct InterruptEnable(u32);

impl InterruptEnable {
    /// Check if the pin interrupt is enabled.
    #[inline]
    pub const fn is_interrupt_enabled(self, pin: usize) -> bool {
        assert!(pin < 32, "Pin index out of range (expected 0..=31)");
        (self.0 & (1 << pin)) != 0
    }
    /// Enable the pin interrupt.
    #[inline]
    pub const fn enable_interrupt(self, pin: usize) -> Self {
        assert!(pin < 32, "Pin index out of range (expected 0..=31)");
        Self(self.0 | (1 << pin))
    }
    /// Disable the pin interrupt.
    #[inline]
    pub const fn disable_interrupt(self, pin: usize) -> Self {
        assert!(pin < 32, "Pin index out of range (expected 0..=31)");
        Self(self.0 & !(1 << pin))
    }
}

/// Interrupt state register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct InterruptState(u32);

impl InterruptState {
    /// Check if the pin interrupt is pending.
    #[inline]
    pub const fn is_interrupt_pending(self, pin: usize) -> bool {
        assert!(pin < 32, "Pin index out of range (expected 0..=31)");
        (self.0 & (1 << pin)) != 0
    }
    /// Clear the pin interrupt.
    #[inline]
    pub const fn clear_interrupt(self, pin: usize) -> Self {
        assert!(pin < 32, "Pin index out of range (expected 0..=31)");
        Self(self.0 | (1 << pin))
    }
}

/// Clear output register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OutputClear(u32);

impl OutputClear {
    /// Clear the pin output (set low).
    #[inline]
    pub const fn clear_output(self, pin: usize) -> Self {
        assert!(pin < 32, "Pin index out of range (expected 0..=31)");
        Self(self.0 | (1 << pin))
    }
}

/// Set output register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OutputSet(u32);

impl OutputSet {
    /// Set the pin output (set high).
    #[inline]
    pub const fn set_output(self, pin: usize) -> Self {
        assert!(pin < 32, "Pin index out of range (expected 0..=31)");
        Self(self.0 | (1 << pin))
    }
}

/// Toggle output register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct OutputToggle(u32);

impl OutputToggle {
    /// Toggle the pin output.
    #[inline]
    pub const fn toggle_output(self, pin: usize) -> Self {
        assert!(pin < 32, "Pin index out of range (expected 0..=31)");
        Self(self.0 | (1 << pin))
    }
}

/// General function interrupt detection mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GeneralIrqMode {
    /// Falling edge detection.
    FallingEdge,
    /// Rising edge detection.
    RisingEdge,
    /// Low level detection.
    LowLevel,
    /// High level detection.
    HighLevel,
    /// Both edge detection.
    BothEdges,
}

/// Pin pull-up/pull-down configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PinPull {
    /// Disabled (high-impedance state).
    Disabled,
    /// Pull-down with 33 KΩ to GND.
    PullDown = 2,
    /// Pull-up with 33 KΩ to VCC-IO.
    PullUp = 3,
}

/// Pin output drive strength configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PinDriveStrength {
    /// Level 0 (weakest drive).
    /// - 3.3V: 180Ω
    /// - 1.8V: 300Ω
    Level0,
    /// Level 1.
    /// - 3.3V: 90Ω
    /// - 1.8V: 150Ω
    Level1,
    /// Level 2.
    /// - 3.3V: 60Ω
    /// - 1.8V: 100Ω
    Level2,
    /// Level 3.
    /// - 3.3V: 45Ω
    /// - 1.8V: 75Ω
    Level3,
    /// Level 4.
    /// - 3.3V: 36Ω
    /// - 1.8V: 60Ω
    Level4,
    /// Level 5.
    /// - 3.3V: 30Ω
    /// - 1.8V: 50Ω
    Level5,
    /// Level 6.
    /// - 3.3V: 26Ω
    /// - 1.8V: 43Ω
    Level6,
    /// Level 7 (strongest drive).
    /// - 3.3V: 23Ω
    /// - 1.8V: 38Ω
    Level7,
}

/// Pin configuration register.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PinConfig(u32);

impl PinConfig {
    const GEN_IN_DB1_POINT: u32 = 0xF << 28;
    const GEN_IN_DB1_SAMP: u32 = 0xF << 24;
    const GEN_IN_DB0_POINT: u32 = 0xF << 20;
    const SPE_IE_FORCE: u32 = 0x1 << 18;
    const GEN_OE: u32 = 0x1 << 17;
    const GEN_IE: u32 = 0x1 << 16;
    const GEN_IRQ_MODE: u32 = 0x7 << 12;
    const PIN_PULL: u32 = 0x3 << 8;
    const PIN_DRV: u32 = 0x7 << 4;
    const PIN_FUN: u32 = 0xF;

    /// Set debounce reference point for input stage 1 (`GEN_IN_DB1_POINT`).
    ///
    /// Only effective when `PIN_FUN` = 1 and `GEN_IE` = 1.
    /// Setting this value to 0 disables debounce for this stage.
    #[doc(alias = "GEN_IN_DB1_POINT")]
    #[inline]
    pub const fn set_db1_point(self, val: u8) -> Self {
        assert!(
            val < 16,
            "Value out of bounds for GEN_IN_DB1_POINT (expected 0..=15)"
        );
        Self((self.0 & !Self::GEN_IN_DB1_POINT) | (Self::GEN_IN_DB1_POINT & ((val as u32) << 28)))
    }
    /// Get debounce reference point for input stage 1.
    #[inline]
    pub const fn db1_point(self) -> u8 {
        ((self.0 & Self::GEN_IN_DB1_POINT) >> 28) as u8
    }
    /// Set debounce sample period for input stage 1 (`GEN_IN_DB1_SAMP`).
    ///
    /// Only effective when `PIN_FUN` = 1 and `GEN_IE` = 1.
    /// The debounce sampling period is 2 ^ (SAMP + 1) APB0 clock cycles (100 MHz).
    #[doc(alias = "GEN_IN_DB1_SAMP")]
    #[inline]
    pub const fn set_db1_sample(self, val: u8) -> Self {
        assert!(
            val < 16,
            "Value out of bounds for GEN_IN_DB1_SAMP (expected 0..=15)"
        );
        Self((self.0 & !Self::GEN_IN_DB1_SAMP) | (Self::GEN_IN_DB1_SAMP & ((val as u32) << 24)))
    }
    /// Get debounce sample period for input stage 1.
    #[inline]
    pub const fn db1_sample(self) -> u8 {
        ((self.0 & Self::GEN_IN_DB1_SAMP) >> 24) as u8
    }
    /// Set debounce sample count for input stage 0 (`GEN_IN_DB0_POINT`).
    ///
    /// Only effective when `PIN_FUN` = 1 and `GEN_IE` = 1.
    /// The debounce sampling count for this stage is one APB0 clock cycle (100 MHz).
    /// Setting this value to 0 disables debounce for this stage.
    #[doc(alias = "GEN_IN_DB0_POINT")]
    #[inline]
    pub const fn set_db0_point(self, val: u8) -> Self {
        assert!(
            val < 16,
            "Value out of bounds for GEN_IN_DB0_POINT (expected 0..=15)"
        );
        Self((self.0 & !Self::GEN_IN_DB0_POINT) | (Self::GEN_IN_DB0_POINT & ((val as u32) << 20)))
    }
    /// Get debounce sample point for input stage 0.
    #[inline]
    pub const fn db0_point(self) -> u8 {
        ((self.0 & Self::GEN_IN_DB0_POINT) >> 20) as u8
    }
    /// Enable special function input forcibly (`SPE_IE_FORCE`).
    ///
    /// Only effective when `PIN_FUN` is configured to special function modes (2-6).
    /// Primarily intended for debugging and validation purposes.
    #[doc(alias = "SPE_IE_FORCE")]
    #[inline]
    pub const fn enable_special_input_force(self) -> Self {
        Self(self.0 | Self::SPE_IE_FORCE)
    }
    /// Disable forced special function input.
    #[inline]
    pub const fn disable_special_input_force(self) -> Self {
        Self(self.0 & !Self::SPE_IE_FORCE)
    }
    /// Check if forced special function input is enabled.
    #[inline]
    pub const fn is_special_input_force_enabled(self) -> bool {
        (self.0 & Self::SPE_IE_FORCE) != 0
    }
    /// Enable general function output (`GEN_OE`).
    ///
    /// Only effective when `PIN_FUN` = 1.
    #[doc(alias = "GEN_OE")]
    #[inline]
    pub const fn enable_general_output(self) -> Self {
        Self(self.0 | Self::GEN_OE)
    }
    /// Disable general function output.
    #[inline]
    pub const fn disable_general_output(self) -> Self {
        Self(self.0 & !Self::GEN_OE)
    }
    /// Check if general function output is enabled.
    #[inline]
    pub const fn is_general_output_enabled(self) -> bool {
        (self.0 & Self::GEN_OE) != 0
    }
    /// Enable general function input (`GEN_IE`).
    ///
    /// Only effective when `PIN_FUN` = 1.
    #[doc(alias = "GEN_IE")]
    #[inline]
    pub const fn enable_general_input(self) -> Self {
        Self(self.0 | Self::GEN_IE)
    }
    /// Disable general function input.
    #[inline]
    pub const fn disable_general_input(self) -> Self {
        Self(self.0 & !Self::GEN_IE)
    }
    /// Check if general function input is enabled.
    #[inline]
    pub const fn is_general_input_enabled(self) -> bool {
        (self.0 & Self::GEN_IE) != 0
    }
    /// Set general function interrupt detection mode (`GEN_IRQ_MODE`).
    ///
    /// Only effective when `PIN_FUN` = 1 and `GEN_IE` = 1.
    #[doc(alias = "GEN_IRQ_MODE")]
    #[inline]
    pub const fn set_general_irq_mode(self, mode: GeneralIrqMode) -> Self {
        Self((self.0 & !Self::GEN_IRQ_MODE) | (Self::GEN_IRQ_MODE & ((mode as u32) << 12)))
    }
    /// Get general function interrupt detection mode.
    #[inline]
    pub const fn general_irq_mode(self) -> GeneralIrqMode {
        match (self.0 & Self::GEN_IRQ_MODE) >> 12 {
            0 => GeneralIrqMode::FallingEdge,
            1 => GeneralIrqMode::RisingEdge,
            2 => GeneralIrqMode::LowLevel,
            3 => GeneralIrqMode::HighLevel,
            4 => GeneralIrqMode::BothEdges,
            _ => unreachable!(),
        }
    }
    /// Set pin pull-up/pull-down configuration (`PIN_PULL`).
    #[doc(alias = "PIN_PULL")]
    #[inline]
    pub const fn set_pin_pull(self, pull: PinPull) -> Self {
        Self((self.0 & !Self::PIN_PULL) | (Self::PIN_PULL & ((pull as u32) << 8)))
    }
    /// Get pin pull-up/pull-down configuration.
    #[inline]
    pub const fn pin_pull(self) -> PinPull {
        match (self.0 & Self::PIN_PULL) >> 8 {
            0 => PinPull::Disabled,
            2 => PinPull::PullDown,
            3 => PinPull::PullUp,
            _ => unreachable!(),
        }
    }
    /// Set pin output drive strength configuration (`PIN_DRV`).
    #[doc(alias = "PIN_DRV")]
    #[inline]
    pub const fn set_drive_strength(self, strength: PinDriveStrength) -> Self {
        Self((self.0 & !Self::PIN_DRV) | (Self::PIN_DRV & ((strength as u32) << 4)))
    }
    /// Get pin output drive strength configuration.
    #[inline]
    pub const fn drive_strength(self) -> PinDriveStrength {
        match ((self.0 & Self::PIN_DRV) >> 4) as u8 {
            0 => PinDriveStrength::Level0,
            1 => PinDriveStrength::Level1,
            2 => PinDriveStrength::Level2,
            3 => PinDriveStrength::Level3,
            4 => PinDriveStrength::Level4,
            5 => PinDriveStrength::Level5,
            6 => PinDriveStrength::Level6,
            7 => PinDriveStrength::Level7,
            _ => unreachable!(),
        }
    }
    /// Set pin function (`PIN_FUN`).
    #[doc(alias = "PIN_FUN")]
    #[inline]
    pub const fn set_pin_func(self, func: u8) -> Self {
        assert!(
            func < 16,
            "Function value out of bounds for PIN_FUN (expected 0..=15)"
        );
        Self((self.0 & !Self::PIN_FUN) | (Self::PIN_FUN & (func as u32)))
    }
    /// Set pin function.
    #[inline]
    pub const fn pin_func(self) -> u8 {
        (self.0 & Self::PIN_FUN) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::{
        GeneralIrqMode, GpioGroup, InputState, InterruptEnable, InterruptState, OutputClear,
        OutputConfig, OutputSet, OutputToggle, PinConfig, PinDriveStrength, PinPull, RegisterBlock,
    };
    use core::mem::{offset_of, size_of};

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, groups), 0x0);
        assert_eq!(offset_of!(RegisterBlock, version), 0xffc);
    }

    #[test]
    fn struct_gpio_group_offset() {
        assert_eq!(size_of::<GpioGroup>(), 0x100);
        assert_eq!(offset_of!(GpioGroup, input_state), 0x0);
        assert_eq!(offset_of!(GpioGroup, output_config), 0x4);
        assert_eq!(offset_of!(GpioGroup, interrupt_enable), 0x8);
        assert_eq!(offset_of!(GpioGroup, interrupt_state), 0xc);
        assert_eq!(offset_of!(GpioGroup, output_clear), 0x10);
        assert_eq!(offset_of!(GpioGroup, output_set), 0x14);
        assert_eq!(offset_of!(GpioGroup, output_toggle), 0x18);
        assert_eq!(offset_of!(GpioGroup, pin_config), 0x80);
    }

    #[test]
    fn struct_input_state_functions() {
        let mut val = InputState(0x0000_FFFF);
        for pin in 0..16 {
            assert!(val.is_high(pin));
            assert!(!val.is_low(pin));
        }
        val = InputState(0xFFFF_0000);
        for pin in 16..32 {
            assert!(val.is_high(pin));
            assert!(!val.is_low(pin));
        }
    }

    #[test]
    fn struct_output_config_functions() {
        let mut val = OutputConfig(0x0);

        for pin in 0..16 {
            val = val.set_high(pin).set_low(pin + 16);
        }
        assert_eq!(val.0, 0x0000_FFFF);
        for pin in 0..16 {
            assert!(val.is_high(pin));
            assert!(val.is_low(pin + 16));
        }
    }

    #[test]
    fn struct_interrupt_enable_functions() {
        let mut val = InterruptEnable(0x0);

        for pin in 0..16 {
            val = val.enable_interrupt(pin).disable_interrupt(pin + 16);
        }
        assert_eq!(val.0, 0x0000_FFFF);
        for pin in 0..16 {
            assert!(val.is_interrupt_enabled(pin));
            assert!(!val.is_interrupt_enabled(pin + 16));
        }
    }

    #[test]
    fn struct_interrupt_state_functions() {
        let mut val = InterruptState(0x0000_FFFF);

        for pin in 0..16 {
            assert!(val.is_interrupt_pending(pin));
            assert!(!val.is_interrupt_pending(pin + 16));
        }
        val = InterruptState(0x0);
        for pin in 16..32 {
            val = val.clear_interrupt(pin);
        }
        assert_eq!(val.0, 0xFFFF_0000);
    }

    #[test]
    fn struct_output_clear_functions() {
        let mut val = OutputClear(0x0);

        for pin in 0..16 {
            val = val.clear_output(pin)
        }
        assert_eq!(val.0, 0x0000_FFFF);
    }

    #[test]
    fn struct_output_set_functions() {
        let mut val = OutputSet(0x0);

        for pin in 0..16 {
            val = val.set_output(pin)
        }
        assert_eq!(val.0, 0x0000_FFFF);
    }

    #[test]
    fn struct_output_toggle_functions() {
        let mut val = OutputToggle(0x0);

        for pin in 0..16 {
            val = val.toggle_output(pin)
        }
        assert_eq!(val.0, 0x0000_FFFF);
    }

    #[test]
    fn struct_pin_config_functions() {
        let mut val = PinConfig(0x0);

        val = val.set_db1_point(0xF);
        assert_eq!(val.db1_point(), 0xF);
        assert_eq!(val.0, 0xF000_0000);

        val = PinConfig(0x0).set_db1_sample(0xF);
        assert_eq!(val.db1_sample(), 0xF);
        assert_eq!(val.0, 0x0F00_0000);

        val = PinConfig(0x0).set_db0_point(0xF);
        assert_eq!(val.db0_point(), 0xF);
        assert_eq!(val.0, 0x00F0_0000);

        val = PinConfig(0x0).enable_special_input_force();
        assert!(val.is_special_input_force_enabled());
        assert_eq!(val.0, 0x0004_0000);

        val = val.disable_special_input_force();
        assert!(!val.is_special_input_force_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_general_output();
        assert!(val.is_general_output_enabled());
        assert_eq!(val.0, 0x0002_0000);

        val = val.disable_general_output();
        assert!(!val.is_general_output_enabled());
        assert_eq!(val.0, 0x0000_0000);

        val = val.enable_general_input();
        assert!(val.is_general_input_enabled());
        assert_eq!(val.0, 0x0001_0000);

        val = val.disable_general_input();
        assert!(!val.is_general_input_enabled());
        assert_eq!(val.0, 0x0000_0000);

        for i in 0..4 {
            let (test_mode, test_val) = match i {
                0 => (GeneralIrqMode::FallingEdge, 0x0000_0000),
                1 => (GeneralIrqMode::RisingEdge, 0x0000_1000),
                2 => (GeneralIrqMode::LowLevel, 0x0000_2000),
                3 => (GeneralIrqMode::HighLevel, 0x0000_3000),
                4 => (GeneralIrqMode::BothEdges, 0x0000_4000),
                _ => unreachable!(),
            };

            val = val.set_general_irq_mode(test_mode);
            assert_eq!(val.general_irq_mode(), test_mode);
            assert_eq!(val.0, test_val);
        }

        val = PinConfig(0x0);
        for i in 0..3 {
            let (test_pull, test_val) = match i {
                0 => (PinPull::Disabled, 0x0000_0000),
                1 => (PinPull::PullDown, 0x0000_0200),
                2 => (PinPull::PullUp, 0x0000_0300),
                _ => unreachable!(),
            };

            val = val.set_pin_pull(test_pull);
            assert_eq!(val.pin_pull(), test_pull);
            assert_eq!(val.0, test_val);
        }

        val = PinConfig(0x0);
        for i in 0..8 {
            let (test_strength, test_val) = match i {
                0 => (PinDriveStrength::Level0, 0x0000_0000),
                1 => (PinDriveStrength::Level1, 0x0000_0010),
                2 => (PinDriveStrength::Level2, 0x0000_0020),
                3 => (PinDriveStrength::Level3, 0x0000_0030),
                4 => (PinDriveStrength::Level4, 0x0000_0040),
                5 => (PinDriveStrength::Level5, 0x0000_0050),
                6 => (PinDriveStrength::Level6, 0x0000_0060),
                7 => (PinDriveStrength::Level7, 0x0000_0070),
                _ => unreachable!(),
            };

            val = val.set_drive_strength(test_strength);
            assert_eq!(val.drive_strength(), test_strength);
            assert_eq!(val.0, test_val);
        }

        val = PinConfig(0x0).set_pin_func(0xF);
        assert_eq!(val.pin_func(), 0xF);
        assert_eq!(val.0, 0x0000_000F);
    }
}
