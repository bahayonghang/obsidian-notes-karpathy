# Introduction

Obsidian Notes Karpathy is a bundled Obsidian workflow for maintaining a personal knowledge base with LLMs.

## The core idea

Treat knowledge management like compilation:

```text
raw/ (immutable sources) -> kb-compile -> wiki/ (compiled artifact) -> kb-query / kb-health -> outputs/
```

Humans curate sources. The LLM maintains the wiki and produces reusable outputs.

## What makes this different from ordinary RAG

- the wiki is persistent
- links and summaries accumulate over time
- substantive Q&A is archived to `outputs/qa/`
- publishable content can be generated from the compiled wiki and linked back to supporting Q&A
- the system can be linted and maintained like a codebase

## Package shape

| Skill | Role |
|-------|------|
| `obsidian-notes-karpathy` | package entry and routing |
| `kb-init` | initialize the vault contract |
| `kb-compile` | compile raw sources into wiki pages |
| `kb-query` | search, answer, and generate outputs |
| `kb-health` | deep health and maintenance pass |

## Key artifacts

- `wiki/index.md` for content-oriented navigation
- `wiki/log.md` for append-only activity history
- `outputs/qa/` for persistent research answers
- `outputs/health/` for scored maintenance reports
- `outputs/content/` for articles, threads, and talk outlines

## Next steps

- [Quick Start](/guide/quick-start)
- [Installation](/guide/installation)
- [Skills Overview](/skills/overview)
- [Workflow Guide](/workflow/overview)
