//! MCU clock configuration, generated from the active system's `[clock]`
//! selection (or the chip default when none is declared) and validated
//! against the chip's maximum sysclk.

include!(concat!(env!("OUT_DIR"), "/clock_config.rs"));
