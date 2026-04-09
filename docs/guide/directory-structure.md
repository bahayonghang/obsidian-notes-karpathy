# Directory Structure

The vault is intentionally split into evidence, drafts, approved knowledge, and downstream outputs.

```text
vault/
├── raw/
│   ├── human/{articles,papers,podcasts,repos,assets}/
│   └── agents/{role}/
├── MEMORY.md
├── wiki/
│   ├── drafts/{summaries,concepts,entities,indices}/
│   ├── live/{summaries,concepts,entities,indices}/
│   ├── briefings/
│   ├── index.md
│   └── log.md
├── outputs/
│   ├── reviews/
│   ├── qa/      (created when query archiving starts)
│   ├── health/  (created when health reporting starts)
│   └── content/ (created when publish flows start)
├── AGENTS.md
└── CLAUDE.md
```

`outputs/reviews/` is part of the required support layer. The other output surfaces are valid parts of the full contract, but they come online when later stages need them.

`MEMORY.md` is recommended collaboration scaffolding. It should hold preferences, editorial priorities, and coordination context, not approved topic knowledge.

Core live indices such as `INDEX.md`, `CONCEPTS.md`, `SOURCES.md`, `RECENT.md`, and `EDITORIAL-PRIORITIES.md` belong under `wiki/live/indices/`. Optional governance views such as `QUESTIONS.md`, `GAPS.md`, and `ALIASES.md` are created only when the vault needs them.

## Ownership model

| Path | Owner | Purpose |
| --- | --- | --- |
| `raw/human/**` | Human | Curated evidence intake |
| `raw/agents/{role}/**` | Agents | Untrusted captures that still need review |
| `MEMORY.md` | Human + agent collaboration | Preferences, editorial priorities, and coordination context |
| `raw/*.md` | `kb-compile` bootstrap input | Valid compile input during partial bootstrap, but not a replacement for the support layer |
| `wiki/drafts/**` | `kb-compile` | Reviewable summaries, concepts, entities, and indices |
| `wiki/live/**` | `kb-review` promotion target | Approved long-term knowledge |
| `wiki/briefings/**` | `kb-review` | Role-specific context built from live only |
| `outputs/reviews/**` | `kb-review` | Durable review decisions |
| `outputs/qa/**` | `kb-query` | Persistent research answers |
| `outputs/content/**` | `kb-query` | Publish artifacts grounded in approved knowledge |
| `outputs/health/**` | `kb-health` | Maintenance reports |
