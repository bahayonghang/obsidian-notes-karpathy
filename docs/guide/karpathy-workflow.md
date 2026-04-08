# Karpathy Workflow

This workflow treats a knowledge base like a codebase with source, build artifacts, and maintenance passes.

## Mental model

| Software | Knowledge base |
|----------|----------------|
| source code | `raw/` |
| compiler | `kb-compile` |
| build artifacts | `wiki/` |
| logs and reports | `outputs/` |
| lint | `kb-health` |

## Practical consequences

- humans curate sources
- the LLM maintains structure and cross-links
- useful Q&A gets stored as artifacts
- durable outputs can generate writeback work that re-enters drafts and review
- the wiki can be checked for drift, contradictions, and weak structure

## Retrieval model

The default retrieval story is markdown-first:

1. read `wiki/index.md`
2. read derived indices
3. use Backlinks, unlinked mentions, and Properties search
4. follow wikilinks
5. use file search when needed

Only add heavier search infrastructure when the vault outgrows this model. The next step should usually be local structured search such as DuckDB markdown parsing and full-text search, not an immediate jump to vector infrastructure.

## Next steps

- [Directory Structure](/guide/directory-structure)
- [AGENTS.md and CLAUDE.md](/guide/agents-schema)
- [Skills Overview](/skills/overview)
