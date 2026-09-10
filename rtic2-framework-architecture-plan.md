# RTIC2-based Modular Firmware Framework — Architecture Plan

Context document for Claude Code. This captures the design reasoning and open items
from an architecture planning session, distilled into something actionable for a
compile-check spike and hardware iteration.

## Hard constraints

- **Language**: Rust.
- **Concurrency layer**: RTIC2, used unmodified — no forking or patching RTIC2 itself.
  Chosen for compile-time task/resource safety guarantees (Stack Resource Policy /
  priority ceiling protocol) and low overhead.
- **Toolchain**: Ferrocene (qualified Rust compiler). Assurance ceiling is whatever's
  realistically achievable with Ferrocene + RTIC2 together — not assumed proven,
  to be established as part of this work, not taken as a given.
- **Target hardware**: single-core Cortex-M (STM32F4/F7/H7-class), matching typical
  Betaflight-supported flight controllers. RTIC2 doesn't support multicore, so this
  is true by construction, not an extra restriction.
- No DO-178C-style documentation/process overhead front-loaded into the MVP. The
  architecture must not preclude retrofitting that case later.

## Two pillars

1. **Betaflight-scale board support** — plugin/module ecosystem for many peripherals
   on a fixed target set now. Multi-MCU/board portability is a future aspiration,
   not MVP scope.
2. **Safety-assurance-first** — firmware configurable per EASA class **C1–C6 at
   compile time** (feature-gated build, one image per class — class markings are a
   product-labeling axis, not a runtime-selected mode). The SAIL/SORA angle is
   *not* a firmware certification target for this framework — it's a design bias
   toward minimizing operator workload before a companion computer is required.

## MVP scope

- The RTIC2 composition layer itself — solving the "everything only compiles inside
  `#[app]`" problem — modular, reusable task/resource registration.
- A minimal driver set on the reference board (**Foxeer F405 V2**), built against
  `embedded-hal` traits only, for reuse across future boards. Drivers must stay
  RTIC-ignorant — no dependency on `rtic` itself — so they're unit-testable off
  target and portable.
- Peripheral protocol assignment (what runs on a given UART/SPI/I2C/timer) is
  configurable and takes effect on reboot, not hot-swappable at runtime.

## The core problem

RTIC2's `#[app]` macro performs whole-program static analysis (task/priority/resource
graph) to generate its lock-free critical sections. This analysis is not incremental
by design — it needs to see the entire task/resource graph in one macro invocation.
That's why everything only compiles inside `#[app]`, and why naive composition
(assembling an app from independently-declared modules) doesn't work.

**Dead end, confirmed:** `macro_rules!`-generated task blocks placed inside `#[app]`
don't work — attribute macros receive their input before inner `macro_rules!`
invocations expand, so RTIC2's parser sees an unexpanded macro call where it expects
a `#[task] fn` and rejects it.

## Strategy chosen: fixed skeleton + per-role task duplication

Two strategies were compared:

- **Strategy 1 (codegen)** — a generator assembles a complete `#[app]` module from
  declarative module descriptions. Rejected for the MVP: puts a code generator
  directly in the safety-relevant path, which raises a DO-330-style tool
  qualification question. Not ruled out for v2 if the module catalog outgrows a
  fixed skeleton — revisit with real field data at that point.
- **Strategy 2 (fixed skeleton + trait/cfg dispatch)** — **chosen**. One hand-written
  `#[app]` skeleton per MCU family. RTIC2 never sees anything but ordinary,
  hand-authored Rust — no new tooling in the safety-relevant build path.

### How the skeleton is structured

- **Hardware task set is fixed per MCU family**, enumerated from the datasheet (e.g.
  ~14 timer interrupt vectors on an F405), not per application. This set doesn't
  grow with feature count — it's a silicon property, written once per MCU family.
- **Task inclusion** (is this vector used by the selected board at all) is a
  compile-time `#[cfg]` gate — same mechanism as the existing UART port table:
  ```
  UART1 --> SBUS, RC input task
  UART2 --> MSP uart config, MSP task
  UART3 --> GPS uart config, GPS task
  UART4 --> unused, disabled
  ```
- **Role assignment** (which logical role a given physical peripheral plays on a
  board) is a per-board fact — a type alias or const in a per-board module — not a
  runtime flag and not a branch inside the task body:
  ```rust
  // boards/foxeer_f405v2.rs
  pub type Tim1Role = MotorPwm;
  pub type Tim2Role = RcCapture;
  pub type Tim5Role = SchedulerTick;
  // TIM8: no entry -> its hardware task isn't compiled in at all
  ```
- **Per-role resource requirements differ** (different `shared`/`spawn` needs per
  role), and this is where naive genericization breaks: RTIC2's macro parses
  `shared = [...]` and `priority = N` as literal syntax before type-checking, so a
  trait-dispatch call can't carry resource/spawn requirements across the boundary.
  **Resolution:** `cfg_attr` selects between complete, independent `#[task(...)]`
  attribute variants — RTIC2 only ever sees one fully-resolved attribute per build:
  ```rust
  #[cfg(feature = "role-motor-pwm")]
  #[task(binds = TIM2, shared = [esc_state], priority = 3)]
  fn tim2_handler(cx: tim2_handler::Context) {
      cx.shared.esc_state.lock(|s| { /* ... */ });
  }

  #[cfg(feature = "role-rc-capture")]
  #[task(binds = TIM2, shared = [failsafe_state], local = [capture_buf], priority = 4)]
  fn tim2_handler(cx: tim2_handler::Context) {
      cx.local.capture_buf.push_edge(/* ... */);
      if let Some(frame) = cx.local.capture_buf.try_decode() {
          cx.shared.failsafe_state.lock(|f| f.update(frame));
          rc_frame_ready::spawn(frame).ok();
      }
  }
  ```
  Board features are mutually exclusive, so exactly one variant survives per build.
  Full, real `cx.shared`/`spawn()` access — no abstraction layer to fight.
  Protocol-level variation *within* one role (e.g. SBUS vs. CRSF decode) is ordinary
  `#[cfg]` on statements inside the body — outside RTIC's macro concerns entirely.
- **Local-only resources** (state a role owns exclusively) can be genericized via a
  per-board associated type, since `local` fields don't need lock-ceiling analysis —
  this is the one place the "generic slot, concrete type per board" pattern is safe.
- **Duplication cost is proportional to the role catalog** (small, enumerable — motor
  PWM, RC capture, beeper, scheduler tick, etc.), not to board count. Written once by
  the skeleton author per role, not per board integrator.

### Explicitly ruled out

- `#[cfg]` *inside* a single `shared = [...]` list to conditionally include
  individual entries — not supported. RTIC2's resource-access list is a plain
  `Map<Access>`, no per-entry attribute slot exists in its AST.
- Runtime flags/branches to select a timer's role — wrong layer; role is a board
  wiring fact, fixed at build time. A runtime branch prevents dead-code elimination
  of unused roles and forces all possible roles' resource types to coexist in one
  task's signature.

## Known risks — verify before relying on this design further

1. **RTIC2 `#[cfg]`-on-task history**: a real bug existed where disabling hardware/
   software tasks via `#[cfg]` generated broken code (fixed in `rtic-macros` PR #894,
   Feb 2024). Confirm the pinned RTIC2 version behaves correctly — don't assume the
   docs describe current behavior without checking.
2. **Duplicate interrupt binding + cfg interaction**: unconfirmed whether RTIC2's
   "one task per interrupt" conflict check correctly accounts for mutually exclusive
   `#[cfg]` gates, or whether it sees both `cfg`-gated `tim2_handler` variants in the
   raw AST and flags a false duplicate-binding error regardless of feature selection.
3. **DMA channel/stream contention across roles**: not solved by the task-cfg pattern.
   Two roles on different timers can legitimately collide on the same physical DMA
   stream depending on the chip's request mapping. Needs a deliberate compile-time
   validity check (const assertion or build.rs check against the board's declared
   role table) rather than being discovered as a runtime hard fault.

## Review tooling (decided: convenience only, not evidence-grade)

- Goal: let a reviewer see a clean, cfg-resolved view of the skeleton for a given
  board/class, without RTIC-DSL noise, to confirm the firmware configuration is right.
- **Rejected**: a custom syn-based CLI that parses the skeleton and deletes cfg'd-out
  blocks to produce the file that's actually compiled. This reimplements compiler-
  level cfg semantics in an unqualified tool, sitting in the safety-relevant build
  path, redundant with what Ferrocene's toolchain already does correctly.
- **Decided**: keep the actual build on ordinary `cargo build --features X`. For
  review, use `cargo expand --features <board-features>` (or, live, rust-analyzer's
  cfg-aware greying-out) — this uses the real compiler frontend, not a
  reimplementation, so there's no equivalence argument to make. Bonus: shows RTIC's
  actual generated lock/dispatch code, not just cfg-filtered source.
  - Caveat: `cargo expand` typically needs an unstable rustc flag (commonly nightly,
    sometimes `RUSTC_BOOTSTRAP=1` on stable) — verify against whatever channel
    Ferrocene tracks before relying on it beyond local dev review.
- **Status**: kept as a developer convenience only, not a certification artifact, at
  least until there's a concrete need to formalize it as reviewed/recorded evidence.
  If ever promoted to evidence, its narrower job (pruning items by cfg attribute,
  not parsing RTIC's DSL) makes periodic spot-checking against real
  `cargo build --features X -v` output a reasonable ongoing check — not full
  qualification, but not "trust it forever" either.

## Immediate next step

**Before any hardware**: a minimal compile-check spike. Two `tim2_handler`-style
`cfg`-gated task variants (as above), cross-compiled for the real target
(`thumbv7em-none-eabihf` or equivalent) under each feature flag independently.
Confirms or refutes, cheaply:
- Clean compilation under each `role-*` feature.
- No RTIC2 cfg-related codegen bug (risk #1 above).
- No false duplicate-interrupt-binding error (risk #2 above).

**Then, hardware iteration on an ST Nucleo** (F4-series, close enough to the Foxeer
F405 V2's peripheral set that framework mechanics should transfer) — validating
framework mechanics only (does the role-dispatch pattern work on real hardware, does
priority preemption behave as expected), not flight behavior. Not the final target
board — that's the F405 V2 later.

Skip further architecture discussion beyond this point until the compile-check
result is in — the open questions left are compiler/silicon questions, not design
questions.

## Reference facts

- Reference/target board: **Foxeer F405 V2**.
- Hardware testbed for the current phase: **ST Nucleo** (F4-series).
- Example port table (UART, established pattern reused for timers):
  ```
  UART1 --> SBUS, RC input task
  UART2 --> MSP uart config, MSP task
  UART3 --> GPS uart config, GPS task
  UART4 --> unused, disabled
  ```
