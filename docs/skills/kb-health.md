# kb-health

Run a maintenance pass over the approved live brain and its review ecosystem.

## Audit scope

- `wiki/live/**`
- `wiki/briefings/**`
- `outputs/qa/**`
- `outputs/content/**`
- `outputs/reviews/**`
- `outputs/health/**`

## Typical outcomes

- flag approved conflicts
- flag stale briefings
- flag review backlog
- flag writeback backlog
- flag render and provenance issues
- flag collaboration memory mixing with approved knowledge

## Fix posture

`kb-health` is report-first.

It may apply deterministic, reversible fixes when the target is unambiguous, but only inside approved surfaces such as `wiki/live/**`, `wiki/briefings/**`, and `outputs/qa/**`.

It must not mutate `raw/` or promote drafts into live.
