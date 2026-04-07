# CLAUDE.md

This repository defines a V2 review-gated Obsidian knowledge-base contract.

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
- `skills/kb-init/SKILL.md` - setup, repair, and legacy migration into V2
- `skills/kb-compile/SKILL.md` - raw-to-draft compilation
- `skills/kb-review/SKILL.md` - draft review gate, promotion, and briefing rebuild
- `skills/kb-query/SKILL.md` - approved-layer search, answers, and publishing
- `skills/kb-health/SKILL.md` - live-layer maintenance, backlog, and briefing audit
- `skills/obsidian-notes-karpathy/references/` - shared V2 file model, lifecycle matrix, templates, and rubric
- `skills/obsidian-notes-karpathy/scripts/` - deterministic helpers including lifecycle detection, compile delta, review queue, query scope, and mechanical linting
- `skills/obsidian-notes-karpathy/evals/fixtures/` - V1 and V2 fixture vaults
- `README.md` and `README_CN.md` - public docs that must stay aligned

## Contract Rules

- `raw/` is immutable evidence intake.
- `wiki/drafts/` is reviewable knowledge, not retrieval truth.
- `wiki/live/` is the only approved long-term brain.
- `wiki/briefings/` must be generated from `wiki/live/` only.
- `outputs/reviews/` stores the decision ledger for promotion.
- `kb-query` must not treat drafts or raw captures as truth.
- `AGENTS.md` is the required local contract; `CLAUDE.md` is the generated companion.

## Installation

```bash
cp -r skills/* ~/.claude/skills/
cp -r skills/* ~/.codex/skills/
```
