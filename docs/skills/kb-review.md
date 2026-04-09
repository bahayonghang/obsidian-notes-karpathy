# kb-review

Run the explicit gate between draft knowledge and the approved live brain. This is the immediate gate lane for pending drafts and briefing refreshes that belong to the same review pass.

## What it reads

- `wiki/drafts/**`
- directly cited raw captures
- overlapping `wiki/live/**`
- shared review and briefing templates

## What it writes

- `outputs/reviews/**`
- approved pages in `wiki/live/**`
- regenerated `wiki/briefings/**`
- `review` and `brief` entries in `wiki/log.md`

## Review posture

- review the draft package, directly cited raw captures, and overlapping live pages only
- do not preserve a draft merely because the producing agent seems competent
- judge whether the page deserves durable retention in the long-term brain, not just whether it is superficially plausible

## Decision model

- auto-approve when the draft is well-supported and conflict-free
- auto-reject when blocking flags remain or the evidence trail is weak
- escalate to human review when the draft conflicts with approved live knowledge or falls into an ambiguous score band

`kb-review` should explicitly weigh fact/inference separation, alias alignment, duplicate risk, contradiction handling, and promotion value. Broader approved-layer cleanup belongs to `kb-health`, not this gate.
