# Option A — Type-Alias Skeleton (archived, not the adopted architecture)

This is the working, fully-verified "no extra tooling" alternative from the
compile-check spike — kept for reference, not deleted, because the decision to
adopt option B (the `syn`-based cfg-reducer, now at the repo root) was explicitly
conditional: see `compile-check-spike-report.md` at the repo root.

## What this is

One `#[app]` skeleton per exact chip (`src/chips/{f401,f405}_skeleton.rs`),
per-board role reassignment via type alias (`src/board/config_{a,b}.rs`), resolved
after `rtic::app` expands so the macro never sees a collision. No build-time code
generation — stays entirely within ordinary Rust and RTIC2 as shipped.

## Its known cost

`shared = [...]` and `priority` are literal syntax parsed before typecheck, so they
can't vary per role — the skeleton must declare the union of every role ever
assignable to a vector. Measured directly (see the report): this inflates lock
ceilings for tasks unrelated to the role that caused it, a real loss of the
schedulability-analysis tightness RTIC2 is chosen for.

## When to reconsider this

If a future `rtic`/`rtic-macros` release fixes the cfg-blindness bugs documented in
`compile-check-spike-report.md` (§1 and §2 — the `App::parse` duplicate-check
blindness and the `pre_init_enable_interrupts` cross-chip-binding blindness), the
reducer at the repo root may no longer be needed, and this approach becomes the
simpler option again — no bespoke tool in the build path, at the cost of the
union-ceiling tradeoff above.

A regression check to make that moment detectable automatically (rather than
relying on someone remembering to check) is tracked as deferred work in
`compile-check-spike-report.md`, not yet built.

## To build this

Not wired into the root workspace, but buildable in place — it has its own
`Cargo.toml`, `build.rs`, and `memory.x`, and inherits the repo root's
`.cargo/config.toml` (target + linker flag; a duplicate nested config here would
conflict with the root one — Cargo merges them and processes `memory.x` twice):

```powershell
cd archive/option-a-type-alias
cargo build --target thumbv7em-none-eabihf --no-default-features --features chip-f401,board-a
```

To restore it as the active architecture instead of option B, reverse the move
described in `compile-check-spike-report.md`'s repo-state section.
