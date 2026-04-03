# kb-init — 知识库初始化

一次性设置技能，创建标准目录结构和 AGENTS.md schema。

## 何时使用

- 用户说"初始化知识库"、"kb init"、"create knowledge base"、"karpathy setup"
- 启动新的 LLM 驱动的知识库项目
- 为结构化知识管理设置 Obsidian（使用 LLM 编译）

**重要提示**：这是一个**一次性设置**技能——每个 vault/项目运行一次。

## 功能

### 步骤 1：确定 Vault 根目录

询问用户要初始化哪个目录。这应该是：
- Obsidian vault 根目录，或
- vault 中专用于此知识库主题的子目录

如果用户提供了主题名称（例如"深度学习研究"），它会自定义 AGENTS.md 和索引文件。

### 步骤 2：创建目录结构

创建标准的 Karpathy 目录树：

```
{root}/
├── raw/                     # 原始资料
│   └── assets/              # 原始来源中的图片和附件
├── wiki/                    # LLM 编译的 wiki（不要手动编辑）
│   ├── concepts/            # 概念文章（每个关键概念各一篇）
│   ├── summaries/           # 每个原始来源的摘要
│   └── indices/             # 索引和导航文件
├── outputs/                 # 查询结果和生成的内容
│   ├── reports/             # 研究报告
│   ├── slides/              # Marp 幻灯片
│   └── charts/              # Mermaid 图表和可视化
└── AGENTS.md                # LLM 代理的 Schema 定义
```

### 步骤 3：生成 AGENTS.md

在根目录创建 `AGENTS.md`，包含：

- **概览**：知识库的主题和用途
- **目录结构表**：用途和归属（谁写入）
- **Frontmatter schema**：用于原始源、概念文章和摘要
- **编译规则**：七条核心编译规则
- **文件命名约定**：一致的命名模式

主题字段根据用户输入进行自定义。

### 步骤 4：创建初始索引文件

创建四个初始索引文件：

#### `wiki/indices/INDEX.md`

主索引，包含统计信息和文章列表：

```markdown
---
title: Knowledge Base Index
updated_at: {current datetime}
---

# Knowledge Base Index

> [!info] Auto-maintained
> This index is automatically maintained by `kb-compile`. Do not edit manually.

## Statistics

- Total sources: 0
- Total concepts: 0
- Total summaries: 0
- Last compiled: Never

## All Articles

_No articles yet. Run `kb-compile` after adding sources to `raw/`._
```

#### `wiki/indices/CONCEPTS.md`

概念地图，按类别分组：

```markdown
---
title: Concept Map
updated_at: {current datetime}
---

# Concept Map

> [!info] Auto-maintained
> This concept map is automatically maintained by `kb-compile`.

_No concepts yet. Run `kb-compile` after adding sources to `raw/`._
```

#### `wiki/indices/SOURCES.md`

所有原始来源的注册表：

```markdown
---
title: Source Registry
updated_at: {current datetime}
---

# Source Registry

> [!info] Auto-maintained
> This registry tracks all raw sources and their compilation status.

| Source | Type | Date | Compiled |
|--------|------|------|----------|
| _No sources yet_ | | | |
```

#### `wiki/indices/RECENT.md`

变更日志：

```markdown
---
title: Recent Updates
updated_at: {current datetime}
---

# Recent Updates

> [!info] Auto-maintained
> Shows the most recent changes to the knowledge base.

_No updates yet._
```

### 步骤 5：确认初始化完成

向用户报告：
- 创建了什么（目录树）
- 如何添加原始源（使用 Web Clipper 或手动）
- 下一步：向 `raw/` 添加源，然后运行 `kb-compile`

## 实现细节

### 使用的工具

- **`obsidian-markdown` 技能**：用于正确的 Obsidian Flavored Markdown 语法（wikilinks、callouts、properties）
- **`obsidian-cli` 技能**：如果可用，用于在 vault 中创建文件
- **Write 工具**：当 CLI 不可用时直接创建文件

### 安全检查

- 如果目录已有内容，在覆盖前**警告用户**
- 始终使用 UTF-8 编码
- 验证目标目录存在且可访问

## 初始化后

运行 `kb-init` 后，用户应该：

1. **向 `raw/` 添加源**，使用 Web Clipper 或手动
2. **运行 `kb-compile`** 将源处理为 wiki
3. **使用 `kb-query` 查询 wiki**

## 使用示例

```
User: I want to set up a knowledge base for transformer research
Assistant: I'll initialize a Karpathy-style knowledge base for "transformer research"...

✓ Created directory structure:
  raw/
  raw/assets/
  wiki/concepts/
  wiki/summaries/
  wiki/indices/
  outputs/reports/
  outputs/slides/
  outputs/charts/

✓ Generated AGENTS.md with transformer-specific schema
✓ Created initial index files

Next steps:
1. Add sources to raw/ (use Web Clipper or create .md files)
2. Run "compile wiki" to build the wiki
3. Use "query kb" to ask questions
```

## 下一步

- [**kb-compile**](/skills/kb-compile) — 核心编译引擎
- [**目录结构**](/guide/directory-structure) — 了解布局
- [**AGENTS.md Schema**](/guide/agents-schema) — 规则手册