# Documentation routing instructions

These instructions apply to the `docs/` tree. The repository-wide instructions
in `../AGENTS.md` also apply, including the requirement to obtain explicit user
permission before implementing or extending any DSL or DSL-like mechanism.

## Start here

Read `README.md` first. Then load only the category and document needed for the
task; do not read every design report by default.

## Routing table

| Question or task | Read first | Read only if needed |
| --- | --- | --- |
| Overall architecture | `architecture/overview.md` | `architecture/decisions.md` |
| RTIC tasks, resources, cfg, or the reducer | `architecture/rtic-composition.md` | `history/compile-check-spike-report.md` |
| Chips, parts, boards, or systems | `architecture/hardware-and-systems.md` | `architecture/build-and-configuration.md` |
| Hardware ports, software capabilities, or boot routing | `architecture/hardware-software-routing.md` | `architecture/hardware-and-systems.md` |
| The Nucleo `board.toml` prototype, pins, or DMA mappings | `architecture/board-toml-prototype.md` | `architecture/build-and-configuration.md` |
| Startup, clocks, RCC, hardware construction, or monotonic time | `architecture/initialization-and-time.md` | `architecture/hardware-and-systems.md` |
| Cargo features, TOML, linker memory, Embed, or generated fixtures | `architecture/build-and-configuration.md` | `verification/README.md` |
| IDE or rust-analyzer behavior | `development/ide-workflow.md` | `architecture/rtic-composition.md` |
| Test evidence or reproduction | `verification/README.md` | The specific linked plan or report |
| Planned capability/configuration work | `roadmap/next-stage-plan.md` | Relevant current architecture category |
| Why an old decision was made | `architecture/decisions.md` | The specific document under `history/` |

## Authority and update rules

- Files under `architecture/` describe the current adopted design.
- Files under `roadmap/` are proposals, not implemented behavior.
- Files under `verification/` are evidence snapshots and may name older layouts.
- Files under `history/` preserve rationale and rejected or superseded designs;
  they are not authoritative for the current implementation.
- When behavior changes, update the smallest relevant current architecture file
  and its entry in `architecture/decisions.md`.
- Keep current facts out of historical reports. Add a short correction or link
  instead of rewriting the original evidence.
- Prefer links over copying large sections between categories.
- The project's purpose/identity statement (what this repository is for, and
  why) has exactly one authoritative copy: the "Purpose" section of
  `architecture/overview.md`. Every other file (`README.md`, `AGENTS.md`,
  category docs) must link to it rather than restate it in independent prose.
  This rule exists because that duplication already drifted silently once —
  `overview.md`'s purpose text was genericized away from `README.md`'s while
  nobody was checking both — so treat a second independent restatement of the
  purpose anywhere as a bug to fix, not a stylistic choice.
- Capability discussions must follow the repository rule: use chip-family-level
  knowledge, do not introduce an exact-MPN capability database, and accept
  compiler errors for device-specific edge cases.
- Never model a software function as permanently belonging to a physical port.
  Keep both sides independently selectable and connect them through boot-time
  routing.
- The user authorized only the NUCLEO-F401RE board-DSL prototype documented in
  `architecture/board-toml-prototype.md`. Extending its schema or scope requires
  new explicit permission — whether it's currently expressed as Rust data
  (`catalog`, see `architecture/decisions.md`) or TOML.
