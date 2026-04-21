# Chinese LLM Wiki Compatibility

Use this reference when a user speaks in the simpler `raw/wiki/output` vocabulary from `Chinese-LLM-Wiki`, but this bundle still needs to keep the review-gated contract intact.

## Hard boundary

- do not collapse the bundle back into a single canonical `wiki/` layer
- keep `wiki/live/**` as the only approved truth surface
- keep `outputs/**` as reusable artifact archive, not approved truth
- keep `raw/**` immutable and retained in place

## Term mapping

| Chinese-LLM-Wiki term | Current bundle meaning | Usual owner |
| --- | --- | --- |
| `raw/` | same immutable source library plus `raw/_manifest.yaml` | `kb-init`, `kb-ingest` |
| `wiki/` | split into `wiki/drafts/` and `wiki/live/` | `kb-compile`, `kb-review` |
| `output/` | split into artifact archive lanes under `outputs/**` | `kb-query`, `kb-render`, `kb-review` |
| `来源页` | source-grounded draft or live summary created from raw evidence | `kb-compile` then `kb-review` |
| `主题页` | `wiki/drafts/topics/**` or `wiki/live/topics/**` | `kb-compile` then `kb-review` |
| `实体页` | `wiki/drafts/entities/**` or `wiki/live/entities/**` | `kb-compile` then `kb-review` |
| `综合页` | reusable synthesis that may start in archive and only later promote into live | `kb-query` or `kb-review` |
| `output/analyses` | `outputs/qa/**` for grounded analysis and answer archives | `kb-query` |
| `output/reports` | governance reports in `outputs/health/**` or deterministic report renders in `outputs/reports/**` | `kb-review` or `kb-render` |

## Routing hints

- if the user says `先读 wiki/index.md` or asks which step should run next, route through the package skill first
- if the user wants to turn new raw material into `来源页` / `主题页` / `实体页`, treat that as compile work before review
- if the user wants a `lint` report, orphan-page check, contradiction pass, or stale-claim audit, treat that as `kb-review` maintenance
- if the user wants a grounded analysis saved to something like `output/analyses`, treat that as `kb-query`
- if the user wants a deterministic report deck or chart from already approved knowledge, treat that as `kb-render`

## Language and evidence posture

- `中文优先` is a presentation rule, not a routing change
- `原文证据摘录` still means short evidence excerpts, not long raw copy-paste
- `先读 wiki/index.md` remains the default navigation posture for router, query, and review work

## Example translations

- `请把 raw 里的文章整理成来源页，并更新主题页和实体页`
  - compile the raw capture into draft summaries, topics, and entities, then hand it to review
- `请做一份 lint 报告放到 output/reports`
  - run review maintenance; do not confuse this with deterministic report rendering
- `请基于现有 wiki 做一份综合分析，先放 output/analyses`
  - run grounded query mode and archive the result under `outputs/qa/**`
