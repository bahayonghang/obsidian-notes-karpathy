---
name: kb-health
description: Run a deep health check on the approved V2 knowledge base. Use this skill whenever the user says "kb health", "health check", "lint live wiki", "find contradictions", "find stale briefings", "review backlog", "批准层体检", "知识库体检", or wants a maintenance pass over live pages, briefings, Q&A, and review outputs.
---

# KB Health

Deep lint and maintenance workflow for the approved live brain and its review ecosystem.

## Scope

Inspect primarily:

- `wiki/live/`
- `wiki/briefings/`
- `outputs/qa/`
- `outputs/reviews/`
- `outputs/health/`

Reference `raw/` only when provenance requires a spot check.

## What it checks

- conflicting approved concepts or entities
- stale Q&A relative to newer live pages
- stale briefings
- unapproved pages that somehow landed in `wiki/live/`
- review backlog piling up in drafts
- broken render mechanics in live pages
- weak provenance from live pages back to review records

## Report output

Write the report to `outputs/health/health-check-{date}.md` using the shared rubric.

## Relationship to kb-review

`kb-review` handles the immediate gate for pending drafts and briefing rebuilds.

`kb-health` handles longer-horizon maintenance: drift, integrity, backlog pressure, and search posture.
