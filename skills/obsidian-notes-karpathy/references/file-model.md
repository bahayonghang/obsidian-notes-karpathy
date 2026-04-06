# File Model

Canonical knowledge-base layering:

```text
raw/     -> immutable source materials curated by the human
wiki/    -> compiled markdown wiki maintained by the LLM
outputs/ -> generated research memory, health reports, and publishable artifacts
```

Treat the vault like a codebase:

- `raw/` is source code.
- `wiki/` is the compiled artifact.
- `outputs/` is runtime output plus durable deliverables.
- `AGENTS.md` is required local guidance.
- `CLAUDE.md` is recommended and should mirror `AGENTS.md` when present, but the workflow must not hard-fail if it is absent.

## Core rules

1. `raw/` is read-only from the workflow's point of view.
2. Compilation state lives in `wiki/` frontmatter or `outputs/health/`, never in raw source files.
3. `wiki/index.md` is the content-oriented landing page.
4. `wiki/log.md` is the chronological append-only activity log for `ingest`, `query`, `publish`, and `health`.
5. `wiki/indices/` is the canonical derived-navigation directory.
6. If a vault already uses `wiki/indexes/`, preserve it and read from it rather than forcing a rename.
7. `outputs/qa/` stores durable research answers, not disposable chat residue.
8. `outputs/content/` stores publishable derivatives such as article drafts, threads, and talk outlines.

## Expected directories

```text
vault/
├── raw/
│   ├── *.md            # also valid for direct captures or inbox-style source notes
│   ├── articles/
│   ├── papers/
│   ├── podcasts/
│   ├── assets/
│   └── repos/          # optional
├── wiki/
│   ├── concepts/
│   ├── summaries/
│   ├── indices/
│   ├── entities/       # optional
│   ├── index.md
│   └── log.md
├── outputs/
│   ├── qa/
│   ├── health/
│   ├── reports/
│   ├── slides/
│   ├── charts/
│   └── content/
│       ├── articles/
│       ├── threads/
│       └── talks/
├── AGENTS.md
└── CLAUDE.md           # optional but recommended
```

Optional directories are opt-in expansions:

- `raw/repos/` for repo snapshots, README exports, or repo companion notes
- `wiki/entities/` for people, organizations, products, projects, or repos that deserve stable pages

Accepted raw discovery patterns:

- markdown files directly under `raw/`
- categorized markdown files under `raw/articles/`, `raw/papers/`, and `raw/podcasts/`
- optional repo companion notes or repo overviews under `raw/repos/`

If the vault already has channel-specific publishing folders such as `x/`, `公众号/`, or `小红书/`, preserve them. Do not force-migrate them unless the user asks.

## Note classes

### Raw source notes

Live under `raw/` and preserve the original material plus metadata.

Raw source notes may live directly under `raw/` or inside a typed subdirectory. The compiler should accept both without rewriting paths just to satisfy a stricter ideal layout.

Expected properties:

- `title`
- `source`
- `author`
- `date`
- `type`
- `tags`
- `clipped_at`

Never add compile state here.

### Summary notes

Live under `wiki/summaries/` and mirror one source note per file.

Expected properties:

- `title`
- `source_file`
- `source_url`
- `source_type`
- `source_mtime` or `source_hash`
- `compiled_at`
- `key_concepts`
- `key_entities` when applicable

### Concept notes

Live under `wiki/concepts/` and merge evidence across summaries and Q&A.

Expected properties:

- `title`
- `concept_id`
- `aliases`
- `updated_at`
- `status`
- `sources`
- `related`

### Entity notes

Live under `wiki/entities/` when the vault benefits from a dedicated entity layer.

Expected properties:

- `title`
- `entity_id`
- `entity_type`
- `aliases`
- `updated_at`
- `status`
- `sources`
- `related`

### Q&A notes

Live under `outputs/qa/` and capture substantial research answers.

Expected properties:

- `question`
- `asked_at`
- `sources`
- `tags`

### Health reports

Live under `outputs/health/`.

Expected properties:

- `title`
- `date`
- `scope`
- `health_score`

### Publish-mode artifacts

Live under `outputs/content/`, `outputs/reports/`, `outputs/slides/`, or `outputs/charts/` depending on deliverable type.

Expected properties:

- `title`
- `created_at`
- `artifact_type`
- `sources`
- `derived_from`

## Naming and slug rules

Use stable lowercase kebab-case paths unless the user already has a strong naming convention.

- raw source file: `YYYY-MM-DD-short-title.md` when a date is known
- summary file: same basename as the source file
- concept file: semantic slug such as `retrieval-augmented-generation.md`
- entity file: semantic slug such as `andrej-karpathy.md` or `qmd.md`
- q&a file: `YYYY-MM-DD-question-slug.md`
- health report: `health-check-YYYY-MM-DD.md`
- publish artifact: `YYYY-MM-DD-channel-slug.md`

Human-readable titles live in frontmatter and headings. File paths should stay stable even if titles get polished.

## Metadata and graph conventions

Prefer Obsidian-native graph affordances before heavier infrastructure:

- use wikilinks for summary-to-concept, summary-to-entity, and page-to-page references
- maintain `aliases` on concept and entity notes so Backlinks and unlinked mentions remain useful
- use consistent property names so the Properties view and property search stay effective
- use reciprocal `related` fields only when the relationship is real, not speculative
- treat `wiki/index.md` and `wiki/log.md` as the two top-level navigation surfaces: content-first and time-first

## Obsidian-safe markdown conventions

Follow `obsidian-safe-markdown.md` whenever you emit tables or repair rendered markdown.

- Never place alias-style wikilinks such as `[[note|Alias]]` inside Markdown table cells.
- In tables, use plain wikilinks or standard Markdown links instead.
- If a page needs many linked labels, prefer a list over a dense table.

## Incremental compilation contract

The compiler should compare each raw note against its matching summary using:

1. `source_hash` when available and deterministic
2. otherwise `source_mtime`

If neither is reliable, the compiler may fall back to a full rescan, but it must still avoid mutating `raw/`.
