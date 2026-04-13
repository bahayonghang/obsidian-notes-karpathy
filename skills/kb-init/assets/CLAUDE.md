# {{VAULT_NAME}} Contract

This vault is a review-gated Obsidian knowledge base inspired by the Karpathy LLM Wiki pattern.

## Core Rules
- Keep `raw/` immutable.
- Keep `raw/_manifest.yaml` as the canonical source registry.
- Treat `wiki/drafts/` as compiler output, not truth.
- Treat `wiki/live/` as the approved brain.
- Treat `wiki/briefings/` as generated runtime context from live only.
- Treat `MEMORY.md` as editorial and collaboration context only.

## Operating Profile
- `kb_profile: {{KB_PROFILE}}`

## Recommended Loop
1. `kb-ingest`
2. `kb-compile`
3. `kb-review`
4. `kb-query` or `kb-render`
5. `kb-review` maintenance mode

## Deterministic Helpers
- Use lifecycle/status tooling before guessing the next step.
- Prefer migration tooling when legacy direct-compiled paths still exist.
