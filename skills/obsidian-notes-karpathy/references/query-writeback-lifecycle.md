# Query Writeback Lifecycle

Use this reference when `kb-query`, publish outputs, or governance maintenance needs to decide how durable follow-up work should re-enter the review-gated workflow.

## Core rule

Every substantive query should either improve the wiki or explicitly record why it does not need to. This is the Karpathy compounding principle: the wiki grows richer with every source added and every question answered.

Archived Q&A and publish outputs may surface durable follow-up work, but they never mutate `wiki/live/` directly.

Treat them as artifact archive:

- reusable
- inspectable
- maintenance-visible
- still outside approved truth

If an output discovers reusable long-term knowledge, the change must still re-enter through:

`draft -> review -> live`

If an output mainly reveals governance drift, alias problems, stale archives, or maintenance backlog, route that work to `kb-review` maintenance mode instead of pretending it is already approved knowledge.

Compounding doctrine: treat archived outputs as reusable working memory for the system rather than as dead-end deliverables. A good output should either stand on its own for reuse or leave behind clear next actions that help the vault become denser, better linked, and easier to query later.

Archive reuse order should stay disciplined:

1. approved live pages first
2. archived Q&A next
3. archived publish artifacts only when they already reuse approved coverage cleanly

## Required writeback signals for substantive outputs

Substantive Q&A or publish artifacts should record these fields when relevant:

- `source_live_pages` — the approved live pages that grounded the output
- `open_questions_touched` — standing questions materially advanced or reframed by the output
- `writeback_candidates` — concrete long-term follow-up worth re-entering the wiki
- `writeback_status` — `none | pending | triaged | drafted | reviewed | rejected`
- `followup_route` — `none | draft | review`
- optional `confidence_posture` — when the answer should advertise uncertainty or source disagreement explicitly
- optional `compounding_value` — `low | medium | high` to indicate how strongly the artifact is expected to pay off for future reuse, navigation, or follow-up drafting
- optional `crystallized_from_episode` — when the answer or artifact came out of an explicit episodic note

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

### `review`
Use `review` when the next step is an immediate human decision or a governance pass on already-prepared maintenance work, including:

- writeback backlog
- alias drift or duplicate approved pages
- stale archived Q&A relative to newer live sources
- stale briefings
- recurring unresolved question clusters
- provenance drift or thin-source maintenance work
- relationship gaps where approved pages should be linked, grouped, or surfaced through a curated hub before more new prose is written

## Writeback status semantics

Use these statuses as a lightweight lifecycle for durable follow-up:

- `none` — no durable follow-up was created
- `pending` — follow-up exists but nobody has triaged it yet
- `triaged` — next route and target are clear, but no draft or maintenance action has started
- `drafted` — the candidate has re-entered the draft lane or has been converted into an explicit maintenance target
- `reviewed` — the resulting draft or maintenance action has completed review or explicit disposition
- `rejected` — the follow-up was examined and deliberately not carried forward

Prefer advancing status explicitly instead of leaving old artifacts forever at `pending`.

## Writeback candidate posture

`writeback_candidates` should be concrete and reviewable.

Prefer candidates such as:

- `[[wiki/live/concepts/...]]` to update
- a missing concept/entity page to draft
- a new durable link or relationship to capture
- a standing question worth promoting into a governed note or index
- a curated hub or coverage map that should be refreshed because repeated outputs keep touching the same topic

Avoid vague placeholders like "update the wiki later".

When substantive outputs create durable follow-up work, surface that work somewhere operators can triage repeatedly rather than leaving it stranded inside a single archived artifact. Valid surfaces include governance indices, health reports, or another maintained backlog note that points back to the archived output.

Archive backlog is therefore a normal maintenance concern, not a reason to treat archive as truth.

## Relationship-first compounding

Before proposing a brand-new page, check whether the durable value is actually one of these lighter-weight moves:

- add `related` edges between existing live pages
- add a page to an existing `topic_hub`
- strengthen alias coverage for a page that is already the right canonical identity
- promote a recurring question into `QUESTIONS.md`
- record a missing bridge between prior approved coverage and the new artifact

Prefer the smallest durable writeback that improves future retrieval and synthesis without creating redundant pages.

## Governance relationship

Archived outputs can feed governance indices and health reports, especially:

- `QUESTIONS.md`
- `GAPS.md`
- `ALIASES.md`

But those surfaces remain maintenance/navigation views. They do not widen the truth boundary beyond approved live pages.

## Episode relationship

When an answer or publish artifact came from a broader chain of work, keep the breadcrumb:

- the episode captures the work arc
- the Q&A or content artifact captures the reusable answer
- durable knowledge still re-enters through draft -> review -> live

The episode is not the truth source. It is the reusable memory wrapper around the work that produced the artifact.

## Example posture

- grounded answer with no new follow-up -> archive with `followup_route: none`
- answer discovers a missing long-term concept -> archive with `writeback_candidates` and `followup_route: draft`
- answer reveals duplicate aliases across approved pages -> archive with `followup_route: review`
- answer mainly shows that three approved pages need stronger `related` links and a shared hub -> archive with `writeback_candidates`, `followup_route: review`, and `compounding_value: high`
- answer reframes an unresolved governance question -> archive with `open_questions_touched` and `review`

## Simple writeback for personal vaults

The full 6-status lifecycle (`none → pending → triaged → drafted → reviewed → rejected`) is valuable for team or enterprise vaults with formal review cadence. For personal vaults where the same person sources, queries, and reviews, a lighter path is usually enough:

1. **Query** produces an answer grounded in live pages
2. **Writeback signal** — the output records `followup_route` and `writeback_candidates`
3. **Draft** — the candidate re-enters through `wiki/drafts/` (or a relationship / hub upgrade)
4. **Approve** — the owner reviews and promotes to `wiki/live/`

In this mode, `pending → drafted` is the common happy path. Use `triaged` and `rejected` only when the backlog grows large enough to need triage.

The key Karpathy principle still applies: every substantive query should leave the wiki at least slightly better — whether through a new page, a stronger link, an updated hub, or even just a promoted question in `QUESTIONS.md`.
