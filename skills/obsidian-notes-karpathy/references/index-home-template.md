# Index Home Template

Use this structure for `wiki/index.md`.

```markdown
# {Vault Title}

> [!summary] What this vault is
> `raw/` stores immutable source material.
> `wiki/` stores compiled summaries, concepts, and optional entities.
> `outputs/` stores durable Q&A, health reports, and publishable derivatives.

## Lifecycle

1. Add or clip sources into `raw/`
2. Run compilation into `wiki/`
3. Ask questions and archive substantial answers into `outputs/qa/`
4. Run health checks to catch drift, contradictions, and missing links

## Current State

- Sources: {count}
- Summaries: {count}
- Concepts: {count}
- Entities: {count or "not enabled"}
- Archived Q&A: {count}
- Last compile: {datetime or "not yet compiled"}

## Entry Points

- [[wiki/indices/INDEX]]
- [[wiki/indices/CONCEPTS]]
- [[wiki/indices/SOURCES]]
- [[wiki/indices/RECENT]]
- [[wiki/indices/ENTITIES]] {only if the entity layer exists}

## Current Themes

- {theme 1}
- {theme 2}

## Recent Activity

- {recent event 1}
- {recent event 2}

## Notable Tensions

- {open contradiction or unresolved topic}
```

Rules:

- Keep `wiki/index.md` content-oriented rather than chronological.
- Mention optional entity surfaces only when they actually exist.
- If a vault already uses `wiki/indexes/`, preserve those links instead of forcing a rename.
