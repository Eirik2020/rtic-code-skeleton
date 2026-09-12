# rtic-code-skeleton

See [Purpose](docs/architecture/overview.md#purpose) for what this repository
is actually for. Start with the [documentation index](docs/README.md) and
[current architecture overview](docs/architecture/overview.md). The [original
architecture plan](docs/history/original-architecture-plan.md) and
[compile-check spike report](docs/history/compile-check-spike-report.md) are
retained as historical rationale.

## Architecture (adopted)

[`src/app_body_skeleton.rs`](src/app_body_skeleton.rs) is an ordinary Rust module
loaded by `main.rs`. Its `for_chip` attribute runs a small `syn` reducer before
`#[rtic::app]`, removing inactive chip tasks, resources, and initializers. It
preserves the original tokens' source spans, so rust-analyzer completion, hover,
navigation, and compiler diagnostics refer to the file you edit.

The reducer lives in [`tools/rtic-app-cfg`](tools/rtic-app-cfg/src/lib.rs).
It handles registered chip, system, prototype-board, and software selections
with `all`, `any`, and `not`.
Unsupported conditional syntax produces an explicit error. Resource types,
feature gates, and initializer expressions remain hand-written in the skeleton.
Dispatcher selection is at the top of the skeleton.
The unified initializer freezes the clock once, starts a 1 kHz SysTick
monotonic, constructs selected hardware, and then assembles RTIC's resource
owners. `build.rs` generates the linker memory layout; text composition and the
separate chip heads are retained for the `generated-body` and
`invalid-interrupt` regression fixtures.

See [IDE workflow and verification](docs/development/ide-workflow.md) for the tested editor
behavior and limitations. In particular, this rust-analyzer version misses some
unsaved errors inside RTIC tasks; saving runs Cargo checks with correct source
locations.

This is an explicitly **conditional** choice: if a future `rtic`/`rtic-macros`
release fixes the underlying cfg-blindness, this preprocessing step should be
reconsidered in favor of the simpler, no-extra-tooling alternative kept at
[`archive/option-a-type-alias/`](archive/option-a-type-alias/).

## Build

For the ST NUCLEO-F401RE prototype, including the monotonic LD2 blink and the
TIM3-triggered `defmt` report:

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
Nucleo system enables its LED wiring/setup. The Nucleo aliases and IDE also
enable the independent `sw-report` feature; selecting only the system feature
leaves that boot route disabled.

Chip-only checks/builds remain available as `cargo check-f401` / `cargo
build-f401` (also F405/F411). These do **not** initialize Nucleo wiring or blink
its LED. The low-level `embed-f401` alias likewise flashes a chip-only skeleton;
use `embed-nucleo-f401re` for the board firmware.

Exactly one chip and at most one system can be selected. Selecting a system
together with an incompatible chip, or selecting two systems, is rejected.
See [hardware and systems](docs/architecture/hardware-and-systems.md) for ownership
and configuration.

## Test

```powershell
.\tools\run-test.ps1
```

Runs the full 32-case compile-time matrix and writes
`docs/verification/test-report.md`.

For reducer and IDE checks:

```powershell
cargo test -p rtic-app-cfg --target x86_64-pc-windows-msvc
cargo test -p catalog --target x86_64-pc-windows-msvc
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
├── app_prelude.rs           # monotonic/HAL setup the app body imports, not part of it
├── clock.rs                 # generated rcc::Config from the active system's clock selection
├── hardware/                # generic peripheral construction, one file per kind (gpio.rs, timer.rs, common.rs)
├── boards/                  # per-board wiring, one file per board (nucleo_f401re.rs)
└── chips/
    ├── f401/{app_head.rs}
    ├── f405/{app_head.rs, app_body.rs}   # app_body.rs: generated-body stand-in
    └── f411/{app_head.rs}
build.rs                     # chip/system validation, part linker layout, legacy fixtures
catalog/                     # chip/system/board data shared by build and reducer — Rust, not TOML (for now)
clippy.toml                  # restricts peripheral-driver construction methods to src/hardware/
docs/                        # routed architecture, development, verification, roadmap, and history
tools/run-test.ps1           # the test matrix
tools/rtic-app-cfg/          # source-preserving chip reduction attribute macro
tools/check-ide.py           # real LSP checks, including unsaved source edits
archive/option-a-type-alias/ # working alternative, not adopted — see its README
```
