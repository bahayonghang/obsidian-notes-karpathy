# kb-render

`kb-render` is the deterministic derivative lane for approved knowledge.

## Supported modes

- `slides`
- `charts`
- `canvas`
- `report`

## Output rule

Render outputs are downstream derivatives, not approved truth. They should preserve `source_live_pages` and `followup_route`. Static web exports belong to `kb-query`, not `kb-render`.
