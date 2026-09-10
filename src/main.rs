//! Compile-check spike (see rtic2-framework-architecture-plan.md and
//! compile-check-spike-report.md).
//!
//! ONE `#[app]` skeleton (`skeleton.rs`), shared by all boards. Role-to-vector
//! assignment is a per-board type alias table (`board/config_*.rs`, ~2 lines each),
//! per the plan's "Role assignment" and "Local-only resources ... per-board
//! associated type" sections. Build each board independently:
//!
//!   cargo build --no-default-features --features board-a
//!   cargo build --no-default-features --features board-b

#![no_main]
#![no_std]

use panic_halt as _;

mod board;
mod roles;
mod skeleton;
