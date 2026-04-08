# kb-query

Search the approved live brain, answer questions, archive substantive answers, and generate outward-facing artifacts.

## Hard boundary

`kb-query` should read:

- `wiki/live/**`
- relevant `wiki/briefings/**`
- prior `outputs/qa/**`

It should not treat `raw/` or `wiki/drafts/` as retrieval truth.

It should also keep `MEMORY.md` out of default topic retrieval. That file is for collaboration and editorial context, not approved domain knowledge.

When a Q&A note or publish artifact exposes durable follow-up work, capture structured writeback candidates instead of leaving the next draft/review step implicit.
