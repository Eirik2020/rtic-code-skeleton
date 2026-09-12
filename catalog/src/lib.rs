//! Shared build-time data: chip-family facts, system/firmware configuration,
//! and board (physical hardware) manifests. Consumed only by `build.rs` and
//! the `rtic_app_cfg::for_chip` proc-macro — both plain Rust running at build
//! time — so this is plain Rust data rather than a parsed format. TOML is
//! deliberately deferred here, not rejected; see `docs/architecture/decisions.md`.
//!
//! A board declares only *which* peripherals/pins/DMA mappings it enables —
//! not how to build them. Reusable construction and behavior live in
//! `src/hardware/`; concrete selection and wiring live in `src/boards/`.

use std::collections::BTreeSet;

mod boards;
mod chips;
mod systems;

pub use chips::{Chip, ChipConfig, Part};
pub use systems::{ClockSelection, SerialRoute, SystemConfig};

/// Looks up a system by its Cargo feature name (e.g. `"system-nucleo-f401re"`).
pub fn system_config(feature: &str) -> Option<SystemConfig> {
    all_systems()
        .into_iter()
        .find(|system| system.cargo_feature == feature)
}

/// Every registered system. Used to find which one's Cargo feature is active.
pub fn all_systems() -> Vec<SystemConfig> {
    systems::all()
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoardConfig {
    pub name: &'static str,
    pub chip: Chip,
    pub peripherals: BTreeSet<Peripheral>,
    pub pins: BTreeSet<Pin>,
    pub dma: BTreeSet<Dma>,
}

impl BoardConfig {
    pub fn has_peripheral(&self, name: &str) -> bool {
        self.peripherals.iter().any(|p| p.cfg_name() == name)
    }

    pub fn has_pin(&self, name: &str) -> bool {
        self.pins.iter().any(|p| p.cfg_name() == name)
    }

    pub fn has_dma(&self, name: &str) -> bool {
        self.dma.iter().any(|d| d.cfg_name() == name)
    }
}

/// Enabled peripheral instances. Grows as boards need more; not an exact-MPN
/// capability database — see `AGENTS.md`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Peripheral {
    Usart1,
    Usart2,
    Usart6,
    Tim3,
    Tim4,
    Tim5,
}

impl Peripheral {
    pub fn cfg_name(self) -> &'static str {
        match self {
            Peripheral::Usart1 => "usart1",
            Peripheral::Usart2 => "usart2",
            Peripheral::Usart6 => "usart6",
            Peripheral::Tim3 => "tim3",
            Peripheral::Tim4 => "tim4",
            Peripheral::Tim5 => "tim5",
        }
    }
}

/// Named pin/signal mappings a board exposes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pin {
    Led1,
    Led2,
    Led3,
    Serial1,
    Serial2,
    Serial3,
    Pwm1,
    Pwm2,
}

impl Pin {
    pub fn cfg_name(self) -> &'static str {
        match self {
            Pin::Led1 => "led1",
            Pin::Led2 => "led2",
            Pin::Led3 => "led3",
            Pin::Serial1 => "serial1",
            Pin::Serial2 => "serial2",
            Pin::Serial3 => "serial3",
            Pin::Pwm1 => "pwm1",
            Pin::Pwm2 => "pwm2",
        }
    }
}

/// A serial protocol's identity. Independent of any board or port — its
/// intrinsic HAL settings (baud rate, parity, ...) live in the embedded
/// firmware's `crate::software::serial_protocol::SerialProtocol`, not here;
/// this only names the protocol so a system can declare a default route.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Protocol {
    Sbus,
    Crsf,
    Msp,
    Mavlink,
}

impl Protocol {
    /// Must match a variant name on the embedded `SerialProtocol` enum —
    /// `build.rs` uses this to generate a reference to it.
    pub fn variant_name(self) -> &'static str {
        match self {
            Protocol::Sbus => "Sbus",
            Protocol::Crsf => "Crsf",
            Protocol::Msp => "Msp",
            Protocol::Mavlink => "Mavlink",
        }
    }
}

/// Named DMA mappings a board exposes. No board uses one yet.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dma {}

impl Dma {
    pub fn cfg_name(self) -> &'static str {
        match self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nucleo_prototype_matches_expected_hardware() {
        let system = system_config("system-nucleo-f401re").unwrap();
        assert_eq!(system.name, "Nucleo report prototype");
        assert_eq!(system.chip(), Chip::F401);
        assert_eq!(
            system.clock,
            Some(ClockSelection {
                target_hz: 84_000_000,
            })
        );

        let board = system.board.unwrap();
        assert_eq!(board.name, "ST NUCLEO-F401RE");
        assert_eq!(board.chip, Chip::F401);
        assert!(board.has_peripheral("usart1"));
        assert!(board.has_peripheral("usart2"));
        assert!(board.has_peripheral("usart6"));
        assert!(board.has_peripheral("tim3"));
        assert!(board.has_peripheral("tim4"));
        assert!(board.has_peripheral("tim5"));
        assert!(board.has_pin("led1"));
        assert!(board.has_pin("led2"));
        assert!(board.has_pin("led3"));
        assert!(board.has_pin("serial1"));
        assert!(board.has_pin("serial2"));
        assert!(board.has_pin("serial3"));
        assert!(board.has_pin("pwm1"));
        assert!(board.has_pin("pwm2"));
        assert!(!board.has_dma("unused"));

        assert_eq!(
            system.default_serial_routes,
            &[
                SerialRoute {
                    pin: Pin::Serial1,
                    protocol: Protocol::Crsf,
                },
                SerialRoute {
                    pin: Pin::Serial2,
                    protocol: Protocol::Msp,
                },
            ]
        );
    }

    #[test]
    fn default_serial_routes_only_name_pins_the_board_declares() {
        for system in all_systems() {
            let Some(board) = system.board.as_ref() else {
                assert!(
                    system.default_serial_routes.is_empty(),
                    "{}: default_serial_routes requires a board",
                    system.cargo_feature
                );
                continue;
            };
            for route in system.default_serial_routes {
                assert!(
                    board.has_pin(route.pin.cfg_name()),
                    "{}: default route names pin '{}', which board '{}' does not provide",
                    system.cargo_feature,
                    route.pin.cfg_name(),
                    board.name
                );
            }
        }
    }

    #[test]
    fn default_systems_have_no_board_or_clock() {
        for feature in [
            "system-default-f401",
            "system-default-f405",
            "system-default-f411",
        ] {
            let system = system_config(feature).unwrap();
            assert!(system.board.is_none(), "{feature}");
            assert!(system.clock.is_none(), "{feature}");
        }
    }

    #[test]
    fn unknown_system_is_none() {
        assert!(system_config("system-unknown").is_none());
    }

    #[test]
    fn every_chip_has_a_default_part() {
        for chip in Chip::ALL {
            let config = chip.config();
            assert_eq!(config.chip, chip);
            assert!(config.max_sysclk_hz > 0);
            assert!(config.default_part.flash_kib > 0);
            assert!(config.default_part.ram_kib > 0);
        }
    }
}
