# Board wiring ergonomics: clean-slate proposal

Status: implemented for the NUCLEO-F401RE prototype on 2026-09-12. The concrete
implementation is compile-tested but has not been tested on hardware. The Rust
example below records the design sketch; the source is authoritative where the
implementation refined its shape.

See the repository [purpose](../architecture/overview.md#purpose),
[current hardware ownership](../architecture/hardware-and-systems.md), and
[current initialization](../architecture/initialization-and-time.md).

## Intended authoring experience

Make a board module read like a wiring worksheet using ordinary typed Rust.
The board author chooses physical instances, pins, and electrical properties.
Small reusable drivers handle behavior that benefits from reuse; direct HAL
calls remain an option where they already express the wiring clearly.

| Hardware | Board wiring or allocation | Operating settings and behavior |
| --- | --- | --- |
| LED | Physical name, pin, active polarity | Initial state, on/off/toggle |
| Serial port | Physical name, USART instance, TX/RX pins, selected DMA wiring | Baud rate, framing, buffering, interrupt handling |
| External clock | Input frequency, crystal versus externally driven input | Requested system frequency and RCC setup |
| Dedicated periodic timer | Reserved timer instance | Period, start/stop, interrupt acknowledgement |
| PWM group | Timer instance, channels, pins, polarity | Shared frequency, channel pulse/duty updates, output enable |

Operating settings are supplied during initialization. Software capabilities
remain independently selected and are assigned to available hardware ports at
boot, following [hardware/software routing](../architecture/hardware-software-routing.md).
Neither a port name nor its baud rate establishes a permanent software role.

For ordinary periodic software jobs, use a shared monotonic clock with
independently scheduled tasks. Dedicated timers are reserved for requirements
such as hardware triggering, finer timing resolution, or a specifically
prioritized interrupt. Each PWM timer owns a shared timebase: its channels can
have different duty cycles, but do not have independently adjustable periods.

## Illustrative board module

The implemented version is
[`src/boards/nucleo_f401re.rs`](../../src/boards/nucleo_f401re.rs). It keeps
clock selection in common configuration, consumes the complete PAC peripheral
bundle, applies the Nucleo-specific HSE-bypass adjustment, and returns
`hardware::common::Initialized<Board>` with the effective system frequency. It
uses the polarity-aware `hardware::gpio::OutputPin` and the common
`hardware::timer::Periodic` wrapper.

This example uses the API shapes inspected in the installed
`stm32f4xx-hal` 0.23.0 sources. It assumes a Nucleo with two external LEDs added,
all wired active-high. `led1` names the onboard PA5 user LED (LD2 on the PCB);
`led2` and `led3` name the added LEDs. They are not the Nucleo's PCB LED numbers.
`serial1`, `serial2`, and `serial3` are physical port identifiers backed by
USART1, USART2, and USART6 respectively.

The assumed external clock is an 8 MHz input supplied through the appropriate
ST-LINK clock connection. Its availability depends on the actual board wiring;
the example does not detect that wiring. A crystal would not use HSE bypass.

```rust
use crate::hardware::{
    common::Initialized,
    gpio::OutputPin,
    timer::Periodic,
};
use stm32f4xx_hal::{
    pac,
    prelude::*,
    rcc::Config as ClockConfig,
    serial::{config::Config as SerialConfig, Serial},
    time::Hertz,
    timer::pwm::{PwmChannel, PwmHzManager, C1, C2},
};

pub struct Settings {
    pub serial1: SerialConfig,
    pub serial2: SerialConfig,
    pub serial3: SerialConfig,
    pub timer1_frequency: Hertz,
    pub timer2_frequency: Hertz,
    pub pwm_frequency: Hertz,
}

pub type Led = OutputPin;
pub type Timer1 = Periodic<pac::TIM3>;
pub type Timer2 = Periodic<pac::TIM5>;

pub struct PwmOutputs {
    pub timer: PwmHzManager<pac::TIM4>,
    pub output1: PwmChannel<pac::TIM4, C1>,
    pub output2: PwmChannel<pac::TIM4, C2>,
}

pub struct Board {
    pub led1: Led,
    pub led2: Led,
    pub led3: Led,
    pub serial1: Serial<pac::USART1>,
    pub serial2: Serial<pac::USART2>,
    pub serial3: Serial<pac::USART6>,
    pub timer1: Timer1,
    pub timer2: Timer2,
    pub pwm: PwmOutputs,
}

impl Board {
    pub fn new(
        device: pac::Peripherals,
        clock: ClockConfig,
        settings: Settings,
    ) -> Initialized<Self> {
        let mut rcc = device.RCC.freeze(clock.bypass_hse_oscillator());
        let sysclk_hz = rcc.clocks.sysclk().raw();

        let a = device.GPIOA.split(&mut rcc);
        let b = device.GPIOB.split(&mut rcc);
        let c = device.GPIOC.split(&mut rcc);

        // Board file: choose concrete pins and instances.
        let led1 = OutputPin::new(a.pa5, true);
        let led2 = OutputPin::new(b.pb0, true);
        let led3 = OutputPin::new(b.pb1, true);

        let timer1 = Periodic::new(device.TIM3, &mut rcc, settings.timer1_frequency);
        let timer2 = Periodic::new(device.TIM5, &mut rcc, settings.timer2_frequency);

        // Serial and PWM calls associate concrete pins through the HAL.
        // See the source file for the complete, compile-checked constructor.
        todo!()
    }
}
```

With `Settings` imported from the active board module, firmware initialization
supplies operating choices:

```rust
let initialized = boards::active::Board::new(
    cx.device,
    crate::clock::config(),
    Settings {
        serial1: SerialConfig::default().baudrate(115_200.bps()),
        serial2: SerialConfig::default().baudrate(115_200.bps()),
        serial3: SerialConfig::default().baudrate(9_600.bps()),
        timer1_frequency: 1.Hz(),
        timer2_frequency: 10.Hz(),
        pwm_frequency: 400.Hz(),
    },
);

Mono::start(cx.core.SYST, initialized.sysclk_hz);
let board = initialized.hardware;
```

This call is a fragment, not a complete RTIC initializer. Startup still starts
the monotonic from the actual frozen clock, applies timer settings, installs
boot routes, and distributes hardware into RTIC resources.

## Why these types and boundaries

`hardware::gpio::OutputPin` erases LED pin identity after construction, keeping
the public LED type stable when a pin moves. It retains active polarity and
exposes logical `on()` and `off()` operations. Rust ownership still prevents
consuming the same pin value twice.

Serial ports and timers retain concrete peripheral types because those matter
for interrupt and resource ownership. HAL trait checks validate the mappings
used by actual construction. Additional guardrails should remain at chip-family
level; no exact-MPN capability database is proposed.

The PWM manager owns shared period changes, while channel handles control
individual outputs. RTIC must make ownership of that manager explicit so one
consumer cannot unexpectedly change another channel's frequency.

## Remaining decisions

- Add common hardware helpers under `src/hardware` as more reusable behavior is
  needed. PCB board modules retain concrete instance and pin selection only.
- Resolve duplication between catalog resource declarations and actual board
  construction. The required physical manifest remains part of the architecture;
  this example does not replace it or define a new schema or generation scheme.
- Define UART buffering, DMA ownership, interrupt servicing, and boot routing.
  The example constructs serial peripherals but does not provide those drivers.
- Allocate RTIC interrupt bindings and dispatchers together with hardware timers.
  The illustrated TIM3/TIM4/TIM5 allocation cannot simply be pasted into the
  current task graph without reconciling its existing bindings.
- Decide how initialization preserves peripherals that other startup code needs:
  the example consumes the complete PAC peripheral bundle and returns only its
  selected resources.
- Replace the illustrative `expect` policy with the intended handling of invalid
  boot settings. Validate requested frequencies and port/role compatibility
  before enabling normal operation.

No parser, custom macro, cfg extension, or executable configuration schema is
specified here. Extending the existing board-manifest prototype remains subject
to the repository's explicit DSL-permission boundary.

## Evaluation target

Evaluate a board with several LEDs, multiple serial ports, a shared software
timebase, dedicated periodic timers, and a PWM group. Check how many places must
change to move a pin, add a port, or change an operating frequency. Pin changes
should remain local to board wiring; operating settings should remain local to
initialization inputs. Adding a port also requires explicit RTIC integration.

The concrete HAL types and mappings compile for the Nucleo target. Clock and
output behavior still require verification on the actual board.
