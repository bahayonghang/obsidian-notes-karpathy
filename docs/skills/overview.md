# Skills Overview

This bundle has one package entry skill and five operational skills.

## Choose by lifecycle signal

| Signal in the vault or request | Route to | Why |
| --- | --- | --- |
| The support layer is missing, partial, or still in a legacy-layout | `kb-init` | Later skills depend on the review-gated contract being present |
| New or changed raw captures are waiting | `kb-compile` | The draft layer is behind the evidence layer |
| Draft knowledge is waiting or briefings are stale and should be rebuilt in the next review pass | `kb-review` | The immediate next safe step is to gate a specific draft package or rebuild briefings |
| The live wiki exists and the user wants an answer or deliverable | `kb-query` | The task is retrieval, synthesis, archival, or publishing |
| The approved layer feels stale, contradictory, or disconnected, or backlog pressure has become a maintenance problem | `kb-health` | The task is longer-horizon maintenance, diagnosis, or safe mechanical repair across approved surfaces |
| The user talks about the Obsidian vault workflow as a whole or asks what to do next | `obsidian-notes-karpathy` | The package entry skill diagnoses lifecycle state first |

## Shared contract across all skills

- `raw/` is immutable evidence intake.
- `raw/` also acts as the durable source-library surface; creator-facing drafting and publish work should happen downstream rather than rewriting captures.
- raw paper PDFs under `raw/**/papers/*.pdf` remain a `paper-workbench` routing exception, not a normal `kb-compile` ingest.
- `MEMORY.md` is collaboration memory, not topic truth.
- `wiki/drafts/` is reviewable, not queryable truth.
- `wiki/live/` is the approved long-term brain.
- `wiki/briefings/` is generated from live only.
- `outputs/reviews/` stores promotion decisions.
- optional governance views such as `wiki/live/indices/QUESTIONS.md`, `GAPS.md`, and `ALIASES.md` are recommended for mature vaults, but they must stay grounded in approved live pages rather than archived outputs.
- archived `outputs/qa/` and `outputs/content/` can feed maintenance signals, writeback backlog, and follow-up routing, but they do not become approved truth automatically.
- curated hubs or editorial planning surfaces may exist, but they stay navigation/maintenance layers rather than truth-boundary shortcuts.
- `wiki/index.md` is the content-first entry surface.
- `wiki/log.md` is the time-first entry surface.
- legacy-layout vaults migrate through `kb-init` before normal operation.

For design rationale and forward direction, see [Architecture](/architecture/overview).

## Companion skills

These docs also list companion skills such as `obsidian-cli`, `obsidian-markdown`, and `obsidian-canvas-creator`.

They are ecosystem references for adjacent Obsidian workflows, not part of this repository's shipped core 1+5 bundle.
