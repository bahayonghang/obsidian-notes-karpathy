# 工作流总览

## 按当前状态选择入口

| 如果当前更像这样 | 先走哪一步 |
| --- | --- |
| 支撑层缺失、半成品，或仍是旧版目录结构 | `kb-init` |
| 支撑层已经存在，但来源登记表已过期 | `kb-ingest` |
| 新的 raw 捕获尚未编译到草稿层 | `kb-compile` |
| 草稿待审，或角色简报已过期并应在下一次审校中重建 | `kb-review` |
| 用户需要有依据的回答、候选排序、历史答案复用或静态知识站导出 | `kb-query` |
| 用户需要确定性的幻灯片 / 报告 / 图表简报 / Canvas | `kb-render` |
| 已批准知识层需要基线维护、漂移审计或安全清理 | `kb-review`（`维护模式`） |
| 当前还不清楚该走哪个生命周期步骤 | `obsidian-notes-karpathy` |

如果已经出现少量 `wiki/live/` 内容，但 `wiki/drafts/`、`wiki/briefings/`、`outputs/reviews/` 仍然缺失，也要先走 `kb-init`。结构修复优先于常规检索与输出工作。

<WorkflowLifecycleDiagram locale="zh" />

只有当请求是工作流级别且当前步骤不明确时，才先走入口技能。操作已经明确时，直接用对应的操作技能。

