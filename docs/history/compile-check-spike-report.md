# Compile-Check Spike — Report

> Historical experimental record. For the adopted implementation, start with
> the [current architecture overview](../architecture/overview.md).
>
> Update: the editable-source limitation described below has a working solution
> using a span-preserving attribute macro. See [IDE workflow and
> verification](../development/ide-workflow.md).
> The current lockfile resolves `rtic-macros` 2.3.1; the 2.1.0 findings below are
> historical. Text composition is now retained only for the regression fixtures.

Session report for the "Immediate next step" spike defined in
the [original architecture plan](original-architecture-plan.md). Cross-compiled for the real target
(`thumbv7em-none-eabihf`) against real dependencies: `rtic` 2.3.1 (`rtic-macros`
2.1.0), `stm32f4xx-hal` 0.23.0, targeting STM32F401/F405/F411.

**Bottom line:** the plan's `#[cfg]`-selected-task-variant mechanism does not work
on current RTIC2 — not just for board-to-board role reassignment (plan risk #2,
confirmed real) but, more seriously, for genuine cross-chip-family variation,
where a `#[cfg]`-gated-off task's interrupt binding doesn't even exist in the
target chip's PAC. Two working alternatives now exist, independently verified,
with different costs:

1. **Type-alias skeleton + chip-level file selection** (now archived at
   [`archive/option-a-type-alias/`](../../archive/option-a-type-alias/) — see §6) — no extra tooling, but forces a union
   resource set per vector, and needs one skeleton file per exact chip (cheap:
   differences are usually a handful of lines).
2. **`syn`-based cfg-reducer composition** (now adopted at the repo root — see
   §6; originally the `rtic2-multi-head-test/` fixture, independently re-verified
   this session, not just trusted) — avoids the union cost entirely and gives
   genuine rust-analyzer/build parity, at the cost of putting a code-transformation
   tool in the safety-relevant build path, which is exactly what the plan's
   "Review tooling" section pre-rejected for a lesser use.

Both are real, working, measured. Which to adopt is a design decision, addressed
at the end.

**Decision (post-session): option B (`syn`-based cfg-reducer composition) adopted.**
Explicitly framed as conditional, not permanent — the intent is to discard it if a
future `rtic`/`rtic-macros` release fixes the underlying cfg-blindness (§1's failure
modes 1 and 3), reverting to plain item-level `#[cfg]` with no extra tooling. For
that to actually happen rather than quietly never happen, this needs: (a) the
reducer's necessity documented against the *exact* pinned `rtic-macros` behaviors
it works around, and (b) a live regression check — the original failing patterns
(cross-chip `USART3`-style binding, same-binding collision) run directly against
whatever `rtic-macros` version is current, expected to fail today; the day it
passes is the unambiguous signal to migrate off. Not yet built — tracked as a
follow-up, not done this session.

---

## 1. What `#[cfg]` can and cannot do inside `#[app]`

| Pattern | Result |
|---|---|
| Single cfg-gated **hardware** task, unique binding | ✅ works, feature on and off |
| Single cfg-gated **software** task, unique name | ✅ works, feature on and off |
| Statement-level `#[cfg]` inside an ordinary task body | ✅ works — RTIC doesn't reprocess task bodies the way it reprocesses `#[init]` |
| Statement-level `#[cfg]` inside `#[init]`'s body | ❌ `E0658: attributes on expressions are experimental` (RTIC reprocesses `#[init]` for local-resource declarations) |
| Two cfg-gated tasks, same `binds = TIMx` | ❌ `this interrupt is already bound` |
| Two cfg-gated tasks, same fn name (hw or sw) | ❌ `this task is defined multiple times` |
| Two cfg-gated `#[init]` / `#[idle]` | ❌ `must appear at most once` |
| `#[cfg]` on a `#[shared]` field that **survives** (cfg true) | ❌ `E0658: attributes on expressions are experimental` |
| `#[cfg]` on a `#[shared]` field that's **absent** (cfg false) | ✅ works |
| `#[cfg]` on a `#[local]` field, either direction | ✅ works — see below for why |
| Cfg-gated task whose `binds` doesn't exist in the active chip's PAC **at all**, even correctly excluded | ❌ `E0599: no such associated item` — see §2 |

The rule: **`#[cfg]` works for inclusion, breaks for variant selection** — with one
further, more damaging exception found this session (§2).

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
raw, un-cfg-stripped token stream. Confirms plan risk #2 as **real and currently
unfixed**, not merely historical (PR #894-era) behaviour.

### Why — failure mode 2: field cfgs get spliced onto an expression — `#[shared]` only, and only when the field survives

Not anticipated by the plan, which only ruled out `#[cfg]` *inside* a
`shared = [...]` list. Traced to `codegen/shared_resources.rs`:

```rust
let ptr = quote!( #(#cfgs)* #mangled_name.get_mut() as *mut _ );
```

The field's cfgs are carried through codegen correctly as data, then re-emitted onto
an **expression** — the pointer-cast inside the resource's `Mutex` proxy
implementation — not a declaration. That needs the unstable `stmt_expr_attributes`
feature, so it fails. This only happens when the field's cfg evaluates **true**
(the field survives and this codegen path runs for it); with the field genuinely
absent, nothing here ever executes, and it compiles clean. Verified both directions
in isolation: same minimal repro, only the active feature differs.

**Practically this makes the pattern unusable** for any field that needs to exist
on *some* board, which is the only case anyone would reach for it — the one build
where the field is present is exactly the one that fails.

`#[local]` is unaffected in **either** direction — verified the same way. `#[local]`
resources are exclusively owned, so RTIC never generates a `Mutex` proxy for them at
all, and never goes near the expression this bug lives in. This is why the
skeleton's associated-type trick (`tim2_state: <Tim2Role as TimerRole>::State`) is
strictly necessary for *type* variation, but the underlying *field* itself could
have been safely `#[cfg]`-gated on its own for `#[local]` state — it's only the
`#[shared]` fields (`esc_state`, `failsafe_state`) where field-cfg was never a
viable option at all, which is why they're unconditional across every board.

Worth connecting to §3: this means the union cost isn't only "the `shared = [...]`
list is literal syntax" — it starts one layer earlier. The `#[shared]` struct's
field list itself can't shrink per board either, so every board pays for every
`#[shared]` field any role might ever need, in RAM as well as in the lock-list
union. Small on its own (a few bytes per unused field), but the same root cause,
compounding the same direction.

---

## 2. The sharper finding: cross-chip-family variation is broken, not just board variation

F401 only exposes `USART1`, `USART2`, `USART6` as interrupt vectors — `USART3`,
`UART4`, `UART5` don't exist in its PAC at all (they're only present on larger F4
parts). Testing a `#[cfg]`-gated task bound to `USART3`, correctly excluded (feature
inactive) while building for F401:

```
error[E0599]: no variant or associated item named `USART3` found for enum `Interrupt`
```

**Even properly gated off.** Traced to `codegen/bindings/cortex.rs:217-253`,
`pre_init_enable_interrupts`: RTIC's `#[init]`-time NVIC setup iterates
`app.hardware_tasks.values()` and emits, per task, **unconditionally** (no per-item
cfg attached to these specific statements):

```rust
core.NVIC.set_priority(#rt_err::#interrupt::#name, ...);
rtic::export::NVIC::unmask(#rt_err::#interrupt::#name);
```

Unlike the task's own handler function (which correctly disappears — `task.cfgs` *is*
threaded through there), this separate codegen pass isn't cfg-aware, so
`Interrupt::USART3` must resolve even in a build where the USART3 task is gone. This
makes item-level `#[cfg]` **unusable for the plan's stated cross-chip-family goal**,
not merely inefficient.

### This also breaks rust-analyzer identically — verified directly, not inferred

Ran `rust-analyzer diagnostics` (the same binary VSCode uses) headlessly against the
project with the broken pattern active: independently reported the same
`E0599`/"no such associated item" class of error, at the same location, in both the
"cfg actually true" and "cfg correctly false but binding doesn't exist" cases. This
is not a coincidence — RA invokes the real, compiled `rtic-macros` proc-macro (same
dylib `cargo build` uses) and runs genuine resolution on its output. There is no
"IDE looks clean, build fails" split available here: RA and cargo see the same
macro-generated tokens and fail together.

---

## 3. What works, option A: type-alias skeleton + chip-level file selection

The mechanism the plan itself sketched ("Role assignment" type aliases +
"Local-only resources ... genericized via a per-board associated type"), extended
this session with a second, orthogonal selection axis for genuinely different chips.

**Board-level role reassignment** (same chip, different physical wiring) — one
skeleton, per-board type aliases:

```rust
// src/chips/f401_skeleton.rs — written ONCE per chip
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

**Verified by disassembly** — statically dispatched, no vtable:

| build | TIM2 calls | TIM3 calls |
|---|---|---|
| `board-a` | `motor_pwm::MotorPwm::on_irq` | `rc_capture::RcCapture::on_irq` |
| `board-b` | `rc_capture::RcCapture::on_irq` | `motor_pwm::MotorPwm::on_irq` |

**Cross-chip-family variation** (§2's actual failure) — since item-level `#[cfg]`
cannot gate a binding that doesn't exist in the target PAC, the fix has to happen
above where rtic-macros ever sees the tokens: file-level chip selection, same
mechanism as board selection, one level up:

```rust
// src/main.rs
#[cfg_attr(feature = "chip-f401", path = "chips/f401_skeleton.rs")]
#[cfg_attr(feature = "chip-f405", path = "chips/f405_skeleton.rs")]
mod skeleton;
```

`f405_skeleton.rs` adds one extra hardware task bound to `UART4` (genuinely absent
from F401's PAC) alongside the same `TIM2`/`TIM3` role dispatch. **Built and verified
all three real combinations** (`chip-f401,board-a`; `chip-f401,board-b`;
`chip-f405,board-a`) — all clean, and disassembly confirms `UART4` dispatches to
`roles::beeper::Beeper::on_irq`, the chip-exclusive role, correctly.

So the type-alias approach's cross-chip gap **is fixable without adopting extra
tooling** — the cost is one skeleton file per exact chip (not per board), each
differing only in the chip-specific vectors. For F401 vs F405 that's one extra
~25-line task block, not a rewrite.

### The union-ceiling cost, now quantified with real disassembly, not just argued

`shared = [...]` and `priority = N` are literal syntax parsed before typecheck, so
neither can vary per role. The F405 skeleton's `uart4` task demonstrates this for
free: `Beeper::on_irq` never reads or writes `esc_state` or `failsafe_state` at all
— yet the task still declares `shared = [esc_state, failsafe_state]`, because the
skeleton can't know that in advance. Disassembly of the compiled `uart4` task shows
the **full two-resource priority-ceiling lock sequence executes anyway**:

```
basepri::read → cortex_logical2hw → basepri_max::write
  → (nested lock for the second resource, same pattern)
  → call Beeper::on_irq (which touches neither field)
  → basepri::write (restore, twice)
```

This is real BASEPRI-based critical-section overhead — interrupts genuinely masked,
for a duration and priority level unrelated to what actually happens inside — paid
on every `UART4` interrupt, for no reason other than the skeleton having to declare
the union. Not a hypothetical: measured directly in the compiled binary.

**The more serious consequence is not the wasted cycles on `UART4` itself — it's
ceiling propagation to unrelated tasks.** Bumped `uart4`'s priority from 3 to 5
(`Beeper::on_irq` still touches neither field) and rebuilt: `esc_state`'s masking
ceiling, used by `tim2`'s and `tim3`'s lock code — tasks with no relationship to
`uart4` or `Beeper` at all — moved from `3` to `5` in lock-step, confirmed by
diffing the literal ceiling constant in the disassembly before and after. This is
the real cost: any task at priority 4 that could not previously be blocked by
`esc_state`'s lock (ceiling 3 < 4) now can be (ceiling 5 ≥ 4), for a reason with no
operational cause on a board that never even uses `Beeper`. Worst-case blocking
bounds for tasks unrelated to the role in question get worse — conservative, not
unsafe, but a real loss of exactly the schedulability-analysis tightness the plan
chose RTIC2 to get.

It also doesn't go away once a board is picked: the `shared = [...]` list is text
fixed in the *skeleton*, shared by every board using it — board selection changes
which role runs, not what the skeleton declares. A board that only ever assigns
`MotorPwm` to `TIM2` still pays every other anticipated role's ceiling contribution
on that vector, because the skeleton was written once to cover every role ever
assignable there across every board.

**Still open**: how many roles realistically compete for the same vector, and is the
hot path really just failsafe/ESC. That determines whether this union cost is a
footnote or the thing that rules the approach out for a large role catalog.

### Mitigation tested conceptually, not adopted: shrink the ISR, spawn a software task

Relocates the union rather than eliminating it — whatever software task owns the
state needs the union of what its action-handlers touch, and selecting between
per-role software tasks per board hits the same broken case as §1. Worth doing
anyway because **ceiling inflation cost scales with the priority of the
over-declaring task** — moving the union from a priority-4 ISR to a priority-1
dispatcher shrinks the same defect roughly an order of magnitude, at the cost of
`spawn()`-queue-full drops and added latency on whatever's deferred (a bad tradeoff
specifically for a failsafe path, which argues for evaluating failsafe in the ISR
and deferring everything else).

---

## 4. What works, option B: `syn`-based cfg-reducer composition (now the repo root — see §6)

A separate, self-contained fixture (not built by this report's author in this
session — found already present, independently audited and re-tested here rather
than taken on trust). `build.rs` parses the shared app body with `syn`, deletes
every item and struct field whose `#[cfg]` evaluates false for the active chip
feature, splices the result into the selected chip's head
(`#[rtic::app] mod app { /* APP_BODY */ }`), and `main.rs` pulls the generated file
in via `include!(concat!(env!("OUT_DIR"), ...))`. `rtic::app` never sees a `#[cfg]`
it needs to understand.

### Independently re-verified, not just trusted from its own report

- `cargo check --features f405` / `--features f411`: both clean, matches its
  self-reported results.
- The `f405_shared` field is **genuinely absent** from emitted tokens for non-f405
  builds (confirmed by reading the actual generated file) — avoids the field-cfg
  E0658 bug by construction.
- The `USART6`-binding collision between an F405 task and an F411 control task
  (exactly plan risk #2's shape) reduces correctly — only one survives per build.

### rust-analyzer/build parity is real here, and structural, not incidental

Set a default feature so RA would analyze a concrete configuration, then ran
`rust-analyzer diagnostics` headlessly:

- `f405` active, nothing broken → **clean**, matching `cargo check` exactly.
- `f401` + a deliberately invalid `UART8` binding active → RA independently reports
  the identical `E0599`, same location, as `cargo build`.

This *is* the split hoped for — but for a specific, mechanical reason: RA's
build-script integration actually executes `build.rs`, resolves `OUT_DIR`, and
follows the `include!` into the generated file. RA and cargo are analyzing the
same already-reduced input, because the reduction step is shared infrastructure
between them, not because RA is somehow more lenient about cfg. This differs from
§2 and §3's type-alias skeleton, where RA analyzes the annotated source directly and
hits the live macro bug exactly like cargo does.

### Bonus this design gets that option A doesn't — verified with the same test, not just asserted

The union-ceiling cost from §3 **doesn't apply here**, confirmed by mirroring the
exact ceiling-propagation test from §3 against this fixture: added a priority-6
task touching `counter`, gated `f405`-only, alongside the existing priority-2
`usart1` task that also locks `counter`.

| build | high-priority task | `usart1`'s ceiling for `counter` |
|---|---|---|
| `f411` | genuinely absent (deleted by the reducer before `rtic-macros` runs) | **2** — `usart1`'s own priority, unchanged |
| `f405` | genuinely present | **6** — correctly reflects real contention |

Confirmed directly in the disassembled ceiling constant both ways, not inferred.
The mechanical reason: by the time `rtic-macros` computes ceilings, the reducer has
already deleted the task, its resource declaration, and everything about it for a
build where it doesn't apply — there is no shared textual skeleton across chips at
all for a union to form from. Each chip's declaration *is* that build's reality, not
a conservative superset of it. This is the same methodology used to measure the
cost in §3, applied here to confirm its absence.

### New findings this session — two real bugs in the reducer itself

1. **Silent hyphen bug.** `CfgReducer::feature_enabled` does
   `format!("CARGO_FEATURE_{}", name.to_uppercase())` — but Cargo's actual env var
   convention replaces `-` with `_`. A feature named `class-hi` produces
   `CARGO_FEATURE_CLASS-HI` (hyphen preserved) from this code, while Cargo sets
   `CARGO_FEATURE_CLASS_HI` (underscore). Permanent mismatch, confirmed directly:
   built the same skeleton and field under `class-hi` (never detected, field/task
   silently absent even when the feature was active) versus `classhi` (detected
   correctly, field/task present). **This fails with no error message at all** —
   code that should be included is silently excluded. Any project using hyphenated
   feature names (a common convention) hits this invisibly.
2. **Item-level cfg stripping is incomplete, currently harmless.** For struct
   fields, the reducer both filters disabled fields *and* strips the `#[cfg]`
   attribute from ones that survive (`clean_field`). For top-level items
   (`file.items.retain`), it only *deletes* disabled items — it does not strip the
   `#[cfg]` attribute from items that are kept, so a retained (now-always-true)
   `#[cfg(feature = "f405")]` still reaches the generated file the macro consumes.
   Confirmed by reading the actual generated output. Currently harmless because
   rtic-macros' known bugs key on *duplicate* items (already resolved by deletion)
   or *field*-level cfg (already fully cleaned) — but the accurate model is "no
   duplicate bindings and no field-level cfg reach the macro," not "zero cfg reaches
   the macro." Worth flagging in case some other cfg-blind rtic-macros quirk keys off
   item-level cfg presence rather than duplication.
3. **"Multiple heads rejected" is incidental, not enforced by this tool.** Confirmed
   by reading the code: it passes only because `stm32f4xx-hal`'s own build script
   panics on multiple simultaneous chip features — a downstream dependency's guard,
   not a check `build.rs` performs itself. `heads` is built by mapping *every* active
   chip feature to its head file and concatenating them; with a different HAL that
   doesn't panic, this would silently emit two `mod app {}` blocks into one file.
   Cheap fix if adopted: an explicit `assert!(chips.len() == 1)` in `build.rs`,
   independent of what any dependency happens to do.
4. **Failure mode 1 (cfg-blind duplicate-binding check) reappears in the generated
   body for any axis without an external exclusivity guard — confirmed directly,
   generalizing finding 3.** Added two ordinary, unrelated features (`modea`,
   `modeb`, nothing chip-related) each gating a task bound to `EXTI2`. Built with
   both active: `error: this interrupt is already bound`, at the *generated* file,
   identical to the raw-skeleton failure. Not a resurfaced bug — both `#[cfg]`s
   genuinely evaluate true, so this is a real collision the reducer was never
   asked to prevent. The precise takeaway: **the reducer provides no protection of
   its own against colliding predicates that aren't kept mutually exclusive.** It
   works for chip selection only because `stm32f4xx-hal`'s build script happens to
   panic on multiple chip features (finding 3) — any *new* axis added later gets
   no such protection automatically, and the same failure resurfaces with no
   indication of which features caused it.

   **Noted for later, not implemented this session** (explicitly deferred by
   request — the fix is cheap and generally worth doing regardless of the
   option A/B decision, but changes the fixture rather than just probing it):
   - A collision check *is* feasible cheaply in `build.rs`: it already computes,
     per item, whether `#[cfg]` evaluates true for the current build (that's what
     `CfgReducer::enabled()` already does to decide what to delete). Group the
     *enabled* items by `binds = X`; if any group has more than one member,
     `panic!()` naming both items and their owning features. Turns a confusing
     `rtic-macros` error pointing at generated code in `OUT_DIR` into a clear,
     build-time message naming the actual conflicting features. Should
     probably be added regardless of the bigger design decision.
   - This only catches a bad combination *when someone builds it* — it doesn't
     prove no combination could ever collide (that's boolean satisfiability over
     arbitrary `cfg` predicates in general). In practice that gap is closed by CI
     building every combination that will actually ship (the fixture's 20-case
     matrix already does this informally), not by build.rs alone.
   - An invalid-interrupt check (does `binds = USART3` exist on the selected chip
     at all) is also feasible: either hand-maintain a small per-chip valid-vector
     list (which the plan already needs anyway — "enumerated from the datasheet,
     written once per MCU family") or derive it by parsing the PAC crate's actual
     generated `Interrupt` enum source with `syn`, more robust against staleness,
     more implementation work. Replaces `E0599` pointing into rustc's view of
     generated code with a plain "USART3 doesn't exist on STM32F401."

### Not yet exercised by the fixture's own test matrix

Statement-level `#[cfg]` inside a task body — the reducer's `CfgReducer` only visits
`ItemStruct` and `ExprStruct`, so a `#[cfg]`-gated `let` inside a function body
passes through untouched. Tested separately this session (against the type-alias
skeleton, §1): works fine in ordinary task bodies (RTIC doesn't reprocess them), but
breaks inside `#[init]` specifically (RTIC does reprocess `#[init]` for local-resource
declarations). Not yet tested through *this* reducer's pipeline — worth checking
before relying on it, since the reducer leaving such an attribute untouched inside
`#[init]` would hit the same `E0658` regardless of chip selection being otherwise
correct.

---

## 5. Open questions requiring a decision, not more testing

- ~~**Type-alias-per-chip (§3) vs. syn-reducer composition (§4), as the core
  strategy.**~~ **Decided: §4 (syn-reducer) adopted**, explicitly as a conditional
  choice — see the decision note after the bottom line at the top of this report.
  §3 remains fully working and documented in case a future `rtic`/`rtic-macros`
  release removes §4's reason for existing and prompts reverting.
- **Is the union-ceiling cost (§3) acceptable?** Moot while §4 is adopted (§4 has no
  union cost, verified in §4). Becomes relevant again only if reverting to §3.
- ~~**If §4 is chosen, what's the qualification/audit story for real?**~~
  **Decided: dev-convenience only, for now.** The generated `OUT_DIR` file is not
  archived or reviewed as an evidence-grade artifact — the skeleton is the reviewed
  thing. Revisit if/when the project moves toward the DO-178C-style retrofit the
  plan explicitly wants to keep open.
  **Ergonomics follow-up, post-hardware-testing**: editing `app_body_skeleton.rs`
  gives zero live rust-analyzer diagnostics in that file — it's never `mod`-included
  or `include!`d directly, so RA (correctly) treats it as outside the compiled
  crate and greys it out. Real diagnostics land on the build-script-generated file
  instead, which RA *does* track correctly (§4 already established this), but it
  lived at a hashed `OUT_DIR` path and was a single unformatted line. Mitigated,
  not eliminated: `build.rs` now pretty-prints via `prettyplease` and writes a
  second, stable copy to `target/generated-preview/generated_app_<chip>.rs` on
  every build, plus `cargo check-f401`/`405`/`411` aliases to trigger a refresh
  quickly. The fundamental limitation — no inline squiggles in the file you
  actually edit — is structural to "compose before macro expansion" and wasn't
  fixable, only made less painful.
- **EASA class (C1–C6) gating.** **Decided: deferred until a functional prototype
  exists**, same rationale as DMA contention below — what a real class actually
  needs to change (extra resources? different priorities? whole roles present or
  absent?) isn't known yet, so a synthetic test now would validate a guessed shape,
  not the real one. What's already confirmed and doesn't need repeating: a simple
  orthogonal class-like axis (independent of chip selection) composes correctly
  through the reducer once past the hyphen bug (§4, finding 1). What's still
  unknown either way: class-driven `Shared`/`Local` field variation combined with
  per-class priority/resource-list differences — surface this from the prototype's
  actual requirements, not from a guess.
- **Plan risk #3, DMA channel/stream contention across roles** — orthogonal to all
  of the above. **Decided: deferred until prototype implementation begins**, to be
  addressed as it comes up rather than designed for up front. Likely shape when it
  does: a third `build.rs` check alongside the deferred collision and
  invalid-interrupt checks (§5, Deferred work), since all three are the same kind
  of compile-time board-validity check and `build.rs` is already becoming their
  natural home.
- **Version specificity** — all of §1–§2 is specific to rtic 2.3.1 / rtic-macros
  2.1.0. Re-verify on any upgrade before assuming these constraints still hold.
- **Hardware iteration on the Nucleo** — not started. Everything here is
  compile-time and static-binary verification only, per the plan's own sequencing.

### Deferred work — explicitly held off by request, noted so it isn't lost

Not blocking, not forgotten — parked until actively working on option B for real
(rather than still spiking) or until asked for directly:

1. **Tripwire regression check for "can we discard the reducer yet."** A small
   check that runs the original failing patterns (cross-chip `USART3`-style
   binding, same-binding collision) directly against whatever `rtic-macros`
   version is pinned, expected to fail today. The day it passes instead is the
   unambiguous signal that upstream fixed the cfg-blindness and §3 (no extra
   tooling) can be reconsidered. Without this, "discard it if fixed upstream"
   relies on someone remembering to manually re-check — this makes it a runnable
   fact instead.
2. **`build.rs` collision check.** Reuse the reducer's own already-computed
   per-item enabled/disabled status; group enabled items by `binds`; `panic!()`
   naming both conflicting items and their owning features if any group has more
   than one member. Turns `error: this interrupt is already bound` (pointing into
   generated code in `OUT_DIR`, no indication of cause) into a message that
   actually says what's wrong. Confirmed necessary this session — the `modea`/
   `modeb` counter-example shows the reducer has zero protection against this on
   its own for any axis other than chip selection (§4, finding 4).
3. **`build.rs` invalid-interrupt check.** Validate every `binds = X` against a
   real per-chip vector list before composing — either hand-maintained (the plan
   needs this table anyway) or derived from the PAC crate's actual `Interrupt`
   enum via `syn`. Replaces `E0599` pointing into rustc's view of generated code
   with a plain "X doesn't exist on this chip."

Neither 2 nor 3 changes the A/B tradeoff — both only make option B's existing
failure modes loud and clear instead of confusing, if/when implemented.

---

## 6. Repo state (post-cleanup: option B promoted to root, option A archived)

Option B is now the actual project, not a side fixture — promoted from
`rtic2-multi-head-test/` to the repo root, with real per-chip linker support
added (it previously only supported `cargo check`; `cargo build` now produces a
real linked ARM ELF for all three chips, verified by header bytes). Option A was
moved to `archive/`, not deleted, per the decision's explicit conditionality.

```
c:\ws\rtic-code-skeleton\
├── README.md                              (updated for the adopted architecture)
├── rtic2-framework-architecture-plan.md   (unchanged — the north-star doc)
├── compile-check-spike-report.md          (this file)
├── .gitignore                             (NEW — target/ was previously tracked, ~1750 files)
├── Cargo.toml / Cargo.lock                (package: rtic-code-skeleton; features: f401, f405, f411)
├── .cargo/config.toml                     (NEW — target = thumbv7em-none-eabihf, -Tlink.x)
├── build.rs                               (the CfgReducer + composition; NEW: per-chip memory.x
│                                            generation into OUT_DIR, real flash/RAM sizes per
│                                            §4's chip config.toml parts — F401CC 256K/64K,
│                                            F405RG 1024K/192K, F411CE 512K/128K)
├── Embed.toml                             (cargo-embed profiles per chip)
├── src/                                   (THE adopted architecture)
│   ├── main.rs                            (includes the build-script-generated app)
│   ├── app_body_skeleton.rs               (shared RTIC app body — edit this)
│   ├── invalid_interrupt.rs               (negative-test fixture)
│   └── chips/{f401,f405,f411}/            (app_head.rs, config.toml; f405/ also has
│                                            app_body.rs, the "generated-body" stand-in)
├── docs/                                  (moved from rtic2-multi-head-test/docs/ + root)
│   ├── README.md, composition-test-summary.md, test-report.md
│   └── multi-head-app-composition-test-plan.md, multi-head-syn-reduction-test-plan.md
│                                            (moved from repo root, renamed for clarity)
├── tools/run-test.ps1                     (the 20-case test matrix — reconfirmed passing
│                                            from the new root location, and again after
│                                            adding the linker/dependency changes above)
└── archive/option-a-type-alias/           (working alternative, not adopted — own README
                                             explaining what it is and when to reconsider it;
                                             has its own Cargo.toml/.cargo/build.rs/memory.x,
                                             not wired into the root workspace)
    ├── README.md
    └── src/{main.rs, chips/, roles/, board/}
```

Verified after the move: all three chips still `cargo check` clean, the full
20-case test matrix still reports `Result: PASS`, and `cargo build --features f405`
(and f401, f411) now produces a real ARM ELF — confirmed via header bytes
(`0x7F 'E' 'L' 'F'`), not just a successful exit code.

All three real chip×board combinations under option A build clean and warning-free:

```
# from archive/option-a-type-alias/, standalone:
cargo build --no-default-features --features chip-f401,board-a
cargo build --no-default-features --features chip-f401,board-b
cargo build --no-default-features --features chip-f405,board-a
```

Option B, now at the repo root: `cargo build --features f401` (or `f405`/`f411`)
produces a real linked ARM ELF for each chip (confirmed by header bytes, not just
exit code); `.\tools\run-test.ps1` reports `Result: PASS` across all 20 cases.
