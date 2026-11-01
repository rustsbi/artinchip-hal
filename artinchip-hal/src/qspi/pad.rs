//! QSPI pad.

use embedded_hal::digital::OutputPin;

use crate::pad::NoPad;

pub trait QspiPads<const I: u8> {}
pub trait SerialClock<const I: u8> {}
pub trait MasterOutSlaveIn<const I: u8> {}
pub trait MasterInSlaveOut<const I: u8> {}
pub trait ChipSelect<const I: u8> {}
pub trait Hold<const I: u8> {}
pub trait WriteProtect<const I: u8> {}

/// Single-line QSPI pad tuple (`SCK`, `MOSI`, `MISO`) with software CS.
pub type SinglePads<SCK, MOSI, MISO> = (
    SCK,
    Option<MOSI>,
    Option<MISO>,
    Option<NoPad>,
    Option<NoPad>,
    Option<NoPad>,
);

/// Four-line QSPI pad tuple (`SCK`, `MOSI`, `MISO`, `WP`/`IO2`, `HOLD`/`IO3`) with software CS.
pub type QuadPads<SCK, MOSI, MISO, WP, HOLD> = (
    SCK,
    Option<MOSI>,
    Option<MISO>,
    Option<NoPad>,
    Option<WP>,
    Option<HOLD>,
);

impl<const I: u8, SCK, MOSI, MISO, CS, WP, HOLD> QspiPads<I>
    for (
        SCK,
        Option<MOSI>,
        Option<MISO>,
        Option<CS>,
        Option<WP>,
        Option<HOLD>,
    )
where
    SCK: SerialClock<I>,
    MOSI: MasterOutSlaveIn<I>,
    MISO: MasterInSlaveOut<I>,
    CS: ChipSelect<I>,
    WP: WriteProtect<I>,
    HOLD: Hold<I>,
{
}

/// Single-line QSPI pad set that keeps external `/WP` and `/HOLD` pins driven high.
pub struct SinglePadsIdleIo<SCK, MOSI, MISO, WP, HOLD> {
    _sck: SCK,
    _mosi: MOSI,
    _miso: MISO,
    _wp: WP,
    _hold: HOLD,
}

impl<const I: u8, SCK, MOSI, MISO, WP, HOLD> QspiPads<I>
    for SinglePadsIdleIo<SCK, MOSI, MISO, WP, HOLD>
where
    SCK: SerialClock<I>,
    MOSI: MasterOutSlaveIn<I>,
    MISO: MasterInSlaveOut<I>,
    WP: OutputPin,
    HOLD: OutputPin,
{
}

/// Build a single-line QSPI pad tuple (`SCK`, `MOSI`, `MISO`) with software CS.
pub const fn single_pads<const I: u8, SCK, MOSI, MISO>(
    sck: SCK,
    mosi: MOSI,
    miso: MISO,
) -> SinglePads<SCK, MOSI, MISO>
where
    SCK: SerialClock<I>,
    MOSI: MasterOutSlaveIn<I>,
    MISO: MasterInSlaveOut<I>,
{
    (sck, Some(mosi), Some(miso), None, None, None)
}

/// Build a single-line QSPI pad set while driving `/WP` and `/HOLD` high on boards
/// that share `IO2`/`IO3` with QSPI.
pub fn single_pads_idle_io<const I: u8, SCK, MOSI, MISO, WP, HOLD>(
    sck: SCK,
    mosi: MOSI,
    miso: MISO,
    mut wp: WP,
    mut hold: HOLD,
) -> SinglePadsIdleIo<SCK, MOSI, MISO, WP, HOLD>
where
    SCK: SerialClock<I>,
    MOSI: MasterOutSlaveIn<I>,
    MISO: MasterInSlaveOut<I>,
    WP: OutputPin,
    HOLD: OutputPin,
{
    wp.set_high().ok();
    hold.set_high().ok();
    SinglePadsIdleIo {
        _sck: sck,
        _mosi: mosi,
        _miso: miso,
        _wp: wp,
        _hold: hold,
    }
}

/// Build a four-line QSPI pad tuple (`SCK`, `MOSI`, `MISO`, `WP`/`IO2`, `HOLD`/`IO3`) with software CS.
pub const fn quad_pads<const I: u8, SCK, MOSI, MISO, WP, HOLD>(
    sck: SCK,
    mosi: MOSI,
    miso: MISO,
    wp: WP,
    hold: HOLD,
) -> QuadPads<SCK, MOSI, MISO, WP, HOLD>
where
    SCK: SerialClock<I>,
    MOSI: MasterOutSlaveIn<I>,
    MISO: MasterInSlaveOut<I>,
    WP: WriteProtect<I>,
    HOLD: Hold<I>,
{
    (sck, Some(mosi), Some(miso), None, Some(wp), Some(hold))
}
