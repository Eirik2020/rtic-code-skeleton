//! Board selection. Each board file is just its role table — the `#[app]`
//! skeleton itself is written once, in `src/skeleton.rs`, and never duplicated.

#[cfg_attr(feature = "board-a", path = "config_a.rs")]
#[cfg_attr(feature = "board-b", path = "config_b.rs")]
mod selected;

pub use selected::*;
