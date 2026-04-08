# Query and Output

`kb-query` is where the compiled wiki becomes useful.

## Standard sequence

1. read `wiki/index.md`
2. inspect `wiki/live/indices/INDEX.md` and `wiki/live/indices/CONCEPTS.md`
3. use Backlinks, unlinked mentions, or Properties search when they are the fastest route
4. search `outputs/qa/` for relevant prior work
5. read supporting concept pages and summaries
6. answer and save the result when substantive

## Why outputs matter

The workflow treats good answers as reusable knowledge artifacts.

That is why `outputs/qa/` exists as a first-class directory instead of leaving everything in chat history.

When an archived answer reveals durable follow-up work, capture explicit writeback candidates so the next compile/review loop can decide whether the answer should change the wiki.

## Boundary reminder

`kb-query` should not use `raw/`, `wiki/drafts/`, or `MEMORY.md` as default topic retrieval truth.

## Deliverable formats

- markdown reports
- Marp slide decks
- Mermaid diagrams
- Canvas files when explicitly requested
- article drafts
- social threads
- talk outlines
