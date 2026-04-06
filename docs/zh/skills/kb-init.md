# kb-init

为 Karpathy 式工作流初始化新的 Obsidian vault 或子目录。

## 会创建什么

```text
raw/{articles,papers,podcasts,assets}
wiki/{concepts,summaries,indices}
wiki/index.md
wiki/log.md
outputs/{qa,health,reports,slides,charts,content/{articles,threads,talks}}
AGENTS.md
CLAUDE.md
```

## 关键行为

- 同时生成 `AGENTS.md` 和 `CLAUDE.md`
- 两份 schema 文件表达同一个 vault 契约
- 明确 `raw/` 是 immutable 层
- 创建零状态索引和日志文件，供后续技能稳定接入
- 可选创建 publish mode 目录
- 预置示例概念页和 clipper 模板
