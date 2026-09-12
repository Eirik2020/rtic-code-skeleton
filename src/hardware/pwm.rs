//! Generic hardware-timer PWM construction. Board-agnostic — a board file
//! picks the concrete timer instance, frequency, and which channels/pins to
//! attach; this owns how the timer itself enters PWM mode.
//!
//! The one place allowed to call `PwmExt::pwm_hz` directly — see
//! `clippy.toml`.
#![allow(clippy::disallowed_methods)]

use stm32f4xx_hal::{
    pac,
    rcc::Rcc,
    time::Hertz,
    timer::pwm::{C1, C2, PwmChannel, PwmExt, PwmHzManager},
};

pub fn new<TIM: PwmExt>(tim: TIM, freq: Hertz, rcc: &mut Rcc) -> (PwmHzManager<TIM>, TIM::Channels) {
    tim.pwm_hz(freq, rcc)
}

// Constructed-type aliases for TIM4's PWM manager and its first two channels
// — a chip fact (which timer, which channels it has), not a board fact.
pub type Tim4 = PwmHzManager<pac::TIM4>;
pub type Tim4Ch1 = PwmChannel<pac::TIM4, C1>;
pub type Tim4Ch2 = PwmChannel<pac::TIM4, C2>;
