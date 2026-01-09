//! UART configuration.

use embedded_time::rate::Baud;
use uart16550::{CharLen, PARITY};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum DataBits {
    Five = CharLen::FIVE as u8,
    Six = CharLen::SIX as u8,
    Seven = CharLen::SEVEN as u8,
    Eight = CharLen::EIGHT as u8,
}

impl DataBits {
    pub(crate) const fn to_char_len(self) -> CharLen {
        match self {
            DataBits::Five => CharLen::FIVE,
            DataBits::Six => CharLen::SIX,
            DataBits::Seven => CharLen::SEVEN,
            DataBits::Eight => CharLen::EIGHT,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum StopBits {
    One,
    Two,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum Parity {
    None = PARITY::NONE as u8,
    Even = PARITY::EVEN as u8,
    Odd = PARITY::ODD as u8,
}

impl Parity {
    pub(crate) const fn to_parity(self) -> PARITY {
        match self {
            Parity::None => PARITY::NONE,
            Parity::Even => PARITY::EVEN,
            Parity::Odd => PARITY::ODD,
        }
    }
}

pub struct UartConfig {
    pub baud_rate: Baud,
    pub data_bits: DataBits,
    pub stop_bits: StopBits,
    pub parity: Parity,
}

impl Default for UartConfig {
    fn default() -> Self {
        Self {
            baud_rate: Baud(115200_u32),
            data_bits: DataBits::Eight,
            stop_bits: StopBits::One,
            parity: Parity::None,
        }
    }
}
