---
name: kb-init
description: Initialize, migrate, or repair a V2 review-gated Obsidian knowledge base. Use this skill whenever the user says "kb init", "initialize knowledge base", "repair vault", "migrate to V2", "add draft/live review gate", "set up briefings", "初始化知识库", "迁移知识库", "修复知识库结构", or wants a fresh vault or legacy vault brought onto the review-gated contract.
---

# KB Init

One-time setup, migration, and repair for the V2 review-gated workflow.

## Read before writing

Read these shared references first:

- `../obsidian-notes-karpathy/references/file-model.md`
- `../obsidian-notes-karpathy/references/lifecycle-matrix.md`
- `../obsidian-notes-karpathy/references/schema-template.md`
- `../obsidian-notes-karpathy/references/summary-template.md`
- `../obsidian-notes-karpathy/references/review-template.md`
- `../obsidian-notes-karpathy/references/briefing-template.md`
- `../obsidian-notes-karpathy/references/activity-log-template.md`
- `../obsidian-notes-karpathy/references/index-home-template.md`

If `../obsidian-notes-karpathy/scripts/detect_lifecycle.py` exists, run it first to distinguish fresh setup, partial repair, and `legacy-v1` migration.

## Create or repair the canonical structure

Create the minimum V2 structure:

```text
raw/human/{articles,papers,podcasts,repos,assets}
raw/agents/{role}/
wiki/drafts/{summaries,concepts,entities,indices}
wiki/live/{summaries,concepts,entities,indices}
wiki/briefings/
wiki/index.md
wiki/log.md
outputs/{reviews,qa,health,reports,slides,charts,content/{articles,threads,talks}}
AGENTS.md
CLAUDE.md
```

## Migration posture

For V1 vaults:

- preserve legacy content
- explain that direct `wiki/summaries/` / `wiki/concepts/` pages must move into `wiki/live/`
- scaffold the V2 directories first
- treat migration as a repair step, not as a normal compile or query pass

## Contract guarantees

- `raw/` stays immutable
- `kb-compile` writes only to `wiki/drafts/`
- `kb-review` owns promotion into `wiki/live/` and briefing refresh
- `kb-query` must not read drafts as truth
- `AGENTS.md` and `CLAUDE.md` stay aligned on the V2 file model

## Starter files

Create:

- `wiki/index.md`
- `wiki/log.md`
- `wiki/live/indices/INDEX.md`
- `wiki/live/indices/CONCEPTS.md`
- `wiki/live/indices/SOURCES.md`
- `wiki/live/indices/RECENT.md`
- at least one example briefing or a placeholder in `wiki/briefings/`
- a review template example in `outputs/reviews/` when the user wants a demonstrable starter

## Output to the user

Report:

1. what was created or migrated
2. whether a legacy V1 layout was detected
3. what existing content was preserved
4. the next recommended command, usually `kb-compile` or `kb-review`
