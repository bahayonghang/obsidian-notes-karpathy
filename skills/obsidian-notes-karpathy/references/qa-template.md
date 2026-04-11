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
  - "update [[wiki/live/concepts/concept-a]] with a sharper distinction"
writeback_status: none | pending | triaged | drafted | reviewed | rejected
followup_route: none | draft | review | health
confidence_posture: grounded | mixed-evidence | exploratory
compounding_value: low | medium | high
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

## Proposed Durable Delta

- Preferred move: {update live page | draft new page | strengthen relationships | create / expand curated hub}
- Target: {page or hub}
- Why this compounds the wiki: {future reuse, navigation gain, or reduced duplication}

## Writeback Candidates

- {concept page to update}
- {entity page to create or update}
- {new link or relationship to add}
- {hub or synthesis page to create or expand}
- {question worth promoting to a permanent concept note}
```

Rules:

- Archive only substantive answers.
- If the answer becomes the basis for a publishable artifact, link that artifact back here.
- If the answer reveals durable knowledge work, express that as `writeback_candidates` plus a concrete writeback section instead of leaving it implicit.
- Prefer the smallest durable delta that improves future retrieval: sometimes the right move is a new page, but often it is a stronger `related` edge, a hub update, or a promoted governed question.
