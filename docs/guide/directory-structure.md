# Directory Structure

Understanding the organization of a Karpathy-style knowledge base.

## Overview

After running `kb-init`, your vault looks like this:

```
vault/
├── raw/                  # Source materials (articles, papers, tweets...)
│   └── assets/           # Images from sources
├── wiki/                 # LLM-compiled wiki (don't edit manually)
│   ├── concepts/         # One article per key concept
│   ├── summaries/        # One summary per raw source
│   └── indices/          # INDEX.md, CONCEPTS.md, SOURCES.md, RECENT.md
├── outputs/              # Generated content
│   ├── reports/          # Markdown research reports
│   ├── slides/           # Marp slide decks
│   └── charts/           # Mermaid diagrams, Canvas files
└── AGENTS.md             # Schema definition for LLM agents
```

## Directory Details

### `raw/` — Source Materials

**Purpose**: Collection point for all raw information sources.

**Who writes**: Human (via Web Clipper or manual addition)

**Contents**:
- Clipped articles from the web
- Research papers (converted to markdown)
- Tweet threads
- Video transcripts
- Repository READMEs
- Book notes

**Frontmatter schema**:

```yaml
---
title: "Article Title"
source: "https://example.com/article"
author: "Author Name"
date: 2026-04-01
type: article | paper | repo | dataset | tweet | video | book | other
tags:
  - topic/subtopic
clipped_at: 2026-04-01T12:00:00
compiled_at: null          # Set by kb-compile after processing
---
```

**Important**: The `compiled_at` field is used by the compiler to determine if a source needs processing. Leave it as `null` for new sources.

### `raw/assets/` — Images and Attachments

**Purpose**: Store images, diagrams, and other media from raw sources.

**Who writes**: Web Clipper or human manually

**Usage**: Referenced in raw source files using standard markdown image syntax: `![description](assets/image.png)`

### `wiki/concepts/` — Concept Articles

**Purpose**: One article per key concept, technique, person, tool, or theme.

**Who writes**: LLM only (via `kb-compile`)

**Contents**: Comprehensive articles that grow as more sources are compiled. Each concept should be:
- Self-contained and understandable on its own
- Linked to related concepts via wikilinks
- Traced back to original sources

**Frontmatter schema**:

```yaml
---
title: "Concept Name"
aliases:
  - "Alternative Name"
category: "Parent Category"
tags:
  - concept
  - topic/subtopic
related:
  - "[[Other Concept]]"
sources:
  - "[[raw/source-file]]"
created_at: 2026-04-01T12:00:00
updated_at: 2026-04-01T12:00:00
---
```

**Article structure**:

```markdown
# Concept Name

## Definition

Clear, concise definition — 1-2 sentences.

## Overview

Comprehensive explanation drawn from all sources. This section grows as more sources are compiled.

## Key Aspects

### Aspect 1

Details from sources.

### Aspect 2

Details from sources.

## Connections

- [[Related Concept 1]] — how they relate
- [[Related Concept 2]] — how they relate

## Sources

- [[wiki/summaries/source-1]] — what this source contributes
- [[wiki/summaries/source-2]] — what this source contributes
```

### `wiki/summaries/` — Source Summaries

**Purpose**: One summary per raw source, extracting key facts, arguments, and data.

**Who writes**: LLM only (via `kb-compile`)

**Contents**: Concise distillation of each raw source, with links to relevant concepts.

**Frontmatter schema**:

```yaml
---
title: "Summary: Source Title"
source_file: "[[raw/source-file]]"
source_url: "https://example.com"
key_concepts:
  - "[[Concept A]]"
  - "[[Concept B]]"
created_at: 2026-04-01T12:00:00
---
```

### `wiki/indices/` — Index Files

**Purpose**: Navigation and overview files for the entire wiki.

**Who writes**: LLM only (via `kb-compile`)

**Files**:

| File | Purpose |
|------|---------|
| `INDEX.md` | Master index with statistics and all articles |
| `CONCEPTS.md` | Concept map grouped by category |
| `SOURCES.md` | Registry of all raw sources with compilation status |
| `RECENT.md` | Log of recent changes |

### `outputs/` — Generated Content

**Purpose**: Results from querying the knowledge base.

**Who writes**: LLM only (via `kb-query`)

**Subdirectories**:

| Directory | Contents |
|-----------|----------|
| `reports/` | Markdown research reports |
| `slides/` | Marp slide decks |
| `charts/` | Mermaid diagrams, Canvas files |

### `AGENTS.md` — Schema Definition

**Purpose**: Defines the rules, conventions, and frontmatter schemas for LLM agents operating on this knowledge base.

**Who writes**: LLM during `kb-init`, updated as needed

**Contents**:
- Directory structure documentation
- Frontmatter schemas for all file types
- Compilation rules
- File naming conventions
- Topic-specific guidance

## File Naming Conventions

| File Type | Convention | Example |
|-----------|-----------|---------|
| Raw files | Keep original name or `{date}-{slug}.md` | `2026-04-03-attention-is-all-you-need.md` |
| Concepts | `{concept-name}.md` (lowercase, hyphens) | `transformer-architecture.md` |
| Summaries | `{source-file-name}.md` | `attention-is-all-you-need.md` |
| Reports | `{date}-{topic}.md` | `2026-04-03-transformer-evolution.md` |
| Slides | `{date}-{topic}.md` | `2026-04-03-attention-mechanisms.md` |

## Next Steps

- [**AGENTS.md Schema**](/guide/agents-schema) — Deep dive into the schema
- [**kb-init Skill**](/skills/kb-init) — How initialization works
- [**Workflow Guide**](/workflow/overview) — Using the directory structure
