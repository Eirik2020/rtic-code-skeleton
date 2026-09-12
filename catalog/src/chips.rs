//! Chip-family facts: coarse, shared by every system using a chip. Not an
//! exact-MPN capability database — see `AGENTS.md`. One file per chip.

mod f401;
mod f405;
mod f411;

pub(crate) use f401::PART as STM32F401RETX;
pub(crate) use f405::PART as STM32F405RGTX;
pub(crate) use f411::PART as STM32F411CEUX;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Chip {
    F401,
    F405,
    F411,
}

impl Chip {
    pub const ALL: [Chip; 3] = [Chip::F401, Chip::F405, Chip::F411];

    pub fn cargo_feature(self) -> &'static str {
        match self {
            Chip::F401 => "f401",
            Chip::F405 => "f405",
            Chip::F411 => "f411",
        }
    }

    pub fn from_cargo_feature(feature: &str) -> Option<Chip> {
        Chip::ALL
            .into_iter()
            .find(|chip| chip.cargo_feature() == feature)
    }

    pub fn config(self) -> &'static ChipConfig {
        match self {
            Chip::F401 => &f401::CONFIG,
            Chip::F405 => &f405::CONFIG,
            Chip::F411 => &f411::CONFIG,
        }
    }
}

impl std::fmt::Display for Chip {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.cargo_feature())
    }
}

/// Registration/linker metadata for one exact MCU part. Mechanical build
/// metadata, not a capability authority — see `AGENTS.md`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Part {
    pub name: &'static str,
    pub probe_chip: &'static str,
    pub flash_kib: u32,
    pub ram_kib: u32,
    pub ccm_kib: Option<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChipConfig {
    pub chip: Chip,
    pub pac_feature: &'static str,
    pub max_sysclk_hz: u32,
    pub default_part: &'static Part,
}
