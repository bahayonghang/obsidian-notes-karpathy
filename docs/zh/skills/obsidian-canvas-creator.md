# Canvas Creator

> 搭配技能说明：这是相邻生态技能页面，不属于本仓库当前发货的 6 个核心技能。

创建 Obsidian Canvas 文件用于可视化知识映射。

## 概览

Obsidian Canvas 是一个无限画布，你可以在其中排列笔记、创建连接和可视化关系。`obsidian-canvas-creator` 技能以编程方式生成 `.canvas` 文件。

## 什么是 Canvas？

Canvas 文件（`.canvas`）是 JSON 文件，定义：
- **节点**：笔记、文本块、图片、网页
- **边**：节点之间的连接，带有标签
- **布局**：每个元素的位置和大小

在 Obsidian 中渲染为交互式视觉地图。

## 在知识库中的使用

Karpathy 技能使用 Canvas 进行：

### 概念关系网络

可视化概念之间的连接：

```
Concept A ──related to──> Concept B
    │                        │
    │derived from            │extends
    ▼                        ▼
Concept C ──builds on──> Concept D
```

### 源到概念的映射

显示哪些源贡献了哪些概念：

```
Source 1 ──> Concept A
Source 2 ──> Concept A
Source 2 ──> Concept B
Source 3 ──> Concept C
```

### 主题簇

按类别对相关概念分组：

```
┌─ Category 1 ─────────────┐
│  Concept A  Concept B    │
│     Concept C            │
└──────────────────────────┘

┌─ Category 2 ─────────────┐
│  Concept D  Concept E    │
└──────────────────────────┘
```

## 输出位置

Canvas 文件保存到 `outputs/charts/{topic}.canvas`。

## 安装

从 [kepano/obsidian-skills](https://github.com/kepano/obsidian-skills) 安装：

```bash
cp -r obsidian-skills/obsidian-canvas-creator ~/.claude/skills/
```

## 触发短语

- "知识图谱" / "canvas" / "visual knowledge map"
- "show relationships" / "concept map"
- "create canvas" / "generate canvas"

## 参考

- [kepano/obsidian-skills](https://github.com/kepano/obsidian-skills)
- [Obsidian Canvas Documentation](https://help.obsidian.md/Plugins/Canvas)
