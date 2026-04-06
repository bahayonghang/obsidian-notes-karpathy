# Activity Log Template

Use `wiki/log.md` as the append-only activity ledger for the whole knowledge-base lifecycle.

## Ingest / compile batch

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

## Query archive

```markdown
## [{date}] query | {question-slug}

- Question: {human-readable question}
- Reused prior Q&A: yes | no
- Consulted:
  - [[wiki/concepts/concept-a]]
  - [[wiki/summaries/source-b]]
- Archived answer: [[outputs/qa/{file}]]
- Fed back into the wiki:
  - [[wiki/concepts/concept-a]]
  - [[wiki/entities/entity-b]]
```

## Publish artifact

```markdown
## [{date}] publish | {artifact-slug}

- Artifact: [[outputs/content/threads/{file}]]
- Channel: thread | article | talk | report | slides | chart
- Derived from:
  - [[outputs/qa/{file}]]
- Supporting pages:
  - [[wiki/concepts/concept-a]]
```

## Health pass

```markdown
## [{date}] health | {run-slug}

- Report: [[outputs/health/health-check-{date}]]
- Overall score: {score}/100
- Critical issues: {count}
- Fix-now items completed: {count}
- Follow-up recommendations:
  - {recommendation}
```

Rules:

- Append only. Do not rewrite history except to correct obvious formatting mistakes.
- Log batches and substantive answers, not every micro-edit.
- Include touched files whenever practical so later diagnosis can trace changes.
