use super::register::*;
use embedded_time::duration::Nanoseconds;
use embedded_time::rate::Hertz;

/// PWM action.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Action {
    /// Counts down and `TBCTR` = `CMPB` action.
    pub cbd: ActionMode,
    /// Counts up and `TBCTR` = `CMPA` action.
    pub cbu: ActionMode,
    /// Counts down and `TBCTR` = `CMPA` action.
    pub cad: ActionMode,
    /// Counts up and `TBCTR` = `CMPA` action.
    pub cau: ActionMode,
    /// `TBCTR` = `PRD` action.
    pub prd: ActionMode,
    /// `TBCTR` = 0 action.
    pub zro: ActionMode,
}

/// PWM configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PwmConfig {
    pub freq: Hertz,
    /// Unit: nanoseconds.
    pub duty_a: Nanoseconds,
    /// Unit: nanoseconds.
    pub duty_b: Nanoseconds,
    /// Unit: nanoseconds.
    pub period: Nanoseconds,
    pub cnt_mode: CntMode,
    pub tb_clk_rate: Hertz,
    pub init_level: InitLevel,
    pub action_0: Option<Action>,
    pub action_1: Option<Action>,
}

impl Default for PwmConfig {
    fn default() -> Self {
        Self {
            freq: Hertz::new(1_000),
            duty_a: Nanoseconds(500_000),
            duty_b: Nanoseconds(500_000),
            period: Nanoseconds(1_000_000),
            cnt_mode: CntMode::CountUp,
            tb_clk_rate: Hertz(24_000_000),
            init_level: InitLevel::High,
            action_0: Some(Action {
                cbd: ActionMode::NoOp,
                cbu: ActionMode::NoOp,
                cad: ActionMode::NoOp,
                cau: ActionMode::SetLow,
                prd: ActionMode::SetHigh,
                zro: ActionMode::NoOp,
            }),
            action_1: Some(Action {
                cbd: ActionMode::NoOp,
                cbu: ActionMode::SetLow,
                cad: ActionMode::NoOp,
                cau: ActionMode::NoOp,
                prd: ActionMode::SetHigh,
                zro: ActionMode::NoOp,
            }),
        }
    }
}
