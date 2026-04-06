# kb-query

Search the compiled wiki, answer questions, and generate reports, slides, diagrams, or audience-facing content drafts.

## Modes

- search mode
- research mode
- publish mode

## Query order

1. start with `wiki/index.md`
2. read the derived indices
3. use Backlinks, unlinked mentions, or Properties search when that reveals related evidence faster
4. check `outputs/qa/` for prior related research
5. read the most relevant concept pages and summaries
6. answer with provenance

## Default archival rule

Substantive answers are archived to `outputs/qa/` by default.

That makes Q&A part of the knowledge base, not part of disposable chat history.

## Output targets

- `outputs/qa/` for persistent research answers
- `outputs/reports/` for markdown reports
- `outputs/slides/` for Marp decks
- `outputs/charts/` for Mermaid or Canvas artifacts
- `outputs/content/articles/` for article drafts
- `outputs/content/threads/` for social threads
- `outputs/content/talks/` for talk outlines

## Feedback into the wiki

After a useful answer, the skill may:

- create a new concept page
- add evidence to an existing concept page
- add missing connections between concept pages
