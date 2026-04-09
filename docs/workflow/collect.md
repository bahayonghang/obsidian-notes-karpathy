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

If the source artifact is a paper PDF, place it under a `papers/` subtree inside `raw/`, usually `raw/human/papers/`.

- if you already know the paper handle, optionally add `paper-name.source.md` next to the PDF with frontmatter such as `paper_id` or `source`
- routing treats the `papers/` subtree as the signal, so any `raw/**/papers/*.pdf` must go through `paper-workbench`
- `kb-compile` should surface or defer those PDFs rather than compiling them as ordinary markdown captures
- the sidecar or filename handle is metadata for provenance and debugging, not the routing gate
- if `paper-workbench` is not installed, report the missing install step instead of pretending the paper was compiled

## Raw-layer rule

`raw/` is immutable from the compiler's point of view.

That means:

- put source metadata in raw notes
- put compilation metadata in summary pages
- let `kb-compile` update wiki pages, not the source file itself

## Suggested directories

- `raw/human/articles/`
- `raw/human/papers/`
- `raw/human/podcasts/`
- `raw/human/repos/`
- `raw/human/assets/`
