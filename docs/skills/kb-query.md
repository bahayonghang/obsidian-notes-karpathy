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

When a Q&A note or publish artifact exposes durable follow-up work, capture structured `writeback_candidates` and `open_questions_touched` instead of leaving the next draft/review step implicit.
