# Activity Log Template

Use `wiki/log.md` as the append-only activity ledger for the whole knowledge-base lifecycle.

## Ingest / compile batch

```markdown
## [{date}] ingest | {batch-slug}

- Raw captures scanned: {count}
- Draft summaries created: {count}
- Draft summaries updated: {count}
- Draft concepts touched: {count}
- Draft entities touched: {count}
- Touched files:
  - [[wiki/drafts/summaries/human/articles/source-a]]
  - [[wiki/drafts/concepts/concept-b]]
```

## Review pass

```markdown
## [{date}] review | {batch-slug}

- Review report:
  - [[outputs/reviews/{file}]]
- Decisions: approve={count} reject={count} needs-human={count}
- Promoted to live:
  - [[wiki/live/summaries/human/articles/source-a]]
- Rejected drafts:
  - [[wiki/drafts/summaries/agents/researcher/source-b]]
```

## Briefing refresh

```markdown
## [{date}] brief | {role}

- Briefing: [[wiki/briefings/{role}]]
- Rebuilt from:
  - [[wiki/live/concepts/concept-a]]
  - [[wiki/live/summaries/human/articles/source-b]]
```

## Query archive

```markdown
## [{date}] query | {question-slug}

- Question: {human-readable question}
- Consulted live pages:
  - [[wiki/live/concepts/concept-a]]
  - [[wiki/live/summaries/human/articles/source-b]]
- Used briefing: [[wiki/briefings/{role}]]
- Archived answer: [[outputs/qa/{file}]]
```

## Archived artifact reuse

```markdown
## [{date}] query | reuse-{reuse-slug}

- Reused artifact:
  - [[outputs/qa/{file}]]
- Reuse target:
  - [[outputs/content/articles/{file}]]
- Grounding still checked against:
  - [[wiki/live/concepts/concept-a]]
```

## Publish artifact

```markdown
## [{date}] publish | {artifact-slug}

- Artifact: [[outputs/content/threads/{file}]]
- Channel: thread | article | talk | report | slides | chart
- Derived from:
  - [[outputs/qa/{file}]]
```

## Publish archive

```markdown
## [{date}] publish | archive-{artifact-slug}

- Archived artifact:
  - [[outputs/content/articles/{file}]]
- Prior archived coverage reused:
  - [[outputs/qa/{file}]]
- Follow-up route: none | draft | review
```

## Health pass

```markdown
## [{date}] health | {run-slug}

- Report: [[outputs/health/health-check-{date}]]
- Overall score: {score}/100
- Critical issues: {count}
- Review backlog items: {count}
- Stale briefings: {count}
```

## Archived backlog escalation

```markdown
## [{date}] health | archive-backlog-{run-slug}

- Escalated archived outputs:
  - [[outputs/qa/{file}]]
  - [[outputs/content/articles/{file}]]
- Reason: writeback backlog | stale archive | reuse gap | scope leak
- Routed to: kb-review (maintenance)
```

Rules:

- append only
- log batch-level work, not every micro-edit
- include enough paths that later diagnosis can trace a promotion chain
