use super::{Chip, ChipConfig, Part};

pub(crate) static PART: Part = Part {
    name: "STM32F405RGTx",
    probe_chip: "STM32F405RGTx",
    flash_kib: 1024,
    // SRAM1 + SRAM2; CCM is a separate address region, not contiguous RAM.
    ram_kib: 128,
    ccm_kib: Some(64),
};

pub(crate) static CONFIG: ChipConfig = ChipConfig {
    chip: Chip::F405,
    pac_feature: "stm32f4xx-hal/stm32f405",
    max_sysclk_hz: 168_000_000,
    default_part: &PART,
};
