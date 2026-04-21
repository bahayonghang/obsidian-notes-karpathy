# Chinese-LLM-Wiki 兼容映射

这套 bundle 即使遇到 `Chinese-LLM-Wiki` 那种更简单的 `raw/wiki/output` 话术，也仍然保持 review-gated 契约。

## 核心规则

- 不把仓库退回单层 canonical `wiki/`
- `wiki/live/**` 仍然是批准真相层
- `outputs/**` 仍然是可复用归档面

## 术语映射

| Chinese-LLM-Wiki 术语 | 当前 bundle 含义 | 通常路由到 |
| --- | --- | --- |
| `raw/` | 同样是保留来源的素材库，加上 manifest | `kb-init`、`kb-ingest` |
| `wiki/` | 被拆成 `wiki/drafts/` 和 `wiki/live/` | compile + review |
| `output/` | 被拆成 `outputs/**` 下的产物归档面 | query / render / review |
| `来源页` | 基于来源证据整理出的 summary 候选页 | `kb-compile` 再 `kb-review` |
| `主题页` | 主题草稿页或已批准 live topic 页 | `kb-compile` 再 `kb-review` |
| `实体页` | 实体草稿页或已批准 live entity 页 | `kb-compile` 再 `kb-review` |
| `综合页` | 可能先落在 archive 的 grounded synthesis | `kb-query` 或 `kb-review` |
| `output/analyses` | 归档的 grounded answer / analysis | `kb-query` |
| `output/reports` | 视上下文可能是治理报告，也可能是确定性渲染报告 | `kb-review` 或 `kb-render` |

## 路由提示

- `先读 wiki/index.md` 通常意味着先走 router / query / review 的导航姿态
- `把 raw 里的内容整理成来源页 / 主题页 / 实体页` 先走 compile
- `做 lint 报告`、`查孤儿页 / 断链 / 旧结论漂移` 先走 review maintenance
- `做综合分析，先放 output/analyses` 先走 grounded query archive
- `把已批准知识做成报告 / 幻灯片 / 图表` 先走 deterministic render

## 语言姿态

- `中文优先` 只改变表达方式，不改变真相边界
- `原文证据摘录` 仍然意味着短证据摘录，不是大段复制原文
- `先读 wiki/index.md` 仍然是默认导航规则
