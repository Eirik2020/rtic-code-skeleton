//! Serial protocol identities and their intrinsic settings. A protocol's
//! baud rate, parity, and stop bits are fixed by the protocol itself, not
//! chosen per board or per system — see `catalog::Protocol`, which names
//! these same identities for a system's default routing declaration.
//!
//! Not every variant is routed by a default today (only Nucleo's CRSF/MSP
//! routes exist) — the full vocabulary is defined so a future system can
//! route to it without adding a variant first.
#![allow(dead_code)]

use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::serial::config::{Config, StopBits};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SerialProtocol {
    /// FrSky-originated RC link: 100000 baud, even parity, 2 stop bits,
    /// inverted UART signal. This HAL's `serial::config::Config` has no
    /// signal-inversion option, so a board wiring SBUS to a port needs its
    /// own hardware inverter (or a chip/pin capable of RX inversion) — not
    /// something this settings translation can express.
    Sbus,
    /// ExpressLRS/Crossfire RC link with built-in telemetry: 420000 baud,
    /// 8N1, not inverted.
    Crsf,
    /// Betaflight/iNav configurator protocol: 115200 baud, 8N1.
    Msp,
    /// MAVLink telemetry: 57600 baud, 8N1 (the common default; some setups
    /// run it faster).
    Mavlink,
}

impl SerialProtocol {
    pub fn serial_config(self) -> Config {
        match self {
            SerialProtocol::Sbus => Config::default()
                .baudrate(100_000.bps())
                .parity_even()
                .stopbits(StopBits::STOP2),
            SerialProtocol::Crsf => Config::default().baudrate(420_000.bps()),
            SerialProtocol::Msp => Config::default().baudrate(115_200.bps()),
            SerialProtocol::Mavlink => Config::default().baudrate(57_600.bps()),
        }
    }
}

/// The settings an enabled serial port gets when no protocol has claimed it
/// (yet) — a generic default, not tied to any protocol identity.
pub fn default_serial_config() -> Config {
    Config::default().baudrate(115_200.bps())
}

/// Resolves a port's default route into its actual settings, falling back
/// to `default_serial_config()` when unrouted. The one place that combines
/// "did the routing layer assign this port a protocol" with "what settings
/// does that imply."
pub fn resolve(route: Option<SerialProtocol>) -> Config {
    route.map(SerialProtocol::serial_config).unwrap_or_else(default_serial_config)
}
