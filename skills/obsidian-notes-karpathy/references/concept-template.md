# Concept Template

```markdown
---
title: "{Concept Title}"
concept_id: "{concept-slug}"
aliases:
  - "{common alias}"
updated_at: "{datetime}"
last_reviewed_at: "{datetime}"
status: active | draft | conflicting
sources:
  - "[[wiki/live/summaries/source-a]]"
related:
  - "[[wiki/live/concepts/related-concept]]"
  - "[[wiki/live/entities/related-entity]]"
---

# {Concept Title}

## Definition

{one concise definition grounded in the current evidence}

## Why It Matters

{why this concept deserves its own page}

## Established

- {current statement you are willing to reuse}
- {second stable statement grounded in the evidence}

## Inference

- {synthesis or framing added on top of the source material}

## Evidence

- [[wiki/live/summaries/source-a]] - {claim or support}
- [[wiki/live/summaries/source-b]] - {claim or contrast}

## Related Concepts

- [[wiki/live/concepts/related-concept]] - {relationship}

## Related Entities

- [[wiki/live/entities/related-entity]] - {relationship}

## Tensions and Contradictions

- {disagreement between sources, if any}

## Open Questions

- {what remains uncertain}
```

Rules:

- Merge evidence instead of duplicating concept pages.
- When used under `wiki/drafts/**`, point `sources` and `related` at draft pages; when promoted under `wiki/live/**`, point them at approved live pages.
- Use `aliases` aggressively so backlinks and unlinked mentions still work when terminology shifts.
- Use `related` for real relationships to concepts or entities, not speculative link spam.
- Set `status: conflicting` when unresolved disagreement remains.
