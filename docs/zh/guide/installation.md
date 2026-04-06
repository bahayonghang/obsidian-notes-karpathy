# 安装

可以通过 `npx` 安装，也可以手动复制技能目录。

## 使用 npx 安装

全局安装：

```bash
npx skills add bahayonghang/obsidian-notes-karpathy -g
```

项目内安装：

```bash
cd /path/to/your/obsidian-vault
npx skills add bahayonghang/obsidian-notes-karpathy
```

## 手动安装

```bash
cp -r skills/obsidian-notes-karpathy/* ~/.claude/skills/
```

PowerShell：

```powershell
Copy-Item -Recurse skills/obsidian-notes-karpathy\* $env:USERPROFILE\.claude\skills\
```

## 验证

检查你的 skills 目录下是否有这些目录：

- `obsidian-notes-karpathy/`
- `kb-init/`
- `kb-compile/`
- `kb-query/`
- `kb-health/`

## 推荐搭配技能

- `obsidian-markdown`
- `obsidian-cli`
- `obsidian-canvas-creator`
