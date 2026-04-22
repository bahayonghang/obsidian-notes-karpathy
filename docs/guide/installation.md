# Installation

Install the Rust CLI first, then let `onkb` install the embedded skill bundle.

## Install the CLI

For normal use, install directly from GitHub:

```bash
cargo install --locked --git https://github.com/bahayonghang/obsidian-notes-karpathy.git onkb
```

For local development from the repo root:

```bash
cargo install --path . --locked --force
```

Verify the binary:

```bash
onkb version
onkb doctor
```

Use `onkb --json doctor` when a script needs machine-readable output.

If a skill or shell step reports that `onkb` is not installed, use the GitHub install command above as the default fallback, then rerun the same `onkb ...` command.

## Install the embedded skills

Inside the target vault or workspace:

```bash
onkb skill install --claude --codex
```

Or point at another directory explicitly:

```bash
onkb skill install --dir /path/to/your/obsidian-vault --claude --codex
```

PowerShell:

```powershell
onkb skill install --dir D:\path\to\your\obsidian-vault --claude --codex
```

## Verify

List your skills home and confirm these directories exist:

- `obsidian-notes-karpathy/`
- `kb-init/`
- `kb-ingest/`
- `kb-compile/`
- `kb-review/`
- `kb-query/`
- `kb-render/`

Then confirm the bundled resources live inside the shared package home:

- `obsidian-notes-karpathy/references/`
- `obsidian-notes-karpathy/scripts/`

The runtime install intentionally excludes repo-only dev assets such as eval fixtures and benchmark manifests.
Those stay in the repository tree under:

- `evals/skills/obsidian-notes-karpathy/`

The bundle is embedded in the CLI binary. `onkb skill install` does not depend on the repo source tree being present at install time.

## Recommended companion skills

- `obsidian-markdown`
- `obsidian-cli`
- `obsidian-canvas-creator`

For paper/PDF ingestion under `raw/**/papers/`, also install:

- `paper-workbench` as the required paper companion for `raw/**/papers/*.pdf`; use `json` for paper normalization, `interpret` for direct paper explanation, and `xray` for deeper critique
- `pdf` for non-paper PDF handling outside the strict `raw/**/papers` compile path

Install those companion skills into the same skill home your runtime is actually using. If paper PDFs are still being surfaced as skipped work, verify that `paper-workbench` is available in the active home before changing the vault.
