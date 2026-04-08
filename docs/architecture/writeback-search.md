# Writeback and Search

The bundle treats good outputs as assets, but only through a review-gated loop.

## Writeback posture

Current direction:

- `kb-query` archives durable answers into `outputs/qa/`
- publish artifacts live under `outputs/content/`
- high-value outputs can produce structured writeback candidates
- writeback should re-enter the system through draft -> review -> live, not directly into live

## Why not direct mutation

Directly writing query outputs into `wiki/live/` would collapse the production/judgment split that the bundle depends on.

## Search posture

The default search order stays:

1. `wiki/index.md`
2. `wiki/live/indices/*`
3. backlinks, aliases, and metadata affordances
4. local structured search only when needed

## Planned / evolving search upgrades

- stronger alias maps
- richer derived indices such as `RECENT.md`
- optional local tools such as qmd or DuckDB

Hosted vector infrastructure is intentionally not the default path. The bundle prefers the cheapest retrieval layer that preserves traceability.
