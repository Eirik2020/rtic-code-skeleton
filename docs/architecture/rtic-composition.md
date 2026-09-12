# RTIC composition

Status: current adopted composition design.

## Problem

RTIC performs whole-application analysis inside one `#[rtic::app]` invocation.
The required chip variants cannot be assembled from inner `macro_rules!` task
blocks, and the pinned RTIC implementation does not correctly eliminate all
inactive `#[cfg]` tasks and resource fields before its duplicate-binding,
interrupt, and resource code generation.

Direct feature gating inside the application can therefore produce failures such
as:

- duplicate task or interrupt bindings from mutually exclusive variants;
- references to interrupt variants absent from the selected PAC;
- surviving shared-resource cfg attributes copied into generated expression
  positions;
- multiple `#[init]` functions being rejected before cfg elimination.

The tuple returned by `#[init]`, `(Shared, Local)`, is ordinary RTIC/Rust usage.
The problematic boundary is conditional application structure seen by RTIC.

## Adopted normal path

`src/app_body_skeleton.rs` contains one inline RTIC application. An outer
`rtic_app_cfg::for_chip(...)` attribute runs before `#[rtic::app]` and removes
inactive syntax while preserving the source token spans.

The macro accepts an explicitly selected chip, optional system, and an explicit
list of active software features:

```rust
rtic_app_cfg::for_chip(f401)
rtic_app_cfg::for_chip(f401, "system-nucleo-f401re")
rtic_app_cfg::for_chip(
    f401,
    "system-nucleo-f401re",
    software = ["sw-report"],
)
```

Inside the RTIC module it evaluates only:

- registered chip feature predicates;
- registered system and software feature predicates;
- `board_peripheral`, `board_pin`, and `board_dma` predicates from the selected
  prototype board manifest;
- `all(...)`, `any(...)`, and single-operand `not(...)` combinations.

It reduces supported module items, struct fields, struct-literal fields, and
`let` statements. It removes retained cfg attributes as well as inactive nodes,
because leaving a true attribute can still trigger RTIC code-generation defects.
Unsupported predicates or placements fail closed with a compile error.

This is custom, safety-relevant preprocessing. The Nucleo board predicates and
`sw-report` selector are an explicitly authorized prototype. Extending their
grammar, values, or system coverage requires fresh user permission.

## Source and IDE behavior

The macro transforms the compiler-provided token stream and emits surviving
tokens without stringifying or reparsing the application file. This keeps normal
diagnostics, hover, completion, and navigation associated with the editable
skeleton. Only one selected feature configuration is active in an editor session.

## Legacy fixture path

The `generated-body` and `invalid-interrupt` features exercise the original
`build.rs` text-composition implementation:

```text
chip app_head.rs + reduced app body -> generated_app_<chip>.rs
```

The `/* APP_BODY */` marker and the cfg reducer inside `build.rs` belong only to
these regression fixtures. They are not the normal development path.

## Rules

- Keep RTIC task declarations, resource lists, priorities, and bindings
  hand-written and visible in the skeleton.
- Do not hide RTIC structural declarations inside another macro.
- Do not turn the reducer into a general cfg evaluator or configuration compiler.
- Registering a system currently requires updating the system catalog and the
  explicit selector at the top of the skeleton.
- Re-test the original raw-RTIC failure patterns when upgrading RTIC.

Detailed experimental evidence is preserved in the
[compile-check spike report](../history/compile-check-spike-report.md).
