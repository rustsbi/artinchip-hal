/// A no-op pad for flexible pin assignments.
pub struct NoPad;

impl<const I: u8> super::qspi::SerialClock<I> for NoPad {}
impl<const I: u8> super::qspi::MasterInSlaveOut<I> for NoPad {}
impl<const I: u8> super::qspi::MasterOutSlaveIn<I> for NoPad {}
impl<const I: u8> super::qspi::ChipSelect<I> for NoPad {}
impl<const I: u8> super::qspi::Hold<I> for NoPad {}
impl<const I: u8> super::qspi::WriteProtect<I> for NoPad {}
