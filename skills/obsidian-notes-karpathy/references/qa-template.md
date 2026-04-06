# Q and A Archive Template

Persist substantive research answers to `outputs/qa/`.

```markdown
---
question: "{original-question}"
asked_at: "{datetime}"
sources:
  - "[[wiki/concepts/concept-a]]"
  - "[[wiki/entities/entity-b]]"
  - "[[wiki/summaries/source-b]]"
tags:
  - qa
  - topic/subtopic
---

# {title}

## TL;DR

{one sentence answer}

## Conclusions

{2-4 paragraphs}

## Key Findings

1. **{finding 1}** - {explanation}
   - Sources: [[wiki/summaries/source-b]]

2. **{finding 2}** - {explanation}
   - Sources: [[wiki/concepts/concept-a]], [[wiki/summaries/source-c]]

## Evidence Trail

- [[wiki/concepts/concept-a]] - {contribution}
- [[wiki/entities/entity-b]] - {contribution}
- [[wiki/summaries/source-b]] - {contribution}
- [[raw/articles/source-file]] - {specific excerpt or locator}

## Uncertainty

- {open question}
- {where sources disagree}
- {what new source would reduce uncertainty}

## Feed Back Into The Wiki

- {concept page to update}
- {entity page to create or update}
- {new link to add}
- {question worth promoting to a permanent concept note}
```

Rules:

- Archive only substantive answers.
- If the answer becomes the basis for a publishable artifact, link that artifact back here.
- If the answer resolves a mechanical gap in the wiki, capture that in `Feed Back Into The Wiki` rather than leaving it implicit.
