# 快速开始

在几分钟内搭建你的 Karpathy 风格知识库。

## 前置条件

- 一个 Obsidian Vault（或其中的某个子目录）
- 能够访问本项目 skills 的 Claude Code

## 步骤 1：初始化知识库

使用一个主题名称运行 `kb-init` 技能：

```
Initialize knowledge base for "deep learning research"
```

这将创建标准目录结构：

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
└── AGENTS.md             # Schema 定义文件
```

## 步骤 2：添加第一个来源

向 `raw/` 添加原始资料。你可以：

- 使用 **Obsidian Web Clipper** 浏览器扩展
- 手动创建带有 frontmatter 的 `.md` 文件：

```yaml
---
title: "Attention Is All You Need"
source: "https://arxiv.org/abs/1706.03762"
author: "Vaswani et al."
date: 2017-06-12
type: paper
tags:
  - transformers
  - attention
clipped_at: 2026-04-03T10:00:00
compiled_at: null
---

# 你在此处粘贴或记录内容
```

## 步骤 3：编译知识库

运行编译：

```
compile wiki
```

LLM 将执行：
1. 扫描 `raw/` 中的新增或更新来源
2. 在 `wiki/summaries/` 中生成摘要
3. 在 `wiki/concepts/` 中提取并创建概念文章
4. 在相关内容之间建立 wikilink
5. 更新索引文件
6. 运行健康检查

## 步骤 4：查询你的知识库

提出问题：

```
What are the key concepts in transformer architecture?
```

LLM 将：
1. 导航知识库索引
2. 沿着 wikilink 找到相关概念文章
3. 综合答案并附上完整的来源引用
4. 可选地将 Q&A 中发现的新见解归档到知识库

## 步骤 5：生成输出

创建报告或幻灯片：

```
生成报告 on transformer attention mechanisms
```

```
create slides about the evolution of attention mechanisms
```

输出保存到 `outputs/` 中的各种格式：
- **Markdown 报告**，附带完整引用
- **Marp 幻灯片**，可直接用于演示
- **Mermaid 图表**，在 Obsidian 中渲染
- **Canvas 文件**，用于可视化知识地图

## 接下来

- [**安装指南**](/guide/installation) — 详细设置说明
- [**技能概览**](/skills/overview) — 深入了解每个技能
- [**工作流指南**](/workflow/overview) — 掌握完整流水线
- [**健康检查**](/workflow/health-checks) — 维护知识库质量

## 成功建议

1. **从小开始**：先添加 2-3 个来源，编译，看看效果
2. **检查输出**：阅读生成的摘要和概念
3. **持续迭代**：随时间添加更多来源，重新编译以扩展知识库
4. **提出复杂问题**：知识库越大，回答质量越高
5. **归档新见解**：让 LLM 在 Q&A 中发现新概念并添加到知识库
