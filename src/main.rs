#![no_main]
#![no_std]

use panic_halt as _;

mod systems;

#[cfg(not(any(feature = "generated-body", feature = "invalid-interrupt")))]
mod app_body_skeleton;

// Keep the original composition fixtures available for regression testing.
#[cfg(any(feature = "generated-body", feature = "invalid-interrupt"))]
include!(concat!(env!("OUT_DIR"), "/", env!("RTIC_GENERATED_APP")));
