# Content Output Template

Use this when the user wants to turn the knowledge base into publishable content.

```markdown
---
title: "{Artifact Title}"
created_at: "{datetime}"
artifact_type: article | thread | newsletter | talk-outline
channel: "{channel}"
sources:
  - "[[wiki/concepts/concept-a]]"
  - "[[wiki/entities/entity-b]]"
  - "[[outputs/qa/question-a]]"
derived_from:
  - "[[outputs/qa/question-a]]"
---

# {Artifact Title}

## Brief

- Audience: {audience}
- Goal: {goal}
- Angle: {angle}

## Core Claims

1. {claim 1}
2. {claim 2}
3. {claim 3}

## Draft

{full draft}

## Provenance

- [[wiki/concepts/concept-a]] - {what it contributed}
- [[wiki/entities/entity-b]] - {what it contributed}
- [[outputs/qa/question-a]] - {what it contributed}

## Feed Back Into The Wiki

- {new concept, entity, or source gap discovered while drafting}
```

Rules:

- Publish artifacts should stay grounded in the wiki, not free-associate away from the evidence.
- If the draft introduces a new synthesis worth preserving, consider also archiving a supporting Q&A note.
