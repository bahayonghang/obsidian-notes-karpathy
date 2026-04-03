# 目录结构

了解 Karpathy 风格知识库的组织结构。

## 概览

运行 `kb-init` 后，你的 vault 结构如下：

```
vault/
├── raw/                  # 原始资料（文章、论文、推文...）
│   └── assets/           # 来源中的图片
├── wiki/                 # LLM 编译的知识库（请勿手动编辑）
│   ├── concepts/         # 每个关键概念一篇文章
│   ├── summaries/        # 每个原始来源一篇摘要
│   └── indices/          # INDEX.md, CONCEPTS.md, SOURCES.md, RECENT.md
├── outputs/              # 生成的内容
│   ├── reports/          # Markdown 研究报告
│   ├── slides/           # Marp 幻灯片
│   └── charts/           # Mermaid 图表、Canvas 文件
└── AGENTS.md             # LLM 代理的 Schema 定义
```

## 目录详情

### `raw/` — 原始资料

**用途**：收集所有原始信息来源。

**谁写入**：人类（通过 Web Clipper 或手动添加）

**内容**：
- 从网页剪辑的文章
- 研究论文（转换为 Markdown）
- 推文串
- 视频转录
- 仓库 README
- 读书笔记

**Frontmatter schema**：

```yaml
---
title: "文章标题"
source: "https://example.com/article"
author: "作者姓名"
date: 2026-04-01
type: article | paper | repo | dataset | tweet | video | book | other
tags:
  - topic/subtopic
clipped_at: 2026-04-01T12:00:00
compiled_at: null          # 由 kb-compile 处理后设置
---
```

**重要**：`compiled_at` 字段用于编译器判断来源是否需要处理。新来源保持为 `null`。

### `raw/assets/` — 图片和附件

**用途**：存储原始来源中的图片、图表和其他媒体。

**谁写入**：Web Clipper 或人类手动添加

**用法**：在原始源文件中使用标准 Markdown 图片语法引用：`![description](assets/image.png)`

### `wiki/concepts/` — 概念文章

**用途**：每个关键概念、技术、人物、工具或主题各一篇文章。

**谁写入**：仅 LLM（通过 `kb-compile`）

**内容**：随着更多来源被编译而不断成长的综合文章。每个概念应该：
- 自包含且可独立理解
- 通过 wikilink 链接到相关概念
- 可追溯回原始来源

**Frontmatter schema**：

```yaml
---
title: "概念名称"
aliases:
  - "替代名称"
category: "父类别"
tags:
  - concept
  - topic/subtopic
related:
  - "[[其他概念]]"
sources:
  - "[[raw/来源文件]]"
created_at: 2026-04-01T12:00:00
updated_at: 2026-04-01T12:00:00
---
```

**文章结构**：

```markdown
# 概念名称

## 定义

清晰简洁的定义 — 1-2 句话。

## 概述

来自所有来源的综合解释。本节会随着更多来源的编译而扩展。

## 关键方面

### 方面 1

来自来源的详情。

### 方面 2

来自来源的详情。

## 关联

- [[相关概念 1]] — 它们之间的关系
- [[相关概念 2]] — 它们之间的关系

## 来源

- [[wiki/summaries/source-1]] — 此来源的贡献
- [[wiki/summaries/source-2]] — 此来源的贡献
```

### `wiki/summaries/` — 来源摘要

**用途**：每个原始来源各一篇摘要，提取关键事实、论点和数据。

**谁写入**：仅 LLM（通过 `kb-compile`）

**内容**：每个原始来源的简洁提炼，并链接到相关概念。

**Frontmatter schema**：

```yaml
---
title: "摘要：来源标题"
source_file: "[[raw/来源文件]]"
source_url: "https://example.com"
key_concepts:
  - "[[概念 A]]"
  - "[[概念 B]]"
created_at: 2026-04-01T12:00:00
---
```

### `wiki/indices/` — 索引文件

**用途**：整个知识库的导航和概览文件。

**谁写入**：仅 LLM（通过 `kb-compile`）

**文件**：

| 文件 | 用途 |
|------|------|
| `INDEX.md` | 主索引，包含统计数据和所有文章 |
| `CONCEPTS.md` | 按类别分组的映射 |
| `SOURCES.md` | 所有原始来源及其编译状态的注册表 |
| `RECENT.md` | 最近变更的日志 |

### `outputs/` — 生成内容

**用途**：查询知识库的结果。

**谁写入**：仅 LLM（通过 `kb-query`）

**子目录**：

| 目录 | 内容 |
|------|------|
| `reports/` | Markdown 研究报告 |
| `slides/` | Marp 幻灯片 |
| `charts/` | Mermaid 图表、Canvas 文件 |

### `AGENTS.md` — Schema 定义

**用途**：定义在此知识库上运行的 LLM 代理的规则、约定和 frontmatter schema。

**谁写入**：LLM 在 `kb-init` 期间创建，按需更新

**内容**：
- 目录结构文档
- 所有文件类型的 Frontmatter schema
- 编译规则
- 文件命名约定
- 特定主题指导

## 文件命名约定

| 文件类型 | 约定 | 示例 |
|----------|------|------|
| 原始文件 | 保持原始名称或 `{date}-{slug}.md` | `2026-04-03-attention-is-all-you-need.md` |
| 概念 | `{concept-name}.md`（小写，连字符） | `transformer-architecture.md` |
| 摘要 | `{source-file-name}.md` | `attention-is-all-you-need.md` |
| 报告 | `{date}-{topic}.md` | `2026-04-03-transformer-evolution.md` |
| 幻灯片 | `{date}-{topic}.md` | `2026-04-03-attention-mechanisms.md` |

## 接下来

- [**AGENTS.md Schema**](/guide/agents-schema) — 深入了解 schema
- [**kb-init 技能**](/skills/kb-init) — 初始化如何工作
- [**工作流指南**](/workflow/overview) — 使用目录结构
