# kb-compile — 增量 Wiki 编译

Karpathy 知识库工作流的核心引擎。将原始源材料转换为结构化的、相互链接的 wiki。

## 何时使用

- 向 `raw/` 添加新源后（通过 Web Clipper 或手动）
- 用户说"compile"、"编译"、"更新知识库"、"sync wiki"
- 用户说"lint"、"health check"、"检查知识库"（仅运行第三阶段或完整流水线）
- 定期运行以保持 wiki 新鲜和良好连接

**这是核心技能**——向 `raw/` 添加新源后运行它。

## 编译理念

> 原始数据是"事实来源"，wiki 是"编译产物"。像编译器一样，这个过程是确定性的和增量的——只有新的或变更的源会触发重新编译。

## 前置条件

- 知识库必须已初始化（之前运行过 `kb-init`）
- `AGENTS.md` 必须存在于 vault/项目根目录
- 首先阅读 `AGENTS.md` 以理解 schema 和约定

## 编译流水线

### 第一阶段：预处理 raw/（摄入）

扫描 `raw/` 并准备源材料以进行编译。

#### 1.1 发现源

列出 `raw/` 中的所有 `.md` 文件（不包括 `raw/assets/`）。对于每个文件：

1. 读取其 frontmatter
2. 检查 `compiled_at` 是否为 null 或缺失 → **新源，需要编译**
3. 检查文件 mtime > `compiled_at` → **已更新源，需要重新编译**
4. 否则 → **跳过**（已编译且未变更）

**报告**："发现 X 个新源，Y 个更新源，Z 个未变更（跳过）。"

#### 1.2 验证和充实 Frontmatter

对于每个新/更新源，验证所需 frontmatter 字段是否存在。如果缺失，推断它们：

| 字段 | 推断策略 |
|------|---------|
| `title` | 从第一个 `# 标题` 或文件名提取 |
| `source` | 在内容中查找 URL |
| `date` | 使用文件创建日期或从内容提取 |
| `type` | 从内容推断（article、paper、tweet、repo 等） |
| `tags` | 从内容分析生成 2-5 个相关主题标签 |
| `clipped_at` | 如果缺失则使用文件创建时间 |

用丰富的字段原地更新原始文件的 frontmatter。

#### 1.3 更新 SOURCES.md

重建 `wiki/indices/SOURCES.md`，包含所有原始源的完整表格，包括它们的类型、日期和编译状态。

### 第二阶段：增量 Wiki 编译

核心编译步骤。处理每个新/更新源并将其集成到 wiki 中。

#### 2.1 生成摘要

对于每个要编译的源，创建或更新 `wiki/summaries/{source-filename}.md`：

```markdown
---
title: "Summary: {Source Title}"
source_file: "[[raw/{filename}]]"
source_url: "{url if available}"
key_concepts:
  - "[[Concept A]]"
  - "[[Concept B]]"
created_at: {datetime}
---

# Summary: {Source Title}

> [!abstract] Source
> **Type**: {type} | **Author**: {author} | **Date**: {date}
> **Source**: [[raw/{filename}]]

## Key Points

- {要点 1 — 最重要的收获}
- {要点 2}
- {要点 3}

## Detailed Summary

{2-4 段，涵盖主要内容、论点、数据和结论}

## Key Concepts

- [[Concept A]] — {此源与该概念的关系}
- [[Concept B]] — {此源与该概念的关系}

## Notable Quotes / Data

> {来自源的直接引用或具体数据点}

## Related Sources

- [[wiki/summaries/other-source]] — {关系}
```

#### 2.2 提取和编译概念

摘要完成后，识别值得拥有自己 wiki 文章的独特概念。**概念**是在多个源中反复出现的想法、技术、人物、工具或主题，或者足够重要值得专门覆盖。

对于每个概念，在 `wiki/concepts/{concept-name}.md` **创建**新文章或**更新**现有文章：

```markdown
---
title: "{Concept Name}"
aliases:
  - "{Alternative Name}"
category: "{Parent Category}"
tags:
  - concept
  - {topic/subtopic}
related:
  - "[[Other Concept]]"
sources:
  - "[[raw/source-1]]"
  - "[[raw/source-2]]"
created_at: {datetime}
updated_at: {datetime}
---

# {Concept Name}

## Definition

{清晰简洁的概念定义 — 1-2 句话}

## Overview

{综合解释，来自所有提及此概念的源。
随着编译更多源，这一部分会增长。}

## Key Aspects

### {Aspect 1}

{来自源的细节}

### {Aspect 2}

{来自源的细节}

## Connections

- [[Related Concept 1]] — {它们的关系}
- [[Related Concept 2]] — {它们的关系}

## Sources

- [[wiki/summaries/source-1]] — {此源的贡献}
- [[wiki/summaries/source-2]] — {此源的贡献}
```

**更新现有概念时：**
- 将新信息合并到现有部分（不要重复）
- 将新源添加到 frontmatter 的 `sources` 列表
- 更新 `updated_at` 时间戳
- 添加发现的任何新连接
- **保留现有内容**——追加和完善，永不删除

#### 2.3 维护 Wikilinks

编译所有源后，扫描所有 wiki 文章并确保：

1. **前向链接**：每个已知概念的提及都用 `[[wikilinks]]` 包裹
2. **反向链接**：Obsidian 自动处理，但验证 frontmatter 中的 `related` 字段是双向的
3. **交叉引用**：摘要链接到概念，概念链接到摘要和其他概念

#### 2.4 更新索引文件

重建所有索引文件以反映 wiki 的当前状态。

**`wiki/indices/INDEX.md`** — 主索引：

```markdown
## Statistics
- Total sources: {count}
- Total concepts: {count}
- Total summaries: {count}
- Last compiled: {datetime}

## Concepts
| Concept | Category | Sources | Updated |
|---------|----------|---------|---------|
| [[concept-name]] | {category} | {count} | {date} |

## Summaries
| Summary | Source Type | Date |
|---------|------------|------|
| [[summary-name]] | {type} | {date} |
```

**`wiki/indices/CONCEPTS.md`** — 按类别的概念地图：

```markdown
## {Category 1}
- [[Concept A]] — {单行描述}
  - Related: [[Concept B]], [[Concept C]]
- [[Concept B]] — {单行描述}

## {Category 2}
...
```

**`wiki/indices/RECENT.md`** — 最近变更：

```markdown
## {Today's Date}
- Compiled {N} new sources
- Created concepts: [[Concept X]], [[Concept Y]]
- Updated concepts: [[Concept Z]]

## {Previous Date}
...
```

#### 2.5 标记源为已编译

对于每个成功编译的源，更新其 frontmatter：

```yaml
compiled_at: {current datetime}
```

这防止下次运行时重新编译，除非文件被修改。

### 第三阶段：健康检查（Lint）

在编译后自动运行，或在用户请求健康检查时独立运行。

#### 3.1 一致性检查

- 扫描概念文章中的矛盾陈述
- 标记过时信息（超过阈值的源）
- 验证所有 wikilinks 解析到现有文件（无断链）

#### 3.2 缺失数据检测

- 识别内容稀疏的概念文章（< 100 字）
- 查找没有标签或 frontmatter 不完整的原始源
- 检测缺少 `key_concepts` 的摘要

#### 3.3 连接发现

- 成对比较所有概念文章，寻找潜在但未建立的连接
- 建议在共享主题或关键词的文章之间添加新 wikilinks
- 识别在文本中提及但没有自己文章的概念

#### 3.4 孤立检测

- 查找没有对应摘要的原始源
- 查找没有入链的概念文章
- 查找没有链接到任何概念的摘要

#### 3.5 生成健康报告

将结果写入 `outputs/reports/health-check-{date}.md`：

```markdown
---
title: "Health Check Report"
date: {datetime}
---

# Knowledge Base Health Check

## Summary
- Overall health: {Good / Needs Attention / Poor}
- Sources: {total} ({new} new, {orphaned} orphaned)
- Concepts: {total} ({sparse} sparse, {well-connected} well-connected)
- Broken links: {count}

## Issues Found

### Critical
> [!danger] {Issue description}
> {Details and suggested fix}

### Warnings
> [!warning] {Issue description}
> {Details and suggested fix}

### Suggestions
> [!tip] {Suggestion}
> {Details}

## Recommended Actions
1. {Action item with specific file references}
2. {Action item}
```

## 执行说明

- **始终先阅读 `AGENTS.md`** 以了解此知识库的特定 schema
- 使用 `obsidian-markdown` 技能处理所有 Markdown 语法
- 使用 `obsidian-cli` 技能在可用时搜索和阅读 vault 内容
- **按时间顺序处理源**（最早的优先）以获得一致的概念演化
- 当 wiki 很大时（>50 个概念），编译前阅读 `wiki/indices/INDEX.md` 以了解现有结构并避免创建重复概念
- 对于非常大的编译（>10 个新源），考虑分批：每次编译 5 个，更新索引，然后继续
- **始终报告进度**："正在编译源 3/7：{filename}..."

## 下一步

- [**kb-query**](/skills/kb-query) — 搜索和查询已编译的 wiki
- [**健康检查**](/workflow/health-checks) — 理解 lint 流水线
- [**工作流：编译**](/workflow/compile) — 详细编译工作流