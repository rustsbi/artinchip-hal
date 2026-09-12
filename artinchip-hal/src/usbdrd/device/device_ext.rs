//! USB device extension trait.

#[cfg(feature = "clic-interrupts")]
use super::{AsyncAicUpg, AsyncUpgHandler};
use super::{UsbDevDriver, UsbDevError};
use crate::cmu::Cmu;
use crate::sys_cfg::SysCfg;
use crate::usbdrd::{UsbConfig, UsbPads};

/// Extension trait for creating an initialized USB device driver.
pub trait UsbDevExt<'a> {
    /// Configure the USB device controller and consume its pads.
    ///
    /// Named `new_driver` rather than `new` so it cannot be mistaken for a
    /// constructor that returns `Self`: it consumes the peripheral (`self`) and
    /// hands back a driver built around it. Matches `QspiExt`/`XspiExt`.
    fn new_driver<PAD>(
        self,
        pads: PAD,
        config: UsbConfig,
        cmu: &mut Cmu,
        syscfg: &mut SysCfg,
    ) -> Result<UsbDevDriver<'a, PAD>, UsbDevError>
    where
        PAD: UsbPads<0>;

    /// Create the upgrade device driven by the USB interrupt.
    #[cfg(feature = "clic-interrupts")]
    fn new_async_aic_upg<PAD, IRQS>(
        self,
        pads: PAD,
        config: UsbConfig,
        cmu: &mut Cmu,
        syscfg: &mut SysCfg,
        irqs: IRQS,
    ) -> Result<AsyncAicUpg<PAD>, UsbDevError>
    where
        PAD: UsbPads<0>,
        IRQS: crate::interrupt::clic::typelevel::Binding<
                crate::interrupt::clic::typelevel::USB_DEV,
                AsyncUpgHandler,
            >;
}
