# Obsidian CLI

> 搭配技能说明：这是相邻生态技能页面，不属于本仓库当前发货的 6 个核心技能。

用于与 Obsidian vault 交互的命令行界面。

## 概览

`obsidian-cli` 技能提供对 Obsidian vault 操作的命令行访问，支持与知识库的编程交互。

## 能力

### 文件操作

- 在 vault 中创建、读取、更新和删除笔记
- 按查询文本搜索 vault 内容
- 列出目录内容
- 移动和重命名文件

### 搜索

```bash
obsidian search query="transformer architecture"
obsidian search tag="concept"
obsidian search path="wiki/live/concepts/"
```

### Vault 管理

- 在 Obsidian 应用中打开 vault
- 列出可用的 vault
- 获取 vault 配置

## 在知识库中的使用

Karpathy 技能使用 `obsidian-cli` 进行：

1. **kb-init**：创建初始目录结构和文件
2. **kb-compile**：在编译期间搜索和阅读 vault 内容
3. **kb-query**：跨 wiki 的全文搜索

## 当 CLI 不可用时

如果 `obsidian-cli` 未安装，技能回退到：
- **Write 工具**：直接创建/编辑文件
- **Grep 工具**：搜索文件内容
- **Read 工具**：读取文件内容

## 安装

从 [kepano/obsidian-skills](https://github.com/kepano/obsidian-skills) 安装：

```bash
cp -r obsidian-skills/obsidian-cli ~/.claude/skills/
```

## 参考

- [kepano/obsidian-skills](https://github.com/kepano/obsidian-skills)
- [Obsidian CLI Documentation](https://help.obsidian.md/Advanced+topics/Using+Obsidian+CLI)
