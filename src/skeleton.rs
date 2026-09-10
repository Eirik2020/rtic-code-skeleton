//! THE `#[app]` skeleton — written once, per MCU family, never duplicated per board.
//!
//! Which role each timer vector plays is a per-board type alias (`crate::board`),
//! resolved at compile time. Adding a board adds a role table, not another app.

#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [USART1])]
mod app {
    use crate::board::{Tim2Role, Tim3Role};
    use crate::roles::common::RcFrame;
    use crate::roles::{rc_capture, RoleCtx, RoleState, TimerRole};

    #[shared]
    struct Shared {
        esc_state: u32,
        failsafe_state: u32,
    }

    #[local]
    struct Local {
        // Per-board associated types: each board gets exactly the state its
        // assigned roles need, with no `#[cfg]` anywhere near the struct.
        tim2_state: <Tim2Role as TimerRole>::State,
        tim3_state: <Tim3Role as TimerRole>::State,
    }

    #[init]
    fn init(_cx: init::Context) -> (Shared, Local) {
        (
            Shared {
                esc_state: 0,
                failsafe_state: 0,
            },
            Local {
                tim2_state: <<Tim2Role as TimerRole>::State as RoleState>::new(),
                tim3_state: <<Tim3Role as TimerRole>::State as RoleState>::new(),
            },
        )
    }

    #[task(binds = TIM2, shared = [esc_state, failsafe_state], local = [tim2_state], priority = 3)]
    fn tim2(cx: tim2::Context) {
        let frame = (cx.shared.esc_state, cx.shared.failsafe_state).lock(|esc, fs| {
            <Tim2Role as TimerRole>::on_irq(
                cx.local.tim2_state,
                RoleCtx {
                    esc_state: esc,
                    failsafe_state: fs,
                },
            )
        });
        if let Some(frame) = frame {
            rc_frame_ready::spawn(frame).ok();
        }
    }

    #[task(binds = TIM3, shared = [esc_state, failsafe_state], local = [tim3_state], priority = 3)]
    fn tim3(cx: tim3::Context) {
        let frame = (cx.shared.esc_state, cx.shared.failsafe_state).lock(|esc, fs| {
            <Tim3Role as TimerRole>::on_irq(
                cx.local.tim3_state,
                RoleCtx {
                    esc_state: esc,
                    failsafe_state: fs,
                },
            )
        });
        if let Some(frame) = frame {
            rc_frame_ready::spawn(frame).ok();
        }
    }

    #[task(shared = [failsafe_state], priority = 2)]
    async fn rc_frame_ready(mut cx: rc_frame_ready::Context, frame: RcFrame) {
        cx.shared
            .failsafe_state
            .lock(|f| rc_capture::on_frame(f, frame));
    }

    // TEMP: gated OFF under the active feature set (board-a is default,
    // board-b is NOT active). USART3 doesn't exist on F401. This is the
    // precise "correctly excluded" case — does rust-analyzer's OWN
    // diagnostics stay clean here, the way a source-level cfg reader would
    // expect, or does it fail identically to cargo build?
    #[cfg(feature = "board-b")]
    #[task(binds = USART3, priority = 1)]
    fn ra_probe(_cx: ra_probe::Context) {}
}
