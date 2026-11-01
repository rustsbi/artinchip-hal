//! USB device extension trait.

#[cfg(feature = "async-usb")]
use super::UsbDevInterrupt;
use super::{UsbDevDriver, UsbDevError, UsbDevOptions};
use crate::cmu::Cmu;
use crate::sys_cfg::SocSysCfg;
use crate::usbdrd::{UsbConfig, UsbPads};

/// Extension trait for creating an initialized USB device driver.
pub trait UsbDevExt<'a> {
    /// Configure the USB device controller, connect D+, and unmask its global interrupt.
    fn new_driver<PAD>(
        self,
        pads: PAD,
        config: UsbConfig,
        cmu: &mut Cmu,
        syscfg: &mut SocSysCfg,
    ) -> Result<UsbDevDriver<'a, PAD>, UsbDevError>
    where
        PAD: UsbPads<0>;

    /// Configure the USB device controller with explicit connection/interrupt options.
    fn new_driver_with_options<PAD>(
        self,
        pads: PAD,
        config: UsbConfig,
        options: UsbDevOptions,
        cmu: &mut Cmu,
        syscfg: &mut SocSysCfg,
    ) -> Result<UsbDevDriver<'a, PAD>, UsbDevError>
    where
        PAD: UsbPads<0>;

    /// Configure the USB device controller for `embassy-usb` (`Driver` implementation).
    #[cfg(feature = "async-usb")]
    fn new_embassy_driver<PAD, IRQS>(
        self,
        pads: PAD,
        config: UsbConfig,
        irqs: IRQS,
        cmu: &mut Cmu,
        syscfg: &mut SocSysCfg,
    ) -> Result<UsbDevDriver<'a, PAD>, UsbDevError>
    where
        PAD: UsbPads<0> + 'static,
        IRQS: crate::interrupt::typelevel::Binding<
                crate::interrupt::typelevel::USB_DEV,
                UsbDevInterrupt,
            >;
}
