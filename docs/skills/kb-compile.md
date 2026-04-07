# kb-compile

Compile immutable raw captures into reviewable draft knowledge.

## Responsibility

`kb-compile` reads from `raw/` and writes only to `wiki/drafts/`.

It should never promote directly into `wiki/live/`.

## Main outputs

- `wiki/drafts/summaries/**`
- `wiki/drafts/concepts/**`
- `wiki/drafts/entities/**`
- draft indices
- an `ingest` entry in `wiki/log.md`
