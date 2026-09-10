//! Motor PWM role. Written once; any board can bind it to any timer vector.

use super::common::RcFrame;
use super::{RoleCtx, RoleState, TimerRole};

pub struct MotorPwm;

pub struct MotorState {
    ticks: u32,
}

impl RoleState for MotorState {
    fn new() -> Self {
        Self { ticks: 0 }
    }
}

impl TimerRole for MotorPwm {
    type State = MotorState;

    fn on_irq(state: &mut Self::State, ctx: RoleCtx<'_>) -> Option<RcFrame> {
        state.ticks = state.ticks.wrapping_add(1);
        if *ctx.failsafe_state != 0 {
            *ctx.esc_state = 0;
        } else {
            *ctx.esc_state = ctx.esc_state.wrapping_add(1);
        }
        None
    }
}
