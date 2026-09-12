# NUCLEO-F401RE board manifest prototype

Status: implemented, explicitly authorized prototype pending review. Originally
TOML; currently typed Rust data in `catalog` — see
[decisions.md](decisions.md#toml-deferred-for-chipsystemboard-data). The
authorization and DSL-permission boundary below is unchanged by that move: it
governs the *manifest's scope* (what a board may declare), not which format
currently expresses it.

## Scope

The prototype is `catalog::boards::nucleo_f401re::config()`. The system's entry
in `catalog::systems::all()` points to it via its `board` field. Default
systems do not yet have board manifests (`board: None`).

The manifest currently declares a name and the *names* of the peripherals/pins
it enables — nothing about how to build them. HSE's frequency and drive mode
are not catalog data; see the note below.

```rust
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
```

`Peripheral`/`Pin`/`Dma` are small enums (typo-proofed, not bare strings —
see [decisions.md](decisions.md#board-manifests-declare-names-not-construction-detail)).
Their `.cfg_name()` produces the `board_peripheral`/`board_pin`/`board_dma`
cfg values. Inside the RTIC application, the source-preserving reducer
evaluates those predicates from the same `BoardConfig` value `build.rs` uses —
one definition, not two parses to keep in sync. `name` is not a cfg
predicate — `build.rs` reads it directly, currently only for documentation.

Only peripherals the shared skeleton (`app_body_skeleton.rs`) actually needs
to conditionally include/exclude belong in `Peripheral` — each USART/TIM
variant backs a `Local` field and task gated individually. GPIO ports
(GPIOA/B/C) are *not* listed: `Board::new` is already conditionally compiled
as a whole, so nothing in the shared skeleton ever needs to ask "does this
board have GPIOA" as a separate fact — it would only produce an unused
`board_peripheral` cfg value. A `Peripheral`/`Pin` entry with no
corresponding `#[cfg(...)]` anywhere in `src/` is a sign it shouldn't be
here.

HSE's frequency and drive mode are *not* catalog data. This project's domain
is flight controllers, which always run from HSE when a board provides one —
there's no independent HSI-vs-HSE choice to model, so there's nothing for a
catalog field to disambiguate. The board owns its own oscillator setup
directly, as hand-written Rust: `crate::boards::nucleo_f401re::clock_config()`
returns the board's `rcc::Config` base (HSE frequency, bypass mode), and
`build.rs`'s generated `crate::clock::config()` calls it when the system has a
board, applying only the system's chosen `target_hz` on top — see
[initialization and monotonic time](initialization-and-time.md#clock-configuration)
and [decisions.md](decisions.md#hsi-vs-hse-selection-lives-in-the-board-not-the-catalog).

## Compiler-backed mapping checks

There is no generated intermediate layer anymore. `src/boards/nucleo_f401re.rs`
constructs the board's peripherals with real `stm32f4xx_hal` types, calling
generic, board-agnostic constructors from `src/hardware/` for every kind of
peripheral-driver setup — GPIO (`gpio.rs`), periodic timers (`timer.rs`),
serial ports (`serial.rs`), and PWM (`pwm.rs`) — and only picking concrete
instances, pins, and pin-to-channel associations itself. An invalid pin or
peripheral mapping fails ordinary Rust compilation exactly where it's written,
using the selected PAC and HAL as the capability authority. There is no
exact-MPN pin or DMA database.

Mode, active level, and DMA direction used to be validated by a generated
type-alias/trait-assertion step in `build.rs` (`generate_board_module`,
removed); now that the same construction is hand-written real code, that
generation step has nothing left to do — the compiler checks the same thing
directly. See [decisions.md](decisions.md#common-hardware-behavior-stays-out-of-board-declarations)
and [board wiring ergonomics](../roadmap/board-wiring-ergonomics.md).

The prototype was originally checked manually with PA99 (failed on the
missing HAL GPIO field) and DMA99 (failed on the missing PAC type) against the
now-removed generated layer; the same invalid mappings fail the same way
today, just directly in `src/boards/nucleo_f401re.rs` instead of generated
code.

## Hardware and software behavior

- SysTick provides the 1 kHz `Mono` monotonic.
- The `blink` software task drives PA5 and the two assumed external LEDs on
  PB0/PB1, waiting 500 ms between states on `Mono`.
- TIM3 runs at 1 Hz. Its hardware task clears the update flag and consults the
  boot-initialized report route.
- TIM5 runs at 10 Hz and updates the example shared counter.
- TIM4 supplies one 400 Hz timebase for PWM channels on PB6/PB7. Both outputs
  start enabled at zero duty.
- USART1/2/6 are constructed as physical serial ports. No protocol role is
  assigned to them by the board.
- The independent Cargo feature `sw-report` compiles the `report` software task.
- When that software is selected, boot initialization enables the TIM3 route and
  the hardware task spawns `report`.
- `report` emits `defmt::info!("Hello World!")` through `defmt-rtt`.

The Nucleo Cargo aliases select both `system-nucleo-f401re` and `sw-report`.
Selecting the system feature directly without `sw-report` still constructs and
services TIM3 but leaves the report route disabled.

## Prototype limitations

- Only NUCLEO-F401RE is registered in the shared catalog.
- UART buffering, DMA transfers, and the generalized boot router are not yet
  implemented. The serial IRQ tasks currently only establish static ownership.
- No general boot configuration store or router exists. The current route is a
  boolean installed during `init`.
- The manifest does not identify RTIC interrupt bindings. The Nucleo USART1/2/6
  and TIM3/TIM5 bindings remain hand-written and are checked by compilation.
- Cross-entry pin/DMA conflict detection is not implemented.
- The board's HSE frequency/bypass mode (in `src/boards/nucleo_f401re.rs`'s
  `clock_config()`) is not cross-checked against any other clock fact (there
  is no PLL-divider feasibility modeling — see [initialization and monotonic
  time](initialization-and-time.md#clock-configuration)).

Any extension beyond this boundary requires explicit DSL permission — whether
expressed as Rust data today or TOML again later.
