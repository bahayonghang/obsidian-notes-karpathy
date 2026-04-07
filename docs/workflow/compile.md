# Compile Wiki

`kb-compile` is the compiler pass for the knowledge base.

## Inputs

- markdown files under `raw/`
- paper PDFs under `raw/papers/`, handled by preferring `alphaxiv-paper-lookup` and then falling back to `pdf`
- local schema from `AGENTS.md` and `CLAUDE.md`

## Outputs

- summary pages
- concept pages
- rebuilt indices
- updated `wiki/index.md`
- appended `wiki/log.md`

## Important rule

Compilation reads raw notes but does not mutate them.

If neither paper companion skill is installed, the compiler should skip only the affected PDFs and return install guidance.

## Large ingest batches

For first-time compilation, prefer batches of 5 sources so the resulting concept map and summaries can be reviewed between batches.
