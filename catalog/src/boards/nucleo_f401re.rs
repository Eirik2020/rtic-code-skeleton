use crate::{BoardConfig, Chip, Peripheral, Pin};
use std::collections::BTreeSet;

pub(crate) fn config() -> BoardConfig {
    BoardConfig {
        name: "ST NUCLEO-F401RE",
        chip: Chip::F401,
        peripherals: BTreeSet::from([
            Peripheral::Usart1,
            Peripheral::Usart2,
            Peripheral::Usart6,
            Peripheral::Tim3,
            Peripheral::Tim4,
            Peripheral::Tim5,
        ]),
        // PB0/PB1 are expansion-header connections for two external LEDs in
        // this prototype; only PA5 is an LED populated on the Nucleo PCB.
        pins: BTreeSet::from([
            Pin::Led1,
            Pin::Led2,
            Pin::Led3,
            Pin::Serial1,
            Pin::Serial2,
            Pin::Serial3,
            Pin::Pwm1,
            Pin::Pwm2,
        ]),
        dma: BTreeSet::new(),
    }
}
