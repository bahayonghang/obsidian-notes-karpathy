# obsidian-notes-karpathy

当用户描述的是整个 Obsidian vault 工作流，或者问“下一步该做什么”时，用这个入口技能。

如果操作已经很明确，优先直接用对应的操作技能，不要绕 router。

## 主要路由结果

- 新库、半成品、或 legacy-layout，尤其是 `wiki/drafts/`、`wiki/live/`、`wiki/briefings/`、`outputs/reviews/` 还没齐 -> `kb-init`
- `raw/human/**`、`raw/agents/{role}/**`、或 bootstrap 阶段的 `raw/*.md` 领先于 drafts -> `kb-compile`
- 当前马上要处理 pending drafts，或 briefing 应该在这一次 gate pass 里重建 -> `kb-review`
- live 层已准备好检索、归档研究结果或产出对外交付物 -> `kb-query`
- approved 层进入更长期维护，治理视图需要刷新，或 writeback backlog / memory 与 knowledge 混写开始出现 -> `kb-health`

## 路由姿态

- 先看生命周期信号，不只看用户表述
- 把 `kb-review` 当成立即门禁，把 `kb-health` 当成更广义维护入口
- 始终守住 `raw/` 是证据、`wiki/drafts/` 是待审层、`wiki/live/` 才是批准真相层的边界
