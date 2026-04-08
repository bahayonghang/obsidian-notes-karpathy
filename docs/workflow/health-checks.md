# Health Checks

`kb-health` is the report-first maintenance pass for the approved live brain and its downstream review ecosystem.

## Run it when

- the approved layer feels disconnected
- old approved notes may have been superseded
- concept pages disagree
- you want a weekly or monthly maintenance baseline
- archived Q&A or publish artifacts may have writeback backlog
- you need to check whether collaboration memory is leaking into approved knowledge

If the immediate job is to approve pending drafts or rebuild stale briefings, use `kb-review` first. `kb-health` is for broader maintenance across approved surfaces.

## Report destination

Health reports live in:

`outputs/health/health-check-{date}.md`

## Scoring dimensions

- Completeness
- Consistency
- Connectivity
- Freshness
- Provenance

## Typical outcomes

- fix obvious cross-link issues in approved surfaces
- flag contradictions for human review
- flag alias drift or duplicated concepts
- identify missing concept pages
- identify writeback backlog
- identify collaboration memory drifting into approved knowledge
- recommend new sources to add to `raw/`
