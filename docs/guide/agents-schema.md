# AGENTS.md Schema

The schema and rules that govern LLM agents operating on your knowledge base.

## What is AGENTS.md?

`AGENTS.md` is the **rulebook** for LLM agents working on your knowledge base. It's created during `kb-init` and defines:

- Directory structure and purposes
- Frontmatter schemas for all file types
- Compilation rules and conventions
- File naming conventions
- Topic-specific guidance

Think of it as the **"compiler specification"** for your knowledge base.

## Core Sections

### 1. Overview

Describes the knowledge base topic and purpose:

```markdown
# Knowledge Base: {Topic Name}

This is an LLM-compiled knowledge base following the Karpathy workflow pattern.
Raw sources are collected in `raw/`, then compiled by LLM into a structured wiki in `wiki/`.
The wiki is the LLM's domain — humans read it in Obsidian but rarely edit directly.
```

### 2. Directory Structure Table

Documents each directory's purpose and ownership:

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

### 3. Frontmatter Schemas

#### Raw Source Files (`raw/*.md`)

```yaml
---
title: "Article Title"              # Required: Source title
source: "https://example.com"       # Required: Original URL
author: "Author Name"               # Optional: Author(s)
date: 2026-04-01                    # Required: Publication date
type: article | paper | repo | dataset | tweet | video | book | other
tags:                               # Required: 2-5 topic tags
  - topic/subtopic
clipped_at: 2026-04-01T12:00:00    # Required: When you added it
compiled_at: null                   # Set by kb-compile after processing
---
```

#### Wiki Concept Articles (`wiki/concepts/*.md`)

```yaml
---
title: "Concept Name"               # Required: Primary name
aliases:                            # Optional: Alternative names
  - "Alternative Name"
category: "Parent Category"         # Optional: For grouping in CONCEPTS.md
tags:                               # Required: At least 'concept' tag
  - concept
  - topic/subtopic
related:                            # Optional: Bidirectional wikilinks
  - "[[Other Concept]]"
sources:                            # Required: Source files that mention this concept
  - "[[raw/source-file]]"
created_at: 2026-04-01T12:00:00    # Set on creation
updated_at: 2026-04-01T12:00:00    # Updated on each compilation
---
```

#### Wiki Summaries (`wiki/summaries/*.md`)

```yaml
---
title: "Summary: Source Title"      # Required: Prefixed with "Summary: "
source_file: "[[raw/source-file]]"  # Required: Link to raw source
source_url: "https://example.com"   # Optional: Original URL
key_concepts:                       # Required: Concepts extracted from this source
  - "[[Concept A]]"
  - "[[Concept B]]"
created_at: 2026-04-01T12:00:00    # Set on creation
---
```

### 4. Compilation Rules

Seven core rules that guide the compilation process:

1. **Incremental**: Only process raw files where `compiled_at` is null or older than file mtime
2. **Summaries**: One summary per raw source, extracting key facts, arguments, and data
3. **Concepts**: Extract distinct concepts, create/update dedicated articles
4. **Wikilinks**: Always use `[[wikilinks]]` to connect related content
5. **Indices**: Keep INDEX.md, CONCEPTS.md, SOURCES.md, RECENT.md up to date
6. **No duplication**: If a concept already exists, update it rather than creating a new file
7. **Citations**: Always trace claims back to `[[raw/source]]` references

### 5. File Naming Conventions

| File Type | Convention | Example |
|-----------|-----------|---------|
| Raw files | Keep original name, or `{date}-{slug}.md` | `2026-04-03-attention-is-all-you-need.md` |
| Concepts | `{concept-name}.md` (lowercase, hyphens for spaces) | `transformer-architecture.md` |
| Summaries | `{source-file-name}.md` (matching the raw file name) | `attention-is-all-you-need.md` |
| Reports | `{date}-{topic}.md` | `2026-04-03-transformer-evolution.md` |
| Slides | `{date}-{topic}.md` | `2026-04-03-attention-mechanisms.md` |

## Customization

You can customize `AGENTS.md` for your specific knowledge base:

### Topic-Specific Guidance

Add a section with domain-specific conventions:

```markdown
## Domain Conventions: Deep Learning Research

- Papers should include model architecture details in summaries
- Concepts should distinguish between algorithm variants (e.g., "Multi-Head Attention" vs "Cross-Attention")
- Always note dataset and benchmark results when mentioned
- Track evolution of techniques chronologically
```

### Custom Tags

Define your own tagging taxonomy:

```markdown
## Tag Taxonomy

- `architecture/*` — Model architectures (transformer, cnn, rnn)
- `training/*` — Training methods (supervised, self-supervised, RL)
- `application/*` — Application domains (NLP, CV, robotics)
- `concept` — Always added to concept articles
```

### Additional Frontmatter Fields

Add fields specific to your domain:

```yaml
# For paper sources
benchmark_results:
  - dataset: "ImageNet"
    metric: "Top-1 Accuracy"
    value: "85.3%"

# For concept articles
complexity: "beginner | intermediate | advanced"
```

## Best Practices

1. **Read AGENTS.md first**: Always review it before running `kb-compile` or `kb-query`
2. **Update when needed**: Modify schemas as your knowledge base evolves
3. **Keep it consistent**: Don't change schemas mid-compilation without recompiling
4. **Document conventions**: Add notes about naming, tagging, and categorization choices

## Next Steps

- [**Skills Overview**](/skills/overview) — The three core skills
- [**kb-init**](/skills/kb-init) — How AGENTS.md is created
- [**kb-compile**](/skills/kb-compile) — How compilation uses the schema
