---
name: kb-review
description: Run the independent review gate for a Obsidian knowledge base. Use this skill whenever the user says "kb review", "review gate", "approve drafts", "reject draft knowledge", "promote to live", "rebuild briefings", "审校草稿", "批准草稿", "质量门", or whenever lifecycle detection reports `needs-review` or `needs-briefing-refresh`. Treat this as the immediate gate for pending drafts and briefing refresh work, not the longer-horizon maintenance lane.
---

# KB Review

Run the explicit gate between draft knowledge and the approved live brain. This is the immediate gate lane: resolve pending draft decisions, decide promotion or rejection, and rebuild briefings when that refresh belongs to the current review pass.

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
- `../obsidian-notes-karpathy/references/provenance-and-alias-policy.md`

Treat `skill-contract-registry.json` as the canonical source for required references, baseline script, and expected write surfaces.

If available, run:

- `../obsidian-notes-karpathy/scripts/scan_review_queue.py`
- `../obsidian-notes-karpathy/scripts/scan_query_scope.py` when validating downstream boundaries

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

## Outputs

`kb-review` may write:

- read and resolve pending work from `wiki/drafts/**`
- `outputs/reviews/*.md`
- promoted pages under `wiki/live/**`
- regenerated `wiki/briefings/{role}.md`
- `review` and `brief` entries in `wiki/log.md`

It should not mutate raw captures.

`kb-review` is the immediate gate for specific pending draft packages and briefing rebuilds. When the task is broader maintenance across approved surfaces, archived answers, backlog pressure, or provenance drift, route that work to `kb-health` instead.

Review records should capture whether fact and inference are separated cleanly enough for safe reuse, plus a short promotion reason explaining why the page should or should not persist.
