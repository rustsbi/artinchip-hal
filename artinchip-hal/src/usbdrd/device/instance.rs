//! USB device controller peripheral instance.

use core::marker::PhantomData;

#[cfg(feature = "async-usb")]
use super::UsbDevInterrupt;
use super::dev_register::RegisterBlock;
use super::device_ext::UsbDevExt;
use super::{UsbDevDriver, UsbDevError, UsbDevOptions};
use crate::cmu::Cmu;
use crate::sys_cfg::SocSysCfg;
use crate::usbdrd::{UsbConfig, UsbPads};

/// USB device controller peripheral instance.
pub struct UsbDev {
    reg: *const RegisterBlock,
    _private: PhantomData<()>,
}

impl UsbDev {
    /// Create a new USB device controller instance from its MMIO base address.
    pub const fn __new(reg: *const RegisterBlock) -> Self {
        Self {
            reg,
            _private: PhantomData,
        }
    }

    /// Return a shared reference to the USB device register block.
    pub const fn register_block(&self) -> &'static RegisterBlock {
        unsafe { &*self.reg }
    }

    /// Mask the USB_DEV interrupt, assert module/PHY resets, and gate their clocks.
    ///
    /// Use this when releasing a controller that has no active [`UsbDevDriver`]
    /// (for example before jumping to a next-stage image or resetting an inherited
    /// BootROM USB session); live drivers should call [`UsbDevDriver::deinit`] instead.
    pub fn release(&mut self, cmu: &mut Cmu) {
        #[cfg(feature = "interrupts")]
        {
            use crate::interrupt::InterruptSource as _;
            use crate::interrupt::typelevel::USB_DEV;
            USB_DEV::disable();
            USB_DEV::clear_pending();
        }
        super::driver::release_clocks_and_reset(cmu);
    }
}

impl UsbDevExt<'static> for UsbDev {
    #[inline]
    fn new_driver<PAD>(
        self,
        pads: PAD,
        config: UsbConfig,
        cmu: &mut Cmu,
        syscfg: &mut SocSysCfg,
    ) -> Result<UsbDevDriver<'static, PAD>, UsbDevError>
    where
        PAD: UsbPads<0>,
    {
        UsbDevDriver::new(self.register_block(), pads, config, cmu, syscfg)
    }

    #[inline]
    fn new_driver_with_options<PAD>(
        self,
        pads: PAD,
        config: UsbConfig,
        options: UsbDevOptions,
        cmu: &mut Cmu,
        syscfg: &mut SocSysCfg,
    ) -> Result<UsbDevDriver<'static, PAD>, UsbDevError>
    where
        PAD: UsbPads<0>,
    {
        UsbDevDriver::new_with_options(self.register_block(), pads, config, options, cmu, syscfg)
    }

    #[cfg(feature = "async-usb")]
    #[inline]
    fn new_embassy_driver<PAD, IRQS>(
        self,
        pads: PAD,
        config: UsbConfig,
        irqs: IRQS,
        cmu: &mut Cmu,
        syscfg: &mut SocSysCfg,
    ) -> Result<UsbDevDriver<'static, PAD>, UsbDevError>
    where
        PAD: UsbPads<0> + 'static,
        IRQS: crate::interrupt::typelevel::Binding<
                crate::interrupt::typelevel::USB_DEV,
                UsbDevInterrupt,
            >,
    {
        UsbDevDriver::new_embassy(self.register_block(), pads, config, irqs, cmu, syscfg)
    }
}
