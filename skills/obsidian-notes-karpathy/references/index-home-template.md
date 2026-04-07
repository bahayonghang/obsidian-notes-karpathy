# Index Home Template

Use this structure for `wiki/index.md`.

```markdown
# {Vault Title}

> [!summary] What this vault is
> `raw/` stores immutable human and agent captures.
> `wiki/drafts/` stores compiled knowledge waiting for review.
> `wiki/live/` stores approved long-term knowledge.
> `wiki/briefings/` stores per-role context built from live pages.
> `outputs/` stores reviews, Q&A, health reports, and publishable derivatives.

## Lifecycle

1. Add captures into `raw/human/` or `raw/agents/{role}/`
2. Compile into `wiki/drafts/`
3. Review and promote into `wiki/live/`
4. Rebuild affected briefings
5. Query or publish from the live layer
6. Run health checks to catch drift, stale briefings, and backlog

## Current State

- Raw captures: {count}
- Draft summaries: {count}
- Live summaries: {count}
- Live concepts: {count}
- Briefings: {count}
- Review backlog: {count}
- Archived Q&A: {count}

## Entry Points

- [[wiki/live/indices/INDEX]]
- [[wiki/live/indices/CONCEPTS]]
- [[wiki/live/indices/SOURCES]]
- [[wiki/briefings/researcher]]

## Draft Queue

- {pending draft 1}
- {pending draft 2}

## Recent Activity

- {recent event 1}
- {recent event 2}
```

Rules:

- keep `wiki/index.md` content-oriented rather than chronological
- show live and draft state separately
- do not imply drafts are queryable truth
