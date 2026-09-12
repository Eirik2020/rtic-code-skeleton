//! Generic peripheral construction, board-agnostic. Boards pick concrete
//! instances/pins and call into these; the logic for *how* to build a given
//! kind of peripheral lives here exactly once. One file per peripheral kind.

pub mod common;
pub mod gpio;
pub mod pwm;
pub mod serial;
pub mod timer;
