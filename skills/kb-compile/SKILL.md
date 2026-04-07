---
name: kb-compile
description: Incrementally compile an Obsidian knowledge base from immutable raw sources. Use this skill whenever the user says "compile wiki", "compile kb", "sync wiki", "refresh wiki", "digest these articles", "turn my clips into notes", "process my repo notes", "编译wiki", "更新知识库", "同步wiki", "消化文章", "编译笔记", or wants newly added sources turned into summaries, concepts, entities, links, and indices. Also trigger when new files landed under `raw/`, including markdown files placed directly under `raw/` and PDF papers placed under `raw/papers/`, and the user asks what to do next. Prefer this skill over the package router when the operation is clearly a compile pass rather than a workflow-level diagnosis.
---

# KB Compile

Incrementally turn immutable raw notes into a maintained wiki.

This skill behaves like a compiler, not an editor of source material. Read from `raw/`, write to `wiki/` and `outputs/`, and keep provenance intact.

## Read before compiling

Read these files first:

- local `AGENTS.md`
- local `CLAUDE.md` if present
- `../obsidian-notes-karpathy/references/file-model.md`
- `../obsidian-notes-karpathy/references/lifecycle-matrix.md`
- `../obsidian-notes-karpathy/references/schema-template.md`
- `../obsidian-notes-karpathy/references/summary-template.md`
- `../obsidian-notes-karpathy/references/concept-template.md`
- `../obsidian-notes-karpathy/references/entity-template.md`
- `../obsidian-notes-karpathy/references/source-registry-template.md`
- `../obsidian-notes-karpathy/references/index-home-template.md`
- `../obsidian-notes-karpathy/references/activity-log-template.md`
- `../obsidian-notes-karpathy/references/obsidian-safe-markdown.md`

If `../obsidian-notes-karpathy/scripts/scan_compile_delta.py` exists, run it first and use its JSON result as the authoritative starting point for new, changed, unchanged, and skipped counts. Fall back to manual inspection only when the script is unavailable.

If one or more shared references are missing, do not hard-fail immediately. Continue with the minimum compatible contract:

- do not rewrite `raw/`
- parse frontmatter with a YAML-aware path when available
- preserve deterministic summary and index paths
- obey Obsidian-safe markdown rules for generated tables

Report missing shared references in the final user summary.

## Non-negotiable rules

- Do not rewrite raw source files.
- Determine incremental state by comparing raw file mtime or hash against the metadata stored in the corresponding summary page.
- Treat `wiki/index.md`, `wiki/log.md`, and `wiki/indices/*` as derived content that can be rebuilt.
- Preserve and refine existing concept pages. Do not blow them away just because a new source arrived.
- Preserve and refine entity pages when the optional entity layer exists.
- Use deterministic target paths: summaries should keep the source basename, and concepts or entities should keep stable slugs.
- Prefer canonical `wiki/indices/`; tolerate `wiki/indexes/` only when the target vault already uses it.

## Phase 1: Discover work

Scan `raw/` for supported source files, excluding `raw/assets/` and template files.

Supported source classes:

- markdown notes directly under `raw/`
- markdown notes under `raw/articles/`, `raw/papers/`, and `raw/podcasts/`
- PDF papers under `raw/papers/`
- optional repo notes under `raw/repos/`
- optional full repo folders under `raw/repos/`, but in that case compile from a top-level `README.md` or explicit overview note unless the user asked for deeper code synthesis

For PDF papers under `raw/papers/`:

1. prefer `alphaxiv-paper-lookup` when the source metadata gives you a clean paper handle such as an arXiv URL, alphaxiv URL, paper ID, or a strong title match
2. if that is unavailable or not applicable, fall back to the `pdf` skill
3. if neither companion skill is available, skip only that PDF source and report install guidance for `alphaxiv-paper-lookup` and `pdf`
4. never write extracted markdown back into `raw/`; go straight from the PDF handling step into the compiled summary, concept, and entity outputs
5. do not keep both `foo.md` and `foo.pdf` with the same basename under `raw/papers/`; pick one canonical paper source

For each raw source:

1. locate the matching summary in `wiki/summaries/`
2. if no summary exists, mark as new
3. if summary exists and its stored `source_hash` or `source_mtime` is older than the current raw file, mark as changed
4. otherwise skip

Report a short status such as:

`Found 3 new sources, 1 changed source, 9 unchanged sources.`

## Phase 2: Compile summaries

For each new or changed source:

1. if the source is markdown, read the raw markdown
2. if the source is a PDF under `raw/papers/`, derive the working paper understanding through the PDF handling chain above without mutating `raw/`
3. parse frontmatter with a YAML-aware path when metadata matters
4. inspect local images from `raw/assets/` only when they materially affect meaning
5. create or update `wiki/summaries/{source-name}.md` using `../obsidian-notes-karpathy/references/summary-template.md`
6. store tracking fields in summary frontmatter:
   - `source_hash` when available
   - otherwise `source_mtime`
   - keep both when both are available
7. include:
   - thesis
   - key takeaways
   - evidence with a concrete locator when possible
   - extracted key concepts
   - extracted key entities when the source anchors durable named things
   - related-source links where they are real, not speculative

## Phase 3: Compile concept pages and entity pages

After each summary:

1. extract recurring concepts, entities, tools, people, organizations, products, repositories, or themes
2. create new concept pages when the concept is significant or repeated
3. create or update entity pages only when the named thing is durable enough to deserve a stable page
4. otherwise keep the item in summary prose or concept aliases rather than forcing a weak entity page

When updating a concept or entity page:

- merge the new evidence into existing sections
- update `sources`, `related`, `aliases`, and `updated_at`
- preserve older evidence unless it is explicitly superseded

If new evidence conflicts with the existing page:

- do not silently overwrite the old definition
- add or update a `Tensions and Contradictions` section
- set `status: conflicting` when the disagreement remains unresolved

If durable entity coverage becomes necessary and `wiki/entities/` does not exist yet:

- create `wiki/entities/`
- create `wiki/indices/ENTITIES.md`
- then continue the compile pass

## Phase 4: Rebuild navigation surfaces

Rebuild or refresh:

- `wiki/index.md`
- `wiki/indices/INDEX.md`
- `wiki/indices/CONCEPTS.md`
- `wiki/indices/SOURCES.md`
- `wiki/indices/RECENT.md`
- `wiki/indices/ENTITIES.md` when the entity layer exists

Any table emitted during this phase must obey `../obsidian-notes-karpathy/references/obsidian-safe-markdown.md`. Never place alias-style wikilinks inside table cells.

`wiki/index.md` should stay content-oriented:

- counts
- high-level entry points
- notable concepts, entities, or recent themes
- recent activity derived from the log rather than hand-written drift

Append one batch entry to `wiki/log.md` using `../obsidian-notes-karpathy/references/activity-log-template.md`.

## Phase 5: Light sanity check

Run a lightweight post-compile check and fix obvious mechanical issues:

- broken links introduced by this compile pass
- missing reciprocal `related` fields you can infer confidently
- summaries missing `key_concepts`
- entity pages without source backing
- source registry rows that no longer match the compiled state
- alias-style wikilinks inside Markdown table cells that would break Obsidian rendering

If the user asked for a full health review, route or continue into `kb-health`.

## Large batches

For first-time or heavy ingestion:

- recommend batches of 5 sources
- update indices after each batch
- report progress in batch terms rather than waiting for the entire run to finish

## Output to the user

Always report:

1. how many sources were new, changed, or skipped
2. how many summaries were created or updated
3. how many concepts were created or updated
4. how many entities were created or updated
5. whether any contradictions were surfaced
6. whether any shared references or optional local files were missing
7. whether any PDF papers had to fall back from `alphaxiv-paper-lookup` to `pdf`, or were skipped because neither companion skill was available
8. whether a deeper `kb-health` pass is recommended

## Tooling notes

- Use `obsidian-markdown` for all markdown output.
- Use `obsidian-cli` when available for search and vault-aware operations.
- Keep health reports in `outputs/health/`, not `outputs/reports/`.
