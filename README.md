# Obsidian Notes Karpathy

Review-gated, multi-agent-friendly Obsidian knowledge-base skills inspired by Karpathy-style markdown wikis.

```text
raw/            -> immutable human + agent captures
MEMORY.md       -> collaboration memory and editorial context
wiki/drafts/    -> compiled draft knowledge
wiki/live/      -> approved long-term brain
wiki/briefings/ -> role-specific context generated from live
outputs/        -> reviews, Q&A, health reports, and publishable artifacts
```

The core idea is no longer just "compile notes into a wiki". It is "separate production from judgment so unreviewed drafts never become compound retrieval truth."

## Skill set

- `obsidian-notes-karpathy` - lifecycle router
- `kb-init` - initialize, repair, or migrate a vault into the review-gated layout
- `kb-compile` - compile raw captures into `wiki/drafts/`
- `kb-review` - approve, reject, or escalate drafts; rebuild `wiki/briefings/`
- `kb-query` - search and publish from `wiki/live/`
- `kb-health` - audit `wiki/live/`, `wiki/briefings/`, `outputs/qa/`, and `outputs/reviews/`

## Contract highlights

- `raw/` is immutable.
- `MEMORY.md` is collaboration memory, not retrieval truth.
- `kb-compile` writes only to `wiki/drafts/`.
- `kb-review` is the only promotion gate into `wiki/live/`.
- `wiki/briefings/` is generated from approved live pages only.
- `kb-query` reads `wiki/live/`, briefings, and prior Q&A, not drafts.
- legacy-layout vaults are detected and should be migrated before normal operation.

## Required support vs optional outputs

The required support layer is `raw/`, `wiki/drafts/`, `wiki/live/`, `wiki/briefings/`, `wiki/index.md`, `wiki/log.md`, `outputs/reviews/`, `AGENTS.md`, and `CLAUDE.md`.

Downstream output surfaces are created when the later stages need them:

- `outputs/qa/` - durable research answers from `kb-query`
- `outputs/content/` - publish artifacts from `kb-query`
- `outputs/health/` - maintenance reports from `kb-health`
- `MEMORY.md` - recommended collaboration memory and editorial context

Substantive answers and publish artifacts may also carry structured writeback candidates so later compile/review passes can decide whether they should feed back into the wiki.

## Route ownership

- `kb-init` owns setup, repair, and legacy-layout migration.
- `kb-compile` owns raw-to-draft updates, including bootstrap `raw/*.md` captures.
- `kb-review` is the immediate draft-promotion gate and briefing rebuild lane.
- `kb-query` reads approved live knowledge only.
- `kb-health` is the longer-horizon maintenance lane: report-first drift checks, backlog pressure, provenance audits, and safe mechanical fixes in approved surfaces only.

## Deterministic helpers

- `skills/obsidian-notes-karpathy/scripts/skill-contract-registry.json`
- `scripts/detect_lifecycle.py`
- `scripts/scan_compile_delta.py`
- `scripts/scan_review_queue.py`
- `scripts/scan_query_scope.py`
- `scripts/lint_obsidian_mechanics.py`

## Install

```bash
cp -r skills/* ~/.claude/skills/
cp -r skills/* ~/.codex/skills/
```
