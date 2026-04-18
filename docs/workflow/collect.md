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

### web-access or browser-side collection

Use this before `kb-ingest` when the source is still on the web and has not been captured into `raw/` yet.

- `web-access` is the preferred upstream companion when the site needs real-browser navigation, login state, or asset-aware collection
- after collection, commit the markdown capture and any linked local assets into `raw/`
- let `kb-ingest` register the capture rather than writing manifest entries by hand

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
- put browser/CDP or Web Clipper provenance into manifest metadata such as `capture_method`
- keep local images or attachments visible through `linked_assets` when the markdown depends on them
- put compilation metadata in summary pages
- let `kb-compile` update wiki pages, not the source file itself

## Suggested directories

- `raw/human/articles/`
- `raw/human/papers/`
- `raw/human/podcasts/`
- `raw/human/repos/`
- `raw/human/assets/`
