# Schema Template Notes

Keep `AGENTS.md` and `CLAUDE.md` aligned. They should describe the same vault contract with only minimal agent-specific wrapper text.

Use ISO dates whenever possible:

- date only: `YYYY-MM-DD`
- datetime: `YYYY-MM-DDTHH:mm:ssZ`

Use one property vocabulary consistently across the vault. Global property consistency matters because Obsidian Properties view and property search depend on it.

## Raw source frontmatter

```yaml
---
title: "Source Title"
source: "https://example.com"
author: "Author Name"
date: 2026-04-01
type: article | paper | repo | dataset | tweet | video | book | podcast | other
tags:
  - topic/subtopic
clipped_at: 2026-04-01T12:00:00Z
---
```

Notes:

- Do not add compilation-state fields here.
- If metadata is missing, leave the field blank or omit it; do not hallucinate.

## Summary frontmatter

```yaml
---
title: "Summary: Source Title"
source_file: "[[raw/articles/2026-04-01-source-title]]"
source_url: "https://example.com"
source_type: article
source_mtime: "2026-04-01T12:00:00Z"
source_hash: "optional-stable-hash"
compiled_at: "2026-04-02T08:00:00Z"
key_concepts:
  - "[[retrieval-augmented-generation]]"
  - "[[markdown-index]]"
key_entities:
  - "[[wiki/entities/andrej-karpathy]]"
---
```

Rules:

- `source_hash` is preferred when the environment supports deterministic hashing.
- If both hash and mtime are available, keep both.
- `key_concepts` should reference real concept pages or clearly planned new ones.
- `key_entities` is optional and should be used only when named entities deserve stable pages.

## Concept frontmatter

```yaml
---
title: "Retrieval-Augmented Generation"
concept_id: "retrieval-augmented-generation"
aliases:
  - "RAG"
updated_at: "2026-04-05T09:00:00Z"
status: active | draft | conflicting
sources:
  - "[[wiki/summaries/2026-04-01-rag-vs-markdown]]"
related:
  - "[[wiki/concepts/markdown-index]]"
  - "[[wiki/concepts/vector-search]]"
---
```

Rules:

- `concept_id` should match the concept filename basename.
- `aliases` should capture common abbreviations and alternate spellings.
- Set `status: conflicting` when the concept contains unresolved tension across sources.

## Entity frontmatter

```yaml
---
title: "Andrej Karpathy"
entity_id: "andrej-karpathy"
entity_type: person
aliases:
  - "Karpathy"
updated_at: "2026-04-05T09:00:00Z"
status: active | draft | conflicting
sources:
  - "[[wiki/summaries/2026-04-04-llm-wiki]]"
related:
  - "[[wiki/concepts/knowledge-compilation]]"
  - "[[wiki/entities/qmd]]"
---
```

Rules:

- `entity_id` should match the entity filename basename.
- `entity_type` should stay small and stable.
- Only create entity pages for named things with durable value in the vault.

## Q&A frontmatter

```yaml
---
question: "When should I use markdown indices instead of RAG?"
asked_at: "2026-04-05T11:15:00Z"
sources:
  - "[[wiki/concepts/retrieval-augmented-generation]]"
  - "[[wiki/entities/qmd]]"
  - "[[wiki/summaries/2026-04-01-rag-vs-markdown]]"
tags:
  - qa
  - retrieval
---
```

## Health report frontmatter

```yaml
---
title: "Health Check Report"
date: "2026-04-05T12:00:00Z"
scope: "wiki/, outputs/qa/"
health_score: 84
---
```

## Publish artifact frontmatter

```yaml
---
title: "Why Markdown Indices Beat Premature RAG"
created_at: "2026-04-05T13:00:00Z"
artifact_type: article | thread | newsletter | talk-outline | report | slide-deck | chart
sources:
  - "[[wiki/concepts/retrieval-augmented-generation]]"
  - "[[wiki/entities/qmd]]"
  - "[[outputs/qa/2026-04-05-rag-vs-markdown]]"
derived_from:
  - "[[outputs/qa/2026-04-05-rag-vs-markdown]]"
---
```

## Log entry header

Use a parseable heading prefix:

```markdown
## [2026-04-05] ingest | source-title
## [2026-04-05] query | rag-vs-markdown
## [2026-04-05] publish | why-markdown-indices-beat-premature-rag
## [2026-04-05] health | weekly-check
```

## Naming rules

- filenames should use lowercase kebab-case
- frontmatter `title` should stay human-readable
- concept aliases belong in `aliases`, not in duplicated concept pages
- entity aliases belong in `aliases`, not in duplicated entity pages
- if a concept gets renamed, preserve the old term in `aliases`
