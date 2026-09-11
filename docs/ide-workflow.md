# Editing the RTIC skeleton with rust-analyzer

The normal firmware now compiles `src/app_body_skeleton.rs` as a source module.
Keep editing this file. Choose one system (or a chip-only configuration) in `.vscode/settings.json`:

```json
{
  "rust-analyzer.cargo.features": ["system-nucleo-f401re"],
  "rust-analyzer.check.allTargets": false
}
```

This selects the Nucleo and enables `f401` automatically. Other choices include
`system-default-f401` / `system-default-f405` / `system-default-f411`, or a bare
`f401` / `f405` / `f411` feature for chip-only work. The ARM target
comes from `.cargo/config.toml`. Leave procedural macro expansion and build
scripts enabled (their defaults). Reload the rust-analyzer workspace if the
editor has not picked up the new local macro dependency. Do not select all
features: the chips are mutually exclusive.

## Why this works

The previous path was source text -> `build.rs` -> a different file in `OUT_DIR`
-> RTIC. The editable skeleton was not a member of Rust's source module graph,
and reparsing the generated text lost its original source locations.

The new path is `mod app_body_skeleton` -> `for_chip` attribute -> RTIC. The
skeleton contains an inline `mod app { ... }`; its outer `cfg_attr` attributes
select the reducer's chip/system arguments and RTIC's dispatcher list. The
Nucleo uses TIM2, leaving TIM3 available for its LED blink. Default systems and
chip-only builds use TIM2 and TIM3. See [chips and systems](chips-and-systems.md).

`for_chip` parses the input token stream with `syn` and emits the surviving AST
with `quote`. It never stringifies/reparses the application or reads application
files from the filesystem. Existing token spans survive both transformations.
The selected chip is an explicit macro argument, so expansion does not guess the
target's features from the host proc-macro process environment.

This is a supported use of procedural macro tokens and spans; see the
[Rust reference](https://doc.rust-lang.org/reference/procedural-macros.html).
It does not require an IDE-only fake RTIC context or a different program under
`cfg(rust_analyzer)`.

## Original IDE verification

Tested with rustc/rust-analyzer `1.94.0-nightly (fa5eda19, 2025-12-12)`,
`rtic` 2.3.1, and the **resolved `rtic-macros` 2.3.1**. The older spike report's
2.1.0 references describe an earlier dependency state.

| Check | Result |
| --- | --- |
| `cargo build --offline --features f401/f405/f411` (separately) | All three link successfully |
| Existing `tools/run-test.ps1` matrix | 20/20 expected outcomes |
| Reducer unit tests against the actual skeleton and failure cases | 3/3 pass |
| LSP completion at `cx.local.` | Offers `buffer: &mut [u8; 32]` for all three chips |
| LSP hover on `cx.local.buffer` | Resolves the resource type for all three chips |
| LSP go-to-definition on local/shared resources | Returns locations in `app_body_skeleton.rs` for all three chips |
| Unsaved change of buffer length from 32 to 64 | Hover updates to `&mut [u8; 64]` for all three chips, without a build |
| Saved type error: assign `true` to `cx.local.buffer[0]` | Cargo reports E0308 directly at `src/app_body_skeleton.rs:100`, rather than `OUT_DIR` |
| Unsaved call to an undefined function inside the task | Native rust-analyzer diagnostics did **not** report it on this version |

The injected compiler error was removed and a clean F401 check was run afterward.
No hardware was flashed during this investigation. The Nucleo initialization
and blink logic now live in its system module, selected by `system-nucleo-f401re`.
The system separation adds default systems, with the current matrix recorded
in [test-report.md](test-report.md). Source line numbers above refer to the
original IDE experiment before the system refactor.

Run the IDE checks with Python and rust-analyzer on PATH:

```powershell
py -X utf8 tools/check-ide.py f401
py -X utf8 tools/check-ide.py f405
py -X utf8 tools/check-ide.py f411
py -X utf8 tools/check-ide.py system-nucleo-f401re
py -X utf8 tools/check-ide.py system-default-f401
py -X utf8 tools/check-ide.py system-default-f405
py -X utf8 tools/check-ide.py system-default-f411
```

These talk to a real rust-analyzer LSP process and send unsaved document edits;
they never modify the source on disk. Native diagnostic results are printed
separately, not counted as passing merely because completion works.

## Boundaries

- Only the selected configuration is semantically active in the editor. Use
  the build matrix to check other chips and systems.
- Keep Cargo check-on-save enabled. Native unsaved diagnostics inside RTIC
  expansions remain incomplete on the installed rust-analyzer. Completion and
  unsaved type inference are verified; universal refactoring support is not.
- Resource navigation can land on the corresponding name in the RTIC `task`
  attribute, because that is where RTIC creates the context field's span. It
  stays in the editable file, but does not necessarily jump to `Shared`/`Local`.
- The reducer supports registered chip/system-feature predicates. It reduces
  module items, struct fields, struct-literal fields, and `let` statements.
  It rejects unsupported predicates, `cfg_attr` inside the body, and other
  visited conditional positions rather than silently deleting code. Adding
  new systems requires registering them; new class axes require extending the evaluator and its tests. This
  is not a general Rust conditional-compilation implementation.
- Macro invocation contents are opaque. Do not hide RTIC task declarations or
  structural conditions inside another macro; RTIC needs those declarations
  before expanding the app.
- `generated-body` and `invalid-interrupt` intentionally keep the old text
  composition route for the existing regression fixtures. Do not enable those
  while developing the shared skeleton. Chip `app_head.rs` files and preview
  copies now belong to that legacy route; the normal app header is in the
  skeleton itself.

## Other options investigated

Using RTIC directly still rejects the two mutually exclusive `#[init]` functions
with “must appear at most once” on the resolved 2.3.1 macro version. Updating the
version number is therefore not, by itself, a fix for this skeleton.

The external [`cfg_eval` crate](https://docs.rs/cfg_eval), with its `items`
feature, was tested in an isolated scratch package. It compiled the F401/F411
variants, but the F405 shared field still triggered E0658: true field `cfg`
attributes remained for RTIC to copy into generated expression positions.
The custom reducer removes those true attributes as well as inactive nodes.

The local macro is still a workaround for RTIC's handling of cfg. Its advantage
over the previous reducer is where it runs and preservation of source spans,
not the elimination of preprocessing.
