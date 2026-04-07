# Compile Wiki

`kb-compile` is the compiler pass for the knowledge base.

## Inputs

- markdown files under `raw/`
- paper PDFs under `raw/papers/`, always handled through `paper-workbench` in `json` mode; deterministic handles from sidecars or filenames remain optional metadata only
- local schema from `AGENTS.md` and `CLAUDE.md`

## Outputs

- summary pages
- concept pages
- rebuilt indices
- updated `wiki/index.md`
- appended `wiki/log.md`

## Important rule

Compilation reads raw notes but does not mutate them.

If `paper-workbench` is not installed, the compiler should skip only the affected `raw/papers` PDFs and return install guidance.

An optional `paper-name.source.md` sidecar may sit next to the PDF to provide `paper_id` or `source` metadata without becoming a second raw source.

## Large ingest batches

For first-time compilation, prefer batches of 5 sources so the resulting concept map and summaries can be reviewed between batches.
