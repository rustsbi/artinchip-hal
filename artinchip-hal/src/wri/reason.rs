//! Warm reset reason.

/// Warm reset reason.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResetReason {
    /// System power-on reset.
    PowerOnReset,
    /// RTC power-on reset.
    RtcPowerOnReset,
    /// External pin triggered reset.
    PinReset,
    /// CPU debug module triggered reset.
    CpuDebugReset,
    /// Watchdog timer reset.
    WatchdogReset,
    /// Over-temperature reset.
    ThermalReset,
    /// Voltage comparator / threshold reset.
    ComparatorReset,
}
