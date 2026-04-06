# kb-compile

Compile immutable raw notes into the maintained wiki.

## What it does

- discovers new or changed raw sources
- writes or updates summary pages
- creates or updates concept pages
- expands aliases and contradiction markers when needed
- rebuilds index and log surfaces
- runs a light post-compile sanity check

## What it does not do

- it does not rewrite raw source files
- it does not own the deep health-check workflow

## Incremental model

`kb-compile` compares each raw source with the metadata stored in its corresponding summary page. If the summary is missing or older than the raw source, that source is recompiled.

The preferred tracking fields are `source_hash` and `source_mtime`.

## Main outputs

- `wiki/summaries/*.md`
- `wiki/concepts/*.md`
- `wiki/index.md`
- `wiki/log.md`
- `wiki/indices/*`

If the user wants a full maintenance report, follow with [kb-health](/skills/kb-health).
