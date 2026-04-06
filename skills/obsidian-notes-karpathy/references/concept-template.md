# Concept Template

```markdown
---
title: "{Concept Title}"
concept_id: "{concept-slug}"
aliases:
  - "{common alias}"
updated_at: "{datetime}"
status: active | draft | conflicting
sources:
  - "[[wiki/summaries/source-a]]"
related:
  - "[[wiki/concepts/related-concept]]"
  - "[[wiki/entities/related-entity]]"
---

# {Concept Title}

## Definition

{one concise definition grounded in the current evidence}

## Why It Matters

{why this concept deserves its own page}

## Evidence

- [[wiki/summaries/source-a]] - {claim or support}
- [[wiki/summaries/source-b]] - {claim or contrast}

## Related Concepts

- [[wiki/concepts/related-concept]] - {relationship}

## Related Entities

- [[wiki/entities/related-entity]] - {relationship}

## Tensions and Contradictions

- {disagreement between sources, if any}

## Open Questions

- {what remains uncertain}
```

Rules:

- Merge evidence instead of duplicating concept pages.
- Use `aliases` aggressively so backlinks and unlinked mentions still work when terminology shifts.
- Use `related` for real relationships to concepts or entities, not speculative link spam.
- Set `status: conflicting` when unresolved disagreement remains.
