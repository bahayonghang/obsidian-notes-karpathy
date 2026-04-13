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
cp -r skills/* ~/.claude/skills/
```

PowerShell：

```powershell
Copy-Item -Recurse skills\* $env:USERPROFILE\.claude\skills\
```

## 验证

检查你的 skills 目录下是否有这些目录：

- `obsidian-notes-karpathy/`
- `kb-init/`
- `kb-ingest/`
- `kb-compile/`
- `kb-review/`
- `kb-query/`
- `kb-render/`

然后确认内置资源位于 shared package home 内部：

- `obsidian-notes-karpathy/references/`
- `obsidian-notes-karpathy/scripts/`
- `obsidian-notes-karpathy/evals/`

## 推荐搭配技能

- `obsidian-markdown`
- `obsidian-cli`
- `obsidian-canvas-creator`

如果你准备把 PDF 论文直接放进 `raw/**/papers/`，还建议安装：

- `paper-workbench`：`raw/**/papers/*.pdf` 必需的论文 companion skill；`json` 用于论文标准化，`interpret` 用于直接讲解论文，`xray` 用于深拆论文
- `pdf`：严格 `raw/**/papers` 编译链之外的通用 PDF 处理 companion skill

这两个 companion skill 要装到当前运行时真正使用的 skill home 里；如果论文 PDF 仍然被当成待处理项跳过，先确认 `paper-workbench` 是否真的在 active skill home 里。
