# kb-review

Run the canonical governance lane between draft knowledge and the approved live brain. `kb-review` now covers both the immediate `gate` mode and the longer-horizon `maintenance` mode.

If a user speaks in `Chinese-LLM-Wiki` maintenance terms such as `lint`, `孤儿页`, `断链`, `旧结论被覆盖`, or asks for a governance-style `output/reports`, route that work here rather than to deterministic rendering.

## What it reads

- `wiki/drafts/**`
- directly cited raw captures
- overlapping `wiki/live/**`
- shared review and briefing templates

## What it writes

- `outputs/reviews/**`
- approved pages in `wiki/live/**`
- regenerated `wiki/briefings/**`
- refreshed governance indices in `wiki/live/indices/**`
- maintenance reports in `outputs/health/**`
- graph snapshots in `outputs/health/graph-snapshot.json`
- `review` and `brief` entries in `wiki/log.md`

## Review posture

- review the draft package, directly cited raw captures, and overlapping live pages only
- do not preserve a draft merely because the producing agent seems competent
- judge whether the page deserves durable retention in the long-term brain, not just whether it is superficially plausible

## Decision model

- auto-approve when the draft is well-supported and conflict-free
- auto-reject when blocking flags remain or the evidence trail is weak
- escalate to human review when the draft conflicts with approved live knowledge or falls into an ambiguous score band

`kb-review` should explicitly weigh fact/inference separation, alias alignment, duplicate risk, contradiction handling, and promotion value. In maintenance mode it should also audit drift, backlog pressure, provenance quality, stale archived outputs, weak graph structure across approved surfaces, and creator consistency between `CLAUDE.md`, `MEMORY.md`, account `_style-guide.md`, and account briefings.

Archive hygiene is part of maintenance mode:

- stale archived Q&A relative to newer live pages
- archived publish artifacts with reuse drift
- writeback backlog in archived outputs
- scope leaks in archived outputs

That work keeps archive reusable. It does not promote archive into truth.
