# 技能概览

这个包由一个包级入口技能和四个操作型技能组成。

## 先按生命周期信号选技能

| Vault 或请求出现了什么信号 | 应该路由到 | 原因 |
| --- | --- | --- |
| 支撑层缺失、不完整，或者明显坏了 | `kb-init` | 后续所有技能都依赖这层契约 |
| `raw/` 下有新资料或变更资料等待处理 | `kb-compile` | 编译层落后于源资料层 |
| 编译层已经存在，用户想要答案或交付物 | `kb-query` | 当前任务是提取、综合、归档或发布 |
| 编译层开始陈旧、矛盾、割裂 | `kb-health` | 当前任务是维护、诊断或修复 |
| 用户谈的是整套工作流，或不知道下一步该做什么 | `obsidian-notes-karpathy` | 先由包级入口判断生命周期阶段 |

## 技能矩阵

| 技能 | 核心职责 | 先读什么 | 主要写到哪里 |
| --- | --- | --- | --- |
| `obsidian-notes-karpathy` | 诊断生命周期并路由 | 共享 references、本地 guidance、`wiki/index.md` 顶部、最近的 `wiki/log.md` | 只给出路由建议 |
| `kb-init` | 创建或修复 Vault 契约 | 文件模型和模板类 references | 支撑层、起始索引、本地 guidance |
| `kb-compile` | 把 raw 资料和论文 PDF 编译成可维护的 wiki | 共享模板、本地 guidance、raw 资料 | `wiki/`、派生索引、`wiki/log.md` |
| `kb-query` | 从编译层搜索、回答、归档和发布 | 共享 references、`wiki/index.md`、索引页、既有 Q&A | `outputs/`、`wiki/log.md`，必要时回写 `wiki/` |
| `kb-health` | 为编译层打分并做审计 | 健康评分标准、本地 guidance、编译层内容 | `outputs/health/`、`wiki/log.md`、安全的机械修复项 |

## 全技能共享契约

- `raw/` 对编译器来说是不可变输入层。
- `wiki/` 是持续维护的编译产物层。
- `outputs/qa/` 默认沉淀实质性回答。
- `AGENTS.md` 是必需的本地契约；`CLAUDE.md` 是生成出来的 companion。
- `wiki/index.md` 是内容优先入口。
- `wiki/log.md` 是时间优先入口。
- 检索策略应先坚持 markdown-first，再考虑更重的升级。

## 包内支撑资源

包级入口技能还携带：

- `references/`：schema、summary、concept、entity、Q&A、内容输出、健康检查等模板
- `evals/evals.json`：包级回归提示词
- `evals/fixtures/`：fresh、partial、compiled、drift、broken 等多种 Vault 状态样例
