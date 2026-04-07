# Health Rubric

Use this structure for `outputs/health/health-check-{date}.md`.

```markdown
---
title: "Health Check Report"
date: "{datetime}"
scope: "wiki/live/, wiki/briefings/, outputs/qa/, outputs/reviews/"
health_score: {overall}
---

# Knowledge Base Health Check

## Health Score: {overall}/100

| Dimension | Score | Notes |
|-----------|-------|-------|
| Completeness | {score} | {notes} |
| Consistency | {score} | {notes} |
| Connectivity | {score} | {notes} |
| Freshness | {score} | {notes} |
| Provenance | {score} | {notes} |

## Critical Issues

- {issue}

## Warnings

- {warning}

## Review Backlog

- {pending draft or human review queue item}

## Briefing Staleness

- {stale briefing}

## Suggested Follow-Up Questions

- {source gap or ambiguity}

## Search Upgrade Recommendation

- Current stage: {1 | 2 | 3 | 4}
- Why: {why this stage fits the current vault}
- Next tool or pattern: {Backlinks | Properties | qmd | DuckDB | Dataview | vector retrieval}

## Fix Now

- {safe automatic fix}

## Propose Fix

- {fix that is likely correct but should be reviewed}

## Human Review Needed

- {judgment call}
```

Suggested weighting:

- Completeness: 20%
- Consistency: 25%
- Connectivity: 15%
- Freshness: 15%
- Provenance: 25%

## What to grade

### Completeness

- live concept or entity gaps relative to approved summaries
- missing briefings for active roles
- review outputs that never generated the corresponding live page or briefing

### Consistency

- conflicting approved concepts or entities
- duplicate approved concepts or entities
- approved claims contradicted by newer approved sources

### Connectivity

- orphan live summaries, concepts, entities, or Q&A
- weak cross-linking between live pages and briefings
- missing `approved_from` / `review_record` provenance edges

### Freshness

- stale briefings
- approved pages that should have been rebuilt after new reviews
- old Q&A that should be refreshed against newer approved knowledge

### Provenance

- live pages missing `review_record`
- live pages missing `trust_level: approved`
- answers or briefings that cite drafts or raw captures as truth

## Mechanical integrity

Treat these as report-worthy mechanical issues even though they do not add a new scored dimension:

- alias-style wikilinks inside Markdown table cells
- unapproved pages under `wiki/live/`
- pending review backlog that is aging without resolution
