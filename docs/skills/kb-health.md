# kb-health

Run a deep, report-oriented health check on the compiled knowledge base.

## Audit scope

The health pass focuses on:

- `wiki/`
- `outputs/qa/`
- `outputs/health/`

It references `raw/` only when provenance or freshness requires a spot check.

When available, it should start with `scripts/lint_obsidian_mechanics.py` so that deterministic mechanical issues are separated from judgment-heavy audit findings.

## What it checks

- contradictory definitions or superseded claims
- stale Q&A and stale concept or entity pages
- alias drift and duplicate concepts or entities
- sparse pages and missing obvious topics
- orphan pages and weak cross-linking
- missing local assets
- weak provenance
- malformed tables or wikilinks that render badly in Obsidian
- whether the vault now needs a search upgrade

## Report output

The health report is written to `outputs/health/health-check-{date}.md`.

It should include:

- an overall score out of 100
- sub-scores for Completeness, Consistency, Connectivity, Freshness, and Provenance
- critical issues
- warnings
- suggested follow-up questions or source gaps
- a search upgrade recommendation
- clear separation between safe fixes and human-judgment items

## Relationship to kb-compile

`kb-compile` can repair obvious mechanical issues immediately after ingestion.

`kb-health` owns the deeper maintenance pass, including drift, contradictions, provenance gaps, and long-horizon structure problems.
