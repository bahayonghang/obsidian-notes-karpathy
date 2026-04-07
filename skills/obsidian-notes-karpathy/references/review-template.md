# Review Template

Use this structure for `outputs/reviews/*.md`.

```markdown
---
title: "Review Record: {Source Title}"
decision: approve | reject | needs-human
accuracy: 0.91
provenance: 0.95
conflict_risk: 0.18
composability: 0.89
reviewed_at: "{datetime}"
---

# Review Record: {Source Title}

## Inputs Reviewed

- Draft: [[wiki/drafts/summaries/{file}]]
- Raw captures:
  - [[raw/human/articles/{file}]]
- Overlapping live pages:
  - [[wiki/live/concepts/{file}]]

## Decision

- Decision: {approve | reject | needs-human}
- Why: {short reason}

## Blocking Flags

- {flag or "none"}

## Promotion Result

- Promoted live pages:
  - [[wiki/live/summaries/{file}]]
- Rebuilt briefings:
  - [[wiki/briefings/{role}]]
```

Rules:

- reviewers judge the draft package, not the generating agent
- include enough context to reproduce the decision later
