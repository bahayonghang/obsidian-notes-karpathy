# Query and Output

`kb-query` is where the compiled wiki becomes useful.

## Standard sequence

1. read `wiki/index.md`
2. inspect `wiki/live/indices/INDEX.md`, `CONCEPTS.md`, and relevant source or question views
3. use Backlinks, unlinked mentions, or Properties search when they are the fastest route
4. search `outputs/qa/` for relevant prior work
5. read supporting concept pages and summaries
6. answer and save the result to `outputs/qa/` or `outputs/content/` when substantive

## Why outputs matter

The workflow treats good answers as reusable knowledge artifacts.

That is why `outputs/qa/` exists as a first-class directory instead of leaving everything in chat history.

When an archived answer reveals durable follow-up work, capture explicit writeback candidates so the next compile/review loop can decide whether the answer should change the wiki.

Archive remains downstream:

- `outputs/qa/**` and `outputs/content/**` are reusable artifact archive
- they help future work start faster
- they still do not replace `wiki/live/**` as approved truth

## Reuse order

Before writing something new:

1. check approved live pages
2. check indices and briefings
3. check prior archived Q&A
4. check archived publish artifacts if they already reuse approved coverage cleanly

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
