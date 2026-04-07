---
name: kb-init
description: Initialize or repair a Karpathy-style LLM knowledge base inside an Obsidian vault. Use this skill whenever the user says "kb init", "initialize knowledge base", "Karpathy setup", "LLM Wiki setup", "setup vault", "repair vault", "fix my KB structure", "turn my Obsidian into a living book", "set up a second brain", "start a compile-first knowledge base", "初始化知识库", "新建知识库", "创建wiki", "修知识库结构", "搭建知识库编译流程", or wants a fresh vault or a partially initialized vault repaired. Prefer this skill over the package router when the task is clearly setup or repair rather than lifecycle diagnosis.
---

# KB Init

One-time setup for a Karpathy-style knowledge base.

This skill creates the vault contract that later skills depend on. The main architectural decision is unchanged: `raw/` stays immutable while the LLM owns `wiki/` and `outputs/`.

## Read before writing

Read these shared references first:

- `../obsidian-notes-karpathy/references/file-model.md`
- `../obsidian-notes-karpathy/references/lifecycle-matrix.md`
- `../obsidian-notes-karpathy/references/schema-template.md`
- `../obsidian-notes-karpathy/references/concept-template.md`
- `../obsidian-notes-karpathy/references/entity-template.md`
- `../obsidian-notes-karpathy/references/source-registry-template.md`
- `../obsidian-notes-karpathy/references/content-output-template.md`
- `../obsidian-notes-karpathy/references/clipper-template.md`
- `../obsidian-notes-karpathy/references/index-home-template.md`
- `../obsidian-notes-karpathy/references/activity-log-template.md`
- `../obsidian-notes-karpathy/references/obsidian-safe-markdown.md`

If `../obsidian-notes-karpathy/scripts/detect_lifecycle.py` exists, run it against the target vault before writing anything. Use it to distinguish fresh setup from partial repair, then preserve existing content accordingly.

## Inputs to gather

Determine:

1. target vault root
2. topic or domain name
3. whether this is a fresh vault or a dedicated subfolder inside an existing vault
4. whether the user wants publish-mode content folders now or later
5. whether optional repo or entity support is warranted now

If the target already contains files, preserve existing content and only create missing directories and templates.

If one or more shared references are unavailable, continue with the minimum compatible contract:

- preserve `raw/` immutability
- create or repair the vault support layer first
- keep schema/frontmatter aligned across local guidance files
- follow Obsidian-safe markdown rules for any generated tables

Report missing shared references in the final summary instead of silently pretending they existed.

## Repair the support layer first

If the target vault is only partially initialized, repair the KB support layer before doing any cosmetic setup.

At minimum, restore or create:

- the local guidance contract
- `wiki/index.md`
- `wiki/log.md`
- the `wiki/indices/` pages that later KB skills depend on

Treat this as safe bootstrap work, not as an exceptional path.

## Create the canonical structure

Create the minimum canonical structure:

```text
{root}/
├── raw/
│   ├── *.md            # allowed for direct captures or inbox-style source notes
│   ├── articles/
│   ├── papers/         # markdown paper notes or PDF papers
│   ├── podcasts/
│   └── assets/
├── wiki/
│   ├── concepts/
│   ├── summaries/
│   ├── indices/
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

Opt-in expansions:

- create `raw/repos/` only when the user plans to ingest repo snapshots or repo companion notes
- create `wiki/entities/` only when the domain obviously benefits from stable pages for named people, organizations, products, or repositories
- if you enable `wiki/entities/`, also create `wiki/indices/ENTITIES.md`

If the user already has channel-specific content folders, preserve them and document how they map onto the workflow.

## Generate schema files

For a fresh vault, create both `AGENTS.md` and `CLAUDE.md`.

For a repair pass over an existing vault:

- always restore or create `AGENTS.md` when it is missing
- create or refresh `CLAUDE.md` when it is missing or stale, but treat its absence as a repair target rather than a hard failure
- if the vault clearly standardizes on `AGENTS.md` only, preserve that choice and report it

Requirements:

- the two files must express the same file model and frontmatter rules
- both must say that `raw/` is immutable and compilation state is tracked in `wiki/`
- both must name `wiki/index.md` and `wiki/log.md` as first-class files
- both must explain that `outputs/qa/` stores persistent research answers
- both should mention `outputs/content/` when publish-mode artifacts are in scope
- both should mention that `raw/repos/` and `wiki/entities/` are optional expansions, not mandatory defaults
- both should preserve canonical `wiki/indices/` naming while tolerating older `wiki/indexes/` vaults
- both should include the Obsidian-safe markdown rule that alias-style wikilinks must not appear inside Markdown table cells

Use `../obsidian-notes-karpathy/references/schema-template.md` as the baseline, then customize the topic wording for the user's domain.

## Create starter files

Create these initial files:

### `wiki/index.md`

Build it from `../obsidian-notes-karpathy/references/index-home-template.md`.

Include:

- vault title
- a short explanation of the three-layer model
- statistics placeholders
- links to `wiki/indices/INDEX.md`, `wiki/indices/CONCEPTS.md`, `wiki/indices/SOURCES.md`, and `wiki/indices/RECENT.md`
- optional link to `wiki/indices/ENTITIES.md` only when the entity layer exists

### `wiki/log.md`

Include:

- append-only notice
- short note explaining the four event types: `ingest`, `query`, `publish`, and `health`
- starter examples adapted from `../obsidian-notes-karpathy/references/activity-log-template.md`

### `wiki/indices/INDEX.md`

Create a derived master index with zero-state placeholders.

### `wiki/indices/CONCEPTS.md`

Create a zero-state concept map.

### `wiki/indices/SOURCES.md`

Create a zero-state source registry using `../obsidian-notes-karpathy/references/source-registry-template.md`.

### `wiki/indices/RECENT.md`

Create a zero-state recent-updates page.

### `wiki/indices/ENTITIES.md`

Create this only when `wiki/entities/` exists.

### `wiki/concepts/_example-concept.md`

Create one example concept page using `../obsidian-notes-karpathy/references/concept-template.md` so future compile passes have a concrete contract to follow.

### `wiki/entities/_example-entity.md`

Create this only when the entity layer is enabled, using `../obsidian-notes-karpathy/references/entity-template.md`.

### `raw/articles/_web-clipper-template.md`

Write a ready-to-import clipper template using `../obsidian-notes-karpathy/references/clipper-template.md`.

## Property and naming conventions

Document:

- standardized property names from `../obsidian-notes-karpathy/references/schema-template.md`
- lowercase kebab-case filenames
- concept aliases as a first-class tool for discoverability and backlink quality
- entity aliases when the entity layer is enabled
- root-level raw notes are valid and do not need to be moved into category folders just to satisfy the workflow
- Markdown tables must obey `../obsidian-notes-karpathy/references/obsidian-safe-markdown.md`

## Output to the user

Report:

1. what was created
2. any existing content that was preserved
3. whether publish-mode folders were enabled
4. whether optional `raw/repos/` or `wiki/entities/` support was enabled
5. whether any missing support files were repaired
6. the next recommended command

Then give the two-week bootstrap:

1. clip or add 5-10 sources into `raw/articles/`, `raw/papers/`, or `raw/podcasts/`; `raw/papers/` may contain markdown paper notes or PDFs
2. if relevant, add one repo companion note into `raw/repos/`
3. run `kb-compile` for the first compilation
4. ask a substantive question with `kb-query`, which should archive into `outputs/qa/`
5. if relevant, generate one article or thread draft from `kb-query`
6. run `kb-health` for the first deep maintenance pass

## Tooling notes

- Use `obsidian-markdown` for file content and callouts.
- Use `obsidian-cli` when it is available for vault-aware file creation.
- Never add `compiled_at` or similar compilation-state fields to raw notes.
