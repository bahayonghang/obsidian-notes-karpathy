# Boundaries

This bundle works only if the surfaces stay separate.

## Shipped boundaries

- `raw/` is evidence intake and should not be mutated by later stages.
- `wiki/drafts/` is reviewable knowledge, not retrieval truth.
- `wiki/live/` is the approved long-term brain.
- `wiki/live/procedures/` is approved procedural memory.
- `wiki/briefings/` is generated from live only.
- `outputs/reviews/` is the promotion ledger.
- `outputs/episodes/` is episodic memory, not approved semantic truth.
- `outputs/audit/operations.jsonl` is the machine-readable audit surface.
- governance indices under `wiki/live/indices/` belong to the approved layer and must stay grounded in approved live pages.

## Coordination vs knowledge

`MEMORY.md` is the coordination surface for:

- preferences
- collaboration rules
- editorial priorities
- current focus areas

It is not the place for source-grounded topic conclusions. Those belong in `wiki/drafts/` or `wiki/live/`.

## Archived outputs vs approved truth

`outputs/qa/` and `outputs/content/` can store durable work products, but they do not become approved truth automatically.

They may:

- provide maintenance signals
- surface `writeback_candidates`
- record `followup_route`
- carry `crystallized_from_episode`
- inform health and backlog views

But long-term knowledge still re-enters through draft -> review -> live.

## Why this matters

Without clear boundaries, three failure modes compound quickly:

- task state and topic knowledge get mixed together
- query-time synthesis pulls in the wrong kind of context
- health checks cannot tell drift from normal collaboration residue

## Planned / evolving checks

The health pass is moving toward explicit checks for:

- collaboration memory leaking into approved knowledge
- approved knowledge leaking into coordination memory
- writeback backlog piling up without review
- archived outputs being mistaken for approved live governance truth
