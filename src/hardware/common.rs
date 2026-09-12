//! Common STM32F4 resource aliasing, shared across boards to avoid every
//! board file repeating the same verbose peripheral-splitting boilerplate.
//! Skeleton: only what's actually used today. Add a port/DMA controller here
//! when a board actually needs it, not before.
//!
//! Not wired into any board yet — each board still splits its own GPIO ports
//! directly. Worth revisiting once a second board needs the same set of
//! ports, not before.
#![allow(dead_code)]

use stm32f4xx_hal::pac;

pub struct Common {
    pub gpioa: pac::GPIOA,
}

pub fn split(dp: pac::Peripherals) -> Common {
    Common { gpioa: dp.GPIOA }
}

/// Constructed board hardware plus the effective system clock, for a board
/// that takes the whole `pac::Peripherals` and freezes RCC itself rather
/// than receiving an already-frozen `&mut Rcc`. Needed because a board can
/// only take the full peripheral bundle (avoiding one parameter per
/// peripheral) if it extracts and freezes `RCC` before anything else is
/// split out of it — see `docs/architecture/decisions.md`.
pub struct Initialized<T> {
    pub hardware: T,
    pub sysclk_hz: u32,
}
