# kb-health

Run a maintenance pass over the approved live brain and its review ecosystem. This is the longer-horizon maintenance lane, not the immediate draft-promotion gate.

## Audit scope

- `wiki/live/**`
- `wiki/briefings/**`
- `outputs/qa/**`
- `outputs/content/**`
- `outputs/reviews/**`
- `outputs/health/**`

Reference `raw/` only when provenance needs a spot check.

## Typical outcomes

- flag approved conflicts and duplicate concepts
- flag stale archived Q&A
- flag stale briefings
- flag review backlog
- flag writeback backlog
- flag render, provenance, source-drift, and alias issues
- flag collaboration memory mixing with approved knowledge
- refresh `QUESTIONS.md`, `GAPS.md`, or `ALIASES.md` views when governance maintenance is requested

## Report and fix posture

Health reports should land in `outputs/health/health-check-{date}.md`.

`kb-health` is report-first.

It may apply deterministic, reversible fixes when the target is unambiguous, but only inside approved surfaces such as `wiki/live/**`, `wiki/briefings/**`, `outputs/qa/**`, and `outputs/content/**`.

It must not mutate `raw/` or promote drafts into live.
