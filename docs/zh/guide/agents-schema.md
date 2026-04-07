# AGENTS.md 与 CLAUDE.md

这个 workflow 的本地契约以 `AGENTS.md` 为主。

`kb-init` 还会生成一份 `CLAUDE.md` 作为 companion file，让不同编码代理读到同一套 Vault 规则。

## 它们定义什么

- 目录用途
- frontmatter schema
- immutable raw 规则
- 摘要页跟踪字段，如 `source_mtime` 或 source hash
- index 和 log 的职责
- Q&A 默认沉淀规则

## 为什么要有两份

不同编码代理会优先读取不同的顶层 guidance 文件。把两份都生成出来，才能在不改变 workflow 的前提下保持跨 agent 可移植。

实际强度约定如下：

- `AGENTS.md` 是必需的本地 guidance
- `CLAUDE.md` 是生成出来的 companion
- 如果一个本来可用的 Vault 只缺 `CLAUDE.md`，应把它视为修复目标，而不是直接阻断 compile、query 或 health
