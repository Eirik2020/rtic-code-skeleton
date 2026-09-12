//! Common setup the RTIC app body needs but isn't itself part of it: monotonic
//! time and the HAL types used by every chip/system selection's `#[init]`.

pub use rtic_monotonics::systick::prelude::*;
pub use stm32f4xx_hal::prelude::*;

// SysTick is common to all three Cortex-M4 chips and does not consume an
// STM32 timer or one of RTIC's software-task dispatchers.
systick_monotonic!(Mono, 1_000);
