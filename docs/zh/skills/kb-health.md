# kb-health

对 approved live brain 及其 review 生态做维护体检。这是更长期的维护入口，不是即时的 draft promotion gate。

重点审计：

- `wiki/live/**`
- `wiki/briefings/**`
- `outputs/qa/**`
- `outputs/content/**`
- `outputs/reviews/**`
- `outputs/health/**`

只有在 provenance 需要 spot check 时才回看 `raw/`。

## 典型结果

- 标记批准层冲突与重复概念
- 标记过期的归档 Q&A
- 标记 stale briefings
- 标记 review backlog
- 标记 writeback backlog
- 标记 provenance、source drift、render 或 alias 问题
- 标记协作记忆与批准层知识混写
- 在需要时刷新 `QUESTIONS.md`、`GAPS.md`、`ALIASES.md`

## 报告与修复策略

健康报告默认写入 `outputs/health/health-check-{date}.md`。

`kb-health` 默认先出报告。

只有在目标明确、可逆、纯机械时，才允许修改 `wiki/live/**`、`wiki/briefings/**`、`outputs/qa/**`、`outputs/content/**` 等 approved surfaces。

不能改 `raw/`，也不能把 drafts 直接提升到 live。
