# kb-init

Initialize, repair, or migrate a vault onto the review-gated contract.

## Creates or repairs

- `raw/human/**`
- `raw/agents/{role}/**`
- `MEMORY.md`
- `wiki/drafts/**`
- `wiki/live/**`
- `wiki/briefings/**`
- `wiki/index.md`
- `wiki/log.md`
- `outputs/reviews/**`
- `AGENTS.md`
- `CLAUDE.md`

Optional downstream scaffolding such as `outputs/qa/**`, `outputs/health/**`, or `outputs/content/**` should be created only when the user wants full setup or the later stages need them.

`kb-init` should also scaffold `wiki/live/indices/EDITORIAL-PRIORITIES.md` so the vault has an explicit editor-in-chief surface instead of relying on implicit preferences.
