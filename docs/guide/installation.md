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
cp -r skills/* ~/.claude/skills/
```

Codex:

```bash
cp -r skills/* ~/.codex/skills/
```

PowerShell:

```powershell
Copy-Item -Recurse skills\* $env:USERPROFILE\.claude\skills\
```

## Verify

List your skills home and confirm these directories exist:

- `obsidian-notes-karpathy/`
- `kb-init/`
- `kb-compile/`
- `kb-query/`
- `kb-health/`

Then confirm the bundled resources live inside the entry skill:

- `obsidian-notes-karpathy/references/`
- `obsidian-notes-karpathy/scripts/`
- `obsidian-notes-karpathy/evals/`

## Recommended companion skills

- `obsidian-markdown`
- `obsidian-cli`
- `obsidian-canvas-creator`

For paper/PDF ingestion under `raw/papers/`, also install:

- `alphaxiv-paper-lookup` as the required paper companion for `raw/papers/*.pdf`
- `pdf` for non-paper PDF handling outside the strict `raw/papers` compile path

Install those companion skills into the same skill home your runtime is actually using. If `raw/papers` PDFs are still skipped, verify that `alphaxiv-paper-lookup` is available in the active home before changing the vault.
