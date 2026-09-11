//! Compile-check spike (see rtic2-framework-architecture-plan.md and
//! compile-check-spike-report.md).
//!
//! Two-level selection, testing whether the type-alias approach's cross-chip
//! gap (found in this session: item-level `#[cfg]` can't gate a binding that
//! doesn't exist on the target chip's PAC at all) is fixable without adopting
//! the syn-reducer's tooling cost:
//!
//!   - CHIP selects which skeleton (file-level, cheap, no tooling) —
//!     `chip-f401` -> TIM2/TIM3 only; `chip-f405` -> TIM2/TIM3/UART4.
//!   - BOARD selects role-to-vector wiring within a chip (type alias, already
//!     verified) — `board-a`/`board-b` swap which role sits on TIM2 vs TIM3.
//!
//!   cargo build --no-default-features --features chip-f401,board-a
//!   cargo build --no-default-features --features chip-f401,board-b
//!   cargo build --no-default-features --features chip-f405,board-a

#![no_main]
#![no_std]

use panic_halt as _;

mod board;
mod roles;

#[cfg_attr(feature = "chip-f401", path = "chips/f401_skeleton.rs")]
#[cfg_attr(feature = "chip-f405", path = "chips/f405_skeleton.rs")]
mod skeleton;
