//! ST NUCLEO-F401RE: LD2 on PA5, driven by the TIM3 update interrupt.

use stm32f4xx_hal::{
    gpio::{Output, Pin, PushPull},
    pac,
    prelude::*,
    timer::{CounterHz, Event, Flag},
};

pub struct Hardware {
    led: Pin<'A', 5, Output<PushPull>>,
    blink_timer: CounterHz<pac::TIM3>,
}

impl Hardware {
    pub fn new(dp: pac::Peripherals) -> Self {
        let mut rcc = dp.RCC.freeze(Default::default());
        let gpioa = dp.GPIOA.split(&mut rcc);
        let mut led = gpioa.pa5.into_push_pull_output();
        led.set_low();

        let mut blink_timer = dp.TIM3.counter_hz(&mut rcc);
        blink_timer.start(10.Hz()).unwrap();
        blink_timer.listen(Event::Update);

        Self { led, blink_timer }
    }

    pub fn on_blink_interrupt(&mut self) {
        self.blink_timer.clear_flags(Flag::Update);
        self.led.toggle();
    }
}
