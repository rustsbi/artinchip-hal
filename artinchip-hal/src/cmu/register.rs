//! CMU register blocks and registers.

use volatile_register::{RO, RW};

/// Clock Manage Unit register block.
#[repr(C)]
pub struct RegisterBlock {
    /// PLL_INT0 general register (`PLL_INT0_GEN`).
    #[doc(alias = "PLL_INT0_GEN")]
    pub pll_int0_general: RW<u32>,
    /// PLL_INT1 general register (`PLL_INT1_GEN`).
    #[doc(alias = "PLL_INT1_GEN")]
    pub pll_int1_general: RW<u32>,
    _reserved0: [u8; 0x18],
    /// PLL_FRA0 general register (`PLL_FRA0_GEN`).
    #[doc(alias = "PLL_FRA0_GEN")]
    pub pll_fra0_general: RW<u32>,
    /// PLL_FRA1 general register (`PLL_FRA1_GEN`).
    #[doc(alias = "PLL_FRA1_GEN")]
    pub pll_fra1_general: RW<u32>,
    /// PLL_FRA2 general register (`PLL_FRA2_GEN`).
    #[doc(alias = "PLL_FRA2_GEN")]
    pub pll_fra2_general: RW<u32>,
    _reserved1: [u8; 0x14],
    /// PLL_INT0 configuration register (`PLL_INT0_CFG`).
    #[doc(alias = "PLL_INT0_CFG")]
    pub pll_int0_config: RW<u32>,
    /// PLL_INT1 configuration register (`PLL_INT1_CFG`).
    #[doc(alias = "PLL_INT1_CFG")]
    pub pll_int1_config: RW<u32>,
    _reserved2: [u8; 0x18],
    /// PLL_FRA0 configuration register (`PLL_FRA0_CFG`).
    #[doc(alias = "PLL_FRA0_CFG")]
    pub pll_fra0_config: RW<u32>,
    /// PLL_FRA1 configuration register (`PLL_FRA1_CFG`).
    #[doc(alias = "PLL_FRA1_CFG")]
    pub pll_fra1_config: RW<u32>,
    /// PLL_FRA2 configuration register (`PLL_FRA2_CFG`).
    #[doc(alias = "PLL_FRA2_CFG")]
    pub pll_fra2_config: RW<u32>,
    _reserved3: [u8; 0x14],
    /// PLL_FRA0 spread spectrum register (`PLL_FRA0_SDM`).
    #[doc(alias = "PLL_FRA0_SDM")]
    pub pll_fra0_spread_spectrum: RW<u32>,
    /// PLL_FRA1 spread spectrum register (`PLL_FRA1_SDM`).
    #[doc(alias = "PLL_FRA1_SDM")]
    pub pll_fra1_spread_spectrum: RW<u32>,
    /// PLL_FRA2 spread spectrum register (`PLL_FRA2_SDM`).
    #[doc(alias = "PLL_FRA2_SDM")]
    pub pll_fra2_spread_spectrum: RW<u32>,
    _reserved4: [u8; 0x14],
    /// PLL common register (`PLL_COMMON`).
    #[doc(alias = "PLL_COMMON")]
    pub pll_common: RW<u32>,
    /// PLL input register (`PLL_IN`).
    #[doc(alias = "PLL_IN")]
    pub pll_input: RW<u32>,
    _reserved5: [u8; 0x38],
    /// Clock 0 output register (`CLK_OUT0`).
    #[doc(alias = "CLK_OUT0")]
    pub clock_out0: RW<u32>,
    /// Clock 1 output register (`CLK_OUT1`).
    #[doc(alias = "CLK_OUT1")]
    pub clock_out1: RW<u32>,
    /// Clock 2 output register (`CLK_OUT2`).
    #[doc(alias = "CLK_OUT2")]
    pub clock_out2: RW<u32>,
    /// Clock 3 output register (`CLK_OUT3`).
    #[doc(alias = "CLK_OUT3")]
    pub clock_out3: RW<u32>,
    _reserved6: [u8; 0x10],
    /// AXI and AHB clock register (`CLK_AXI_AHB`).
    #[doc(alias = "CLK_AXI_AHB")]
    pub clock_axi_ahb: RW<u32>,
    _reserved7: [u8; 0xC],
    /// AHB clock register (`CLK_AHB`).
    #[doc(alias = "CLK_AHB")]
    pub clock_ahb: RW<u32>,
    _reserved8: [u8; 0xC],
    /// APB0 clock register (`CLK_APB0`).
    #[doc(alias = "CLK_APB0")]
    pub clock_apb0: RW<u32>,
    _reserved9: [u8; 0xDC],
    /// CPU clock register (`CLK_CPU`).
    #[doc(alias = "CLK_CPU")]
    pub clock_cpu: RW<u32>,
    /// DM clock register (`CLK_DM`).
    #[doc(alias = "CLK_DM")]
    pub clock_dm: RW<u32>,
    _reserved10: [u8; 0x4],
    /// WDOG clock register (`CLK_WDOG`).
    #[doc(alias = "CLK_WDOG")]
    pub clock_wdog: RW<u32>,
    /// DDR clock register (`CLK_DDR`).
    #[doc(alias = "CLK_DDR")]
    pub clock_ddr: RW<u32>,
    _reserved11: [u8; 0xC],
    /// DISP clock register (`CLK_DISP`).
    #[doc(alias = "CLK_DISP")]
    pub clock_disp: RW<u32>,
    _reserved12: [u8; 0xC],
    /// Audio serial clock register (`CLK_AUD_SCLK`).
    #[doc(alias = "CLK_AUD_SCLK")]
    pub clock_audio_serial: RW<u32>,
    _reserved13: [u8; 0xC],
    /// PWMCS and SDFM clock register (`CLK_PWMCS_SDFM`).
    #[doc(alias = "CLK_PWMCS_SDFM")]
    pub clock_pwmcs_sdfm: RW<u32>,
    _reserved14: [u8; 0x1CC],
    /// DMA clock register (`CLK_DMA`).
    #[doc(alias = "CLK_DMA")]
    pub clock_dma: RW<u32>,
    _reserved15: [u8; 0x4],
    /// CE clock register (`CLK_CE`).
    #[doc(alias = "CLK_CE")]
    pub clock_ce: RW<u32>,
    /// USB DEV clock register (`CLK_USB_DEV`).
    #[doc(alias = "CLK_USB_DEV")]
    pub clock_usb_dev: RW<u32>,
    /// USB HOST0 clock register (`CLK_USB_HOST0`|`CLK_USB_HOST`).
    #[doc(alias = "CLK_USB_HOST0")]
    #[doc(alias = "CLK_USB_HOST")]
    pub clock_usb_host0: RW<u32>,
    /// USB HOST1 clock register (`CLK_USB_HOST1`).
    #[doc(alias = "CLK_USB_HOST1")]
    pub clock_usb_host1: RW<u32>,
    _reserved16: [u8; 0x8],
    /// USB PHY0 clock register (`CLK_USB_PHY0`|`CLK_USB_PHY`).
    #[doc(alias = "CLK_USB_PHY0")]
    #[doc(alias = "CLK_USB_PHY")]
    pub clock_usb_phy0: RW<u32>,
    /// USB PHY1 clock register (`CLK_USB_PHY1`).
    #[doc(alias = "CLK_USB_PHY1")]
    pub clock_usb_phy1: RW<u32>,
    _reserved17: [u8; 0x8],
    /// EMAC or GMAC0 clock register (`CLK_EMAC`|`CLK_GMAC0`).
    #[doc(alias = "CLK_EMAC")]
    #[doc(alias = "CLK_GMAC0")]
    pub clock_emac_gmac0: RW<u32>,
    /// GMAC1 clock register (`CLK_GMAC1`).
    #[doc(alias = "CLK_GMAC1")]
    pub clock_gmac1: RW<u32>,
    _reserved18: [u8; 0x14],
    /// XSPI clock register (`CLK_XSPI`).
    #[doc(alias = "CLK_XSPI")]
    pub clock_xspi: RW<u32>,
    /// QSPI0 clock register (`CLK_QSPI0`).
    #[doc(alias = "CLK_QSPI0")]
    pub clock_qspi0: RW<u32>,
    /// QSPI1 clock register (`CLK_QSPI1`).
    #[doc(alias = "CLK_QSPI1")]
    pub clock_qspi1: RW<u32>,
    /// QSPI2 clock register (`CLK_QSPI2`).
    #[doc(alias = "CLK_QSPI2")]
    pub clock_qspi2: RW<u32>,
    /// QSPI3 clock register (`CLK_QSPI3`).
    #[doc(alias = "CLK_QSPI3")]
    pub clock_qspi3: RW<u32>,
    /// SDMC0 clock register (`CLK_SDMC0`).
    #[doc(alias = "CLK_SDMC0")]
    pub clock_sdmc0: RW<u32>,
    /// SDMC1 clock register (`CLK_SDMC1`).
    #[doc(alias = "CLK_SDMC1")]
    pub clock_sdmc1: RW<u32>,
    /// SDMC2 clock register (`CLK_SDMC2`).
    #[doc(alias = "CLK_SDMC2")]
    pub clock_sdmc2: RW<u32>,
    _reserved19: [u8; 0x14],
    /// CORDIC clock register (`CLK_CORDIC`).
    #[doc(alias = "CLK_CORDIC")]
    pub clock_cordic: RW<u32>,
    /// HCL clock register (`CLK_HCL`).
    #[doc(alias = "CLK_HCL")]
    pub clock_hcl: RW<u32>,
    _reserved20: [u8; 0x8],
    /// PBUS clock register (`CLK_PBUS`).
    #[doc(alias = "CLK_PBUS")]
    pub clock_pbus: RW<u32>,
    _reserved21: [u8; 0x35C],
    /// SYSCFG clock register (`CLK_SYSCFG`).
    #[doc(alias = "CLK_SYSCFG")]
    pub clock_syscfg: RW<u32>,
    _reserved22: [u8; 0xC],
    /// SPI_ENC clock register (`CLK_SPI_ENC`).
    #[doc(alias = "CLK_SPI_ENC")]
    pub clock_spi_enc: RW<u32>,
    /// PWMCS clock register (`CLK_PWMCS`).
    #[doc(alias = "CLK_PWMCS")]
    pub clock_pwmcs: RW<u32>,
    /// PSADC clock register (`CLK_PSADC`).
    #[doc(alias = "CLK_PSADC")]
    pub clock_psadc: RW<u32>,
    /// MTOP clock register (`CLK_MTOP`).
    #[doc(alias = "CLK_MTOP")]
    pub clock_mtop: RW<u32>,
    /// I2S0 clock register (`CLK_I2S0`|`CLK_I2S`).
    #[doc(alias = "CLK_I2S0")]
    #[doc(alias = "CLK_I2S")]
    pub clock_i2s0: RW<u32>,
    /// I2S1 clock register (`CLK_I2S1`).
    #[doc(alias = "CLK_I2S1")]
    pub clock_i2s1: RW<u32>,
    _reserved23: [u8; 0x14],
    /// GPIO clock register (`CLK_GPIO`).
    #[doc(alias = "CLK_GPIO")]
    pub clock_gpio: RW<u32>,
    /// UART0 clock register (`CLK_UART0`).
    #[doc(alias = "CLK_UART0")]
    pub clock_uart0: RW<u32>,
    /// UART1 clock register (`CLK_UART1`).
    #[doc(alias = "CLK_UART1")]
    pub clock_uart1: RW<u32>,
    /// UART2 clock register (`CLK_UART2`).
    #[doc(alias = "CLK_UART2")]
    pub clock_uart2: RW<u32>,
    /// UART3 clock register (`CLK_UART3`).
    #[doc(alias = "CLK_UART3")]
    pub clock_uart3: RW<u32>,
    /// UART4 clock register (`CLK_UART4`).
    #[doc(alias = "CLK_UART4")]
    pub clock_uart4: RW<u32>,
    /// UART5 clock register (`CLK_UART5`).
    #[doc(alias = "CLK_UART5")]
    pub clock_uart5: RW<u32>,
    /// UART6 clock register (`CLK_UART6`).
    #[doc(alias = "CLK_UART6")]
    pub clock_uart6: RW<u32>,
    /// UART7 clock register (`CLK_UART7`).
    #[doc(alias = "CLK_UART7")]
    pub clock_uart7: RW<u32>,
    _reserved24: [u8; 0x10],
    /// TA interface clock register (`CLK_TA_IF`).
    #[doc(alias = "CLK_TA_IF")]
    pub clock_ta_if: RW<u32>,
    /// EDAT interface clock register (`CLK_EDAT_IF`).
    #[doc(alias = "CLK_EDAT_IF")]
    pub clock_edat_if: RW<u32>,
    /// BIS interface clock register (`CLK_BIS_IF`).
    #[doc(alias = "CLK_BIS_IF")]
    pub clock_bis_if: RW<u32>,
    /// SDFM clock register (`CLK_SDFM`).
    #[doc(alias = "CLK_SDFM")]
    pub clock_sdfm: RW<u32>,
    /// LCD clock register (`CLK_LCD`).
    #[doc(alias = "CLK_LCD")]
    pub clock_lcd: RW<u32>,
    /// LVDS clock register (`CLK_LVDS`).
    #[doc(alias = "CLK_LVDS")]
    pub clock_lvds: RW<u32>,
    /// MIPI_DSI clock register (`CLK_MIPI_DSI`).
    #[doc(alias = "CLK_MIPI_DSI")]
    pub clock_mipi_dsi: RW<u32>,
    _reserved25: [u8; 0x4],
    /// DVP clock register (`CLK_DVP`).
    #[doc(alias = "CLK_DVP")]
    pub clock_dvp: RW<u32>,
    _reserved26: [u8; 0x4],
    /// MIPI_CSI clock register (`CLK_MIPI_CSI`).
    #[doc(alias = "CLK_MIPI_CSI")]
    pub clock_mipi_csi: RW<u32>,
    _reserved27: [u8; 0x24],
    /// DE clock register (`CLK_DE`).
    #[doc(alias = "CLK_DE")]
    pub clock_de: RW<u32>,
    /// GE clock register (`CLK_GE`).
    #[doc(alias = "CLK_GE")]
    pub clock_ge: RW<u32>,
    /// VE clock register (`CLK_VE`).
    #[doc(alias = "CLK_VE")]
    pub clock_ve: RW<u32>,
    _reserved28: [u8; 0x38],
    /// SID clock register (`CLK_SID`).
    #[doc(alias = "CLK_SID")]
    pub clock_sid: RW<u32>,
    /// RTC clock register (`CLK_RTC`).
    #[doc(alias = "CLK_RTC")]
    pub clock_rtc: RW<u32>,
    /// GTC clock register (`CLK_GTC`).
    #[doc(alias = "CLK_GTC")]
    pub clock_gtc: RW<u32>,
    _reserved29: [u8; 0x50],
    /// I2C0 clock register (`CLK_I2C0`).
    #[doc(alias = "CLK_I2C0")]
    pub clock_i2c0: RW<u32>,
    /// I2C1 clock register (`CLK_I2C1`).
    #[doc(alias = "CLK_I2C1")]
    pub clock_i2c1: RW<u32>,
    /// I2C2 clock register (`CLK_I2C2`).
    #[doc(alias = "CLK_I2C2")]
    pub clock_i2c2: RW<u32>,
    /// I2C3 clock register (`CLK_I2C3`).
    #[doc(alias = "CLK_I2C3")]
    pub clock_i2c3: RW<u32>,
    _reserved30: [u8; 0x10],
    /// CAN0 clock register (`CLK_CAN0`).
    #[doc(alias = "CLK_CAN0")]
    pub clock_can0: RW<u32>,
    /// CAN1 clock register (`CLK_CAN1`).
    #[doc(alias = "CLK_CAN1")]
    pub clock_can1: RW<u32>,
    _reserved31: [u8; 0x8],
    /// PWM/XPWM0 clock register (`CLK_PWM`).
    #[doc(alias = "CLK_PWM")]
    pub clock_pwm: RW<u32>,
    _reserved32: [u8; 0xC],
    /// ADCIM clock register (`CLK_ADCIM`).
    #[doc(alias = "CLK_ADCIM")]
    pub clock_adcim: RW<u32>,
    /// GPAI clock register (`CLK_GPAI`).
    #[doc(alias = "CLK_GPAI")]
    pub clock_gpai: RW<u32>,
    /// RTP clock register (`CLK_RTP`).
    #[doc(alias = "CLK_RTP")]
    pub clock_rtp: RW<u32>,
    /// THS clock register (`CLK_THS`).
    #[doc(alias = "CLK_THS")]
    pub clock_ths: RW<u32>,
    /// CIR clock register (`CLK_CIR`).
    #[doc(alias = "CLK_CIR")]
    pub clock_cir: RW<u32>,
    _reserved33: [u8; 0x648],
    /// CMU version register (`VERSION`).
    #[doc(alias = "VERSION")]
    pub version: RO<u32>,
}

#[cfg(test)]
mod tests {
    use super::RegisterBlock;
    use core::mem::offset_of;

    #[test]
    fn struct_register_block_offset() {
        assert_eq!(offset_of!(RegisterBlock, pll_int0_general), 0x0);
        assert_eq!(offset_of!(RegisterBlock, pll_int1_general), 0x4);
        assert_eq!(offset_of!(RegisterBlock, pll_fra0_general), 0x20);
        assert_eq!(offset_of!(RegisterBlock, pll_fra1_general), 0x24);
        assert_eq!(offset_of!(RegisterBlock, pll_fra2_general), 0x28);
        assert_eq!(offset_of!(RegisterBlock, pll_int0_config), 0x40);
        assert_eq!(offset_of!(RegisterBlock, pll_int1_config), 0x44);
        assert_eq!(offset_of!(RegisterBlock, pll_fra0_config), 0x60);
        assert_eq!(offset_of!(RegisterBlock, pll_fra1_config), 0x64);
        assert_eq!(offset_of!(RegisterBlock, pll_fra2_config), 0x68);
        assert_eq!(offset_of!(RegisterBlock, pll_fra0_spread_spectrum), 0x80);
        assert_eq!(offset_of!(RegisterBlock, pll_fra1_spread_spectrum), 0x84);
        assert_eq!(offset_of!(RegisterBlock, pll_fra2_spread_spectrum), 0x88);
        assert_eq!(offset_of!(RegisterBlock, pll_common), 0xA0);
        assert_eq!(offset_of!(RegisterBlock, pll_input), 0xA4);
        assert_eq!(offset_of!(RegisterBlock, clock_out0), 0xE0);
        assert_eq!(offset_of!(RegisterBlock, clock_out1), 0xE4);
        assert_eq!(offset_of!(RegisterBlock, clock_out2), 0xE8);
        assert_eq!(offset_of!(RegisterBlock, clock_out3), 0xEC);
        assert_eq!(offset_of!(RegisterBlock, clock_axi_ahb), 0x100);
        assert_eq!(offset_of!(RegisterBlock, clock_ahb), 0x110);
        assert_eq!(offset_of!(RegisterBlock, clock_apb0), 0x120);
        assert_eq!(offset_of!(RegisterBlock, clock_cpu), 0x200);
        assert_eq!(offset_of!(RegisterBlock, clock_dm), 0x204);
        assert_eq!(offset_of!(RegisterBlock, clock_wdog), 0x20C);
        assert_eq!(offset_of!(RegisterBlock, clock_ddr), 0x210);
        assert_eq!(offset_of!(RegisterBlock, clock_disp), 0x220);
        assert_eq!(offset_of!(RegisterBlock, clock_audio_serial), 0x230);
        assert_eq!(offset_of!(RegisterBlock, clock_pwmcs_sdfm), 0x240);
        assert_eq!(offset_of!(RegisterBlock, clock_dma), 0x410);
        assert_eq!(offset_of!(RegisterBlock, clock_ce), 0x418);
        assert_eq!(offset_of!(RegisterBlock, clock_usb_dev), 0x41C);
        assert_eq!(offset_of!(RegisterBlock, clock_usb_host0), 0x420);
        assert_eq!(offset_of!(RegisterBlock, clock_usb_host1), 0x424);
        assert_eq!(offset_of!(RegisterBlock, clock_usb_phy0), 0x430);
        assert_eq!(offset_of!(RegisterBlock, clock_usb_phy1), 0x434);
        assert_eq!(offset_of!(RegisterBlock, clock_emac_gmac0), 0x440);
        assert_eq!(offset_of!(RegisterBlock, clock_gmac1), 0x444);
        assert_eq!(offset_of!(RegisterBlock, clock_xspi), 0x45C);
        assert_eq!(offset_of!(RegisterBlock, clock_qspi0), 0x460);
        assert_eq!(offset_of!(RegisterBlock, clock_qspi1), 0x464);
        assert_eq!(offset_of!(RegisterBlock, clock_qspi2), 0x468);
        assert_eq!(offset_of!(RegisterBlock, clock_qspi3), 0x46C);
        assert_eq!(offset_of!(RegisterBlock, clock_sdmc0), 0x470);
        assert_eq!(offset_of!(RegisterBlock, clock_sdmc1), 0x474);
        assert_eq!(offset_of!(RegisterBlock, clock_sdmc2), 0x478);
        assert_eq!(offset_of!(RegisterBlock, clock_cordic), 0x490);
        assert_eq!(offset_of!(RegisterBlock, clock_hcl), 0x494);
        assert_eq!(offset_of!(RegisterBlock, clock_pbus), 0x4A0);
        assert_eq!(offset_of!(RegisterBlock, clock_syscfg), 0x800);
        assert_eq!(offset_of!(RegisterBlock, clock_spi_enc), 0x810);
        assert_eq!(offset_of!(RegisterBlock, clock_pwmcs), 0x814);
        assert_eq!(offset_of!(RegisterBlock, clock_psadc), 0x818);
        assert_eq!(offset_of!(RegisterBlock, clock_mtop), 0x81C);
        assert_eq!(offset_of!(RegisterBlock, clock_i2s0), 0x820);
        assert_eq!(offset_of!(RegisterBlock, clock_i2s1), 0x824);
        assert_eq!(offset_of!(RegisterBlock, clock_gpio), 0x83C);
        assert_eq!(offset_of!(RegisterBlock, clock_uart0), 0x840);
        assert_eq!(offset_of!(RegisterBlock, clock_uart1), 0x844);
        assert_eq!(offset_of!(RegisterBlock, clock_uart2), 0x848);
        assert_eq!(offset_of!(RegisterBlock, clock_uart3), 0x84C);
        assert_eq!(offset_of!(RegisterBlock, clock_uart4), 0x850);
        assert_eq!(offset_of!(RegisterBlock, clock_uart5), 0x854);
        assert_eq!(offset_of!(RegisterBlock, clock_uart6), 0x858);
        assert_eq!(offset_of!(RegisterBlock, clock_uart7), 0x85C);
        assert_eq!(offset_of!(RegisterBlock, clock_ta_if), 0x870);
        assert_eq!(offset_of!(RegisterBlock, clock_edat_if), 0x874);
        assert_eq!(offset_of!(RegisterBlock, clock_bis_if), 0x878);
        assert_eq!(offset_of!(RegisterBlock, clock_sdfm), 0x87C);
        assert_eq!(offset_of!(RegisterBlock, clock_lcd), 0x880);
        assert_eq!(offset_of!(RegisterBlock, clock_lvds), 0x884);
        assert_eq!(offset_of!(RegisterBlock, clock_mipi_dsi), 0x888);
        assert_eq!(offset_of!(RegisterBlock, clock_dvp), 0x890);
        assert_eq!(offset_of!(RegisterBlock, clock_mipi_csi), 0x898);
        assert_eq!(offset_of!(RegisterBlock, clock_de), 0x8C0);
        assert_eq!(offset_of!(RegisterBlock, clock_ge), 0x8C4);
        assert_eq!(offset_of!(RegisterBlock, clock_ve), 0x8C8);
        assert_eq!(offset_of!(RegisterBlock, clock_sid), 0x904);
        assert_eq!(offset_of!(RegisterBlock, clock_rtc), 0x908);
        assert_eq!(offset_of!(RegisterBlock, clock_gtc), 0x90C);
        assert_eq!(offset_of!(RegisterBlock, clock_i2c0), 0x960);
        assert_eq!(offset_of!(RegisterBlock, clock_i2c1), 0x964);
        assert_eq!(offset_of!(RegisterBlock, clock_i2c2), 0x968);
        assert_eq!(offset_of!(RegisterBlock, clock_i2c3), 0x96C);
        assert_eq!(offset_of!(RegisterBlock, clock_can0), 0x980);
        assert_eq!(offset_of!(RegisterBlock, clock_can1), 0x984);
        assert_eq!(offset_of!(RegisterBlock, clock_pwm), 0x990);
        assert_eq!(offset_of!(RegisterBlock, clock_adcim), 0x9A0);
        assert_eq!(offset_of!(RegisterBlock, clock_gpai), 0x9A4);
        assert_eq!(offset_of!(RegisterBlock, clock_rtp), 0x9A8);
        assert_eq!(offset_of!(RegisterBlock, clock_ths), 0x9AC);
        assert_eq!(offset_of!(RegisterBlock, clock_cir), 0x9B0);
        assert_eq!(offset_of!(RegisterBlock, version), 0xFFC);
    }
}
