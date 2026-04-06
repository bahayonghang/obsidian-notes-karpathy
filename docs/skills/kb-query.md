# kb-query

Search the compiled wiki, answer questions, archive substantive answers, and generate outward-facing artifacts.

## Modes

| Mode | Use it when | Primary write target |
| --- | --- | --- |
| Search | The user wants relevant notes and evidence quickly | usually none |
| Research | The user wants a grounded answer | `outputs/qa/` |
| Publish | The user wants a report, thread, article, slides, charts, or talk outline | `outputs/reports/`, `outputs/slides/`, `outputs/charts/`, or `outputs/content/` |

## Research order

1. start with `wiki/index.md`
2. inspect derived indices such as `INDEX.md`, `CONCEPTS.md`, and `SOURCES.md`
3. use backlinks, unlinked mentions, or Properties search when they surface evidence faster
4. search `outputs/qa/` for prior answers before re-deriving the same result
5. read the most relevant concept pages, entity pages, and summaries
6. answer with provenance and explicit uncertainty where needed

## Default archival rule

Substantive answers are archived to `outputs/qa/` by default. That makes research output part of the vault, not part of disposable chat history.

## Output targets

- `outputs/qa/` for persistent research answers
- `outputs/reports/` for markdown reports
- `outputs/slides/` for Marp decks
- `outputs/charts/` for Mermaid or Canvas artifacts
- `outputs/content/articles/` for article drafts
- `outputs/content/threads/` for social threads
- `outputs/content/talks/` for talk outlines

## Feedback into the wiki

After a substantive answer or publish artifact, the skill may also:

- create a new concept page
- create or enrich an entity page when the entity layer exists
- add evidence to an existing concept or entity page
- add missing connections between pages
- append a `query` or `publish` event to `wiki/log.md`
