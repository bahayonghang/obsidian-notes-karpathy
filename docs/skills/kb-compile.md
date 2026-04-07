# kb-compile

Compile immutable raw notes into the maintained wiki.

## Responsibility

`kb-compile` behaves like a compiler pass, not like a source editor. It reads from `raw/`, writes to `wiki/`, and keeps provenance intact.

`raw/papers/` is dual-mode: it may contain markdown paper notes or PDF papers.

## Read first

The live skill contract reads:

- local `AGENTS.md`
- local `CLAUDE.md` when present
- shared templates for schema, summaries, concepts, entities, indices, logs, lifecycle routing, and Obsidian-safe markdown
- `scripts/scan_compile_delta.py` when available, before any manual compile-status inference

## Incremental model

Each raw source is matched against its summary in `wiki/summaries/`.

Recompile when:

- the summary is missing
- `source_hash` is older than the raw file
- `source_mtime` is older than the raw file

Skip when the compiled summary already matches the current raw source.

## PDF paper fallback chain

For PDFs under `raw/papers/`:

1. prefer `alphaxiv-paper-lookup`
2. if that is unavailable or not applicable, fall back to `pdf`
3. if neither companion skill is installed, skip only that PDF and tell the user what to install

The compiler should never write extracted markdown back into `raw/`.

## Main outputs

- `wiki/summaries/*.md`
- `wiki/concepts/*.md`
- `wiki/entities/*.md` when enabled
- `wiki/index.md`
- `wiki/log.md`
- `wiki/indices/INDEX.md`
- `wiki/indices/CONCEPTS.md`
- `wiki/indices/SOURCES.md`
- `wiki/indices/RECENT.md`
- `wiki/indices/ENTITIES.md` when the entity layer exists

## Conflict handling

When new evidence conflicts with an existing concept or entity page, the compiler should not silently overwrite the old claim. It should surface the tension and keep provenance visible.

## Light sanity check

After compilation, the skill can fix obvious mechanical issues introduced by the pass, such as:

- broken links created during the update
- missing reciprocal `related` fields
- summary metadata gaps
- alias-style wikilinks inside Markdown table cells

For a full maintenance report, hand off to [kb-health](/skills/kb-health).
