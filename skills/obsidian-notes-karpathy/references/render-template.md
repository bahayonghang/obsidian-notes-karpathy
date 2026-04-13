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
