---
name: kb-review
description: Run the canonical governance lane for an Obsidian knowledge base. Use this skill whenever the user says "kb review", "kb health", "review gate", "health check", "approve drafts", "reject draft knowledge", "promote to live", "rebuild briefings", "lint live wiki", "clear archive backlog", "audit archived outputs", "检查归档内容", "检查风格约束是否冲突", "检查账号 briefing", "审校草稿", "批准草稿", "知识库体检", or uses Chinese-LLM-Wiki maintenance wording such as `lint`, `孤儿页`, `断链`, `旧结论被覆盖`, `output/reports`, `治理报告`, `先做 lint`, and `原文证据摘录是否充分`. This skill owns both the immediate gate and the longer-horizon maintenance lane through internal `gate` and `maintenance` modes. Do not use it for normal approved-layer retrieval or deterministic derivative rendering.
---

# KB Review

Run the canonical governance lane between draft knowledge and the approved live brain. `kb-review` owns two internal modes:

- `gate` mode for pending draft decisions, promotion/rejection, and briefing rebuilds tied to the same review pass
- `maintenance` mode for approved-layer drift, stale briefings, provenance and alias refresh, creator consistency checks, governance-index rebuilds, graph gaps, backlog pressure, and safe mechanical fixes

In Karpathy's LLM Wiki, this maps to two operations: the implicit quality judgment during ingest (our `gate` mode), and the explicit "Lint" pass he describes — "look for contradictions between pages, stale claims that newer sources have superseded, orphan pages with no inbound links, important concepts mentioned but lacking their own page, missing cross-references" (our `maintenance` mode). This contract makes both operations explicit and separable.

When a user says `output/reports`, decide whether they mean a governance report or a deterministic rendered report. Lint, contradiction, orphan-page, and stale-claim work stays here in `maintenance` mode.

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
- `../obsidian-notes-karpathy/references/chinese-llm-wiki-compat.md`
- `../obsidian-notes-karpathy/references/archive-model.md`
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

Treat `skill-contract-registry.json` as the canonical source for required references, baseline command, and expected write surfaces.

If `onkb` is available, run:

- `onkb --json review queue <vault-root>`
- `onkb --json query scope <vault-root>` when validating downstream boundaries
- `onkb --json review lint <vault-root>` for maintenance-mode diagnostics
- `onkb --json review governance <vault-root>` when maintenance mode should refresh `QUESTIONS.md`, `GAPS.md`, `ALIASES.md`, `ENTITIES.md`, or `RELATIONSHIPS.md`
- `onkb --json review graph <vault-root>` when the user wants machine-readable graph export during maintenance

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

## Procedural promotion

When a draft carries `promotion_target: procedural` in its frontmatter, promote the durable delta into `wiki/live/procedures/{slug}.md` instead of `wiki/live/concepts/`. Use the procedure frontmatter from `../obsidian-notes-karpathy/references/procedure-template.md` (`procedure_id`, `confidence_band`, `decay_class`, `next_review_due_at`, etc.).

Promote procedurally when the durable delta answers "how to do X" rather than "what X is" — a workflow, playbook, or repeated decision pattern. Typical signals: the draft reads as a sequence of steps, the same pattern applies across future situations, repeated question clusters resolve into the same sequence of actions.

Decision rules:

- if `promotion_target: procedural` is set and the signals hold, promote into `wiki/live/procedures/`
- if the flag is set but the content is actually semantic, flip to `promotion_target: semantic` in the review record and promote into `wiki/live/concepts/`
- if both targets plausibly fit, prefer procedural when the reuse value is in the sequence of actions; prefer semantic when it is in a definition
- if neither target fits cleanly, record the ambiguity in the review record and send to human review

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
- `review` and `brief` entries in `wiki/log.md`

It should not mutate raw captures.

`outputs/qa/**` and `outputs/content/**` are owned by `kb-query`. `kb-review` touches them only as `writes_mechanical_fix_only` targets declared in `scripts/skill-contract-registry.json` — deterministic, unambiguous mechanical fixes during maintenance mode (broken links, stale frontmatter, scope metadata drift, obvious typos). Any substantive new content or re-synthesis belongs back in `kb-query`, not here.

## Mode selection

Use `gate` mode when:

- drafts are pending
- a human approval boundary is the next safe step
- briefings should be rebuilt because the current review pass changed approved truth

Use `maintenance` mode when:

- the approved layer is drifting, contradictory, stale, or weakly linked
- archived answers or content have writeback backlog
- archived outputs have reuse drift, stale claims, or private/shared scope leaks
- confidence metadata, supersession bookkeeping, or audit trails have decayed
- governance indices or graph exports need deterministic refresh
- safe mechanical fixes in approved or archived surfaces are clearer than creating new prose
- creator-facing guidance surfaces such as `CLAUDE.md`, `MEMORY.md`, account `_style-guide.md`, or account briefings may be drifting apart
- archived publish outputs are not reusing prior approved coverage cleanly
- important approved sources are staying underused in downstream Q&A or publish artifacts

`kb-review` is now the only public governance surface. If the user literally says `kb-health`, treat that as `kb-review` maintenance mode.

Review records should capture whether fact and inference are separated cleanly enough for safe reuse, plus a short promotion reason explaining why the page should or should not persist.

Archive hygiene in maintenance mode should stay explicit:

- stale archived Q&A relative to newer live pages
- archived publish artifacts that no longer reuse the best approved coverage
- archive backlog with `writeback_status: pending`
- scope leaks or sensitivity metadata drift in archived outputs

This is maintenance of the artifact archive, not promotion of archive into truth.
