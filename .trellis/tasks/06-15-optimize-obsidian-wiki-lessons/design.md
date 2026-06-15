# Design

## Recommendation

Adopt selected obsidian-wiki strengths as additive lanes around the existing review-gated core. The first implementation wave should improve `onkb` setup/distribution and installation visibility. The second wave should add cross-agent memory/query skills. Query ergonomics and Obsidian graph/export helpers should follow only after the core surfaces are stable.

## Why This Direction

The reference repo's strongest advantage is product packaging: a user can install one package, run setup, and get skills plus bootstrap files across many agents. This repo's strongest advantage is deterministic Rust lifecycle enforcement. The optimization should combine those strengths instead of copying the reference repo's single-layer write model.

## Phase 1: CLI Distribution Setup

Add a more complete setup lane to `onkb` without changing vault lifecycle semantics.

Candidate scope:

- Extend skill installation beyond `.claude/skills` and `.agents/skills`.
- Add global install targets for supported agent homes where appropriate.
- Add project-local bootstrap file support only when explicitly requested.
- Add an `info` or enhanced `doctor` view that reports installed skills, missing targets, and stale versions.
- Keep embedded runtime skills as the single source of truth.

Likely repo surfaces:

- `src/cli/args.rs`
- `src/cli/dispatch.rs`
- `src/support/skills.rs`
- `src/payload/mod.rs`
- `tests/dev_tools.rs` or new CLI install tests
- README, README_CN, CLAUDE, docs skill-install pages

## Phase 2: Cross-Agent Memory Skills

Add a review-gated equivalent of the reference repo's `wiki-agent` and `memory-bridge`, but route writes through this repo's draft/review model.

Candidate scope:

- A `kb-agent-history` companion or core-adjacent skill for targeted history discovery.
- A `kb-memory-bridge` read-side skill for source-origin browsing and diffing.
- History-derived material should enter `raw/agents/<role>/` or a deferred output surface, then pass through `kb-ingest`, `kb-compile`, and `kb-review`.
- The skill should never write directly into `wiki/live`.

Likely repo surfaces:

- New `skills/kb-agent-history/SKILL.md` if adopted as runtime skill.
- New `skills/kb-memory-bridge/SKILL.md` if adopted as runtime skill.
- `skills/obsidian-notes-karpathy/scripts/skill-contract-registry.json`
- Shared references for source provenance and memory lifecycle.
- Docs and fixture coverage for routing and truth-boundary behavior.

## Phase 3: Query Ergonomics

Improve `kb-query` retrieval quality while preserving approved-layer-only semantics.

Candidate scope:

- Tiered retrieval: index/frontmatter first, then sections, then full pages.
- Optional hot-cache read from approved/generated surfaces only.
- Optional semantic search hook if configured, with grep fallback.
- Typed-edge traversal for relationship and multi-hop questions, using approved `wiki/live` metadata only.
- Stale/lifecycle annotations for cited pages.

Likely repo surfaces:

- `skills/kb-query/SKILL.md`
- `skills/obsidian-notes-karpathy/references/search-upgrades.md`
- `src/kb/query.rs` if deterministic ranking needs to grow.
- Query tests and fixture vaults.

## Phase 4: Obsidian UX Helpers

Add graph/export/dashboard helpers only after the first three phases are stable.

Candidate scope:

- Graph export formats and interactive HTML export.
- Obsidian graph colorization.
- Dashboard or Bases helper as a companion skill.
- Import/export interoperability only if the format can preserve review state and provenance.

Likely repo surfaces:

- `kb-render` or new companion skills.
- `src/kb/render/` only for deterministic generation.
- Docs, fixtures, and export-output tests.

## Rejected Alternative

Do not port obsidian-wiki wholesale. It would widen the skill surface quickly, but it would import single-layer wiki assumptions that conflict with this repo's `draft -> review -> live` contract and would likely create inconsistent write paths.

## Risk Boundaries

- Installation changes can affect user home directories; default behavior should remain project-local or explicit.
- Cross-agent history parsing touches private local data; skills must document scope and avoid broad reads unless the user asks for them.
- Semantic search integrations are optional. Missing external tools must degrade to deterministic local search.
- Export/import must not turn derived artifacts into approved truth.

## Rollback Shape

Each phase should be independently mergeable. Phase 1 can ship as CLI setup improvements without any new knowledge workflow. Phase 2 can ship as skills that only stage or report findings before review. Phase 3 can ship as query contract updates without changing write paths. Phase 4 can ship as deterministic derived-output helpers.
