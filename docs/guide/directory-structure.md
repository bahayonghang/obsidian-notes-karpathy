# Directory Structure

The vault is intentionally split into immutable sources, compiled knowledge, and generated outputs.

```text
vault/
├── raw/
│   ├── articles/
│   ├── papers/
│   ├── podcasts/
│   └── assets/
├── wiki/
│   ├── concepts/
│   ├── summaries/
│   ├── indices/
│   ├── index.md
│   └── log.md
├── outputs/
│   ├── qa/
│   ├── health/
│   ├── reports/
│   ├── slides/
│   └── charts/
├── AGENTS.md
└── CLAUDE.md
```

## Meanings

- `raw/`: human-curated, immutable source material
- `wiki/`: LLM-maintained compiled artifact
- `outputs/`: generated artifacts and persistent research results

## Important consequence

Do not track compilation state in raw notes. Use summary frontmatter or health outputs instead.
