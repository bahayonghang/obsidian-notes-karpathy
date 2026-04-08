# Obsidian-Safe Markdown

Use this reference whenever the KB workflow writes or repairs markdown that will be rendered inside Obsidian.

## Hard rules

1. Never place alias-style wikilinks such as `[[note|Alias]]` inside Markdown table cells.
2. In tables, use one of these instead:
   - plain wikilinks without an alias, such as `[[wiki/live/concepts/rag]]`
   - standard Markdown links, such as `[RAG](../concepts/rag.md)`
3. Prefer lists over tables when several cells would otherwise need many note links or display aliases.
4. Treat malformed table rendering as a mechanical correctness bug, not a cosmetic issue.

## Why this exists

Markdown tables parse `|` before Obsidian resolves wikilink aliases. A cell like `[[note|Alias]]` therefore splits the row and breaks rendering even though the text looks superficially correct.

## Where this rule applies

- `wiki/live/indices/*.md`
- `wiki/index.md` when it contains tables
- `outputs/qa/*.md`
- `outputs/content/**/*`
- `outputs/reports/*.md`
- `outputs/slides/*.md`
- any health-report remediation that rewrites table rows

## Safe replacements

Bad:

```md
| Concept | [[retrieval-augmented-generation|RAG]] |
```

Good:

```md
| Concept | [[retrieval-augmented-generation]] |
| Concept | [RAG](../concepts/retrieval-augmented-generation.md) |
```

## Repair posture

When you detect alias-style wikilinks inside table cells:

- treat the issue as safe to repair when the target note is clear
- preserve the visible label using a standard Markdown link when an alias matters
- mention the repair in the compile or health summary if the file was changed
