# Health Checks

`kb-health` is the maintenance pass for the compiled wiki.

## Run it when

- the wiki feels disconnected
- old notes may have been superseded
- concept pages disagree
- you want a weekly or monthly maintenance baseline

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

- fix obvious cross-link issues
- flag contradictions for human review
- flag alias drift or duplicated concepts
- identify missing concept pages
- recommend new sources to add to `raw/`
