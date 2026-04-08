# kb-health

对 approved live brain 及其 review 生态做维护体检。

重点审计：

- `wiki/live/**`
- `wiki/briefings/**`
- `outputs/qa/**`
- `outputs/content/**`
- `outputs/reviews/**`
- `outputs/health/**`

## 典型结果

- 标记 stale briefings
- 标记 review backlog
- 标记 writeback backlog
- 标记 provenance 或 render 问题
- 标记协作记忆与批准层知识混写

## 修复策略

`kb-health` 默认先出报告。

只有在目标明确、可逆、纯机械时，才允许修改 `wiki/live/**`、`wiki/briefings/**`、`outputs/qa/**` 等 approved surfaces。

不能改 `raw/`，也不能把 drafts 直接提升到 live。
