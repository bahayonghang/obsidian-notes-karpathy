# Skills Overview

This bundle has one package entry skill and five operational skills.

## Choose by lifecycle signal

| Signal in the vault or request | Route to | Why |
| --- | --- | --- |
| The support layer is missing, partial, or still V1 | `kb-init` | Later skills depend on the V2 contract being present |
| New or changed raw captures are waiting | `kb-compile` | The draft layer is behind the evidence layer |
| Draft knowledge is waiting or briefings are stale | `kb-review` | The review gate is the next safe step |
| The live wiki exists and the user wants an answer or deliverable | `kb-query` | The task is retrieval, synthesis, archival, or publishing |
| The approved layer feels stale, contradictory, or disconnected | `kb-health` | The task is maintenance, diagnosis, or repair |
| The user talks about the workflow as a whole or asks what to do next | `obsidian-notes-karpathy` | The package entry skill diagnoses lifecycle state first |

## Shared contract across all skills

- `raw/` is immutable evidence intake.
- `wiki/drafts/` is reviewable, not queryable truth.
- `wiki/live/` is the approved long-term brain.
- `wiki/briefings/` is generated from live only.
- `outputs/reviews/` stores promotion decisions.
- `wiki/index.md` is the content-first entry surface.
- `wiki/log.md` is the time-first entry surface.
