# RTIC2 Multi-Head Composition and Syn Reduction Test Plan

## Objective

Validate the RTIC2 application composition architecture before
implementing the full `syn` reducer.

The test verifies:

-   MCU-specific RTIC app headers can be selected independently.
-   A shared RTIC application body can be reused.
-   The selected app head and body can be composed before `#[rtic::app]`
    expansion.
-   Dispatcher lists can be defined per MCU/application head.
-   The future `syn` stage has a valid insertion point.

------------------------------------------------------------------------

# Architecture Under Test

The intended structure:

``` text
src/
├── main.rs
├── app_body.rs
├── f401.rs
├── f405.rs
├── f411.rs
└── generated/
    └── app.rs
```

## MCU Head

Each MCU file contains the RTIC application header.

Example:

``` rust
#[rtic::app(
    device = pac,
    dispatchers = [
        TIM6,
        TIM7
    ]
)]
mod app {
    /* APP_BODY */
}
```

The MCU head defines:

-   PAC device
-   RTIC dispatcher list
-   MCU-specific configuration

------------------------------------------------------------------------

## Application Body

The shared body contains:

-   Shared resources
-   Local resources
-   Hardware tasks
-   Software tasks
-   Initialization
-   Application logic

Example:

``` rust
#[shared]
struct Shared {
}

#[local]
struct Local {
}

#[init]
fn init(_: init::Context) -> (Shared, Local) {
    (Shared {}, Local {})
}
```

------------------------------------------------------------------------

# Test Environment

Target:

``` text
thumbv7em-none-eabihf
```

Build system:

``` text
cargo check
```

Expected MCU variants:

-   STM32F401
-   STM32F405
-   STM32F411

------------------------------------------------------------------------

# Test Cases

## TC-001: Single MCU Head Selection

### Purpose

Verify one MCU head can compose with the shared body.

### Procedure

Build:

``` bash
cargo check --features f405
```

### Expected Result

PASS.

The selected RTIC app header and application body compile.

------------------------------------------------------------------------

# TC-002: Multiple MCU Heads Independently Compile

### Purpose

Verify the same application body works with multiple MCU heads.

### Procedure

Run:

``` bash
cargo check --features f401
cargo check --features f405
cargo check --features f411
```

### Expected Result

All builds pass independently.

------------------------------------------------------------------------

# TC-003: Dispatcher List Variation

### Purpose

Verify dispatcher lists can vary between MCU heads.

Example:

F401:

``` rust
dispatchers = [
    TIM6
]
```

F405:

``` rust
dispatchers = [
    TIM6,
    TIM7
]
```

### Expected Result

Both MCU variants compile.

------------------------------------------------------------------------

# TC-004: Shared Resource Support

### Purpose

Verify RTIC shared resources work through the composed application.

Example:

``` rust
#[shared]
struct Shared {
    counter: u32,
}
```

Task:

``` rust
#[task(
    binds = USART1,
    shared = [counter]
)]
fn usart1(cx: usart1::Context) {
    cx.shared.counter.lock(|counter| {
        *counter += 1;
    });
}
```

### Expected Result

Application compiles.

------------------------------------------------------------------------

# TC-005: Local Resource Support

### Purpose

Verify local RTIC resources work through the composed application.

Example:

``` rust
#[local]
struct Local {
    buffer: [u8; 32],
}
```

### Expected Result

Application compiles.

------------------------------------------------------------------------

# TC-006: Hardware Task Binding Validation

### Purpose

Verify RTIC still validates interrupt bindings.

Procedure:

Add an invalid interrupt:

``` rust
#[task(binds = UART8)]
fn uart8(_: uart8::Context) {
}
```

Build on a MCU without UART8.

### Expected Result

Compilation fails.

The failure should originate from the PAC interrupt definition.

------------------------------------------------------------------------

# TC-007: Multiple RTIC Heads Selected

### Purpose

Verify exactly one RTIC app head is required.

Procedure:

``` bash
cargo check --features "f401,f405"
```

### Expected Result

Compilation fails.

Expected reason:

``` text
module `app` defined multiple times
```

------------------------------------------------------------------------

# TC-008: Generated Body Compatibility

### Purpose

Verify generated source can replace manually maintained source.

Procedure:

Replace:

``` rust
include!("app_body.rs");
```

with:

``` rust
include!("generated_app_body.rs");
```

where the generated file contains equivalent content.

### Expected Result

No change in RTIC behaviour.

------------------------------------------------------------------------

# TC-009: Syn Reduction Prototype

## Purpose

Validate the actual future architecture.

## Procedure

Implement minimal reducer:

Input:

``` rust
app_body.rs
```

Containing:

``` rust
#[cfg(feature = "uart4")]
#[task(binds = UART4)]
fn uart4(...) {
}
```

Run reducer with:

``` text
uart4 disabled
```

Generate:

``` text
generated_app.rs
```

Compile generated application.

## Expected Result

The disabled task is removed before `#[rtic::app]` expands.

------------------------------------------------------------------------

# Acceptance Criteria

  Requirement                                    Result
  ---------------------------------------------- ----------
  MCU-specific RTIC heads compile                Required
  Shared body compiles with all supported MCUs   Required
  Dispatcher lists can vary                      Required
  Shared resources work                          Required
  Local resources work                           Required
  Invalid interrupt bindings fail                Required
  Multiple heads are rejected                    Required
  Generated body works                           Required
  Syn removal occurs before RTIC expansion       Required

------------------------------------------------------------------------

# Expected Outcome

Successful completion demonstrates that:

``` text
MCU head
    +
resolved application body
    |
    v
single RTIC application
    |
    v
RTIC analysis
    |
    v
firmware build
```

is a viable architecture.

The next development step is implementing the `syn` reducer that
performs the body reduction and app composition automatically.
