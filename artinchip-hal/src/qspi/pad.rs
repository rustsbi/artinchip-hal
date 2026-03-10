//! QSPI pad.

pub trait QspiPads<const I: u8> {}
pub trait SerialClock<const I: u8> {}
pub trait MasterOutSlaveIn<const I: u8> {}
pub trait MasterInSlaveOut<const I: u8> {}
pub trait ChipSelect<const I: u8> {}
pub trait Hold<const I: u8> {}
pub trait WriteProtect<const I: u8> {}

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
