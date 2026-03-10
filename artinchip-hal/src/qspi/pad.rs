//! QSPI pad.

pub trait QspiPad<const I: u8> {}
pub trait SerialClock<const I: u8>: QspiPad<I> {}
pub trait MasterOutSlaveIn<const I: u8>: QspiPad<I> {}
pub trait MasterInSlaveOut<const I: u8>: QspiPad<I> {}
pub trait ChipSelect<const I: u8>: QspiPad<I> {}
pub trait Hold<const I: u8>: QspiPad<I> {}
pub trait WriteProtect<const I: u8>: QspiPad<I> {}
