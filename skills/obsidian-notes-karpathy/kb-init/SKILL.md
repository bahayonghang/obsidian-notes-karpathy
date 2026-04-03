---
name: kb-init
description: Initialize a Karpathy-style LLM knowledge base in an Obsidian vault. Creates the standard directory structure (raw/, wiki/, outputs/), generates AGENTS.md schema, and sets up index files. Use this skill when the user wants to set up a new knowledge base, mentions "kb init", "initialize knowledge base", "karpathy setup", or wants to create a structured wiki workspace for LLM-driven knowledge management. This is a one-time setup — run it once per vault/project.
---

# KB Init — Knowledge Base Initialization

Initialize a Karpathy-style LLM knowledge base: raw sources → LLM-compiled wiki → Q&A & outputs.

This skill is inspired by Andrej Karpathy's workflow where raw data from various sources is collected, then "compiled" by an LLM into a .md wiki, then operated on by various tools to do Q&A and incrementally enhance the wiki — all viewable in Obsidian. The user rarely edits the wiki manually; it's the domain of the LLM.

## When to Use

- User says "初始化知识库", "kb init", "create knowledge base", "karpathy setup"
- User wants to start a new LLM-driven knowledge base project
- User is setting up Obsidian for structured knowledge management with LLM compilation

## Workflow

### Step 1: Determine the vault root

Ask the user which directory to initialize. This should be an Obsidian vault root or a subdirectory within a vault dedicated to this knowledge base topic.

If the user provides a topic name (e.g., "deep learning research"), use it to customize the AGENTS.md and index files.

### Step 2: Create directory structure

Create the following directories. Use the filesystem tools (Write/Bash) to create them:

```
{root}/
├── raw/                     # Raw source materials
│   └── assets/              # Images and attachments from raw sources
├── wiki/                    # LLM-compiled wiki (DO NOT manually edit)
│   ├── concepts/            # Concept articles (one per key concept)
│   ├── summaries/           # Summaries of each raw source
│   └── indices/             # Index and navigation files
├── outputs/                 # Query results and generated content
│   ├── reports/             # Research reports
│   ├── slides/              # Marp slide decks
│   └── charts/              # Mermaid diagrams and visualizations
└── AGENTS.md                # Schema definition for LLM agents
```

### Step 3: Generate AGENTS.md

Create `AGENTS.md` at the root. This file defines the schema and rules for LLM agents operating on this knowledge base. Customize the `topic` field based on user input.

Use the `obsidian-markdown` skill for proper Obsidian Flavored Markdown syntax (wikilinks, frontmatter, callouts, etc.).

```markdown
# Knowledge Base: {Topic Name}

## Overview

This is an LLM-compiled knowledge base following the Karpathy workflow pattern.
Raw sources are collected in `raw/`, then compiled by LLM into a structured wiki in `wiki/`.
The wiki is the LLM's domain — humans read it in Obsidian but rarely edit directly.

## Directory Structure

| Directory | Purpose | Who writes |
|-----------|---------|------------|
| `raw/` | Source materials (articles, papers, clips) | Human (via Web Clipper, manual) |
| `raw/assets/` | Images and attachments from sources | Human / Web Clipper |
| `wiki/concepts/` | Concept articles (one per key concept) | LLM only |
| `wiki/summaries/` | One summary per raw source | LLM only |
| `wiki/indices/` | Index files for navigation | LLM only |
| `outputs/reports/` | Research reports from Q&A | LLM only |
| `outputs/slides/` | Marp slide decks | LLM only |
| `outputs/charts/` | Diagrams and visualizations | LLM only |

## Frontmatter Schema

### Raw source files (`raw/*.md`)

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

### Wiki concept articles (`wiki/concepts/*.md`)

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

### Wiki summary files (`wiki/summaries/*.md`)

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

## Compilation Rules

1. **Incremental**: Only process raw files where `compiled_at` is null or older than file mtime
2. **Summaries**: One summary per raw source, extracting key facts, arguments, and data
3. **Concepts**: Extract distinct concepts, create/update dedicated articles
4. **Wikilinks**: Always use `[[wikilinks]]` to connect related content
5. **Indices**: Keep INDEX.md, CONCEPTS.md, SOURCES.md, RECENT.md up to date
6. **No duplication**: If a concept already exists, update it rather than creating a new file
7. **Citations**: Always trace claims back to `[[raw/source]]` references

## File Naming Conventions

- Raw files: Keep original name from Web Clipper, or use `{date}-{slug}.md`
- Concepts: `{concept-name}.md` (lowercase, hyphens for spaces)
- Summaries: `{source-file-name}.md` (matching the raw file name)
- Reports: `{date}-{topic}.md`
- Slides: `{date}-{topic}.md`
```

### Step 4: Create initial index files

Create these starter index files:

**`wiki/indices/INDEX.md`**:
```markdown
---
title: Knowledge Base Index
updated_at: {current datetime}
---

# Knowledge Base Index

> [!info] Auto-maintained
> This index is automatically maintained by `kb-compile`. Do not edit manually.

## Statistics

- Total sources: 0
- Total concepts: 0
- Total summaries: 0
- Last compiled: Never

## All Articles

_No articles yet. Run `kb-compile` after adding sources to `raw/`._
```

**`wiki/indices/CONCEPTS.md`**:
```markdown
---
title: Concept Map
updated_at: {current datetime}
---

# Concept Map

> [!info] Auto-maintained
> This concept map is automatically maintained by `kb-compile`.

_No concepts yet. Run `kb-compile` after adding sources to `raw/`._
```

**`wiki/indices/SOURCES.md`**:
```markdown
---
title: Source Registry
updated_at: {current datetime}
---

# Source Registry

> [!info] Auto-maintained
> This registry tracks all raw sources and their compilation status.

| Source | Type | Date | Compiled |
|--------|------|------|----------|
| _No sources yet_ | | | |
```

**`wiki/indices/RECENT.md`**:
```markdown
---
title: Recent Updates
updated_at: {current datetime}
---

# Recent Updates

> [!info] Auto-maintained
> Shows the most recent changes to the knowledge base.

_No updates yet._
```

### Step 5: Confirm initialization

Report to the user:
- What was created (directory tree)
- How to add raw sources (Web Clipper or manual)
- Next steps: add sources to `raw/`, then run `kb-compile`

## Important Notes

- Use `obsidian-markdown` skill for all Markdown syntax (wikilinks, callouts, properties)
- If Obsidian CLI is available (`obsidian-cli` skill), use it for file creation within the vault
- Otherwise, use Write tool directly to create files
- Always use UTF-8 encoding
- If the directory already has content, warn the user before overwriting
