# Design

## Recommendation

Extend `onkb skill install` with a table-driven target model. Preserve the existing local default, add explicit local target flags, add explicit global target mode, and expose all install outcomes through a stable `targets` array.

## CLI Shape

Existing flags stay valid:

- `--claude`
- `--codex`
- `--dir <PATH>`
- `--overwrite`

New flags:

- `--cursor`
- `--windsurf`
- `--kiro`
- `--pi`
- `--agents`
- `--gemini`
- `--all-local`
- `--global`

Selection rules:

- No target flags and no `--global`: install the current local default targets, `claude` and `codex`.
- Local mode: resolve selected targets relative to `--dir` or current directory.
- `--all-local`: select all local targets.
- `--global`: resolve selected global targets under the current user's home directory.
- `--global` with no target flags: select all supported global targets.
- `--dir` is ignored in global mode or rejected if the implementation can produce a clearer error; rejecting is safer.

## Target Keys

Local targets:

| Key | Path |
| --- | --- |
| `claude` | `.claude/skills` |
| `codex` | `.agents/skills` |
| `cursor` | `.cursor/skills` |
| `windsurf` | `.windsurf/skills` |
| `kiro` | `.kiro/skills` |
| `pi` | `.pi/skills` |

Global targets:

| Key | Path |
| --- | --- |
| `claude` | `~/.claude/skills` |
| `codex` | `~/.codex/skills` |
| `gemini` | `~/.gemini/skills` |
| `agents` | `~/.agents/skills` |

## JSON Contract

Add:

```json
{
  "targets": [
    {
      "key": "claude",
      "scope": "local",
      "label": "Claude Code",
      "target_dir": ".../.claude/skills",
      "installed": ["..."],
      "skipped": ["..."]
    }
  ]
}
```

Keep legacy fields:

- `claude`
- `codex`

These remain aliases for local `claude` and local `codex` target results when selected. They are retained to avoid breaking downstream tests or consumers.

## Rust Boundaries

- `src/cli/args.rs` owns flags only.
- `src/cli/dispatch.rs` passes the parsed install args to support logic.
- `src/support/skills.rs` owns target resolution, embedded skill listing, and copy behavior.
- `src/payload/mod.rs` owns stable payload types.

## Safety

- Default behavior stays project-local and matches current behavior.
- Global writes require explicit `--global`.
- Existing target skill folders remain skipped unless `--overwrite` is set.
- No symlinks are introduced in this task.
- No bootstrap files are written in this task.

## Tests

Add focused tests under `tests/skill_bundle.rs`, because that file already covers embedded skill installation behavior.

Required coverage:

- default local install writes `.claude/skills` and `.agents/skills`.
- explicit local install writes requested target paths only.
- `--all-local` writes all local targets.
- global install resolves under a controlled temporary home if test infrastructure supports environment override; if not, unit-test target resolution directly.
- overwrite false skips existing skill directories.
- overwrite true replaces existing managed skill directories.

## Docs

Update:

- `README.md`
- `README_CN.md`
- `CLAUDE.md`
- `docs/guide/installation.md`
- `docs/zh/guide/installation.md`

Docs should state that bootstrap/rules file installation is not part of this command yet.
