# Documentation index

This directory is the canonical entry point for repository documentation. Read
the smallest category that answers the question; historical reports are kept for
rationale, not as descriptions of the current implementation.

| Category | Use it for | Start here |
| --- | --- | --- |
| `architecture/` | Current design, ownership, initialization, and build behavior | [Architecture overview](architecture/overview.md) |
| `development/` | Editor and contributor workflows | [Development documentation](development/README.md) |
| `verification/` | Test plans, captured reports, and reproduction evidence | [Verification documentation](verification/README.md) |
| `roadmap/` | Proposed future work that is not necessarily implemented | [Roadmap documentation](roadmap/README.md) |
| `history/` | Superseded plans and experimental rationale | [Historical design documents](history/README.md) |

## Source-of-truth order

When documents disagree, use this order:

1. Current implementation and tests.
2. Documents under `architecture/`.
3. Development guidance under `development/`.
4. Evidence snapshots under `verification/`.
5. Proposals under `roadmap/`.
6. Preserved material under `history/`.

Repository agents should also follow [the documentation routing
instructions](AGENTS.md), which point each kind of task to a focused document.
