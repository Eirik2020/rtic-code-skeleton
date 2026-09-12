use super::SystemConfig;
use crate::{chips, Chip};

pub(crate) fn config() -> SystemConfig {
    SystemConfig {
        cargo_feature: "system-default-f411",
        name: "Default F411 system",
        part: &chips::STM32F411CEUX,
        embed_profile: "default-f411",
        board: None,
        clock: None,
        default_serial_routes: &[],
        chip_without_board: Some(Chip::F411),
    }
}
