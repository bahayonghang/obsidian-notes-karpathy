# Health Checks

`kb-review` maintenance mode is the report-first maintenance pass for the approved live brain and its downstream review ecosystem.

## Run it when

- the approved layer feels disconnected
- old approved notes may have been superseded
- concept pages disagree
- you want a weekly or monthly maintenance baseline
- archived Q&A or publish artifacts may have writeback backlog
- governance views such as `QUESTIONS.md`, `GAPS.md`, or `ALIASES.md` need a refresh
- you need to check whether collaboration memory is leaking into approved knowledge

If the immediate job is to approve pending drafts or rebuild stale briefings, use `kb-review` in `gate` mode. The same public skill switches to `maintenance` mode for broader upkeep across approved surfaces.

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
- flag stale archived Q&A or publish outputs
- identify missing concept pages
- identify writeback backlog
- identify collaboration memory drifting into approved knowledge
- refresh governance views when the target is deterministic
- recommend new sources to add to `raw/`
