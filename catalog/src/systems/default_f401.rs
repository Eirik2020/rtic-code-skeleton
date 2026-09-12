use super::SystemConfig;
use crate::{chips, Chip};

pub(crate) fn config() -> SystemConfig {
    SystemConfig {
        cargo_feature: "system-default-f401",
        name: "Default F401 system",
        part: &chips::STM32F401RETX,
        embed_profile: "default-f401",
        board: None,
        clock: None,
        default_serial_routes: &[],
        chip_without_board: Some(Chip::F401),
    }
}
