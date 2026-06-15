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
onkb skill install
```

Or point at another directory explicitly:

```bash
onkb skill install --dir /path/to/your/obsidian-vault
```

PowerShell:

```powershell
onkb skill install --dir D:\path\to\your\obsidian-vault
```

The default is intentionally local and backward compatible. It installs into:

| Target | Directory |
| --- | --- |
| Claude Code | `.claude/skills` |
| Codex / generic AGENTS tools | `.agents/skills` |

Additional local targets are opt-in:

```bash
onkb skill install --cursor --windsurf --kiro --pi
onkb skill install --all-local
```

| Flag | Directory |
| --- | --- |
| `--cursor` | `.cursor/skills` |
| `--windsurf` | `.windsurf/skills` |
| `--kiro` | `.kiro/skills` |
| `--pi` | `.pi/skills` |

Global user-home targets require `--global`:

```bash
onkb skill install --global --claude --codex --gemini --agents
```

| Flag | Directory |
| --- | --- |
| `--claude` | `~/.claude/skills` |
| `--codex` | `~/.codex/skills` |
| `--gemini` | `~/.gemini/skills` |
| `--agents` | `~/.agents/skills` |

Use `--overwrite` to replace existing installed skill directories. Without it,
existing directories are skipped. `--json` output includes a `targets[]` array
with each selected target key, label, target directory, installed skills, and
skipped skills. Legacy local `claude` and `codex` fields remain available for
compatibility.

`onkb skill install` only copies the embedded skill bundle. It does not write
bootstrap files such as `AGENTS.md`, Cursor rules, or Kiro steering files.

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
