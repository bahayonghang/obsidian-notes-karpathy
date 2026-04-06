# Collect Sources

Collection is the human-owned stage of the workflow.

## Goal

Capture useful source material in markdown form under `raw/` with enough metadata for later compilation.

## Recommended collection methods

### Obsidian Web Clipper

Good default for articles and web pages.

Use metadata such as:

- `title`
- `source`
- `author`
- `date`
- `type`
- `tags`
- `clipped_at`

### Manual markdown creation

Use the same metadata shape, but keep raw notes free of compilation-state fields.

## Raw-layer rule

`raw/` is immutable from the compiler's point of view.

That means:

- put source metadata in raw notes
- put compilation metadata in summary pages
- let `kb-compile` update wiki pages, not the source file itself

## Suggested directories

- `raw/articles/`
- `raw/papers/`
- `raw/podcasts/`
- `raw/assets/`
