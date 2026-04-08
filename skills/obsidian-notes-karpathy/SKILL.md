---
name: obsidian-notes-karpathy
description: Diagnose and route ambiguous, workflow-level review-gated Obsidian vault requests. Use this skill when the user is talking about an Obsidian vault workflow as a whole, asks which lifecycle step should run next, mentions review gates, draft/live separation, briefings, or a markdown-first "living book" / "second brain" in Obsidian. Prefer the operation-specific skills when the user already clearly means init, compile, review, query, or health, and only route through this package entry skill when the workflow step is genuinely ambiguous.
---

# Obsidian Notes Karpathy

Package-level entry point for the review-gated knowledge workflow.

Use this skill when the user talks about the workflow as a whole, not just one operation. Diagnose the lifecycle stage first, then route to the correct operational skill. If the operation is already obvious, skip the router and go straight to the matching `kb-*` skill.

## Read before routing

Read these shared references first:

- `./scripts/skill-contract-registry.json`
- `./references/file-model.md`
- `./references/lifecycle-matrix.md`
- `./references/search-upgrades.md`
- `./references/activity-log-template.md`

Treat `skill-contract-registry.json` as the canonical list of package roles, required shared references, baseline scripts, and output surfaces.

If the target vault already exists, inspect:

- local `AGENTS.md`
- local `CLAUDE.md` if present
- the top of `wiki/index.md`
- the most recent entries in `wiki/log.md` when available

If `./scripts/detect_lifecycle.py` exists, run it first and treat its JSON output as the deterministic baseline.

## Lifecycle signals

### Init signals

Route to `kb-init` when:

- the support layer does not exist yet
- `AGENTS.md` is missing
- `wiki/index.md` or `wiki/log.md` is missing
- `wiki/drafts/`, `wiki/live/`, `wiki/briefings/`, or `outputs/reviews/` is missing
- the vault is a `legacy-layout` that must be migrated first

### Compile signals

Route to `kb-compile` when:

- new or changed raw captures exist under `raw/human/**`, `raw/agents/{role}/**`, or directly under `raw/` in a bootstrap vault
- the draft summary matching a raw capture is missing or outdated
- a legacy-layout raw source still needs to be converted into the draft layer during migration

### Review signals

Route to `kb-review` when:

- draft knowledge exists under `wiki/drafts/**` and still has `review_state: pending`
- the user explicitly asks to run the quality gate, approve drafts, reject drafts, or rebuild briefings
- briefings are stale relative to the live layer and the next immediate step is to rebuild them through the gate

`kb-review` owns the immediate gate: pending draft decisions, promotion into `wiki/live/`, and briefing rebuilds that should happen as part of the current review pass.

### Query signals

Route to `kb-query` when:

- `wiki/live/**` is trustworthy and current
- the user wants an answer, report, thread, slides, or other artifact grounded in the approved brain

### Health signals

Route to `kb-health` when:

- the live layer feels contradictory, weakly linked, or poorly provenanced
- approved pages appear to have bypassed review
- review backlog, stale briefings, or writeback pressure have become longer-horizon maintenance issues rather than the next immediate gate
- archived answers have pending writeback work
- collaboration memory and approved knowledge appear to be mixing
- the user wants a maintenance baseline, drift audit, or report-first cleanup pass across approved surfaces

`kb-health` owns the longer-horizon maintenance lane: approved-layer drift, backlog pressure, archived-output hygiene, and safe mechanical fixes after the immediate review gate has passed.

## Package doctrine

- Treat `raw/` as immutable evidence intake.
- Treat `wiki/drafts/` as reviewable knowledge, not query truth.
- Treat `wiki/live/` as the only approved long-term brain.
- Treat `wiki/briefings/` as per-role context generated from live only.
- Treat `outputs/reviews/` as the durable decision ledger.
- Keep `wiki/index.md` and `wiki/log.md` as complementary navigation surfaces.

## Output requirements

When the user asks what to do next, answer with:

1. the lifecycle stage you detected
2. the concrete signals that led you there
3. the operational skill you are routing to
4. the next concrete action in the vault
5. any assumption you had to make
6. any repair target, migration warning, or missing companion guidance
