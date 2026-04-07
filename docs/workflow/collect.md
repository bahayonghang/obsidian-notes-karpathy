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

### Paper PDFs

If the source artifact is a paper PDF, place it under `raw/papers/`.

- if you already know the paper handle, optionally add `paper-name.source.md` next to the PDF with frontmatter such as `paper_id` or `source`
- prefer `alphaxiv-paper-lookup` during compilation only when the PDF resolves to a deterministic paper handle
- otherwise fall back to the `pdf` skill
- if neither companion skill is installed, report the missing install step instead of pretending the paper was compiled

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
