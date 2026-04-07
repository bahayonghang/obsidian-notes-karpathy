# Briefing Template

Use this structure for `wiki/briefings/{role}.md`.

```markdown
---
title: "{Role} Briefing"
brief_for: "{role}"
built_from: "wiki/live/"
updated_at: "{datetime}"
staleness_after: "{datetime}"
source_live_pages:
  - "[[wiki/live/concepts/concept-a]]"
  - "[[wiki/live/summaries/human/articles/source-b]]"
---

# {Role} Briefing

## Mission Context

- {what this role should keep in working memory}

## Approved Facts to Reuse

- [[wiki/live/concepts/concept-a]] - {why it matters}
- [[wiki/live/summaries/human/articles/source-b]] - {why it matters}

## Open Questions

- {what still needs new evidence}

## Guardrails

- Never treat drafts as approved truth.
- Never cite raw captures unless a human explicitly asks for source evidence.
```

Rules:

- briefings must be generated only from live pages
- briefings should be short, role-specific, and easy to regenerate
