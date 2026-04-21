# Web Export Template

Use this reference when `kb-query` runs in `web` mode to export a static browseable site from the approved live layer.

This template covers what `render-template.md` deliberately omits. Static web export is owned by `kb-query`, not `kb-render`, because it is a navigational surface over approved knowledge, not a deterministic outward artifact.

## Output root

- package root: `outputs/web/{slug}/`
- entrypoint: `outputs/web/{slug}/index.html`
- per-page path: `outputs/web/{slug}/pages/{page-slug}.html`
- search manifest: `outputs/web/{slug}/search.json`
- asset folder: `outputs/web/{slug}/assets/`

A single export is self-contained. Do not cross-link between different `{slug}` packages.

## Slug rules

- `{slug}` is a stable lowercase kebab-case identifier, derived from user intent or a primary live topic.
- `{page-slug}` mirrors the approved live page's basename, preserving kebab-case.
- Do not mint new slugs for pages that already exist under `wiki/live/`; reuse the live basename so future exports stay stable.
- If a live page is renamed, the export should follow — never maintain a divergent slug history.

## Source boundary

- only read `wiki/live/**`, `wiki/live/indices/**`, `wiki/live/topics/**`, `wiki/live/procedures/**`, and approved `wiki/briefings/**`
- never read `wiki/drafts/**` or `raw/**`
- archived `outputs/qa/**` may be referenced only when the Q&A is already grounded in approved live pages; cite the live page, not the archive
- `MEMORY.md` is not a web export source

## Per-page frontmatter

Each exported page must declare:

- `title`
- `canonical_slug`
- `source_live_page` (relative path under `wiki/live/`)
- `exported_at`
- `last_source_check_at`
- `export_mode: web`
- optional `topic_hub` when the page belongs to a curated hub
- optional `visibility_scope` when the page is intentionally private or shared

## Navigation structure

- `index.html` shows the top-level table of contents derived from `wiki/live/indices/INDEX.md` or the curated hub list
- left or top nav mirrors `wiki/live/indices/TOPICS.md` when present, else the approved concept list
- every page carries a "source" link back to its `wiki/live/` path so provenance stays one click away
- breadcrumbs follow topic -> concept -> page when the live graph supports it
- search is keyword-only; populate `search.json` with `{title, slug, summary, tags}` per page

## Asset handling

- render markdown to HTML at export time; do not copy raw Obsidian-style wikilinks
- convert alias-style wikilinks into plain anchor tags pointing to the exported `{page-slug}.html`
- inline images and diagrams resolve through `assets/`
- do not pull binary assets from `raw/**`; use only assets already linked from approved live pages

## Metadata footer

Every exported page should end with a small metadata footer:

- approved-at date
- last-reviewed-at date
- source live page link
- export run id (matches the `query` log entry in `wiki/log.md`)

## Log and writeback

- append a `query` entry in `wiki/log.md` with `mode: web`, `slug`, `page_count`, and `source_live_pages`
- if the export surfaces durable follow-up (gaps, alias splits, stale pages), record them as `writeback_candidates` in the run log, not inside the exported HTML
- web exports must not claim or promote knowledge; they are read-only views over approved truth

## Boundary with kb-render

- `kb-render` still owns slides, reports, chart briefs, and canvas
- `kb-query` owns web export
- if the user asks for "a report site", disambiguate: deterministic report goes to `kb-render`; browseable site stays here

## Minimal output checklist

Before handing the export back to the user, confirm:

- `index.html` renders and links to every exported page
- every page carries the required frontmatter fields
- no draft or raw content leaked into the package
- `search.json` covers all exported pages
- the `wiki/log.md` `query` entry references the export
