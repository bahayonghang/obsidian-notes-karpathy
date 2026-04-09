# obsidian-notes-karpathy

Use the package entry skill when the user describes the Obsidian vault workflow as a whole or asks what lifecycle step should run next.

If the operation is already clear, prefer the operation-specific skill instead of the router.

## Main routing outcomes

- `kb-init` for fresh, partial, or legacy-layout vaults, especially when `wiki/drafts/`, `wiki/live/`, `wiki/briefings/`, or `outputs/reviews/` is still missing
- `kb-compile` when markdown captures under `raw/human/**`, `raw/agents/{role}/**`, or bootstrap `raw/*.md` are ahead of the draft layer
- `kb-review` when drafts are pending, the immediate next job is to promote or reject them, or briefings should be rebuilt as part of the same gate pass
- `kb-query` when the live layer is ready for retrieval, grounded publishing, or archived research outputs
- `kb-health` when approved knowledge needs longer-horizon maintenance, governance refresh, writeback backlog review, or collaboration memory and approved knowledge appear to be mixing

## Router posture

- inspect lifecycle signals first instead of guessing from the user's wording alone
- treat `kb-review` as the immediate gate lane and `kb-health` as the broader maintenance lane
- preserve the hard boundary that `raw/` is evidence, `wiki/drafts/` is reviewable, and `wiki/live/` is the only approved retrieval truth
