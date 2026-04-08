# obsidian-notes-karpathy

当用户描述的是整个 Obsidian vault 工作流，或者问“下一步该做什么”时，用这个入口技能。

如果操作已经很明确，优先直接用对应的操作技能，不要绕 router。

## 主要路由结果

- 新库、半成品、或 legacy-layout -> `kb-init`
- raw 捕获领先于 drafts，或 bootstrap 阶段直接放在 `raw/*.md` 的内容需要编译 -> `kb-compile`
- 草稿待审或 briefing 过期 -> `kb-review`
- live 层已准备好检索 -> `kb-query`
- approved 知识层需要体检，或 writeback backlog / memory 与 knowledge 混写开始出现 -> `kb-health`
