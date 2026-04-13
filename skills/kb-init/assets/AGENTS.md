# Vault Guidance

This Obsidian vault follows the review-gated Karpathy LLM Wiki contract.

## Active Profile
- `{{KB_PROFILE}}`

## Truth Boundary
- `raw/` is immutable evidence intake.
- `raw/_manifest.yaml` is the canonical source registry.
- `wiki/drafts/` contains reviewable, unapproved knowledge.
- `wiki/live/` contains approved long-term knowledge.
- `wiki/briefings/` is generated from approved live pages only.
- `outputs/reviews/` is the decision ledger.
- `MEMORY.md` is collaboration context, not topic truth.

## Safe Next Steps
1. Register new sources with `kb-ingest`.
2. Compile raw captures into `wiki/drafts/` with `kb-compile`.
3. Promote only reviewed knowledge with `kb-review`.
4. Query or render only from approved live knowledge.

## Migration Note
- If legacy `wiki/summaries/`, `wiki/concepts/`, or `raw/articles/` paths still exist, treat them as migration surfaces rather than normal active layers.
