# kb-review

负责 draft knowledge 与已批准知识层之间的显式门禁。这是待审草稿与本轮角色简报重建的即时入口。

## 它会读

- `wiki/drafts/**`
- 直接引用的 raw captures
- 重叠的 `wiki/live/**`

## 它会写

- `outputs/reviews/**`
- `wiki/live/**`
- `wiki/briefings/**`
- `wiki/log.md` 中的 `review` / `brief` 事件

## 审校姿态

- 只审 draft package、直接引用的 raw captures，以及重叠的 live 页面
- 不因为产出它的 Agent “看起来靠谱”就放宽门
- 不只判断“像不像对的”，还要判断它是否值得进入长期知识层

## 决策模型

- 证据充分、无冲突、无阻塞时自动批准
- 证据弱、blocking flags 未解除时自动拒绝
- 与 live 冲突、别名/重复风险未解，或分数落在模糊带时升级人工复核

`kb-review` 还应该显式判断 fact / inference 分离、alias 对齐、duplicate 风险、矛盾是否被记录，以及页面是否值得长期保留。切到 `维护模式` 时，它还要审计 drift、backlog pressure、provenance、过期归档产物、creator consistency（`CLAUDE.md` / `MEMORY.md` / `_style-guide.md` / briefing），以及已批准层面上的图结构薄弱点。

archive hygiene 也是 maintenance 的一部分：

- archived Q&A 相对 live pages 已经过时
- archived publish artifacts 出现 reuse drift
- archived outputs 里有 writeback backlog
- archived outputs 出现 scope leak

这些都属于“维护 artifact archive”，不是“把 archive 升格成真相层”。

