# 技能总览

这个 bundle 现在包含 1 个入口技能和 5 个操作技能。

## 按生命周期信号选择

| 信号 | 路由到 | 原因 |
| --- | --- | --- |
| 支撑层缺失、半成品、或还是 V1 | `kb-init` | 先把 V2 契约建好 |
| 新 raw 捕获还没进草稿层 | `kb-compile` | draft 层落后于 evidence 层 |
| 草稿待审或 briefing 过期 | `kb-review` | 现在最安全的下一步是过审校门 |
| live 层已存在且用户要答案或交付物 | `kb-query` | 现在是提取、综合、归档或发布 |
| approved layer 变脏、矛盾、断链 | `kb-health` | 当前任务是维护和诊断 |
