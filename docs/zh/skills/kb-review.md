# kb-review

负责 draft knowledge 和 approved live brain 之间的显式门禁。这是 pending drafts 与本轮 briefing 重建的即时 gate 入口。

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
- 不只判断“像不像对的”，还要判断它是否值得进入长期脑

## 决策模型

- 证据充分、无冲突、无阻塞时自动批准
- 证据弱、blocking flags 未解除时自动拒绝
- 与 live 冲突、别名/重复风险未解，或分数落在模糊带时升级人工复核

`kb-review` 还应该显式判断 fact / inference 分离、alias 对齐、duplicate 风险、矛盾是否被记录，以及页面是否值得长期保留。切到 `maintenance` mode 时，它还要审计 drift、backlog pressure、provenance、过期 archived outputs，以及 approved surfaces 上的图结构薄弱点。
