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
evidence_coverage: 0.85
uncertainty_level: low | medium | high
---

# Draft Summary: {Source Title}

## Thesis

{one-paragraph statement of what the source is really saying}

## Source Claims

- {claim the source directly supports}
- {second claim the source directly supports}

## Compiler Inferences

- {synthesis, implication, or comparison added by the compiler}
- {second inference that may need review}

## Evidence

- "{quote or concrete datapoint}" - {where it appears}
- "{second datapoint}" - {why it matters}

## Proposed Concepts

- [[wiki/drafts/concepts/concept-a]] - {relationship}
- [[wiki/drafts/concepts/concept-b]] - {relationship}

## Proposed Entities

- [[wiki/drafts/entities/entity-a]] - {relationship}

## Open Questions

- {what remains uncertain}
- {what new source or review step would reduce uncertainty}

## Tensions and Review Notes

- {uncertainty, contradiction, or reason this may need human review}
```

Rules:

- draft summaries are reviewable evidence packages, not polished prose
- keep provenance explicit enough that a reviewer can judge the draft without agent-specific production context
- keep direct source claims separate from compiler-added inferences
- omit entity sections when the source does not introduce durable named entities
