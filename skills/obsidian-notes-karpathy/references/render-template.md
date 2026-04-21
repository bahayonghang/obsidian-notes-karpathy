# Render Template

Use this reference when generating deterministic outward-facing artifacts from approved knowledge.

## Supported modes

- `slides`
- `charts`
- `canvas`
- `report`
- `web`

## Output posture

- Slides should be Marp-compatible markdown.
- Charts should be markdown chart briefs or chart specs, not binary renders.
- Canvas outputs should be `.canvas` JSON files.
- Reports should be markdown.
- Web outputs should be static packages rooted at `outputs/web/{slug}/index.html` with local assets and manifest payloads.

## Required metadata for markdown render outputs

- `title`
- `render_mode`
- `source_live_pages`
- `followup_route`

Render outputs are downstream derivatives. They do not become approved truth automatically.

## Mode-specific skeletons

### slides

- output path: `outputs/slides/`
- file type: Marp-compatible markdown
- required frontmatter extras: `marp: true`, `paginate: true`
- preferred shape: title slide -> 3-5 content slides -> provenance slide

### report

- output path: `outputs/reports/`
- file type: markdown
- required sections: `Executive Summary`, `Key Findings`, `Analysis`, `Provenance`
- if the report is governance or drift-oriented rather than deterministic rendering, route back to `kb-review`

### charts

- output path: `outputs/charts/`
- file type: markdown chart brief or spec
- required sections: `Data`, `Spec`, `Interpretation`
- prefer tabular data plus a small machine-readable spec block

### canvas

- output path: `outputs/charts/` unless the user gives a better destination
- file type: Obsidian `.canvas` JSON
- keep nodes grounded in approved live pages or short derived text blocks

### web

- output path: `outputs/web/{slug}/index.html`
- owner: `kb-query`, not `kb-render`
- listed here only to make the boundary explicit
