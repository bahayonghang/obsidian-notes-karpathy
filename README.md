# Obsidian Notes Karpathy

Review-gated, multi-agent-friendly Obsidian knowledge-base skills inspired by Karpathy-style markdown wikis.

```text
raw/            -> immutable human + agent captures
wiki/drafts/    -> compiled draft knowledge
wiki/live/      -> approved long-term brain
wiki/briefings/ -> role-specific context generated from live
outputs/        -> reviews, Q&A, health reports, and publishable artifacts
```

The core idea is no longer just "compile notes into a wiki". It is "separate production from judgment so unreviewed drafts never become compound retrieval truth."

## Skill set

- `obsidian-notes-karpathy` - lifecycle router
- `kb-init` - initialize, repair, or migrate a vault into V2
- `kb-compile` - compile raw captures into `wiki/drafts/`
- `kb-review` - approve, reject, or escalate drafts; rebuild `wiki/briefings/`
- `kb-query` - search and publish from `wiki/live/`
- `kb-health` - audit `wiki/live/`, `wiki/briefings/`, `outputs/qa/`, and `outputs/reviews/`

## Contract highlights

- `raw/` is immutable.
- `kb-compile` writes only to `wiki/drafts/`.
- `kb-review` is the only promotion gate into `wiki/live/`.
- `wiki/briefings/` is generated from approved live pages only.
- `kb-query` reads `wiki/live/`, briefings, and prior Q&A, not drafts.
- legacy V1 vaults are detected and should be migrated before normal V2 operation.

## Deterministic helpers

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
