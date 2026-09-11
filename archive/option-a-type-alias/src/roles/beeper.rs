//! Beeper role. Only ever bound on chips that actually expose a vector for it
//! (e.g. UART4, absent on F401) — proves a role can be genuinely chip-specific,
//! not just board-specific.

use super::{RoleCtx, RoleState, TimerRole};

pub struct Beeper;

pub struct BeeperState {
    beeps: u32,
}

impl RoleState for BeeperState {
    fn new() -> Self {
        Self { beeps: 0 }
    }
}

impl TimerRole for Beeper {
    type State = BeeperState;

    fn on_irq(state: &mut Self::State, _ctx: RoleCtx<'_>) -> Option<super::common::RcFrame> {
        state.beeps = state.beeps.wrapping_add(1);
        None
    }
}
