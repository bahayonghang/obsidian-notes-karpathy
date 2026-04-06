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
status: active | draft | conflicting
sources:
  - "[[wiki/summaries/source-a]]"
related:
  - "[[wiki/concepts/related-concept]]"
  - "[[wiki/entities/related-entity]]"
---

# {Entity Title}

## Who or What It Is

{one concise definition grounded in the evidence}

## Why It Matters Here

{why this entity deserves a stable page in this vault}

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

- Prefer entity pages only for named things that recur, anchor multiple notes, or clearly deserve stable provenance.
- If the evidence is thin, keep the item in summaries or concept aliases rather than creating a weak entity page.
- Set `status: conflicting` when unresolved disagreement remains.
