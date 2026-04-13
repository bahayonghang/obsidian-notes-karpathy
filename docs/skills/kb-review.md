# kb-review

Run the canonical governance lane between draft knowledge and the approved live brain. `kb-review` now covers both the immediate `gate` mode and the longer-horizon `maintenance` mode.

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

`kb-review` should explicitly weigh fact/inference separation, alias alignment, duplicate risk, contradiction handling, and promotion value. In maintenance mode it should also audit drift, backlog pressure, provenance quality, stale archived outputs, and weak graph structure across approved surfaces.
