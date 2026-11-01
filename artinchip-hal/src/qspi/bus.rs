//! QSPI quad-phase bus trait.

/// Bus extension for SPI flash drivers that run a four-line data phase.
pub trait QuadSpiBus: embedded_hal::spi::SpiBus<u8> {
    /// Send `command` on one line, clock dummy bytes derived from `tclqv_ns` + `margin`,
    /// then read `data` across four lines.
    fn read_quad(
        &mut self,
        command: &[u8],
        tclqv_ns: u32,
        margin: u8,
        data: &mut [u8],
    ) -> Result<(), Self::Error>;

    /// Send `command` on one line, then transmit `data` across four lines.
    fn write_quad(&mut self, command: &[u8], data: &[u8]) -> Result<(), Self::Error>;
}
