# Registering board peripherals: two approaches compared

Status: **resolved and implemented** — Approach A, kept below for the
reasoning. The full target architecture (including the two safeguards this
comparison's conclusion depends on) is implemented; see
`docs/roadmap/hardware-construction-redesign.md`.

## The question

Once board construction uses real `stm32f4xx_hal` types directly (see
`decisions.md`), `build.rs` still needs to know *which* peripherals/pins a
board enables, to emit the `board_peripheral`/`board_pin` cfg flags the RTIC
reducer uses. Two ways to get that fact into `build.rs`:

- **A — Declarative fact.** A thin, name-only list in `catalog` (e.g.
  `peripherals: BTreeSet<&'static str>`), read directly by `build.rs`.
- **B — Enforced construction.** Require peripheral construction to follow one
  standard, machine-checkable shape; have `build.rs` parse the real
  construction code (in `src/`) to derive the same facts.

## Comparison

| | A — Declarative fact | B — Enforced construction |
| --- | --- | --- |
| What `build.rs` reads | A bounded, fully-specified schema | Rust source, pattern-matched against a required shape |
| New syntax introduced | None | A mandated construction convention (the more rigidly enforced, the more it's a DSL in Rust's clothing — see `AGENTS.md`) |
| Scales to new peripheral kinds | Add a name to the list | Extend the parser/macro for each new construction shape |
| Where the fact and the code can disagree | Two places (catalog name vs. `#[cfg]` gate) | Two places (mandated shape vs. however someone actually wrote it) |

Neither approach removes the possibility of a mismatch. They differ in **how
loud the mismatch is**.

## Failure modes

### A — Declarative fact

| Mistake | Result | Caught by |
| --- | --- | --- |
| Declared in `catalog`, construction never written | Dead cfg flag, no effect | Nothing automatic — grep-able, harmless |
| Construction written and gated, never declared in `catalog` | Cfg always false → code and its callers compile out together | `unexpected_cfgs` lint (**warning** by default; **error** with `[lints.rust] unexpected_cfgs = "deny"`) |
| Typo in the cfg string on one side only | Same as above | Same lint, same fix |

With `deny` set, every mismatch in this category becomes a hard build
failure, not a silent omission. This is the current recommendation.

### B — Enforced construction

| Mistake | Result | Caught by |
| --- | --- | --- |
| Construction written in a shape the parser doesn't recognize (extra local variable, wrapped in a helper, spread syntax, ...) | Parser misses it silently *or* build.rs errors on "unrecognized shape" | Depends entirely on how defensively the parser is written — no equivalent to `unexpected_cfgs` exists for arbitrary code shapes |
| A peripheral kind that doesn't fit the mandated shape (e.g. one needing a genuinely different construction sequence) | Convention has to be extended, or bypassed | Whoever extends the parser next |
| Someone follows plain Rust idiom instead of the mandated convention | Silently unrecognized, same as the first row | Same as above |

The failure mode here isn't bounded the way A's is — "did the parser recognize
this particular way of writing normal Rust" isn't a closed question the way
"is this name in a known set" is.

## Recommendation

A, with `unexpected_cfgs = "deny"`. It gets the same hard-failure guarantee B
is reaching for, with a strictly smaller mechanism: no new syntax to define,
document, or keep a parser in sync with as peripheral kinds grow.
