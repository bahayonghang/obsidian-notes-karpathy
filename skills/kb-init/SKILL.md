---
name: kb-init
description: Initialize, migrate, or repair a review-gated Obsidian knowledge base. Use this skill whenever the user says "kb init", "initialize knowledge base", "repair vault", "migrate an old layout", "add draft/live review gate", "set up briefings", "LLM Wiki", "Karpathy wiki", "Obsidian IDE", "knowledge compiler", "personal knowledge base", "second brain", "初始化知识库", "迁移知识库", "修复知识库结构", "中文优先 wiki", "raw/wiki/output 脚手架", "把这个 vault 搭成中文优先 wiki", or wants a fresh vault or legacy-layout vault brought onto the review-gated contract. Do not use it for normal compile, review, query, or maintenance work once the support layer is already healthy.
---

# KB Init

One-time setup, migration, and repair for the review-gated workflow.

## Quick Start

For a fresh vault, the happy path is four steps:

1. **Init** — run `kb-init` to create the support layer (`raw/`, `wiki/`, `outputs/`, `AGENTS.md`)
2. **Ingest** — run `kb-ingest` to register raw sources into `raw/_manifest.yaml`
3. **Compile** — run `kb-compile` to build reviewable drafts
4. **Review** — run `kb-review` to approve drafts into the permanent wiki under `wiki/live/`

After that, use `kb-query` to ask questions, `kb-render` to generate deterministic outward artifacts, and `kb-review` maintenance mode to audit drift.

This mirrors Karpathy's LLM Wiki pattern: raw sources are immutable evidence, the LLM compiles and maintains the wiki, and you curate by sourcing documents and asking the right questions.

If the user arrives with the simpler `raw/wiki/output` language from `Chinese-LLM-Wiki`, keep that as onboarding vocabulary only. The implementation still lands on the review-gated support layer.

## Read before writing

Read these shared references first:

- `../obsidian-notes-karpathy/scripts/skill-contract-registry.json`
- `../obsidian-notes-karpathy/references/chinese-llm-wiki-compat.md`
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
- `../obsidian-notes-karpathy/references/query-writeback-lifecycle.md`
- `../obsidian-notes-karpathy/references/taxonomy-and-hubs.md`
- `../obsidian-notes-karpathy/references/memory-lifecycle.md`
- `../obsidian-notes-karpathy/references/graph-contract.md`
- `../obsidian-notes-karpathy/references/source-manifest-contract.md`
- `../obsidian-notes-karpathy/references/topic-template.md`
- `../obsidian-notes-karpathy/references/profile-contract.md`
- `../obsidian-notes-karpathy/references/automation-hooks.md`
- `../obsidian-notes-karpathy/references/procedure-template.md`
- `../obsidian-notes-karpathy/references/episode-template.md`

Treat `skill-contract-registry.json` as the canonical source for role, baseline command, required references, and expected write surfaces.

If `onkb` is available, use these commands first:

- `onkb --json status <vault-root>` to distinguish setup, repair, and `needs-migration`
- `onkb --json init <vault-root> ...` for fresh setup and non-destructive repair scaffolding
- `onkb --json migrate <vault-root> ...` for legacy single-layer vault migration
- `onkb --json review governance <vault-root> --write` when optional governance scaffolding should be materialized

## Profile choice

Surface the operating profile during init or repair:

- `governed-team` for the strictest default governance
- `standard` for lighter maintenance noise without widening truth
- `fast-personal` for solo use where briefings can stay out of default query scope

Persist the chosen profile in scaffolded starter files so later lifecycle and query tooling can see it deterministically.

`MEMORY.md` is recommended collaboration scaffolding but not a blocking requirement. Solo / fast-personal vaults can skip it by running `onkb --json init <vault-root> --skip-memory`. The rest of the support layer still lands.

## Create or repair the canonical structure

Create the required support layer:

```text
raw/human/{articles,papers,podcasts,repos,assets}
raw/human/data
raw/agents/{role}/
raw/_manifest.yaml
wiki/drafts/{summaries,topics,concepts,entities,procedures,overviews,comparisons,indices}
wiki/live/{summaries,topics,concepts,entities,procedures,overviews,comparisons,indices}
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
- `wiki/live/indices/ENTITIES.md`
- `wiki/live/indices/RELATIONSHIPS.md`

Optional latest-capability scaffolding may include:

- `outputs/episodes/`
- `outputs/audit/operations.jsonl`
- `outputs/health/graph-snapshot.json`

## Migration posture

For legacy-layout vaults:

- preserve legacy content
- explain that direct `wiki/summaries/` / `wiki/concepts/` pages must move into `wiki/live/`
- scaffold the review-gated directories first
- treat migration as a repair step, not as a normal compile or query pass

## Checkpoint

Before creating or repairing the support layer, confirm with the user:

- which operating profile to use (`governed-team`, `standard`, or `fast-personal`)
- whether optional governance scaffolding (`QUESTIONS.md`, `GAPS.md`, `ALIASES.md`) is wanted
- whether this is a fresh setup, a repair of an existing vault, or a legacy-layout migration

For a fresh vault with no existing content, proceed after profile selection without further confirmation.

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
- `raw/_manifest.yaml`
- `wiki/live/indices/INDEX.md`
- `wiki/live/indices/CONCEPTS.md`
- `wiki/live/indices/SOURCES.md`
- `wiki/live/indices/TOPICS.md`
- `wiki/live/indices/RECENT.md`
- `wiki/live/indices/EDITORIAL-PRIORITIES.md`
- optional `wiki/live/indices/QUESTIONS.md` when the user wants governance scaffolding
- optional `wiki/live/indices/ENTITIES.md` and `RELATIONSHIPS.md` when the user wants graph-friendly navigation
- at least one example briefing or a placeholder in `wiki/briefings/`
- a review template example in `outputs/reviews/` when the user wants a demonstrable starter

## Output to the user

Report:

1. what was created or migrated
2. whether a legacy-layout / `needs-migration` state was detected
3. what existing content was preserved
4. the next recommended command, usually `kb-compile` or `kb-review`
