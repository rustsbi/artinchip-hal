//! DMA request device IDs.

#[cfg(not(feature = "d21x"))]
pub use d13x::DmaId;

#[cfg(feature = "d21x")]
pub use d21x::DmaId;

/// DMA request IDs for D13x, D12x, G73x, and M6800 series.
pub mod d13x {
    /// DMA request device ID for D13x, D12x, G73x, and M6800 series.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[repr(u8)]
    pub enum DmaId {
        /// SRAM / PSRAM / XIP.
        Sram = 0,
        /// PSADC0 FIFO queue 1.
        Psadc0Q1 = 4,
        /// PSADC1 FIFO queue 1.
        Psadc1Q1 = 5,
        /// PSADC0 FIFO queue 2.
        Psadc0Q2 = 6,
        /// PSADC1 FIFO queue 2.
        Psadc1Q2 = 7,
        /// SPI2.
        Spi2 = 8,
        /// SPI3.
        Spi3 = 9,
        /// SPI0.
        Spi0 = 10,
        /// SPI1.
        Spi1 = 11,
        /// I2S0.
        I2s0 = 12,
        /// I2S1.
        I2s1 = 13,
        /// Audio DMIC.
        AudioDmic = 14,
        /// UART0.
        Uart0 = 16,
        /// UART1.
        Uart1 = 17,
        /// UART2.
        Uart2 = 18,
        /// UART3.
        Uart3 = 19,
        /// UART4.
        Uart4 = 20,
        /// UART5.
        Uart5 = 21,
        /// UART6.
        Uart6 = 22,
        /// UART7.
        Uart7 = 23,
        /// XSPI.
        Xspi = 24,
        /// SDFM0.
        Sdfm0 = 25,
        /// SDFM1.
        Sdfm1 = 26,
        /// SDFM2.
        Sdfm2 = 27,
        /// SDFM3.
        Sdfm3 = 28,
    }

    impl DmaId {
        /// Alias for PSRAM (shared with SRAM / XIP ID 0).
        pub const PSRAM: Self = Self::Sram;
        /// Alias for XIP (shared with SRAM / PSRAM ID 0).
        pub const XIP: Self = Self::Sram;

        /// Return raw numeric DMA request ID.
        #[inline]
        pub const fn raw(self) -> u8 {
            self as u8
        }
    }

    impl TryFrom<u8> for DmaId {
        type Error = u8;

        fn try_from(val: u8) -> Result<Self, Self::Error> {
            match val {
                0 => Ok(Self::Sram),
                4 => Ok(Self::Psadc0Q1),
                5 => Ok(Self::Psadc1Q1),
                6 => Ok(Self::Psadc0Q2),
                7 => Ok(Self::Psadc1Q2),
                8 => Ok(Self::Spi2),
                9 => Ok(Self::Spi3),
                10 => Ok(Self::Spi0),
                11 => Ok(Self::Spi1),
                12 => Ok(Self::I2s0),
                13 => Ok(Self::I2s1),
                14 => Ok(Self::AudioDmic),
                16 => Ok(Self::Uart0),
                17 => Ok(Self::Uart1),
                18 => Ok(Self::Uart2),
                19 => Ok(Self::Uart3),
                20 => Ok(Self::Uart4),
                21 => Ok(Self::Uart5),
                22 => Ok(Self::Uart6),
                23 => Ok(Self::Uart7),
                24 => Ok(Self::Xspi),
                25 => Ok(Self::Sdfm0),
                26 => Ok(Self::Sdfm1),
                27 => Ok(Self::Sdfm2),
                28 => Ok(Self::Sdfm3),
                _ => Err(val),
            }
        }
    }
}

/// DMA request IDs for D21x series.
pub mod d21x {
    /// DMA request device ID for D21x series.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[repr(u8)]
    pub enum DmaId {
        /// SRAM.
        Sram = 0,
        /// DRAM.
        Dram = 1,
        /// PSADC FIFO queue 1.
        PsadcQ1 = 4,
        /// PSADC FIFO queue 2.
        PsadcQ2 = 5,
        /// SPI2.
        Spi2 = 8,
        /// SPI3.
        Spi3 = 9,
        /// SPI0.
        Spi0 = 10,
        /// SPI1.
        Spi1 = 11,
        /// I2S0.
        I2s0 = 12,
        /// I2S1.
        I2s1 = 13,
        /// Audio DMIC.
        AudioDmic = 14,
        /// Audio ADC.
        AudioAdc = 15,
        /// UART0.
        Uart0 = 16,
        /// UART1.
        Uart1 = 17,
        /// UART2.
        Uart2 = 18,
        /// UART3.
        Uart3 = 19,
        /// UART4.
        Uart4 = 20,
        /// UART5.
        Uart5 = 21,
        /// UART6.
        Uart6 = 22,
        /// UART7.
        Uart7 = 23,
    }

    impl DmaId {
        /// Return raw numeric DMA request ID.
        #[inline]
        pub const fn raw(self) -> u8 {
            self as u8
        }
    }

    impl TryFrom<u8> for DmaId {
        type Error = u8;

        fn try_from(val: u8) -> Result<Self, Self::Error> {
            match val {
                0 => Ok(Self::Sram),
                1 => Ok(Self::Dram),
                4 => Ok(Self::PsadcQ1),
                5 => Ok(Self::PsadcQ2),
                8 => Ok(Self::Spi2),
                9 => Ok(Self::Spi3),
                10 => Ok(Self::Spi0),
                11 => Ok(Self::Spi1),
                12 => Ok(Self::I2s0),
                13 => Ok(Self::I2s1),
                14 => Ok(Self::AudioDmic),
                15 => Ok(Self::AudioAdc),
                16 => Ok(Self::Uart0),
                17 => Ok(Self::Uart1),
                18 => Ok(Self::Uart2),
                19 => Ok(Self::Uart3),
                20 => Ok(Self::Uart4),
                21 => Ok(Self::Uart5),
                22 => Ok(Self::Uart6),
                23 => Ok(Self::Uart7),
                _ => Err(val),
            }
        }
    }
}
