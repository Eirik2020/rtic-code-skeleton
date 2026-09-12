//! Per-board setup: which concrete instances/pins this board uses and how its
//! typed HAL resources are constructed. One file per board.
//!
//! `active` is a compile-time re-export of whichever board the selected
//! system uses. The shared RTIC skeleton (`app_body_skeleton.rs`) must go
//! through `crate::boards::active` and never name a board module directly —
//! that's what keeps it reusable across systems without knowing which board
//! any of them has. Each board's own wiring stays fully hand-written; only
//! this indirection is generic.

#[cfg(feature = "system-nucleo-f401re")]
pub mod nucleo_f401re;

#[cfg(feature = "system-nucleo-f401re")]
pub use nucleo_f401re as active;
