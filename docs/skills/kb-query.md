# kb-query

Search the approved live brain, answer questions, archive substantive answers, reuse prior Q&A, reuse archived publish artifacts when appropriate, and export static web packages from one canonical read-side lane.

## Hard boundary

`kb-query` should read:

- `wiki/live/**`
- `wiki/live/indices/**`
- `wiki/live/topics/**`
- relevant `wiki/briefings/**`
- prior `outputs/qa/**`

It should not treat `raw/` or `wiki/drafts/` as retrieval truth.

It should also keep `MEMORY.md` out of default topic retrieval. That file is for collaboration and editorial context, not approved domain knowledge.

## Main modes

- research mode for grounded answers archived to `outputs/qa/**`
- search mode for local-first candidate ranking before synthesis
- publish mode for outward-facing prose grounded in approved knowledge and saved under `outputs/content/**`
- web mode for static exports saved under `outputs/web/**`
- reflect-lite mode for synthesis or gap notes that should stay outside live until re-reviewed

Archive is a first-class responsibility here, not an afterthought:

- Q&A archive -> `outputs/qa/**`
- publish archive -> `outputs/content/**`
- both are reusable artifact archive
- both still sit below the truth boundary

If the user still says `kb-search`, route it here as `kb-query` search mode.

When a Q&A note or static web export creates durable follow-up work, capture structured `writeback_candidates`, `open_questions_touched`, `source_live_pages`, and a `followup_route` instead of leaving the next draft/review step implicit.

Before drafting a new outward-facing artifact, prefer checking whether prior approved live coverage or archived Q&A already explains the background well enough to reuse directly. Creator-facing requests such as public articles, threads, or newsletters belong to `publish` mode rather than generic writing.

Archived publish artifacts can also be reused, but only after live grounding and archived Q&A reuse have already been checked.

## Retrieval ladder

Default retrieval order:

1. `wiki/index.md`
2. `wiki/live/indices/*`
3. `wiki/live/topics/*`
4. governance indices such as `QUESTIONS.md`, `GAPS.md`, and `ALIASES.md` when present
5. relevant `wiki/briefings/{role}.md`
6. prior `outputs/qa/`
7. local structured or metadata-driven search
8. optional semantic retrieval only as candidate surfacing

Approved live pages remain the truth source throughout this ladder.

## Writeback contract

Substantive Q&A or publish outputs should record:

- `source_live_pages` when specific approved pages grounded the output
- `open_questions_touched` when the output materially advances a standing question
- `writeback_candidates` when the output discovers durable follow-up worth re-entering the wiki
- `followup_route` as `none | draft | review`

Use `draft` for durable knowledge updates that must re-enter draft -> review -> live, and `review` for immediate human decision points or maintenance-mode governance follow-up.
