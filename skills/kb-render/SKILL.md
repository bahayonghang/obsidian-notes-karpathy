---
name: kb-render
description: Render deterministic outward-facing artifacts from approved knowledge. Use this skill whenever the user says "kb render", "render slides", "render report", "make a canvas", "generate chart brief", "生成幻灯片", "生成报告", or "输出 canvas" and the task is a deterministic derivative from approved live pages or archived grounded answers. This is a separate shipped skill for slides, reports, charts, and canvas outputs. Do not use it for retrieval, grounded Q&A, static web export, or governance/maintenance work.
---

# KB Render

Turn approved knowledge into deterministic outward-facing artifacts.

Karpathy's LLM Wiki emphasizes that queries naturally produce slides, charts, and reports. `kb-render` is the dedicated lane for this: it takes approved live pages as input and produces reproducible derivative artifacts as output.

## Minimal loop

1. confirm which approved pages ground the artifact
2. select the render mode
3. generate the artifact following the mode-specific format
4. save to the correct output directory
5. log the render entry

## When this compounds the wiki

A good render pass should not be disposable. If the artifact reveals a missing synthesis, a gap in approved coverage, or a useful new relationship, record `writeback_candidates` so the next compile/review pass can decide what should flow back into the wiki.

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

If `../obsidian-notes-karpathy/scripts/render_live_artifact.py` exists, run it first with `--mode` and `--source` to get the deterministic baseline before any manual shaping.

## Non-negotiable rules

- only ground artifacts in `wiki/live/` pages and already-cited `outputs/qa/` answers
- already-cited archived grounded answers are valid inputs, but they remain artifact archive rather than truth
- never read `wiki/drafts/` or `raw/` as source material
- static web export belongs to `kb-query`, not here
- obey Obsidian-safe markdown rules: no alias-style wikilinks inside table cells

## Render modes

### slides

Output: Marp-compatible markdown under `outputs/slides/`.

Format:

```markdown
---
title: "{title}"
render_mode: slides
source_live_pages:
  - "[[wiki/live/...]]"
followup_route: none | draft
marp: true
theme: default
paginate: true
---

# {Title}

{Subtitle or one-line summary}

---

## {Section Heading}

- {key point 1}
- {key point 2}
- {key point 3}

<!-- speaker notes if useful -->

---

## {Next Section}

{content}
```

Guidance:

- keep each slide to 3-5 bullet points or one key visual concept
- use `---` as the slide separator
- prefer concrete claims grounded in specific approved pages over vague summaries
- add a final slide with a provenance summary listing source pages

### report

Output: structured markdown under `outputs/reports/`.

Format:

```markdown
---
title: "{title}"
render_mode: report
source_live_pages:
  - "[[wiki/live/...]]"
followup_route: none | draft
---

# {Title}

## Executive Summary

{2-3 sentence overview}

## Key Findings

1. {finding with evidence}
2. {finding with evidence}

## Analysis

{detailed sections grounded in approved pages}

## Provenance

- [[wiki/live/...]] - {what it contributed}

## Open Questions

- {unresolved questions worth tracking}
```

Guidance:

- lead with the conclusion, then support it
- cite specific approved pages inline
- flag disagreements between sources explicitly rather than flattening them

### charts

Output: chart specification markdown under `outputs/charts/`.

Format:

```markdown
---
title: "{title}"
render_mode: charts
chart_type: bar | line | scatter | pie | table | comparison
source_live_pages:
  - "[[wiki/live/...]]"
followup_route: none | draft
---

# {Title}

## Data

| {Column A} | {Column B} | {Column C} |
|---|---|---|
| {value} | {value} | {value} |

## Spec

- chart_type: {type}
- x_axis: {field}
- y_axis: {field}
- grouping: {field, if relevant}

## Interpretation

{what the data shows, grounded in approved pages}
```

Guidance:

- produce a human-readable data table and a machine-parseable spec block
- prefer tabular data over prose when the user asked for a chart
- keep interpretation grounded: cite the approved pages that sourced each data point

### canvas

Output: Obsidian `.canvas` JSON file under `outputs/charts/` or a user-specified path.

Format: standard Obsidian canvas JSON with `nodes` and `edges` arrays. Each node should reference an approved live page or contain a text summary derived from one.

Guidance:

- keep the layout readable: spread nodes with enough spacing
- use edges to show relationships that already exist in `related` fields of live pages
- prefer referencing existing pages over duplicating content into canvas text nodes

## Checkpoint

Before generating any artifact, confirm with the user:

- which approved pages to ground on (or accept a topic and locate them)
- which render mode fits the request
- any audience, scope, or format preferences

If the user provides a clear directive such as "make slides about X", locate the approved pages silently and proceed. Only pause for confirmation when the source selection is ambiguous or the render mode is unclear.

## Output to the user

Report:

1. which approved pages grounded the artifact
2. which deterministic render mode was used
3. where the artifact was saved
4. whether any `writeback_candidates` were discovered during rendering
5. whether a `render` log entry should be appended to `wiki/log.md`
