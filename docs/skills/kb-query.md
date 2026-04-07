# kb-query

Search the approved live brain, answer questions, archive substantive answers, and generate outward-facing artifacts.

## Hard boundary

`kb-query` should read:

- `wiki/live/**`
- relevant `wiki/briefings/**`
- prior `outputs/qa/**`

It should not treat `raw/` or `wiki/drafts/` as retrieval truth.
