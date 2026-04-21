# obsidian-notes-karpathy

当用户描述的是整个 Obsidian vault 工作流、问“下一步该做什么”，或者只会说“LLM Wiki”“Karpathy wiki”“Obsidian IDE”“知识编译器”“个人知识库 / second brain”，甚至直接说 `来源页`、`主题页`、`实体页`、`综合页`、`output/analyses`、`output/reports` 这类 `Chinese-LLM-Wiki` 话术但没有说明具体操作时，用这个入口技能。

如果操作已经很明确，优先直接用对应的操作技能，不要绕路由入口。

## 主要路由结果

- 新库、半成品，或仍是旧版目录结构，尤其是 `wiki/drafts/`、`wiki/live/`、`wiki/briefings/`、`outputs/reviews/` 还没齐 -> `kb-init`
- `raw/_manifest.yaml` 已过期，或 raw source 变动还没登记 -> `kb-ingest`
- `raw/human/**`、`raw/agents/{role}/**`、或 bootstrap 阶段的 `raw/*.md` 领先于 drafts -> `kb-compile`
- 当前马上要处理待审草稿，或角色简报应该在这一次审校流程里重建 -> `kb-review`
- live 层已准备好检索、候选排序、归档研究结果复用或静态 web 导出 -> `kb-query`
- 用户要把一个有价值的回答归档回 vault，或想复用已有 archive -> `kb-query`
- 用户明确沿用旧的 `kb-search` 说法 -> `kb-query`
- 用户明确要 slides / reports / charts / canvas 这类确定性派生产物 -> `kb-render`
- 已批准知识层或 archived outputs 进入更长期维护，治理视图需要刷新，或 writeback backlog / memory 与 knowledge 混写开始出现 -> `kb-review`

## 路由姿态

- 先看生命周期信号，不只看用户表述
- 如果用户主要想知道“我现在在哪个阶段、下一步是什么”，优先用 status wrapper 给出简明结论
- 把 `kb-review` 当成统一治理技能，内部再区分即时审校和维护模式
- 始终守住 `raw/` 是证据、`wiki/drafts/` 是待审层、`wiki/live/` 才是批准真相层的边界
- 把更简单的 `raw/wiki/output` 话术翻译到当前 draft/live 契约上，而不是机械照搬旧结构


