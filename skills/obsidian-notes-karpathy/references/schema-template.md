# Schema Template Notes

Keep `AGENTS.md` and `CLAUDE.md` aligned when both exist. `AGENTS.md` is the required contract surface; `CLAUDE.md` is the generated companion and should mirror it rather than introducing a second policy.

When present, `MEMORY.md` is the coordination surface for preferences, editorial priorities, and long-running collaboration context. It is not part of the default knowledge retrieval truth layer.

Use ISO dates whenever possible:

- date only: `YYYY-MM-DD`
- datetime: `YYYY-MM-DDTHH:mm:ssZ`

Use one property vocabulary consistently across the vault. Global property consistency matters because routing, review, and briefing regeneration depend on it.

## Raw capture frontmatter

```yaml
---
title: "Source Title"
source: "https://example.com"
author: "Author Name"
date: 2026-04-01
type: article | paper | repo | dataset | tweet | video | book | podcast | other
tags:
  - topic/subtopic
clipped_at: 2026-04-01T12:00:00Z
---
```

Rules:

- use `raw/human/**` for curated captures
- use `raw/agents/{role}/**` for untrusted agent captures
- do not add compile or review state to raw captures

## Draft summary frontmatter

```yaml
---
title: "Draft Summary: Source Title"
source_file: "[[raw/human/articles/2026-04-01-source-title]]"
source_hash: "stable-hash"
source_mtime: "2026-04-01T12:00:00Z"
compiled_at: "2026-04-02T08:00:00Z"
draft_id: "draft-source-title"
compiled_from:
  - "[[raw/human/articles/2026-04-01-source-title]]"
capture_sources:
  - "[[raw/human/articles/2026-04-01-source-title]]"
review_state: pending | promoted | rejected
review_score: 0.88
blocking_flags:
  - "live_conflict"
evidence_coverage: 0.85
uncertainty_level: medium
---
```

## Live page frontmatter

```yaml
---
title: "Source Title"
approved_at: "2026-04-02T10:00:00Z"
approved_from: "[[wiki/drafts/summaries/human/articles/2026-04-01-source-title]]"
review_record: "[[outputs/reviews/source-title]]"
trust_level: approved
updated_at: "2026-04-02T10:00:00Z"
last_reviewed_at: "2026-04-02T10:00:00Z"
status: active | conflicting
sources:
  - "[[wiki/live/summaries/human/articles/2026-04-01-source-title]]"
related:
  - "[[wiki/live/concepts/review-gate]]"
---
```

## Briefing frontmatter

```yaml
---
title: "Researcher Briefing"
brief_for: "researcher"
built_from: "wiki/live/"
updated_at: "2026-04-02T11:00:00Z"
staleness_after: "2026-04-02T11:00:00Z"
source_live_pages:
  - "[[wiki/live/concepts/review-gate]]"
  - "[[wiki/live/summaries/human/articles/2026-04-01-source-title]]"
---
```

## Review record frontmatter

```yaml
---
title: "Review Record: Source Title"
decision: approve | reject | needs-human
accuracy: 0.92
provenance: 0.95
conflict_risk: 0.18
composability: 0.90
fact_inference_separation: 0.90
promotion_reason: "The page cleanly separates direct evidence from synthesis and is worth reusing."
reviewed_at: "2026-04-02T10:00:00Z"
---
```

## Q&A frontmatter

```yaml
---
question: "When should I rebuild briefings?"
asked_at: "2026-04-05T11:15:00Z"
sources:
  - "[[wiki/live/concepts/review-gate]]"
  - "[[wiki/live/summaries/human/articles/2026-04-01-source-title]]"
tags:
  - qa
  - review
writeback_candidates:
  - "[[wiki/live/concepts/review-gate]]"
writeback_status: pending
---
```

## Health report frontmatter

```yaml
---
title: "Health Check Report"
date: "2026-04-05T12:00:00Z"
scope: "wiki/live/, wiki/briefings/, outputs/qa/, outputs/reviews/"
health_score: 84
---
```

## Naming rules

- keep raw capture filenames stable and lowercase kebab-case
- preserve relative capture structure under `wiki/drafts/summaries/**`
- keep promoted live slugs stable even when titles evolve
- any generated tables must obey `obsidian-safe-markdown.md`; never emit alias-style wikilinks inside table cells
