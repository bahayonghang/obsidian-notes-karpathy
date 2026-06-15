# 安装

先装 Rust CLI，再用 `onkb` 下发内嵌 skill bundle。

## 安装 CLI

正常使用时，直接从 GitHub 安装：

```bash
cargo install --locked --git https://github.com/bahayonghang/obsidian-notes-karpathy.git onkb
```

如果你是在仓库根目录做本地开发，再执行：

```bash
cargo install --path . --locked --force
```

确认二进制可用：

```bash
onkb version
onkb doctor
```

如果脚本需要机器可读输出，再用 `onkb --json doctor`。

如果 skill 或 shell 提示 `onkb` 没有安装，默认就用上面的 GitHub 安装命令补装，然后重试同一个 `onkb ...` 命令。

## 安装内嵌 skills

在目标 vault 或工作目录中执行：

```bash
onkb skill install
```

PowerShell：

```powershell
onkb skill install --dir D:\path\to\your\obsidian-vault
```

默认行为是安全且向后兼容的，只会安装到当前工作目录下的本地目标：

| 目标 | 目录 |
| --- | --- |
| Claude Code | `.claude/skills` |
| Codex / 通用 AGENTS 工具 | `.agents/skills` |

其他本地目标需要显式开启：

```bash
onkb skill install --cursor --windsurf --kiro --pi
onkb skill install --all-local
```

| Flag | 目录 |
| --- | --- |
| `--cursor` | `.cursor/skills` |
| `--windsurf` | `.windsurf/skills` |
| `--kiro` | `.kiro/skills` |
| `--pi` | `.pi/skills` |

写入用户 home 下的全局目标必须显式加 `--global`：

```bash
onkb skill install --global --claude --codex --gemini --agents
```

| Flag | 目录 |
| --- | --- |
| `--claude` | `~/.claude/skills` |
| `--codex` | `~/.codex/skills` |
| `--gemini` | `~/.gemini/skills` |
| `--agents` | `~/.agents/skills` |

`--overwrite` 会替换已有的 skill 目录；不传时默认跳过已有目录。`--json` 输出会带
`targets[]` 数组，里面包含每个目标的 key、label、target_dir、installed 和
skipped。兼容用的旧字段 `claude` 和 `codex` 仍然保留。

`onkb skill install` 只复制内嵌 skill bundle，不会写 bootstrap 文件、Cursor
rules 或 Kiro steering files。

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

运行时安装不会把 repo 内的 dev / eval 资源一起带进去。
这些资源仍然留在仓库树里：

- `evals/skills/obsidian-notes-karpathy/`

这个 bundle 直接内嵌在 CLI 二进制里。`onkb skill install` 安装时不依赖仓库源码仍然留在本地。

## 推荐搭配技能

- `obsidian-markdown`
- `obsidian-cli`
- `obsidian-canvas-creator`

如果你准备把 PDF 论文直接放进 `raw/**/papers/`，还建议安装：

- `paper-workbench`：`raw/**/papers/*.pdf` 必需的论文 companion skill；`json` 用于论文标准化，`interpret` 用于直接讲解论文，`xray` 用于深拆论文
- `pdf`：严格 `raw/**/papers` 编译链之外的通用 PDF 处理 companion skill

这两个 companion skill 要装到当前运行时真正使用的 skill home 里；如果论文 PDF 仍然被当成待处理项跳过，先确认 `paper-workbench` 是否真的在 active skill home 里。
