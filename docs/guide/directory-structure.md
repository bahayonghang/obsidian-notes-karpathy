# Directory Structure

The vault is intentionally split into evidence, drafts, approved knowledge, and downstream outputs.

```text
vault/
├── raw/
│   ├── human/
│   └── agents/{role}/
├── wiki/
│   ├── drafts/
│   ├── live/
│   ├── briefings/
│   ├── index.md
│   └── log.md
├── outputs/
│   ├── reviews/
│   ├── qa/
│   ├── health/
│   └── content/
├── AGENTS.md
└── CLAUDE.md
```

## Ownership model

| Path | Owner | Purpose |
| --- | --- | --- |
| `raw/human/**` | Human | Curated evidence intake |
| `raw/agents/{role}/**` | Agents | Untrusted captures that still need review |
| `wiki/drafts/**` | `kb-compile` | Reviewable summaries, concepts, entities, and indices |
| `wiki/live/**` | `kb-review` promotion target | Approved long-term knowledge |
| `wiki/briefings/**` | `kb-review` | Role-specific context built from live only |
| `outputs/reviews/**` | `kb-review` | Durable review decisions |
| `outputs/qa/**` | `kb-query` | Persistent research answers |
| `outputs/health/**` | `kb-health` | Maintenance reports |
