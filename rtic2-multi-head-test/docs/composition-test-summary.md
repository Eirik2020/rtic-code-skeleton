# RTIC2 Multi-Head App Composition Test Summary

Date: 2026-09-10

## Conclusion

The composition architecture is viable:

- Each supported MCU can have its own RTIC app head.
- The application body can remain unified and shared.
- Cargo features select the supported hardware configuration.
- The selected head and filtered body must be composed before `#[rtic::app]` expands.
- The generated application compiles through the normal Cargo and RTIC toolchain.

## Test Workspace

The self-contained fixture is `rtic2-multi-head-test/`.

```text
rtic2-multi-head-test/
|-- Cargo.toml
|-- build.rs
|-- Embed.toml
|-- docs/
|   |-- README.md
|   |-- composition-test-summary.md
|   `-- test-report.md
|-- tools/
|   `-- run-test.ps1
|-- src/
|   |-- main.rs
|   |-- app_body_skeleton.rs
|   |-- invalid_interrupt.rs
|   `-- chips/
|       |-- f401/
|       |   |-- app_head.rs
|       |   |-- config.toml
|       |   `-- app_body.rs
|       |-- f405/
|       |   |-- app_head.rs
|       |   |-- config.toml
|       |   `-- app_body.rs
|       `-- f411/
|           |-- app_head.rs
|           |-- config.toml
|           `-- app_body.rs
```

`app_body_skeleton.rs` is the manually authored shared body. Each chip's
`app_body.rs` is a stand-in for future `syn` output. Each `config.toml` records the chip
and PAC feature mapping. `build.rs` selects a
chip directory, replaces the `/* APP_BODY */` marker, and emits a chip-named concrete
app under Cargo's `OUT_DIR`.

## Results

All eleven matrix cases passed:

| Case | Expected | Result |
| --- | --- | --- |
| F401 shared body | Compile | Pass |
| F405 shared body | Compile | Pass |
| F411 shared body | Compile | Pass |
| F401 generated body | Compile | Pass |
| F405 generated body | Compile | Pass |
| Multiple heads selected | Reject | Pass |
| Invalid `UART8` binding | Reject | Pass |
| Syn reduction removes F405-only `UART4` task for F411 | Compile | Pass |

The complete command output is retained in [test-report.md](test-report.md).

## Reproduce

From the fixture root:

```powershell
.\tools\run-test.ps1
```

The matrix uses the `thumbv7em-none-eabihf` target and Cargo offline mode.

## Cargo Embed

The chip profiles are in `Embed.toml`. The current `cargo-embed` interface does not
map a profile name to a Cargo feature automatically, so use:

```powershell
cargo embed f405 --features f405 --target thumbv7em-none-eabihf
```

The same pattern applies to `f401` and `f411`.

## Design Boundary

Rust Analyzer can validate the feature-selected source view. The future `syn` tool
should then parse the shared body, remove hardware-incompatible `cfg` items, compose
the selected head, and emit a concrete RTIC app. Cargo should compile that generated
output for every supported feature.

A raw nested `include!("app_body_skeleton.rs")` inside `#[rtic::app]` is not sufficient: the
procedural macro sees the include before the body is expanded and reports that the
RTIC resources are missing. The build-time composition in this fixture demonstrates
the required ordering.

The minimal TC-009 reducer uses `syn` to parse the body and remove the top-level
`#[cfg(feature = "f405")]` `UART4` task when F411 is selected. The resulting app is
then composed and compiled before RTIC expansion. This proves the insertion point for
the future full reducer; it does not yet implement general `cfg` expression evaluation.

The generated filename is also chip-specific, and the build script declares each chip
source and feature environment variable as an input. This keeps successive F401, F405,
and F411 builds from sharing or overwriting the wrong generated app artifact.
