# Repository Guidelines

## Project Structure & Module Organization
This repository ships review-gated Obsidian knowledge-base skills, not a standalone app. Core logic lives in `skills/obsidian-notes-karpathy/`: `scripts/` contains deterministic Python helpers and the skill contract registry, `references/` holds shared templates and rules, and `evals/fixtures/` stores fixture vaults for contract tests. Companion skills live in `skills/kb-init`, `skills/kb-compile`, `skills/kb-review`, `skills/kb-query`, and `skills/kb-render`. Public docs are under `docs/`, with static assets in `docs/public/`. Keep `README.md`, `README_CN.md`, and `CLAUDE.md` aligned when the contract changes.

## Build, Test, and Development Commands
- `just install`: install VitePress dependencies in `docs/`.
- `just docs` or `just docs-dev`: start the local docs server.
- `just docs-build`: build the production docs site.
- `just test`: run deterministic `unittest` coverage from the `tests/test_*.py` suite.
- `just ci`: run tests, the lightweight lint check, and the docs build together.
- `just clean`: remove VitePress caches and `docs/node_modules`.

## Coding Style & Naming Conventions
Use Python 3 with 4-space indentation, type hints, `pathlib.Path`, and snake_case module and function names such as `detect_lifecycle.py` and `scan_query_scope.py`. Prefer deterministic, stdlib-first helpers over framework-heavy additions. Reuse shared utilities in `skills/obsidian-notes-karpathy/scripts/_vault_utils.py` before creating new helpers. Markdown docs should use short headings and keep repository terms exact: `raw/`, `wiki/drafts/`, `wiki/live/`, `wiki/briefings/`, and `outputs/reviews/`. Fixture note files follow date-prefixed slugs like `2026-04-05-approved-summary.md`.

## Testing Guidelines
Tests use Python `unittest`. When changing routing, vault mechanics, or contract wording, add or update fixture-driven cases under `skills/obsidian-notes-karpathy/evals/fixtures/` and assertions in the `tests/test_*.py` suite. Name test methods `test_<behavior>`. Keep script output machine-readable JSON so tests can assert exact fields. Run `just test` before every PR; run `just ci` when changes touch both code and docs.

## Commit & Pull Request Guidelines
Follow the existing Conventional Commit pattern from history: `feat(知识库): ...`, `fix(知识库): ...`, `docs(知识库): ...`, or `build(仓库): ...`. Keep each commit focused on one concern and write imperative subjects. `just commit msg="..." type="feat"` is available for simple commits. Pull requests should explain the affected skills or docs pages, list verification commands run, call out any new fixture coverage, and include screenshots when `docs/` output changes.
