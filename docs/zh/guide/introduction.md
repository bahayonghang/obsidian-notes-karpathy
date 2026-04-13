# 介绍

Obsidian Notes Karpathy 是一组面向 Obsidian 的技能，用来在 Vault 里运行带显式审校与批准流程的 compile-first 知识库工作流。

## 这套 bundle 管什么

```text
raw/            -> 不可变证据输入层
raw/_manifest.yaml -> 来源登记表
MEMORY.md       -> 协作记忆与编辑上下文
wiki/drafts/    -> 编译出的草稿知识层
wiki/live/      -> 已批准的长期知识层
wiki/live/topics/ -> 已批准知识的浏览层主题图
wiki/briefings/ -> 只从 live 生成的角色简报
outputs/        -> reviews、Q&A、health 报告和对外交付物
```

核心不是“谁最后说了什么就拿来检索”，而是“先编译成草稿，再审校，再只复用已批准知识层”。

`MEMORY.md` 不属于默认知识检索真相层。它应该承接偏好、编辑优先级和协作规则，而不是带来源的专题结论。

## 核心技能

这个 bundle 由一个共享包目录加 6 个操作技能组成。`skills/obsidian-notes-karpathy/` 负责承载共享的 `references/`、`scripts/`、`evals/` 和路由契约，而顶层的 `skills/kb-*` 目录才是执行各生命周期步骤的操作技能。

| 技能 | 职责 | 何时使用 |
| --- | --- | --- |
| `obsidian-notes-karpathy` | 包级入口与生命周期路由 | 用户在聊整个工作流，或在问下一步该做什么 |
| `kb-init` | 创建、修复或迁移 vault 契约 | 支撑层缺失、半成品，或仍是旧版目录结构 |
| `kb-ingest` | 把 raw 来源登记到来源登记表 | 支撑层存在，但来源登记表已过期 |
| `kb-compile` | 把 raw 捕获编译成可审校草稿 | 新证据或更新过的证据还没进入草稿层 |
| `kb-review` | 决定哪些内容值得持久化并重建角色简报 | 草稿待审，或角色简报需要在当前审校流程中立即刷新 |
| `kb-query` | 从已批准知识层检索、回答、归档复用并导出静态站点 | vault 已有足够知识，用户要答案、候选排序、历史答案复用或静态站点 |
| `kb-render` | 生成确定性的幻灯片、报告、图表与 Canvas | 已批准知识已经就绪，现在要做派生产物 |
| `kb-review`（`维护模式`） | 审计漂移、积压、角色简报与溯源 | 已批准知识层开始失真、积压或断链 |


