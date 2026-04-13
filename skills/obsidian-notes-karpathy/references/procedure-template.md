# Procedure Template

Use this structure for `wiki/drafts/procedures/**` and `wiki/live/procedures/**`.

```markdown
---
title: "{Procedure Title}"
procedure_id: "{procedure-slug}"
visibility_scope: private | shared
confidence_score: 0.81
confidence_band: strong | moderate | weak
support_count: 2
contradiction_count: 0
updated_at: "{datetime}"
last_reviewed_at: "{datetime}"
last_confirmed_at: "{datetime}"
next_review_due_at: "{datetime}"
decay_class: workflow
status: active | draft | conflicting
approved_at: "{datetime}"
approved_from:
  - "[[wiki/drafts/procedures/{procedure-slug}]]"
review_record: "[[outputs/reviews/procedures--{procedure-slug}]]"
trust_level: approved
sources:
  - "[[wiki/live/summaries/source-a]]"
related:
  - "[[wiki/live/concepts/related-concept]]"
---

# {Procedure Title}

## Trigger

{When this workflow should be used}

## Steps

1. {step one}
2. {step two}
3. {step three}

## Failure Modes

- {where this procedure usually goes wrong}

## Evidence

- [[wiki/live/summaries/source-a]] - {what supported this workflow}
```

Rules:

- Use procedures for repeated workflows, playbooks, and decision patterns.
- Keep procedures grounded in approved evidence, not just habit.
- Prefer procedural pages over semantic pages when the durable delta is "how to do it" rather than "what it means".
