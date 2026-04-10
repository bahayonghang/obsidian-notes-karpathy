# kb-query

Search the approved live brain, answer questions, archive substantive answers, and generate outward-facing artifacts.

## Hard boundary

`kb-query` should read:

- `wiki/live/**`
- `wiki/live/indices/**`
- relevant `wiki/briefings/**`
- prior `outputs/qa/**`

It should not treat `raw/` or `wiki/drafts/` as retrieval truth.

It should also keep `MEMORY.md` out of default topic retrieval. That file is for collaboration and editorial context, not approved domain knowledge.

## Main modes

- search mode for locating approved pages quickly
- research mode for grounded answers archived to `outputs/qa/**`
- publish mode for reports, threads, talks, or slides saved under `outputs/content/**`
- reflect-lite mode for synthesis or gap notes that should stay outside live until re-reviewed

When a Q&A note or publish artifact creates durable follow-up work, capture structured `writeback_candidates`, `open_questions_touched`, `source_live_pages`, and a `followup_route` instead of leaving the next draft/review step implicit.

Before drafting a new outward-facing artifact, prefer checking whether prior approved live coverage or archived Q&A already explains the background well enough to reuse directly.

## Retrieval ladder

Default retrieval order:

1. `wiki/index.md`
2. `wiki/live/indices/*`
3. governance indices such as `QUESTIONS.md`, `GAPS.md`, and `ALIASES.md` when present
4. relevant `wiki/briefings/{role}.md`
5. prior `outputs/qa/`
6. local structured or metadata-driven search
7. optional semantic retrieval only as candidate surfacing

Approved live pages remain the truth source throughout this ladder.

## Writeback contract

Substantive Q&A or publish outputs should record:

- `source_live_pages` when specific approved pages grounded the output
- `open_questions_touched` when the output materially advances a standing question
- `writeback_candidates` when the output discovers durable follow-up worth re-entering the wiki
- `followup_route` as `none | draft | review | health`

Use `draft` for durable knowledge updates that must re-enter draft -> review -> live, `review` for immediate human decision points, and `health` for governance/backlog/drift work rather than truth mutation.
