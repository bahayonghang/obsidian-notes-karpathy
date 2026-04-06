# Quick Start

Get a minimal Karpathy-style knowledge base running quickly.

## 1. Initialize the vault contract

Ask the package entry skill or run `kb-init`.

This creates:

```text
raw/{articles,papers,podcasts,assets}
wiki/{concepts,summaries,indices}
wiki/index.md
wiki/log.md
outputs/{qa,health,reports,slides,charts,content/{articles,threads,talks}}
AGENTS.md
CLAUDE.md
```

## 2. Add your first raw source

Create a markdown file under `raw/articles/`, `raw/papers/`, or `raw/podcasts/`.

Use source metadata only:

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
---
```

Do not add compilation-state fields to raw notes.

## 3. Compile the wiki

Run `kb-compile`.

Expected results:

- summary pages in `wiki/summaries/`
- concept pages in `wiki/concepts/`
- rebuilt `wiki/index.md`, `wiki/log.md`, and `wiki/indices/*`

## 4. Ask a substantive question

Use `kb-query`.

Substantive answers should be archived to `outputs/qa/` by default.

## 5. Turn one answer into content

Use `kb-query` again in publish mode.

Typical targets:

- `outputs/content/articles/`
- `outputs/content/threads/`
- `outputs/content/talks/`

## 6. Run a health pass

Use `kb-health` to generate a scored report in `outputs/health/`.

## Recommended first-week loop

1. add 5-10 sources
2. compile in small batches
3. ask one or two synthesis questions
4. turn one good answer into a publishable draft
5. run a health pass
