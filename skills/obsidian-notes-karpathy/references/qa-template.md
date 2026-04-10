# Q and A Archive Template

Persist substantive research answers to `outputs/qa/`.

```markdown
---
question: "{original-question}"
asked_at: "{datetime}"
sources:
  - "[[wiki/live/concepts/concept-a]]"
  - "[[wiki/live/entities/entity-b]]"
  - "[[wiki/live/summaries/source-b]]"
source_live_pages:
  - "[[wiki/live/concepts/concept-a]]"
  - "[[wiki/live/summaries/source-b]]"
tags:
  - qa
  - topic/subtopic
open_questions_touched:
  - "Which findings should be promoted into permanent notes?"
writeback_candidates:
  - "[[wiki/live/concepts/concept-a]]"
writeback_status: none | pending | compiled | rejected
followup_route: none | draft | review | health
confidence_posture: grounded | mixed-evidence | exploratory
---

# {title}

## TL;DR

{one sentence answer}

## Conclusions

{2-4 paragraphs}

## Key Findings

1. **{finding 1}** - {explanation}
   - Sources: [[wiki/live/summaries/source-b]]

2. **{finding 2}** - {explanation}
   - Sources: [[wiki/live/concepts/concept-a]], [[wiki/live/summaries/source-c]]

## Evidence Trail

- [[wiki/live/concepts/concept-a]] - {contribution}
- [[wiki/live/entities/entity-b]] - {contribution}
- [[wiki/live/summaries/source-b]] - {contribution}
- [[raw/human/articles/source-file]] - {specific excerpt or locator}

## Uncertainty

- {open question}
- {where sources disagree}
- {what new source would reduce uncertainty}

## Writeback Candidates

- {concept page to update}
- {entity page to create or update}
- {new link to add}
- {question worth promoting to a permanent concept note}
```

Rules:

- Archive only substantive answers.
- If the answer becomes the basis for a publishable artifact, link that artifact back here.
- If the answer reveals durable knowledge work, express that as `writeback_candidates` plus a concrete writeback section instead of leaving it implicit.
