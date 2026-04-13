---
name: kb-review
description: Run the canonical governance lane for an Obsidian knowledge base. Use this skill whenever the user says "kb review", "kb health", "review gate", "health check", "approve drafts", "reject draft knowledge", "promote to live", "rebuild briefings", "lint live wiki", "审校草稿", "批准草稿", "知识库体检", or whenever lifecycle detection reports `needs-review`, `needs-briefing-refresh`, or `needs-maintenance`. This skill owns both the immediate gate and the longer-horizon maintenance lane through internal `gate` and `maintenance` modes. Do not use it for normal approved-layer retrieval or deterministic derivative rendering.
---

# KB Review

Run the canonical governance lane between draft knowledge and the approved live brain. `kb-review` owns two internal modes:

- `gate` mode for pending draft decisions, promotion/rejection, and briefing rebuilds tied to the same review pass
- `maintenance` mode for approved-layer drift, stale briefings, provenance and alias refresh, governance-index rebuilds, graph gaps, backlog pressure, and safe mechanical fixes

## Minimal loop

1. inspect the draft package, cited captures, and overlapping live pages
2. decide approve, reject, or escalate for human judgment
3. promote only durable, well-supported knowledge into live
4. rebuild briefings when the approved change should alter downstream context

## When this compounds the wiki

`kb-review` is where candidate structure becomes durable truth. Good review keeps the brain dense and reusable without letting noisy, redundant, or weakly-linked material accumulate in `wiki/live/`.

## When not to promote

A draft can be technically correct and still fail the promotion bar if it is redundant, weakly provenanced, poorly linked, or better expressed as a relationship or hub update instead of a new live page.

## Read before reviewing

Read these files first:

- local `AGENTS.md`
- local `CLAUDE.md` if present
- `../obsidian-notes-karpathy/scripts/skill-contract-registry.json`
- `../obsidian-notes-karpathy/references/file-model.md`
- `../obsidian-notes-karpathy/references/lifecycle-matrix.md`
- `../obsidian-notes-karpathy/references/schema-template.md`
- `../obsidian-notes-karpathy/references/review-template.md`
- `../obsidian-notes-karpathy/references/briefing-template.md`
- `../obsidian-notes-karpathy/references/activity-log-template.md`
- `../obsidian-notes-karpathy/references/health-rubric.md`
- `../obsidian-notes-karpathy/references/search-upgrades.md`
- `../obsidian-notes-karpathy/references/provenance-and-alias-policy.md`
- `../obsidian-notes-karpathy/references/questions-and-reflection-policy.md`
- `../obsidian-notes-karpathy/references/query-writeback-lifecycle.md`
- `../obsidian-notes-karpathy/references/memory-lifecycle.md`
- `../obsidian-notes-karpathy/references/graph-contract.md`
- `../obsidian-notes-karpathy/references/source-manifest-contract.md`
- `../obsidian-notes-karpathy/references/topic-template.md`
- `../obsidian-notes-karpathy/references/profile-contract.md`
- `../obsidian-notes-karpathy/references/automation-hooks.md`
- `../obsidian-notes-karpathy/references/episode-template.md`
- `../obsidian-notes-karpathy/references/procedure-template.md`

Treat `skill-contract-registry.json` as the canonical source for required references, baseline script, and expected write surfaces.

If available, run:

- `../obsidian-notes-karpathy/scripts/scan_review_queue.py`
- `../obsidian-notes-karpathy/scripts/scan_query_scope.py` when validating downstream boundaries
- `../obsidian-notes-karpathy/scripts/lint_obsidian_mechanics.py` for maintenance-mode diagnostics
- `../obsidian-notes-karpathy/scripts/build_governance_indices.py` when maintenance mode should refresh `QUESTIONS.md`, `GAPS.md`, `ALIASES.md`, `ENTITIES.md`, or `RELATIONSHIPS.md`
- `../obsidian-notes-karpathy/scripts/build_graph_snapshot.py` when the user wants machine-readable graph export during maintenance

## Independence rule

Review the draft package, the directly cited raw captures, and overlapping live pages only.

Do not preserve a draft merely because the producing agent seems competent or because the generation path looks sophisticated.

## Mixed gate policy

- auto-approve when the draft has no blocking flags, no unresolved live conflict, no unresolved alias/duplicate concern, and the review score clears the threshold
- auto-reject when evidence is weak, blocking flags remain, or the draft is clearly unsafe to compound
- send to human review when the draft conflicts with live knowledge, source integrity is suspect, or the score band is ambiguous

When the draft is borderline, weigh both accuracy and whether the page deserves promotion into the long-term brain. A technically correct draft can still stay out of `wiki/live/` if it is noisy, redundant, weakly provenanced, or not durable enough to merit reuse.

## What reviewers must judge explicitly

- are source claims and compiler inferences cleanly separated?
- is source integrity still credible?
- are aliases aligned with the existing approved vocabulary?
- does this draft duplicate an existing approved page?
- are contradictions recorded rather than silently overwritten?
- does the page deserve durable retention in the long-term brain?
- should the durable delta land in `wiki/live/procedures/` instead of another semantic page?
- is the confidence metadata ready to be promoted, reduced, or left unset?
- should the durable improvement be a promoted page, a stronger relationship edge, or inclusion in a curated hub?
- does this draft improve the browseable graph of the wiki, or does it just add more prose?

## Outputs

`kb-review` may write:

- read and resolve pending work from `wiki/drafts/**`
- `outputs/reviews/*.md`
- promoted pages under `wiki/live/**`
- promoted browse-layer topic pages under `wiki/live/topics/**`
- refreshed governance indices under `wiki/live/indices/**`
- regenerated `wiki/briefings/{role}.md`
- maintenance reports under `outputs/health/**`
- graph snapshot exports under `outputs/health/graph-snapshot.json`
- safe mechanical fixes in archived `outputs/qa/**` or `outputs/content/**` when the target is unambiguous
- `review` and `brief` entries in `wiki/log.md`

It should not mutate raw captures.

## Mode selection

Use `gate` mode when:

- drafts are pending
- a human approval boundary is the next safe step
- briefings should be rebuilt because the current review pass changed approved truth

Use `maintenance` mode when:

- the approved layer is drifting, contradictory, stale, or weakly linked
- archived answers or content have writeback backlog
- confidence metadata, supersession bookkeeping, or audit trails have decayed
- governance indices or graph exports need deterministic refresh
- safe mechanical fixes in approved or archived surfaces are clearer than creating new prose

`kb-review` is now the only public governance surface. If the user literally says `kb-health`, treat that as `kb-review` maintenance mode.

Review records should capture whether fact and inference are separated cleanly enough for safe reuse, plus a short promotion reason explaining why the page should or should not persist.
