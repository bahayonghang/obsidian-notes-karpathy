# Installation

Install the bundle with `npx` or copy the skill directories manually.

## Install with npx

Global:

```bash
npx skills add bahayonghang/obsidian-notes-karpathy -g
```

Project-local:

```bash
cd /path/to/your/obsidian-vault
npx skills add bahayonghang/obsidian-notes-karpathy
```

## Manual installation

```bash
cp -r skills/obsidian-notes-karpathy/* ~/.claude/skills/
```

PowerShell:

```powershell
Copy-Item -Recurse skills/obsidian-notes-karpathy\* $env:USERPROFILE\.claude\skills\
```

## Verify

List your skills home and confirm these directories exist:

- `obsidian-notes-karpathy/`
- `kb-init/`
- `kb-compile/`
- `kb-query/`
- `kb-health/`

## Recommended companion skills

- `obsidian-markdown`
- `obsidian-cli`
- `obsidian-canvas-creator`
