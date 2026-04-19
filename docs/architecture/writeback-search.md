# Writeback and Search

The bundle treats good outputs as assets, but only through a review-gated loop.

## Writeback posture

Current direction:

- `kb-query` archives durable answers into `outputs/qa/`
- publish artifacts live under `outputs/content/`
- both of those surfaces belong to artifact archive, not approved truth
- high-value outputs can produce structured `writeback_candidates`, `source_live_pages`, and `followup_route`
- prior approved coverage and archived Q&A should be reused explicitly before a new publish artifact re-explains the same background
- writeback should re-enter the system through draft -> review -> live, not directly into live
- archived outputs can feed backlog and maintenance signals without becoming approved truth

## Why not direct mutation

Directly writing query outputs into `wiki/live/` would collapse the production/judgment split that the bundle depends on.

The same rule applies to governance indices: approved-layer views such as `QUESTIONS.md` should stay grounded in approved live pages instead of silently ingesting unresolved archived outputs.

## Search posture

The default search order stays:

1. `wiki/index.md`
2. `wiki/live/indices/*`
3. governance views such as `QUESTIONS.md`, `GAPS.md`, and `ALIASES.md`
4. relevant briefings
5. prior `outputs/qa/`
6. local structured or metadata-driven search only when needed
7. semantic retrieval only as candidate surfacing when earlier layers stop being sufficient

Approved live pages remain the truth source throughout that ladder.

Archive reuse order stays:

1. approved live pages
2. live indices and briefings
3. prior archived Q&A
4. prior archived publish artifacts when they already reuse approved coverage cleanly

## Planned / evolving search upgrades

- stronger alias maps
- richer derived indices such as `RECENT.md`
- clearer follow-up routing across query and health outputs
- optional local tools such as qmd or DuckDB

Hosted vector infrastructure is intentionally not the default path. The bundle prefers the cheapest retrieval layer that preserves traceability.
