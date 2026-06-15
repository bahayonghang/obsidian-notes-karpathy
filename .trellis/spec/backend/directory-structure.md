# Directory Structure

This project is a Rust CLI plus a runtime skill bundle for review-gated
Obsidian knowledge bases. Treat "backend" work as deterministic CLI,
repository-maintenance, contract-validation, and vault-file workflow logic.

## Repository Layout

```text
src/
  cli/                 # clap argument definitions, command dispatch, output rendering
  dev/                 # bundle validation, skill audit, trigger/runtime eval helpers
  kb/                  # knowledge-base lifecycle implementation
    compile/           # raw -> draft package scanning/building
    health/            # review-gate and maintenance audit rules
    ingest/            # raw source manifest scanning/writing
    init/              # vault scaffold, status, and legacy-layout migration
    render/            # deterministic artifact rendering
  payload/             # serde payload structs shared by CLI/tests
  support/             # path, JSON, config, time, skill-install, audit-log helpers
tests/                 # Rust integration tests and JSON snapshots
skills/
  obsidian-notes-karpathy/
    SKILL.md
    references/        # shared runtime contract docs and templates
    scripts/           # registry/compatibility metadata, not the primary runtime baseline
  kb-init|kb-ingest|kb-compile|kb-review|kb-query|kb-render/
docs/                  # VitePress public docs
evals/skills/...       # fixture vaults and eval manifests used by tests
```

Examples in the current tree:

- `src/cli/args.rs` owns clap command shapes; `src/cli/dispatch.rs` maps those
  commands to implementation functions.
- `src/kb/init/legacy_impl.rs`, `src/kb/ingest/legacy_impl.rs`, and
  `src/kb/compile/legacy_impl.rs` still contain large legacy implementation
  bodies behind smaller module facades. Do not widen this pattern for new code.
- `src/kb/health/engine.rs` coordinates health rules from
  `src/kb/health/rules/`.
- `src/support/audit_log.rs` is the only append channel for
  `outputs/audit/operations.jsonl`.

## Module Organization

Keep command parsing, orchestration, and domain logic separate:

- Add CLI flags or subcommands in `src/cli/args.rs`.
- Dispatch a command in `src/cli/dispatch.rs`, returning a `serde_json::Value`
  through `CommandOutcome`.
- Put lifecycle logic under the matching `src/kb/<stage>/` module.
- Put shared parsing, path normalization, timestamp, audit, and skill-install
  helpers under `src/support/`.
- Put machine-readable payload structs under `src/payload/` when tests or
  downstream tools should assert a stable JSON contract.
- Put repo-development checks and eval harness code under `src/dev/`.

Do not place real implementation in `main.rs`; it should remain a tiny
`onkb::cli::run()` entry point.

## Naming Conventions

- Rust files and modules use `snake_case`.
- Test functions use `snake_case`.
- Fixture note files use date-prefixed slugs such as
  `2026-04-05-approved-summary.md`.
- Public knowledge-base terms must stay exact in docs and specs:
  `raw/`, `raw/_manifest.yaml`, `wiki/drafts/`, `wiki/live/`,
  `wiki/briefings/`, `outputs/reviews/`, and `outputs/audit/operations.jsonl`.
- Vault paths in JSON payloads should be normalized to forward slashes via
  helpers such as `relative_posix` and `normalize_path_string`.

## Where To Add New Work

- New vault lifecycle behavior: extend the relevant `src/kb/<stage>/` module
  and add integration coverage in `tests/*.rs`.
- New health rule: prefer a focused helper in `src/kb/health/rules/` if the
  rule is standalone; wire it through `src/kb/health/engine.rs`.
- New generated or persisted file shape: update the relevant
  `skills/obsidian-notes-karpathy/references/*.md`, fixtures, and JSON
  snapshot tests together.
- New docs-visible contract term: keep `README.md`, `README_CN.md`,
  `CLAUDE.md`, and `docs/` aligned when the public contract changes.

## Avoid

- Do not add ad-hoc Python scripts for core runtime behavior. The contract
  validator in `src/dev/contract.rs` actively blocks legacy Python script
  references.
- Do not hand-edit generated docs or eval outputs without checking the owning
  source and tests.
- Do not broaden `legacy_impl.rs` files when a small focused Rust module can
  own the new behavior.
- Do not add abstractions for a single command path; keep helpers small and
  justified by reuse.
