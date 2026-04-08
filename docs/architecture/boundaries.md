# Boundaries

This bundle works only if the surfaces stay separate.

## Shipped boundaries

- `raw/` is evidence intake and should not be mutated by later stages.
- `wiki/drafts/` is reviewable knowledge, not retrieval truth.
- `wiki/live/` is the approved long-term brain.
- `wiki/briefings/` is generated from live only.
- `outputs/reviews/` is the promotion ledger.

## Coordination vs knowledge

`MEMORY.md` is the coordination surface for:

- preferences
- collaboration rules
- editorial priorities
- current focus areas

It is not the place for source-grounded topic conclusions. Those belong in `wiki/drafts/` or `wiki/live/`.

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
