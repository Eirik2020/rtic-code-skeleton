# RTIC Framework Next Stage Plan

Status: roadmap proposal. It does not describe implemented behavior. Compare it
with the [current architecture overview](../architecture/overview.md) before
acting on it.

Capability modelling in this proposal must be interpreted as coarse
chip-family-level validation. Exact-MPN capability databases are explicitly out
of scope; device-specific incompatibilities may be left to compilation, the
HAL, or the PAC.

## Objective

Extend the current RTIC code skeleton into a more complete hardware abstraction and configuration framework.

The current architecture uses a source-preserving preprocessing stage before `#[rtic::app]` because RTIC currently cannot correctly handle the required `cfg` usage inside the application macro.

The next stage focuses on:

- MCU capability modelling
- clock configuration handling
- peripheral resource validation
- improved initialization structure

The goal is to make unsupported hardware configurations fail early and clearly.

---

# Current Architecture

Current flow:

```
Application source
        |
        v
rtic-app-cfg preprocessor
        |
        v
Reduced RTIC application
        |
        v
RTIC macro expansion
        |
        v
Firmware
```

The preprocessor remains responsible only for solving RTIC `cfg` limitations.

It should not become a general Rust preprocessor.

---

# Stage 1 — MCU Capability Database

## Goal

Create a machine-readable description of supported MCUs.

The database defines what exists physically on the chip.

Example:

```
chips/
 ├── stm32f401.toml
 ├── stm32f405.toml
 ├── stm32f411.toml
 └── stm32h743.toml
```

---

## MCU Definition

Example:

```toml
[chip]
name = "STM32F405"
family = "stm32f4"
core = "cortex-m4"
fpu = true

[clock]
max_frequency = 168000000

[peripherals]

uart = 6
spi = 3
i2c = 3
adc = 3
timers = 14
```

---

## Requirements

The MCU database must define:

- CPU core
- architecture
- FPU availability
- maximum clock frequency
- memory regions
- interrupt vectors
- available peripherals
- DMA capabilities

---

# Stage 2 — Peripheral Resource Model

## Goal

Prevent invalid hardware configurations.

The framework should know:

- which peripherals exist
- which resources they can use
- which combinations are impossible

Example:

STM32F405:

```
UART1
UART2
UART3
UART4
UART5
UART6
```

A request for:

```
UART7
```

should fail during configuration generation.

---

# Peripheral Description

Example:

```toml
[uart.uart4]

available = true

[dma]

rx = [
    "DMA1_STREAM2_CH4",
    "DMA1_STREAM5_CH4"
]

tx = [
    "DMA1_STREAM4_CH4"
]
```

---

# Stage 3 — Clock Configuration

Adopted in a bounded form: a system's clock selection validated against a
chip-family max sysclk, HSE availability declared on its board. No
PLL-divider-feasibility modeling. Currently expressed as typed Rust data
(`catalog`), not TOML — see
[`decisions.md`](../architecture/decisions.md#clock-configuration-roadmap-stage-3-adopted-in-bounded-form)
and
[`decisions.md`](../architecture/decisions.md#toml-deferred-for-chipsystemboard-data).

## Goal

Move clock validation out of application code.

The selected MCU defines the valid clock limits.

The board defines the clock source.

Example:

```toml
[clock]

source = "HSE"
input = 8000000
target = 168000000
```

Validation:

```
Requested:
168 MHz

STM32F405 maximum:
168 MHz

Result:
VALID
```

Invalid example:

```
Requested:
200 MHz

Result:
ERROR:
Clock frequency exceeds MCU capability
```

---

# Stage 4 — Board Description

## Goal

Separate MCU capability from physical board implementation.

Each registered system will contain its own physical manifest:

```text
src/systems/<system>/board.toml
```

The board defines:

- selected chip family and board clock facts;
- enabled peripheral instances;
- peripheral signal pins;
- peripheral DMA mappings;
- connected hardware and board-specific setup.

It does not assign software roles such as RC input or MSP to those peripherals.
That association is installed from boot configuration. A concrete manifest
schema and parser require explicit DSL permission and are not defined by this
roadmap text.

---

# Stage 5 — Initialization Layer

## Goal

Create a common hardware initialization interface.

Current:

```rust
init()
{
    manually configure everything
}
```

Target:

```rust
init()
{
    let hardware = Hardware::init(device);

    ...
}
```

---

Generated structure:

```
hardware/

clock.rs
gpio.rs
uart.rs
dma.rs
spi.rs
timers.rs
```

---

# Stage 6 — Boot-Time Peripheral Role Assignment

## Architecture decision

Hardware ports and software capabilities are selected independently. Do not
compile a fixed association such as RC input belonging to UART4. Boot
configuration validates and installs mappings such as:

```
UART4

    |
    +-- MSP
    +-- GPS
    +-- Telemetry
    +-- Disabled
```

The hardware ISR and RTIC resource ownership remain statically bound to UART4.
The boot router determines which compiled software capability receives or sends
through that port. The configuration format and routing interface remain open
design items.

---

# Stage 7 — Configuration Compiler

Long-term goal:

```
board.toml

        |
        v

ferro-config

        |
        +--> Cargo features
        |
        +--> linker memory
        |
        +--> clock configuration
        |
        +--> pin configuration
        |
        +--> RTIC configuration
        |
        +--> validation report
```

---

# Design Rules

## Rule 1

The RTIC application remains hand-written.

Do not generate RTIC task code unless absolutely necessary.

---

## Rule 2

The configuration system validates hardware.

It should catch:

- nonexistent peripherals
- invalid DMA mappings
- impossible clocks
- conflicting resources

before compilation.

---

## Rule 3

The MCU database contains hardware facts only.

It must not contain application decisions.

Correct:

```
STM32F405 has UART4
```

Incorrect:

```
UART4 is MSP
```

---

## Rule 4

Safety-critical ownership stays compile-time.

Examples:

- motor outputs
- watchdog
- IMU timing
- control loop timers

These should not change dynamically.

Logical boot routing does not change this RTIC ownership. It changes which
compiled software capability communicates through an already-owned hardware
driver.

---

# Development Order

## Phase 1

Complete MCU database.

Target:

- STM32F401
- STM32F405
- STM32F411

---

## Phase 2

Implement clock validation.

---

## Phase 3

Implement peripheral capability validation.

---

## Phase 4

Connect generated hardware initialization.

---

## Phase 5

Expand to Pixhawk-class STM32H7 hardware.

---

# Expected Result

The final architecture should provide:

```
Single board description
        |
        v
Validated hardware model
        |
        v
Generated configuration
        |
        v
RTIC application
```

The developer should only need to describe the hardware setup. Invalid configurations should fail with a clear configuration error instead of a deep Rust compiler error.
