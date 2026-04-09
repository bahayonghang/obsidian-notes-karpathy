---
name: kb-health
description: Run a deep health check on the approved knowledge base. Use this skill whenever the user says "kb health", "health check", "lint live wiki", "find contradictions", "find stale briefings", "review backlog", "批准层体检", "知识库体检", or wants a maintenance pass over approved live pages, briefings, archived Q&A, publish artifacts, and review outputs. Treat this as the longer-horizon maintenance lane for drift, backlog pressure, stale outputs, provenance issues, alias drift, and safe mechanical fixes. Do not use it for normal retrieval/publishing work or for the immediate draft-promotion gate that belongs to `kb-review`.
---

# KB Health

Deep lint and maintenance workflow for the approved live brain and its review ecosystem. This is the longer-horizon maintenance lane: inspect drift, integrity, backlog pressure, stale outputs, and only apply safe mechanical fixes when the target is unambiguous.

## Scope

Before checking the vault, read these files first:

- local `AGENTS.md`
- local `CLAUDE.md` if present
- `../obsidian-notes-karpathy/scripts/skill-contract-registry.json`
- `../obsidian-notes-karpathy/references/file-model.md`
- `../obsidian-notes-karpathy/references/lifecycle-matrix.md`
- `../obsidian-notes-karpathy/references/activity-log-template.md`
- `../obsidian-notes-karpathy/references/health-rubric.md`
- `../obsidian-notes-karpathy/references/search-upgrades.md`
- `../obsidian-notes-karpathy/references/provenance-and-alias-policy.md`
- `../obsidian-notes-karpathy/references/questions-and-reflection-policy.md`

Treat `skill-contract-registry.json` as the canonical source for required references, baseline script, and expected write surfaces.

If `../obsidian-notes-karpathy/scripts/lint_obsidian_mechanics.py` exists, run it first and treat its output as the deterministic baseline.

If `../obsidian-notes-karpathy/scripts/build_governance_indices.py` exists, use it when the user wants refreshed `QUESTIONS.md`, `GAPS.md`, or `ALIASES.md` views from the current approved layer.

Inspect primarily:

- `wiki/live/`
- `wiki/briefings/`
- `outputs/qa/`
- `outputs/content/`
- `outputs/reviews/`
- `outputs/health/`

Reference `raw/` only when provenance requires a spot check.

## What it checks

- conflicting approved concepts or entities
- duplicate approved concepts or entities
- stale Q&A relative to newer live pages
- writeback backlog in archived Q&A or publish artifacts
- stale briefings
- unapproved pages that somehow landed in `wiki/live/`
- review backlog piling up in drafts
- broken render mechanics in live pages
- weak provenance from live pages back to review records
- collaboration memory leaking into approved knowledge, or vice versa
- source hash drift or outdated verification signals
- alias drift or cross-language splits that should be surfaced for merge review
- open questions and gap reports that keep reappearing without resolution

## Report output

Write the report to `outputs/health/health-check-{date}.md` using the shared rubric from `../obsidian-notes-karpathy/references/health-rubric.md`.

## Safe mechanical fixes

`kb-health` is report-first, but it may apply deterministic, reversible fixes when the target is unambiguous.

Allowed fix scope:

- approved pages under `wiki/live/**`
- briefings under `wiki/briefings/**`
- archived answers under `outputs/qa/**`
- publish artifacts under `outputs/content/**` when the fix is purely mechanical

Do not:

- mutate `raw/`
- promote drafts into live
- rewrite content that requires semantic judgment just because the issue is visible

## Relationship to kb-review

`kb-review` handles the immediate gate for pending drafts and the briefing rebuild that belongs to that same review pass.

`kb-health` handles longer-horizon maintenance: drift, integrity, backlog pressure, stale archived outputs, repeated briefing drift, alias splits, and source-integrity drift that has become an approved-layer maintenance problem.
