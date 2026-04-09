# Questions and Reflection Policy

Use this policy when the user wants open questions, synthesis, gap analysis, or reflective maintenance work without collapsing the review gate.

## Core rule

Questions, gap reports, and reflection outputs may be archived directly to `outputs/qa/` or `outputs/health/`, but they only become reusable long-term knowledge after re-entering draft -> review -> live.

## Open questions

- open questions may be tracked in optional indices such as `wiki/live/indices/QUESTIONS.md`
- Q&A artifacts should record `open_questions_touched` when they materially advance a standing question
- unresolved gaps should be explicit rather than implied in prose

## Reflection posture

- reflection should start from approved live pages, not drafts or raw captures by default
- when reflection finds a new durable synthesis, capture it as a writeback candidate or draft candidate
- when reflection finds contradiction or missing evidence, route the follow-up to health or review rather than silently rewriting live notes

## Gap analysis

Useful gap signals include:

- repeated question themes across archived Q&A
- concepts with thin source support
- duplicate aliases split across multiple live pages
- stale pages in volatile domains
- briefings that keep surfacing the same unresolved evidence hole

## Human involvement

- if a synthesis materially changes an approved interpretation, prefer `needs-human` review instead of auto-promoting confidence
- if a reflective output only summarizes existing approved knowledge, it may stay in `outputs/qa/` or `outputs/content/` without mutating live
