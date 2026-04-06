# Introduction

Obsidian Notes Karpathy is a bundle of Obsidian-oriented skills for running a compile-first knowledge base inside a vault.

## What the bundle owns

```text
raw/     -> human-curated, immutable source notes
wiki/    -> compiled markdown maintained by the LLM
outputs/ -> persistent answers, reports, and publishable derivatives
```

The workflow is not "retrieve from chat memory whenever needed." It is "compile knowledge once, maintain it over time, and reuse the result."

## How this differs from ordinary RAG

| Ordinary chat or ad hoc RAG | Obsidian Notes Karpathy |
| --- | --- |
| Answers are often ephemeral | Substantive answers are archived to `outputs/qa/` |
| Retrieval happens on every question | The vault keeps a maintained `wiki/` layer |
| Structure lives mostly in prompts | Structure lives in summaries, concepts, indices, and logs |
| Maintenance is informal | `kb-health` provides a scored maintenance pass |
| Publishing is a separate manual task | `kb-query` can turn grounded notes into reports and drafts |

## Core skills

| Skill | Responsibility | Reach for it when |
| --- | --- | --- |
| `obsidian-notes-karpathy` | Package entry and lifecycle router | The user talks about the workflow as a whole or asks what to do next |
| `kb-init` | Create or repair the vault contract | The support layer is missing or partial |
| `kb-compile` | Turn raw notes into summaries, concepts, indices, and logs | New or changed source material is waiting |
| `kb-query` | Search, answer, archive, and publish from the compiled layer | The vault already knows enough and the user wants an answer or artifact |
| `kb-health` | Audit drift, contradictions, weak links, and stale outputs | The compiled layer feels unreliable or disconnected |

## Non-negotiable operating rules

- `raw/` is immutable from the compiler's point of view.
- `wiki/index.md` and `wiki/log.md` are first-class navigation surfaces.
- `outputs/qa/` stores persistent research memory, not disposable chat residue.
- The canonical index folder is `wiki/indices/`, while legacy `wiki/indexes/` vaults are tolerated.
- Search starts with markdown, backlinks, unlinked mentions, and Properties search before heavier infrastructure.

## Start paths

1. New vault or broken support layer: go to [Quick Start](/guide/quick-start) and route to `kb-init`.
2. Fresh source material under `raw/`: route to [kb-compile](/skills/kb-compile).
3. Need an answer, report, thread, or talk outline: route to [kb-query](/skills/kb-query).
4. Notes feel stale, contradictory, or disconnected: route to [kb-health](/skills/kb-health).
