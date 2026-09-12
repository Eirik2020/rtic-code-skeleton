# Hardware and systems

Status: current chip, part, and board ownership model.

A **chip** describes the MCU family supported by the HAL/PAC. A **system**
selects a chip and a part, optionally a board (physical wiring — enabled
peripherals and their construction) and a clock selection. Common hardware
behavior lives under `src/hardware`; the current Nucleo prototype selects its
concrete resources in an ordinary typed Rust board module — see
[initialization and monotonic time](initialization-and-time.md). Software
roles remain independent and are connected to hardware by boot configuration;
see [hardware and software routing](hardware-software-routing.md).

The current build files still name exact parts to obtain linker memory sizes and
probe identifiers. Those strings are mechanical build metadata, not the basis
of peripheral capability decisions.

ST NUCLEO-F401RE (the board referred to as NUCLEO-F401RET) is a system selecting
chip `f401` and part `STM32F401RETx`. ST's
[board information](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
identifies it as a Nucleo-64 board with an STM32F401RE MCU.

| System | Feature | Chip | Current linker target | Board setup |
| --- | --- | --- | --- | --- |
| Nucleo report prototype | `system-nucleo-f401re` | f401 | STM32F401RETx | PA5 onboard LED; PB0/PB1 external LEDs; USART1/2/6; TIM3/TIM5 periodic timers; TIM4 PWM on PB6/PB7; 8 MHz bypassed HSE input |
| Default F401 system | `system-default-f401` | f401 | STM32F401RETx | None |
| Default F405 system | `system-default-f405` | f405 | STM32F405RGTx | None |
| Default F411 system | `system-default-f411` | f411 | STM32F411CEUx | None |

Default systems are minimal configurations using the generic skeleton. They do
not assume an LED pin, external peripherals or board clock setup. They are
useful starting points for registering actual hardware. They are explicitly
selected features, not Cargo defaults or fallback systems.

## Ownership

| Owner | Location | Contents |
| --- | --- | --- |
| Chip catalog | `catalog/src/chips.rs` + one file per chip under `catalog/src/chips/` | `Chip` enum and shared types in `chips.rs`; each chip's `pac_feature`, `max_sysclk_hz`, and `default_part` (`probe_chip`/`flash_kib`/`ram_kib`/optional `ccm_kib`) in its own file (e.g. `chips/f401.rs`); not an MPN capability catalog |
| System catalog | `catalog/src/systems.rs` + one file per system under `catalog/src/systems/` | Shared types (`SystemConfig`, `ClockSelection`) and the `all()` aggregator in `systems.rs`; each system's own firmware name, part, Cargo feature, Embed profile, optional board, optional clock selection in its own file (e.g. `systems/nucleo_f401re.rs`). Declares `chip` itself only when it has no board; a system with one takes chip from it instead — see the board catalog row |
| Board catalog | `catalog/src/boards.rs` + one file per board under `catalog/src/boards/` | Board name, chip, and enabled peripheral/pin/DMA *names* only (`Peripheral`/`Pin`/`Dma` enums — no mode, pin letter, or DMA stream/channel detail), one file per board (currently only `boards/nucleo_f401re.rs`). Oscillator setup (HSE frequency/bypass mode) is not here — it's hand-written in the board's own `src/boards/<name>.rs` alongside its other wiring, see [decisions.md](decisions.md#hsi-vs-hse-selection-lives-in-the-board-not-the-catalog) |
| Common hardware behavior | `src/hardware/<kind>.rs` | Board-agnostic, generic peripheral-driver construction — one file per kind: `gpio.rs` (polarity-aware outputs), `timer.rs` (periodic interrupt timers), `serial.rs` (UART/USART), `pwm.rs` (PWM timers). Enforced by Clippy's `disallowed-methods` (`clippy.toml`): the raw HAL methods that actually configure a driver may only be called from here |
| Board-specific setup | `src/boards/<name>.rs`, re-exported as `src/boards.rs`'s `active` | One file per board (currently only `nucleo_f401re.rs`) picks concrete instances/pins, calls the generic constructors above for actual driver setup, does its own pin-to-channel/pin-to-signal wiring (a wiring choice, not construction logic), and exposes its `Board` bundle and operating `Settings` |
| Clock configuration | `src/clock.rs` | Generated `Config` value from the active system's clock selection, validated against the chip's `max_sysclk_hz` |
| Shared types and lookup | `catalog` | `BoardConfig`/`ChipConfig`/`SystemConfig` and friends, `system_config`/`all_systems`, used identically by `build.rs` and the RTIC reducer. Plain Rust data, not a parsed format — see [decisions.md](decisions.md#toml-deferred-for-chipsystemboard-data) |
| Application graph | `src/app_body_skeleton.rs` | Unified clock/monotonic init, shared resources/tasks, system resources and IRQ bindings |
| Flashing | `Embed.toml` and `.cargo/config.toml` | System probe profiles and matching aliases |

`build.rs` reads the chip catalog directly (a normal Rust dependency, not a
parsed file). For a selected system, the build validates its chip, current
linker record, feature, and probe-profile mapping before writing `memory.x`.
Chip-only builds use the catalog's `default_part` for linking. These records
must not be expanded into exhaustive per-MPN peripheral descriptions.

The Nucleo prototype deliberately delegates peripheral, pin, and DMA validity
to the real `stm32f4xx_hal` types and trait bounds used in
`src/boards/nucleo_f401re.rs` itself — an invalid mapping fails normal
compilation there, with no generated intermediate layer needed to catch it.
This provides a chip-family-level guardrail without maintaining an exact-MPN
capability database; see the [prototype specification](board-toml-prototype.md).

## Per-system board manifest

The adopted architecture requires every system eventually to have a board
manifest — a `BoardConfig` value in `catalog/src/boards.rs` today; a
`board.toml` per system once TOML returns, per
[decisions.md](decisions.md#toml-deferred-for-chipsystemboard-data).

This is a physical hardware manifest, following the same broad separation used
by Betaflight targets. It is the single source of truth for a board's physical
facts, and records:

- the chip family the board uses;
- which peripheral instances the system enables;
- the pins assigned to each enabled peripheral signal;
- the DMA mapping selected for peripheral transfers.

It must not assign RC input, MSP, GPS, telemetry, or another software capability
to a hardware port. Those associations belong to boot-time configuration as
described in [hardware and software routing](hardware-software-routing.md).

The explicitly authorized NUCLEO-F401RE prototype now contains this manifest;
see the [prototype specification](board-toml-prototype.md). The system catalog
entry (`catalog/src/systems.rs`) is separate build-registration and
linker/probe metadata; it does not repeat any fact the board manifest already
declares, chip included — when a system has a board, `build.rs` reads chip
from it, not from the system's own entry. Other systems do not yet have
manifests, and extending the prototype requires fresh DSL permission
regardless of which format currently expresses it.

## Selecting firmware

```powershell
cargo build-nucleo-f401re
cargo embed-nucleo-f401re

cargo build-default-f401
cargo build-default-f405
cargo build-default-f411

cargo embed-default-f401
cargo embed-default-f405
cargo embed-default-f411
```

Each build alias enables its `system-...` feature. Each embed alias also picks
the matching Embed profile. Add `--release` for optimized firmware. The Nucleo
aliases select `system-nucleo-f401re,sw-report`; the default-system aliases
select only their corresponding system feature.

Use the same feature in `rust-analyzer.cargo.features`. At most one system may
be selected, even when two systems share a chip. A system automatically enables
its chip. An explicit extra chip feature must match; incompatible chips fail.

Chip-only `build-f401` / `build-f405` / `build-f411` remain available, without a
system. The old chip-only `embed-f401` command no longer initializes the Nucleo
LED; use `embed-nucleo-f401re` for that firmware.

The Nucleo system reserves TIM3 as the 1 Hz report trigger, TIM5 as a 10 Hz
periodic interrupt, TIM4 as a 400 Hz two-channel PWM timebase, and TIM2 as its
RTIC software-task dispatcher. The three configured LEDs blink from SysTick.
Default systems and chip-only skeletons use TIM2 and TIM3 as dispatchers. The
F405-only UART4 task remains chip-gated as a current test resource. In the target
architecture, peripheral availability belongs to hardware while roles such as
RC input, MSP, GPS, and telemetry are separate software capabilities assigned at
boot.

The skeleton has one shared `#[init]`. Standard chip and system feature gates
select `Shared` and `Local` resources, their initializer bindings, and system
hardware. The reducer removes inactive fields and expressions before RTIC
expansion, so F405-only resources and Nucleo hardware initialization are absent
from other configurations.

The unified initializer freezes RCC exactly once, using the active system's
generated clock configuration (or bare HSI when none is declared) — for a
system with a board, that generated code calls the board's own hand-written
`clock_config()` (e.g. the Nucleo's HSE bypass mode) and applies the target
sysclk on top; the oscillator choice itself is never made by `build.rs` or at
`#[init]` time, only by the board. *Who* freezes RCC differs by path: a boardless system freezes
it directly in `#[init]`; a board freezes it itself, inside `Board::new`,
because `Board::new` takes the whole `pac::Peripherals` bundle rather than
one parameter per peripheral (so its signature doesn't grow with every pin a
board adds), and freezing RCC is the first thing that has to happen before
anything else is split out of that bundle. Either way, the initializer starts
a 1 kHz `rtic-monotonics` SysTick monotonic from the resulting system clock
frequency once it's known. SysTick is common to all three Cortex-M4 chips and
does not consume an STM32 timer or an RTIC dispatcher. Which path runs (and
the `Local` fields it feeds) is gated on board-capability cfgs
(`board_pin`/`board_peripheral`), never a system's own feature name, so the
skeleton's control flow doesn't change shape as boards are added.

**`app_body_skeleton.rs` must never name a board module directly, or branch
on a specific system's feature name to decide board-hardware control flow.**
It is shared and cfg-reduced across every chip/system selection, so any
reference to hardware it needs goes through `crate::boards::active` (`Board`,
`Board::new`, and its pin/timer type aliases), never `crate::boards::<name>::...`,
and any conditional construction is gated on what a board provides
(`board_pin`/`board_peripheral`), never on `feature = "system-<name>"`.
Each board's own file stays fully hand-written and board-specific —
Betaflight-style per-board wiring is the intended shape, not something to
make generic — but the skeleton itself stays board-agnostic through this one
indirection. Today there is exactly one board, so `active` always resolves to
`nucleo_f401re`; adding a second board only means adding its own
`pub use <name> as active;` arm to `src/boards.rs`, cfg-gated on that
system's feature, with no change needed to the skeleton.

## Adding another system

1. Add `catalog/src/systems/<name>.rs` with a `pub(crate) fn config() ->
   SystemConfig` and register it in `systems.rs::all()`. Supply the linker and
   probe metadata required by the current shape without adding per-MPN
   capability data. Once it needs real hardware, add
   `catalog/src/boards/<name>.rs` with a `pub(crate) fn config() ->
   BoardConfig` declaring enabled peripheral/pin/DMA *names* (the `Peripheral`/
   `Pin`/`Dma` enums — add a variant if the new board needs one that doesn't
   exist yet), register it in `boards.rs`, and reference it from the new
   system.
2. Add a `system-...` Cargo feature enabling the required chip. The catalog
   entry from step 1 is the only registration needed — `build.rs` and the
   reducer both discover it through `catalog::all_systems()`.
3. Add an exclusive selector at the top of the skeleton. Pass both the chip
   and system to `for_chip`; keep the chip-only fallback mutually exclusive.
   Add `src/boards/<name>.rs` with the new board's `Board`/`Board::new` and
   typed HAL wiring. Put common construction, policy, and reusable behavior in
   `src/hardware/<kind>.rs`; keep concrete instance/pin choices in the board.
   Give `Board::new` the whole `pac::Peripherals` bundle (not one parameter
   per peripheral) and have it freeze RCC itself before splitting anything
   else out, following the Nucleo board's shape — this is what keeps the
   signature from growing as the board adds more hardware. Add a matching
   `#[cfg(feature = "system-<name>")] pub use <name> as active;` arm to
   `src/boards.rs` — this is what keeps `app_body_skeleton.rs` from ever
   naming the new board directly. In the skeleton, gate the `Board::new` call
   (and any resources it feeds) on board-capability cfgs
   (`board_pin`/`board_peripheral`), never on `feature = "system-<name>"` —
   see [decisions.md](decisions.md#the-shared-skeleton-never-names-a-board-directly).
4. Add cfg-gated hardware resources, initialization, and driver tasks as needed.
   Do not assign application roles to those ports. Allocate dispatchers so none
   collide with the system's hardware tasks.
5. Add a matching Embed profile, aliases, and positive/negative tests.

The reducer receives the selected system explicitly, preserving deterministic
rust-analyzer expansion. Its supported system predicates come from the same
catalog as the build script. Other cfg axes still need explicit implementation.

System features cannot be combined with `generated-body`: that is a synthetic
legacy fixture. `invalid-interrupt` remains a negative test. The separate chip
`app_head.rs`/`app_body.rs` files belong to those legacy fixtures; normal
development uses the shared source skeleton.

## Memory layout and verification

While moving memory sizes into the part catalogs, the F405 layout was corrected:
its 192 KiB total was previously represented as contiguous RAM at `0x20000000`.
It has 128 KiB there and separate 64 KiB CCM at `0x10000000`; see ST's
[datasheet](https://www.st.com/resource/en/datasheet/stm32f405rg.pdf). The linker
now receives separate `RAM` and `CCM` regions. Ordinary data and stack use `RAM`;
placing data in CCM would require explicit linker sections.

The compile matrix covers all default systems, Nucleo, matching/mismatching
chips, two systems sharing F401, systems using different chips, and legacy
negative fixtures. Reducer tests verify that defaults omit Nucleo hardware,
that only F405 retains UART4, and that each selection retains exactly one init.

The Nucleo's PA5 wiring is represented in the board catalog. TIM3 now triggers the
`report` software task at 1 Hz, while the LED toggles every 500 ms using SysTick.
No firmware was flashed as part of this prototype.

Verified after the split: all four system configurations link ARM ELF files;
their generated linker layouts match the selected parts (including the separate
F405 CCM region). The current compile matrix and parser/reducer tests are recorded
under `../verification/`.
Real rust-analyzer LSP checks pass for each of the four systems: resource
completion, hover, navigation and unsaved field-type updates. The previously
documented limitation in native unsaved task diagnostics remains.
