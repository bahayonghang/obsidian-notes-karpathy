# Improve onkb skill install targets and install status

## Goal

Improve `onkb skill install` so the embedded runtime skills can be installed into more AI agent discovery paths with machine-readable install status, while preserving the current safe default and JSON compatibility expectations.

## Parent Context

Parent task: `.trellis/tasks/06-15-optimize-obsidian-wiki-lessons`.

This child implements Phase 1 of the recommended roadmap: CLI distribution setup and install visibility.

## Confirmed Facts

- Current install logic writes embedded runtime skills only to project-local `.claude/skills` and `.agents/skills`.
- Current CLI flags are `--claude`, `--codex`, `--dir`, and `--overwrite`.
- Current payload has fixed optional fields `claude` and `codex`, which does not scale to many targets.
- Public docs currently describe `onkb skill install --claude --codex`.
- The reference repo supports many agent paths, but it also writes broadly into user home directories; this repo should keep broad writes explicit.

## Requirements

- Add a scalable target model for skill installation.
- Keep the no-target default safe and backward compatible by installing only the existing project-local Claude and Codex/generic targets.
- Add explicit flags for additional local agent targets used by the reference repo where the path convention is stable enough for this repo:
  - Cursor: `.cursor/skills`
  - Windsurf: `.windsurf/skills`
  - Kiro: `.kiro/skills`
  - Pi: `.pi/skills`
- Add an explicit global installation mode for supported home-directory targets:
  - Claude: `~/.claude/skills`
  - Codex: `~/.codex/skills`
  - Gemini: `~/.gemini/skills`
  - generic AGENTS-aware tools: `~/.agents/skills`
- Return a machine-readable list of target results in addition to any legacy `claude` / `codex` fields needed for compatibility.
- Preserve non-destructive default behavior: existing target skill directories are skipped unless `--overwrite` is set.
- Keep all installed skill content sourced from the embedded runtime registry.

## Out of Scope

- Writing bootstrap files such as `AGENTS.md`, Cursor rules, or Kiro steering files.
- Adding symlink mode. This task keeps the existing copy-from-embedded behavior.
- Adding GitHub vault sync or cron setup.
- Changing vault lifecycle behavior or review-gated semantics.
- Adding cross-agent history ingestion skills.

## Acceptance Criteria

- [ ] `onkb --json skill install --dir <tmp>` still installs the existing `.claude/skills` and `.agents/skills` targets.
- [ ] `onkb --json skill install --dir <tmp> --cursor --windsurf --kiro --pi` installs those project-local target directories.
- [ ] `onkb --json skill install --dir <tmp> --all-local` installs all project-local targets.
- [ ] `onkb --json skill install --global --claude --codex --gemini --agents` resolves home-directory targets without using `--dir`.
- [ ] JSON output includes a stable `targets` array with each target key, label, target directory, installed skills, and skipped skills.
- [ ] Existing `claude` and `codex` payload fields remain present for the legacy local targets when selected.
- [ ] Existing target skill directories are skipped unless `--overwrite` is provided.
- [ ] Tests cover default, explicit local, all-local, global path resolution, and overwrite/skip behavior.
- [ ] Public installation docs are updated in English and Chinese.

## Notes

- Implementation starts only after this task is marked `in_progress`.
