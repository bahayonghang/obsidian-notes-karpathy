# Implementation Plan

## Execution Strategy

Treat this parent task as the optimization roadmap. Before implementation, split the work into independently verifiable child tasks. Start with Phase 1 because it has the clearest blast radius and provides immediate user value without changing knowledge semantics.

## Child Task Candidates

1. CLI setup and install visibility
   - Goal: improve `onkb skill install` or add a setup/info lane for multi-agent skill distribution.
   - Verification: CLI payload tests, install-target fixture tests, `cargo test`, `cargo run -- --json dev contract-validate`.

2. Cross-agent memory bridge planning and first skill
   - Goal: add a read/report or staged-ingest workflow for agent-history provenance without bypassing review.
   - Verification: skill audit, docs contract, fixture manifest behavior, routing tests.

3. Approved-layer query ergonomics
   - Goal: upgrade `kb-query` contract and deterministic ranking support for tiered retrieval and relationship traversal.
   - Verification: query fixture tests, contract validation, docs alignment.

4. Obsidian graph/export helpers
   - Goal: add derived graph/export helpers with no truth-boundary changes.
   - Verification: deterministic output tests and docs contract.

## Phase 1 Checklist

- [ ] Read backend spec files before editing: directory structure, persistence, error handling, logging, quality.
- [ ] Inventory existing install behavior in `src/support/skills.rs` and tests.
- [ ] Decide exact install targets for the first release.
- [ ] Add CLI arguments or subcommands in `src/cli/args.rs`.
- [ ] Wire dispatch in `src/cli/dispatch.rs`.
- [ ] Add or update payload structs if JSON output changes.
- [ ] Extend skill-install logic with explicit target definitions and non-destructive overwrite behavior.
- [ ] Add tests for default behavior, explicit target behavior, skip/overwrite behavior, and path normalization.
- [ ] Update README, README_CN, CLAUDE, and docs pages.
- [ ] Run `cargo fmt --check`.
- [ ] Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [ ] Run `cargo test`.
- [ ] Run `cargo run -- --json dev contract-validate`.

## Phase 2 Checklist

- [ ] Define whether new history skills are runtime core skills or companion skills.
- [ ] Map each agent-history source into a safe staging surface.
- [ ] Update `memory-lifecycle.md` and provenance references before adding public skill wording.
- [ ] Add routing in the package skill and registry only after the contract is clear.
- [ ] Add fixtures showing history-derived content does not become `wiki/live` truth directly.
- [ ] Validate with `cargo run -- --json dev audit-skills` and docs contract tests.

## Phase 3 Checklist

- [ ] Update `search-upgrades.md` with tiered retrieval and typed-edge rules.
- [ ] Update `kb-query/SKILL.md` to make approved-layer-only retrieval explicit in every new mode.
- [ ] Extend Rust query helpers only where deterministic ranking or scope payloads need stable fields.
- [ ] Add query fixture coverage for summaries, stale pages, relationships, and missing semantic-search fallback.
- [ ] Run `cargo test` and docs contract tests.

## Phase 4 Checklist

- [ ] Decide whether export helpers belong in `kb-render` or separate companion skills.
- [ ] Preserve review state and provenance in every export format.
- [ ] Ensure generated files stay under downstream output surfaces.
- [ ] Add deterministic output tests for generated graph/export artifacts.

## Risky Files

- `src/support/skills.rs`: can overwrite user-managed skill folders if target handling is wrong.
- `src/cli/args.rs` and `src/cli/dispatch.rs`: JSON command contracts are tested and should remain stable.
- `skills/obsidian-notes-karpathy/scripts/skill-contract-registry.json`: changes affect install scope, audits, docs checks, and runtime skill discovery.
- `skills/kb-query/SKILL.md`: wording can accidentally widen truth boundaries.
- `README.md`, `README_CN.md`, `CLAUDE.md`, and `docs/`: public contract surfaces must stay aligned.

## Validation Gates

For planning-only changes:

- `$pattern = 'TB' + 'D|TO' + 'DO'; Select-String -Path .trellis/tasks/06-15-optimize-obsidian-wiki-lessons/*.md -Pattern $pattern`
- `git diff --check`

For implementation changes:

- `just lint`
- `just test`
- `just docs-build` when docs or public contract wording changes
- `just ci` before finishing a multi-surface implementation

## Handoff Criteria

Planning is ready when the user confirms the phase order or asks to implement the recommended first child task. Implementation should not start from this parent task unless the user explicitly wants the parent to own direct edits.
