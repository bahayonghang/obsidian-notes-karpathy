# 目录结构

Vault 被有意拆成不可变输入层、编译知识层和输出层。

```text
vault/
├── raw/
│   ├── *.md
│   ├── articles/
│   ├── papers/
│   ├── podcasts/
│   ├── assets/
│   └── repos/              # 可选
├── wiki/
│   ├── concepts/
│   ├── summaries/
│   ├── indices/
│   ├── entities/           # 可选
│   ├── index.md
│   └── log.md
├── outputs/
│   ├── qa/
│   ├── health/
│   ├── reports/
│   ├── slides/
│   ├── charts/
│   └── content/
│       ├── articles/
│       ├── threads/
│       └── talks/
├── AGENTS.md
└── CLAUDE.md
```

## 所有权模型

| 路径 | 主要维护者 | 作用 |
| --- | --- | --- |
| `raw/` | 人类 | 不可变原始资料和 inbox 式采集笔记 |
| `wiki/summaries/` | `kb-compile` 驱动的 LLM | 带跟踪元数据的摘要页 |
| `wiki/concepts/` | `kb-compile` 和 `kb-query` | 持续累积证据的概念页 |
| `wiki/entities/` | 启用后由 LLM 维护 | 人物、组织、产品、仓库等稳定实体页 |
| `wiki/indices/` | LLM 派生层 | `INDEX.md`、`CONCEPTS.md`、`SOURCES.md`、`RECENT.md` 等导航面 |
| `wiki/index.md` | LLM 派生层 | 编译知识层的内容入口 |
| `wiki/log.md` | LLM 追加 | `ingest`、`query`、`publish`、`health` 事件时间线 |
| `outputs/qa/` | `kb-query` | 持久化研究答案 |
| `outputs/health/` | `kb-health` | 评分式维护报告 |
| `outputs/content/` 及同级输出目录 | `kb-query` | 从 wiki 证据层生成的对外内容和交付物 |

## 可选扩展

- `raw/repos/` 适合放 repo 快照或伴随笔记，不是每个 Vault 都需要。
- `wiki/entities/` 适合命名实体稳定、需要长期维护的领域。
- 一旦启用 `wiki/entities/`，就应该同时维护 `wiki/indices/ENTITIES.md`。

## 命名与兼容性规则

- 文件名优先用小写 kebab-case。
- 标准索引目录名是 `wiki/indices/`。
- 旧 Vault 如果使用 `wiki/indexes/`，需要兼容。
- Markdown 表格单元格里不要使用 alias-style wikilink，这在 Obsidian 里渲染很差。

## 关键后果

不要把编译状态写回 raw。应把状态放到摘要 frontmatter、派生索引、日志或健康报告里。
