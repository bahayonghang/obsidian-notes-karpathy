# Content Output Template

Use this when the user wants to turn the knowledge base into publishable content.

```markdown
---
title: "{Artifact Title}"
created_at: "{datetime}"
artifact_type: article | thread | newsletter | talk-outline
channel: "{channel}"
sources:
  - "[[wiki/live/concepts/concept-a]]"
  - "[[wiki/live/entities/entity-b]]"
  - "[[outputs/qa/question-a]]"
source_live_pages:
  - "[[wiki/live/concepts/concept-a]]"
  - "[[wiki/live/entities/entity-b]]"
derived_from:
  - "[[outputs/qa/question-a]]"
writeback_candidates:
  - "[[wiki/live/concepts/concept-a]]"
writeback_status: none | pending | compiled | rejected
followup_route: none | draft | review | health
confidence_posture: grounded | mixed-evidence | exploratory
---

# {Artifact Title}

## Brief

- Audience: {audience}
- Goal: {goal}
- Angle: {angle}
- Prior coverage reused: {existing approved page or archived artifact reused instead of re-explaining from scratch}

## Core Claims

1. {claim 1}
2. {claim 2}
3. {claim 3}

## Draft

{full draft}

## Provenance

- [[wiki/live/concepts/concept-a]] - {what it contributed}
- [[wiki/live/entities/entity-b]] - {what it contributed}
- [[outputs/qa/question-a]] - {what it contributed}

## Writeback Candidates

- {new concept, entity, or source gap discovered while drafting}
```

Rules:

- Publish artifacts should stay grounded in the wiki, not free-associate away from the evidence.
- Prefer reusing prior approved coverage or archived outputs explicitly before restating the same background explanation in a new artifact.
- If the draft introduces a new synthesis worth preserving, pair the artifact with explicit `writeback_candidates` or a supporting Q&A note.
- Keep raw source captures immutable; editorial shaping belongs in the artifact, Q&A archive, or the later draft/review lane.
