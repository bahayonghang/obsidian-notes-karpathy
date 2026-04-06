# Compile Wiki

`kb-compile` is the compiler pass for the knowledge base.

## Inputs

- markdown files under `raw/`
- local schema from `AGENTS.md` and `CLAUDE.md`

## Outputs

- summary pages
- concept pages
- rebuilt indices
- updated `wiki/index.md`
- appended `wiki/log.md`

## Important rule

Compilation reads raw notes but does not mutate them.

## Large ingest batches

For first-time compilation, prefer batches of 5 sources so the resulting concept map and summaries can be reviewed between batches.
