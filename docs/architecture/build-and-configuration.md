# Build and configuration

Status: current Cargo, catalog, linker, and fixture behavior.

## Selection model

Cargo features select the compiled target:

| Kind | Values |
| --- | --- |
| Chip | `f401`, `f405`, `f411` |
| System | `system-nucleo-f401re`, `system-default-f401`, `system-default-f405`, `system-default-f411` |
| Software prototype | `sw-report` |
| Regression fixture | `generated-body`, `invalid-interrupt` |

Exactly one chip must be active. A system automatically enables its chip. At most
one system may be active, and explicitly combining a system with another chip is
rejected.

## Chip, system, and board data

Chip, system, and board data is plain Rust in the `catalog` crate
(`catalog/src/{chips,systems,boards}.rs`), not TOML — deliberately, for
now; see [decisions.md](decisions.md#toml-deferred-for-chipsystemboard-data).
Both `build.rs` and the `rtic_app_cfg::for_chip` reducer depend on `catalog`
normally, so there is exactly one definition of this data, not two parses to
keep in sync.

- `catalog::Chip` is an enum (`F401`/`F405`/`F411`); `Chip::config()` returns
  its `ChipConfig` — `pac_feature`, `max_sysclk_hz`, and a `default_part`
  (`Part`: `probe_chip`, `flash_kib`, `ram_kib`, optional `ccm_kib`). This is
  mechanical build metadata, not a peripheral capability model — it must not
  grow into an exhaustive MPN database.
- `catalog::system_config(feature)` returns a `SystemConfig` — firmware
  `name`, `part`, `embed_profile`, an optional `board` (`BoardConfig`), and an
  optional `clock` selection. A system with a board takes its chip from the
  board rather than declaring its own — the board is the single source of
  truth for physical facts.
- `catalog::BoardConfig` (currently only `boards::nucleo_f401re::config()`) declares
  enabled peripheral instances, their pin/DMA mappings, and HSE availability.
  It does not map hardware ports to software capabilities — that association
  is chosen and validated at boot.

`build.rs` verifies the selected system's chip against the active Cargo
feature, its exact part, and its Embed probe profile. The NUCLEO-F401RE
prototype additionally has real peripheral/pin/DMA data; see [the prototype
specification](board-toml-prototype.md) for its scope and limits — the
authorization and DSL-permission boundary described there applies to this
Rust data exactly as it did to the TOML it replaced.

## Linker generation

`build.rs` selects the exact part and writes `memory.x` into Cargo's `OUT_DIR`.
FLASH and ordinary RAM are emitted as separate regions. The F405 part additionally
receives its non-contiguous CCM region at `0x10000000`.

## Clock generation

`build.rs` writes `OUT_DIR/clock_config.rs`, consumed by `src/clock.rs`,
generating a real `stm32f4xx_hal::rcc::Config` from the active system's clock
selection — validated against the chip's `max_sysclk_hz` — or bare
`Config::hsi()` when none is declared. See [initialization and monotonic
time](initialization-and-time.md#clock-configuration).

## Normal and fixture builds

Normal builds compile `src/app_body_skeleton.rs` through the source-preserving
attribute macro. No RTIC task source file is generated for this path.

Fixture builds use the historical text composer and chip `app_head.rs` files.
They write a generated application into `OUT_DIR` and a stable inspection copy
under `target/generated-preview/`.

## Developer commands

`.cargo/config.toml` provides chip and system aliases such as:

```powershell
cargo check-f401
cargo build-default-f405
cargo embed-nucleo-f401re
```

Cargo Embed profile names are configured separately in `Embed.toml`; selecting a
profile does not itself select the corresponding Cargo feature. `Embed.toml` is
the one file in this pipeline that stays TOML regardless — it's `cargo-embed`'s
own external config format, not ours to redesign.

## Validation boundary

Current build validation covers selection compatibility, exact-part lookup,
Embed probe matching, and clock target vs. the chip's maximum sysclk. For the
Nucleo prototype, peripheral, pin, and DMA selections also become generated
PAC/HAL types and trait assertions. Invalid mappings therefore fail normal
compilation without an exact-MPN database.

Cross-entry resource conflicts, interrupt derivation, and PLL-divider
feasibility (whether a requested clock target is exactly reachable from its
source, as opposed to merely under the chip's ceiling) are not implemented. It
remains acceptable for part-specific mistakes outside the coarse family model
to reach a compiler, HAL, or PAC error.
