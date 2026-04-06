# Health Rubric

Use this structure for `outputs/health/health-check-{date}.md`.

```markdown
---
title: "Health Check Report"
date: "{datetime}"
scope: "wiki/, outputs/qa/"
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

- Completeness: 25%
- Consistency: 25%
- Connectivity: 20%
- Freshness: 15%
- Provenance: 15%

## What to grade

### Completeness

- sparse concept pages
- sparse entity pages
- summaries missing concepts, evidence, or metadata
- important source clusters with no concept coverage
- high-frequency named entities or tools with no stable page when one is clearly warranted

### Consistency

- conflicting definitions
- alias drift
- duplicated concepts that should be merged
- duplicated entities that should be merged
- stale claims contradicted by newer summaries or Q&A

### Connectivity

- orphan summaries
- orphan concepts
- orphan entities
- orphan Q&A notes
- weakly linked clusters
- missing obvious reciprocal relations

### Freshness

- concepts not updated despite newer relevant material
- entities not updated despite newer relevant material
- old Q&A that should be refreshed or annotated

### Provenance

- claims with weak source backing
- concept pages missing source links
- entity pages missing source links
- archived answers without enough evidence trail
