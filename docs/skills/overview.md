# Skills Overview

This bundle has one package entry skill and four operational skills.

## Why the split exists

The workflow has distinct phases:

- detect lifecycle stage
- initialize a vault contract
- compile immutable raw sources into a wiki
- query the wiki and persist useful answers
- run deep health checks

Keeping these phases separate makes triggering cleaner and keeps each SKILL.md focused.

## Skills

| Skill | Role | Use when |
|-------|------|----------|
| `obsidian-notes-karpathy` | Package entry and router | The user talks about the workflow as a whole |
| `kb-init` | One-time setup | The canonical vault structure does not exist yet |
| `kb-compile` | Ingest and compile | New or changed raw sources need to become wiki pages |
| `kb-query` | Search, answer, generate | The user wants insights or deliverables from the wiki |
| `kb-health` | Deep lint and maintenance | The user wants a report-oriented health pass |

## Shared resources

The package also ships:

- `references/` for schema, summary, concept, Q&A, publish, and health-report templates
- `evals/evals.json` for package-level regression prompts
- `evals/fixtures/` for realistic vault states such as fresh, compiled, and drifted vaults

## Design contract

- `raw/` is immutable
- `wiki/` is the compiled artifact
- `outputs/qa/` stores substantive Q&A by default
- `outputs/content/` stores grounded publishable artifacts
- `wiki/index.md` is the content entry point
- `wiki/log.md` is the chronological log

## Next pages

- [obsidian-notes-karpathy](/skills/obsidian-notes-karpathy)
- [kb-init](/skills/kb-init)
- [kb-compile](/skills/kb-compile)
- [kb-query](/skills/kb-query)
- [kb-health](/skills/kb-health)
