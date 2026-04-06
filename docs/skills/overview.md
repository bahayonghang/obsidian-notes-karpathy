# Skills Overview

This bundle has one package entry skill and four operational skills.

## Choose by lifecycle signal

| Signal in the vault or request | Route to | Why |
| --- | --- | --- |
| The support layer is missing, partial, or obviously broken | `kb-init` | Later skills depend on the contract being present |
| New or changed sources are waiting under `raw/` | `kb-compile` | The compiled layer is behind the source layer |
| The compiled wiki exists and the user wants an answer or deliverable | `kb-query` | The task is extraction, synthesis, archival, or publishing |
| The compiled layer feels stale, contradictory, or disconnected | `kb-health` | The task is maintenance, diagnosis, or repair |
| The user talks about the workflow as a whole or asks what to do next | `obsidian-notes-karpathy` | The package entry skill diagnoses lifecycle state first |

## Skill matrix

| Skill | Primary job | Reads first | Writes to |
| --- | --- | --- | --- |
| `obsidian-notes-karpathy` | Diagnose lifecycle stage and route | shared references, local guidance, top of `wiki/index.md`, recent `wiki/log.md` | routing recommendation only |
| `kb-init` | Create or repair the vault contract | shared templates and file model references | support layer, starter indices, local guidance files |
| `kb-compile` | Turn raw notes into maintained wiki pages | shared templates, local guidance, raw sources | `wiki/`, derived indices, `wiki/log.md` |
| `kb-query` | Search, answer, archive, and publish from the compiled layer | shared references, `wiki/index.md`, indices, prior Q&A | `outputs/`, `wiki/log.md`, sometimes `wiki/` |
| `kb-health` | Score and audit the compiled layer | shared health rubric, local guidance, compiled layers | `outputs/health/`, `wiki/log.md`, safe mechanical fixes |

## Shared contract across all skills

- `raw/` is immutable from the compiler's point of view.
- `wiki/` is the maintained, compiled artifact.
- `outputs/qa/` stores substantive answers by default.
- `wiki/index.md` is the content-first entry surface.
- `wiki/log.md` is the time-first entry surface.
- Search should stay markdown-first until the vault genuinely needs a heavier upgrade.

## Bundled support material

The entry skill also ships:

- `references/` for schema, lifecycle matrix, summary, concept, entity, Q&A, content, and health templates
- `scripts/` for deterministic lifecycle detection, compile-delta scanning, and mechanical linting
- `evals/evals.json` for package-level regression prompts
- `evals/trigger-evals.json` for description and routing coverage
- `evals/fixtures/` for fresh, partial, compiled, drifted, and broken vault states

## Next pages

- [obsidian-notes-karpathy](/skills/obsidian-notes-karpathy)
- [kb-init](/skills/kb-init)
- [kb-compile](/skills/kb-compile)
- [kb-query](/skills/kb-query)
- [kb-health](/skills/kb-health)
