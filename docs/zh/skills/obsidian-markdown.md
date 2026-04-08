# Obsidian Markdown

> 搭配技能说明：这是相邻生态技能页面，不属于本仓库当前发货的 6 个核心技能。

Obsidian Flavored Markdown 语法参考——所有 wiki 内容的基础。

## 概览

知识库中的所有内容使用 **Obsidian Flavored Markdown**，它在标准 Markdown 基础上扩展了额外的知识管理功能。

## 关键特性

### Wikilinks

最重要的功能。在笔记之间创建双向链接：

```markdown
[[Concept Name]]
[[concept-name|Custom Link Text]]
[[path/to/note]]
```

Wikilinks 在 wiki 中广泛使用，用于连接：
- 摘要到概念
- 概念到相关概念
- 概念回到源
- 报告到源材料

### Callouts

带图标和颜色的富信息块：

```markdown
> [!info] Title
> Informational content

> [!warning] Title
> Warning content

> [!danger] Title
> Critical/dangerous content

> [!tip] Title
> Helpful tip

> [!abstract] Title
> Summary/abstract content

> [!example] Title
> Example content
```

Callouts 用于：
- 索引文件（关于自动维护的信息块）
- 摘要（源信息的摘要块）
- 健康报告（warning、danger、tip 块）

### YAML Frontmatter

Markdown 文件顶部的元数据：

```yaml
---
title: "Document Title"
tags:
  - tag1
  - tag2
---
```

用于：
- 源跟踪（`compiled_at`、`clipped_at`）
- 概念元数据（`category`、`related`、`sources`）
- 组织和过滤

### Tags

使用斜杠的层级标签：

```markdown
#topic/subtopic
#concept
#report
```

标签支持：
- 过滤搜索结果
- 分组相关内容
- 识别文件类型

## 在知识库中的使用

每个 SKILL.md 文件和 wiki 文档都使用这些特性。LLM 必须：

1. **始终使用 `[[wikilinks]]`** 而不是标准 Markdown 链接用于内部链接
2. **使用 callout 块** 用于元数据和警告
3. **包含适当的 frontmatter** 根据 AGENTS.md 中的 schema
4. **使用层级标签** 用于一致的组织

## 参考

- [Obsidian Help: Markdown](https://help.obsidian.md/Editing+and+formatting/Basic+formatting+syntax)
- [Obsidian Help: Wikilinks](https://help.obsidian.md/Linking+notes+and+attachments/Internal+links)
- [Obsidian Help: Callouts](https://help.obsidian.md/Editing+and+formatting/Callouts)
