# Board hardware construction: target architecture

Status: **superseded implementation record**. The name-only board manifest was
retained, but the Nucleo prototype now uses readable, typed HAL construction in
its board module. See [board wiring ergonomics](board-wiring-ergonomics.md) and
the current [architecture decisions](../architecture/decisions.md). The text
below records the earlier generic-constructor approach.

## Decision recap

1. `build.rs` keeps learning *which* peripherals/pins/DMA a board declares
   from a small, name-only fact in `catalog` (Approach A) — not by parsing
   real construction code.
2. Real peripheral construction stops living in `catalog` (host-only) and
   moves to `src/` (embedded target) as plain `stm32f4xx_hal` code — no
   generated type aliases, no generated `DMASet` assertions. The compiler's
   own type-checking of real construction *is* the validity check.
3. Two safeguards close the gaps that move introduces:
   - `unexpected_cfgs = "deny"` — a declared-but-ungated or gated-but-
     undeclared peripheral becomes a hard build failure, not a silent
     omission.
   - Clippy `disallowed-methods`/`disallowed-types` — raw HAL driver
     construction (pin-mode conversion, `Serial::new`, DMA stream setup, ...)
     is only permitted inside `src/hardware/`; board files may split ports
     and name resources, but may not reimplement construction logic.

## Shape: generic logic vs. board-specific wiring

Same split already established for `Hardware`/`hardware.rs` — kept, just with
finer-grained generic pieces:

- **`src/hardware/<kind>.rs`** — one file per peripheral *kind* (`gpio.rs`,
  `timer.rs`, later `uart.rs`, ...). Generic, reusable, board-agnostic
  constructors using real `stm32f4xx_hal` types and trait bounds directly.
- **`src/boards/<name>.rs`** — one file per board (new; today's single
  `nucleo_f401re` case). Picks concrete pins/instances and calls the generic
  constructors. This is where the `Hardware` struct/`init` now lives — its
  *shape* (which fields exist) is inherently board-specific, so it can't stay
  generic the way the individual constructors can.

## `catalog::BoardConfig`, shrunk

Today:

```rust
pub struct BoardConfig {
    pub name: &'static str,
    pub chip: Chip,
    pub hse_hz: Option<u32>,
    pub peripherals: BTreeMap<&'static str, PeripheralConfig>, // pins, dma, timer detail
}
```

`PeripheralConfig`, `PinConfig`, `PinMode`, `ActiveLevel`, `DmaConfig`,
`DmaDirection`, `TimerConfig` all exist solely to describe *how* to build
something — once building happens in real `src/` code, none of it is needed.

Target:

```rust
pub struct BoardConfig {
    pub name: &'static str,
    pub chip: Chip,
    pub hse_hz: Option<u32>,
    pub peripherals: BTreeSet<Peripheral>, // {Gpioa, Tim3}
    pub pins: BTreeSet<Pin>,               // {StatusLed}
    pub dma: BTreeSet<Dma>,                // {} today
}
```

(`Peripheral`/`Pin`/`Dma` are small enums, not bare strings — see "Decided"
below for why and what they look like.)

`has_peripheral`/`has_pin`/`has_dma` become plain `BTreeSet::contains` calls.
`build.rs` reads this exactly as it reads the current version — same cfg
flags emitted, just sourced from a flatter type.

## `build.rs`: `generate_board_module` goes away

No more PAC type-alias generation, no more `DMASet` trait-check generation,
no more `board_config.rs` written to `OUT_DIR`. `configure_board` shrinks to:
read the board's three name sets, emit `board_peripheral`/`board_pin`/
`board_dma` cfg flags and their `rustc-check-cfg` value lists. That's the
entire remaining job.

## Concrete migration: today's two peripherals

**`src/hardware/gpio.rs`** (new, generic):

```rust
pub struct OutputPin<P> {
    pin: P,
}

impl<P: OutputPin_> OutputPin<P> {
    pub fn new(mut pin: P, active_high: bool) -> Self {
        if active_high { pin.set_low(); } else { pin.set_high(); }
        Self { pin }
    }

    pub fn toggle(&mut self) {
        self.pin.toggle();
    }
}
```

**`src/hardware/timer.rs`** (new, generic):

```rust
pub struct Periodic<T> {
    timer: CounterHz<T>,
}

impl<T: Instance> Periodic<T> {
    pub fn new(tim: T, rcc: &mut Rcc, freq: Hertz) -> Self {
        let mut timer = tim.counter_hz(rcc);
        timer.start(freq).unwrap();
        timer.listen(Event::Update);
        Self { timer }
    }

    pub fn clear_interrupt(&mut self) {
        self.timer.clear_flags(Flag::Update);
    }
}
```

**`src/boards/nucleo_f401re.rs`** (new — replaces the board-specific half of
today's `src/hardware.rs`):

```rust
pub struct Hardware {
    #[cfg(board_pin = "status_led")]
    pub status_led: hardware::gpio::OutputPin<gpioa::PA5<Output<PushPull>>>,
    #[cfg(board_peripheral = "tim3")]
    pub report_timer: hardware::timer::Periodic<TIM3>,
}

impl Hardware {
    pub fn init(gpioa: pac::GPIOA, tim3: pac::TIM3, rcc: &mut Rcc) -> Self {
        Self {
            #[cfg(board_pin = "status_led")]
            status_led: {
                let parts = gpioa.split(rcc);
                hardware::gpio::OutputPin::new(parts.pa5.into_push_pull_output(), true)
            },
            #[cfg(board_peripheral = "tim3")]
            report_timer: hardware::timer::Periodic::new(tim3, rcc, 1.Hz()),
        }
    }
}
```

`app_body_skeleton.rs` changes only its one call site:
`crate::hardware::Hardware::init(...)` → `crate::boards::nucleo_f401re::Hardware::init(...)`.
`Local`'s field types become the generic `hardware::gpio::OutputPin<...>` /
`hardware::timer::Periodic<...>`, not board-named types — same shape as
today, just sourced from the generic module instead of a board-flavored one.

(Exact generic type signatures above are illustrative — the real trait
bounds depend on what `stm32f4xx_hal` actually exposes for a generic output
pin / generic timer instance, to be nailed down during implementation, not
guessed here.)

## Safeguards to add alongside this

1. **`Cargo.toml`**:
   ```toml
   [lints.rust]
   unexpected_cfgs = "deny"
   ```
2. **`clippy.toml`** `disallowed-methods` / `disallowed-types`: list the
   specific `stm32f4xx_hal` construction paths (pin-mode conversions, driver
   `::new` calls, DMA stream configuration) that may only appear inside
   `src/hardware/`. Exact list is implementation work — depends on which
   HAL methods each new `hardware/<kind>.rs` module actually touches.

## Decided

- **`catalog`'s name sets become enums, not `&'static str`.** Same reasoning
  as `PinMode`/`ActiveLevel`: a small, cheap win even though the consuming
  `#[cfg(...)]` side stays string-keyed regardless (see
  `peripheral-registration-approaches.md`). Each enum variant maps to its cfg
  string via a `.cfg_name()`-style method, mirroring how `Chip::cargo_feature()`
  already works:

  ```rust
  #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
  pub enum Peripheral { Gpioa, Tim3 }

  impl Peripheral {
      pub fn cfg_name(self) -> &'static str {
          match self {
              Peripheral::Gpioa => "gpioa",
              Peripheral::Tim3 => "tim3",
          }
      }
  }

  pub struct BoardConfig {
      ...
      pub peripherals: BTreeSet<Peripheral>,
      pub pins: BTreeSet<Pin>,   // same pattern
      pub dma: BTreeSet<Dma>,    // same pattern
  }
  ```

- **Add a skeleton `src/hardware/common.rs` now, expand as needed.** Not a
  fully generalized "split every port" helper up front (only GPIOA is used
  today) — just the shape that lets a second port/DMA controller be added
  later without restructuring board files again:

  ```rust
  pub struct Common {
      pub gpioa: pac::GPIOA,
      // more ports/DMA controllers added here as boards actually need them
  }

  pub fn split(dp: pac::Peripherals) -> Common {
      Common { gpioa: dp.GPIOA }
  }
  ```

## Migration steps, in order

1. Add the safeguards first (`unexpected_cfgs = "deny"`, initial empty/minimal
   `clippy.toml`) against the *current* code, to confirm they don't fire
   false positives before anything else changes.
2. Shrink `catalog::BoardConfig` and delete `PeripheralConfig`/`PinConfig`/
   `PinMode`/`ActiveLevel`/`DmaConfig`/`DmaDirection`/`TimerConfig`; update
   `catalog`'s own tests.
3. Simplify `build.rs`'s `configure_board` to the name-set-only version;
   delete `generate_board_module`, `pin_parts`, `pascal_case` if nothing else
   uses them.
4. Add `src/hardware/gpio.rs` and `src/hardware/timer.rs`; delete the
   board-specific `StatusLed`/`ReportTimer` from `src/hardware.rs`.
5. Add `src/boards/nucleo_f401re.rs` with the new `Hardware` struct/`init`;
   update `app_body_skeleton.rs`'s one call site and `Local` field types.
6. Fill in the Clippy disallow-list once the real HAL method names used by
   step 4 are known.
7. Full verification: `cargo test -p catalog`, `cargo test -p rtic-app-cfg`,
   `cargo build-nucleo-f401re`, all default/chip-only checks, full
   `tools/run-test.ps1`, plus `cargo clippy` to confirm the new lint config
   passes clean on the migrated code.

## Docs to update once implemented

`decisions.md` (new entry), `hardware-and-systems.md` (ownership table),
`board-toml-prototype.md` (manifest scope shrinks), `initialization-and-time.md`.
