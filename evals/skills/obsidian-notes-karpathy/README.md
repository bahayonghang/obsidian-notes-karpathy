# Evals and fixtures

This directory stores the bundle's regression fixtures and evaluation context.

## How to read this folder

The fixtures are grouped by lifecycle and contract boundary rather than by a single happy path.

### Lifecycle routing fixtures
- `needs-setup/` - uninitialized vaults that should route to `kb-init`
- `needs-repair/` and `needs-repair-live-only/` - partial support-layer cases that still route to `kb-init`
- `needs-migration/` - legacy-layout vaults that must migrate before normal operation
- `needs-compilation/` and `bootstrap-raw-intake/` - raw captures that should route to `kb-compile`
- `needs-review/` and `needs-briefing-refresh/` - draft gate and briefing-refresh cases for `kb-review`
- `ready-for-query/` - approved live-layer path for `kb-query`
- `needs-maintenance/` - long-horizon maintenance path for `kb-review` maintenance mode

### Contract boundary fixtures
- `review-conflict-mixed-gate/` - approve / reject / needs-human mixed review outcomes
- `memory-boundary/` - proves `MEMORY.md` is collaboration context, not topic truth
- `writeback-backlog/` - archived output backlog that belongs to maintenance
- `paper-intake-with-handle/` and `paper-intake-missing-handle/` - strict `raw/papers` companion-skill routing

## Canonical examples
Use these first when explaining or debugging the contract:
- `needs-review/` - immediate review gate, trust split between human and agent captures
- `ready-for-query/` - approved live retrieval path
- `memory-boundary/` - retrieval boundary excluding `MEMORY.md`
- `needs-briefing-refresh/` - review-triggered briefing rebuild
- `writeback-backlog/` - health-side backlog and writeback pressure

## Companion skill fixture note
Some fixture homes intentionally contain `SKILL.md` files under `fixtures/companion-skill-homes/**`. Those are fixture-only companion skill homes used to test paper/PDF routing. They are not part of this bundle's top-level skill family.

## Test coverage map
`tests/contract_routing.rs`, `tests/compile_review.rs`, `tests/query_health.rs`, `tests/docs_contract.rs`, and `tests/dev_tools.rs` mainly protect:
- lifecycle routing and state detection
- compile provenance and paper intake policy
- mixed-gate review behavior
- query truth boundaries excluding raw, drafts, and `MEMORY.md`
- health checks for stale briefings, backlog pressure, memory leakage, and writeback backlog
- bundle validation and runtime-eval scaffolding

## Compatibility benchmark loop

When you need an old/new review pass for skill wording:

1. snapshot the tracked baseline into `skills/obsidian-notes-karpathy-workspace/skill-snapshot/`
2. run `onkb dev eval-trigger --dry-run --skill <name>` and `onkb dev eval-runtime --dry-run --skill <name> [--eval-id <id>] [--reuse-baseline-from <workspace>]` for the current tree
3. write benchmark artifacts under `skills/obsidian-notes-karpathy-workspace/iteration-1/`
4. capture the result with `onkb dev audit-skills --json`

On this Windows machine:

- treat `evals/trigger-evals.json`, `evals/runtime-evals.json`, and `evals/runtime-evals-writable.json` as the authoritative scenario lists
- prefer `onkb dev audit-skills --json` over ad-hoc reviewer scripts
- keep evaluation entrypoints on `onkb`; the bundle no longer uses repo-local script wrappers
- for `obsidian-notes-karpathy`, `kb-init`, `kb-ingest`, and `kb-compile`, prefer `--skill` / `--eval-id` slices and raise `--timeout-sec` before attempting a broad runtime batch
- when iterating on a read-only skill, point `--reuse-baseline-from` at the previous runtime workspace so only the with-skill leg reruns
