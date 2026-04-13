# obsidian-notes-karpathy

Use the package entry skill when the user describes the Obsidian vault workflow as a whole, asks what lifecycle step should run next, or uses novice phrases like `LLM Wiki`, `Karpathy wiki`, `Obsidian IDE`, `knowledge compiler`, `personal knowledge base`, or `second brain`.

If the operation is already clear, prefer the operation-specific skill instead of the router.

## Main routing outcomes

- `kb-init` for fresh, partial, or legacy-layout vaults, especially when `wiki/drafts/`, `wiki/live/`, `wiki/briefings/`, or `outputs/reviews/` is still missing
- `kb-ingest` when `raw/_manifest.yaml` is stale or missing tracked raw-source updates
- `kb-compile` when markdown captures under `raw/human/**`, `raw/agents/{role}/**`, or bootstrap `raw/*.md` are ahead of the draft layer
- `kb-review` when drafts are pending, the immediate next job is to promote or reject them, or briefings should be rebuilt as part of the same gate pass
- `kb-query` when the live layer is ready for retrieval, local-first candidate ranking, grounded answers, archived answer reuse, or static web export
- `kb-query` when the user explicitly uses older `kb-search` wording for search-first retrieval
- `kb-render` when the user wants deterministic derivatives such as slides, reports, charts, or canvas artifacts
- `kb-review` when approved knowledge needs longer-horizon maintenance, governance refresh, writeback backlog review, or collaboration memory and approved knowledge appear to be mixing

## Router posture

- inspect lifecycle signals first instead of guessing from the user's wording alone
- prefer the status wrapper when the user mainly wants a concise current-stage summary
- treat `kb-review` as the canonical governance skill, with `gate` and `maintenance` modes
- preserve the hard boundary that `raw/` is evidence, `wiki/drafts/` is reviewable, and `wiki/live/` is the only approved retrieval truth
