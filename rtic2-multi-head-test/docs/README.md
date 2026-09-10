# RTIC2 Multi-Head App Composition Test

This is a self-contained Cargo package for `rtic2_multi_head_app_composition_test_plan.md`.
It lives below the main repository but has its own manifest and source tree, so the main
application can continue to be built normally.

The heads in `src/chips/f401/`, `src/chips/f405/`, and `src/chips/f411/` provide
different RTIC headers and dispatcher lists. `build.rs` composes the selected head
with the shared `src/app_body_skeleton.rs` before the RTIC attribute expands it.

Chip-owned files now live under `src/chips/f401/`, `src/chips/f405/`, and
`src/chips/f411/`. Each directory contains its app head, TOML chip config, and
generated `app_body.rs` body. The composer emits `generated_app_f401.rs`, `generated_app_f405.rs`, or
`generated_app_f411.rs` in Cargo's per-build output directory, so one chip build does
not overwrite another chip's generated app.

This composition step is intentional: a raw `include!("app_body_skeleton.rs")` nested inside
`#[rtic::app]` is seen too late by the procedural attribute and does not compile.

## Run

From the fixture root:

```powershell
.\tools\run-test.ps1
```

Or from the repository root:

```powershell
.\rtic2-multi-head-test\tools\run-test.ps1
```

The script checks the positive cases, expects the multiple-head build to fail, expects
the invalid `UART8` binding to fail, and writes the complete evidence to
`docs/test-report.md`. `Cargo.lock` is retained as part of the test artifact after the first
run.

The fixture also contains a minimal `syn` reducer in `build.rs`. It parses the selected
body and removes top-level items guarded by `#[cfg(feature = "f405")]` when another
chip is selected, before composing the RTIC head. The reduction case is included in
the matrix and the emitted source is under Cargo's `target/.../out/generated_app_<chip>.rs`.

The reducer also filters F405-only `Shared` and `Local` resource fields and their
`init` struct-literal fields. This keeps chip-specific resources out of the RTIC app
when another chip is selected.

The matrix also tests progressively more aggressive cfg placement: an entire task,
cfg on the task header, and a task using a cfg-gated local resource. Each F405 task
shares its interrupt with an F411 control task, so leaving both branches in the output
would produce an RTIC duplicate-binding error.

Compound `all`, `any`, and `not` predicates are evaluated by the reducer, and a
cfg-gated software function is covered from the authored skeleton. A build with no
chip feature is also expected to fail clearly.

## Cargo Embed

`Embed.toml` provides `f401`, `f405`, and `f411` probe-chip profiles. Because
`cargo-embed` treats the positional name as an Embed profile, not a Cargo feature, pass
the matching feature explicitly:

```powershell
cargo embed f401 --features f401 --target thumbv7em-none-eabihf
cargo embed f405 --features f405 --target thumbv7em-none-eabihf
cargo embed f411 --features f411 --target thumbv7em-none-eabihf
```

This composition fixture currently builds all three simulated heads against the cached
F401 PAC. The profiles describe the intended probe chips, but should not be used to
flash real F405 or F411 hardware until the dependency is switched to the matching PAC.
