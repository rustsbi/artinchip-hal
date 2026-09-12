//! USB device instance.

use super::dev_register::RegisterBlock;
use super::device_ext::UsbDevExt;
#[cfg(feature = "clic-interrupts")]
use super::{AsyncAicUpg, AsyncUpgHandler};
use super::{UsbDevDriver, UsbDevError};
use crate::cmu::Cmu;
use crate::sys_cfg::SysCfg;
use crate::usbdrd::{UsbConfig, UsbPads};
use core::marker::PhantomData;

/// USB device controller instance.
pub struct UsbDev {
    reg: *const RegisterBlock,
    _private: PhantomData<()>,
}

impl UsbDev {
    /// Create a new USB device controller instance.
    pub const fn __new(reg: *const RegisterBlock) -> Self {
        Self {
            reg,
            _private: PhantomData,
        }
    }

    /// Get a reference to the register block.
    pub const fn register_block(&self) -> &'static RegisterBlock {
        unsafe { &*self.reg }
    }
}

impl UsbDevExt<'static> for UsbDev {
    #[inline]
    fn new_driver<PAD>(
        self,
        pads: PAD,
        config: UsbConfig,
        cmu: &mut Cmu,
        syscfg: &mut SysCfg,
    ) -> Result<UsbDevDriver<'static, PAD>, UsbDevError>
    where
        PAD: UsbPads<0>,
    {
        UsbDevDriver::new(self.register_block(), pads, config, cmu, syscfg)
    }

    #[cfg(feature = "clic-interrupts")]
    #[inline]
    fn new_async_aic_upg<PAD, IRQS>(
        self,
        pads: PAD,
        config: UsbConfig,
        cmu: &mut Cmu,
        syscfg: &mut SysCfg,
        _irqs: IRQS,
    ) -> Result<AsyncAicUpg<PAD>, UsbDevError>
    where
        PAD: UsbPads<0>,
        IRQS: crate::interrupt::clic::typelevel::Binding<
                crate::interrupt::clic::typelevel::USB_DEV,
                AsyncUpgHandler,
            >,
    {
        AsyncAicUpg::new(UsbDevDriver::new(
            self.register_block(),
            pads,
            config,
            cmu,
            syscfg,
        )?)
    }
}
