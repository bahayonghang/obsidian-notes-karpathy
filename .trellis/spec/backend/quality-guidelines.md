# Quality Guidelines

Quality in this repository means preserving deterministic CLI behavior, stable
skill contracts, and review-gated vault semantics. Small changes should stay
surgical; contract changes must update code, docs, references, fixtures, and
tests together.

## Required Gates

Run the narrowest useful check while iterating, then run the broader gate before
finishing meaningful work:

- `just lint`: `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`,
  and `cargo run -- --json dev contract-validate`
- `just test`: full Rust integration and snapshot coverage via `cargo test`
- `just docs-build`: VitePress docs build
- `just ci`: lint, test, and docs build

For documentation-only Trellis spec edits, at minimum run a text check for
placeholder content and `git diff --check`. Run `just ci` when the change touches
runtime code, public docs, skill contracts, fixtures, or generated payloads.

## Testing Requirements

- Add or update integration tests under `tests/*.rs` for routing, vault
  mechanics, persisted fields, and CLI payloads.
- Update fixture vaults under
  `evals/skills/obsidian-notes-karpathy/fixtures/` when a contract change
  depends on realistic vault state.
- Use `insta` JSON snapshots for stable payload compatibility. Redact unstable
  fields such as timestamps, local vault roots, and binary paths.
- Keep tests machine-readable by using `tests/common/mod.rs::run_json` for CLI
  commands that should return JSON.
- Run docs contract tests when `README.md`, `README_CN.md`, `CLAUDE.md`,
  `docs/`, or `skills/obsidian-notes-karpathy/scripts/skill-contract-registry.json`
  changes.

## Contract Synchronization

When changing core knowledge-base semantics, check all relevant surfaces:

- Rust implementation under `src/kb/**` or `src/dev/**`
- payload structs in `src/payload/mod.rs`
- shared references in `skills/obsidian-notes-karpathy/references/`
- core `SKILL.md` files under `skills/`
- public docs in `README.md`, `README_CN.md`, `CLAUDE.md`, and `docs/`
- fixture vaults and eval manifests under `evals/skills/obsidian-notes-karpathy/`
- JSON snapshots under `tests/snapshots/`

`tests/docs_contract.rs` and `src/dev/contract.rs` encode many of these
cross-surface invariants.

## Forbidden Patterns

- Do not treat `raw/`, `wiki/drafts/`, `MEMORY.md`, or `outputs/episodes/` as
  approved query truth.
- Do not bypass the review gate when promoting content into `wiki/live/`.
- Do not add Python runtime scripts for core behavior. The validator blocks
  legacy script markers.
- Do not emit noisy stdout from domain modules; it breaks JSON contract tests.
- Do not change path separators in payloads from forward slashes to backslashes.
- Do not update contract wording without matching tests or fixtures when the
  behavior is executable.

## Review Checklist

Before considering a change done:

- The changed lines map directly to the requested task.
- Public contract terms are exact and consistent.
- New persisted fields have tests, fixtures, and reference docs.
- CLI outputs remain valid JSON under `--json`.
- File writes preserve existing user content unless an explicit write/overwrite
  mode was requested.
- Paths in payloads are normalized and deterministic.
- No placeholder Trellis text remains in `.trellis/spec/backend/`.
