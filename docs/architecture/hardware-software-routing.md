# Hardware and software routing

Status: adopted direction, partially implemented. A generalized *default*
serial-protocol route is implemented (this section); a persistent,
runtime-configurable router, and any actual protocol decoding logic, are not.

## Separation

Physical hardware and software behavior are independent selections:

| Layer | Examples | Responsibility |
| --- | --- | --- |
| Chip family | UART, SPI, timer instances | Coarse statement of hardware the family can support |
| System hardware | Wired and enabled UART4, SPI1, TIM3 | Pins, electrical connections, peripheral construction, interrupts, and DMA |
| Software capability | RC input, MSP, GPS, telemetry | Protocol and application behavior independent of a physical port |
| Boot routing | RC input uses UART4 | Validated association for this boot configuration |

A system exposing UART4 does not imply that UART4 is RC input. Likewise,
compiling RC input does not select a UART. Both must exist before a boot mapping
can connect them.

## Compile-time responsibilities

Compilation establishes the available pieces:

- the selected chip family supplies coarse hardware guardrails;
- the system selects and constructs its physically available ports;
- independently selected software capabilities are compiled;
- RTIC interrupt bindings, resource ownership, and driver tasks remain static;
- impossible family-level hardware selections cause a build failure.

Only enabled hardware and software should be present in the firmware. No
hardware-to-software role assignment is baked into their cfg selection.

## Boot-time responsibilities

At boot, configuration associates software roles with enabled hardware ports.
The routing layer must reject mappings that reference an unavailable port, use
an incompatible port kind, or claim the same exclusive resource more than once.

Boot routing does not require changing RTIC's task graph dynamically. A
peripheral interrupt remains bound to its hardware driver; the driver communicates
through a stable routing boundary to whichever compiled software capability was
assigned during initialization.

## Implemented: default serial-protocol routing

A system may declare default associations between its board's serial ports
and a serial protocol identity — `catalog::SerialRoute { pin, protocol }`,
listed in `SystemConfig.default_serial_routes`. This is genuinely the
separation the table above describes, not a hardwired binding:

- **Software capability**: `crate::software::serial_protocol::SerialProtocol`
  (`Sbus`/`Crsf`/`Msp`/`Mavlink`) is plain embedded Rust, independent of any
  board or port. It only knows a protocol's own intrinsic HAL settings (baud
  rate, parity, stop bits) — no port, no decoding logic.
- **System hardware**: unchanged — a board's serial ports are constructed the
  same way regardless of routing (see
  [decisions.md](decisions.md#common-hardware-behavior-stays-out-of-board-declarations)).
- **Boot routing**: `build.rs` generates `crate::routing` — one function per
  named port (`serial1_protocol()`, ...) returning that system's default
  `Option<SerialProtocol>`. `app_body_skeleton.rs`'s `#[init]` calls these
  when assembling `Settings`, translating the routed protocol (if any) into
  the port's actual `SerialConfig`; an unrouted port gets a generic default.
  This is the boot-time association step, done in ordinary Rust — no reducer
  or cfg predicate involved.
- **Rejecting an impossible mapping**: `build.rs` validates every declared
  route names a pin the active board actually declares, and that no pin is
  routed twice, failing the build with a clear message otherwise (the
  equivalent of the table's "reject... an unavailable port... the same
  exclusive resource more than once," enforced at build time since there is
  no persistent runtime store yet).

**What this is not**: there is no SBUS/CRSF/MSP/MAVLink frame decoder — a
port gets the right baud rate and is tagged with its intended role, but
nothing reads or interprets its bytes yet. Nor is the routing user-editable
at runtime; `default_serial_routes` is compiled-in data, the generalized
version of the same idea as the Nucleo's existing boolean TIM3-to-report
route, not a persistent/configurable router.

## What remains unspecified

The storage format, configuration syntax, type-erasure strategy, and message
or buffer interfaces for a fully generalized, runtime-configurable router are
still deliberately unspecified. Defining that broader system, adding actual
protocol decoders, or extending the cfg reducer still requires explicit
permission.
