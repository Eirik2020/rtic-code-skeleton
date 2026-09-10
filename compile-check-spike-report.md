# Compile-Check Spike — Report

Session report for the "Immediate next step" spike defined in
`rtic2-framework-architecture-plan.md`. Cross-compiled for the real target
(`thumbv7em-none-eabihf`) against real dependencies: `rtic` 2.3.1 (`rtic-macros`
2.1.0), `stm32f4xx-hal` 0.23.0, targeting the STM32F401RE (Nucleo-F401RE).

**Bottom line:** the plan's `#[cfg]`-selected-task-variant mechanism does not work
on current RTIC2. The plan's *other* sketch — per-board type aliases + associated
types — does work, compiles to correct per-board machine code, and costs 4 lines
per board. Its price is that `shared = [...]` and `priority = N` cannot vary per
role, which forces a union resource set on each vector. Whether that union is a
footnote or fatal depends on the shape of the real role catalog, which is the open
question this spike could not answer.

---

## 1. What `#[cfg]` can and cannot do inside `#[app]`

All rows tested this session against rtic 2.3.1.

| Pattern | Result |
|---|---|
| Single cfg-gated **hardware** task, unique binding | ✅ works, feature on and off |
| Single cfg-gated **software** task, unique name | ✅ works, feature on and off |
| Two cfg-gated tasks, same `binds = TIMx` | ❌ `this interrupt is already bound` |
| Two cfg-gated tasks, same fn name (hw or sw) | ❌ `this task is defined multiple times` |
| Two cfg-gated `#[init]` / `#[idle]` | ❌ `must appear at most once` |
| `#[cfg]` on a `#[shared]`/`#[local]` **struct field** | ❌ `E0658: attributes on expressions are experimental` |

The rule: **`#[cfg]` works for inclusion, breaks for variant selection.** Turning a
task on or off is fine. Choosing between two versions of the same thing is not.

This applies identically to software tasks — including `priority`, so cfg cannot be
used to vary a software task's priority per board either.

### Why — failure mode 1: the parser is `#[cfg]`-blind

`App::parse` (`rtic-macros-2.1.0/src/syntax/parse/app.rs`) walks every item in the
`#[app]` mod with no `#[cfg]` awareness at all:

```rust
let mut check_binding = |ident: &Ident| {
    if bindings.contains(ident) {
        return Err(parse::Error::new(ident.span(), "this interrupt is already bound"));
    }
    ...
};
```

`check_binding`, `check_ident`, and the `init.is_some()`/`idle.is_some()` checks all
fire unconditionally, regardless of active features — the macro receives the mod's
raw, un-cfg-stripped token stream. Same root cause as the plan's already-confirmed
`macro_rules!`-inside-`#[app]` dead end: an outer attribute macro sees its body
before any inner `#[cfg]` is resolved.

This confirms plan risk #2 as **real and currently unfixed**, not merely historical
(PR #894-era) behaviour.

### Why — failure mode 2: field cfgs get spliced onto an expression

Not anticipated by the plan, which only ruled out `#[cfg]` *inside* a
`shared = [...]` list. Traced to `codegen/shared_resources.rs`:

```rust
let ptr = quote!(
    #(#cfgs)*
    #mangled_name.get_mut() as *mut _
);
```

The field's cfgs are carried through codegen correctly as data, then re-emitted onto
an **expression**, not a declaration. Attributes on arbitrary expressions need the
unstable `stmt_expr_attributes` feature. So per-field cfg on resource structs is
unusable.

---

## 2. What works: one skeleton + per-board type-alias role table

The mechanism the plan itself sketched ("Role assignment" type aliases + "Local-only
resources ... genericized via a per-board associated type"). Type-level dispatch
never creates a *collision*, because the skeleton declares each vector exactly once —
what varies per board is a *type*, resolved long after rtic-macros has run.

```rust
// src/skeleton.rs — written ONCE, per MCU family
#[task(binds = TIM2, shared = [esc_state, failsafe_state], local = [tim2_state], priority = 3)]
fn tim2(cx: tim2::Context) {
    let frame = (cx.shared.esc_state, cx.shared.failsafe_state).lock(|esc, fs| {
        <Tim2Role as TimerRole>::on_irq(cx.local.tim2_state,
            RoleCtx { esc_state: esc, failsafe_state: fs })
    });
    if let Some(frame) = frame { rc_frame_ready::spawn(frame).ok(); }
}

// src/board/config_a.rs — the ENTIRE per-board cost
pub type Tim2Role = crate::roles::motor_pwm::MotorPwm;
pub type Tim3Role = crate::roles::rc_capture::RcCapture;
```

Per-board local state comes from the associated type, keeping `#[cfg]` away from the
resource structs (failure mode 2):

```rust
#[local]
struct Local {
    tim2_state: <Tim2Role as TimerRole>::State,
    tim3_state: <Tim3Role as TimerRole>::State,
}
```

**Verified by disassembly** — statically dispatched, no vtable, genuinely different
per board:

| build | TIM2 calls | TIM3 calls |
|---|---|---|
| `board-a` | `motor_pwm::MotorPwm::on_irq` | `rc_capture::RcCapture::on_irq` |
| `board-b` | `rc_capture::RcCapture::on_irq` | `motor_pwm::MotorPwm::on_irq` |

Line cost: `skeleton.rs` 78 lines written once; each board table **4 lines**. Adding
a board adds a role table, not an app.

---

## 3. The cost: forced union resource sets

RTIC parses `shared = [...]` and `priority = N` as literal syntax before typecheck,
so neither can vary per role. The skeleton's lock set must therefore be the **union**
of what every role assignable to that vector might need:

```rust
// TIM2 can be MotorPwm | RcCapture | Beeper | ...
#[task(binds = TIM2, shared = [esc_state, failsafe_state, beeper_state, ...], ...)]
```

RTIC computes each resource's ceiling from the *declared* list — it cannot know what
the body actually touches. So on a board where TIM2 is `RcCapture` and never reads
`beeper_state`, `beeper_state`'s ceiling still includes TIM2's priority, and every
other task locking it masks interrupts higher and longer than necessary.

Two things make this worse than it first appears:

- **It compounds with catalog size.** If any role can land on any vector, each
  vector's union converges on "every shared resource in the system" — effectively a
  global lock, and SRP stops buying anything.
- **It weakens the audit story.** The resource list is meant to be the reviewable
  statement of what an ISR can touch. A union describes no particular build — a poor
  position for the DO-178C-style argument the plan wants to keep open.

### Does "shrink the ISR, spawn a software task" solve it? Only partly.

It **relocates** the union rather than eliminating it. Whatever software task owns
the state needs the union of what its possible action-handlers touch. You cannot
dodge that with one software task per role, because selecting between them per board
is exactly the broken case in §1. It would need `type Action` as an associated type
and one software task per *vector* — which carries the union again.

It is still worth doing, for one specific reason:

> **Ceiling inflation cost scales with the priority of the over-declaring task.**

Moving the union from a priority-4 ISR to a priority-1 dispatcher means the inflated
ceiling is 1, not 4. Everyone else locking those resources masks far less. Same
defect, an order of magnitude cheaper. Secondary benefit: ISRs holding no shared
resources need no critical section, so ISR duration drops and worst-case latency
improves system-wide.

New costs, one of which bites this domain specifically:

- **`spawn()` fails when the queue is full.** The skeleton currently does `.ok()`,
  silently dropping. For RC frames / failsafe that is a correctness decision, not a
  style one.
- **Latency on the failsafe path.** Deferring state updates to low priority is
  backwards for the one resource that matters most — you may *want* failsafe
  evaluated in the ISR precisely because it is safety-critical. The mitigation is in
  direct tension with the safety argument.

### Likely shape of the answer

Not all-or-nothing: keep the small hot set (`esc_state`, `failsafe_state`) in the
ISR, where the union is small and justifiable, and push the long tail — beeper, GPS,
telemetry, config — to a low-priority dispatcher where inflated ceilings are cheap.
The union only becomes pathological when one vector can host many unrelated roles.

**This is the open question the spike cannot close:** how many roles realistically
compete for the same timer vector, and is the hot path really just failsafe/ESC?
That determines whether the union is a footnote or the thing that kills this
approach.

### Worth naming plainly

This union cost is the price of refusing codegen. The plan's Strategy 1 solves it
exactly — a generator emits the precise `shared = [...]` per board. That tradeoff is
now concrete rather than theoretical, and should be re-weighed against the DO-330
tool-qualification concern with this cost in hand.

---

## 4. Superseded: file-level whole-app selection

An intermediate step, kept only as a reasoning trail — **do not use.**

`#[cfg_attr(feature = "...", path = "...")]` selecting a whole per-board `#[app]`
file does compile, and does sidestep both failure modes (rtic-macros only ever sees
one board's tokens). It was validated fairly hard, including disassembly confirming
per-board vector binding and real `basepri::read` → `basepri_max::write` → body →
`basepri::write` priority-ceiling sections.

But it is not a framework: the two board files were ~55 identical lines differing in
two tokens (`TIM2`/`TIM3`). It inverts the plan's stated goal — duplication
proportional to the **role catalog**, not to board count — into one hand-maintained
copy of the entire app per board. At Betaflight scale that is untenable. Superseded
by §2, which achieves the same result at 4 lines per board.

---

## 5. Open questions and next steps

- **Role catalog shape** (§3) — how many roles compete per vector; is the hot shared
  set really just failsafe/ESC. Determines whether the union cost is acceptable.
- **EASA class (C1–C6) gating** — untested. If a class needs to add/remove
  `Shared`/`Local` fields, it hits failure mode 2. Would need the same associated-type
  treatment, or per-(board × class) selection.
- **Plan risk #3, DMA channel/stream contention across roles** — unaffected by any of
  the above, still needs its own compile-time validity check (const assertion or
  build.rs against the board's role table).
- **Version specificity** — all of §1 is specific to rtic 2.3.1 / rtic-macros 2.1.0.
  The `#[cfg]`-blindness and the field-cfg splice could plausibly be fixed upstream;
  re-verify on any upgrade before assuming these constraints still hold.
- **Hardware iteration on the Nucleo** — not started. This spike validated
  compile-time and static-binary behaviour only, per the plan's own sequencing.

---

## 6. Repo state

```
c:\ws\rtic-code-skeleton\
├── rtic2-framework-architecture-plan.md   (unchanged)
├── compile-check-spike-report.md          (this file)
├── Cargo.toml / Cargo.lock                (features: board-a, board-b)
├── .cargo/config.toml                     (target = thumbv7em-none-eabihf)
├── build.rs / memory.x                    (STM32F401RE: 512K flash / 96K RAM)
└── src/
    ├── main.rs                            (module wiring only)
    ├── skeleton.rs                        (THE #[app] — written once, 78 lines)
    ├── roles/                             (RTIC-ignorant role catalog)
    │   ├── mod.rs                         (TimerRole, RoleState, RoleCtx traits)
    │   ├── common.rs                      (CaptureBuffer, RcFrame)
    │   ├── motor_pwm.rs                   (MotorPwm: TimerRole)
    │   └── rc_capture.rs                  (RcCapture: TimerRole)
    └── board/
        ├── mod.rs                         (cfg_attr path selection)
        ├── config_a.rs                    (4 lines: TIM2=motor, TIM3=rc-capture)
        └── config_b.rs                    (4 lines: TIM2=rc-capture, TIM3=motor)
```

Both boards build clean and warning-free:

```
cargo build --no-default-features --features board-a
cargo build --no-default-features --features board-b
```
