# obsidian-notes-karpathy

Karpathy 式知识库工作流的包级入口技能。

## 核心职责

这个技能不直接执行整条流水线。它的任务是先判断 Vault 目前处在哪个生命周期阶段，再把请求路由到正确的操作型技能。

## 路由矩阵

| 检测到的信号 | 路由到 | 下一步动作 |
| --- | --- | --- |
| 缺少 `raw/`、`wiki/`、`outputs/`、必需的 `AGENTS.md`，或关键支撑文件 | `kb-init` | 先把契约建好或修好 |
| `raw/` 下有新资料或变更资料，或 `raw/papers/` 下有 PDF 论文 | `kb-compile` | 更新摘要、概念页、索引和日志 |
| 用户要问题答案、报告、文章、推文串或幻灯片 | `kb-query` | 先基于编译层作答，再在需要时归档 |
| 出现漂移、矛盾、过时结论、索引损坏或弱连接 | `kb-health` | 给知识库打分，并区分安全修复项和判断题 |

## 路由前会看什么

- `skills/obsidian-notes-karpathy/references/` 下的共享 references
- 本地 `AGENTS.md`
- 本地 `CLAUDE.md`（如果存在）
- `wiki/index.md` 顶部
- `wiki/log.md` 最近几条记录

如果 `raw/papers/` 下有待处理的 PDF，预期的编译行为是：先从 sidecar 或文件名解析确定 handle，只有 handle 存在时才用 `alphaxiv-paper-lookup`，否则降级到 `pdf`，再不行就提示缺失的 companion skill。

如果 Vault 其余部分已经可用，只是缺 `CLAUDE.md`，路由器应把它视为修复项，而不是立即强制回到 `kb-init`。

## 为什么需要包级入口

像“帮我在 Obsidian 里搭一个会自己生长的知识库”或“我的笔记开始腐烂了”这类请求，本质上是在描述症状而不是命令。包级入口的价值，就是让用户不必先掌握生命周期术语，也能被正确引导到下一步。

## 包级原则

- `raw/` 永远当作不可变输入层
- `wiki/` 是 LLM 负责维护的编译层
- `outputs/qa/` 是持久研究记忆
- `wiki/index.md` 与 `wiki/log.md` 是互补导航面
- 搜索升级应先走 markdown-first，再考虑更重基础设施
