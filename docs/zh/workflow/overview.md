# 工作流总览

## 按症状进入

| 如果当前更像这样 | 先走哪一步 |
| --- | --- |
| 支撑层缺失、半成品、或还是 legacy-layout | `kb-init` |
| 新 raw 捕获还没编译到草稿层 | `kb-compile` |
| 草稿待审，或 briefing 已过期且应在下一次 gate pass 中重建 | `kb-review` |
| 用户要答案、报告、文章、幻灯片 | `kb-query` |
| 批准层需要维护基线、drift 审计或安全清理 | `kb-health` |
| 当前到底该走哪一步不清楚 | `obsidian-notes-karpathy` |

如果已经出现少量 live 内容，但 `wiki/drafts/`、`wiki/briefings/`、`outputs/reviews/` 仍然缺失，也要先走 `kb-init`。结构修复优先于正常 query。

```mermaid
graph LR
    A[obsidian-notes-karpathy] --> B[kb-init]
    A --> C[kb-compile]
    A --> D[kb-review]
    A --> E[kb-query]
    A --> F[kb-health]
    B --> C
    C --> D
    D --> E
    D --> F
    E -.->|writeback 候选需要批准| C
    F -.->|修复或重建| D
```

只有当请求是工作流级别且当前步骤不明确时，才先走入口技能。操作已经明确时，直接用对应的操作技能。
