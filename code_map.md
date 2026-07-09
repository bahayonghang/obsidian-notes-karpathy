# code_map.md

Repo navigation and search anchors for the `onkb` CLI + review-gated KB skill bundle.
Read this before broad grep. For the KB contract and behavioral rules, see `./CLAUDE.md`.

## Top-Level Routing

| Path                                             | What lives here                                        | Start here when                       |
| ------------------------------------------------ | ------------------------------------------------------ | ------------------------------------- |
| `src/`                                           | Rust CLI (`onkb`) implementation                       | changing CLI behavior or KB logic     |
| `skills/`                                        | `SKILL.md` contracts (7 skill homes)                   | changing lifecycle wording or routing |
| `tests/`                                         | Rust integration tests + snapshots                     | verifying behavior, adding fixtures   |
| `evals/skills/obsidian-notes-karpathy/fixtures/` | fixture vaults for contract tests                      | reproducing a vault state             |
| `docs/`                                          | VitePress site (`README.md`, `guide/`, `zh/`)          | editing public docs                   |
| `ref/repo/`                                      | **vendored reference repos — not this project's code** | never edit; read-only examples        |
| `target/`                                        | Cargo build output                                     | never edit; generated                 |

## `src/` — the `onkb` crate

Entry: `src/main.rs` → `src/lib.rs`. Commands are parsed in `src/cli/` and dispatched into `src/kb/` and `src/dev/`.

| Module         | Files (largest first)                                                                                                                                         | Responsibility                                                                       | Anchors                                           |
| -------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------- |
| `src/cli/`     | `dispatch.rs`, `args.rs`, `doctor.rs`, `output.rs`, `version.rs`                                                                                              | arg parsing, command dispatch, `--json` output, `doctor`                             | `fn dispatch`, subcommand names                   |
| `src/kb/`      | `governance.rs` (24K), `layout.rs` (16K), `markdown.rs` (16K), `query.rs`, `review.rs`, `episodes.rs`, `graph.rs`, `index.rs`, `automation.rs`, `guidance.rs` | core KB logic; lifecycle stages in `compile/ health/ ingest/ init/ render/`          | vault layout, review gate, obsidian-safe markdown |
| `src/dev/`     | `skill_audit.rs` (14K), `trigger_eval.rs` (11K), `contract.rs` (11K), `registry.rs`, `repo.rs`, `runtime_eval/`                                               | maintainer commands: `dev contract-validate`, `dev audit-skills`, `dev eval-runtime` | `just lint`/`just skill-audit` targets            |
| `src/payload/` | `mod.rs` (10K)                                                                                                                                                | embedded skill payload → `onkb skill install`                                        | `skill install`, target selection                 |
| `src/support/` | `skills.rs` (14K), `audit_log.rs`, `config.rs`, `json.rs`, `pathing.rs`, `time.rs`                                                                            | shared helpers, audit log, path/time utils                                           | `outputs/audit/operations.jsonl` writer           |

## `skills/` — SKILL.md contracts

`obsidian-notes-karpathy/` is the package router; `kb-init`, `kb-ingest`, `kb-compile`, `kb-review`, `kb-query`, `kb-render` are the lifecycle stages (roles enumerated in `CLAUDE.md`).

- `obsidian-notes-karpathy/references/` — shared file model and templates (`file-model.md` 16K, `lifecycle-matrix.md`, `draft-schema.md`, `health-rubric.md`, `*-template.md`, governance policies). Grep here for contract wording.
- `obsidian-notes-karpathy/scripts/skill-contract-registry.json` — compatibility/registry source used by `dev` commands.

## `tests/`

`cli_smoke.rs`, `compile_review.rs`, `contract_routing.rs`, `dev_tools.rs`, `doc_fragments.rs`, `docs_contract.rs`, `init_workflow.rs`, `payload_compat.rs`; shared setup in `common/`, insta snapshots in `snapshots/`. Run with `just test` (`cargo test`).

## Generated / vendored / ignored — do not treat as source

- `target/` — Cargo output
- `ref/repo/**` — vendored external repos (their own `CLAUDE.md`/`AGENTS.md` are theirs, not ours)
- `docs/node_modules/`, `docs/.vitepress/{dist,cache}` — npm/VitePress build state
- `evals/**/fixtures/**` — fixture vaults are test data, not runtime code

## Commands

Build/test/lint/CI and commit conventions live in `AGENTS.md` (`just ci` = `lint` + `test` + `docs-build`; `just lint` = `cargo fmt --check` + `cargo clippy -D warnings` + `dev contract-validate`).
