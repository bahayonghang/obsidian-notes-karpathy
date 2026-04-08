# 快速开始

## 0. 先迁移 legacy-layout

如果 vault 还在直接使用 `wiki/summaries/` 或 `wiki/concepts/`，先用 `kb-init` 做迁移，再进入正常的 compile / query / health 流程。

## 1. 建好必需的支撑层

最小支撑层需要有：

- `raw/human/**`
- `raw/agents/{role}/**`
- `wiki/drafts/**`
- `wiki/live/**`
- `wiki/briefings/**`
- `outputs/reviews/**`
- `wiki/index.md`
- `wiki/log.md`
- `AGENTS.md`
- `CLAUDE.md`

`outputs/qa/**`、`outputs/health/**`、`outputs/content/**` 等下游输出目录按阶段按需创建，不是初始化最低要求。

`MEMORY.md` 推荐一开始就建好，但它不是真相层。它只承接偏好、协作规则和编辑优先级。

## 2. 放入捕获内容

- 人类整理的资料放进 `raw/human/**`
- Agent 自动产出的内容放进 `raw/agents/{role}/**`
- bootstrap 阶段也可以直接把 markdown 放进 `raw/`，但这不等于已经具备完整支撑层

## 3. 编译成草稿

运行 `kb-compile`，输出应该落在 `wiki/drafts/`。

## 4. 通过审校门

运行 `kb-review`，把合格草稿提升到 `wiki/live/`，并重建 `wiki/briefings/`。

## 5. 基于批准层提问

运行 `kb-query`，只从 `wiki/live/` 和相关 briefing 做检索与生成，不把 `MEMORY.md` 当专题知识来源。

如果一个回答暴露出值得长期保留的结论，应该把它记录成明确的 writeback 候选，重新走 compile / review 闭环。

## 6. 做一次 health baseline

运行 `kb-health`，报告默认写入 `outputs/health/health-check-{date}.md`。health 流程默认先出报告，只在目标明确时做确定性的机械修复。
