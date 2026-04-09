# 技能总览

这个 bundle 现在包含 1 个入口技能和 5 个操作技能。

## 按生命周期信号选择

| 信号 | 路由到 | 原因 |
| --- | --- | --- |
| 支撑层缺失、半成品、或还是 legacy-layout | `kb-init` | 先把 review-gated 契约建好 |
| 新 raw 捕获还没进草稿层 | `kb-compile` | draft 层落后于 evidence 层 |
| 草稿待审，或 briefing 已过期且应在下一次 review pass 中重建 | `kb-review` | 当前最安全的下一步是处理具体 draft package，或立刻重建 briefing |
| live 层已存在且用户要答案或交付物 | `kb-query` | 现在是提取、综合、归档或发布 |
| approved layer 变脏、矛盾、断链，或 backlog pressure 已经变成维护问题 | `kb-health` | 当前任务是更长期的维护、诊断和 approved surfaces 上的安全机械修复 |
| 用户在聊整个 Obsidian vault 工作流，或问下一步该做什么 | `obsidian-notes-karpathy` | 先做生命周期诊断，再分流到正确技能 |

## 共享合同

- `raw/` 永远不可变。
- `raw/**/papers/*.pdf` 下的论文 PDF 仍然是 `paper-workbench` 路由例外，不属于普通 `kb-compile` 入口。
- `MEMORY.md` 是协作记忆层，不是专题真相层。
- `wiki/drafts/` 可审校，但不是真相层。
- `wiki/live/` 是批准后的长期知识层。
- `wiki/briefings/` 只能从 live 构建。
- `outputs/reviews/` 存 promotion 决策。
- `wiki/live/indices/QUESTIONS.md`、`GAPS.md`、`ALIASES.md` 这类治理视图按需维护，不是每个 vault 都强制存在。
- legacy-layout vault 先经 `kb-init` 迁移，再进入正常流程。

设计理由与后续演化请看 [架构说明](/zh/architecture/overview)。

## 搭配技能

文档里也保留了 `obsidian-cli`、`obsidian-markdown`、`obsidian-canvas-creator` 这类搭配技能页面。

它们属于相邻 Obsidian 生态参考，不是本仓库当前发货的核心 1+5 bundle。
