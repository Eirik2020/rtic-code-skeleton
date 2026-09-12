//! Independently-compiled software capabilities. A capability here knows
//! nothing about which physical port, if any, it ends up connected to —
//! that association happens at boot, in `crate::routing`. See
//! `docs/architecture/hardware-software-routing.md`.

pub mod serial_protocol;
