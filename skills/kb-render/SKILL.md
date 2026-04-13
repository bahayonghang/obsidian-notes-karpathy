---
name: kb-render
description: Render deterministic outward-facing artifacts from approved knowledge. Use this skill whenever the user says "kb render", "render slides", "render report", "make a canvas", "generate chart brief", "生成幻灯片", "生成报告", or "输出 canvas" and the task is a deterministic derivative from approved live pages or archived grounded answers. This is a separate shipped skill for slides, reports, charts, and canvas outputs. Do not use it for retrieval, grounded Q&A, static web export, or governance/maintenance work.
---

# KB Render

Deterministic outward-facing artifacts from approved knowledge.

## Read before rendering

- local `AGENTS.md`
- local `CLAUDE.md` if present
- `../obsidian-notes-karpathy/scripts/skill-contract-registry.json`
- `../obsidian-notes-karpathy/references/file-model.md`
- `../obsidian-notes-karpathy/references/lifecycle-matrix.md`
- `../obsidian-notes-karpathy/references/content-output-template.md`
- `../obsidian-notes-karpathy/references/render-template.md`
- `../obsidian-notes-karpathy/references/obsidian-safe-markdown.md`
- `../obsidian-notes-karpathy/references/profile-contract.md`

If `render_live_artifact.py` exists, use it as the deterministic baseline.

Supported deterministic outputs remain:

- `slides`
- `charts`
- `canvas`
- `report`

Keep approved live pages and already-cited archived outputs as the only grounding sources.

Static web export belongs to `kb-query`, not `kb-render`.

## Output to the user

Report:

1. which approved pages grounded the artifact
2. which deterministic render mode was used
3. where the artifact was saved
