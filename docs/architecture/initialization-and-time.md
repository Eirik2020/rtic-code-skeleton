# Initialization and monotonic time

Status: current startup design and known gap.

## Current initializer

The RTIC skeleton contains exactly one `#[init]` function for every supported
selection. It currently performs these operations in order:

1. Freeze RCC exactly once, using `crate::clock::config()` — already includes
   any board-specific adjustment (e.g. the Nucleo's HSE bypass mode) baked in
   at build time, so the *generated Config* carries board-specific detail,
   not `#[init]`'s control flow. A board with real hardware freezes RCC
   itself, inside `Board::new` (see step 5); a boardless system freezes it
   directly in `#[init]`. Which path runs is gated on board *capability*
   (`board_pin`/`board_peripheral`), never a system's own feature name — see
   [decisions.md](decisions.md#the-shared-skeleton-never-names-a-board-directly).
2. Start a 1 kHz SysTick monotonic using the actual frozen system-clock rate,
   from whichever path ran.
3. Create common application resource values.
4. Create F405-only test resources under the existing chip feature.
5. For a board, call `crate::boards::active::Board::new` once, passing the
   whole PAC peripheral bundle plus the generated clock config and operating
   settings — not one parameter per peripheral, which would grow without
   bound as a board adds more hardware. It freezes RCC itself before
   splitting anything else out of the bundle, then builds three LEDs,
   USART1/2/6, TIM3/TIM5 periodic timers, and the two-channel TIM4 PWM group,
   returning both the built `Board` and the effective `sysclk_hz` (see
   `hardware::common::Initialized`).
6. Install the prototype report route, start PWM at zero duty, and spawn the
   monotonic LED task.
7. Assemble the initialized values into RTIC's `Shared` and `Local` owners.

Initialization expressions are placed in local bindings before resource assembly.
The `Shared` and `Local` literals only establish RTIC ownership.

## Monotonic time

`rtic-monotonics` provides a 1 kHz SysTick monotonic named `Mono`. SysTick is
present across the three Cortex-M4 targets and does not consume TIM2, TIM3, or an
RTIC software-task dispatcher. The initializer supplies
`rcc.clocks.sysclk().raw()` to avoid duplicating the effective system frequency.

The current SysTick implementation is periodic. A different monotonic backend
would require an explicit timer/interrupt ownership decision.

On NUCLEO-F401RE, the async `blink` task drives all three configured LEDs on and
off with 500 ms delays on `Mono`. PA5 is onboard; PB0/PB1 assume externally
connected LEDs. TIM3 is not involved in blinking.

## Clock configuration

A system's `catalog::SystemConfig` may declare a clock selection — just a
target sysclk, nothing about which oscillator to use:

```rust
ClockSelection { target_hz: 84_000_000 }
```

`build.rs` validates `target_hz` against the selected chip's `max_sysclk_hz`
(a chip-family fact on `catalog::ChipConfig` — 84 MHz for F401, 100 MHz for
F411, 168 MHz for F405) and generates a real `stm32f4xx_hal::rcc::Config`
expression consumed by `crate::clock::config()` (`src/clock.rs`). A system
with no clock selection gets bare `Config::hsi()`, identical to every build
before this existed.

There is no separate HSI-vs-HSE choice for `build.rs` to make: when a system
has a board, the generated code calls that board's own hand-written
`clock_config()` for the base `Config` and applies `.sysclk(target_hz)` on
top; a boardless system gets bare `Config::hsi()`. A board's `clock_config()`
owns its own oscillator setup entirely — for the Nucleo prototype,
`src/boards/nucleo_f401re.rs`'s `clock_config()` returns
`Config::hse(8.MHz()).bypass_hse_oscillator()` (ST-LINK's MCO drives HSE
externally, not a crystal). This project's domain is flight controllers,
which always run from HSE when a board provides one, so there's no real case
for a board with HSE deliberately running on HSI — see
[decisions.md](decisions.md#hsi-vs-hse-selection-lives-in-the-board-not-the-catalog).

This validates only the chip's maximum sysclk ceiling, matching the roadmap's
original Stage 3 example exactly. It does not model PLL-divider feasibility —
whether a specific `target_hz` is exactly reachable from the selected source
is left to the HAL's own `freeze()` at device runtime, same as before this
existed.

## Chip versus system initialization

The current code has a unified RTIC entry point, not unified hardware support for
all three chips.

- Common clock and monotonic setup executes for F401, F405, and F411.
- `crate::boards::active::Board::new` constructs the Nucleo resources selected
  by the board manifest. It calls generic GPIO, serial, timer, and PWM
  constructors from `src/hardware/` for every kind of peripheral-driver setup,
  and only picks concrete instances, pins, and pin-to-channel associations
  itself. It takes the whole `pac::Peripherals` bundle (so its parameter list
  doesn't grow with every peripheral the board adds) and therefore freezes
  RCC itself, before splitting anything else out of the bundle — a boardless
  system freezes RCC directly in `#[init]` instead. The shared skeleton
  reaches it through `crate::boards::active`, never a board's own name, and
  chooses which path runs based on board-capability cfgs
  (`board_pin`/`board_peripheral`), never a system's own feature name — see
  [decisions.md](decisions.md#the-shared-skeleton-never-names-a-board-directly).
- Default F401, F405, and F411 systems do not construct board peripherals.
- There are no `chips::f401::init`, `chips::f405::init`, or
  `chips::f411::init` layers today.

This distinction must remain explicit in reviews: the presence of one `#[init]`
function does not imply equivalent hardware initialization across chips.

## Ownership direction

Without defining a configuration DSL, a future implementation can use ordinary
Rust functions and existing Cargo feature selection:

- common startup owns core-wide concerns such as the selected clock result and
  monotonic start;
- board manifests (`catalog`) own only physical pins, connected devices, and
  enabled ports — declarative names, never construction code;
- common construction, policy, and reusable behavior live in
  `src/hardware/<kind>.rs`;
- a board's typed setup lives in `src/boards/<name>.rs`, where it selects
  concrete instances and pins and calls the common helpers;
- software capabilities such as RC input remain independent of physical ports;
- boot initialization validates and installs the configured associations between
  those software capabilities and enabled ports;
- the RTIC initializer assembles the resulting values into visible resource
  ownership.

Boot-time routing changes logical associations, not RTIC's static ownership or
interrupt bindings. The concrete routing interface remains an open design item.

The concrete return types and resources for F401, F405, and F411 remain an open
design item. They should be derived from real hardware requirements rather than
invented merely to make symmetric branches.

## Timer allocation

- SysTick: monotonic clock.
- TIM3 on Nucleo: 1 Hz interrupt that can spawn the `report` software task.
- TIM4 on Nucleo: shared 400 Hz PWM timebase for PB6/PB7.
- TIM5 on Nucleo: 10 Hz periodic interrupt.
- TIM2 on Nucleo: RTIC dispatcher.
- TIM2 and TIM3 on chip-only/default systems: RTIC dispatchers.

Any new initialization must check this allocation before taking a timer.
