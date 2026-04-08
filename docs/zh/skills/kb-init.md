# kb-init

初始化、修复或迁移到 review-gated 契约。

它会创建或修复：

- `raw/human/**`
- `raw/agents/{role}/**`
- `MEMORY.md`
- `wiki/drafts/**`
- `wiki/live/**`
- `wiki/briefings/**`
- `outputs/reviews/**`
- `wiki/index.md`
- `wiki/log.md`
- `AGENTS.md`
- `CLAUDE.md`

`outputs/qa/**`、`outputs/health/**`、`outputs/content/**` 等下游目录按需创建，不是最小初始化要求。

`kb-init` 也应该补上 `wiki/live/indices/EDITORIAL-PRIORITIES.md`，把“总编层”明确成一个 surface，而不是把优先级散落在聊天里。
