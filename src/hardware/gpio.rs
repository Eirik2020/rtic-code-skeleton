//! Board-agnostic digital output construction and polarity handling.

use stm32f4xx_hal::gpio::{AnyPin, Output, Pin, PinMode, PinState};

/// A type-erased push-pull output with logical active/inactive operations.
///
/// Erasing the concrete pin keeps a board's public resource type stable when
/// its wiring moves to another GPIO.
pub struct OutputPin {
    pin: AnyPin<Output>,
    active_high: bool,
}

impl OutputPin {
    pub fn new<const PORT: char, const NUM: u8, PriorMode: PinMode>(
        pin: Pin<PORT, NUM, PriorMode>,
        active_high: bool,
    ) -> Self {
        let inactive = if active_high {
            PinState::Low
        } else {
            PinState::High
        };

        Self {
            pin: pin.into_push_pull_output_in_state(inactive).erase(),
            active_high,
        }
    }

    pub fn on(&mut self) {
        if self.active_high {
            self.pin.set_high();
        } else {
            self.pin.set_low();
        }
    }

    pub fn off(&mut self) {
        if self.active_high {
            self.pin.set_low();
        } else {
            self.pin.set_high();
        }
    }
}
