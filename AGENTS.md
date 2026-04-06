# Repository Guidelines

## Project Structure & Module Organization

`skills/` is the core product. `skills/obsidian-notes-karpathy/` is the entry bundle and contains the package `SKILL.md`, shared `references/`, deterministic Python `scripts/`, and `evals/fixtures/`. Operation-specific skills live in sibling folders such as `skills/kb-init/`, `skills/kb-compile/`, `skills/kb-query/`, and `skills/kb-health/`.

`docs/` is the bilingual VitePress site. Use `docs/guide/` for conceptual pages, `docs/skills/` for skill-specific pages, `docs/workflow/` for lifecycle docs, and `docs/zh/` for the Chinese mirror. `tests/test_skill_bundle.py` holds the regression suite. Treat `ref/` as read-only reference material.

## Build, Test, and Development Commands

- `just docs-install` installs the VitePress dependencies in `docs/`.
- `just docs` or `just docs-dev` starts the local docs server.
- `just docs-build` builds the production site; `just docs-preview` serves the build locally.
- `just test` runs `python3 -m unittest tests/test_skill_bundle.py`.
- `just ci` runs the current local validation chain: tests, the lightweight `lint` recipe, and a docs build.

For docs-only work, you can also run `cd docs && npm run dev`.

## Coding Style & Naming Conventions

Keep edits contract-first: `SKILL.md` files and deterministic scripts are the source of truth, and docs should explain them without inventing parallel behavior. Use 4-space indentation in Python. Follow existing kebab-case naming for skill folders and doc files such as `kb-health` and `quick-start.md`.

When editing skill content, preserve Obsidian-flavored Markdown conventions like YAML frontmatter, `[[wikilinks]]`, and callout blocks. Keep `README.md` and `README_CN.md` aligned whenever public behavior changes.

## Testing Guidelines

Add or update `unittest` coverage for every routing rule, helper script change, or contract-visible behavior. Place new fixtures under `skills/obsidian-notes-karpathy/evals/fixtures/` and keep them minimal and deterministic. Prefer assertions on concrete JSON fields, paths, and route decisions.

## Commit & Pull Request Guidelines

Recent history uses scoped Conventional Commits, for example `docs(技能): ...`, `test(知识库): ...`, `refactor(知识库): ...`, and `chore(仓库): ...`. Keep commits narrow and use imperative subjects.

PRs should summarize the affected skill or docs path, list any added fixtures or tests, and include screenshots when `docs/` output changes visually. Link the related issue when one exists.
