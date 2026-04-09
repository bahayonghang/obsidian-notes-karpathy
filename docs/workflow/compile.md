# Compile Wiki

`kb-compile` is the compiler pass for the knowledge base.

## Inputs

- markdown files under `raw/human/**`
- markdown files under `raw/agents/{role}/**`
- bootstrap markdown files under `raw/*.md`
- paper PDFs under `raw/**/papers/*.pdf`, treated as `paper-workbench` routing exceptions rather than normal compile input
- local schema from `AGENTS.md` and `CLAUDE.md`

## Outputs

- `wiki/drafts/summaries/**`
- `wiki/drafts/concepts/**`
- `wiki/drafts/entities/**`
- `wiki/drafts/indices/**`
- appended `wiki/log.md`

## Important rule

Compilation reads raw notes but does not mutate them, and it does not promote directly into `wiki/live/`.

The compile pass should also surface alias and duplicate candidates, plus evidence metadata such as `source_hash`, `last_verified_at`, and `possibly_outdated`, so `kb-review` can make an explicit gate decision.

If `paper-workbench` is not installed, the compiler should skip only the affected `raw/**/papers/*.pdf` files and return install guidance instead of pretending they compiled normally.

An optional `paper-name.source.md` sidecar may sit next to the PDF to provide `paper_id` or `source` metadata without becoming a second raw source or changing the routing rule.

## Large ingest batches

For first-time compilation, prefer batches of 5 sources so the resulting concept map and summaries can be reviewed between batches.
