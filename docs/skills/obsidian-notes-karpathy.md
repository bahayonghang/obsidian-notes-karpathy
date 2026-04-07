# obsidian-notes-karpathy

Package-level entry skill for the Karpathy-style workflow.

## Responsibility

This skill does not execute the whole workflow itself. It diagnoses the vault's lifecycle stage and routes to the correct operational skill.

When the operation is already explicit, the package skill should get out of the way and let the matching `kb-*` skill trigger directly.

## Routing matrix

| Detected signal | Route to | Expected next move |
| --- | --- | --- |
| Missing `raw/`, `wiki/`, `outputs/`, or support files | `kb-init` | Create or repair the contract first |
| New or changed notes under `raw/`, or PDF papers under `raw/papers/` | `kb-compile` | Update summaries, concepts, indices, and logs |
| A question, report, article, thread, or slide request | `kb-query` | Ground the answer in the compiled layer and archive it when substantive |
| Drift, contradiction, stale claims, broken indices, or weak links | `kb-health` | Score the vault and separate safe fixes from judgment calls |

## What it inspects before routing

- shared references under `skills/obsidian-notes-karpathy/references/`
- the shared lifecycle matrix
- `skills/obsidian-notes-karpathy/scripts/detect_lifecycle.py` when available
- local `AGENTS.md`
- local `CLAUDE.md` when present
- the top of `wiki/index.md`
- recent entries in `wiki/log.md`

When pending paper PDFs are found under `raw/papers/`, the expected compile behavior is: prefer `alphaxiv-paper-lookup`, then fall back to `pdf`, then report missing companion skills if needed.

## Why the router matters

Broad prompts such as "set up a living book in Obsidian" or "my notes are rotting" are easy to under-specify. The package entry skill makes the workflow discoverable without forcing the user to know the lifecycle vocabulary in advance.

## Package doctrine

- treat `raw/` as immutable input
- treat `wiki/` as the LLM-owned compiled artifact
- treat `outputs/qa/` as persistent research memory
- keep `wiki/index.md` and `wiki/log.md` as complementary entry surfaces
- prefer markdown-first navigation before heavier retrieval infrastructure

## Use this page when

Use the package entry skill whenever the user is talking about the workflow as a whole, wants a recommendation about the next operation, or gives symptoms instead of a concrete lifecycle command.
