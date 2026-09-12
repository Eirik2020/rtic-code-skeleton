//! Generic UART/USART construction. Board-agnostic — a board file picks the
//! concrete instance, pins, and settings; this owns how a serial port
//! actually gets initialized.
//!
//! The one place allowed to call `SerialExt::serial` directly — see
//! `clippy.toml`.
#![allow(clippy::disallowed_methods)]

use stm32f4xx_hal::{
    gpio::PushPull,
    pac,
    rcc::Rcc,
    serial::{
        Instance, Serial, SerialExt,
        config::{Config, InvalidConfig},
    },
};

pub fn new<USART: Instance>(
    usart: USART,
    pins: (impl Into<USART::Tx<PushPull>>, impl Into<USART::Rx<PushPull>>),
    config: Config,
    rcc: &mut Rcc,
) -> Result<Serial<USART>, InvalidConfig> {
    usart.serial(pins, config, rcc)
}

// Constructed-type aliases for each USART this chip family has. The
// underlying `pac::USARTn` type is a chip fact (which peripherals exist),
// not a board fact — a board only chooses which of these it wires up and to
// which pins. Named after the real instance, not the board's role for it.
pub type Usart1 = Serial<pac::USART1>;
pub type Usart2 = Serial<pac::USART2>;
pub type Usart6 = Serial<pac::USART6>;
