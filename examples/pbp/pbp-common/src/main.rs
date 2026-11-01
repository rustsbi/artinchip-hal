//! First-stage PBP shared by the boot chain and `AIC.FW` RAM updater slots.
#![no_std]
#![no_main]

#[cfg(all(feature = "d13x", feature = "d21x"))]
compile_error!("select exactly one chip feature: `d13x` or `d21x`");
#[cfg(not(any(feature = "d13x", feature = "d21x")))]
compile_error!("select a chip feature: `d13x` or `d21x`");

#[cfg(feature = "d13x")]
mod d13x;
#[cfg(feature = "d21x")]
mod d21x;
mod layout;

use artinchip_rt::{pbp_entry, prelude::*};
use panic_halt as _;

#[pbp_entry]
fn pbp_main(boot_param: BootParam, private_data: &[u8]) {
    #[cfg(feature = "d13x")]
    d13x::run(&boot_param, private_data);

    #[cfg(feature = "d21x")]
    d21x::run(&boot_param, private_data);
}
