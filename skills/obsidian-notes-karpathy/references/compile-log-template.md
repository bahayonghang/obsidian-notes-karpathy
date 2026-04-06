# Compile Log Template

This file is the ingest-only subset of `activity-log-template.md`.

Use it when you only need the compile-batch shape and do not need the full lifecycle log reference.

```markdown
## [{date}] ingest | {batch-slug}

- Sources scanned: {count}
- New summaries: {count}
- Updated summaries: {count}
- New concepts: {count}
- Updated concepts: {count}
- New entities: {count}
- Updated entities: {count}
- Notable tensions: {count}
- Touched files:
  - [[wiki/summaries/source-a]]
  - [[wiki/concepts/concept-b]]
  - [[wiki/entities/entity-c]]
```

Rules:

- log batches, not every micro-edit
- include touched files so later debugging and health checks can trace what changed
- prefer `activity-log-template.md` when query, publish, or health events are also in scope
