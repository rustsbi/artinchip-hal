//! I2C configuration.

use super::register::SpeedMode;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum Role {
    Master,
    Slave,
}

pub struct I2cConfig {
    pub role: Role,
    pub speed_mode: SpeedMode,
}

impl Default for I2cConfig {
    fn default() -> Self {
        Self {
            role: Role::Master,
            speed_mode: SpeedMode::Fast,
        }
    }
}
