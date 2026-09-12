#![no_main]
#![no_std]

use panic_halt as _;

#[cfg(feature = "sw-report")]
use defmt_rtt as _;

// Board-agnostic helpers used to construct and operate physical resources.
#[cfg(any(board_pin = "led1", board_peripheral = "tim3"))]
mod hardware;

// Per-board physical wiring. Self-gating per board inside (see src/boards.rs);
// harmless/empty when no board exists.
#[cfg(any(board_pin = "led1", board_peripheral = "tim3"))]
mod boards;

// Independently-compiled software capabilities and the boot-time routing
// that connects them to hardware ports. Only meaningful when a board exists.
#[cfg(any(board_pin = "led1", board_peripheral = "tim3"))]
mod software;
#[cfg(any(board_pin = "led1", board_peripheral = "tim3"))]
mod routing;

// generated-body is a self-contained legacy fixture with its own clock-free
// init and never references this; every other build does.
#[cfg(not(feature = "generated-body"))]
mod clock;

// invalid-interrupt's generated body is text-composed from app_body_skeleton.rs
// (see build.rs), so it needs this module too; generated-body uses its own
// self-contained chip fixture and never references it.
#[cfg(not(feature = "generated-body"))]
mod app_prelude;

#[cfg(not(any(feature = "generated-body", feature = "invalid-interrupt")))]
mod app_body_skeleton;

// Keep the original composition fixtures available for regression testing.
#[cfg(any(feature = "generated-body", feature = "invalid-interrupt"))]
include!(concat!(env!("OUT_DIR"), "/", env!("RTIC_GENERATED_APP")));
