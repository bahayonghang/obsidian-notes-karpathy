# AGENTS.md Schema

管理 LLM 代理在知识库上操作的 schema 和规则。

## 什么是 AGENTS.md？

`AGENTS.md` 是在你的知识库上工作的 LLM 代理的**规则手册**。它在 `kb-init` 期间创建，定义了：

- 目录结构及其用途
- 所有文件类型的 Frontmatter schema
- 编译规则和约定
- 文件命名约定
- 特定主题指导

把它看作是你知识库的**"编译器规范"**。

## 核心部分

### 1. 概览

描述知识库的主题和用途：

```markdown
# Knowledge Base: {Topic Name}

这是一个遵循 Karpathy 工作流模式的 LLM 编译知识库。
原始来源收集在 `raw/` 中，然后由 LLM 编译为 `wiki/` 中的结构化 wiki。
wiki 是 LLM 的领域——人类在 Obsidian 中阅读但很少直接编辑。
```

### 2. 目录结构表

记录每个目录的用途和所有权：

| 目录 | 用途 | 谁写入 |
|------|------|--------|
| `raw/` | 原始资料（文章、论文、剪辑） | 人类（通过 Web Clipper、手动） |
| `raw/assets/` | 来源中的图片和附件 | 人类 / Web Clipper |
| `wiki/concepts/` | 概念文章（每个关键概念各一篇） | 仅 LLM |
| `wiki/summaries/` | 每个原始来源各一篇摘要 | 仅 LLM |
| `wiki/indices/` | 导航用索引文件 | 仅 LLM |
| `outputs/reports/` | Q&A 生成的研究报告 | 仅 LLM |
| `outputs/slides/` | Marp 幻灯片 | 仅 LLM |
| `outputs/charts/` | 图表和可视化 | 仅 LLM |

### 3. Frontmatter Schema

#### 原始源文件 (`raw/*.md`)

```yaml
---
title: "文章标题"              # 必填：来源标题
source: "https://example.com"       # 必填：原始 URL
author: "作者姓名"               # 可选：作者
date: 2026-04-01                    # 必填：发布日期
type: article | paper | repo | dataset | tweet | video | book | other
tags:                               # 必填：2-5 个主题标签
  - topic/subtopic
clipped_at: 2026-04-01T12:00:00    # 必填：添加时间
compiled_at: null                   # 由 kb-compile 处理后设置
---
```

#### Wiki 概念文章 (`wiki/concepts/*.md`)

```yaml
---
title: "概念名称"               # 必填：主要名称
aliases:                            # 可选：替代名称
  - "替代名称"
category: "父类别"         # 可选：用于 CONCEPTS.md 中分组
tags:                               # 必填：至少包含 'concept' 标签
  - concept
  - topic/subtopic
related:                            # 可选：双向 wikilink
  - "[[其他概念]]"
sources:                            # 必填：提及此概念的源文件
  - "[[raw/来源文件