# RTIC2 Multi-Head App Composition Test Plan

## Objective

Verify that a single RTIC application body can be reused with multiple
MCU-specific RTIC app headers.

The test validates:

-   Different MCU heads can define different `#[rtic::app(...)]`
    attributes.
-   A shared application body can compile with each head.
-   Dispatcher lists can vary between MCU heads.
-   RTIC receives only one app instance per build.
-   The architecture works before introducing the `syn` reducer.

------------------------------------------------------------------------

# Test Structure

## File Layout

``` text
src/
├── main.rs
├── app_body.rs
├── f401.rs
├── f405.rs
└── f411.rs
```

------------------------------------------------------------------------

# Test 1 --- Basic Single Head Compilation

## Purpose

Verify that the RTIC app can be split into a header and body.

## Configuration

`main.rs`

``` rust
#[cfg(feature = "f405")]
mod f405;
```

`f405.rs`

``` rust
#[rtic::app(
    device = stm32f4xx_hal::pac,
    dispatchers = [TIM6, TIM7]
)]
mod app {
    include!("app_body.rs");
}
```

`app_body.rs`

``` rust
#[shared]
struct Shared {}

#[local]
struct Local {}

#[init]
fn init(_: init::Context) -> (Shared, Local) {
    (Shared {}, Local {})
}
```

## Expected Result

``` bash
cargo check --features f405
```

Passes.

------------------------------------------------------------------------

# Test 2 --- Multiple MCU Heads

## Purpose

Verify different RTIC headers can use the same application body.

Add:

`f401.rs`

``` rust
#[rtic::app(
    device = stm32f4xx_hal::pac,
    dispatchers = [TIM6]
)]
mod app {
    include!("app_body.rs");
}
```

## Test

``` bash
cargo check --features f401
cargo check --features f405
```

## Expected Result

Both builds compile.

------------------------------------------------------------------------

# Test 3 --- Dispatcher Variation

## Purpose

Verify dispatcher lists belong to the MCU/application head.

Example:

F401:

``` rust
dispatchers = [TIM6]
```

F405:

``` rust
dispatchers = [TIM6, TIM7]
```

## Expected Result

Both builds compile.

------------------------------------------------------------------------

# Test 4 --- Shared Hardware Task Body

## Purpose

Verify hardware tasks can exist inside the shared application body.

Add:

``` rust
#[task(binds = USART1)]
fn usart1(_: usart1::Context) {

}
```

## Expected Result

Build compiles for compatible MCU heads.

------------------------------------------------------------------------

# Test 5 --- MCU-Specific Interrupt Availability

## Purpose

Verify invalid interrupt bindings fail.

Example:

``` rust
#[task(binds = UART8)]
fn uart8(_: uart8::Context) {

}
```

## Expected Result

The MCU build without UART8 support fails.

------------------------------------------------------------------------

# Test 6 --- Shared Resources

## Purpose

Verify shared RTIC resources work through the split architecture.

Add:

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
    cx.shared.counter.lock(|x| {
        *x += 1;
    });
}
```

## Expected Result

Build compiles.

------------------------------------------------------------------------

# Test 7 --- Local Resources

## Purpose

Verify local RTIC resources work through the split architecture.

Add:

``` rust
#[local]
struct Local {
    buffer: [u8; 32],
}
```

Task:

``` rust
#[task(
    binds = USART1,
    local = [buffer]
)]
fn usart1(cx: usart1::Context) {

}
```

## Expected Result

Build compiles.

------------------------------------------------------------------------

# Test 8 --- Multiple RTIC Apps Selected

## Purpose

Verify that multiple app heads cannot be enabled simultaneously.

Test:

``` bash
cargo check --features "f401 f405"
```

## Expected Result

Build fails because multiple `#[rtic::app]` instances exist.

------------------------------------------------------------------------

# Test 9 --- Prepare for Syn Integration

## Purpose

Verify generated application body behaves the same as manually included
body.

Replace:

``` rust
include!("app_body.rs");
```

with:

``` rust
include!("generated_app_body.rs");
```

where the generated file contains the same RTIC body.

## Expected Result

No change in RTIC behavior.

------------------------------------------------------------------------

# Success Criteria

  Test                                 Requirement
  ------------------------------------ -------------
  Single head compilation              Pass
  Multiple heads independently         Pass
  Different dispatcher lists           Pass
  Shared resources                     Pass
  Local resources                      Pass
  Hardware tasks                       Pass
  Invalid interrupt binding rejected   Pass
  Multiple heads rejected              Pass
  Generated body compatibility         Pass

------------------------------------------------------------------------

# Next Step After Successful Spike

Do not implement the full generator immediately.

The next minimal `syn` experiment:

1.  Parse `app_body.rs`.
2.  Remove one disabled `#[cfg]` task.
3.  Insert the remaining body into a selected MCU head.
4.  Emit `generated_app.rs`.
5.  Compile the generated application.

This validates the actual architecture with minimal tooling.
