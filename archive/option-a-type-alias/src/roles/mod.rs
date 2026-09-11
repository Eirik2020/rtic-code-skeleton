//! Role catalog. RTIC-ignorant: no dependency on `rtic`, unit-testable off target.
//!
//! A role is a type implementing [`TimerRole`]. Boards assign roles to timer
//! vectors with a type alias (see `src/board/`), so the `#[app]` skeleton is
//! written ONCE and never duplicated per board.

pub mod beeper;
pub mod common;
pub mod motor_pwm;
pub mod rc_capture;

use common::RcFrame;

/// Per-role exclusive state. The skeleton stores this in `#[local]` via the
/// associated type, so each board gets exactly the state its roles need.
pub trait RoleState {
    fn new() -> Self;
}

/// Shared-resource view handed to a role after the skeleton has locked.
///
/// Deliberately uniform across roles: RTIC parses `shared = [...]` as literal
/// syntax before typecheck, so the skeleton's lock set cannot vary by role.
/// Cost of that is noted in the report (conservative ceilings).
pub struct RoleCtx<'a> {
    pub esc_state: &'a mut u32,
    pub failsafe_state: &'a mut u32,
}

/// A role bound to a timer interrupt vector.
pub trait TimerRole {
    type State: RoleState;

    /// Returns a decoded RC frame if this IRQ produced one, so the skeleton can
    /// spawn the software task. Roles never touch `spawn()` themselves — that
    /// would drag `rtic` into the role catalog.
    fn on_irq(state: &mut Self::State, ctx: RoleCtx<'_>) -> Option<RcFrame>;
}
