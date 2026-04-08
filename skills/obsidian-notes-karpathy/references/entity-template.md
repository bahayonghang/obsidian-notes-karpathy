# Entity Template

Use dedicated entity pages only when the named thing is durable enough to deserve one page of its own.

Good candidates:

- people
- organizations
- products
- tools
- projects
- repositories

```markdown
---
title: "{Entity Title}"
entity_id: "{entity-slug}"
entity_type: person | organization | product | tool | project | repo | other
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

# {Entity Title}

## Who or What It Is

{one concise definition grounded in the evidence}

## Why It Matters Here

{why this entity deserves a stable page in this vault}

## Established

- {stable fact grounded in the approved evidence}
- {second stable fact that should remain reusable}

## Inference

- {synthesis or interpretation added on top of the evidence}

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

- Prefer entity pages only for named things that recur, anchor multiple notes, or clearly deserve stable provenance.
- When used under `wiki/drafts/**`, point `sources` and `related` at draft pages; when promoted under `wiki/live/**`, point them at approved live pages.
- If the evidence is thin, keep the item in summaries or concept aliases rather than creating a weak entity page.
- Set `status: conflicting` when unresolved disagreement remains.
