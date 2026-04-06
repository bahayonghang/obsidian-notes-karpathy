# AGENTS.md 与 CLAUDE.md

这个 workflow 用两份 schema 文件描述同一个 vault 契约：

- `AGENTS.md`
- `CLAUDE.md`

它们应该表达同一套规则，只保留很少的 agent-specific 包装差异。

## 它们定义什么

- 目录用途
- frontmatter schema
- immutable raw 规则
- 摘要页跟踪字段，如 `source_mtime` 或 source hash
- index 和 log 的职责
- Q&A 默认沉淀规则
