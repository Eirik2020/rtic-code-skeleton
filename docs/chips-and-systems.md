# Chips and systems

A **chip** describes the MCU family supported by the HAL/PAC and its available
parts. A **system** selects a chip and an exact part, and owns board wiring,
clock setup, and peripheral roles.

ST NUCLEO-F401RE (the board referred to as NUCLEO-F401RET) is a system selecting
chip `f401` and part `STM32F401RETx`. ST's
[board information](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
identifies it as a Nucleo-64 board with an STM32F401RE MCU.

| System directory | Feature | Chip | Part | Board setup |
| --- | --- | --- | --- | --- |
| `nucleo_f401re` | `system-nucleo-f401re` | f401 | STM32F401RETx | LD2 on PA5, TIM3 blink |
| `default_f401` | `system-default-f401` | f401 | STM32F401RETx | None |
| `default_f405` | `system-default-f405` | f405 | STM32F405RGTx | None |
| `default_f411` | `system-default-f411` | f411 | STM32F411CEUx | None |

Default systems are minimal configurations using the generic skeleton. They do
not assume an LED pin, external peripherals or board clock setup. They are
useful starting points for registering actual hardware. They are explicitly
selected features, not Cargo defaults or fallback systems.

## Ownership

| Owner | Location | Contents |
| --- | --- | --- |
| Chip support | `src/chips/{f401,f405,f411}/config.toml` | Chip/HAL feature, exact-part catalog, memory sizes and probe identifiers |
| System registration | `src/systems/catalog.rs` | Shared feature-to-chip mapping for build.rs and the RTIC reducer |
| System configuration | `src/systems/<system>/config.toml` | Name, chip, selected part, Cargo feature and Embed profile |
| Board wiring and setup | `src/systems/nucleo_f401re/mod.rs` | LD2 GPIO, clocks, TIM3 timer and interrupt handling |
| Application graph | `src/app_body_skeleton.rs` | Shared resources/tasks plus system-selected init, resources and IRQ bindings |
| Flashing | `Embed.toml` and `.cargo/config.toml` | System probe profiles and matching aliases |

The chip/part TOML catalogs are read by `build.rs`. For a selected system, the
build validates its chip, part, feature and probe-profile mapping before writing
`memory.x`. Chip-only builds use the catalog's explicit `default_part` for
linking. A smaller or otherwise different MCU part needs its own catalog entry.

## Selecting firmware

```powershell
cargo build-nucleo-f401re
cargo embed-nucleo-f401re

cargo build-default-f401
cargo build-default-f405
cargo build-default-f411

cargo embed-default-f401
cargo embed-default-f405
cargo embed-default-f411
```

Each build alias enables its `system-...` feature. Each embed alias also picks
the matching Embed profile. Add `--release` for optimized firmware. For example,
`cargo build-nucleo-f401re` is `cargo build --features system-nucleo-f401re`.

Use the same feature in `rust-analyzer.cargo.features`. At most one system may
be selected, even when two systems share a chip. A system automatically enables
its chip. An explicit extra chip feature must match; incompatible chips fail.

Chip-only `build-f401` / `build-f405` / `build-f411` remain available, without a
system. The old chip-only `embed-f401` command no longer initializes the Nucleo
LED; use `embed-nucleo-f401re` for that firmware.

The Nucleo system reserves TIM3 for blinking and uses TIM2 as its RTIC dispatcher.
Default systems and chip-only skeletons use TIM2 and TIM3 as dispatchers. The
F405-only UART4 task remains chip-gated: peripheral availability belongs to the
chip, while a physical board's use of that peripheral belongs to the system.

The skeleton has one shared `#[init]`. Conditional fields in its `Shared` and
`Local` initializers select chip resources and system hardware. The reducer
removes inactive fields and their expressions before RTIC expansion, so Nucleo
hardware initialization is absent from other configurations.

## Adding another system

1. Add `src/systems/<name>/config.toml`. Choose a part from a chip catalog,
   adding a verified part entry if necessary. Add a Rust module for physical
   wiring/setup if the system needs one.
2. Add a `system-...` Cargo feature enabling the required chip. Register its
   feature/chip pair in `src/systems/catalog.rs`. Its directory name is the
   feature suffix with hyphens replaced by underscores.
3. Add an exclusive selector at the top of the skeleton. Pass both the chip
   and system to `for_chip`; keep the chip-only fallback mutually exclusive.
   Expose any hardware module through its feature in `src/systems/mod.rs`.
4. Add cfg-gated system resources, initialization and tasks as needed. Allocate
   dispatchers so none collide with the system's hardware tasks.
5. Add a matching Embed profile, aliases, and positive/negative tests.

The reducer receives the selected system explicitly, preserving deterministic
rust-analyzer expansion. Its supported system predicates come from the same
catalog as the build script. Other cfg axes still need explicit implementation.

System features cannot be combined with `generated-body`: that is a synthetic
legacy fixture. `invalid-interrupt` remains a negative test. The separate chip
`app_head.rs`/`app_body.rs` files belong to those legacy fixtures; normal
development uses the shared source skeleton.

## Memory layout and verification

While moving memory sizes into the part catalogs, the F405 layout was corrected:
its 192 KiB total was previously represented as contiguous RAM at `0x20000000`.
It has 128 KiB there and separate 64 KiB CCM at `0x10000000`; see ST's
[datasheet](https://www.st.com/resource/en/datasheet/stm32f405rg.pdf). The linker
now receives separate `RAM` and `CCM` regions. Ordinary data and stack use `RAM`;
placing data in CCM would require explicit linker sections.

The compile matrix covers all default systems, Nucleo, matching/mismatching
chips, two systems sharing F401, systems using different chips, and legacy
negative fixtures. Reducer tests verify that defaults omit Nucleo hardware,
that only F405 retains UART4, and that each selection retains exactly one init.

The Nucleo's PA5 wiring and 10 Hz timer-update setup are preserved. No firmware
was flashed as part of this refactor.

Verified after the split: all four system configurations link ARM ELF files;
their generated linker layouts match the selected parts (including the separate
F405 CCM region). All 30 compile-matrix cases and six reducer unit tests pass.
Real rust-analyzer LSP checks pass for each of the four systems: resource
completion, hover, navigation and unsaved field-type updates. The previously
documented limitation in native unsaved task diagnostics remains.
