# Quick Start

Use this page when you want a minimal V2 loop in one session.

## 1. Create or repair the V2 contract

Expected support layer:

```text
raw/human/{articles,papers,podcasts,repos,assets}
raw/agents/{role}/
wiki/drafts/{summaries,concepts,entities,indices}
wiki/live/{summaries,concepts,entities,indices}
wiki/briefings/
wiki/index.md
wiki/log.md
outputs/{reviews,qa,health,reports,slides,charts,content/{articles,threads,talks}}
AGENTS.md
CLAUDE.md
```

## 2. Add captures

Place human-curated material under `raw/human/**`.

Place agent-produced captures under `raw/agents/{role}/**`.

## 3. Compile to drafts

Run `kb-compile`.

You should see reviewable pages in `wiki/drafts/`, not directly in `wiki/live/`.

## 4. Review and promote

Run `kb-review`.

You should see:

- review records in `outputs/reviews/`
- approved pages in `wiki/live/`
- rebuilt role briefings in `wiki/briefings/`

## 5. Ask a real question

Run `kb-query` after approval.

The query pass should read `wiki/live/`, optionally load the relevant briefing, and archive substantive answers to `outputs/qa/`.

## 6. Run a health baseline

Use `kb-health` once the first round-trip is complete.

The report should land in `outputs/health/health-check-{date}.md` and cover live integrity, backlog, briefings, and provenance.
