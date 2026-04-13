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
`tests/test_contract_and_routing.py`, `tests/test_compile_review.py`, `tests/test_query_health.py`, `tests/test_docs_and_evals.py`, and `tests/test_quality_commands.py` mainly protect:
- lifecycle routing and state detection
- compile provenance and paper intake policy
- mixed-gate review behavior
- query truth boundaries excluding raw, drafts, and `MEMORY.md`
- health checks for stale briefings, backlog pressure, memory leakage, and writeback backlog
- bundle validation and runtime-eval scaffolding
