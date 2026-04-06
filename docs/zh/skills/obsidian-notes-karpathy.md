# obsidian-notes-karpathy

Karpathy 式知识库工作流的包级入口技能。

## 它做什么

这个技能不会独自执行全部流程，而是先判断当前生命周期阶段，再路由到正确的操作型技能。

## 路由逻辑

- 缺少标准目录 -> `kb-init`
- raw 中有新资料或变更资料 -> `kb-compile`
- 用户要问问题、生成报告、幻灯片或发布内容 -> `kb-query`
- 用户要做 lint、体检、过时检查、冲突审计或“笔记越来越散”的诊断 -> `kb-health`

## 为什么需要它

像“帮我搭一个 Karpathy workflow”这样的宽泛请求，单靠子技能很容易欠触发。包级入口技能就是这个 bundle 的前门。

## 核心原则

- `raw/` 永远视为不可变
- 优先从 `wiki/index.md` 和 `wiki/log.md` 开始
- 实质性 Q&A 默认归档
- 搜索升级先用 markdown、Backlinks 和 Properties，再考虑更重的基础设施
