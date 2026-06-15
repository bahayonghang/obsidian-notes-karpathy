# Implementation Plan

## Pre-Development

- [ ] Read backend specs required for CLI and payload work.
- [ ] Inspect current install tests in `tests/skill_bundle.rs`.
- [ ] Confirm whether `dirs::home_dir()` can be safely isolated in tests; if not, keep global path assertions at the support helper level.

## Implementation Steps

1. Update payload structs.
   - Add a `SkillInstallTargetResult` or equivalent with `key`, `scope`, `label`, `target_dir`, `installed`, and `skipped`.
   - Add `targets: Vec<...>` to `SkillInstallPayload`.
   - Preserve `claude` and `codex` optional fields.

2. Update CLI args.
   - Add target flags in `SkillInstallArgs`.
   - Keep existing flags and meanings.

3. Refactor install target resolution.
   - Introduce local/global target definitions in `src/support/skills.rs`.
   - Resolve selection from flags.
   - Keep default local selection as Claude + Codex/generic.
   - Require explicit `--global` for home-directory targets.

4. Keep installation behavior non-destructive.
   - Reuse existing skip behavior.
   - Ensure `--overwrite` replaces existing skill folders only for selected targets.

5. Add tests.
   - Extend `tests/skill_bundle.rs`.
   - Assert `targets` contents and target directories.
   - Assert legacy `claude` / `codex` fields for default local install.
   - Assert skip/overwrite behavior.

6. Update docs.
   - Update command examples and target matrix.
   - Mention that bootstrap/rules files are future work.

## Validation

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`
- `cargo run -- --json dev contract-validate`
- `git diff --check`

## Rollback

If the expanded target model causes compatibility issues, keep the payload struct changes but gate new targets behind explicit flags only. The original default behavior can remain unchanged by reverting target-selection expansion while retaining tests for legacy behavior.
