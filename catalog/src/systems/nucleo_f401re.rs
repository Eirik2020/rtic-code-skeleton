use super::{ClockSelection, SerialRoute, SystemConfig};
use crate::{Pin, Protocol, boards, chips};

pub(crate) fn config() -> SystemConfig {
    SystemConfig {
        cargo_feature: "system-nucleo-f401re",
        name: "Nucleo report prototype",
        part: &chips::STM32F401RETX,
        embed_profile: "nucleo-f401re",
        board: Some(boards::nucleo_f401re::config()),
        clock: Some(ClockSelection {
            target_hz: 84_000_000,
        }),
        default_serial_routes: &[
            SerialRoute {
                pin: Pin::Serial1,
                protocol: Protocol::Crsf,
            },
            SerialRoute {
                pin: Pin::Serial2,
                protocol: Protocol::Msp,
            },
        ],
        chip_without_board: None,
    }
}
