# Provenance and Alias Policy

Use this policy whenever compile, review, query, or health logic must decide whether two notes refer to the same underlying concept or whether a page still has trustworthy source grounding.

## Goals

- keep the review gate intact
- reduce duplicate concept and entity pages
- surface cross-language overlap without inventing silent merges
- make provenance drift visible before it pollutes the approved layer

## Provenance rules

- `source_hash` is the strongest compile-time indicator that a raw capture has changed.
- `source_mtime` is a fallback when a hash is unavailable.
- `last_verified_at` records the most recent time the workflow checked the raw source against the current draft or live representation.
- `possibly_outdated` is a hint that the source may need freshness review; it is not an automatic rejection.
- if a hash mismatch appears after promotion, the note should re-enter draft -> review rather than being silently rewritten in place.

## Alias rules

- every durable live concept or entity should prefer one `canonical_name` in lowercase kebab-case
- `aliases` should include cross-language names, common spelling variants, and stable terminology shifts
- compile may propose `alias_candidates`, but review decides whether they join the approved alias set
- if two approved pages share aliases or normalized identities, health should flag them as duplicate or merge candidates rather than auto-merging them

## Merge posture

- never silently merge conflicting approved pages
- merge candidates should preserve provenance edges and reviewer visibility
- cross-language overlap is a governance issue, not a reason to bypass review

## Query posture

- answers should prefer approved summaries and review-backed live pages for evidence trails
- aliases improve retrieval and backlink coverage, but they do not widen the truth boundary beyond `wiki/live/`
