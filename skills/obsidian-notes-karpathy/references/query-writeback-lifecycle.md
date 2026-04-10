# Query Writeback Lifecycle

Use this reference when `kb-query`, publish outputs, or health maintenance needs to decide how durable follow-up work should re-enter the review-gated workflow.

## Core rule

Archived Q&A and publish outputs may surface durable follow-up work, but they never mutate `wiki/live/` directly.

If an output discovers reusable long-term knowledge, the change must still re-enter through:

`draft -> review -> live`

If an output mainly reveals governance drift, alias problems, stale archives, or maintenance backlog, route that work to `kb-health` instead of pretending it is already approved knowledge.

## Required writeback signals for substantive outputs

Substantive Q&A or publish artifacts should record these fields when relevant:

- `source_live_pages` — the approved live pages that grounded the output
- `open_questions_touched` — standing questions materially advanced or reframed by the output
- `writeback_candidates` — concrete long-term follow-up worth re-entering the wiki
- `writeback_status` — `none | pending | compiled | rejected`
- `followup_route` — `none | draft | review | health`
- optional `confidence_posture` — when the answer should advertise uncertainty or source disagreement explicitly

## How to choose `followup_route`

### `none`
Use when the output is grounded, complete enough for archival reuse, and does not create durable follow-up work.

### `draft`
Use when the output suggests:

- a new concept/entity/summary note
- a durable update to an approved live page
- a reusable synthesis that should be promoted only after normal review
- a new curated hub/topic page backed by approved evidence
- a creator-facing brief or publish synthesis that discovered durable knowledge worth formalizing beyond the artifact itself

### `review`
Use when the next step is an immediate human decision on an already-prepared candidate, disputed interpretation, or promotion boundary question.

### `health`
Use when the output mainly surfaces:

- writeback backlog
- alias drift or duplicate approved pages
- stale archived Q&A relative to newer live sources
- stale briefings
- recurring unresolved question clusters
- provenance drift or thin-source maintenance work

## Writeback candidate posture

`writeback_candidates` should be concrete and reviewable.

Prefer candidates such as:

- `[[wiki/live/concepts/...]]` to update
- a missing concept/entity page to draft
- a new durable link or relationship to capture
- a standing question worth promoting into a governed note or index

Avoid vague placeholders like "update the wiki later".

## Governance relationship

Archived outputs can feed governance indices and health reports, especially:

- `QUESTIONS.md`
- `GAPS.md`
- `ALIASES.md`

But those surfaces remain maintenance/navigation views. They do not widen the truth boundary beyond approved live pages.

## Example posture

- grounded answer with no new follow-up -> archive with `followup_route: none`
- answer discovers a missing long-term concept -> archive with `writeback_candidates` and `followup_route: draft`
- answer reveals duplicate aliases across approved pages -> archive with `followup_route: health`
- answer reframes an unresolved governance question -> archive with `open_questions_touched` and either `review` or `health`, depending on the next action
