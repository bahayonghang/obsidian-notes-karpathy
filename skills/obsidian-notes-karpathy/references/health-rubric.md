# Health Rubric

Use this structure for `outputs/health/health-check-{date}.md`.

```markdown
---
title: "Health Check Report"
date: "{datetime}"
scope: "wiki/live/, wiki/briefings/, outputs/qa/, outputs/content/, outputs/reviews/"
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

## Duplicate and Alias Drift

- {duplicate concept/entity or alias split}

## Creator Consistency

- {editorial drift across CLAUDE.md, MEMORY.md, style guides, or briefings}
- {profile conflict between account guidance surfaces}

## Review Backlog

- {pending draft or human review queue item}

## Briefing Staleness

- {stale briefing}

## Writeback Backlog

- {qa or content artifact with pending writeback candidates}

## Reuse Signals

- {reuse gap in archived creator outputs}
- {underused approved source that is not showing up in reusable outputs}

## Open Questions and Gaps

- {source gap or ambiguity}

## Growth Opportunities

### Compile Next

- {capture or question cluster that should re-enter draft creation}

### Review Next

- {draft, promotion boundary, or disputed interpretation needing review}

### Hub Build Next

- {topic / program / question cluster that now deserves a curated hub}

### Source Acquisition Next

- {missing source that would reduce uncertainty or fill a recurring gap}

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
- open questions or writeback candidates that recur without an owner
- repeated archived outputs that imply a missing synthesis or curated hub
- procedural candidates that never graduated into `wiki/drafts/procedures/` or `wiki/live/procedures/`
- episodic notes that never re-entered the semantic/procedural lanes despite durable signals

### Consistency

- conflicting approved concepts or entities
- duplicate approved concepts or entities
- approved claims contradicted by newer approved sources
- aliases split across multiple live notes that should converge
- supersession chains that are missing reciprocal links, timestamps, or reasons

### Connectivity

- orphan live summaries, concepts, entities, or Q&A
- weak cross-linking between live pages and briefings
- missing `approved_from` / `review_record` provenance edges
- weakly connected live pages that should likely participate in a synthesis, relationship edge, or curated hub
- graph-signaled pages with `relationship_notes` or graph requirements but no durable `related` edges

### Freshness

- stale briefings
- approved pages that should have been rebuilt after new reviews
- old Q&A that should be refreshed against newer approved knowledge
- pending writeback candidates that have not re-entered the draft/review loop
- pages whose `domain_volatility` suggests they should be reviewed sooner
- repeated question clusters that remain unresolved without visible progress
- confidence decay windows that have passed without a fresh source check or review

### Provenance

- live pages missing `review_record`
- live pages missing `trust_level: approved`
- answers or briefings that cite drafts or raw captures as truth
- approved pages with empty or missing `sources`
- source hash drift or outdated verification metadata
- pages using the latest lifecycle metadata missing core confidence metadata (`confidence_score`, `confidence_band`, `support_count`, `contradiction_count`)
- missing machine-readable audit events for automation surfaces that claim to be governed

## Creator consistency checks

Treat these as first-class maintenance diagnostics when the vault has creator-facing outputs:

- `CLAUDE.md`, `MEMORY.md`, account `_style-guide.md` files, and account-facing briefings drifting apart
- forbidden terms, tone rules, or publishing constraints present in one creator surface but missing from another
- creator/account profile collisions where two guidance surfaces disagree on voice, positioning, or audience
- archived creator outputs that fail to record reuse of prior approved coverage
- approved sources that have not been reused in archived Q&A or publish outputs despite remaining central to the topic area

## Mechanical integrity

Treat these as report-worthy mechanical issues even though they do not add a new scored dimension:

- alias-style wikilinks inside Markdown table cells
- unapproved pages under `wiki/live/`
- pending review backlog that is aging without resolution
- collaboration memory mixing with approved knowledge surfaces
- broken wikilinks in approved surfaces
- curated hubs that are stale, orphaned, or badly imbalanced toward a small portion of the approved graph
- missing or empty `outputs/audit/operations.jsonl` when audit scaffolding exists
