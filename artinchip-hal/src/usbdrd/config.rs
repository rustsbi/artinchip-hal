//! USB configuration.

/// USB PHY interface.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum UsbPhyInterface {
    /// Use the internal UTMI+ PHY interface.
    #[default]
    Utmi,
    /// Use an external ULPI PHY interface.
    Ulpi,
}

/// UTMI+ PHY interface width.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum UsbUtmiInterfaceWidth {
    /// Use an 8-bit interface.
    #[default]
    Bits8,
    /// Use a 16-bit interface.
    Bits16,
}

/// AHB DMA burst length.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum UsbDmaBurstLength {
    /// Single transfers.
    Single,
    /// Unspecified-length incrementing transfers.
    Incr,
    /// Four-beat incrementing transfers.
    #[default]
    Incr4,
    /// Eight-beat incrementing transfers.
    Incr8,
    /// Sixteen-beat incrementing transfers.
    Incr16,
}

/// USB controller configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsbConfig {
    /// PHY interface selection.
    pub phy_interface: UsbPhyInterface,
    /// UTMI+ interface width when `phy_interface` is [`UsbPhyInterface::Utmi`].
    pub utmi_interface_width: UsbUtmiInterfaceWidth,
    /// DMA burst length.
    pub dma_burst_length: UsbDmaBurstLength,
    /// Enable DMA for the device controller.
    pub enable_dma: bool,
    /// Enable the controller's global interrupt output.
    pub enable_global_interrupt: bool,
    /// AHB clock feeding the controller, in Hz.
    ///
    /// Only used to derive the full-speed USB turnaround time; high-speed
    /// operation uses a fixed value.
    pub ahb_clock_hz: u32,
}

impl Default for UsbConfig {
    fn default() -> Self {
        Self {
            phy_interface: UsbPhyInterface::Utmi,
            utmi_interface_width: UsbUtmiInterfaceWidth::Bits8,
            dma_burst_length: UsbDmaBurstLength::Incr4,
            enable_dma: true,
            enable_global_interrupt: true,
            ahb_clock_hz: 200_000_000,
        }
    }
}
