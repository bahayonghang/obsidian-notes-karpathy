# Chinese-LLM-Wiki Compatibility

This bundle keeps the review-gated contract even when users speak in the simpler `Chinese-LLM-Wiki` vocabulary.

## Core rule

- do not collapse the repo back into a single canonical `wiki/`
- keep `wiki/live/**` as approved truth
- keep `outputs/**` as reusable artifact archive

## Term mapping

| Chinese-LLM-Wiki term | Current bundle meaning | Usually routes to |
| --- | --- | --- |
| `raw/` | same retained source library plus manifest | `kb-init`, `kb-ingest` |
| `wiki/` | split into `wiki/drafts/` and `wiki/live/` | compile + review |
| `output/` | split artifact archive under `outputs/**` | query / render / review |
| `来源页` | source-grounded summary page candidate | `kb-compile` then `kb-review` |
| `主题页` | topic draft or approved live topic page | `kb-compile` then `kb-review` |
| `实体页` | entity draft or approved live entity page | `kb-compile` then `kb-review` |
| `综合页` | grounded synthesis that may start in archive | `kb-query` or `kb-review` |
| `output/analyses` | archived grounded answers | `kb-query` |
| `output/reports` | governance report or deterministic rendered report, depending context | `kb-review` or `kb-render` |

## Routing hints

- `先读 wiki/index.md` usually means router, query, or review posture
- `把 raw 里的内容整理成来源页 / 主题页 / 实体页` means compile first
- `做 lint 报告` or `看有没有孤儿页 / 断链 / 旧结论漂移` means review maintenance
- `做综合分析，先放 output/analyses` means grounded query archive
- `把批准知识做成报告 / 幻灯片 / 图表` means deterministic render

## Language posture

- `中文优先` changes presentation, not the truth boundary
- `原文证据摘录` still means short evidence excerpts
- `先读 wiki/index.md` still works as the default navigation rule
