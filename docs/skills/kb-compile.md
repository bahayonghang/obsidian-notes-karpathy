# kb-compile

Compile immutable raw captures into reviewable draft knowledge.

## Responsibility

`kb-compile` reads from:

- `raw/human/**`
- `raw/agents/{role}/**`
- bootstrap `raw/*.md`
- legacy raw paths only during migration

It writes only to:

- `wiki/drafts/summaries/**`
- `wiki/drafts/concepts/**`
- `wiki/drafts/entities/**`
- `wiki/drafts/indices/**`
- `wiki/log.md`

It should never promote directly into `wiki/live/`.

## Paper and evidence rules

Raw paper PDFs under `raw/**/papers/*.pdf` are routing exceptions for `paper-workbench`, not normal compile inputs. `kb-compile` should surface or defer them and report skipped PDFs when the companion skill is unavailable.

Before shaping drafts, the compiler should normalize source metadata such as `source_hash`, `source_mtime`, `last_verified_at`, and `possibly_outdated`.

## Main outputs

- reviewable summaries with `compiled_from`, `capture_sources`, `review_state`, and `review_score`
- concept and entity drafts with explicit evidence trails
- draft indices
- surfaced `alias_candidates` and `duplicate_candidates`
- an `ingest` entry in `wiki/log.md`

Drafts should stay shaped for review rather than final polish. They should make uncertainty, evidence coverage, and blocking flags easy for `kb-review` to judge.
