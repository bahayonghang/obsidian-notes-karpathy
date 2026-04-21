# kb-render

`kb-render` is the deterministic derivative lane for approved knowledge.

If the user says `output/reports`, decide whether they mean a governance report or a deterministic rendered report. Governance and lint work stays in `kb-review`; deterministic derivatives stay here.

## Supported modes

- `slides`
- `charts`
- `canvas`
- `report`

## Output rule

Render outputs are downstream derivatives, not approved truth. They should preserve `source_live_pages` and `followup_route`. Static web exports belong to `kb-query`, not `kb-render`.
