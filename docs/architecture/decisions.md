# Architecture decisions and open items

Status: concise decision register for the current implementation.

## Adopted decisions

### One hand-written RTIC application graph

Tasks, priorities, resource access lists, and interrupt bindings remain explicit
in `src/app_body_skeleton.rs`. RTIC task code is not generated in normal builds.

### Source-preserving cfg reduction

A custom attribute macro removes inactive chip/system structure before RTIC
expansion. This was selected after raw RTIC cfg gating failed for cross-chip
interrupts, duplicate variants, shared fields, and multiple initializers.

The decision is conditional: remove the reducer if the pinned RTIC version can
eventually handle the required patterns directly.

### Chip, build metadata, and system separation

Chip catalogs hold family facts plus the linker/probe records required by the
current build. Systems select a chip and own board integration. Application
roles and RTIC ownership remain in Rust source.

### Family-level capability guardrails

Capability validation is intentionally coarse and chip-family-based. It should
catch common mistakes, such as requesting a peripheral outside the supported
family set, but it must not become an exhaustive exact-MPN database. A
variant-specific mistake may be rejected by Rust, the HAL, or the PAC; that
compile failure is sufficient.

### Hardware and software remain independently selectable

Systems expose physical hardware ports; software capabilities such as RC input,
MSP, GPS, and telemetry do not belong permanently to a UART or other peripheral.
Boot configuration maps compiled software roles onto enabled hardware ports.
RTIC's underlying interrupt bindings and ownership remain compile-time static.

### Default serial-protocol routing, in bounded form

A system may declare `SerialRoute { pin, protocol }` entries
(`SystemConfig.default_serial_routes`) associating one of its board's serial
ports with a `catalog::Protocol` (`Sbus`/`Crsf`/`Msp`/`Mavlink`) by default.
This is the first generalized instance of the boot-routing model above — see
[hardware and software routing](hardware-software-routing.md) for the full
design — rather than a one-off like the existing boolean TIM3-to-report route.

The protocol's own intrinsic settings (baud rate, parity, stop bits) live in
`crate::software::serial_protocol::SerialProtocol`, plain embedded Rust with
no knowledge of boards or ports — the same "compiled independently of
hardware" property the routing model requires. `build.rs` generates
`crate::routing` (one function per named port, returning that system's
default route) from the catalog data, validating that every declared route
names a pin the active board actually provides and that no pin is routed
twice — a build failure otherwise, the same "reject an impossible mapping"
guardrail already used for boards/chips elsewhere. `app_body_skeleton.rs`
calls these functions when assembling `Settings`, translating a routed
protocol into the port's actual `SerialConfig`; an unrouted port keeps a
generic default.

Bounded deliberately: this only decides a port's *settings* and tags it with
an intended role. There is no SBUS/CRSF/MSP/MAVLink frame decoder — reading
and interpreting a port's bytes according to its protocol is a separate,
larger, not-yet-built task. The routing itself is also compiled-in data, not
a persistent or runtime-configurable store; that broader system remains
unspecified and still requires explicit permission to design, per
[hardware and software routing](hardware-software-routing.md#what-remains-unspecified).

### One physical board manifest per system

Every system will have a board manifest that declares its enabled peripheral
instances and their pin and DMA mappings. The manifest contains physical board
facts and selections only; it must not assign software capabilities to ports. An
explicitly authorized NUCLEO-F401RE prototype now exercises this model. Further
schema or system support remains subject to the DSL-permission rule.

A board manifest is the single source of truth for those facts, chip included:
a system with one does not also declare its own `chip`. `build.rs` reads chip
from the board in that case, falling back to the system's own declaration only
for systems that have no board yet (currently the default systems). This keeps
a fact from needing two edits to stay consistent, and sets up board manifests
to be reusable by more than one system later, without requiring that reuse —
or a new location for manifests — now.

### Board manifests declare names, not construction detail

`BoardConfig` holds only `BTreeSet<Peripheral>`/`BTreeSet<Pin>`/`BTreeSet<Dma>`
— small enums, not the `PeripheralConfig`/`PinConfig`/`PinMode`/`ActiveLevel`/
`DmaConfig`/`DmaDirection`/`TimerConfig` detail that used to live alongside
them. That detail existed only to let `build.rs` *generate* PAC type aliases
and `DMASet` trait-check assertions before anything was really built; once
peripheral construction is real `stm32f4xx_hal` code (previous entry), the
compiler's own type-checking of that real code *is* the validity check, and
the generated layer has nothing left to do.

The enums (not bare strings) are the smaller, second-order win discussed in
`docs/roadmap/peripheral-registration-approaches.md`: the consuming
`#[cfg(board_peripheral = "...")]` side stays string-keyed regardless (that's
inherent to `#[cfg(...)]`), but the declaring side gets the same typo
protection already adopted for `PinMode`/`ActiveLevel`/`Chip`.

### Common hardware behavior stays out of board declarations

The board manifest declares only *which* peripherals/pins/DMA a board enables —
names, nothing about how to build them. Reusable construction and behavior live
under `src/hardware/`, one file per peripheral kind: `gpio.rs`, `timer.rs`,
`serial.rs`, `pwm.rs`. The Nucleo board module selects PA5/PB0/PB1 and calls
the common polarity-aware `gpio::OutputPin`; it selects TIM3/TIM5 and calls
`timer::Periodic`; it selects USART1/2/6 and calls `serial::new`; it selects
TIM4 and calls `pwm::new`. The compiler validates the concrete types at those
call sites.

Board-local code still does its own *wiring* — pin-to-signal and
pin-to-channel association (`.with(pin)` on a PWM channel, which TX/RX pins
go to which USART) — since that's choosing what this board connects to what,
not configuring how a peripheral driver works. The line is the same one
`gpio.rs`/`timer.rs` already drew: `.split()` and `.into_push_pull_output()`
stay in board files as wiring choices, while the calls that actually
configure a driver (`counter_hz`+`start`, `serial`, `pwm_hz`) are centralized.
A custom helper that owns policy or repeated construction behavior belongs
under `src/hardware/`, not in a board file.

The board module exposes physical serial ports, LEDs, timers, and a PWM group.
It does not assign a protocol role to any serial port. The PWM manager retains
ownership of the period shared by both channels, while each channel has its own
output handle.

Two safeguards close the gaps this split would otherwise open:

- **`[lints.rust] unexpected_cfgs = "deny"`** (`Cargo.toml`) — a peripheral
  gated in `src/` but never declared in the board manifest (or vice versa)
  is a hard build failure, not a silently-missing feature. See
  `docs/roadmap/peripheral-registration-approaches.md` for the failure-mode
  reasoning.
- **Clippy `disallowed-methods`** (`clippy.toml`) — the HAL methods that
  actually configure a peripheral driver (`TimerExt::counter_hz`,
  `SerialExt::serial`, `PwmExt::pwm_hz`) may only be called from inside
  `src/hardware/`; board modules use `timer::Periodic::new`, `serial::new`,
  and `pwm::new` instead. Extend this list whenever a new kind of driver
  setup is added — a method left off the list is a safeguard gap, not a
  signal that the call is fine in a board file.

### The shared skeleton never names a board directly

`app_body_skeleton.rs` is compiled and cfg-reduced for every chip/system
selection, so it must not reference a specific board module
(`crate::boards::nucleo_f401re::...`) even though only one board exists today.
`src/boards.rs` re-exports whichever board the selected system uses under a
fixed name: `pub use nucleo_f401re as active;`, gated on that system's
feature. The skeleton refers only to types and functions under
`crate::boards::active` — never a board's own name.

The same rule covers a subtler form of the same mistake: branching the
skeleton's *control flow* on a specific system's feature name
(`#[cfg(feature = "system-nucleo-f401re")]`) to decide whether/how to
construct board hardware. That reintroduces exactly the same coupling one
level up — the skeleton would still need editing for every new system, just
keyed on a feature name instead of a module path. The fix is the same one
already used everywhere else in `Local`/`#[init]`: gate on what a board
*provides* (`board_pin`/`board_peripheral`), not on which system it is.

This came up concretely two ways. Any board-specific clock adjustment (the
Nucleo's HSE bypass mode) is baked into the build-time-generated
`crate::clock::config()` (see [Clock
configuration](#clock-configuration-roadmap-stage-3-adopted-in-bounded-form)),
not applied imperatively wherever RCC gets frozen — that part of the *data*
is board-specific, but the *code path* that reads it is not. Separately,
`Board::new` legitimately does own the RCC freeze itself, rather than
receiving an already-frozen `&mut Rcc` — because it takes the whole
`pac::Peripherals` bundle instead of one parameter per peripheral (avoiding a
signature that grows without bound as a board adds hardware), and freezing
RCC first is required before anything else can be split out of that bundle.
A boardless system freezes RCC directly in `#[init]` instead. Both of those
are fine: what would not be fine is choosing between them based on
`feature = "system-nucleo-f401re"` rather than `board_pin`/`board_peripheral`
— the rule is about *what the skeleton branches on*, not whether a board's
constructor is allowed to own RCC-freezing.

This is a one-line indirection, not a generalization of the board DSL itself:
each board file stays fully hand-written and board-specific (Betaflight-style
per-board wiring is the intended, permanent shape — see the repository's
[flight-controller framing](overview.md#purpose)), and adding a board means
adding one more cfg-gated `pub use <name> as active;` arm, not touching the
skeleton. What this rule prevents is the skeleton silently accumulating
board- or system-name references over time as boards are added, which would
make it no longer shared in practice even though it's still one file.

### Clock configuration (roadmap Stage 3), adopted in bounded form

A system may declare a clock selection — just a target sysclk (`target_hz`);
`build.rs` validates it against the selected chip's `max_sysclk_hz` (a
chip-family fact, alongside `flash_kib`/`ram_kib`/`probe_chip`) and generates
the matching `stm32f4xx_hal::rcc::Config` expression, consumed by
`crate::clock::config()`. No clock selection means bare `Config::hsi()`,
identical to every build before this existed. Which oscillator to run from
isn't part of this selection at all — see the entry below.

Bounded deliberately: only the chip's maximum sysclk is checked. There is no
PLL-divider-feasibility modeling — an exactly-reachable target is still the
HAL's own concern at `freeze()` time, same as before this existed. This
resolves the former open item "how board clock requirements become a
validated RCC configuration," in this bounded form, not by deferring it.

### HSI-vs-HSE selection lives in the board, not the catalog

Earlier, `catalog::BoardConfig` carried an `hse: Option<HseSource>` field
(frequency + bypass mode), and `SystemConfig.clock` carried a `source:
ClockSource` (HSI or HSE) alongside `target_hz`, so a system explicitly chose
its oscillator and `build.rs` used the board's declared HSE fact to bake the
right `Config::hse(...)` expression.

Both are gone. This project's domain is flight controllers: a board that
provides HSE is always run from it, and there's no concrete case anywhere in
this codebase (or in real FC firmware generally) of a board with HSE
deliberately running on HSI instead — the independent `source` choice was
unexercised flexibility for a divergence that doesn't happen, the same
category of premature generality already rejected elsewhere (a shared
hardware struct with `Option` fields, generalizing `boards::active` with one
board to design against). Modeling it added a field to `SystemConfig` and a
field to `BoardConfig` that only ever took one value between them.

The fix: a board's own oscillator setup is hand-written Rust, not catalog
data. `src/boards/nucleo_f401re.rs` exposes `pub fn clock_config() ->
rcc::Config`, returning `Config::hse(8.MHz()).bypass_hse_oscillator()`
directly — the same file that already owns every other physical wiring
decision for this board. `build.rs`'s generated `crate::clock::config()`
calls it (`crate::boards::active::clock_config().sysclk(target_hz)`) whenever
the system has a board, and falls back to `Config::hsi().sysclk(target_hz)`
otherwise — deciding *whether* to call into the board is a build-time fact
`build.rs` already has (`system.board.is_some()`), not something requiring
its own catalog field.

This also fixes the layering the earlier design had backwards: whether HSE
hardware exists, at what frequency, and how it's driven is exactly the kind
of physical fact `src/boards/<name>.rs` already owns for every other
peripheral (see [Common hardware behavior stays out of board
declarations](#common-hardware-behavior-stays-out-of-board-declarations)) —
routing it through `catalog` and `build.rs` codegen was unnecessary
indirection for a fact with no build-time cross-check left to perform once
the independent `source` selection was removed.

### Board and firmware naming are separate facts

A board manifest's `name` identifies the physical board; a system's `name`
identifies the firmware built for it — one of each, never shared. A system
without a board manifest (the three default systems) has only the firmware
name, since there's no separate board fact to name.

### TOML deferred for chip/system/board data

Chip, system, and board data (`src/chips/*/config.toml`,
`src/systems/*/config.toml`, `src/systems/nucleo_f401re/board.toml`) is now
plain Rust in `catalog` instead of parsed TOML. Both consumers
(`build.rs` and the `rtic_app_cfg::for_chip` reducer) are themselves plain
Rust running at build time — never an end user, never an external tool — so a
serialization format bought nothing there but a hand-written validate-and-
convert layer (`required_string`, `validate_cfg_name`, `pin_parts`, ...) that
the compiler now does for free. It also removed a real bug class: the board
manifest used to be parsed twice (once by `build.rs`, once via `board-config`'s
`include_str!`-embedded copy for the proc-macro), which is why a "build and
reducer board views differ" self-check existed at all. One Rust definition,
used by both, makes that check structurally unnecessary instead of something
to verify.

This is a deferral, not a reversal of "every system gets a board manifest" or
of TOML as the eventual, human-editable form: several fields that were
strings only because TOML forced them to be (`pin.mode`, `pin.active`,
`dma.direction`, `clock.source`, `chip` itself) are now closed enums, so a
typo is a compile error instead of a build-time panic. Re-adding a
deserializer (almost certainly `serde`) is expected once that type design has
proven out — a smaller job done once, against settled types, than co-evolving
a schema and a type design at the same time. Until then, the DSL-permission
boundary in `AGENTS.md` applies to this Rust data exactly as it applied to the
TOML it replaced: declarative facts only, never executable behavior, and no
schema/scope change without explicit permission — regardless of which format
currently expresses it. `Embed.toml` is unaffected; it is `cargo-embed`'s own
external config format, not ours to redesign.

### Unified RTIC entry point

All normal configurations use one `#[init]` function. This is not currently a
claim of complete hardware initialization for every chip.

### SysTick monotonic

SysTick supplies common monotonic time without consuming an STM32 timer already
needed by dispatchers or the Nucleo report trigger. The Nucleo LED blink is an
async software task delayed by SysTick.

### Explicit approval for DSL work

No DSL or DSL-like mechanism may be designed, introduced, extended, or modified
without explicit user permission. This includes extending the reducer grammar or
making configuration schemas encode executable program behavior.

## Rejected or superseded approaches

- Inner `macro_rules!` task composition: RTIC sees the invocation before it is
  expanded.
- Raw conditional task variants inside one RTIC app: the required cfg patterns
  are not correctly handled by the pinned RTIC implementation.
- Build-script text composition for normal development: it loses the desired
  editable-source and IDE experience. It remains only as a regression fixture.
- Type-alias role dispatch as the adopted root design: it compiles without custom
  preprocessing but imposes conservative union resource/priority costs. A working
  version is preserved under `archive/option-a-type-alias/`.

## Open architecture items

1. Define real chip-level initialization responsibilities and return types for
   F401, F405, and F411.
2. Add family-level peripheral and conflict guardrails without turning them into
   an exact-MPN database or allowing configuration to become an unapproved
   programming DSL.
3. Review the bounded NUCLEO-F401RE board manifest prototype before authorizing
   any additional schema, system, pin-mode, or DMA-endpoint support.
4. Define the boot router and its compatibility/conflict validation while
   keeping its configuration format unspecified until explicitly approved.
5. Add a regression tripwire that detects when upstream RTIC no longer needs the
   reducer.
6. Reduce manual registration duplication across Cargo features, the system
   catalog, skeleton selectors, aliases, and Embed profiles.
7. Reassess safety/tooling evidence for the reducer before treating the framework
   as assurance evidence rather than a prototype.
8. Validate behavior on hardware; current evidence is primarily compilation,
   linking, disassembly experiments, and IDE checks — including this session's
   clock configuration, which has not been verified on real hardware.

## Historical rationale

Use the historical documents only when the reasoning behind these decisions is
needed:

- [Original architecture plan](../history/original-architecture-plan.md)
- [Compile-check spike report](../history/compile-check-spike-report.md)
- [Archived alternative](../../archive/option-a-type-alias/README.md)
