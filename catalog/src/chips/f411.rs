use super::{Chip, ChipConfig, Part};

pub(crate) static PART: Part = Part {
    name: "STM32F411CEUx",
    probe_chip: "STM32F411CEUx",
    flash_kib: 512,
    ram_kib: 128,
    ccm_kib: None,
};

pub(crate) static CONFIG: ChipConfig = ChipConfig {
    chip: Chip::F411,
    pac_feature: "stm32f4xx-hal/stm32f411",
    max_sysclk_hz: 100_000_000,
    default_part: &PART,
};
