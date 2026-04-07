# Draft Summary Template

Use this structure for `wiki/drafts/summaries/**`.

```markdown
---
title: "Draft Summary: {Source Title}"
source_file: "[[{raw-link}]]"
source_hash: "{optional-hash}"
source_mtime: "{mtime}"
compiled_at: "{datetime}"
draft_id: "{stable-draft-id}"
compiled_from:
  - "[[{raw-link}]]"
capture_sources:
  - "[[{raw-link}]]"
review_state: "pending"
review_score: 0.88
blocking_flags: []
accuracy: 0.90
provenance: 0.92
composability: 0.84
conflict_risk: 0.12
---

# Draft Summary: {Source Title}

## Thesis

{one-paragraph statement of what the source is really saying}

## Key Takeaways

- {takeaway 1}
- {takeaway 2}
- {takeaway 3}

## Evidence

- "{quote or concrete datapoint}" - {where it appears}
- "{second datapoint}" - {why it matters}

## Proposed Concepts

- [[wiki/drafts/concepts/concept-a]] - {relationship}
- [[wiki/drafts/concepts/concept-b]] - {relationship}

## Proposed Entities

- [[wiki/drafts/entities/entity-a]] - {relationship}

## Tensions and Review Notes

- {uncertainty, contradiction, or reason this may need human review}
```

Rules:

- draft summaries are reviewable evidence packages, not polished prose
- keep provenance explicit enough that a reviewer can judge the draft without agent-specific production context
- omit entity sections when the source does not introduce durable named entities
