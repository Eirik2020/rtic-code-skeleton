//! Generic periodic hardware timer construction. Board-agnostic — a board
//! file picks the concrete timer instance and frequency; this owns how a
//! periodic, interrupt-driven counter gets initialized.
//!
//! The one place allowed to call `TimerExt::counter_hz` directly — see
//! `clippy.toml`.
#![allow(clippy::disallowed_methods)]

use stm32f4xx_hal::{
    ClearFlags, Listen, pac,
    rcc::Rcc,
    time::Hertz,
    timer::{CounterHz, Event, Flag, Instance, TimerExt},
};

pub struct Periodic<TIM: Instance> {
    timer: CounterHz<TIM>,
}

impl<TIM: Instance> Periodic<TIM> {
    pub fn new(tim: TIM, rcc: &mut Rcc, freq: Hertz) -> Self {
        let mut timer = tim.counter_hz(rcc);
        timer.start(freq).unwrap();
        timer.listen(Event::Update);
        Self { timer }
    }

    pub fn clear_interrupt(&mut self) {
        self.timer.clear_flags(Flag::Update);
    }
}

// Constructed-type aliases for each timer this chip family has, for the same
// reason `serial.rs` aliases its USARTs: which timers exist is a chip fact,
// not a board fact.
pub type Tim3 = Periodic<pac::TIM3>;
pub type Tim5 = Periodic<pac::TIM5>;
