# Source Registry Template

Use this structure for `wiki/indices/SOURCES.md`.

```markdown
# Sources Registry

| Source | Type | Date | Compile State | Key Concepts / Entities | Notes |
|--------|------|------|---------------|-------------------------|-------|
| [[raw/articles/2026-04-01-source-title]] | article | 2026-04-01 | summarized | [[concept-a]], [[wiki/entities/entity-b]] | {optional note} |
```

Rules:

- one row per raw source note
- `Compile State` should be human-readable, such as `new`, `summarized`, `changed`, or `needs-review`
- key concepts or entities should link to wiki pages, not plain text labels
