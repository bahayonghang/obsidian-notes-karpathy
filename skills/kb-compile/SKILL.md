---
name: kb-compile
description: Incrementally compile immutable raw captures into V2 draft knowledge. Use this skill whenever the user says "compile wiki", "compile kb", "sync drafts", "digest these captures", "turn my clips into drafts", "编译wiki", "更新草稿层", "同步草稿", or wants newly added raw material turned into reviewable summaries, concepts, entities, and draft indices.
---

# KB Compile

Incrementally turn immutable raw captures into reviewable draft knowledge.

## Read before compiling

Read these files first:

- local `AGENTS.md`
- local `CLAUDE.md` if present
- `../obsidian-notes-karpathy/references/file-model.md`
- `../obsidian-notes-karpathy/references/lifecycle-matrix.md`
- `../obsidian-notes-karpathy/references/schema-template.md`
- `../obsidian-notes-karpathy/references/summary-template.md`
- `../obsidian-notes-karpathy/references/activity-log-template.md`

If `../obsidian-notes-karpathy/scripts/scan_compile_delta.py` exists, run it first.

## Non-negotiable rules

- do not rewrite `raw/`
- write only to `wiki/drafts/` and draft indices
- never promote directly into `wiki/live/`
- keep human captures and agent captures distinguishable in provenance
- keep PDF paper handling strict: `raw/**/papers/*.pdf` still routes through `paper-workbench`

## Source discovery

Accept:

- markdown captures under `raw/human/**`
- markdown captures under `raw/agents/{role}/**`
- legacy markdown captures under V1 paths only during migration
- paper PDFs under any `papers/` subtree inside raw

## Main outputs

- `wiki/drafts/summaries/**`
- `wiki/drafts/concepts/**`
- `wiki/drafts/entities/**` when needed
- `wiki/drafts/indices/*`
- batch `ingest` entry in `wiki/log.md`

The compile pass exists to hand clean draft packages to `kb-review`, which then writes `outputs/reviews/**`, promotes approved pages into `wiki/live/**`, and rebuilds `wiki/briefings/**`.

## Draft requirements

Every draft should include:

- explicit evidence
- `draft_id`
- `compiled_from`
- `capture_sources`
- `review_state`
- `review_score`
- `blocking_flags`

Drafts should be shaped for review, not for final polish.

## Output to the user

Always report:

1. how many captures were new, changed, or unchanged
2. how many draft summaries were created or updated
3. how many draft concepts or entities were touched
4. whether any PDFs were skipped because `paper-workbench` was unavailable
5. whether the next step is `kb-review`
