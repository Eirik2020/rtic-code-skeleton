//! RC capture role. Written once; any board can bind it to any timer vector.

use super::common::{CaptureBuffer, RcFrame};
use super::{RoleCtx, RoleState, TimerRole};

pub struct RcCapture;

impl RoleState for CaptureBuffer {
    fn new() -> Self {
        CaptureBuffer::new()
    }
}

impl TimerRole for RcCapture {
    type State = CaptureBuffer;

    fn on_irq(state: &mut Self::State, _ctx: RoleCtx<'_>) -> Option<RcFrame> {
        state.push_edge();
        state.try_decode()
    }
}

/// Applied by the skeleton's software task once a frame is decoded.
pub fn on_frame(failsafe_state: &mut u32, frame: RcFrame) {
    *failsafe_state = if frame.0 == 0 { 1 } else { 0 };
}
