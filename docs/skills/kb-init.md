# kb-init

Initialize, repair, or migrate a vault onto the review-gated contract. This is also the right skill when the user asks for an `LLM Wiki`, a `Karpathy wiki`, an `Obsidian IDE` for notes, a `knowledge compiler`, or a markdown-first personal knowledge base without knowing the `kb-init` name.

## Creates or repairs the support layer

- `raw/human/{articles,papers,podcasts,repos,assets}`
- `raw/human/data`
- `raw/agents/{role}/**`
- `raw/_manifest.yaml`
- `MEMORY.md`
- `wiki/drafts/{summaries,topics,concepts,entities,indices}`
- `wiki/live/{summaries,topics,concepts,entities,indices}`
- `wiki/briefings/**`
- `wiki/index.md`
- `wiki/log.md`
- `outputs/reviews/**`
- `AGENTS.md`
- `CLAUDE.md`

## Governance and starter files

`kb-init` should also scaffold the core live indices:

- `wiki/live/indices/INDEX.md`
- `wiki/live/indices/CONCEPTS.md`
- `wiki/live/indices/SOURCES.md`
- `wiki/live/indices/TOPICS.md`
- `wiki/live/indices/RECENT.md`
- `wiki/live/indices/EDITORIAL-PRIORITIES.md`

Optional governance views such as `wiki/live/indices/QUESTIONS.md`, `GAPS.md`, and `ALIASES.md` should be created only when the user asks for richer maintenance scaffolding.

`MEMORY.md` is recommended collaboration scaffolding rather than approved knowledge storage. It holds preferences, editorial priorities, and coordination context.

## Deterministic helpers

- `bootstrap_review_gated_vault.py` for fresh setup and non-destructive repair
- `migrate_legacy_vault.py` for single-layer legacy vault migration with a preserved-source report
- `vault_status.py` for a quick current-stage summary after setup or repair

Optional downstream scaffolding such as `outputs/qa/**`, `outputs/health/**`, `outputs/content/**`, `outputs/reports/**`, `outputs/slides/**`, or `outputs/charts/**` should be created only when the user wants full setup or later stages need them.

## Profile choice

`kb-init` should surface and preserve one of three profiles in starter files:

- `governed-team`
- `standard`
- `fast-personal`

## Migration posture

For legacy-layout vaults, `kb-init` should scaffold the review-gated structure first, preserve existing material, and treat direct `wiki/summaries/` or `wiki/concepts/` content as migration work rather than normal compile output.
