# Quick Start

Get your Karpathy-style knowledge base up and running in minutes.

## Prerequisites

- An Obsidian vault (or a subdirectory within one)
- Claude Code with access to the skills in this project

## Step 1: Initialize Your Knowledge Base

Run the `kb-init` skill with a topic name:

```
Initialize knowledge base for "deep learning research"
```

This will create the standard directory structure:

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

## Step 2: Add Your First Source

Add a raw source to `raw/`. You can:

- Use **Obsidian Web Clipper** browser extension
- Manually create a `.md` file with frontmatter:

```yaml
---
title: "Attention Is All You Need"
source: "https://arxiv.org/abs/1706.03762"
author: "Vaswani et al."
date: 2017-06-12
type: paper
tags:
  - transformers
  - attention
clipped_at: 2026-04-03T10:00:00
compiled_at: null
---

# Your clipped content or notes here
```

## Step 3: Compile the Wiki

Run the compilation:

```
compile wiki
```

The LLM will:
1. Scan `raw/` for new or updated sources
2. Generate summaries in `wiki/summaries/`
3. Extract and create concept articles in `wiki/concepts/`
4. Build wikilinks between related content
5. Update index files
6. Run health checks

## Step 4: Query Your Knowledge Base

Ask questions:

```
What are the key concepts in transformer architecture?
```

The LLM will:
1. Navigate the wiki indices
2. Follow wikilinks to relevant concept articles
3. Synthesize an answer with full source citations
4. Optionally archive new insights back to the wiki

## Step 5: Generate Outputs

Create reports or slides:

```
生成报告 on transformer attention mechanisms
```

```
create slides about the evolution of attention mechanisms
```

Outputs are saved to `outputs/` in various formats:
- **Markdown reports** with full citations
- **Marp slide decks** ready for presentations
- **Mermaid diagrams** rendered in Obsidian
- **Canvas files** for visual knowledge maps

## What's Next?

- [**Installation**](/guide/installation) — Detailed setup instructions
- [**Skills Overview**](/skills/overview) — Understand each skill in depth
- [**Workflow Guide**](/workflow/overview) — Master the complete pipeline
- [**Health Checks**](/workflow/health-checks) — Maintain wiki quality

## Tips for Success

1. **Start small**: Add 2-3 sources first, compile, see how it works
2. **Review the output**: Read the generated summaries and concepts
3. **Iterate**: Add more sources over time, recompile to grow the wiki
4. **Ask complex questions**: The bigger the wiki, the better the answers
5. **Archive insights**: Let the LLM add new concepts discovered during Q&A
