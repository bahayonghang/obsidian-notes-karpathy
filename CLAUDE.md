# CLAUDE.md

This repository defines a review-gated Obsidian knowledge-base contract.

## Project Overview

This is a Rust CLI plus skills project. The deliverables are:

- `SKILL.md` contracts
- `onkb` Rust CLI
- shared references under `skills/obsidian-notes-karpathy/references/`
- compatibility and repo-maintenance scripts under `skills/obsidian-notes-karpathy/scripts/`
- docs pages
- eval fixtures and tests

Canonical workflow:

`collect` (`web-access` / Web Clipper / manual markdown) -> `kb-ingest` -> `kb-compile` (`浓缩 -> 质疑 -> 对标`) -> `wiki/drafts/` -> `kb-review` (`gate` / `maintenance`) -> `wiki/live/` + `wiki/live/topics/` + `wiki/live/procedures/` + `wiki/briefings/` -> `kb-query` (`research` / `publish` / `web`) or `kb-render` -> `outputs/`

## Repository Structure

- `skills/obsidian-notes-karpathy/SKILL.md` - package-level lifecycle router
- `skills/kb-init/SKILL.md` - setup, repair, and legacy-layout migration
- `skills/kb-ingest/SKILL.md` - source registration and manifest refresh
- `skills/kb-compile/SKILL.md` - raw-to-draft compilation
- `skills/kb-review/SKILL.md` - draft review gate plus approved-layer maintenance and governance refresh
- `skills/kb-query/SKILL.md` - approved-layer search, answers, publish-mode outputs, archived Q&A reuse, and static web export
- `skills/kb-render/SKILL.md` - deterministic slides/report/chart/canvas generation
- `src/` - Rust CLI implementation for `onkb`
- `skills/obsidian-notes-karpathy/references/` - shared review-gated file model, lifecycle matrix, templates, governance policy, and rubric
- `skills/obsidian-notes-karpathy/scripts/` - compatibility and repo-maintenance helpers that should no longer be the primary skill baseline
- `skills/obsidian-notes-karpathy/evals/fixtures/` - legacy and review-gated fixture vaults
- `README.md` and `README_CN.md` - public docs that must stay aligned

## Contract Rules

- `raw/` is immutable evidence intake.
- `raw/_manifest.yaml` is the canonical source registry.
- `MEMORY.md` is collaboration memory and editorial context, not topic retrieval truth.
- `outputs/episodes/` is episodic memory and stays outside the default topic-truth boundary.
- `wiki/drafts/` is reviewable knowledge, not retrieval truth.
- `wiki/live/` is the only approved long-term brain.
- `wiki/live/topics/` is the default browse layer over approved knowledge.
- `wiki/live/procedures/` is approved procedural memory for durable workflows and playbooks.
- `wiki/briefings/` must be generated from `wiki/live/` only.
- `outputs/reviews/` stores the decision ledger for promotion.
- `outputs/audit/operations.jsonl` is the machine-readable audit surface for automation.
- `kb-query` must not treat drafts or raw captures as truth.
- `AGENTS.md` is the required local contract; `CLAUDE.md` is the generated companion.
- legacy-layout vaults must route through `kb-init` migration before normal operation.
- source integrity, alias alignment, stale-page checks, and question / gap tracking are governance enhancements layered on top of the review gate rather than replacements for it.
- creator consistency checks across `CLAUDE.md`, `MEMORY.md`, account `_style-guide.md`, and account briefings belong to `kb-review` maintenance mode.

## Support Layer Notes

- The required support layer is `raw/`, `wiki/drafts/`, `wiki/live/`, `wiki/briefings/`, `wiki/index.md`, `wiki/log.md`, `outputs/reviews/`, `AGENTS.md`, and `CLAUDE.md`.
- `outputs/qa/`, `outputs/content/`, and `outputs/health/` are downstream output surfaces created when later stages need them.
- `MEMORY.md` is recommended collaboration scaffolding rather than a blocking support-layer requirement.
- `outputs/qa/` and `outputs/content/` may carry structured writeback candidates that must still re-enter the system through draft -> review -> live.
- bootstrap root captures under `raw/*.md` are valid compile inputs, but missing support directories still make the vault a repair case.
- optional governance indices like `wiki/live/indices/QUESTIONS.md`, `GAPS.md`, and `ALIASES.md` may be created when richer maintenance surfaces are useful.
- `kb-review` maintenance mode is report-first and may apply deterministic mechanical fixes in approved surfaces only; it must never mutate `raw/` or promote drafts without the review gate.

## Installation

```bash
cargo install --path . --locked
onkb skill install
```
