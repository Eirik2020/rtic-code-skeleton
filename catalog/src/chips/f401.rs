use super::{Chip, ChipConfig, Part};

pub(crate) static PART: Part = Part {
    name: "STM32F401RETx",
    probe_chip: "STM32F401RE",
    flash_kib: 512,
    ram_kib: 96,
    ccm_kib: None,
};

pub(crate) static CONFIG: ChipConfig = ChipConfig {
    chip: Chip::F401,
    pac_feature: "stm32f4xx-hal/stm32f401",
    max_sysclk_hz: 84_000_000,
    default_part: &PART,
};
