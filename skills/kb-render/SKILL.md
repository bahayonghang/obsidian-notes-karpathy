---
name: kb-render
description: Render deterministic outward-facing artifacts from approved knowledge. Use this skill whenever the user says "kb render", "render slides", "render report", "make a canvas", "generate chart brief", "生成幻灯片", "生成报告", "输出 canvas", `把已批准主题做成报告`, or clearly wants a deterministic derivative from approved live pages or already-grounded archived answers. This is the separate shipped skill for slides, reports, charts, and canvas outputs. Do not use it for retrieval, grounded Q&A, static web export, governance reports, or maintenance work.
---

# KB Render

Turn approved knowledge into deterministic outward-facing artifacts.

Karpathy's LLM Wiki naturally produces slides, charts, and reports. `kb-render` is the dedicated lane for those deterministic derivatives after the grounding work is already done.

## Minimal loop

1. confirm which approved pages ground the artifact
2. choose the render mode
3. generate the artifact in the expected format
4. save it under the correct output directory
5. record any durable follow-up as writeback metadata

## Read before rendering

- local `AGENTS.md`
- local `CLAUDE.md` if present
- `../obsidian-notes-karpathy/scripts/skill-contract-registry.json`
- `../obsidian-notes-karpathy/references/archive-model.md`
- `../obsidian-notes-karpathy/references/file-model.md`
- `../obsidian-notes-karpathy/references/lifecycle-matrix.md`
- `../obsidian-notes-karpathy/references/content-output-template.md`
- `../obsidian-notes-karpathy/references/render-template.md`
- `../obsidian-notes-karpathy/references/obsidian-safe-markdown.md`
- `../obsidian-notes-karpathy/references/profile-contract.md`

Treat `skill-contract-registry.json` as the canonical source for required references, baseline command, and expected write surfaces.

If `onkb` is available, run `onkb --json render <vault-root> --mode <mode> --source <path>` first to get the deterministic baseline before manual shaping.

## Non-negotiable rules

- only ground artifacts in `wiki/live/` pages and already-cited `outputs/qa/` answers
- archived grounded answers are valid inputs, but they remain artifact archive rather than truth
- never read `wiki/drafts/` or `raw/` as render source material
- static web export belongs to `kb-query`, not here
- obey Obsidian-safe markdown rules, especially no alias-style wikilinks inside table cells

## Render modes

- `slides` -> Marp-compatible markdown under `outputs/slides/`
- `report` -> deterministic markdown report under `outputs/reports/`
- `charts` -> markdown chart brief or chart spec under `outputs/charts/`
- `canvas` -> Obsidian `.canvas` JSON, usually under `outputs/charts/`

Use `../obsidian-notes-karpathy/references/render-template.md` for the detailed skeletons, metadata, and per-mode formatting rules.

If the user says `output/reports` but clearly means a governance or lint report, route back to `kb-review` maintenance instead of rendering here.

## Checkpoint

Before generating any artifact, confirm:

- which approved pages or archived grounded answers are the source
- which render mode fits the request
- whether any audience or format constraint changes the template

If the request is already explicit, locate the approved pages silently and proceed. Only pause when source selection or mode is genuinely ambiguous.

## Output to the user

Report:

1. which approved pages grounded the artifact
2. which render mode was used
3. where the artifact was saved
4. whether `writeback_candidates` were discovered during rendering
5. whether a `render` log entry should be appended to `wiki/log.md`
