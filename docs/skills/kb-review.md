# kb-review

Run the explicit gate between draft knowledge and the approved live brain.

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

## Decision model

- auto-approve when the draft is well-supported and conflict-free
- auto-reject when blocking flags remain or the evidence trail is weak
- escalate to human review when the draft conflicts with approved live knowledge or falls into an ambiguous score band
