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
fact_inference_separation: 0.90
promotion_reason: "{why this should or should not persist in the long-term brain}"
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
- Promotion reason: {why this deserves durable retention, or why it does not}

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
- include whether fact and inference are cleanly separated enough for safe reuse
- include enough context to reproduce the decision later
