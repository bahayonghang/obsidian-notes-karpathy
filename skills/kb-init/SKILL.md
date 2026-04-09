---
name: kb-init
description: Initialize, migrate, or repair a review-gated Obsidian knowledge base. Use this skill whenever the user says "kb init", "initialize knowledge base", "repair vault", "migrate an old layout", "add draft/live review gate", "set up briefings", "初始化知识库", "迁移知识库", "修复知识库结构", or wants a fresh vault or legacy-layout vault brought onto the review-gated contract.
---

# KB Init

One-time setup, migration, and repair for the review-gated workflow.

## Read before writing

Read these shared references first:

- `../obsidian-notes-karpathy/scripts/skill-contract-registry.json`
- `../obsidian-notes-karpathy/references/file-model.md`
- `../obsidian-notes-karpathy/references/lifecycle-matrix.md`
- `../obsidian-notes-karpathy/references/schema-template.md`
- `../obsidian-notes-karpathy/references/summary-template.md`
- `../obsidian-notes-karpathy/references/review-template.md`
- `../obsidian-notes-karpathy/references/briefing-template.md`
- `../obsidian-notes-karpathy/references/activity-log-template.md`
- `../obsidian-notes-karpathy/references/index-home-template.md`
- `../obsidian-notes-karpathy/references/questions-template.md`
- `../obsidian-notes-karpathy/references/provenance-and-alias-policy.md`

Treat `skill-contract-registry.json` as the canonical source for role, baseline script, required references, and expected write surfaces.

If `../obsidian-notes-karpathy/scripts/detect_lifecycle.py` exists, run it first to distinguish setup, repair, and `needs-migration` / legacy-layout migration.

If `../obsidian-notes-karpathy/scripts/build_governance_indices.py` exists and the user wants optional governance scaffolding, use it to generate starter `QUESTIONS.md`, `GAPS.md`, and `ALIASES.md` content after the support layer exists.

## Create or repair the canonical structure

Create the required support layer:

```text
raw/human/{articles,papers,podcasts,repos,assets}
raw/agents/{role}/
wiki/drafts/{summaries,concepts,entities,indices}
wiki/live/{summaries,concepts,entities,indices}
wiki/briefings/
wiki/index.md
wiki/log.md
outputs/reviews/
AGENTS.md
CLAUDE.md
```

Create downstream output directories such as `outputs/qa/`, `outputs/health/`, `outputs/reports/`, `outputs/slides/`, `outputs/charts/`, and `outputs/content/**` only when the user wants full scaffolding or the later stages need them.

Treat `MEMORY.md` as recommended collaboration scaffolding. It should hold preferences, editorial priorities, and coordination context, not source-grounded topic knowledge.

Optional governance scaffolding may include:

- `wiki/live/indices/QUESTIONS.md`
- `wiki/live/indices/GAPS.md`
- `wiki/live/indices/ALIASES.md`

## Migration posture

For legacy-layout vaults:

- preserve legacy content
- explain that direct `wiki/summaries/` / `wiki/concepts/` pages must move into `wiki/live/`
- scaffold the review-gated directories first
- treat migration as a repair step, not as a normal compile or query pass

## Contract guarantees

- `raw/` stays immutable
- `kb-compile` writes only to `wiki/drafts/`
- `kb-review` owns promotion into `wiki/live/` and briefing refresh
- `kb-query` must not read drafts as truth
- `AGENTS.md` and `CLAUDE.md` stay aligned on the review-gated file model
- downstream outputs beyond `outputs/reviews/` are optional scaffolding, not minimum support-layer requirements

## Starter files

Create:

- `wiki/index.md`
- `wiki/log.md`
- `MEMORY.md`
- `wiki/live/indices/INDEX.md`
- `wiki/live/indices/CONCEPTS.md`
- `wiki/live/indices/SOURCES.md`
- `wiki/live/indices/RECENT.md`
- `wiki/live/indices/EDITORIAL-PRIORITIES.md`
- optional `wiki/live/indices/QUESTIONS.md` when the user wants governance scaffolding
- at least one example briefing or a placeholder in `wiki/briefings/`
- a review template example in `outputs/reviews/` when the user wants a demonstrable starter

## Output to the user

Report:

1. what was created or migrated
2. whether a legacy-layout / `needs-migration` state was detected
3. what existing content was preserved
4. the next recommended command, usually `kb-compile` or `kb-review`
