use super::SystemConfig;
use crate::{chips, Chip};

pub(crate) fn config() -> SystemConfig {
    SystemConfig {
        cargo_feature: "system-default-f405",
        name: "Default F405 system",
        part: &chips::STM32F405RGTX,
        embed_profile: "default-f405",
        board: None,
        clock: None,
        default_serial_routes: &[],
        chip_without_board: Some(Chip::F405),
    }
}
