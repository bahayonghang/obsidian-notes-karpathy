# kb-init

Initialize, repair, or migrate a vault onto the review-gated contract.

## Creates or repairs the support layer

- `raw/human/{articles,papers,podcasts,repos,assets}`
- `raw/agents/{role}/**`
- `MEMORY.md`
- `wiki/drafts/{summaries,concepts,entities,indices}`
- `wiki/live/{summaries,concepts,entities,indices}`
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
- `wiki/live/indices/RECENT.md`
- `wiki/live/indices/EDITORIAL-PRIORITIES.md`

Optional governance views such as `wiki/live/indices/QUESTIONS.md`, `GAPS.md`, and `ALIASES.md` should be created only when the user asks for richer maintenance scaffolding.

`MEMORY.md` is recommended collaboration scaffolding rather than approved knowledge storage. It holds preferences, editorial priorities, and coordination context.

Optional downstream scaffolding such as `outputs/qa/**`, `outputs/health/**`, `outputs/content/**`, `outputs/reports/**`, `outputs/slides/**`, or `outputs/charts/**` should be created only when the user wants full setup or later stages need them.

## Migration posture

For legacy-layout vaults, `kb-init` should scaffold the review-gated structure first, preserve existing material, and treat direct `wiki/summaries/` or `wiki/concepts/` content as migration work rather than normal compile output.
