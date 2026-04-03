# 安装指南

多种安装 Obsidian Notes Karpathy 技能的方式。

## 方式一：使用 npx 安装（推荐）

最简单的安装方式是使用 `npx skills add` 命令。

### 全局安装

全局安装，可在所有项目中使用：

```bash
npx skills add bahayonghang/obsidian-notes-karpathy -g
```

这会将技能安装到全局 agents 目录，使其在任何地方都可用。

### 项目特定安装

安装到你的 Obsidian vault 或项目目录中，仅供该项目使用：

```bash
# 导航到你的 Obsidian vault 根目录
cd /path/to/your/obsidian-vault

# 本地安装技能
npx skills add bahayonghang/obsidian-notes-karpathy
```

这会在你的项目根目录创建一个 `skills/` 目录，包含所有必要的技能。

## 方式二：手动安装

如果你偏好手动安装或想要自定义技能：

```bash
# 复制技能到你的 Claude Code 技能文件夹
cp -r skills/obsidian-notes-karpathy/* ~/.claude/skills/
```

在 Windows 上（PowerShell）：

```powershell
Copy-Item -Recurse skills/obsidian-notes-karpathy\* $env:USERPROFILE\.claude\skills\
```

## 方式三：克隆并自定义

适用于想要修改技能的高级用户：

1. **克隆仓库**
   ```bash
   git clone https://github.com/bahayonghang/obsidian-notes-karpathy.git
   cd obsidian-notes-karpathy
   ```

2. **修改** SKILL.md 文件以适应你的工作流

3. **安装** 使用 npx 或复制到你的技能文件夹

## 验证安装

通过列出已安装的技能来检查安装是否成功：

```bash
ls ~/.claude/skills/obsidian-notes-karpathy/
```

你应该看到：

```
kb-init/
  └── SKILL.md
kb-compile/
  └── SKILL.md
kb-query/
  └── SKILL.md
```

## 必需的依赖

这些技能基于 [kepano/obsidian-skills](https://github.com/kepano/obsidian-skills) 构建。确保你还安装了以下技能：

- **`obsidian-markdown`** — Obsidian Flavored Markdown 语法（wikilinks、callouts、frontmatter）
- **`obsidian-cli`** — 通过命令行与 Vault 交互
- **`obsidian-canvas-creator`** — 生成 Canvas 可视化

从 [kepano/obsidian-skills](https://github.com/kepano/obsidian-skills) 仓库安装这些技能：

```bash
# 克隆 obsidian-skills 仓库
git clone https://github.com/kepano/obsidian-skills.git

# 复制所需的技能
cp -r obsidian-skills/obsidian-markdown ~/.claude/skills/
cp -r obsidian-skills/obsidian-cli ~/.claude/skills/
cp -r obsidian-skills/obsidian-canvas-creator ~/.claude/skills/
```

## 可选：Web Clipper

为了获得最佳体验，请安装 **Obsidian Web Clipper** 浏览器扩展：

- [Obsidian Web Clipper](https://obsidian.md/clipper) — 支持 Chrome、Firefox、Edge

这可以让你直接将文章剪辑到你的 `raw/` 目录。

## 接下来

- [**快速开始**](/guide/quick-start) — 快速上手运行
- [**Karpathy 工作流**](/guide/karpathy-workflow) — 理解背后的理念
- [**技能概览**](/skills/overview) — 了解每个技能
