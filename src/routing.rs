//! Default hardware/software routing, generated from the active system's
//! `default_serial_routes` (see `docs/architecture/hardware-software-routing.md`).
//! One function per named serial port, returning its default protocol route.

include!(concat!(env!("OUT_DIR"), "/routing.rs"));
