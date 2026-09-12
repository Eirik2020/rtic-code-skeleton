# Architecture overview

Status: current architecture index.

## Purpose

The end goal of this repository is Betaflight-like board-level support for a
flight controller (FC) firmware framework — not a generic "any embedded
project" skeleton. Board manifests are meant to work like Betaflight's own
per-board target files; RC input, MSP, GPS, telemetry, motor/servo outputs,
and similar concepts that appear throughout these docs are the literal target
domain, not illustrative examples of a more general system. The
peripheral/use-case surface is deliberately constrained to what real flight
controllers need — a known, bounded set (motor/servo PWM, UART, SPI, I2C, ADC,
status LED, buzzer), matching Betaflight's own fixed resource vocabulary —
not something to design as an open-ended, general-purpose abstraction. There
is no plan to support HAL portability beyond STM32F4 chips.

Today this is a modular RTIC2 firmware skeleton for single-core STM32F4
targets, supporting STM32F401, STM32F405, and STM32F411 selections plus an ST
NUCLEO-F401RE system implementation — the prototype the FC-specific
architecture is being proven out on before it needs to generalize to real FC
hardware.

The main architectural goals are:

- keep the RTIC application and task/resource graph hand-written;
- reject incompatible chip and system selections at build time;
- preserve useful source locations and rust-analyzer behavior;
- separate chip-family facts from board wiring and application behavior;
- retain RTIC's compile-time ownership and priority analysis.

## Current build flow

```text
Cargo chip/system/software features + catalog chip/system/board data
          |
          +----> build.rs writes memory.x, clock config, board cfgs, and HAL/PAC type checks
          |
Editable app_body_skeleton.rs
          |
          v
rtic_app_cfg::for_chip
          |
          v
cfg-reduced RTIC application
          |
          v
#[rtic::app] expansion
          |
          v
Firmware
```

Normal builds compile the editable skeleton directly through a source-preserving
attribute macro. The older text-composition path is retained only for explicit
regression fixtures.

## Terms and ownership

| Term | Meaning | Current owner |
| --- | --- | --- |
| Chip | HAL/PAC family selection such as `f401` | Cargo feature and `catalog::Chip`/`ChipConfig` |
| Build target record | Current linker memory and probe properties | A `Part` in the chip's `ChipConfig` |
| System | Chip selection plus physical board integration | `catalog::SystemConfig` |
| Board manifest | Chip, enabled peripheral instances, and their physical pin and DMA mappings | Prototype `catalog::boards::nucleo_f401re::config()` |
| Software capability | Independently compiled behavior such as RC input or telemetry | `crate::software::<capability>` (e.g. `serial_protocol::SerialProtocol`) |
| Boot routing | Default association between a software role and an enabled hardware port | `catalog::SerialRoute`/`SystemConfig.default_serial_routes`, resolved by `crate::routing` in `#[init]` |
| Application graph | RTIC resources, tasks, priorities, and bindings | `src/app_body_skeleton.rs` |
| Build validation | Feature compatibility, board/PAC/HAL checks, part lookup, probe match, linker memory, clock ceiling | `build.rs` |

The chip catalog currently contains exact-part strings for linker memory and
probe selection. These are mechanical build records, not capability
authorities. They do not select RTIC resources or generate initialization
code.

## Architectural boundaries

- The RTIC cfg reducer is a narrow workaround for known RTIC cfg handling. It is
  not a general Rust preprocessor.
- Board wiring is system-owned. It must not be represented as a generic chip
  capability.
- Hardware availability and software capabilities are independent. Systems do
  not permanently bind software roles to peripheral instances; boot-time
  configuration makes and validates those associations.
- Application resources and task ownership remain visible in the hand-written
  RTIC skeleton.
- Configuration files may describe data already supported by the build. Adding a
  DSL or making configuration encode executable behavior requires explicit user
  permission under the repository instructions.
- Peripheral capability checks stop at the chip-family level. The framework is
  intended to prevent common configuration mistakes, not maintain an exhaustive
  exact-MPN database. Device-specific mismatches may fail later in Rust, the HAL,
  or the PAC, and that compile failure is an acceptable backstop.

## Current limitations

- There is one unified RTIC `#[init]` entry point, but not yet a three-chip
  hardware-initialization abstraction.
- A generalized *default* serial-protocol route is implemented (system data
  translated into a port's settings at boot — see
  [hardware and software routing](hardware-software-routing.md)), but there
  is no actual protocol decoding logic yet, and no persistent or
  runtime-configurable routing store; the Nucleo prototype's TIM3-to-report
  route remains the only non-serial example, still a fixed boolean.
- Board manifest data/code generation is implemented only for NUCLEO-F401RE.
  Other systems do not yet have boards.
- Only the Nucleo system constructs real board hardware; F405 and F411 currently
  compile the skeleton without chip-specific peripheral setup.
- Clock selection is validated only against the chip's maximum sysclk — there
  is no PLL-divider-feasibility model, so an exactly-reachable target frequency
  is still the HAL's own concern at `freeze()` time.
- The prototype validates actual peripheral and pin mappings through the typed
  HAL construction in its board module. Cross-entry conflicts, DMA setup, and interrupt derivation are not
  yet modeled, nor is a complete per-device capability model intended.
- The reducer remains safety-relevant build tooling and must be reconsidered if
  upstream RTIC becomes cfg-aware for the required patterns.

## Category map

- [RTIC composition](rtic-composition.md)
- [Hardware and systems](hardware-and-systems.md)
- [Hardware and software routing](hardware-software-routing.md)
- [NUCLEO-F401RE board manifest prototype](board-toml-prototype.md)
- [Initialization and monotonic time](initialization-and-time.md)
- [Build and configuration](build-and-configuration.md)
- [Decisions and open items](decisions.md)
