# Directory Structure

The vault is intentionally split into immutable sources, compiled knowledge, and generated outputs.

```text
vault/
├── raw/
│   ├── *.md
│   ├── articles/
│   ├── papers/
│   ├── podcasts/
│   ├── assets/
│   └── repos/              # optional
├── wiki/
│   ├── concepts/
│   ├── summaries/
│   ├── indices/
│   ├── entities/           # optional
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
└── CLAUDE.md
```

## Ownership model

| Path | Owner | Purpose |
| --- | --- | --- |
| `raw/` | Human | Immutable source material and inbox-style captures |
| `wiki/summaries/` | LLM via `kb-compile` | Source-level summaries with tracking metadata |
| `wiki/concepts/` | LLM via `kb-compile` and `kb-query` | Durable concept pages that accumulate evidence |
| `wiki/entities/` | LLM when enabled | Stable pages for named people, orgs, products, or repos |
| `wiki/indices/` | LLM-derived | Navigation surfaces such as `INDEX.md`, `CONCEPTS.md`, `SOURCES.md`, and `RECENT.md` |
| `wiki/index.md` | LLM-derived | Content-oriented entry page for the compiled wiki |
| `wiki/log.md` | LLM-appended | Append-only history for `ingest`, `query`, `publish`, and `health` events |
| `outputs/qa/` | LLM via `kb-query` | Persistent research answers |
| `outputs/health/` | LLM via `kb-health` | Scored maintenance reports |
| `outputs/content/` and sibling folders | LLM via `kb-query` | Publishable derivatives grounded in the wiki |

## Optional expansions

- `raw/repos/` is useful for repo snapshots or companion notes. It is not required for every vault.
- `wiki/entities/` is useful when named entities need stable pages. Enable it only when the domain warrants it.
- When `wiki/entities/` exists, `wiki/indices/ENTITIES.md` should exist as well.

## Naming and compatibility rules

- Prefer lowercase kebab-case filenames.
- Prefer the canonical `wiki/indices/` folder name.
- Tolerate legacy vaults that still use `wiki/indexes/`.
- Keep alias-style wikilinks out of Markdown table cells because they render poorly in Obsidian.

## Important consequence

Do not track compilation state in raw notes. Store it in summary frontmatter, derived indices, logs, or health outputs instead.
