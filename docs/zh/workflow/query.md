# 查询与输出

通过搜索、问答和多格式输出生成，从编译好的知识库中提取价值。

## 概览

一旦你的 wiki 编译完成，`kb-query` 技能让你能够**提取价值**从所有策划的知识中。这是投资回报的地方。

## 三项能力

### 1. 搜索

**何时**：你想在 wiki 中查找特定信息

**触发**："search kb"、"搜索知识库"、"查找关于...的信息"

**过程**：
1. 从 `wiki/indices/INDEX.md` 开始了解概览
2. 检查 `wiki/indices/CONCEPTS.md` 查看概念地图
3. 使用 `obsidian-cli` 或 Grep 进行全文搜索
4. 按 frontmatter 中的标签过滤

**输出**：相关文章的结构化列表，带摘录

### 2. Q&A 研究

**何时**：你有复杂的问题

**触发**："query kb"、"问知识库"、"research"、"wiki 怎么说关于..."

**过程**：
1. 解析问题中的关键概念
2. 导航 wiki 索引和概念文章
3. 跟踪 wikilinks 发现相关内容
4. 综合带完整引用的答案

**输出**： comprehensive 答案，带来源追溯

### 3. 多格式输出

**何时**：你想分享知识

**触发**："create report"、"create slides"、"generate diagram"、"生成报告"、"生成幻灯片"

**格式**：
- **Markdown 报告** → `outputs/reports/`
- **Marp 幻灯片** → `outputs/slides/`
- **Mermaid 图表** → `outputs/charts/`
- **Canvas 文件** → `outputs/charts/`

## 查询示例

### 简单搜索

```
用户：search for transformer variants

助手：## 搜索结果："transformer variants"

找到 5 篇相关文章：

1. **[[wiki/concepts/vision-transformer]]** — ViT 将 transformers 应用于图像块
   > Vision Transformers (ViT) 将图像块视为 tokens，应用标准 transformer 架构...
   > 标签：#architecture/transformer #application/cv

2. **[[wiki/concepts/swin-transformer]]** — 使用移位窗口的分层 transformer
   > Swin Transformer 使用移位窗口实现高效的层次视觉...
   > 标签：#architecture/transformer #application/cv

_在 wiki 中搜索了 23 篇文章_
```

### 复杂问答

```
用户：注意力机制从原始 transformer 以来如何演变？

助手：## 答案：注意力机制的演变

注意力机制从原始"Attention Is All You Need"论文以来已经显著演变...

### 关键发现

1. **多头注意力（2017）** — 原始 transformer 引入了多头自注意力
   - 来源：[[wiki/summaries/attention-is-all-you-need]]

2. **稀疏注意力（2019-2020）** — 降低二次复杂度
   - 来源：[[wiki/concepts/sparse-transformer]], [[wiki/summaries/reformer]]

3. **线性注意力（2020+）** — O(n) 复杂度方法
   - 来源：[[wiki/concepts/linear-attention]], [[wiki/summaries/performer]]

### 分析

演变趋势显示清晰模式：在保持表达能力的同时降低计算复杂度...

### 参考

- [[wiki/concepts/attention-mechanism]] — 核心概念定义
- [[wiki/concepts/transformer-architecture]] — 原始架构
- [[wiki/summaries/attention-is-all-you-need]] — 原始论文
```

### 输出生成

```
用户：create slides about the history of transformer architectures

助手：正在生成幻灯片...

✓ 已创建 outputs/slides/2026-04-03-transformer-history.md

幻灯片包括：
- 15 张幻灯片覆盖 2017 到 2026 的演变
- 关键里程碑和创新
- Mermaid 时间线图
- 对 wiki 来源的完整引用

在 Obsidian 中用 Marp 插件打开查看。
```

## 输出格式

### Markdown 报告

**最适合**：综合研究、文献综述、详细分析

**结构**：
- 执行摘要
- 带 wikilinks 的详细章节
- 结论和含义
- 参考表格

**示例**：

```markdown
---
title: "Transformer 研究现状"
date: 2026-04-03
tags:
  - report
  - transformers
sources_consulted: 23
---

# Transformer 研究现状

## 执行摘要

本报告综合了 23 份资料的发现...

## Transformer 变体

[[wiki/concepts/vision-transformer]] 已被应用于...

## 参考

| 来源 | 类型 | 关键贡献 |
|--------|------|-----------------|
| [[attention-is-all-you-need]] | paper | 原始 transformer |
```

### Marp 幻灯片

**最适合**：演示、演讲、教学材料

**指南**：
- 综合主题 10-20 张幻灯片
- 每张幻灯片最多 5-7 个要点
- 包含演讲者备注
- 使用 `![bg right](image.png)` 添加图片

**查看幻灯片**：在 Obsidian 中用 Marp Slides 插件打开，或导出为 PDF/PPTX

### Mermaid 图表

**最适合**：可视化关系、时间线、概念地图

**类型**：
- **图图表**：概念关系
- **时间线**：年代演变
- **思维导图**：层次主题分解

**渲染**：图表在 Obsidian 中原生渲染

### Canvas 文件

**最适合**：交互式知识地图、可视化探索

**特性**：
- 拖拽节点
- 创建自定义分组
- 添加可视化注释
- 导出为图片

**查看**：在 Obsidian Canvas 中打开 `.canvas` 文件

## 归档洞察

在问答期间，LLM 可能发现**新连接**或**涌现概念**。它会提议归档：

```
助手：这个答案揭示了稀疏注意力和高效 transformer 之间的有趣连接。
你想将其归档为：
1. 新概念文章：[[wiki/concepts/efficient-attention]]
2. 研究报告：outputs/reports/2026-04-03-efficient-attention.md
3. 更新现有概念：将这些连接添加到 [[wiki/concepts/sparse-attention]]
```

**建议**：定期归档洞察 —— 这有机地增长 wiki。

## 最佳实践

### 1. 提出复杂问题

wiki 越大，答案越好。不要问"X 是什么？"—— 问"X 如何演变以及权衡是什么？"

### 2. 始终检查输出

生成的内容是起点，不是终点。检查：
- 准确性（检查来源链接）
- 完整性（是否遗漏了什么？）
- 清晰度（是否有意义？）

### 3. 广泛分享

输出设计用于分享：
- 报告 → 发送给团队、发布为博客
- 幻灯片 → 在会议、大会上演示
- 图表 → 包含在论文、文档中

### 4. 迭代

查询 → 生成 → 归档 → 重新编译 → 再次查询

每个周期都改进 wiki。

## 下一步

- [**健康检查**](/zh/workflow/health-checks) — 维护 wiki 质量
- [**kb-query 技能**](/zh/skills/kb-query) — 详细技能参考
- [**多格式输出**](/zh/skills/kb-query#能力-3多格式输出) — 输出格式指南
