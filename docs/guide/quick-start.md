# Quick Start

Use this page when you want a minimal review-gated loop in one session.

## 0. Migrate legacy vaults first

If the vault still uses direct `wiki/summaries/` or `wiki/concepts/` paths, run `kb-init` in migration mode before you try normal compile, query, or health work.

## 1. Create or repair the required support layer

Expected minimum support layer:

```text
raw/human/{articles,papers,podcasts,repos,assets}
raw/agents/{role}/
wiki/drafts/{summaries,concepts,entities,indices}
wiki/live/{summaries,concepts,entities,indices}
wiki/briefings/
wiki/index.md
wiki/log.md
outputs/reviews/
AGENTS.md
CLAUDE.md
```

Optional downstream directories such as `outputs/qa/`, `outputs/health/`, and `outputs/content/**` are created when later stages need them.

`MEMORY.md` is recommended even though it is not retrieval truth. Use it for collaboration rules, preferences, and editorial priorities.

## 2. Add captures

Place human-curated material under `raw/human/**`.

Place agent-produced captures under `raw/agents/{role}/**`.

Bootstrap vaults may also place markdown directly under `raw/`. That is a valid compile input, but it does not replace the required support layer.

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

The query pass should read `wiki/live/`, optionally load the relevant briefing, ignore `MEMORY.md` for topic knowledge, and archive substantive answers to `outputs/qa/`.

When a strong answer reveals durable follow-up work, capture explicit writeback candidates so the next compile/review loop can decide whether that answer should feed back into the wiki.

## 6. Run a health baseline

Use `kb-health` once the first round-trip is complete.

The report should land in `outputs/health/health-check-{date}.md` and cover live integrity, backlog, briefings, and provenance. Health work is report-first and should only apply deterministic mechanical fixes when the target is unambiguous.
