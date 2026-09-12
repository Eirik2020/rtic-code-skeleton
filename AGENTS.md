# Repository instructions

See [Purpose](docs/architecture/overview.md#purpose) for what this repository
is actually for before reading the rules below — do not restate it here or
anywhere else; link to it instead, so it can't drift out of sync the way it
already has once. That purpose is why RC input, MSP, GPS, telemetry, and
similar concepts appear below and throughout `docs/`: they're the literal
target domain, not illustrative examples, and the rules below (especially
hardware/software separation and the chip-family-level capability boundary)
are scoped to what real flight controllers need, not a hypothetical
general-purpose framework.

- Do not design, add, extend, or modify any domain-specific language (DSL) or
  DSL-like mechanism without the user's explicit permission.
- Do not infer that permission from a broader feature, refactor, configuration,
  code-generation, macro, or tooling request. Ask first.
- This requirement includes custom configuration predicates, mini-languages,
  parser-defined syntaxes, and schemas that encode executable program behavior.
- Do not build or propose an exact-MCU-MPN capability database. Capability
  checks are intentionally chip-family-level and should catch common mistakes,
  not exhaustively model every device variant. Part-specific incompatibilities
  may be left for the Rust compiler, HAL, or PAC to reject.
- Existing exact-part identifiers used for linker memory and probe selection are
  mechanical build metadata. Do not treat them as capability authorities or
  expand them into per-MPN peripheral records.
- Keep physical hardware ports separate from software capabilities. A system
  selects and wires hardware such as UART instances; it must not hardwire a
  software role such as RC input, MSP, GPS, or telemetry to one instance.
- Hardware-port-to-software-role assignments are selected and validated at boot
  so they remain configurable. Compile-time RTIC interrupt bindings and resource
  ownership may stay static underneath that routing layer.
- The target architecture gives every system a physical hardware manifest
  (`board.toml`, deliberately deferred in favor of typed Rust data in
  `catalog` for now — see `docs/architecture/decisions.md`). It declares
  which peripheral instances the board enables and their pin and DMA mappings.
  It must not assign software roles to those peripherals. This rule binds
  whichever form currently expresses the manifest, Rust or TOML.
- Explicitly authorized prototype exception: the NUCLEO-F401RE board manifest,
  generated HAL/PAC type checks, board cfg predicates, and `sw-report` reducer
  selection may be implemented and reviewed. Do not generalize this DSL to
  more systems, fields, modes, or software features without fresh permission —
  regardless of whether the manifest is currently Rust or TOML.
- Prefer typed Rust (enums, structs, newtypes) over bare strings wherever
  practical. Reach for a string only when the consumer genuinely requires
  one — e.g. `#[cfg(...)]` predicates are string-keyed by design and stay
  that way no matter how the declaring side is typed.
- Existing code and established conventions, including anything that predates
  this file, are open to question — legacy shape is not settled just because
  it's already there. Flag it and propose occasional rewrites to streamline
  code that's grown inconsistent or unnecessarily stringly-typed, rather than
  preserving it by default; propose non-trivial rewrites before making them,
  consistent with the DSL-permission rule above.
- Format chat responses that explain architecture, tradeoffs, or anything
  with more than one part — headers, bullets, or code blocks over unbroken
  prose paragraphs. Apply this by default on every response, not only when
  asked.
