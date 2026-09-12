//! ST NUCLEO-F401RE physical wiring, including two LEDs attached through the
//! expansion headers in addition to the onboard PA5 user LED.
//!
//! The serial names identify physical ports only. Software roles such as GPS,
//! MSP, RC input, and telemetry are assigned separately during boot.

use crate::hardware::{self, common::Initialized, gpio::OutputPin, serial::new as new_serial, timer::Periodic};
use stm32f4xx_hal::{pac, prelude::*, rcc::Config as ClockConfig, serial::config::Config as SerialConfig, time::Hertz};

/// This board's oscillator setup: 8 MHz HSE fed from the onboard ST-LINK's
/// MCO (SB16/SB50 closed), the factory-default configuration per ST's UM1724
/// user guide — not a populated crystal at X3, so HSE is externally driven
/// and needs bypass mode. A PCB fact, not a per-firmware choice: this
/// project's domain (flight controllers) always runs from HSE when a board
/// provides one, so there's no separate HSI-vs-HSE selection to make — see
/// `docs/architecture/decisions.md`.
pub fn clock_config() -> ClockConfig {
    ClockConfig::hse(8.MHz()).bypass_hse_oscillator()
}

/// Operating settings chosen by the firmware, independently of board wiring.
pub struct Settings {
    pub serial1: SerialConfig,
    pub serial2: SerialConfig,
    pub serial3: SerialConfig,
    pub timer1_frequency: Hertz,
    pub timer2_frequency: Hertz,
    pub pwm_frequency: Hertz,
}

pub struct PwmOutputs {
    /// Owns the period shared by both output channels.
    pub timer: hardware::pwm::Tim4,
    pub output1: hardware::pwm::Tim4Ch1,
    pub output2: hardware::pwm::Tim4Ch2,
}

pub struct Board {
    pub led1: OutputPin,
    pub led2: OutputPin,
    pub led3: OutputPin,
    pub serial1: hardware::serial::Usart1,
    pub serial2: hardware::serial::Usart2,
    pub serial3: hardware::serial::Usart6,
    pub timer1: hardware::timer::Tim3,
    pub timer2: hardware::timer::Tim5,
    pub pwm: PwmOutputs,
}

impl Board {
    /// Takes the whole PAC peripheral bundle rather than one parameter per
    /// peripheral, so this signature doesn't grow with every pin/peripheral
    /// this board adds. That requires freezing RCC here, before anything
    /// else is split out of `dp` — see `hardware::common::Initialized`.
    pub fn new(dp: pac::Peripherals, clock: ClockConfig, settings: Settings) -> Initialized<Self> {
        let mut rcc = dp.RCC.freeze(clock);
        let sysclk_hz = rcc.clocks.sysclk().raw();

        let a = dp.GPIOA.split(&mut rcc);
        let b = dp.GPIOB.split(&mut rcc);
        let c = dp.GPIOC.split(&mut rcc);

        let led1 = OutputPin::new(a.pa5, true);
        let led2 = OutputPin::new(b.pb0, true);
        let led3 = OutputPin::new(b.pb1, true);

        let serial1 = new_serial(dp.USART1, (a.pa9, a.pa10), settings.serial1, &mut rcc)
            .expect("invalid serial1 settings");
        let serial2 = new_serial(dp.USART2, (a.pa2, a.pa3), settings.serial2, &mut rcc)
            .expect("invalid serial2 settings");
        let serial3 = new_serial(dp.USART6, (c.pc6, c.pc7), settings.serial3, &mut rcc)
            .expect("invalid serial3 settings");

        let timer1 = Periodic::new(dp.TIM3, &mut rcc, settings.timer1_frequency);
        let timer2 = Periodic::new(dp.TIM5, &mut rcc, settings.timer2_frequency);

        let (pwm_timer, (pwm_ch1, pwm_ch2, ..)) =
            hardware::pwm::new(dp.TIM4, settings.pwm_frequency, &mut rcc);
        let pwm = PwmOutputs {
            timer: pwm_timer,
            output1: pwm_ch1.with(b.pb6),
            output2: pwm_ch2.with(b.pb7),
        };

        Initialized {
            hardware: Self {
                led1,
                led2,
                led3,
                serial1,
                serial2,
                serial3,
                timer1,
                timer2,
                pwm,
            },
            sysclk_hz,
        }
    }
}
