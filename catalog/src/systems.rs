//! System (firmware) configuration: which part and Embed profile, which board
//! (if any), and which clock. Registration/build metadata plus a clock
//! selection — physical facts belong to the board a system selects, when it
//! has one. One file per system.

use crate::{BoardConfig, Chip, Part, Pin, Protocol};

mod default_f401;
mod default_f405;
mod default_f411;
mod nucleo_f401re;

/// A system's default association between one of its board's serial ports
/// and a software protocol — the boot-time routing default (see
/// `docs/architecture/hardware-software-routing.md`). This is a *default*,
/// not a permanent binding: the physical port and the protocol are each
/// still independently selectable (the port exists regardless of which
/// protocol, if any, is compiled; the protocol's own logic is an
/// independent Cargo feature). `build.rs` validates that `pin` is actually
/// one this system's board declares.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SerialRoute {
    pub pin: Pin,
    pub protocol: Protocol,
}

/// A system's target sysclk. Which oscillator actually gets used (HSI, or
/// the board's HSE if it has one) isn't a separate choice here — a board
/// that provides HSE is expected to always be run from it (this project's
/// domain is flight controllers; there's no real case for deliberately
/// running a board's firmware on HSI instead when a crystal exists). The
/// active board's own `clock_config()` decides that; see
/// `docs/architecture/decisions.md`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClockSelection {
    pub target_hz: u32,
}

#[derive(Clone, Debug)]
pub struct SystemConfig {
    pub cargo_feature: &'static str,
    pub name: &'static str,
    pub part: &'static Part,
    pub embed_profile: &'static str,
    pub board: Option<BoardConfig>,
    pub clock: Option<ClockSelection>,
    /// Default serial protocol routes for this system. Empty for systems
    /// with no board (nothing to route) or that simply don't assign any
    /// port a default role yet.
    pub default_serial_routes: &'static [SerialRoute],
    /// Only meaningful when `board` is `None`. A system with a board takes
    /// its chip from the board instead — the board is the single source of
    /// truth for physical facts, chip included.
    chip_without_board: Option<Chip>,
}

impl SystemConfig {
    pub fn chip(&self) -> Chip {
        self.board
            .as_ref()
            .map(|board| board.chip)
            .or(self.chip_without_board)
            .expect("system must declare a chip when it has no board")
    }
}

pub fn all() -> Vec<SystemConfig> {
    vec![
        nucleo_f401re::config(),
        default_f401::config(),
        default_f405::config(),
        default_f411::config(),
    ]
}
