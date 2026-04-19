# 技能总览

这个 bundle 现在包含 1 个入口技能和 6 个操作技能，也应该对那些只会说“LLM Wiki”“Karpathy wiki”“Obsidian IDE”“知识编译器”“second brain”的用户足够可发现。

## 按生命周期信号选择

| 信号 | 路由到 | 原因 |
| --- | --- | --- |
| 支撑层缺失、半成品，或仍是旧版目录结构 | `kb-init` | 先把带审校与批准流程的契约建好 |
| 支撑层已存在，但来源登记表已过期 | `kb-ingest` | 先把 manifest 刷新好，再进入 compile |
| 新 raw 捕获还没进草稿层 | `kb-compile` | draft 层落后于 evidence 层 |
| 草稿待审，或角色简报已过期且应在下一次审校流程中重建 | `kb-review` | 当前最安全的下一步是处理具体 draft package，或立刻重建角色简报 |
| live 层已存在且用户要答案、候选排序、历史答案复用、对外文章 / thread，或静态知识站 | `kb-query` | 当前任务是提取、综合、publish-mode prose、归档复用或静态导出 |
| 用户想要确定性的幻灯片 / 报告 / 图表简报 / Canvas | `kb-render` | 当前任务是把已批准知识变成确定性派生产物 |
| 用户明确沿用旧的 `kb-search` 说法 | `kb-query` | `kb-search` 会直接落到统一查询技能 |
| 已批准知识层变脏、矛盾、断链，或 backlog pressure 已经变成维护问题 | `kb-review` | 统一治理技能会切到 `维护模式` 处理已批准层面的维护工作 |
| 用户在聊整个 Obsidian vault 工作流，或问下一步该做什么 | `obsidian-notes-karpathy` | 先做生命周期诊断，再分流到正确技能 |

## 共享合同

- `raw/` 永远不可变。
- `raw/_manifest.yaml` 是来源登记表。
- `raw/` 也承担长期素材库的角色；创作者的写作、复用与发布编排应落在下游工作面，而不是重写 source captures。
- `raw/**/papers/*.pdf` 下的论文 PDF 仍然是 `paper-workbench` 路由例外，不属于普通 `kb-compile` 入口。
- `MEMORY.md` 是协作记忆层，不是专题真相层。
- `wiki/drafts/` 可审校，但不是真相层。
- `wiki/live/` 是批准后的长期知识层。
- `wiki/live/topics/` 是批准知识的默认浏览层。
- `wiki/briefings/` 只能从 live 构建。
- `outputs/reviews/` 存 promotion 决策。
- `wiki/live/indices/QUESTIONS.md`、`GAPS.md`、`ALIASES.md` 这类治理视图更适合成熟 vault，但它们必须以已批准的 live 页面为基础，不能直接把归档产物抬升成真相层。
- `outputs/qa/` 与 `outputs/content/` 里的归档内容可以提供维护信号、writeback backlog 与后续路由，但不会自动变成已批准知识层。
- source retention archive（`raw/**` + `raw/_manifest.yaml`）和 artifact archive（`outputs/**`）都是持久面，但都不会替代 `wiki/live/**`。
- 资料还没进入 `raw/` 时，网页采集应先走 `web-access` 或 Obsidian Web Clipper；核心 bundle 从 vault 内部生命周期开始接管。
- curated hub 或编辑规划面可以存在，但它们仍然是导航 / 维护层，而不是绕过真相边界的捷径。
- `wiki/index.md` 是内容优先入口。
- `wiki/log.md` 是时间优先入口。
- 旧版目录结构 vault 先经 `kb-init` 迁移，再进入正常流程。

设计理由与后续演化请看 [架构说明](/zh/architecture/overview)。

## 搭配技能

| 需求 | 核心路由 | 搭配技能 |
| --- | --- | --- |
| `raw/**/papers/*.pdf` 论文 PDF | 不属于核心路由 | `paper-workbench` |
| 超出普通确定性渲染的 canvas 专项产物 | 普通派生产物仍走 `kb-render` | 专门的 Obsidian canvas 工具链 |
| 在运行中的 Obsidian 里做 CLI 人机工效增强 | 生命周期仍由核心 bundle 管 | 可选的 Obsidian CLI 生态技能 |

文档里也保留了 `obsidian-cli`、`obsidian-markdown`、`obsidian-canvas-creator` 这类搭配技能页面。

它们属于相邻 Obsidian 生态参考，不是本仓库当前发货的核心 1+6 bundle。


