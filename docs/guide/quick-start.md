# Quick Start

Use this page when you want a minimal, working vault loop in one session.

## 1. Create or repair the vault contract

Start with the package entry skill or route directly to `kb-init`.

Expected support layer:

```text
raw/{articles,papers,podcasts,assets}
wiki/{concepts,summaries,indices}
wiki/index.md
wiki/log.md
outputs/{qa,health,reports,slides,charts,content/{articles,threads,talks}}
AGENTS.md
CLAUDE.md
```

`kb-init` should create both files. After initialization, `AGENTS.md` remains the required local contract, while a missing `CLAUDE.md` should be treated as repair work rather than a reason to stop compile/query/health.

Optional layers are enabled only when needed:

- `raw/repos/` for repo snapshots or companion notes
- `wiki/entities/` plus `wiki/indices/ENTITIES.md` for durable named entities

## 2. Add 5 to 10 real sources

Place markdown notes under `raw/`, `raw/articles/`, `raw/papers/`, or `raw/podcasts/`.

`raw/papers/` also accepts paper PDFs. You may place an optional `paper-name.source.md` sidecar next to the PDF when you want deterministic metadata such as `paper_id` or `source`.

When the source is a PDF:

- always use `paper-workbench` in `json` mode when the PDF lives under `raw/papers/`
- use the sidecar or filename only to capture deterministic metadata such as `paper_handle`
- if `paper-workbench` is unavailable, skip only that PDF and show install guidance instead of falling back to `pdf`

If you want to read or critique the paper outside compilation, keep using `paper-workbench` as the public entrypoint: `interpret` for explanation and `xray` for deconstruction.

Raw notes should keep only source metadata:

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

Do not put compilation state in raw notes.

Do not keep both `paper-name.md` and `paper-name.pdf` with the same basename under `raw/papers/`.

## 3. Compile in small batches

Run `kb-compile` after each batch of about 5 sources.

You should see:

- summaries in `wiki/summaries/`
- concept pages in `wiki/concepts/`
- optional entity pages when the entity layer is warranted
- rebuilt `wiki/index.md`, `wiki/log.md`, and `wiki/indices/*`

## 4. Ask one substantive question

Run `kb-query` after the first compilation pass.

The skill should:

- search the compiled wiki first
- check prior answers in `outputs/qa/`
- answer with provenance
- archive a substantive result to `outputs/qa/`

## 5. Publish one derivative artifact

Use `kb-query` in publish mode when you want an outward-facing deliverable.

Typical targets:

- `outputs/reports/`
- `outputs/slides/`
- `outputs/charts/`
- `outputs/content/articles/`
- `outputs/content/threads/`
- `outputs/content/talks/`

## 6. Run a health baseline

Use `kb-health` once the first round-trip is complete.

The report should land in `outputs/health/health-check-{date}.md` and score the vault across completeness, consistency, connectivity, freshness, and provenance.

## Minimal bootstrap loop

1. create the support layer with `kb-init`
2. add a small batch of sources
3. compile with `kb-compile`
4. ask a real research question with `kb-query`
5. turn one answer into a publishable artifact if relevant
6. run `kb-health` to establish a maintenance baseline
