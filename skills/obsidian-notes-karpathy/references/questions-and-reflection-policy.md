# Questions and Reflection Policy

Use this policy when the user wants open questions, synthesis, gap analysis, or reflective maintenance work without collapsing the review gate.

## Core rule

Questions, gap reports, and reflection outputs may be archived directly to `outputs/qa/` or `outputs/health/`, but they only become reusable long-term knowledge after re-entering draft -> review -> live.

Use `query-writeback-lifecycle.md` for the detailed field-level contract when a query or publish output discovers durable follow-up work.

## Open questions

- open questions should be tracked in governance indices such as `wiki/live/indices/QUESTIONS.md` for mature vaults, but that live-layer index should stay grounded in approved live pages rather than archived outputs
- Q&A artifacts should record `open_questions_touched` when they materially advance a standing question
- unresolved gaps should be explicit rather than implied in prose
- archived outputs may contribute question signals, but they do not become approved truth on their own

## Reflection posture

- reflection should start from approved live pages, not drafts or raw captures by default
- when reflection finds a new durable synthesis, capture it as a writeback candidate and set a `followup_route`
- when reflection finds contradiction or missing evidence, route the follow-up to health or review rather than silently rewriting live notes

## Follow-up routing

Use:

- `none` when the output is grounded and does not create durable follow-up work
- `draft` when the output suggests new or updated long-term knowledge that must re-enter draft -> review -> live
- `review` when the next action is an immediate human decision on a prepared candidate or unresolved interpretation
- `health` when the output primarily reveals governance drift, stale outputs, alias problems, or backlog maintenance work

## Gap analysis

Useful gap signals include:

- repeated question themes across archived Q&A
- concepts with thin source support
- duplicate aliases split across multiple live pages
- stale pages in volatile domains
- briefings that keep surfacing the same unresolved evidence hole
- archived outputs with `writeback_candidates` that remain pending or untriaged
- topic programs or curated hubs that show repeated coverage gaps, imbalance, or over-reliance on the same prior explanation

## Human involvement

- if a synthesis materially changes an approved interpretation, prefer `needs-human` review instead of auto-promoting confidence
- if a reflective output only summarizes existing approved knowledge, it may stay in `outputs/qa/` or `outputs/content/` without mutating live
