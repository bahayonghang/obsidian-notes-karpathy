# kb-health

Run a deep, report-oriented health check on the compiled knowledge base.

## Checks

- contradictory definitions
- stale claims and stale Q&A
- alias drift and duplicated concepts
- sparse pages
- orphan pages
- weak cross-linking
- broken local asset references
- weak provenance

## Output

Writes a scored report to `outputs/health/health-check-{date}.md`.

The report includes:

- overall score
- sub-scores for Completeness, Consistency, Connectivity, Freshness, and Provenance
- critical issues
- warnings
- suggested follow-up questions and source gaps

## Relationship to kb-compile

`kb-compile` can repair obvious mechanical issues after ingestion.

`kb-health` is the dedicated maintenance pass when the user wants a deeper diagnosis.
