# Episode Template

Use this structure for `outputs/episodes/**`.

```markdown
---
title: "Episode: {Episode Title}"
episode_id: "{episode-slug}"
memory_tier: episodic
captured_at: "{datetime}"
episode_scope: qa | content | session
source_artifacts:
  - "[[outputs/qa/example-output]]"
source_live_pages:
  - "[[wiki/live/concepts/example-concept]]"
open_questions_touched:
  - "{standing question}"
writeback_candidates:
  - "{durable follow-up}"
followup_route: none | draft | review
consolidation_status: pending | drafted | reviewed | completed
visibility_scope: private | shared
---

# Episode: {Episode Title}

## What Happened

{Short narrative of the work arc}

## Durable Signals

- {reusable pattern}
- {question that should feed back into the wiki}
```

Rules:

- Episode pages are reusable memory, not approved topic truth.
- Prefer compact summaries over raw transcripts.
- Link back to the originating Q&A or content output so later audit and supersession work stays traceable.
