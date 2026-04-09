# CLAUDE.md

This repository defines a review-gated Obsidian knowledge-base contract.

## Project Overview

This is a skills-only project. The deliverables are:

- `SKILL.md` contracts
- shared references under `skills/obsidian-notes-karpathy/references/`
- deterministic scripts under `skills/obsidian-notes-karpathy/scripts/`
- docs pages
- eval fixtures and tests

Canonical workflow:

`raw/` -> `kb-compile` -> `wiki/drafts/` -> `kb-review` -> `wiki/live/` + `wiki/briefings/` -> `kb-query` / `kb-health` -> `outputs/`

## Repository Structure

- `skills/obsidian-notes-karpathy/SKILL.md` - package-level lifecycle router
- `skills/kb-init/SKILL.md` - setup, repair, and legacy-layout migration
- `skills/kb-compile/SKILL.md` - raw-to-draft compilation
- `skills/kb-review/SKILL.md` - draft review gate, promotion, and briefing rebuild
- `skills/kb-query/SKILL.md` - approved-layer search, answers, and publishing
- `skills/kb-health/SKILL.md` - live-layer maintenance, backlog, and briefing audit
- `skills/obsidian-notes-karpathy/references/` - shared review-gated file model, lifecycle matrix, templates, governance policy, and rubric
- `skills/obsidian-notes-karpathy/scripts/` - deterministic helpers including the skill contract registry, lifecycle detection, compile delta, review queue, query scope, and mechanical linting
- `skills/obsidian-notes-karpathy/evals/fixtures/` - legacy and review-gated fixture vaults
- `README.md` and `README_CN.md` - public docs that must stay aligned

## Contract Rules

- `raw/` is immutable evidence intake.
- `MEMORY.md` is collaboration memory and editorial context, not topic retrieval truth.
- `wiki/drafts/` is reviewable knowledge, not retrieval truth.
- `wiki/live/` is the only approved long-term brain.
- `wiki/briefings/` must be generated from `wiki/live/` only.
- `outputs/reviews/` stores the decision ledger for promotion.
- `kb-query` must not treat drafts or raw captures as truth.
- `AGENTS.md` is the required local contract; `CLAUDE.md` is the generated companion.
- legacy-layout vaults must route through `kb-init` migration before normal operation.
- source integrity, alias alignment, stale-page checks, and question / gap tracking are governance enhancements layered on top of the review gate rather than replacements for it.

## Support Layer Notes

- The required support layer is `raw/`, `wiki/drafts/`, `wiki/live/`, `wiki/briefings/`, `wiki/index.md`, `wiki/log.md`, `outputs/reviews/`, `AGENTS.md`, and `CLAUDE.md`.
- `outputs/qa/`, `outputs/content/`, and `outputs/health/` are downstream output surfaces created when later stages need them.
- `MEMORY.md` is recommended collaboration scaffolding rather than a blocking support-layer requirement.
- `outputs/qa/` and `outputs/content/` may carry structured writeback candidates that must still re-enter the system through draft -> review -> live.
- bootstrap root captures under `raw/*.md` are valid compile inputs, but missing support directories still make the vault a repair case.
- optional governance indices like `wiki/live/indices/QUESTIONS.md`, `GAPS.md`, and `ALIASES.md` may be created when richer maintenance surfaces are useful.
- `kb-health` is report-first and may apply deterministic mechanical fixes in approved surfaces only; it must never mutate `raw/` or promote drafts.

## Installation

```bash
cp -r skills/* ~/.claude/skills/
cp -r skills/* ~/.codex/skills/
```
