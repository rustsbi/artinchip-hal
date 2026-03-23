//! GPIO pin multiplexer macros.

/// NOTE: GPIO struct must be implemented before using this macro.
macro_rules! gpio {
    ($Pads:ident, $G:expr, [$($N:tt),*]) => {
        paste! {
            #[doc = concat!("GPIO", $G, " pads.")]
            pub struct $Pads {
                $(
                    #[doc = concat!("GPIO pad P", $G, $N, ".")]
                    pub [<p $G:lower $N>]: crate::gpio::GpioPad<$G, $N>,
                )*
            }

            impl $Pads {
                #[inline]
                const fn __new() -> Self {
                    Self {
                        $(
                            [<p $G:lower $N>]: unsafe {crate::gpio::GpioPad::__new(&*GPIO::ptr())},
                        )*
                    }
                }
            }
        }
    };
}

// UART pin multiplexer macros.

/// Implements the `Transmit` and `UartPad` traits for multiple UART pins.
#[allow(unused_macros)]
macro_rules! uart_tx {
    ($uart_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl artinchip_hal::uart::UartPad<$uart_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
            impl artinchip_hal::uart::Transmit<$uart_num> for crate::gpio::Function<'_, $port, $pin, $func> {}

            paste! {
                impl<'a> crate::gpio::GpioPad<$port, $pin> {
                    #[inline]
                    pub fn [<into_uart $uart_num _tx>](self) -> crate::gpio::Function<'a, $port, $pin, $func> {
                        self.into_function::<$func>()
                    }
                }
            }
        )+
    };
}

/// Implements the `Receive` and `UartPad` traits for multiple UART pins.
#[allow(unused_macros)]
macro_rules! uart_rx {
    ($uart_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl artinchip_hal::uart::UartPad<$uart_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
            impl artinchip_hal::uart::Receive<$uart_num> for crate::gpio::Function<'_, $port, $pin, $func> {}

            paste! {
                impl<'a> crate::gpio::GpioPad<$port, $pin> {
                    #[inline]
                    pub fn [<into_uart $uart_num _rx>](self) -> crate::gpio::Function<'a, $port, $pin, $func> {
                        self.into_function::<$func>()
                    }
                }
            }
        )+
    };
}

// QSPI pin multiplexer macros.

/// Implements the `SerialClock` traits for multiple QSPI pins.
#[allow(unused_macros)]
macro_rules! qspi_sck {
    ($qspi_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl artinchip_hal::qspi::SerialClock<$qspi_num> for crate::gpio::Function<'_, $port, $pin, $func> {}

            paste! {
                impl<'a> crate::gpio::GpioPad<$port, $pin> {
                    #[inline]
                    pub fn [<into_qspi $qspi_num _sck>](self) -> crate::gpio::Function<'a, $port, $pin, $func> {
                        self.into_function::<$func>()
                    }
                }
            }
        )+
    };
}

/// Implements the `MasterOutSlaveIn` traits for multiple QSPI pins.
#[allow(unused_macros)]
macro_rules! qspi_mosi {
    ($qspi_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl artinchip_hal::qspi::MasterOutSlaveIn<$qspi_num> for crate::gpio::Function<'_, $port, $pin, $func> {}

            paste! {
                impl<'a> crate::gpio::GpioPad<$port, $pin> {
                    #[inline]
                    pub fn [<into_qspi $qspi_num _mosi>](self) -> crate::gpio::Function<'a, $port, $pin, $func> {
                        self.into_function::<$func>()
                    }
                }
            }
        )+
    };
}

/// Implements the `MasterInSlaveOut` traits for multiple QSPI pins.
#[allow(unused_macros)]
macro_rules! qspi_miso {
    ($qspi_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl artinchip_hal::qspi::MasterInSlaveOut<$qspi_num> for crate::gpio::Function<'_, $port, $pin, $func> {}

            paste! {
                impl<'a> crate::gpio::GpioPad<$port, $pin> {
                    #[inline]
                    pub fn [<into_qspi $qspi_num _miso>](self) -> crate::gpio::Function<'a, $port, $pin, $func> {
                        self.into_function::<$func>()
                    }
                }
            }
        )+
    };
}

/// Implements the `ChipSelect` traits for multiple QSPI pins.
#[allow(unused_macros)]
macro_rules! qspi_cs {
    ($qspi_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl artinchip_hal::qspi::ChipSelect<$qspi_num> for crate::gpio::Function<'_, $port, $pin, $func> {}

            paste! {
                impl<'a> crate::gpio::GpioPad<$port, $pin> {
                    #[inline]
                    pub fn [<into_qspi $qspi_num _cs>](self) -> crate::gpio::Function<'a, $port, $pin, $func> {
                        self.into_function::<$func>()
                    }
                }
            }
        )+
    };
}

/// Implements the `WriteProtect` traits for multiple QSPI pins.
#[allow(unused_macros)]
macro_rules! qspi_wp {
    ($qspi_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl artinchip_hal::qspi::WriteProtect<$qspi_num> for crate::gpio::Function<'_, $port, $pin, $func> {}

            paste! {
                impl<'a> crate::gpio::GpioPad<$port, $pin> {
                    #[inline]
                    pub fn [<into_qspi $qspi_num _wp>](self) -> crate::gpio::Function<'a, $port, $pin, $func> {
                        self.into_function::<$func>()
                    }
                }
            }
        )+
    };
}

/// Implements the `Hold` traits for multiple QSPI pins.
#[allow(unused_macros)]
macro_rules! qspi_hold {
    ($qspi_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl artinchip_hal::qspi::Hold<$qspi_num> for crate::gpio::Function<'_, $port, $pin, $func> {}

            paste! {
                impl<'a> crate::gpio::GpioPad<$port, $pin> {
                    #[inline]
                    pub fn [<into_qspi $qspi_num _hold>](self) -> crate::gpio::Function<'a, $port, $pin, $func> {
                        self.into_function::<$func>()
                    }
                }
            }
        )+
    };
}

// I2C pin multiplexer macros.

/// Implements the `SerialClock` traits for multiple I2C pins.
#[allow(unused_macros)]
macro_rules! i2c_scl {
    ($i2c_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl artinchip_hal::i2c::SerialClock<$i2c_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
            paste! {
                impl<'a> crate::gpio::GpioPad<$port, $pin> {
                    #[inline]
                    pub fn [<into_i2c $i2c_num _scl>](self) -> crate::gpio::Function<'a, $port, $pin, $func> {
                        self.into_function::<$func>()
                    }
                }
            }
        )+
    };
}

/// Implements the `SerialData` traits for multiple I2C pins.
#[allow(unused_macros)]
macro_rules! i2c_sda {
    ($i2c_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl artinchip_hal::i2c::SerialData<$i2c_num> for crate::gpio::Function<'_, $port, $pin, $func> {}
            paste! {
                impl<'a> crate::gpio::GpioPad<$port, $pin> {
                    #[inline]
                    pub fn [<into_i2c $i2c_num _sda>](self) -> crate::gpio::Function<'a, $port, $pin, $func> {
                        self.into_function::<$func>()
                    }
                }
            }
        )+
    };
}

/// Implements the `PortA` trait for multiple PWM pins.
#[allow(unused_macros)]
macro_rules! pwm_a {
    ($pwm_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl artinchip_hal::pwm::PortA<$pwm_num> for crate::gpio::Function<'_, $port, $pin, $func> {}

            paste! {
                impl<'a> crate::gpio::GpioPad<$port, $pin> {
                    #[inline]
                    pub fn [<into_pwm_ch $pwm_num _a>](self) -> crate::gpio::Function<'a, $port, $pin, $func> {
                        self.into_function::<$func>()
                    }
                }
            }
        )+
    };
}

/// Implements the `PortB` trait for multiple PWM pins.
#[allow(unused_macros)]
macro_rules! pwm_b {
    ($pwm_num:expr, $(($port:literal, $pin:expr, $func:expr)),+) => {
        $(
            impl artinchip_hal::pwm::PortB<$pwm_num> for crate::gpio::Function<'_, $port, $pin, $func> {}

            paste! {
                impl<'a> crate::gpio::GpioPad<$port, $pin> {
                    #[inline]
                    pub fn [<into_pwm_ch $pwm_num _b>](self) -> crate::gpio::Function<'a, $port, $pin, $func> {
                        self.into_function::<$func>()
                    }
                }
            }
        )+
    };
}
