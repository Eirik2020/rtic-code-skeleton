# rtic-code-skeleton

Modular RTIC2 firmware framework for a Betaflight-scale board-support ecosystem
with compile-time-selectable EASA assurance classes. See
[`rtic2-framework-architecture-plan.md`](rtic2-framework-architecture-plan.md) for
the full design context, and [`compile-check-spike-report.md`](compile-check-spike-report.md)
for why this repo is structured the way it is.

## Architecture (adopted)

[`src/app_body_skeleton.rs`](src/app_body_skeleton.rs) is an ordinary Rust module
loaded by `main.rs`. Its `for_chip` attribute runs a small `syn` reducer before
`#[rtic::app]`, removing inactive chip tasks, resources, and initializers. It
preserves the original tokens' source spans, so rust-analyzer completion, hover,
navigation, and compiler diagnostics refer to the file you edit.

The reducer lives in [`tools/rtic-app-cfg`](tools/rtic-app-cfg/src/lib.rs).
It handles registered chip and system features with `all`, `any`, and `not`.
Unsupported conditional syntax produces an explicit error. Dispatcher
selection is at the top of the skeleton. `build.rs` generates the
linker memory layout; text composition and the separate chip heads are retained
for the `generated-body` and `invalid-interrupt` regression fixtures.

See [IDE workflow and verification](docs/ide-workflow.md) for the tested editor
behavior and limitations. In particular, this rust-analyzer version misses some
unsaved errors inside RTIC tasks; saving runs Cargo checks with correct source
locations.

This is an explicitly **conditional** choice: if a future `rtic`/`rtic-macros`
release fixes the underlying cfg-blindness, this preprocessing step should be
reconsidered in favor of the simpler, no-extra-tooling alternative kept at
[`archive/option-a-type-alias/`](archive/option-a-type-alias/).

## Build

For the ST NUCLEO-F401RE system, including the LD2 blink:

```powershell
cargo build-nucleo-f401re
cargo embed-nucleo-f401re     # builds and flashes; add --release if wanted
```

For the minimal default systems (no board-specific wiring):

```powershell
cargo build-default-f401     # or build-default-f405 / build-default-f411
cargo embed-default-f401     # or embed-default-f405 / embed-default-f411
```

Each system feature automatically selects its chip. For example,
`system-nucleo-f401re` and `system-default-f401` both enable `f401`, but only the
Nucleo system enables its LED wiring/setup. The IDE currently selects the Nucleo.

Chip-only checks/builds remain available as `cargo check-f401` / `cargo
build-f401` (also F405/F411). These do **not** initialize Nucleo wiring or blink
its LED. The low-level `embed-f401` alias likewise flashes a chip-only skeleton;
use `embed-nucleo-f401re` for the board firmware.

Exactly one chip and at most one system can be selected. Selecting a system
together with an incompatible chip, or selecting two systems, is rejected.
See [chips and systems](docs/chips-and-systems.md) for ownership and configuration.

## Test

```powershell
.\tools\run-test.ps1
```

Runs the full 30-case compile-time matrix and writes `docs/test-report.md`.

For reducer and IDE checks:

```powershell
cargo test -p rtic-app-cfg --target x86_64-pc-windows-msvc
py -X utf8 tools/check-ide.py f401  # also f405 / f411; needs rust-analyzer on PATH
py -X utf8 tools/check-ide.py system-nucleo-f401re
py -X utf8 tools/check-ide.py system-default-f401  # also default-f405 / default-f411
```

## Layout

```
src/
├── main.rs                  # loads the editable skeleton as a Rust module
├── app_body_skeleton.rs     # shared RTIC app body — the thing you actually edit
├── invalid_interrupt.rs     # deliberately-broken fixture for the negative test case
├── systems/
│   ├── catalog.rs                         # registered systems and their chips
│   ├── nucleo_f401re/{config.toml, mod.rs} # physical board, wiring/setup
│   ├── default_f401/config.toml           # minimal system using an F401 part
│   ├── default_f405/config.toml           # minimal system using an F405 part
│   └── default_f411/config.toml           # minimal system using an F411 part
└── chips/
    ├── f401/{app_head.rs, config.toml}
    ├── f405/{app_head.rs, config.toml, app_body.rs}   # app_body.rs: generated-body stand-in
    └── f411/{app_head.rs, config.toml}
build.rs                     # chip/system validation, part linker layout, legacy fixtures
docs/                        # test plans, test report, design summary
tools/run-test.ps1           # the test matrix
tools/rtic-app-cfg/          # source-preserving chip reduction attribute macro
tools/check-ide.py           # real LSP checks, including unsaved source edits
archive/option-a-type-alias/ # working alternative, not adopted — see its README
```
